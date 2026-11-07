# Game 2162 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02162). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 328 of 328 named (0 author symbols), 328 of 328 described (65 by rule, 97 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 240 of 240 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10237 of 10254 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 71290 |
| code | 32202 |
| data | 502726 |
| padding | 14950 |
| unused | 280064 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Keymap | 1 | 13 | 1 |
| Run | 2 | 117 | 2 |
| Install | 3 | 117 | 2 |
| Else | 2 | 117 | 2 |
| Delete | 2 | 117 | 2 |
| Avail | 11 | 376 | 12 |
| addbuffers | 1 | 101 | 1 |
| Path | 2 | 117 | 2 |
| Resident | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Rename | 2 | 117 | 2 |
| Dir | 2 | 117 | 2 |
| NewCLI | 2 | 117 | 2 |
| NewShell | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| cls | 2 | 20 | 1 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| List | 2 | 117 | 2 |
| CD | 2 | 117 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| LoadWB | 12 | 537 | 30 |
| Version | 11 | 574 | 18 |
| SetMap | 13 | 674 | 30 |
| More | 19 | 2290 | 76 |
| Format | 30 | 2009 | 76 |
| CLI | 11 | 607 | 25 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_BLIT | 11 | 253 | 7 |
| d | 1 | 13 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_CPU | 10 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon | 8 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon_install | 7 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Keymap

- `stub_bad_instruction_data`: Contains only a bad-instruction trap with no usable instructions, so this is stub data rather than a working routine.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen 32-bit words from a fixed offset in a base register into a buffer whose address is passed on the stack, then calls a routine through an indirect pointer.

### Install

- `scan_list_and_install_entry`: Walks a linked list to find the largest length field, then sets up a block sized from that value with a repeating fill pattern.
- `copy_memory_block_3`: Copies a fixed count of words from a memory location to a destination buffer if the destination pointer is not null, before transferring control to another routine.

### Else

- `find_max_and_build_buffer`: Iterates through a linked list to find the maximum value in a field and, if a condition is met, writes a header followed by a sequence of incrementing values and a copied block of data into a buffer.
- `copy_memory_block_4`: Copies a fixed number of words from a source pointer to a destination pointer.

### Delete

- `update_linked_list_header_and_copy_data`: Finds the maximum value in a linked list, writes a new header containing this value and an incrementing counter, then copies a block of data from one memory location to another if certain conditions are met.
- `copy_small_data_block`: copies a fixed number of words from a memory location offset from a pointer to a stack-based or temporary buffer.

### Avail

- `parse_command_arguments`: Allocates a message block and a task port, then splits the command argument text into tokens, expanding quoted strings and asterisk escapes for newline and escape.
- `store_result_and_reply_message`: Stores its argument as the result in a message record, then frees the record's resources and replies to the message port with the stored result.
- `sum_sizes_matching_flags`: Walks a linked list of memory entries and adds the size of each entry, taken as its end minus its start, whose attribute word shares a bit with the mask argument.
- `print_memory_availability_table`: Prints a table with the columns type, available, in-use, maximum and largest, choosing chip, fast or total as the row from the first argument and printing a usage line when the argument is wrong.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_args_to_stack_slot`: Forwards its two arguments to a second routine together with the address of a stack slot, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `format_stack_text_and_emit`: Calls exec's RawDoFmt to fill a 140-byte stack buffer, then measures the text up to its terminator and passes the text and its length to an output routine when the length is nonzero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `set_or_report_drive_buffers`: Parses a drive name argument, rejects a name that is not an existing device with an invalid-name message, and then reads or adjusts that device's buffer count, printing the count.

### Path

- `update_linked_list_header_and_buffer`: Finds the maximum value in a linked list, then writes a header containing this value and a sequence of incrementing or constant values into a memory block.
- `copy_block_and_call_subroutine`: copies a fixed number of words from a memory location pointed to by a register into a stack-allocated buffer and then executes a function pointer previously held in a register.

### Resident

- `calculate_max_and_reconstruct_list`: Iterates through a linked list to find the maximum value in a specific field, then writes a header and reconstructs a new list or buffer containing updated or copied list nodes.
- `copy_block_and_execute_2`: copies a fixed number of words from a memory offset to the stack and then calls another routine.

### Assign

- `calculate_max_and_populate_header`: Traverses a linked list to find the maximum integer value in a field, then writes a header or table entry containing this maximum value and a sequence of incrementing or fixed values.
- `copy_memory_block_5`: Copies a fixed number of words from a memory location to a destination pointer.

### EndCLI

- `find_max_and_prepare_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then writes a header or a certain amount of data into a buffer.
- `copy_fixed_size_block`: copies a fixed number of dwords from a memory location offset by a pointer to a stack or memory buffer, then calls another routine.

