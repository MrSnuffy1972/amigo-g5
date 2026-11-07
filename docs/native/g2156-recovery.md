# Game 2156 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02156). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 55 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 328 of 328 named (0 author symbols), 328 of 328 described (65 by rule, 96 by the local model) |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Keymap | 1 | 13 | 1 |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_BLIT | 11 | 253 | 7 |
| d | 1 | 13 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_CPU | 10 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon | 8 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon_install | 7 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `call_through_address_register` (More): Calls a routine with a global argument, calls a second routine, then jumps through a function pointer held in address register A3. Kind: indirect call/control flow obfuscation. Pass path: the instruction at the address held in A3.

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Keymap

- `stub_bad_instruction_data`: The bytes do not decode to a valid routine.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_table_block_then_call`: Copies sixteen longwords from a fixed table into the caller's buffer when a buffer pointer is given, then calls another routine.

### Install

- `install_command_entry`: Looks up the largest existing entry number in a linked list, allocates a new entry through the system, writes a header followed by a sequence of marker values, and copies a function table into the new entry.
- `copy_memory_block_and_call_8`: Copies sixteen bytes from a memory location to a destination pointed to by a stack address, then executes a function via a pointer.

### Else

- `expand_and_populate_table_2`: Finds the maximum value in a linked list and uses it to expand a table structure by writing a header and a sequence of incrementing values.
- `copy_memory_block_4`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Delete

- `traverse_linked_list_and_update_table`: Traverses a linked list to find the maximum value in a specific field and then updates a table structure.
- `copy_memory_block_and_call_10`: Copies sixteen bytes from a memory location to a destination pointer using a loop, then calls another routine.

### Avail

- `run_avail_command`: Program entry point for the Avail command.
- `cleanup_program_and_reply`: Stores a result code in the control block, then closes the open file handle when present, frees the memory block, closes the library and replies to the owning message.
- `sum_memory_region_sizes`: Adds up the sizes of every memory region in a linked list whose attribute flags match the given mask, and returns the total.
- `print_memory_availability_table`: Prints the memory availability table for the Avail command, showing available, in-use, maximum and largest-block figures for each memory type.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_call_with_stack_slot`: Passes two arguments to another routine together with the address of a stack slot, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_hook_then_measure_string`: Calls a function pointer loaded from a global table, then counts the characters of a stack buffer up to its terminating zero byte.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `add_disk_buffers_command`: Command entry point for adding disk buffers to a drive.

### Path

- `traverse_list_and_populate_table_4`: Traverses a linked list to find the maximum value in a specific field, then uses that value to initialize a table structure.
- `copy_memory_block_and_call_11`: Copies sixteen bytes from a memory location to a destination address via a loop, then executes another routine.

### Resident

- `traverse_list_and_expand_table`: Traverses a linked list to find the maximum value in a specific field and then expands a table by writing a header and copying data blocks.
- `copy_memory_block_and_call_12`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then executes another routine.

### Assign

- `traverse_list_and_expand_table_2`: Traverses a linked list to find the maximum value in a specific field and then expands a table by writing a header and copying data blocks.
- `copy_memory_block_and_call_13`: Copies sixteen bytes from a memory location offset from a pointer into a destination buffer and then calls another routine.

### EndCLI

- `expand_and_populate_table`: Traverses a linked list to find a maximum value, then expands a table by writing a header and a sequence of incrementing values.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location offset from a register to a destination pointer, then calls another routine.

### Rename

- `find_max_and_populate_table`: Traverses a linked list to find the maximum value in a specific field and then populates a table with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_2`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer.

### Dir

- `traverse_list_and_update_table`: Traverses a linked list to find the maximum value in a specific field and then updates a table structure.
- `copy_memory_block_and_call_3`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then calls another routine.

### NewCLI

- `traverse_list_and_populate_buffer`: Traverses a linked list to find the maximum value in a specific field and then populates a destination buffer with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_4`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by the stack.

### NewShell

- `traverse_list_and_populate_buffer_2`: Traverses a linked list to find the maximum value in a specific field, then uses that value to calculate a buffer size and populate a table with a sequence of incrementing values.
- `copy_memory_block_and_call_5`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then executes another routine via a function pointer.

### Echo

- `traverse_list_and_populate_buffer_3`: Traverses a linked list to find the maximum value in a specific field and then populates a destination buffer with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_6`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then calls another routine.

