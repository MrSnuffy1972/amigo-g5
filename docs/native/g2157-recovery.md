# Game 2157 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02157). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 55 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 328 of 328 named (0 author symbols), 328 of 328 described (65 by rule, 125 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 240 of 240 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10237 of 10254 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 71290 |
| code | 32202 |
| data | 501504 |
| padding | 14636 |
| unused | 281600 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Keymap | 1 | 13 | 1 |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_BLIT | 11 | 253 | 7 |
| d | 1 | 13 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_CPU | 10 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon | 8 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon_install | 7 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `stub_unreached_gap_code` (More): The recovered body is only a short call sequence ending in an indirect call, and it was reached only as a gap in the decode. Kind: sabotage path. Pass path: the instruction following the call to the third (unaffected/unreachable) pointer.

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Keymap

- `empty_routine`: the routine performs no operations and contains no valid executable instructions.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_and_call`: If the destination pointer is nonnull, copies 16 longwords from a fixed offset in the structure into it.

### Install

- `build_command_record_with_handler`: Scans a linked list for the largest stored value, then builds a record whose handler is the small routine at offset 310, with a 140-byte stack buffer and words generated from a counter.
- `copy_block_and_branch`: copies a fixed number of words from a memory location to a stack-allocated buffer before calling another routine.

### Else

- `find_max_and_prepare_buffer_5`: Iterates through a linked list to find the maximum value in a specific field and, if a certain condition is met, initializes a header or buffer.
- `copy_block_to_stack`: copies a fixed-size block of 16 or 17 words from a memory offset or buffer to the current stack pointer and then calls another routine.

### Delete

- `find_max_and_populate_buffer_3`: Traverses a linked list to find the maximum integer value in a specific field, then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_data_to_stack`: copies a fixed number of words from a memory offset into the stack.

### Avail

- `parse_command_line_and_start`: Program entry for the Avail utility that opens the dos library and allocates a control block, then splits the command line into argument strings, honoring double quotes and asterisk escapes.
- `release_file_and_library_state`: Fetches a state pointer from a global library base, stores the given value in it, and conditionally closes a file, closes a library, frees memory and replies to a message based on flags in that state.
- `sum_memory_region_sizes_by_flag`: Walks a linked list of memory region records and adds up the size of each region whose flag bits overlap the given mask.
- `print_memory_availability_table`: Prints a memory table with Type, Available, In-Use, Maximum and Largest columns for chip, fast and total memory.
- `call_reschedule_3`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_with_stack_slot_pointer`: Passes its two arguments to another routine together with a pointer to a fixed slot 12 bytes into the caller's stack frame.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `measure_stack_string_and_forward`: Calls a function pointer from a global variable to fill a 140-byte stack buffer, then counts the characters up to the null terminator.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `add_disk_buffers_to_device`: Program entry for the addbuffers command: it reads the DRIVE and BUFFERS arguments, checks the named device, and reports either its current buffer count or an invalid device or volume error.

### Path

- `find_max_and_prepare_buffer_6`: Iterates through a linked list to find the maximum value in a specific field, then initializes a destination buffer with this value and a sequence of incrementing constants.
- `copy_list_element_header`: copies a fixed-size block of data from a structure or list entry to a stack-based or destination buffer.

### Resident

- `find_max_and_initialize_buffer_4`: Iterates through a linked list to find the maximum value in a specific field and, if a destination pointer is valid, writes a header and copies a sequence of incrementing values and a block of data into a target buffer.
- `copy_block_and_execute`: copies a fixed-size block of 16 dwords from a memory location certain enough to be a structure or buffer back to the stack and then calls another routine.

### Assign

- `find_max_and_initialize_buffer_5`: Iterates through a linked list to find the maximum value in a specific field and, if a destination pointer is valid, initializes a buffer or structure header with this maximum value and a sequence of incrementing values.
- `copy_block_and_execute_2`: Copies a fixed number of words from a memory location or structure offset to a stack-based or temporary buffer and then calls another routine.

### EndCLI

- `find_max_and_initialize_buffer`: Walks a linked list to find the maximum value in a specific field, then initializes a buffer or structure header with this value and a sequence of incrementing integers.
- `copy_memory_block`: copies a fixed number of words from a source pointer to a destination pointer.

### Rename

- `find_max_and_initialize_buffer_2`: Iterates through a linked list to find the maximum integer value in a specific field, then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_block_and_call_function`: copies a fixed number of words from a memory location or structure into a stack-based buffer and then executes another routine.