### Rename

- `find_max_and_build_header`: Traverses a linked list to find the maximum value in a specific field, then writes a header or metadata block to a calculated memory location.
- `copy_block_to_stack`: copies a fixed number of dwords from a memory location offset from a pointer into a stack-allocated buffer before calling another routine.

### Dir

- `update_list_header_and_copy_data`: Finds the maximum value in a linked list, writes a new header containing this value and an incrementing sequence, and then copies a block of data from a source buffer to a destination buffer while updating a function pointer in the header.
- `copy_block_to_stack_2`: copies a fixed-size block of data from a memory offset to the current stack pointer.

### NewCLI

- `find_max_and_prepare_buffer_3`: Traverses a linked list to find the maximum integer value in a specific field and, if a certain condition is met, writes a header and copies a fixed number of words from a source buffer to a destination buffer, then updates a table entry with a function pointer.
- `copy_stack_block`: copies a fixed number of words from a memory location offset by a register or pointer into the current stack frame.

### NewShell

- `find_max_and_initialize_buffer`: Iterates through a linked list to find the maximum value in a specific field, then writes a header and copies data or function pointers into a destination buffer if certain conditions are met.
- `copy_block_and_execute`: copies a fixed number of words from a memory location offset from a pointer into a stack-based buffer and then calls another routine.

### Echo

- `find_max_and_prepare_buffer_4`: Iterates through a linked list to find the maximum integer value in a field, then writes a header containing this maximum value and an incrementing sequence into a destination buffer.
- `copy_fixed_size_block_2`: copies a fixed number of dwords from a memory location certain offset from a pointer to a stack-based or temporary buffer before calling another routine.

### cls

- `open_dos_and_check_status`: Opens dos.library through exec, then calls two dos functions through the library base.

### Execute

- `update_max_list_header_and_copy_data`: Traverses a linked list to find the maximum value in a field, then writes a header and a sequence of incrementing values into a buffer.
- `copy_list_field_to_stack`: copies a fixed number of dwords from a memory offset or linked list field to the stack.

### Copy

- `update_linked_list_header_and_copy_buffer`: Finds the maximum value in a linked list and writes a new header containing this value into a calculated table position.
- `copy_memory_block`: copies a fixed number of dwords from a source pointer-offset to a destination stack or memory location.

### Makedir

- `find_max_and_initialize_header`: Iterates through a linked list to find the maximum value in a field, then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_memory_block_2`: Checks if a pointer is non-null and copies a fixed number of words from a source pointer to a destination pointer.

### List

- `find_max_and_prepare_buffer_5`: Iterates through a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a calculated buffer.
- `copy_fixed_size_block_3`: copies a fixed number of words from a memory location offset from a pointer into a stack-based or local buffer, then calls another routine.

### CD

- `find_max_and_prepare_buffer_6`: Traverses a linked list to find the maximum value in a specific field and then writes a header containing this maximum value and a sequence of incrementing identifiers into a buffer.
- `copy_block_to_stack_3`: copies a fixed number of words from a memory location or structure member to the current stack pointer and then calls another routine.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `update_header_buffer_and_copy_data`: Finds the maximum value in a linked list, writes a new header or certain count-based metadata to a buffer, and copies a block of data from an external source or a stack-based buffer into that newly prepared header area.
- `copy_buffer_from_pointer`: Copies a fixed number of words from a memory location to a destination in the stack.

### LoadWB

- `parse_args_and_open_handle`: Allocates a message and task record, then splits the argument text into tokens, expanding quoted strings and asterisk escapes for newline and escape.
- `close_handles_and_reply`: Stores its argument as the result in a message record, closes a DOS handle and a library when they are set, frees the record's memory, and replies to the message port with the stored result.
- `start_workbench_tool`: Opens the library dependencies through exec, applies a byte-wise XOR pass to a string twice so it is left unchanged, and starts a Workbench tool.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte.
- `build_node_list_from_table`: Walks a table of pointer and value pairs, allocating a cleared eight-byte node for each pair and storing a value looked up through a library call in it.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `invoke_library_vector_132`: Loads a library base from a global variable and calls the library function at offset minus 132 from it, with no arguments and no result.
- `invoke_library_vector_138`: Loads a library base from a global variable and calls the library function at offset minus 138 from it, with no arguments and no result.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `pad_workbench_release_text`: Checks a text buffer whose byte at index twenty is the digit two and whose prefix matches a global string.
- `write_period_and_spaces`: Writes a period followed by four spaces into a buffer at offset seventeen, one byte at a time.
- `advance_while_prefix_matches`: Compares a global zero-terminated string with the bytes at a pointer, advancing both until the global string ends or a byte differs.
- `return_without_action`: Contains only a return and performs no observable operation.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_exec_dispatch`: Calls the exec Dispatch function through the library base held in a global variable and returns nothing.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is unable to be found.
- `store_low_byte_at_pointer`: Stores the low byte of the data register at the address held in an address register.