### cls

- `open_dos_library`: Opens the DOS library through the exec library base, runs two library calls on it, and returns zero when the second call's result is not negative.

### Execute

- `traverse_list_and_update_table_2`: Traverses a linked list to find the maximum value in a specific field, then updates a table entry with this value and populates it with a sequence of incrementing longwords.
- `copy_memory_block_and_call_7`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by the stack, then executes another routine.

### Copy

- `find_max_and_populate_table_2`: Traverses a linked list to find the maximum value in a specific field, then writes this value and a sequence of incrementing integers into a table.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer.

### Makedir

- `traverse_list_and_populate_table`: Traverses a linked list to find the maximum value in a specific field, then uses that value to calculate a size and populate a table with a header and subsequent data.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset by a value in a register to a destination pointed to by a stack pointer.

### List

- `traverse_list_and_populate_table_2`: Iterates through a linked list to find the maximum value in a specific field, then uses that value to initialize and populate a structured table.
- `copy_memory_block_3`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### CD

- `traverse_list_and_update_table_3`: Iterates through a linked list to find the maximum value in a specific field, then updates a table entry with this value and populates it with a sequence of incrementing longwords.
- `copy_memory_block_and_call_9`: Copies sixteen bytes from a memory location offset from a register to a destination address on the stack, then executes another routine.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `traverse_list_and_populate_table_3`: Walks a linked list to find the maximum value in a specific field and then populates a table with a header and a sequence of incrementing values.
- `copy_multiple_words`: Copies a fixed number of words from a source pointer to a destination pointer located on the stack.

### LoadWB

- `parse_command_line_and_run`: Program entry point.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_workbench_task`: Starts the Workbench task.
- `compare_strings_ordering`: Compares two zero-terminated strings byte by byte and returns minus one when the first sorts before the second, one when it sorts after, and zero when they are equal.
- `build_linked_list_from_table`: Walks a table of entries, allocating a small node for each one, storing a value from the entry in the node, and chaining the nodes together.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_offset_84`: Calls a function via a pointer located at an offset from a library base address.
- `execute_indirect_function`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `check_workbench_release_string`: Handles a version text buffer: when a fixed character position holds a two, it writes a dot-padding field after the version, and otherwise it compares the buffer with the fixed release text and stops at the first mismatch.
- `write_version_padding`: Writes a period followed by four spaces into a fixed field of the buffer it is given.
- `copy_cstring_to_buffer`: Copies a zero-terminated string byte by byte from the source into the destination buffer, including the terminating zero.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends with a return.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `dispatch_task_queue`: Calls the exec dispatch function through the stored library base, which selects the next task from the ready queue to run.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_through_pointer`: Writes the low byte of the data register to the memory location held in the address register, then returns.

### Version

- `run_version_command`: Program entry point for the Version command.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses the command arguments, opens the named library or device, reads its version and revision from the library base, and compares them with the user-supplied minimums.
- `call_fixed_code_then_forward`: Calls a routine with its first argument and a fixed code, then passes the second argument on to a further routine and returns.
- `parse_signed_decimal`: Reads a signed decimal number from a string by skipping an optional leading plus or minus sign and accumulating digits until a non-digit character, then applies the sign to the result.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `measure_cstring_length`: Counts the bytes that come before the terminating zero byte of a string and returns that count.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatch_offset`: Calls a routine using a pointer retrieved from a global variable with a specific offset.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `allocate_memory_block`: Calls the exec memory allocation function through a stored library base pointer, passing its arguments through and returning the address of the block.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414`: Loads a library base pointer from a global variable and calls the library function at a fixed negative offset from that base, returning its result.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Fetches a function pointer from a location calculated by an offset from a global variable and executes it.
- `invoke_and_verify_stack_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over the local stack buffer until a null terminator is reached.
- `store_byte_via_pointer`: Stores a byte from a register into a memory location pointed to by another register.

### SetMap