### Dir

- `update_max_header_and_copy_data`: Iterates through a linked list to find the maximum value in a field, writes this value and a sequence of incrementing integers into a header, and then performs a block copy of data between a source and destination if a pointer is valid.
- `copy_fixed_size_block`: copies a fixed number of words from a memory location or table, starting at a certain offset, to the stack.

### NewCLI

- `find_max_and_prepare_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or structure header with this value and a sequence of incrementing or fixed data.
- `copy_sixteen_longwords_and_call`: Copies sixteen 32-bit values from a fixed offset in a base register into a caller-supplied buffer when that buffer pointer is non-null.

### NewShell

- `fill_pattern_and_install_handler`: Walks a linked list to find the largest value in a field, then fills a buffer with an incrementing word pattern and records that maximum in its header.
- `copy_fixed_size_block_2`: copies a fixed number of dwords from a memory location certain offset from a pointer into a stack or destination buffer and then calls another routine.

### Echo

- `find_max_and_update_buffer`: Iterates through a linked list to find the maximum value in a field, then writes a header and copies a sequence of longwords into a destination buffer if a condition is met.
- `copy_fixed_block_and_execute`: copies a fixed number of words from a memory location or table to the stack and then calls another routine.

### cls

- `open_dos_library`: Opens a library by name through the exec library table and calls two more functions on the returned base.

### Execute

- `find_max_and_prepare_buffer_3`: Iterates through a linked list to find the maximum value in a specific field, then initializes a destination buffer with a header and copies or writes a block of data if a pointer is non-null.
- `copy_fixed_size_block_3`: copies a fixed number of words from a memory location or structure offset to a stack or memory destination.

### Copy

- `find_max_and_copy_buffer`: Traverses a linked list to find the maximum value in a specific field, then writes a header and copies a block of data or a fixed number of longwords if a destination pointer is valid.
- `copy_memory_block_2`: copies a fixed block of sixteen bytes from a memory location offset from a register or pointer to a destination in the stack.

### Makedir

- `find_max_and_populate_buffer`: Iterates through a linked list to find the maximum value in a specific field, then writes a header and copies a block of data or a pattern of incrementing values into a destination buffer.
- `copy_memory_block_3`: copies a fixed number of words from a memory location or a pointer-offset to a destination buffer.

### List

- `find_max_and_initialize_buffer_3`: Iterates through a linked list to find the maximum value in a specific field, then writes this value to a header followed by an incrementing sequence of integers.
- `copy_fixed_size_block_4`: copies a fixed number of longwords from a memory location offset from a register pointer to a stack or memory location, then calls another routine.

### CD

- `find_max_and_populate_buffer_2`: Walks a linked list to find the maximum value in a specific field, then writes a header and copies a block of data into a destination buffer if a pointer is valid.
- `copy_data_block`: copies a fixed number of words from a memory location to the stack.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_prepare_buffer_4`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or structure header with this value and a sequence of incrementing or decrementing integers.
- `copy_memory_block_4`: Copies a fixed number of words or dwords from a source pointer to a destination pointer.

### LoadWB