### Version

- `parse_directory_string`: Parses a directory path string or string literal, handling quoted strings and converting certain escape-like sequences, and stores the resulting tokens into a message record or a temporary buffer.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version_against_args`: Parses the command arguments for a library or device name with optional version and revision thresholds, opens it, and compares its version and revision with those values.
- `call_library_vector_210`: Calls the library function at offset minus 210 from a global library base with the first argument and the value 128, then calls a second routine with the other argument.
- `parse_signed_decimal_string`: Converts a string to an integer by reading an optional plus or minus sign and then decimal digits, applying the sign to the result.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `measure_null_terminated_length`: Returns the number of bytes before the zero terminator of a string by scanning forward until a zero byte is found.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_dispatch_function_2`: Calls a function pointer retrieved from an offset in a global variable.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `invoke_library_vector_198`: Loads a library base from a global variable and calls the library function at offset minus 198 from it, with no arguments and no result.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `invoke_library_vector_414`: Loads a library base from a global variable and calls the library function at offset minus 414 from it, with no arguments and no result.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `invoke_library_vector_552`: Loads a library base from a global variable and calls the library function at offset minus 552 from it, with no arguments and no result.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_register_to_memory`: Stores the contents of a register into a memory location pointed to by another register.

### SetMap

- `parse_args_and_open_handle_2`: Allocates a message and task record and splits the argument text into tokens, expanding quoted strings and asterisk escapes.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `execute_dispatch_callback`: Executes a function pointer located at a fixed offset from a global address.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `invoke_exec_reschedule`: Calls exec's Reschedule function through the library base held in a global variable and returns nothing.
- `release_stored_table_entries`: Walks a counted table of stored values from the last entry down to the first, passing each nonzero value, shifted right by two bits, to a release routine.
- `store_arguments_then_call`: Stores its two arguments in global variables, then makes one routine call with eight arguments including the values 440 and 60, and finishes with a call that passes the value 20.
- `load_console_keymap_from_args`: Takes a keymap name from the arguments or the icon tool types, lowercases it, and opens the keymap resource and console device.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_function`: Invokes a function pointer located at a fixed offset from a global address.
- `invoke_library_vector_450`: Loads a library base from a global variable and calls the library function at offset minus 450 from it, with no arguments and no result.
- `call_library_subroutine`: Executes a function pointer stored at a specific offset relative to a global variable.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `invoke_library_vector_294`: Loads a library base from a global variable and calls the library function at offset minus 294 from it, with no arguments and no result.
- `invoke_library_vector_210`: Loads a library base from a global variable and calls the library function at offset minus 210 from it, with no arguments and no result.
- `invoke_library_vector_444`: Loads a library base from a global variable and calls the library function at offset minus 444 from it, with no arguments and no result.
- `execute_indirect_function`: Calls a function pointer retrieved from a global structure offset.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `invoke_library_vector_366`: Loads a library base from a global variable and calls the library function at offset minus 366 from it, with no arguments and no result.
- `invoke_library_vector_384`: Loads a library base from a global variable and calls the library function at offset minus 384 from it, with no arguments and no result.
- `invoke_library_vector_96`: Loads a library base from a global variable and calls the library function at offset minus 96 from it, with no arguments and no result.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_with_global_and_stack`: Passes its argument, a value held in a global variable and the address of a stack slot to a routine that calls the library function at offset minus 132.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable and then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_address`: Stores a byte from a register into a location pointed to by a register.