- `parse_command_string`: Parses a command string by splitting it into tokens based on whitespace and handling quoted substrings.
- `update_state_and_trigger_callbacks`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `execute_indirect_function_3`: Loads a function pointer from a global variable and executes it.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `reschedule_tasks`: Calls the exec reschedule function through the stored library base, which lets the scheduler reconsider which task should run next.
- `free_saved_blocks`: Releases each recorded block in a counted table of saved pointers, then releases the remaining saved blocks and the block passed in, using a release routine for each.
- `save_arguments_and_run`: Stores its two arguments in global variables, then calls a routine with fixed size and count values and a final routine with a fixed code.
- `load_keymap_for_console`: Loads a keyboard keymap for the console from the keymaps directory or from the keymap resource.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_5`: Fetches a function pointer from a location relative to a global variable and executes it.
- `execute_indirect_function_6`: Fetches a function pointer from a specific offset within a global structure and executes it.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `execute_indirect_function_7`: Fetches a function pointer from a location within a global table and executes it.
- `execute_indirect_call`: Calls a function pointer retrieved from an offset within a structure pointed to by a global variable.
- `call_library_vector_minus_444`: Loads an Amiga library base pointer from a global variable and calls the library function at offset minus 444 from it.
- `call_library_vector_minus_552`: Loads an Amiga library base pointer from a global variable and calls the library function at offset minus 552 from it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_minus_366`: Loads an Amiga library base pointer from a global variable and calls the library function at offset minus 366 from it.
- `call_library_vector_minus_384`: Loads an Amiga library base pointer from a global variable and calls the library function at offset minus 384 from it.
- `call_library_vector_minus_96`: Loads an Amiga library base pointer from a different global variable and calls the library function at offset minus 96 from it.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_argument_to_library_stub`: Passes its one incoming argument, a library base pointer read from a global variable, and the address of its own stack argument to the library call stub at offset minus 132.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `measure_stack_string_and_forward`: Calls a function pointer stored in a global variable, then counts the characters of a 140-byte stack buffer up to its null terminator.
- `store_byte_through_a3_pointer`: Writes the low byte of data register D0 to the memory address held in address register A3 and returns.

### More

- `parse_command_line_and_start`: Program entry that opens the dos library and allocates a control block, then splits the command line into argument strings, honoring double quotes and asterisk escapes.
- `release_file_and_library_state`: Fetches a state pointer from a global library base, stores the given value in it, and conditionally closes a file, closes a library, frees memory and replies to a message based on flags in that state.
- `run_file_pager_session`: Sets up a pager session for one file, allocating its control block, opening the file and an output window, and choosing the display mode from the arguments.
- `reschedule_four_global_slots`: Calls the reschedule library stub four times, each time pairing a field of its structure argument with one of four consecutive global words.
- `scan_buffer_for_control_bytes`: Fills a buffer of 80 bytes through another routine, makes a second call with zero and minus-one arguments, then scans the bytes.
- `dispatch_pager_key_input`: Waits for a console input signal, reads one key and dispatches it through a large switch on character codes.
- `call_through_address_register`: Calls a routine with a global argument, calls a second routine, then jumps through a function pointer held in address register A3.
- `initialize_session_and_loop`: Sets a counter to 1 and clears a flag in its structure, stores a routine result in one field, and repeats another routine call while the counter stays positive and a size comparison holds.
- `update_entity_and_increment_counter`: Calls two routines on a buffer inside its structure and stores the second result in a field of that structure.
- `read_line_convert_formfeed`: Reads up to 128 bytes into a buffer, scans to the next newline or form-feed, and repositions the stream past that line.
- `run_step_loop_until_quit`: Runs a loop of 9, 13 or 14 steps, with the count chosen by flags in its structure, calling routines from a global table of pointers on each step.
- `search_stream_until_match`: Formats a search message from a flag-selected global string, then reads and tests the stream until a match is found or the attempt counter drops below one.
- `wrap_call_with_setup_teardown`: Runs a setup call, a call using its structure and a global, and an optional second global call.
- `call_pair_then_flagged_once`: Calls another routine with two arguments, then calls it again with its first argument alone only when a short flag at offset 98 in the structure is zero.
- `call_stream_then_window_bound`: Calls one routine with a stream handle field, a second field and minus one, then calls another routine with a window bound value read through a structure pointer.
- `call_stream_then_decrement_bound`: Calls one routine with a stream handle field, zero and one, then calls another routine with the window's bound value minus one.
- `display_text_lines_up_to_count`: Reads text in chunks of at most 128 bytes and counts newline characters until the requested number of lines is reached.
- `reschedule_with_structure_field`: Passes its second argument to another routine, then calls the reschedule stub with the structure's field at offset 16, a global address and its first argument.
- `mark_flag_then_forward`: Sets a short value in a linked structure to one when a field at offset 4 is zero, then calls the reschedule stub with the structure's field at offset 16 and a global address.
- `select_reschedule_arguments_by_flag`: Chooses its second and third arguments by a flag in its structure: either two global addresses, or a global address and a window bound plus one.
- `reschedule_with_flag_arguments`: Calls the reschedule stub with a structure field and either three global addresses or one global address and a window bound value, chosen by a flag in the structure.
- `call_twice_and_clamp_result`: Calls another routine twice with the same first argument and different trailing arguments, then returns the second result with any negative value replaced by zero.
- `consume_buffered_bytes`: Copies up to the requested number of bytes from the structure's buffer into the destination, refilling through another routine first when the buffer is short.
- `refill_read_buffer`: When the 4096-byte read buffer is exactly full, passes its pending bytes to another routine first.
- `update_scroll_window`: Computes a target scroll position from its offset argument, then compares it with the visible window range in its structure.
- `read_line_into_buffer`: Reads characters one at a time through another routine into a destination buffer, storing the newline and stopping there or after 127 characters.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination one at a time, including the terminating null byte.
- `convert_string_to_uppercase`: Converts each lowercase ASCII letter in a null-terminated string to uppercase in place by subtracting 32, and leaves all other characters unchanged.
- `parse_signed_decimal_string`: Reads an optional plus or minus sign followed by decimal digits from a string and returns the signed integer value.
- `call_library_if_pointers_set`: Calls a library routine with a pointer from its structure, its first argument and minus one, but only when that first argument and the structure's field at offset 4 are both nonzero.
- `close_pager_state_and_free`: Tears down a pager session by closing its file and window handles and restoring saved pointers, with a nonempty text argument and a flag causing input to be read until a newline.
- `call_with_single_value_buffer`: Gets a pointer from another routine, stores its argument in a one-slot stack buffer, and passes that buffer with code 994 and length one to a second routine.
- `allocate_block_and_copy_fields`: Allocates a 36-byte block and, if that succeeds, passes a value derived from its address to a library call through a stack buffer.
- `create_block_from_source_words`: Allocates a 68-byte object, copies a given number of words from a source array into it, and links it to a handle from another routine.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_exec_reschedule_vector`: Calls the Exec library Reschedule function through an exec base pointer kept in a global variable, at offset minus 48.
- `call_exec_switch_vector`: Calls the Exec library Switch function through an exec base pointer kept in a global variable, at offset minus 54.
- `call_exec_dispatch_vector`: Calls the Exec library Dispatch function through an exec base pointer kept in a global variable, at offset minus 60.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_exec_allocmem_vector`: Calls the Exec library AllocMem function through an exec base pointer kept in a global variable, at offset minus 198.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_minus_132`: Calls a library routine at offset minus 132 from a library base pointer held in a global variable.
- `call_library_vector_minus_138`: Calls a library routine at offset minus 138 from a library base pointer held in a global variable.
- `call_library_vector_minus_198`: Calls a library routine at offset minus 198 from a library base pointer held in a global variable.
- `call_library_vector_minus_210`: Calls a library routine at offset minus 210 from a library base pointer held in a global variable.
- `call_library_vector_minus_294`: Calls a library routine at offset minus 294 from a library base pointer held in a global variable.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_minus_366_2`: Calls a library routine at offset minus 366 from a library base pointer held in a second global variable.
- `call_library_vector_minus_372`: Calls a library routine at offset minus 372 from a library base pointer held in a global variable.
- `call_library_vector_minus_384_2`: Calls a library routine at offset minus 384 from a library base pointer held in a global variable.
- `call_library_vector_minus_414`: Calls a library routine at offset minus 414 from a library base pointer held in a global variable.
- `call_library_vector_minus_552_2`: Calls a library routine at offset minus 552 from a library base pointer held in a global variable.
- `call_library_vector_minus_624`: Calls a library routine at offset minus 624 from a library base pointer held in a global variable.
- `forward_with_stack_slot_pointer`: Passes its two arguments to another routine together with a pointer to a fixed slot 12 bytes into the caller's stack frame.
- `read_one_byte_or_minus_one`: Calls another routine to read one byte into a small stack buffer.
- `measure_stack_string_and_forward_2`: Calls a function pointer from a global variable to fill a 140-byte stack buffer, then counts the characters up to the null terminator.
- `store_byte_through_a3_pointer_2`: Writes the low byte of data register D0 to the memory address held in address register A3 and returns.
- `divide_32_bit_unsigned`: Divides a 32-bit unsigned value in D1 by the divisor in D2 and returns the quotient and remainder in two registers.
- `multiply_32_bit_integers`: Multiplies the 32-bit values in D0 and D1 from their 16-bit halves and returns the low 32 bits of the product.
- `call_adjacent_routine_and_return`: Calls one unnamed routine and returns, passing no arguments.
- `negate_result_on_flag`: Calls another routine and returns its first result, negated when the second result register is nonzero.
- `call_rawdofmt_from_global`: Calls the Exec RawDoFmt function through a function pointer stored at a fixed offset in a global variable.
- `store_byte_to_pointer`: Stores a byte from a register into a memory location pointed to by a register.
- `call_library_vector_minus_276`: Calls a library routine at offset minus 276 from a library base pointer held in a global variable.
- `allocate_signal_message_block`: Allocates a signal bit and, if that succeeds, allocates a 34-byte message block.
- `free_signal_message_block`: Marks a 34-byte signal message block as deleted, first calling another routine on it when its field at offset 10 is nonzero.
- `search_substring_within_line`: Searches a null-terminated line for a pattern string, stopping at a newline, and returns 1 if the pattern occurs as a substring and 0 otherwise.
- `initialize_empty_exec_list`: Initializes a list header in place: the first word points to the following word, the second word is cleared, and the last word points back to the header.

### Format

- `parse_command_line_and_start_2`: Program entry for the Format utility that opens the dos library and allocates a control block, then splits the command line into argument strings, honoring double quotes and asterisk escapes.
- `update_and_cleanup_state`: Retrieves a pointer from a global variable and stores a parameter into a specific slot within the pointed-to structure.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_exec_allocmem_vector_2`: Calls the Exec AllocMem function through an exec base pointer kept in a global variable, at offset minus 198.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `call_exec_switch_vector_2`: Calls the Exec Switch function through an exec base pointer kept in a global variable, at offset minus 54.
- `call_exec_forbid_vector`: Calls the Exec Forbid function through an exec base pointer kept in a global variable, at offset minus 132.
- `call_exec_dispatch_vector_2`: Calls the Exec Dispatch function through an exec base pointer kept in a global variable, at offset minus 60.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_exec_schedule_vector`: Calls the Exec Schedule function through an exec base pointer kept in a global variable, at offset minus 42.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_exec_allocabs_vector`: Calls the Exec AllocAbs function through an exec base pointer kept in a global variable, at offset minus 204.
- `call_exec_reschedule_vector_2`: Calls the Exec Reschedule function through an exec base pointer kept in a global variable, at offset minus 48.
- `format_disk_from_arguments`: Main routine of the Format utility: it reads the DRIVE, NAME and option arguments, checks the drive, and asks the user to insert a disk.
- `release_format_buffers_and_handles`: Releases the buffers and handles that the format routine allocated, calling a release routine for each field that is set.
- `report_disk_error_and_retry`: Performs a disk operation and loops while it returns error code 28 and the retry request succeeds.
- `report_disk_errors`: Checks a condition and either calls several routines with a global variable or calls a routine with a specific constant and a global variable.
- `store_matched_table_byte`: Searches an eight-entry global byte table for the given value and records the matching entry in a field of a structure obtained from another routine.
- `find_drive_list_entry`: Walks a linked list of drive entries until one matches the given number or pointer, marks that entry, and returns 1.
- `calculate_string_length`: Returns the number of characters in a null-terminated string before its terminator.
- `copy_cstring_to_buffer_2`: Copies a null-terminated string into a destination buffer one byte at a time, including the terminating zero byte.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32.
- `compare_strings_ignoring_case`: Compares two null-terminated strings one character at a time after converting each to uppercase.
- `truncate_string_at_colon`: Replaces the first colon in a string with a null terminator, cutting the string short.
- `call_with_single_value_status`: Passes its first argument in a one-slot stack buffer to another routine with code 31 and length one.
- `create_block_from_source_words_2`: Allocates a 68-byte object, copies up to seven words from a source array into it, and links it to a handle from another routine.
- `call_library_vector_minus_354`: Calls a library routine at offset minus 354 from a library base pointer held in a global variable.
- `execute_indirect_function_pointer`: Fetches a function pointer from a specific offset within a global data structure and executes it.
- `call_library_vector_minus_330`: Calls a library routine at offset minus 330 from a library base pointer held in a global variable.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_minus_450`: Calls a library routine at offset minus 450 from a library base pointer held in a global variable.
- `execute_indirect_function_8`: Fetches a function pointer from a global table and executes it.
- `call_library_vector_minus_456`: Calls a library routine at offset minus 456 from a library base pointer held in a global variable.
- `execute_indirect_function_9`: Fetches a function pointer from a specific offset within a global variable and executes it.
- `execute_indirect_function_10`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_vector_minus_336`: Calls a library routine at offset minus 336 from a library base pointer held in a global variable.
- `call_indirect_via_offset`: Fetches a function pointer from a global table using a fixed offset and executes it.
- `call_library_vector_minus_444_2`: Calls a library routine at offset minus 444 from a library base pointer held in a global variable.
- `execute_indirect_function_11`: Fetches a function pointer from a location within a global data structure and executes it.
- `call_library_vector_minus_366_3`: Calls a library routine at offset minus 366 from a library base pointer held in a global variable.
- `call_library_vector_minus_360`: Calls a library routine at offset minus 360 from a library base pointer held in a global variable.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_minus_306`: Calls a library routine at offset minus 306 from a library base pointer held in a global variable.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_indirect_via_pointer`: Calls a routine through a pointer stored at an offset within a global variable.
- `forward_argument_to_library_stub_2`: Passes its one argument to a library call stub along with a library base pointer from a global variable and the address of its stack argument.
- `call_library_vector_minus_348`: Calls a library routine at offset minus 348 from a library base pointer held in a global variable.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_tracks_and_verify`: Formats a disk track by track, building the track data in a buffer and writing it through the disk device, then verifies the tracks.
- `write_value_to_output_handle`: Calls two output routines with fixed arguments of 20, 30 and 1 on a handle taken from a structure, then passes a value from another routine through the same handle.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_via_pointer_2`: Stores a single byte from a data register into a memory location pointed to by an address register.
- `divide_32_bit_unsigned_2`: Divides a 32-bit unsigned value in D1 by the divisor in D2 and returns the quotient and remainder in two registers.
- `run_restoring_division_steps`: Performs a 16-step restoring division: each step shifts a 32-bit value left through the carry and subtracts the divisor from the high half when it fits, setting a quotient bit.
- `multiply_32_bit_integers_2`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products and combines the results.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer_3`: Stores a byte from a data register into a memory location pointed to by an address register.
- `allocate_and_initialize_block`: Calls a library function to allocate a memory block, fills in fields including a stored parameter and flag bytes, and calls further library functions, returning the block address or zero.
- `reset_object_and_recurse`: Recursively processes a linked structure when a pointer field is non-zero, sets a byte field to all ones and a four-byte field to all ones, then calls another routine on a byte field value and again with a constant second argument.
- `forward_argument_with_code_48`: Passes its one argument to another routine together with the constant 48.
- `forward_single_argument`: Passes its one argument on to another routine and returns nothing.
- `initialize_system_structures`: Initializes several global pointers and variables by retrieving values from a memory block, potentially setting them to default values if the block is null, and performs a series of function calls to manage these structures.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `call_library_vector_minus_240`: Calls a library routine at offset minus 240 from a library base pointer held in a global variable.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `allocate_node_with_type_and_pointer`: Allocates a node of the size given by its second argument.
- `reset_node_then_forward`: If the pointer is nonzero, sets the node's byte at offset 8 to 255 and writes minus one into two longword fields, then passes the node and the word at offset 18 to another routine and returns its result.
- `call_library_vector_minus_90`: Calls a library routine at offset minus 90 from a library base pointer held in a global variable.
- `call_library_vector_minus_78`: Calls a library routine at offset minus 78 from a library base pointer held in a global variable.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_and_start_3`: Program entry for the CLI utility that opens the dos library and allocates a control block, then splits the command line into argument strings, honoring double quotes and asterisk escapes.
- `call_exec_supervisor_vector`: Calls the Exec Supervisor function through an exec base pointer kept in a global variable, at offset minus 30.
- `call_exec_exitintr_vector`: Calls the Exec ExitIntr function through an exec base pointer kept in a global variable, at offset minus 36.
- `call_switch_task`: Calls a function pointer located at an offset from a global variable to perform a task switch.
- `call_dispatch_handler`: Executes a function pointer retrieved from a global variable via a specific offset.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_exec_makefunctions_vector`: Calls the Exec MakeFunctions function through an exec base pointer kept in a global variable, at offset minus 90.
- `call_exec_findresident_vector`: Calls the Exec FindResident function through an exec base pointer kept in a global variable, at offset minus 96.
- `call_exec_enable_vector`: Calls the Exec Enable function through an exec base pointer kept in a global variable, at offset minus 126.
- `call_exec_allocentry_vector`: Calls the Exec AllocEntry function through an exec base pointer kept in a global variable, at offset minus 222.
- `open_cli_window_and_run_startup`: Creates the shell's control block and a console window from a CON or NEWCON specification, then runs the s:CLI-Startup or s:Shell-Startup script when the argument is zero.
- `build_linked_node_chain`: Walks a list of entries, allocating an 8-byte node for each and storing a value from the entry in it, then links the nodes together with shifted pointers.
- `free_linked_node_chain`: Walks the linked node chain built earlier, releasing each node's secondary item through another routine and then freeing the node with its 8-byte size.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_a`: Calls a function pointer stored at a calculated offset within a global table.
- `call_library_function_b`: Invokes a function pointer by accessing a global variable with a specific offset.
- `call_library_function_c`: Performs a call to a function pointer derived from a global variable and an offset.
- `call_library_function_d`: Triggers a function pointer located at a specific offset from a global variable.
- `call_library_function_e`: Calls a function through a pointer found at a designated offset in a global variable.
- `call_library_function_f`: Executes a function pointer retrieved from a global variable using a specific offset.
- `call_library_function_g`: Invokes a function pointer by calculating an offset from a global variable.
- `call_library_vector_minus_96_2`: Calls a library routine at offset minus 96 from a library base pointer held in a global variable.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_3`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products.
- `negate_on_flag`: Calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it into a buffer.

### Ram-Handler

- `execute_indirect_call_2`: Fetches a function pointer from a global variable and executes it.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_BLIT

- `reset_blit_state_at_entry`: Program entry for AM2_BLIT that calls routines and a library function in a loop from 9 down to 6, then clears a word at a fixed location.
- `decrement_until_minus_one`: Decrements a 16-bit counter register in a loop until it reaches minus one, then returns.
- `unpack_stream_and_clear_cache`: Runs seven passes of bit-driven copying within a buffer, which looks like a decompression loop, then calls the Exec CacheClearU function when a 16-bit global field exceeds 36.
- `decode_bitstream_length_code`: Decodes a variable-length code from a packed bit stream using a small table of global bytes, and stores the result in a global counter.
- `expand_packed_bitstream_block`: Repeatedly reads length and distance codes from a packed bit stream and copies earlier output bytes within the buffer, updating a global count.
- `decode_distance_code_from_bits`: Decodes a distance value from a packed bit stream using a small table of global bytes to choose a bit count and base, and returns it in a register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### d

- `null_routine`: This routine is empty and contains no executable instructions.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_CPU

- `clear_cpu_state_at_entry`: Program entry for AM2_CPU that calls routines and a library function in a loop from 8 down to 5, then clears a word at a fixed location.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon

- `reset_game_state_at_entry`: Program entry for Ambermoon that calls routines and a library function in a loop from 6 down to 3, then clears a word at a fixed location.
- `decrement_d6_until_negative_one`: Decrements the value in register D6 until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon_install

- `reset_install_state_at_entry`: Program entry for Ambermoon_install that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_d6_to_negative_one`: Decrements the value in register D6 by one in a loop until the value reaches negative one.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `verify_and_dispatch_data`: Compares a longword at a pointer against a constant and branches if they match.