- `parse_args_and_launch_program`: Allocates a task record and opens the dos library and standard handles, then either opens a named file from the startup message or splits the command string into arguments, honouring quoted strings and star sequences for newline and escape characters.
- `release_process_resources_and_reply`: Stores its parameter as the return value, then closes a file handle, closes a library base and frees a memory block, each step guarded by a stored pointer.
- `start_resident_tool_task`: Opens the icon and intuition libraries and decodes a fixed string in place with a one-byte key.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte.
- `build_lookup_node_list`: Walks a chain of table entries, allocating a node for each and storing in it the result of a library lookup on the entry's value.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_indirect_function`: Calls a function pointer retrieved from a memory location relative to a global pointer.
- `call_indirect_function_offset`: Calls a function pointer retrieved from a memory location with a different offset relative to a global pointer.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `stamp_release_suffix_on_match`: Checks that the buffer pointer is valid and that the byte at a fixed offset is the character two.
- `write_dot_and_four_spaces`: Writes a dot followed by four space characters into a buffer at a fixed offset, filling five bytes.
- `advance_while_matching_global_prefix`: Advances a buffer pointer for as long as its characters match a global string, stopping at the first mismatch or at the end of the global string.
- `empty_routine_2`: The routine performs no observable operation and ends with a return.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_exec_dispatch`: Calls the Dispatch routine of the exec library through a stored library base and returns its result to the caller.
- `call_find_resident_2`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_with_local_stack_slot`: Forwards its two arguments to a second routine together with the address of a slot on its own stack, then returns.
- `format_message_and_forward`: Calls the RawDoFmt routine of the exec library through a stored base to format text into a 140-byte stack buffer.
- `store_byte_at_address`: Writes the low byte of a data register to the address held in an address register.

### Version