### More

- `parse_filesystem_path`: Parses a string representing a file or directory path, handling quoted strings and converting certain escape sequences like newline or escape characters, while populating a structure with pointers to the parsed path components.
- `update_state_and_run_callbacks`: Updates a state field in a structure via a pointer retrieved from a global variable and executes a sequence of conditional cleanup callbacks or task-related cleanup routines, passing a specific value or identifier as a parameter or context before each call.
- `page_through_file_contents`: Opens a window and shows a named file page by page, with a filename prompt, a percentage indicator and a More prompt, and it quits when the q key is pressed.
- `pass_global_words_to_routine`: Calls another routine four times, passing a structure member from its argument together with each of four consecutive global array words in turn.
- `validate_buffer_text_bytes`: Calls a routine twice with buffer arguments derived from its input, then scans up to eighty bytes of the buffer.
- `dispatch_pager_key_command`: Waits for input with a timeout, then reads one key and maps its code to an action such as paging, searching, moving or quitting.
- `call_two_routines_then_indirect`: Calls one routine with a global variable, calls a second routine with no arguments, then calls through an address register and returns.
- `initialize_scan_state_and_loop`: Sets a counter field to one and a flag field to zero, stores the result of a library call in the structure, and calls a routine on it.
- `combine_results_and_increment_counter`: Calls one routine on a buffer in its argument, passes that result with the buffer to a second routine, and stores the final result in a field.
- `scan_line_and_mark_formfeed`: Reads up to one hundred twenty-eight bytes through a library call and finds the first newline or form-feed character.
- `show_page_lines_until_quit`: Repeats a display pass up to a line count computed from its structure, printing table entries while the scroll position is within range, then shows the More prompt.
- `run_search_pass_until_found`: Chooses a search prompt from a flag, runs search passes while a counter remains positive, and stops when a pass reports a match.
- `run_sequence_and_return_result`: Makes a fixed sequence of calls that prepares a block, runs one routine with the caller's arguments and then finishes the block, returning the result of that middle call.
- `call_pair_then_optional_single`: Calls a routine with two arguments, then calls it again with the first argument alone when a short flag in the block is zero.
- `call_with_block_fields`: Calls a routine with two fields from the block and the value minus one, then calls a second routine with a short field from a nested structure.
- `call_pair_with_decremented_field`: Calls a routine with the block's handle, zero and one, then calls a second routine with a short field from a nested structure minus one.
- `print_page_lines_from_buffer`: Prints up to a set number of lines from a text buffer, scanning up to one hundred twenty-eight bytes at a time for newline and form-feed characters to count lines.
- `forward_block_call_with_global`: Calls a routine with the second argument, then calls a routine whose address is taken from a field of the block, passing a global value and the first argument.
- `set_flag_and_forward`: When a field in the block is zero it sets a short flag in a nested structure to one.
- `dispatch_by_flag_argument`: Calls a routine from a field of the block with a global value, passing a second argument that is either a global or a short field from a nested structure plus one, depending on a flag in the block.
- `dispatch_text_output_by_flag`: Calls a routine from a field of the block with either three global references or a global reference and a short field from a nested structure, depending on a flag in the block.
- `clamp_negative_result`: Calls a routine twice with the same first argument, once with zero and one and once with zero and minus one, then returns the second result with negative values clamped to zero.
- `copy_buffered_bytes_forward`: Copies up to a requested number of bytes from a buffer region in the block to the destination, limited to the bytes still available.
- `refill_buffer_and_reset_offsets`: Returns zero while the buffer is partly filled, and when it is full calls a routine to write out the buffered data.
- `update_viewport_position`: Computes a target position from the offset argument, which is absolute, relative to the current position or relative to the end, and checks it against the visible window bounds.
- `read_line_into_buffer`: Reads characters one at a time through a routine into a destination buffer until a newline or a limit of roughly 127 characters is reached.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination until and including the zero terminator.
- `convert_string_to_uppercase`: Walks a zero-terminated string and changes each lowercase letter from a to z to its uppercase form, leaving other bytes unchanged.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_with_mask`: When a pointer argument is nonzero and a field in the structure is set, calls the library routine held in that field with the pointer and minus one.
- `process_entry_and_write_text`: Processes an entry by calling routines on its linked fields, then writes either the given text or the entry's own text to its output handle.
- `pass_single_word_to_library`: Stores its argument in a one-word stack buffer and passes it, with the constant 994 and a size of one, to a library function reached through a pointer from another routine's result.
- `copy_fields_from_allocated_block`: Allocates a cleared 36-byte block and passes its shifted address with the constant 25 to a library function.
- `allocate_linked_node_with_words`: Allocates a cleared 68-byte node, links it to a looked-up handle and copies a count of words from the source into it.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `execute_indirect_function_pointer`: Loads a function pointer from a location relative to a global pointer and executes it.
- `invoke_exec_switch`: Calls exec's Switch function through the library base held in a global variable and returns nothing.
- `execute_indirect_function_2`: loads a function pointer from a table and executes it.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `invoke_exec_allocmem`: Calls exec's AllocMem function through the library base held in a global variable and returns nothing.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `execute_indirect_function_3`: loads a function pointer from a global table using a fixed offset and executes it.
- `call_indirect_handler`: Loads a function pointer from a global structure and calls it.
- `call_indirect_handler_2`: Executes a routine located at an offset within a global structure.
- `call_indirect_handler_3`: Performs an indirect call to a routine pointer stored in a global table.
- `call_indirect_handler_4`: Calls a routine via a pointer retrieved from a global memory location.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_indirect_handler_5`: Retrieves and calls a function pointer from a global structure offset.
- `invoke_library_vector_372`: Loads a library base from a global variable and calls the library function at offset minus 372 from it, with no arguments and no result.
- `call_indirect_handler_6`: Invokes a routine stored in a global pointer or table.
- `call_indirect_handler_7`: Executes a routine pointer located at a fixed offset from a global variable.
- `call_indirect_handler_8`: Performs an indirect call using a pointer from a global structure.
- `invoke_library_vector_624`: Loads a library base from a global variable and calls the library function at offset minus 624 from it, with no arguments and no result.
- `recurse_with_stack_slot_2`: calls another routine passing two parameters and a pointer to a specific stack location.
- `read_one_signed_byte`: Calls a routine with a small stack buffer and the count one, returning the byte as a signed value when one byte was transferred and minus one otherwise.
- `invoke_and_checksum_buffer`: Invokes a function pointer stored relative to a global variable and then computes a rolling checksum over a stack-allocated buffer until a null terminator is reached.
- `store_register_to_pointer`: copies a byte from a data register into a memory location pointed to by an address register.
- `divide_longword_with_remainder`: Divides a 32-bit dividend by a divisor, using bit-by-bit shift and subtract when the divisor exceeds 16 bits and two 16-bit divide steps otherwise.
- `multiply_32bit_integers`: Multiplies two 32-bit values using 16-bit partial products, returning the 32-bit product in its high part and the second operand with its halves swapped in its low part.
- `call_own_entry_then_return`: Calls its own entry point with no arguments and then returns, with no other effect.
- `negate_result_on_flag`: Calls a routine with no arguments and returns its result, negated when the second register value it leaves is nonzero.
- `invoke_exec_rawdofmt`: Calls exec's RawDoFmt through the library base held in a global variable and returns nothing.
- `store_register_to_pointer_2`: stores the contents of a data register into a memory location pointed to by a register.
- `invoke_library_vector_276`: Loads a library base from a global variable and calls the library function at offset minus 276 from it, with no arguments and no result.
- `create_message_port_with_signal`: Allocates a signal bit, then a cleared 34-byte message port whose fields are set from the arguments, with the current task stored in it.
- `delete_message_port_and_free`: Removes a message port from the system when its link field is set, marks its status fields as deleted, frees its signal bit and releases its 34-byte memory block.
- `search_text_for_pattern`: Searches text for a zero-terminated pattern one position at a time and returns one when the pattern is found.
- `initialize_empty_list_header`: Initializes a list header in a block by storing the header's address plus four in its first word, clearing its second word and pointing its third word back at the header.

