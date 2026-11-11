# Game 2159 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02159). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 328 of 328 named (0 author symbols), 328 of 328 described (65 by rule, 130 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 240 of 240 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10234 of 10234 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 71290 |
| code | 32130 |
| data | 502798 |
| padding | 14950 |
| unused | 280064 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Keymap | 1 | 3 | 1 |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_BLIT | 11 | 253 | 7 |
| d | 1 | 3 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_CPU | 10 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon | 8 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon_install | 7 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `call_two_routines_then_jump` (More): Calls one routine with a global value and another with no arguments, then makes an indirect call through a pointer. Kind: checksum or integrity trap. Pass path: the jump to the function pointer (*unaff_A3).

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Keymap

- `null_routine`: The routine is empty and contains no executable instructions.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_to_buffer`: If a destination pointer is given, copies sixteen longwords from a fixed offset in a structure into it, then calls another routine.

### Install

- `install_max_sized_buffer_handler`: Finds the largest value stored in a linked list of entries, then, if a library call succeeds, fills a new buffer header with a pattern and copies a sixteen-entry block into it.
- `copy_memory_block_5`: Transfers a fixed block of sixteen bytes from a source offset to a destination address.

### Else

- `find_max_and_populate_buffer_13`: Traverses a linked list to find the maximum value in a specific field, then uses that value to calculate a buffer size and populate a structure with a header and data.
- `copy_memory_block_9`: Copies sixteen bytes from a source address to a destination address using a loop.

### Delete

- `find_max_and_populate_buffer_14`: Traverses a linked list to identify the maximum value in a specific field, then uses that value to initialize and populate a data buffer.
- `copy_memory_block_and_call_5`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then executes a function via a pointer stored in a register.

### Avail

- `run_program_with_task_setup`: Program entry point that allocates a task structure, opens the DOS library, and then calls the main routine with the command-line text.
- `finish_and_reply_message`: Stores a result in a task field, calls the exec CloseLibrary and FreeMem routines when their pointers are set, then replies to the message through the exec ReplyMsg routine.
- `sum_matching_region_sizes`: Walks a linked list of memory regions and adds up the sizes of those whose attribute word shares at least one bit with the given mask.
- `print_memory_usage_table`: Prints a memory usage report with columns for available, in-use, maximum and largest blocks, for chip, fast or total memory, using the referenced text template.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_with_stack_slot`: Forwards two arguments to another routine together with a pointer to a stack slot.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `format_stack_string_and_forward`: Calls the exec RawDoFmt routine through a stored base to fill a 140-byte stack buffer, then measures the resulting string.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `add_buffers_to_drive`: Looks up the drive named by the first argument in the DOS library and adds buffers to it, then prints how many buffers it now has.

### Path

- `find_max_and_populate_buffer_15`: Iterates through a linked list to find the maximum value in a specific field, then allocates or prepares a buffer based on that value.
- `copy_data_and_call_function_2`: Copies sixteen bytes of data from a memory location offset by a register to a destination pointed to by a stack pointer.

### Resident

- `find_max_and_populate_buffer_16`: Traverses a linked list to find the maximum value in a specific field, then initializes a buffer with a header and populates it by copying data from a source structure and a secondary source.
- `copy_memory_block_10`: Copies sixteen bytes of data from a memory location to a destination specified by a pointer on the stack.

### Assign

- `find_max_and_populate_buffer_17`: Traverses a linked list to find the maximum value in a specific field, then allocates or prepares a buffer by writing a header and a sequence of incrementing values.
- `copy_memory_block_and_call_6`: Copies sixteen bytes from a memory location offset from a register to a destination address, then executes a function pointer.

### EndCLI

- `find_max_and_populate_buffer`: Iterates through a linked list to find the maximum value in a specific field, then uses that value to calculate a buffer size and populate a structure with a header and data.
- `copy_data_and_call_function`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then calls a function via a pointer held in another register.

### Rename

- `find_max_and_populate_buffer_2`: Traverses a linked list to find the maximum value in a specific field, then initializes a buffer with a header and a sequence of incrementing values.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Dir