- `setup_task_and_parse_arguments`: Allocates a task record and opens the dos library, then either fetches a stored handle or parses the command string into arguments, honouring quoted strings and star escapes for newline and escape characters.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses command-line arguments to open a named library or device, then compares its version and revision with the values supplied.
- `forward_args_to_two_helpers`: Passes its first argument together with the constant 128 to one helper, then passes its second argument to a second helper and returns.
- `parse_integer_string`: Converts a decimal string to an integer, accepting an optional leading plus or minus sign.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Returns the number of characters before the terminator of a null-terminated string, by advancing to the terminator and subtracting the start address.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: loads a function pointer from a memory location and jumps to it via an indirect call.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `execute_indirect_handler`: Loads a function pointer from an offset of a global variable and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: loads a function pointer from a memory-offset or table and executes it.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: loads a function pointer from a memory-offset and executes it.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer`: Stores a single byte from a register into a location pointed to by a register.

### SetMap

- `initialize_execution_environment`: Initializes the game's internal state by resolving library pointers, checking for valid memory allocation, and parsing command-line arguments or tool-type strings into a buffer while handling quoted strings and escape-like character substitutions.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `execute_indirect_function`: Loads a function pointer from a global offset and executes it.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `call_reschedule`: Calls the Reschedule routine of the exec library through the stored base and returns its result to the caller.
- `release_table_entries_and_finish`: Walks a table from the last entry back to the first, calling a routine on each non-zero entry with its value shifted right by two.
- `store_pair_and_run_setup`: Stores its two arguments into global variables, then calls a routine with a fixed set of constants and global addresses.
- `load_keymap_for_console`: Opens the console device, icon and keymap resource libraries, then takes the keymap name from the command arguments or the icon's tool types.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `execute_indirect_function_2`: loads a function pointer from a global table and executes it.
- `call_library_vector_450`: Calls a library routine at a fixed offset through the stored library base and returns its result.
- `execute_indirect_function_pointer_4`: loads a function pointer from a global table and calls it.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `execute_indirect_function_3`: dereferences a pointer located at a fixed offset from a global base address and executes the routine found there.
- `execute_indirect_function_4`: loads a function pointer from a de-referenced global table and calls it.
- `execute_indirect_function_5`: loads a function pointer from a location relative to a global base address and jumps to it.
- `call_indirect_pointer`: Calls a function pointer retrieved from a global structure.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_366`: Calls a library routine at a fixed offset through the stored library base, with no arguments set up here, and returns its result.
- `execute_indirect_function_6`: loads a function pointer from a location offset from a global pointer and executes it.
- `execute_indirect_function_7`: fetches a function pointer from a global table and executes it.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_stored_value_with_slot`: Forwards a value held in a global variable together with its own argument and the address of a slot on its stack to a second routine, then returns.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `invoke_and_checksum_stack_buffer_2`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack-allocated buffer until a null terminator is found.
- `store_register_to_memory`: Stores a byte from a data register into a memory location pointed to by an address register.

### More

- `initialize_filesystem_context`: Obtains the current directory and opens a file or directory, then parses its contents or path string into a structured buffer.
- `update_and_cleanup_context`: Updates a state-tracking structure with a new parameter and conditionally executes a sequence of cleanup callback functions stored in a table, passing a specific identifier to each before the call.
- `run_file_pager_loop`: Opens a window and runs an interactive loop over a file, showing a progress percentage and a quit prompt.
- `iterate_table_with_function`: Loops four times over a global table of values, calling a routine with each value and with a fixed field of the structure it is given.
- `validate_input_buffer`: Calls a routine twice to fill or read an eighty-byte buffer, then scans its bytes.
- `dispatch_pager_key_command`: Waits for an input signal with a timeout, then reads a key code and maps it to a pager action such as help, search or an escape-sequence key.
- `stub_unreached_gap_code`: The recovered body is only a short call sequence ending in an indirect call, and it was reached only as a gap in the decode.
- `initialize_state_and_loop`: Sets a counter to one and clears a flag in the structure, then calls a routine and stores its result.
- `update_result_and_increment_counter`: Calls a routine twice with arguments derived from the structure pointer and stores the second result in a field.
- `scan_line_mark_form_feed`: Fills a buffer through a routine, then scans it up to its length for a newline or form-feed character.
- `run_display_loop_until_quit`: Chooses a loop count from flags in the structure, and each pass calls a routine and forwards up to that many global values to another routine.
- `handle_search_loop`: Selects a global string according to a flag in the argument block and runs a search over it, repeatedly calling a routine and counting down until the search reports a match.
- `run_wrapped_call_with_cleanup`: Runs a setup call, a call with the parameter block, an optional call that depends on a flag, then the main call, keeping its result.
- `check_flag_and_call`: Calls a routine with its first two arguments, then calls it again with the first argument only when a flag field in the structure is zero.
- `call_handler_pair_from_struct`: Calls a routine with a stored value and a minus-one argument, then calls it again with a short field read through a pointer held in the structure.
- `call_handler_with_decremented_field`: Calls a routine with a zero argument and the constant one, then calls it again with a short field read through a pointer held in the structure, minus one.
- `emit_text_lines_until_break`: Clears a flag and counts the lines it outputs, limited by a field in the structure capped at 128 characters.
- `call_inner_then_dispatch_handler`: Calls a routine with the structure pointer, then calls a handler stored in the structure with a global address and the first argument.
- `set_flag_and_forward`: When a field in the structure is zero, it sets a short field in a linked structure to one.
- `dispatch_by_flag`: Chooses between a global address and a value read from a linked structure, depending on a flag in the block.
- `dispatch_handler_by_flag`: Calls a routine with a stored handle and a global address, followed by either two more global addresses or a short value read through a linked structure, depending on a flag in the block.
- `clamp_negative_result`: Calls a routine twice with the same first argument and two different trailing values, then returns the second result with negative values replaced by zero.
- `consume_from_buffer`: Takes up to the requested number of bytes from a buffer tracked inside the structure, first calling a refill routine when the request exceeds what is available.
- `reset_buffer_and_refill`: Resets the buffer bookkeeping and calls a routine to fill the remainder of a 4096-byte block, adding the returned count to the remaining size when it is positive.
- `update_scroll_window`: Computes a target offset from a mode argument and compares it with the current window bounds.
- `read_line_from_buffer`: Reads bytes one at a time through a routine into a destination buffer, stopping after a newline or a fixed length limit, then appends a null terminator.
- `copy_null_terminated_string`: Copies bytes from a source to a destination until a null terminator has been copied.
- `convert_string_to_uppercase`: Converts each lowercase ASCII letter in a null-terminated string to uppercase in place, leaving other bytes unchanged.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated string, accepting an optional leading plus or minus sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a unable to be determined and a provided mask.
- `process_text_and_restore_state`: Processes a text string by calling a routine with it, or with the structure when the string is empty, then restores several saved fields and, when a flag is set and the string is non-empty, reads characters until a line feed.
- `build_message_buffer`: Looks up a handle through a routine, places the argument in a one-word stack buffer, then passes that buffer with a constant tag and a length of one to a second routine.
- `copy_fields_from_allocated_block`: Allocates a 36-byte block and, if successful, passes it to a routine together with a constant.
- `allocate_object_and_copy_words`: Looks up a handle and allocates a 68-byte object, linking it to that handle and to a stored value.
- `call_supervisor_2`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr_2`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule_2`: Calls the Reschedule routine of the exec library through a stored base and returns its result to the caller.
- `call_exec_switch`: Calls the Switch routine of the exec library through the stored base, with no arguments set up here.
- `call_dispatched_function`: Calls a function pointer located at an offset from a global variable.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable_2`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Calls the AllocMem routine of the exec library through the stored library base and returns its result to the caller.
- `call_alloc_abs_2`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `execute_indirect_function_8`: Loads a function pointer from an offset within a global table and executes it.
- `execute_indirect_function_pointer_5`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_6`: Fetches a function pointer from a location relative to a global pointer and executes it.
- `execute_indirect_function_9`: Loads a function pointer from a global memory location and jumps to it.
- `call_indirect_function_2`: loads a function pointer from an offset within a global table and executes it.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `execute_indirect_function_10`: Loads a function pointer from a table and executes it.
- `call_library_function_offset_174`: Calls a functionget pointer located at an offset from a global variable.
- `call_library_function_offset_180`: Calls a function pointer located at an offset from a global unable to any or other global variable.
- `execute_indirect_handler_2`: Executes a function pointer retrieved from a global structure offset.
- `execute_indirect_function_pointer_7`: loads a function pointer from a memory location and executes it.
- `execute_indirect_function_11`: loads a function pointer from a global table and executes it.
- `recurse_with_stack_slot_2`: Calls another routine passing two parameters and a pointer to a fixed slot 12 bytes into the caller's stack frame.
- `extract_byte_from_call`: Calls a routine to read one byte into a four-byte stack buffer.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer_2`: copies a byte from a data register to a memory location pointed to by a register.
- `divide_32bit_with_remainder_2`: Divides a 32-bit value by a divisor, using a bit-by-bit shift-and-subtract loop for large divisors and two 16-bit divisions for small ones, and returns the remainder and quotient together as a pair of longwords.
- `multiply_32bit_partial_products`: Computes the low 32 bits of the product of two 32-bit values from 16-bit partial products, returning that product in the high longword.
- `stub_unreached_gap_code_2`: The body is one call followed by a return, and it was reached only as a gap in the decode.
- `negate_on_flag`: Calls a routine and keeps its first result, which is negated when the second result is non-zero.
- `call_rawdofmt_through_base`: Calls the RawDoFmt routine of the exec library through the stored base and returns.
- `store_register_to_pointer`: copies a byte from a data register into a memory location pointed to by an address register.
- `call_library_vector_276`: Calls a library routine at a fixed offset through a stored base pointer taken from a global variable, with no arguments set up here, and returns.
- `allocate_signal_and_message`: Allocates a signal bit and, if that succeeds, a 34-byte message block whose fields are filled from the arguments, the signal number and a value from a routine.
- `release_message_port_resources`: If a link field is set, removes the block from its list, then marks its status fields as deleted.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Initialises a block as an empty linked list head, pointing its first word at itself plus four, clearing the following word and storing the block's own address in the third word.