### Format

- `initialize_filesystem_environment`: Initializes global pointers and parses a directory or file path string into a temporary buffer, handling quoted strings and escaped characters.
- `update_format_state_and_cleanup`: Updates a stored parameter within a structure pointed to by a retrieved pointer and executes a sequence of conditional cleanup callbacks, passing a specific identifier as a parameter to each before calling them if their corresponding pointers in a table are non-null.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_indirect_handler_9`: Calls a routine using a pointer stored in a global structure.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `execute_switched_context`: Executes a function pointer retrieved from an offset of a global variable.
- `invoke_exec_forbid`: Calls exec's Forbid function through the library base held in a global variable and returns nothing.
- `execute_indirect_function_4`: loads a function pointer from a global table and executes it.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `invoke_exec_schedule`: Calls exec's Schedule function through the library base held in a global variable and returns nothing.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `invoke_exec_allocabs`: Calls exec's AllocAbs function through the library base held in a global variable and returns nothing.
- `invoke_indirect_function`: Loads a function pointer from a global table offset and executes it.
- `initialize_disk_from_arguments`: Parses the DRIVE, NAME, FFS, NOICONS and QUICK keywords from the arguments and prints the usage text when they are missing.
- `release_stored_format_buffers`: Releases up to five stored buffers in turn, passing each to a release routine together with its known size where one applies, and then calls a final routine with its argument.
- `map_disk_error_to_message`: Reads a status code from a device call, retries while it reports a particular retry code, and maps the code to one of the messages for a bad sector, a device that cannot be opened, a changed disk, a seek error or a request to try a different disk.
- `report_format_status_message`: Prints a format status message, either a fixed pair of messages with a request or one message selected by a stored value, then returns the status byte.
- `match_value_in_table`: Looks for the argument among eight bytes of a global table and stores the matching byte, or zero when none matches, into a field of a structure obtained through a routine.
- `find_drive_entry_by_name`: Walks a list of entries through a pointer chain, matching each entry whose flag field is zero either by a stored value or by a name argument.
- `calculate_string_length`: Counts the bytes before the zero terminator of a string and returns that count.
- `copy_cstring_to_buffer`: Copies a zero-terminated byte string from the source to the destination, including the terminator.
- `convert_char_to_uppercase`: Returns the byte as an uppercase letter when it is a lowercase letter from a to z, and returns it unchanged otherwise.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings after converting each byte to uppercase.
- `cut_string_at_colon`: Terminates a string at its first colon, or at its existing terminator when no colon is present.
- `call_single_argument_with_code`: Passes its first argument, as a one-element stack array with a count of one, to a routine together with the constant 31.
- `allocate_node_limited_words`: Allocates a cleared 68-byte node, links it to a looked-up handle and copies up to seven words from the source into it.
- `invoke_library_vector_354`: Loads a library base from a global variable and calls the library function at offset minus 354 from it, with no arguments and no result.
- `execute_indirect_function_5`: loads a function pointer from a table and jumps to it.
- `invoke_library_vector_330`: Loads a library base from a global variable and calls the library function at offset minus 330 from it, with no arguments and no result.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `execute_indirect_function_6`: loads a function pointer from a global table using a constant offset and executes it.
- `execute_buffered_format_handler`: loads a function pointer from a location relative to a global table and executes it.
- `invoke_library_vector_456`: Loads a library base from a global variable and calls the library function at offset minus 456 from it, with no arguments and no result.
- `call_indirect_function`: loads a function pointer from a global table and executes it.
- `execute_indirect_routine`: Loads a routine pointer from a global memory location offset by a constant and executes it.
- `invoke_library_vector_336`: Loads a library base from a global variable and calls the library function at offset minus 336 from it, with no arguments and no result.
- `call_indirect_function_2`: loads a function pointer from a global table and executes it.
- `execute_indirect_routine_2`: fetches a routine pointer from a table and executes it via an indirect call.
- `execute_indirect_pointer`: loads a pointer from a global variable and executes it.
- `execute_indirect_pointer_2`: fetches a function pointer from a memory offset and jumps to it.
- `invoke_library_vector_360`: Loads a library base from a global variable and calls the library function at offset minus 360 from it, with no arguments and no result.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `invoke_library_vector_306`: Loads a library base from a global variable and calls the library function at offset minus 306 from it, with no arguments and no result.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `execute_indirect_pointer_3`: resolves a pointer from a global offset and performs an indirect call.
- `forward_with_global_and_stack_2`: Passes its argument, a value held in a global variable and the address of a stack slot to a second routine, then returns.
- `invoke_library_vector_348`: Loads a library base from a global variable and calls the library function at offset minus 348 from it, with no arguments and no result.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_tracks_with_progress`: Opens the disk device, reads its geometry, and writes each track in turn, reporting progress with the labels Formatting, Initializing disk and Verifying and the cylinder count.
- `apply_format_settings_sequence`: Makes a fixed sequence of calls through a library base pointer stored in a global variable, passing two constants first, then a flag, then the argument and the result of an inner routine.
- `invoke_global_and_measure_string`: Calls a function pointer held in a global variable, then counts the length of a 140-byte stack buffer up to its null terminator.
- `store_low_byte_at_pointer_2`: Writes the low byte of the data register to the memory address held in the address register and returns.
- `divide_long_with_remainder`: Divides a 32-bit dividend by a divisor and returns quotient and remainder.
- `shift_subtract_divide_loop`: Runs the 16-step shift-and-subtract division loop on a 32-bit dividend and a divisor held in registers, leaving quotient bits in the low half of the working register and the remainder in the high half.
- `multiply_32_bit_integers`: Multiplies two 32-bit values using 16-bit partial products.
- `invoke_table_entry`: Calls the function whose address is stored at a fixed offset inside a global data block and returns with no result of its own.
- `store_byte_via_address_register`: Stores the low byte of the data register at the address in the address register, then returns.
- `allocate_and_initialize_block`: Allocates a memory block through a library call, fills its pointer, flag and count fields from the arguments, then either sets up a list head or passes the block to another routine.
- `reset_object_and_recurse`: Recursively processes the linked block when its pointer field is set, then marks the byte and long fields as all ones, calls a routine with the stored byte value and calls another routine with the block and a constant.
- `forward_argument_with_constant`: Passes its single argument to another routine together with the constant 48 and returns nothing.
- `forward_single_argument`: Passes its single argument unchanged to another routine and returns nothing.
- `setup_trashcan_with_requesters_disabled`: Clears the process window pointer while it sets up global fields that reference the Trashcan text, runs initial routines and then restores the window pointer.
- `read_signed_byte_result`: Reads one byte through a routine that takes a global argument and a stack buffer.
- `invoke_library_entry_240`: Calls the library function at a fixed negative offset from a global library base, passing no arguments and returning no value in the shown code.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Writes the block's own address plus four into its first long word, clears the second long word and stores the block's original address in the third.
- `store_word_pointer_in_node`: Allocates a node through a library-backed routine when the pointer argument is non-null, then stores the value 5 as a byte, the argument word and the pointer into fixed fields.
- `reset_node_and_release_it`: Returns zero for a null pointer.
- `invoke_library_entry_90`: Calls the library function at a fixed negative offset from a global library base as a plain call, with no argument or result handling in the shown code.
- `execute_indirect_pointer_4`: executes a routine located at a pointer stored in a global variable.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `run_cli_with_parsed_arguments`: Program entry point that allocates a session state block, opens the libraries it needs, tokenises the command line into an argument array with quote and escape handling, runs the command through the shell routines and then releases everything, returning a status code.
- `invoke_supervisor_function`: Calls the Supervisor function of the exec library through a library base stored in a global variable, with no arguments.
- `invoke_exit_interrupt_function`: Calls the ExitIntr function of the exec library through a stored library base to leave an interrupt handler.
- `invoke_task_switch_function`: Calls the Switch function of the exec library through a stored library base to hand control to another task.
- `invoke_dispatch_function`: Calls the Dispatch function of the exec library through a stored library base to run the scheduler.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `invoke_make_functions_table`: Calls the MakeFunctions function of the exec library through a stored library base, which sets up a function table for a library.
- `invoke_find_resident_function`: Calls the FindResident function of the exec library through a stored library base to search for a resident module by name.
- `invoke_enable_interrupts_function`: Calls the Enable function of the exec library through a stored library base, which re-enables interrupts.
- `invoke_alloc_entry_function`: Calls the AllocEntry function of the exec library through a stored library base to allocate an entry structure.
- `open_new_cli_session`: Allocates a 64-byte session block when given a zero argument, otherwise reuses the current session.
- `build_linked_node_list`: Walks a list of entries, allocating an 8-byte node for each one, copying the entry value into the node and chaining the nodes in order.
- `free_linked_node_list`: Walks a linked list, releasing the value attached to each node and then the 8-byte node itself.
- `parse_signed_decimal_string_2`: Parses an optional leading plus or minus sign followed by decimal digits from a null-terminated string and returns the signed value.
- `invoke_library_entry_198`: Calls the library function at a fixed negative offset from a global library base, with no arguments and no result handling in the shown code.
- `execute_indirect_pointer_5`: performs an indirect call using a pointer retrieved from a global address.
- `execute_indirect_pointer_6`: dereferences a pointer located at a specific offset from a global variable and calls it.
- `execute_indirect_pointer_7`: loads a function address from a global variable and transfers control to it.
- `execute_indirect_pointer_8`: invokes a routine via a pointer stored in a global memory location.
- `invoke_library_entry_78`: Calls the library function at a fixed negative offset from a second global library base, with no arguments and no result handling in the shown code.
- `execute_indirect_pointer_9`: executes a routine by dereferencing a pointer found at a global offset.
- `invoke_library_entry_96`: Calls the library function at a fixed negative offset from a secondary global library base, passing no arguments and handling no result.
- `divide_value_with_remainder`: Divides a 32-bit dividend by a divisor and returns quotient and remainder.
- `multiply_32_bit_partial_products`: Multiplies two 32-bit values using 16-bit partial products.
- `negate_on_flag`: Calls another routine and returns its first result value, negated when its second result value is non-zero.