- `find_max_and_populate_buffer_3`: Iterates through a linked list to find the maximum value in a specific field, then allocates or prepares a buffer based on that value.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then calls another routine.

### NewCLI

- `find_max_and_populate_buffer_4`: Traverses a linked list to find the maximum value in a specific field, then uses that value to initialize a buffer with a sequence of incrementing longwords.
- `copy_memory_block_and_call_2`: Copies sixteen bytes from a memory location offset from a pointer to a destination address, then calls another routine.

### NewShell

- `find_max_and_populate_buffer_5`: Traverses a linked list to determine the maximum value in a specific field and then initializes a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Echo

- `find_max_and_populate_buffer_6`: Traverses a linked list to determine the maximum value in a specific field and then initializes a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_3`: Copies sixteen bytes from a memory location offset from a pointer to a destination address, then executes another routine.

### cls

- `open_dos_library`: Opens the DOS library through the exec base, makes two calls on it, and returns zero, or the result of a further call when the second call reports a negative value.

### Execute

- `find_max_and_populate_buffer_7`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_4`: Copies a sequence of sixteen bytes from a memory location offset by a register to a destination address on the stack, then executes a function pointer.

### Copy

- `find_max_and_populate_buffer_8`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_3`: Copies sixteen bytes from a memory location offset from a register to a destination address.

### Makedir

- `find_max_and_populate_buffer_9`: Traverses a linked list to find the maximum value in a specific field, then allocates or prepares a buffer by writing a header and filling it with incrementing values.
- `copy_memory_block_4`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer.

### List

- `find_max_and_populate_buffer_10`: Traverses a linked list to find the maximum value in a specific field, then initializes a buffer with a header and populates it by copying data from a source structure.
- `copy_memory_block_6`: Copies a sequence of sixteen bytes from a memory location offset by a register value to a destination pointer.

### CD

- `find_max_and_populate_buffer_11`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_7`: Copies sixteen bytes from a memory location offset from a register to a destination address.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_populate_buffer_12`: Iterates through a linked list to find the maximum value in a specific field, then uses that value to initialize a buffer with a sequence of incrementing longwords.
- `copy_memory_block_8`: Copies sixteen bytes from a memory location offset from a register to a destination address.

### LoadWB

- `parse_command_line_arguments`: Parses a command line string into individual arguments, handling quoted strings and converting escape sequences like newline and escape characters.
- `finish_and_reply_message_2`: Stores a result in a task field, calls the library cleanup routines when their pointers are set (including closing a file through the DOS Close routine), then replies to the message.
- `start_workbench_tool`: Starts a Workbench tool from a named resident module by checking that the required library is available, allocating a block, processing a name string, and starting the tool.
- `compare_strings`: Compares two null-terminated strings byte by byte.
- `build_resident_lookup_list`: Builds a singly linked list from a source list, allocating an 8-byte node for each entry and storing the result of a resident lookup for it.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_no_args`: Calls a library function through the stored base at a fixed negative offset, passing no arguments.
- `call_library_function`: Invokes a function located at a specific offset within a library base retrieved from a global variable.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `check_release_string_and_pad`: Checks a text buffer against the global release text.
- `write_dotted_blank_field`: Writes a dot followed by four spaces into a text buffer at a fixed offset from its start.
- `match_prefix_against_release_text`: Advances through a text buffer while it matches the global release text byte by byte, returning when the release text ends or a byte differs.
- `empty_routine`: The routine is empty and performs no observable operation before returning.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_task_dispatch_function`: Calls the exec Dispatch function through the stored library base, passing no arguments.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_through_pointer`: Writes the low byte of the first data register to the memory location held in the A3 pointer register.

### Version