### Format

- `initialize_filesystem_and_parse_path`: Initializes a data structure with pointers to libraries and memory, then parses a directory path string while handling quoted strings and escaped or special character replacements.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_indirect_function_via_pointer`: Loads a function pointer from a memory location and executes it.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `execute_indirect_function_12`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_8`: loads a function pointer from a memory-offset relative to a global pointer and executes it.
- `execute_indirect_function_13`: loads a function pointer from a global table and jumps to it via an indirect call.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_exec_schedule`: Calls the Schedule routine of the exec library through the stored base, with no arguments set up here, and returns.
- `call_make_functions_2`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_alloc_abs`: Calls the AllocAbs routine of the exec library through the stored base and returns its result to the caller.
- `execute_indirect_handler_3`: Executes a function pointer retrieved from a global structure unable to be identified via registers.
- `format_disk_from_arguments`: Parses the DRIVE, NAME and option arguments, then checks that the drive exists and is empty and asks the user to insert a disk and press return.
- `release_format_buffers`: Frees each stored buffer that is set.
- `report_disk_error_and_retry`: Retries a disk operation while it returns a specific retry code, then maps the final error code to one of several messages such as bad sector, seek error, disk changed or cannot open device.
- `return_status_after_message`: Chooses between two sets of calls depending on a word argument, then passes a status byte to a further routine and returns it.
- `match_value_in_table`: Scans an eight-byte table in the global data for the given value and records the matching entry, or zero, in a field of a block obtained from a routine.
- `find_drive_entry_in_list`: Walks a linked list of drive entries, matching each either by name text or by a comparison routine.
- `calculate_string_length_2`: Returns the number of bytes before the null terminator of a string.
- `copy_cstring_to_buffer`: Copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers and writing each byte up to and including the zero byte.
- `convert_character_to_uppercase`: Returns the uppercase form of a single byte when it is a lowercase letter, otherwise returns the byte unchanged.
- `compare_strings_ignoring_case`: Compares two null-terminated strings a character at a time, converting each character through a routine before comparing.
- `terminate_string_at_colon`: Advances through a string to the first colon or terminator and overwrites the colon with a zero byte, truncating the string there.
- `store_arg_and_check_result`: Stores its first argument in a one-word stack buffer and passes it with a constant and a length of one to a routine.
- `allocate_object_copy_capped_words`: Looks up a handle and allocates a 68-byte object, linking it to that handle and to a stored value.
- `invoke_indirect_function`: Loads a function pointer from a memory offset and executes it.
- `execute_indirect_function_14`: loads a function pointer from a global offset and calls it.
- `execute_indirect_function_15`: dereferences a pointer stored at a global offset and executes the target routine.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `execute_indirect_function_16`: performs an indirect call to a function pointer located at a global offset.
- `execute_indirect_function_17`: executes a routine via a pointer retrieved from a global memory location.
- `execute_indirect_function_18`: calls a routine using a pointer found at a certain offset in a global table.
- `execute_indirect_function_19`: resolves a function pointer from a global offset and jumps to it.
- `execute_indirect_function_20`: executes a routine located at a pointer stored in a global offset.
- `execute_indirect_function_21`: performs an indirect jump to a routine pointer stored in a global offset.
- `execute_indirect_function_22`: loads a routine address from a global offset and transfers control to it.
- `execute_indirect_function_23`: dereferences a pointer from a global offset and executes the resulting address.
- `execute_indirect_handler_4`: Executes a function pointer loaded from a global variable offset.
- `execute_indirect_function_pointer_9`: Loads a function pointer from a calculated offset within a global table and executes it.
- `call_indirect_function_3`: loads a function pointer from a global table with a fixed offset and calls it.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_10`: loads a function pointer from a fixed offset within a global table and executes it.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_11`: fetches a function pointer from an offset within a global table and executes it.
- `execute_indirect_function_27`: Loads a function pointer from an offset within a global table and executes it.
- `execute_indirect_function_26`: fetches a function pointer from a global table and executes it.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_tracks_with_progress`: Allocates an interrupt server and track buffers, then formats the disk track by track, filling each buffer with DOS-style block headers, writing it out and verifying each track while printing a cylinder progress message.
- `run_device_command_sequence`: Calls a device routine through a stored request block with the constants twenty and thirty, then with one, then forwards its argument together with a value from another routine.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer retrieved from a global structure and then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_address`: Stores a single byte from a register into an address stored in another register.
- `divide_32bit_with_remainder_3`: Divides a 32-bit value by a divisor using the same shift-and-subtract and 16-bit division paths as the other division routine, returning the remainder and quotient as a pair of longwords.
- `shift_subtract_divide_loop`: Runs sixteen shift-and-subtract steps that shift a 32-bit dividend left one bit at a time and subtract the divisor from its high half whenever that half reaches it, setting a quotient bit.
- `multiply_32_bit_integers`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products and combines the results.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_address_2`: Stores a single byte from a register into a memory location pointed to by a register.
- `allocate_and_initialize_block`: Allocates a signal bit and, if that succeeds, a 34-byte block whose fields are filled from the arguments, the signal number and a value from a routine.
- `reset_object_and_recurse`: Recursively processes a linked structure when a pointer field is non-zero, sets a byte field and a four-byte field to all ones, then calls another routine on a byte field value and again with a constant second argument.
- `forward_with_constant_forty_eight`: Passes its argument to a routine together with the constant forty-eight, then returns.
- `forward_argument_unchanged`: Passes its single argument straight through to a second routine and returns.
- `init_trashcan_library_state`: Stores a sentinel value in a field of a task-level structure while it looks up a value, then restores it and sets several global fields from fixed constants and the looked-up block.
- `execute_and_get_result`: Calls a routine with certain arguments and returns a value or a modified byte based on the return status.
- `execute_indirect_function_24`: Loads a function pointer from a global table and executes it.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores a pointer into a memory location, increments that pointer, clears a subsequent word, and stores the original pointer address into a third location.
- `store_size_and_pointer`: If the pointer argument is non-null, allocates a block of the size given and, when that succeeds, stores the constant five in a byte, the size word and the pointer argument into fixed offsets.
- `reset_node_and_free`: When the pointer is non-null, marks a byte and two longword fields as released with all-ones values, then passes the pointer and its stored size word to a routine and returns that result.
- `execute_indirect_function_25`: loads a function pointer from a global table and jumps to that address or calls the routine pointed to by that address.
- `invoke_indirect_function_2`: Loads a function pointer from a location relative to a global pointer and executes it.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_string`: Parses a string from a command line, handling quoted arguments and substituting certain characters.
- `call_supervisor`: Calls the Supervisor routine of the exec library through the stored base, with no arguments set up here.
- `call_exit_intr`: Calls the ExitIntr routine of the exec library through the stored base, with no arguments set up here.
- `call_exec_switch_2`: Invokes the exec library switch routine via an offset from a global pointer.
- `call_exec_dispatch_2`: Invokes the exec library dispatch routine via an offset from a global pointer.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Calls the MakeFunctions routine of the exec library through a stored base, with no arguments set up here.
- `call_find_resident`: Calls the FindResident routine of the exec library through a stored base pointer and returns its result.
- `call_enable`: Calls the Enable routine of the exec library through the stored base, re-enabling interrupts, with no arguments set up here.
- `call_exec_alloc_entry`: Calls the exec library's AllocEntry function through a library base held in a global variable, leaving the result for the caller.
- `start_cli_session`: Starts a command-line interpreter session by opening a new console window and running the CLI and Shell startup scripts named in its text.
- `build_node_chain_from_list`: Walks a list of value records, allocating an eight-byte node for each one, storing the record's value in it and linking the nodes in order.
- `free_node_chain`: Walks a linked list of nodes, releasing each node's payload and then the eight-byte node itself, until the list ends at a zero link.
- `parse_signed_decimal`: Reads a signed decimal integer from a null-terminated string, accepting an optional leading plus or minus sign and stopping at the first non-digit.
- `call_library_entry_198`: Loads a library base from a global variable and calls the library function 198 bytes below that base, passing on the caller's arguments.
- `call_library_entry_210`: Loads a library base from a global variable and calls the library function 210 bytes below that base, passing on the caller's arguments.
- `call_library_entry_294`: Loads a library base from a global variable and calls the library function 294 bytes below that base, passing on the caller's arguments.
- `call_library_entry_414`: Loads a library base from a global variable and calls the library function 414 bytes below that base, passing on the caller's arguments.
- `call_library_entry_552`: Loads a library base from a global variable and calls the library function 552 bytes below that base, passing on the caller's arguments.
- `call_library_entry_78`: Loads a library base from a global variable and calls the library function 78 bytes below that base, passing on the caller's arguments.
- `call_library_entry_90`: Loads a library base from a global variable and calls the library function 90 bytes below that base, passing on the caller's arguments.
- `call_library_entry_96`: Loads a library base from a global variable and calls the library function 96 bytes below that base, passing on the caller's arguments.
- `divide_32bit_with_remainder`: Divides one 32-bit unsigned value by another and returns the quotient and remainder as a pair.
- `multiply_32bit_integers`: Multiplies two 32-bit values using four 16-bit partial products and keeps the low 32 bits of the product.
- `negate_result_on_flag`: Calls a routine and returns its first result, negated when its second result is non-zero.