### Disk-Validator

- `check_disk_bitmap_entries`: Starts a disk validator that writes a two-digit count into a text buffer, initialises a table with all-ones values and then walks bitmap words testing individual allocation bits.

### Ram-Handler

- `dispatch_after_copy_memory`: Calls the exec CopyMem routine through a global library base, then makes an indirect jump through a table the decompiler could not recover, so the final target is not visible.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_BLIT

- `restore_patched_word_after_calls`: Runs a loop of library calls through a global base, then checks a long word in a structure referenced by a global.
- `decrement_until_minus_one`: Decrements a 16-bit counter register in a loop until it reaches minus one, then returns.
- `decode_bitstream_copy_runs`: Loops over a bit-stream reader, decodes bit-coded run lengths and copies runs of words through a relative offset in the buffer, then calls the library cache clear routine when a global counter is above a threshold.
- `copy_bytes_backwards_from_stream`: Copies a run of bytes backwards from an input stream into the output buffer, reading bit-coded run lengths between copies.
- `decode_run_length_table_copies`: Reads bit-coded values, selects run lengths from a small stored table and copies runs backwards through the input and output pointers.
- `decode_bit_coded_offset_run`: Decodes bit-coded run and offset values from a packed bit stream, copies the bytes they describe into the output and returns a 32-bit value taken from the output.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### d

- `stub_bad_instruction_data_2`: Holds only bad instruction data from the file start, so the routine is a stub with no recoverable behaviour.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_CPU

- `swap_patched_word_after_calls`: Calls a function pointer and a library routine in sequence, then checks a long word in a structure referenced by a global.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon

- `initialize_system_memory`: Executes a sequence of function calls and a loop that decrements a counter, then checks if a specific memory pattern exists at a calculated offset to update a global pointer and clear a memory flag.
- `decrement_until_negative_one`: Decrements a 16-bit register or local variable until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon_install

- `reset_install_state_at_entry`: Program entry for Ambermoon_install that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_until_negative_one_2`: Decrements a 16-bit or 32-bit register or local variable until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `compare_and_branch_on_longword`: Compares a longword at a pointer with a constant value and branches if they are equal.