- `parse_command_line_string`: Parses a string into individual tokens, handling quoted substrings and converting specific character sequences into newline or escape characters.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `report_library_version_and_usage`: Parses the command arguments and prints the version of a named library or device, together with the Kickstart and Workbench versions, or a usage message.
- `forward_two_arguments_in_sequence`: Passes its first argument with a fixed numeric code to one routine, then passes its second argument to another routine.
- `parse_signed_decimal`: Reads an optional plus or minus sign followed by decimal digits and returns the signed integer value.
- `compare_strings_2`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Counts the bytes of a null-terminated string up to, but not including, its terminating zero, and returns the count.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatch_function`: Calls a function through a pointer stored at an offset within a global table.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_function_offset_c6`: Calls a function using an offset from a pointer stored in a global table.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `execute_indirect_function`: Fetches a function pointer from an offset within a global table and executes it.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_indirect_function`: Fetches a function pointer from a location offset from a global variable and executes it.
- `invoke_and_checksum_stack`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a null-terminated buffer on the stack.
- `store_byte_to_pointer`: Stores a byte from a register into a location pointed to by another register.

### SetMap

- `parse_command_line_arguments_2`: Parses a command line string into individual tokens, handling quoted substrings and converting specific escape sequences into newline or escape characters.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_indirect_function_2`: Calls a function pointer stored at an offset from a global variable.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `reschedule_tasks_via_exec`: Calls the exec Reschedule routine through the stored library base, passing no arguments.
- `walk_saved_table_backwards`: Walks a table of saved pointers from the last entry backwards and passes each non-null entry to another routine.
- `save_arguments_and_run_setup`: Saves its two arguments into global fields, then calls another routine with a fixed set of numeric values, and finally calls it again with a fixed code.
- `load_keymap_from_arguments`: Reads a keymap identifier from the arguments or the icon tool types, opens the keymap resource, and loads the named keymap file from the keymaps directory.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_indirect_function_3`: Executes a routine located via a pointer within a global table.
- `call_indirect_function_4`: Performs an indirect call to a function address retrieved from a global variable.
- `call_indirect_function_5`: Jumps to a subroutine address found by offsetting a global pointer.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_indirect_function_6`: Invokes a function by dereferencing a pointer stored in a global variable.
- `call_indirect_function_7`: Redirects execution to a routine address held in a global data structure.
- `call_indirect_function_8`: Calls a subroutine through a pointer located at a specific offset in a global variable.
- `call_indirect_function_9`: Uses a pointer from a global variable to call another routine.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_function_10`: Executes a function through an indirect pointer stored in a global variable.
- `call_indirect_function_11`: Calls a routine by accessing a pointer within a global data structure.
- `call_library_vector_no_args_2`: Calls a library function through a global base at a fixed negative offset, passing no arguments.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_global_with_stack_slot`: Forwards a global value and its argument to another routine together with a pointer to a stack slot.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `invoke_and_checksum_stack_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over the local stack buffer until a null terminator is reached.
- `store_byte_indirect`: Stores a byte from a register into a memory location pointed to by another register.

### More

- `parse_command_string`: Parses a command string by splitting it into tokens and handling quoted substrings.
- `update_and_cleanup_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `page_through_file_in_window`: Displays a file one page at a time in a new window, showing how far through it the reader is and a quit prompt.
- `iterate_table_with_function`: Calls another routine four times, passing a value from a structure field together with each of four entries in a fixed global array.
- `validate_input_buffer`: Checks up to eighty bytes of a buffer against a set of accepted byte ranges.
- `wait_and_read_key_code`: Waits for an input signal with a timeout and returns zero when a termination flag is set.
- `call_two_routines_then_jump`: Calls one routine with a global value and another with no arguments, then makes an indirect call through a pointer.
- `initialize_and_run_page_loop`: Sets two state fields to their starting values, calls a routine to set up a result, then repeatedly calls another routine while a counter is positive, the position is before the end and a stop flag is clear.
- `update_entity_and_increment_counter`: Calls two routines on a structure's buffer, stores the second result in a field, and when a flag field is zero it increments a short counter reached through a pointer in the structure.
- `scan_buffer_for_line_end`: Scans a buffer up to its stored size, stopping at a newline or form-feed character, and records how many bytes were scanned.
- `run_pager_display_loop`: Repeatedly fills and draws up to nine to fourteen lines of text, reads a key, and stops when the key is q or the control-C code.
- `run_search_with_prompt`: Chooses one of two global strings from a flag, then runs a search step and loops while the result is zero, updating a counter.
- `run_handler_sequence_and_return`: Runs a fixed sequence of calls to another routine, passing a global and values from a parameter block, and returns the result of its main call.
- `call_then_conditional_follow_up`: Calls another routine with two arguments, then calls it again with one argument only when a short field in the parameter block is zero.
- `call_two_routines_with_bounds`: Calls one routine with a stored value and a fixed negative argument, then calls another routine with a field from a nested structure.
- `call_routines_with_range_end`: Calls one routine with a stored value, a zero and a one, then calls another with a field from a nested structure reduced by one.
- `iterate_text_buffer_lines`: Iterates over a text buffer line by line, counting lines and stopping at a line limit, a stop flag or a special character.
- `call_handler_then_forward_pointer`: Calls another routine with its second argument, then calls a further routine with a pointer taken from the structure together with a global value.
- `set_flag_and_forward`: When a field in the parameter block is zero, sets a short value in a linked structure to one.
- `select_and_forward_by_flag`: Chooses between two values depending on a flag in the parameter block, then calls another routine with a field from the block and the chosen values.
- `dispatch_handler_by_flag`: Calls another routine with a field from the block and either three copies of a global value or a field from a nested structure, depending on a flag.
- `clamp_negative_result`: Calls another routine twice with the same first argument, once with a flag of one and once with minus one, and returns the second result clamped to zero if it is negative.
- `consume_from_buffer`: Takes up to a requested number of bytes from a buffer whose size is recorded in a structure.
- `refill_buffer_to_full_size`: Moves any buffered bytes to the start of the buffer when it is full, resets the offset fields, then fills the remaining space of a 4096-byte buffer by calling another routine.
- `update_scroll_window_position`: Computes a target position from a selector and an offset, using the window's start, end and length fields.
- `read_line_from_buffer`: Reads characters one at a time from a source into a buffer, stopping after a newline or a limit of 127 characters, and adds a terminating zero.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination, including the terminating zero, and stops after copying the zero.
- `convert_string_to_uppercase`: Converts each lowercase ASCII letter in a null-terminated string to uppercase in place, leaving other bytes unchanged.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated string, accepting an optional leading sign and then decimal digits.
- `call_library_function_when_set`: Calls a library function with a fixed argument of minus one, but only when the given pointer and a stored base pointer are both non-null.
- `close_pager_window_and_cleanup`: Cleans up the pager window state by calling routines that release saved pointers and reset fields, reads characters until a newline code, and then re-enables and reschedules through library calls.
- `send_single_value_message`: Stores its argument in a one-element stack buffer and passes it, with a fixed numeric code, to a routine that uses a handle taken from a stored structure.
- `copy_block_fields_and_release`: Allocates a 36-byte block and passes it to a routine with a fixed code, then copies two fields from the block into the caller's structure and frees the block.
- `initialize_object_from_template`: Obtains a handle from a routine, allocates a 68-byte object, links its fields to the handle and a source block, and copies a count of longwords from the source into it.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule_offset`: Calls a routine located at a specific offset from a global pointer.
- `invoke_task_switch_hook`: Calls the exec Switch function through the stored library base, passing no arguments.
- `execute_indirect_function_2`: Fetches a function pointer from a memory location and executes it.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `allocate_memory_via_exec`: Calls the exec AllocMem function through the stored library base, passing no arguments from this code.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_indirect_function_12`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_3`: Fetches a function pointer from a global memory location and executes it.
- `execute_indirect_function_4`: Fetches a function pointer from a global table at a specific offset and executes it.
- `execute_indirect_function_5`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_indirect_pointer`: Executes a function pointer retrieved from a global variable with a specific offset.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_indirect_pointer_2`: Calls a subroutine located at an offset from a global variable.
- `call_indirect_pointer_3`: Performs an indirect call to a function address stored in a global table.
- `call_indirect_pointer_4`: Invokes a routine by dereferencing a pointer found at a global offset.
- `call_indirect_pointer_5`: Triggers a function call using an address stored within a global data structure.
- `call_indirect_pointer_6`: Calls a subroutine via a pointer located at a specific offset from a global variable.
- `call_indirect_pointer_7`: Executes a function through a pointer stored in a global variable.
- `forward_with_stack_slot_2`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `read_signed_byte_or_fail`: Reads one byte through a routine that takes a handle, a buffer and a count of one.
- `call_hook_then_measure_string`: Calls a function pointer loaded from a global table, then counts the characters of a stack buffer up to its terminating zero byte.
- `store_byte_through_pointer_2`: Writes the low byte of the data register to the memory location held in the address register, then returns.
- `divide_32bit_with_remainder`: Divides a 32-bit dividend by a 32-bit divisor and returns the quotient and remainder together.
- `multiply_32bit_integers`: Multiplies two 32-bit values using 16-bit partial products and returns the 32-bit product.
- `stub_self_call_gap`: Calls its own entry point and then returns, with no condition and no other work.
- `negate_result_on_flag`: Calls another routine and returns its result unchanged, or negated when a second value it leaves in an address register is non-zero.
- `call_global_table_entry`: Calls the function whose address is stored at a fixed offset inside a global data block, then returns.
- `store_byte_via_pointer`: Stores the contents of a data register into a memory location pointed to by an address register.
- `call_library_vector_276`: Loads a library base pointer from a global variable and calls the library function at a fixed negative offset from that base, returning its result.
- `create_message_port`: Allocates a signal bit, then allocates and initialises a message port structure with its type, signal bit and task fields set.
- `delete_message_port`: Removes a message port from its list when a linked field is set, marks its type and signal fields as deleted, frees the signal bit and releases the port's memory block.
- `find_pattern_in_text`: Searches a null-terminated text for a non-empty pattern string by comparing the pattern at each starting position.
- `initialise_list_header`: Sets up an empty list header in the given block: the first pointer refers to the field after it, the middle field is cleared, and the last pointer refers back to the header itself.

### Format

- `parse_command_line_and_run`: Program entry point.
- `cleanup_program_and_reply`: Stores a result code in the control block, then closes the open file handle when present, frees the memory block, closes the library and replies to the owning message.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `allocate_memory_block`: Calls the exec memory allocation function through a stored library base pointer, passing its arguments through and returning the address of the block.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `invoke_task_switch`: Calls the exec task-switch function through the stored library base, which lets other ready tasks run.
- `forbid_multitasking`: Calls the exec Forbid function through the stored library base, which stops task switching until multitasking is permitted again.
- `dispatch_task_queue`: Calls the exec dispatch function through the stored library base, which selects the next task from the ready queue to run.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `schedule_next_task`: Calls the exec scheduler through the stored library base to choose the next ready task to run.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `allocate_absolute_memory`: Calls the exec absolute memory allocation function through a stored library base pointer, passing its arguments through and returning the result.
- `reschedule_tasks`: Calls the exec reschedule function through the stored library base, which lets the scheduler reconsider which task should run next.
- `run_format_disk_command`: Main routine of the disk format command.
- `free_format_buffers`: Frees each of several format buffers that are still allocated, using their recorded sizes and clearing their pointers, then calls a final routine with the given argument.
- `translate_disk_error_code`: Runs a disk operation, retrying while the device reports one specific busy code, then classifies the returned error code and displays the matching message text.
- `report_format_status_message`: Selects one of the format-related messages depending on a flag, displays it, and returns the status code it was given.
- `match_byte_in_table`: Scans an eight-entry byte table for the given value and records the matching entry in a field of a structure returned by a routine call.
- `find_drive_entry`: Walks a linked list of drive entries, matching either by a name string or by a numeric identifier.
- `measure_cstring_length`: Counts the bytes that come before the terminating zero byte of a string and returns that count.
- `copy_cstring_to_buffer`: Copies a zero-terminated string byte by byte from the source into the destination buffer, including the terminating zero.
- `convert_char_to_uppercase`: Returns the character in upper case when it is a lowercase letter from a to z, and returns any other character unchanged.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings one character at a time after passing each character through a case conversion routine.
- `terminate_string_at_colon`: Advances through a string until it reaches a colon or the end, then writes a zero byte at that position so the string is cut at its first colon.
- `check_single_argument_result`: Stores one argument value in a small local array, passes it to a lookup routine with a fixed code, and returns a failure code when the lookup fails or a success code otherwise.
- `build_request_block`: Allocates a request block, sets up its self-linked list header, stores a pointer and a value, and copies up to seven words of arguments into it.
- `call_indirect_pointer_8`: Performs an indirect jump to a routine address stored in a global variable.
- `call_indirect_pointer_9`: Calls a function by dereferencing a pointer at a calculated offset from a global variable.
- `call_indirect_pointer_10`: Executes a routine through a pointer retrieved from a global variable.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `execute_indirect_call`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_call_2`: Calls a subroutine located at an offset within a global data structure.
- `execute_indirect_call_3`: Performs an indirect jump to a routine stored in a global table.
- `execute_indirect_call_4`: Loads a pointer from a global variable and branches to that address.
- `execute_indirect_call_5`: Executes a function by dereferencing a pointer found in a global table.
- `execute_indirect_call_6`: Branches to a subroutine address retrieved from a global data structure.
- `execute_indirect_call_7`: Calls a routine via a pointer stored at a specific offset in a global variable.
- `execute_indirect_call_8`: Retrieves a function address from a global table and transfers control to it.
- `execute_indirect_call_9`: Executes a subroutine through a pointer located in a global data structure.
- `execute_indirect_call_10`: Performs an indirect call to a routine address stored in a global table.
- `call_indirect_pointer_11`: Calls a function pointer stored at an offset within a global table.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_indirect_pointer_12`: Executes a routine located at a specific offset from a global variable.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_indirect_pointer_13`: Performs an indirect call to a function address retrieved from a global table.
- `forward_call_with_stack_args`: Forwards its argument, together with a stored value and the address of a stack slot, to another routine and then returns.
- `call_indirect_pointer_15`: Executes a routine via a pointer found at an offset in a global table.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_and_verify_tracks`: Main disk formatting loop.
- `run_device_operation_sequence`: Issues a short sequence of device calls: two calls with fixed constant arguments, then passes its argument through a routine and forwards the routine's result to a final device call.
- `invoke_and_checksum_stack_3`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over the local stack buffer until a null terminator is reached.
- `store_byte_indirect_2`: Stores a single byte into a memory location pointed to by a register.
- `divide_32_bit_integers`: Performs 32-bit division using a shift-and-subtract algorithm or a 16-bit remainder method.
- `divide_32bit_restoring_loop`: Performs a sixteen-step shift-and-subtract division, subtracting the divisor from the high half whenever it fits and leaving the quotient bits in the low half.
- `multiply_32_bit_integers`: Computes the 32-bit product of two 32-bit values using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_indirect_3`: Writes a byte from a register to an address held in another register.
- `allocate_and_initialize_block`: Calls a library function to allocate a memory block, fills in fields including a stored parameter and flag bytes, and calls further library functions, returning the block address or zero.
- `reset_object_and_recurse`: Recursively processes a linked structure when a pointer field is non-zero, sets a byte field to all ones and a four-byte field to all ones, then calls another routine on a byte field value and again with a constant second argument.
- `call_with_fixed_code_argument`: Forwards its argument to another routine together with a fixed constant code, then returns.
- `forward_argument_unchanged`: Passes its argument unchanged to another routine and then returns.
- `setup_trashcan_device`: Initialises the trash-can device for the command environment by storing default values in global fields, taking a copy of a process field around a library call, and calling a routine with the trash-can path text.
- `read_one_byte_or_fail`: Calls a routine to read one character into a one-byte buffer and returns that character as a signed value when the call succeeds, or minus one otherwise.
- `call_indirect_pointer_14`: Calls a subroutine using a pointer retrieved from a global data structure.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `allocate_node_with_fields`: When the first pointer is non-null, allocates a node through a routine and, if that succeeds, stores a byte value, a word argument and the pointer into fixed offsets of the node.
- `reset_node_and_release`: If the pointer is non-null, sets the node's status byte to all ones and clears two longword fields, then passes the node and a stored word to another routine and returns that result.
- `execute_indirect_function_6`: Fetches a function pointer from a memory location and executes it.
- `initialise_structure_from_table`: Calls the exec structure initialisation routine through the stored library base, which fills a memory structure from an initialisation table.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_string_2`: Parses a command line string into individual arguments, handling quoted strings and expanding specific character sequences.
- `call_supervisor_routine`: Calls the exec Supervisor function through the stored library base, which runs a routine in supervisor mode.
- `call_exit_interrupt`: Calls the exec ExitIntr function through the stored library base, which returns control from an interrupt handler.
- `call_switch_2`: Calls a function pointer located at a specific offset within a global structure.
- `execute_indirect_function_7`: Fetches a function pointer from a global table and executes it.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `make_library_functions`: Calls the exec MakeFunctions routine through the stored library base, which installs the function vector table of a library, and returns its result.
- `find_resident_module`: Calls the exec FindResident routine through the stored library base, which searches the resident module list for a module by name.
- `enable_multitasking`: Calls the exec Enable function through the stored library base, which re-enables interrupts after a matching disable call.
- `allocate_entry_table`: Calls the exec AllocEntry function through the stored library base, which allocates a table of memory entries and returns it.
- `start_cli_session`: Starts a command-line shell session.
- `build_node_chain`: Walks a linked list of descriptors, allocating a small node for each one, copying a value from each descriptor into its node, and linking the nodes together.
- `free_node_chain`: Walks a chain of nodes, releasing the resource stored in each node and then the node itself, until the chain ends.
- `parse_signed_decimal_3`: Reads a signed decimal number from a string by skipping an optional leading plus or minus sign and accumulating digits until a non-digit character, then applies the sign to the result.
- `execute_indirect_function_8`: Fetches a function pointer from a table using a fixed offset and executes it.
- `execute_indirect_function_9`: Fetches a function pointer from a memory location using an offset and executes it.
- `execute_indirect_function_10`: Fetches a function pointer from a specific offset within a global data structure and executes it.
- `execute_indirect_function_11`: Fetches a function pointer from an offset within a global structure and executes it.
- `execute_indirect_function_12`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_offset_function`: Calls a subroutine located at a specific offset from a global pointer.
- `invoke_pointer_offset`: Executes a routine by dereferencing a pointer found at a fixed offset from a global address.
- `run_offset_routine`: Performs an indirect jump to a function address stored at a specific offset within a global structure.
- `modular_inverse_32bit`: Performs a 32-bit modular reduction or a 16-bit modulo operation using repeated doubling and subtraction, returning a combined 32-bit result.
- `multiply_32_bit_integers_2`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `negate_on_flag`: Calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.