### Disk-Validator

- `scan_disk_bitmap_flags`: Main routine of the Disk-Validator program.

### Ram-Handler

- `copy_memory_then_call_target`: Program entry that copies a block of memory through the exec library's copy routine, reached via a library base in a global variable, then makes a second indirect call.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_BLIT

- `release_blit_state_slot`: Runs a fixed sequence of library calls, including a memory free, repeated over three passes.
- `decrement_until_minus_one`: Decrements a 16-bit counter register until it reaches minus one, then returns.
- `decode_bitstream_passes`: Runs seven passes of a bit-stream decoder that reads variable-length codes and uses them to copy or add values within a buffer.
- `increment_pointer_address`: Increments a pointer or a memory address by a certain value.
- `decode_bitstream_byte_loop`: Bit-stream decoder working on byte-sized values: it reads variable-length codes from the input and copies or adds values within the output buffer in a loop.
- `copy_decoded_runs_to_output`: Copies decoded bytes into an output buffer four at a time, taking run lengths from the same bit stream as the decoder, and stops when a counter reaches zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### d

- `stub_bad_instruction_data`: The entry of a keymap data file that the decompiler decodes as instructions.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_CPU

- `release_state_slot_after_calls`: Runs a fixed sequence of library calls, including a memory free, repeated over three passes.
- `decrement_until_minus_one_2`: Decrements a 16-bit counter register until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_until_minus_one_3`: Decrements a local variable until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon_install

- `initialize_game_state`: The routine calls another routine several times and then runs a small loop that invokes a function pointer taken from a global variable.
- `decrement_until_minus_one_4`: Decrements a local word-sized variable until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `compare_and_call_routine`: Compares a longword at the address in a0 with a constant value; if they match, it branches to another routine, otherwise it loads a word and a byte from a0 into registers, updates two global pointers with the current stack pointer, sets a0 to point to a global variable, calls another routine, and then branches or jumps based on the resulting sign bit.