### Disk-Validator

- `validate_disk_bitmap`: Entry point of the disk validation tool.

### Ram-Handler

- `execute_indirect_jump`: Performs an indirect call through a pointer stored at an offset from a global variable and then executes an unrecovered jump.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_BLIT

- `reset_blitter_state_entry`: Runs a fixed sequence of routine calls and a library call in a loop, then clears a memory word.
- `busy_wait_countdown`: Busy-waits by decrementing a 16-bit counter from its starting value until it reaches minus one, then returns.
- `decompress_bitstream_passes`: Decodes a bit-packed compressed stream over seven passes: it reads variable-length codes, copies runs of earlier output, and adds values in place.
- `stub_forward_or_zero`: Two-instruction wrapper that calls a one-instruction routine merging zero into a register and returns its result.
- `decode_packed_stream_copy`: Second decoding stage of the same packed format: it reads codes from a bit buffer, copies or repeats bytes from earlier output into the destination, and returns when the stream is exhausted.
- `decode_packed_stream_tail`: Final stage of the packed stream decoder: it copies the remaining run bytes backwards into the destination and returns the first output longword.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### d

- `stub_bad_instruction_data`: The bytes do not decode to a valid routine.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_CPU

- `initialize_cpu_state_entry`: Runs a sequence of routine calls and a library call in a loop, then clears a memory word.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_d6_to_negative_one`: Decrements the value in register D6 until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon_install

- `initialize_game_state`: Runs a sequence of routine calls and a library call in a loop, then runs a small loop of function-pointer calls before clearing a memory word and swapping a table entry with a RAM word when it matches.
- `decrement_register_until_negative_one`: Decrements a 16-bit register value until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `compare_and_call_routine`: Compares a longword at the address in a0 with a constant value; if they match, it branches to another routine, otherwise it loads a value into d0 and returns.
