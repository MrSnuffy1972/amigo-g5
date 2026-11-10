# Game 2161 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a6]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02161). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 328 of 328 named (0 author symbols), 328 of 328 described (65 by rule, 130 by the local model) |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:Keymap | 1 | 13 | 1 |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:AM2_BLIT | 11 | 253 | 7 |
| d | 1 | 13 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:AM2_CPU | 10 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:Ambermoon | 8 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:Ambermoon_install | 7 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a6]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a6]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a6]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a6]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:Keymap

- `stub_bad_instruction_data_2`: Holds only bad instruction data from the file start, so the routine is a stub with no recoverable behaviour.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen long words from a fixed offset in a structure into a buffer whose pointer is passed on the stack, when that pointer is non-null, then calls a stored routine.

### Install

- `scan_max_and_fill_table`: Scans a linked list for the largest value in a field, then when a buffer is supplied writes that value and a run of words that step by two from a constant.
- `copy_memory_block_6`: Transfers a fixed block of sixteen bytes from a memory offset to a destination pointer.

### Else

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field, then updates a structure with this value and populates a buffer using a loop.
- `copy_memory_block_9`: Copies sixteen words from a source address to a destination address and then calls another routine.

### Delete

- `update_linked_list_and_copy_data_3`: Traverses a linked list to find a maximum value and then updates a structure with this value and a sequence of incrementing integers.
- `copy_memory_block_10`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Avail

- `run_program_with_task_state`: Program entry that allocates a state block and opens dos.library, then reads the task message port and current directory before calling two routines.
- `cleanup_program_task_state`: Stores its argument in a task slot, frees the first item and closes the second when they are non-null, then calls a final library routine.
- `sum_masked_entry_sizes`: Walks a linked list of entries and adds the difference between two stored values for each entry whose flag word shares a bit with the argument.
- `print_memory_usage_table`: Prints a memory usage table with type, available, in-use, maximum and largest columns for chip, fast or total memory, selecting the mode from the first argument string.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_with_stack_slot_pointer`: Passes its two arguments to another routine together with a pointer to a fixed stack slot and returns nothing.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `invoke_global_and_measure_string`: Calls a function pointer held in a global variable, then counts the length of a 140-byte stack buffer up to its null terminator.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `add_drive_buffers_command`: Implements a buffer-count command: it looks up the named drive through the library, prints the buffer count, and prints the usage text or an invalid device message when the lookup fails.

### Path

- `update_linked_list_and_buffer`: Iterates through a linked list to find the maximum value in a specific field, then updates a structure with this value and populates a buffer using a sequence of memory copies.
- `copy_memory_block_and_call_6`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack value.

### Resident

- `process_linked_list_and_update_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a structured buffer.
- `copy_memory_block_and_call_7`: Copies sixteen bytes from a memory location offset by a register value to a destination pointer, then executes another routine.

### Assign

- `find_max_and_populate_buffer_11`: Traverses a linked list to identify the largest value in a specific field and then writes a header and a sequence of incrementing values into a calculated memory location.
- `copy_memory_block_and_call_8`: Copies sixteen bytes from a memory location offset from a register to a destination pointer, then calls another routine.

### EndCLI

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field, then updates a structure with this value and populates it with a sequence of incrementing integers.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a destination address and then executes a function pointer.

### Rename

- `find_max_and_populate_buffer`: Traverses a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a buffer.
- `copy_memory_block_and_call_2`: Copies a fixed block of sixteen bytes from a memory location to a destination pointed to by a stack pointer, then calls another routine.

### Dir

- `find_max_and_populate_buffer_2`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### NewCLI

- `find_max_and_populate_buffer_3`: Iterates through a linked list to determine the maximum value in a specific field.
- `copy_memory_block_and_call_3`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then calls another routine.

### NewShell

- `find_max_and_populate_buffer_4`: Traverses a linked list to identify the maximum value in a specific field and then writes a header and a sequence of incrementing values into a buffer.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Echo

- `find_max_and_populate_buffer_5`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_4`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then calls another routine.

### cls

- `open_dos_and_check_result`: Opens dos.library, calls two of its functions, and calls a third when the second result is negative, otherwise it returns zero.

### Execute

- `find_max_and_populate_buffer_6`: Iterates through a linked list to find the maximum value in a specific field, then writes a header and a sequence of incrementing values into a buffer.
- `copy_memory_block_3`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer.

### Copy

- `find_max_and_populate_buffer_7`: Traverses a linked list to identify the maximum value in a specific field and then writes a header and a sequence of incrementing values into a buffer.
- `copy_memory_block_4`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Makedir

- `find_max_and_populate_buffer_8`: Traverses a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a buffer.
- `copy_memory_block_5`: Copies sixteen bytes from a source location to a destination location.

### List

- `find_max_and_populate_buffer_9`: Traverses a linked list to identify the maximum value in a specific field and then writes a header and a sequence of incrementing values into a buffer.
- `copy_memory_block_and_call_5`: Copies a fixed block of sixteen bytes from a memory location to a destination address and then calls another routine.

### CD

- `find_max_and_populate_buffer_10`: Traverses a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a buffer.
- `copy_memory_block_7`: Copies a sequence of sixteen bytes from a memory location offset by a register value to a destination address.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_update_list`: Traverses a linked list to find the maximum value in a specific field and then updates a structure with this value and a sequence of incrementing words.
- `copy_memory_block_8`: Copies a sequence of sixteen words from a memory location to a destination pointed to by a stack pointer.

### LoadWB

- `start_loadwb_with_parsed_arguments`: Program entry that allocates a state block, opens dos.library and the NIL: stream, reads the task's message port and current directory, then calls two routines.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_workbench_tool_process`: Starts a Workbench tool from its resident tag and reports a cannot-start error when that fails.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte and returns minus one when the first differing byte is smaller, one when it is greater, and zero when the strings are equal.
- `build_node_list_from_table`: Walks a table of pointers, allocating a node for each entry, copying a value from the table into the node and linking the nodes into a new list.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `invoke_library_entry_132`: Calls the library function at a fixed negative offset from a global library base, with no arguments and no result handling in the shown code.
- `call_library_function`: Calls a function through a pointer located at a specific offset within a global structure.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `pad_release_text_field`: Checks a buffer for a release-number marker and, when the marker matches, writes dot and space padding into the following characters while comparing against a global text.
- `write_dot_space_padding`: Writes a dot followed by four spaces into five consecutive bytes of a buffer, starting at a fixed offset.
- `walk_global_text_against_buffer`: Steps through a global text string one byte at a time alongside a buffer and returns when the global string reaches its null terminator.
- `stub_empty_return`: Contains only a return and performs no operation.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_dispatch_function`: Calls the exec Dispatch function through a stored library base.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_low_byte_at_pointer`: Writes the low byte of the data register to the memory address held in the address register and returns.

### Version

- `parse_command_line_arguments`: Reads the command line string and splits it into individual arguments.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `forward_pair_with_constant`: Passes its first argument to another routine together with the constant 128, then passes its second argument to another routine.
- `parse_signed_decimal_string`: Parses an optional leading plus or minus sign followed by decimal digits from a null-terminated string and returns the signed value.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length_2`: Returns the number of bytes before the null terminator of a string by stepping forward until a zero byte is found.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatch_function`: Calls a function pointer retrieved from an offset within a global table.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_function_one`: Calls a function pointer located at a specific offset within a global table.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_function_two`: Calls a function pointer located at a different offset within a global table.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function_three`: Calls a function pointer located at a third offset within a global table.
- `invoke_and_checksum_stack`: Invokes a function pointer stored in a global variable and computes a rolling checksum over the local stack buffer until a null terminator is reached.
- `store_register_value`: Stores the contents of a register into a memory location pointed to by another register.

### SetMap

- `parse_command_line_arguments_2`: Parses a command line string into individual tokens, handling quoted strings and converting specific escape sequences into newline or escape characters.
- `update_state_and_trigger_callbacks`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `execute_dispatch_handler`: Calls a function pointer retrieved from a global variable with an offset applied.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `invoke_reschedule_function`: Calls the exec Reschedule function through a stored library base.
- `release_counted_table_entries`: Walks a counted table of pointers from the end, releasing each non-null entry after converting it to a normal pointer, then releases two further global items when they are set.
- `store_arguments_and_configure`: Stores its two arguments in global fields, then calls a routine with a set of constants and a second routine with a further constant.
- `load_keymap_with_error_messages`: Parses arguments and loads the keymap named in the icon tool types, opening the console and icon libraries and the keymap resource.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_indirect_function`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function`: Fetches a function pointer from a location offset from a global variable and executes it.
- `execute_indirect_function_2`: Fetches a function pointer from a global table and executes it.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_indirect_function_2`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_3`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_indirect_function_3`: Fetches a function pointer from a global variable and executes it.
- `call_indirect_function_4`: Fetches a function pointer from a global table and executes it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_function_5`: Fetches a function pointer from a table and executes it.
- `call_indirect_function_6`: Fetches a function pointer from a global variable and executes it.
- `call_indirect_function_7`: Calls a function through a pointer located at a specific offset from a global variable.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_global_and_stack_pointer`: Passes a global value and its argument to another routine together with a pointer to a fixed stack slot, then returns nothing.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_indirect`: Stores a byte from a register into a memory location pointed to by another register.

### More

- `parse_command_line_arguments_3`: Parses a string of command-line arguments into a structured list of pointers and strings.
- `update_state_and_trigger_callbacks_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `display_file_pager_screen`: Shows a file in a paged window and prints the filename request, a percentage indicator, end-of-file and binary-file warnings, and the quit hint.
- `iterate_table_with_function`: Calls another routine four times, each time with a field from a structure and one entry of a four-entry global array, then returns.
- `validate_input_buffer`: Prepares an 80-byte buffer through two routine calls, then scans its bytes and stops at the first byte outside the permitted value ranges.
- `read_pager_key_command`: Waits for input in a loop that also checks a signal mask, then maps the key code to a pager command value.
- `invoke_editor_from_environment`: Calls a routine with a global text argument, then invokes a stored function pointer.
- `initialize_pager_state`: Sets the pager state fields, runs a routine on the structure and stores its result, then conditionally calls further routines based on structure fields.
- `update_page_counter_after_scan`: Calls two routines with arguments derived from the structure, stores the second result in a field, and increments a short counter in a nested block when a flag field is zero.
- `parse_buffer_content`: Reads up to 128 bytes into a buffer and scans for a newline or form-feed character, stopping there.
- `show_page_lines_until_quit`: Computes the number of lines for the page from structure fields, prints table entries while the position is below the limit, reads a key and stops when the q key or Ctrl-C is pressed.
- `handle_search_loop`: Uses a flag in the argument block to choose a global variable, repeatedly calls a search routine while showing the searching message, and increments a counter until a result is found.
- `initialize_handler`: Runs a sequence of calls to another routine using arguments from a parameter block and a global variable, optionally repeating one call, and returns the result of one of those calls.
- `check_flag_and_call`: Calls another routine with two arguments, then calls it again with the first argument alone when a short flag field in the structure is zero.
- `update_window_bounds_calls`: Calls a routine with two fields from the structure and a fixed argument, then calls another routine with a value read from a nested structure.
- `update_text_window_range`: Calls a routine with a field, zero and one, then calls another routine with a value taken from a nested structure minus one.
- `scroll_text_buffer_lines`: Reads up to 128-byte blocks from a text buffer and calls routines for each line, counting lines until the requested number is reached or a flag stops the loop.
- `dispatch_with_structure_pointer`: Calls a routine with a structure pointer, then calls another routine with a pointer read from that structure, a global variable and the first argument.
- `set_flag_and_forward`: When a field is zero it sets a short counter in a nested structure to one, then calls another routine with global values.
- `dispatch_by_flag`: Selects two pointers according to a flag in the parameter block, using globals or a table entry, and calls another routine with them.
- `dispatch_handler_by_flag`: Reads a flag from the parameter block and calls another routine with either three global references or a value from a table entry.
- `clamp_negative_result`: Calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `copy_buffer_chunk_and_advance`: Limits a requested count to the remaining size in a structure, copies that many bytes from the buffer at the current offset into the destination, then advances the offset by the copied count.
- `refill_buffer_from_source`: Reads the next block into the buffer at the current offset when data remains and a state field holds the expected mode, otherwise returns zero.
- `validate_scroll_position`: Checks a requested position against the window bounds and updates the window fields, returning a status value.
- `read_line_from_buffer`: Reads bytes one at a time into the destination, stopping after a newline or a fixed limit of about 127 bytes, then appends a null terminator.
- `copy_null_terminated_string`: Copies bytes from the source string to the destination until and including the null terminator.
- `convert_string_to_uppercase`: Walks a null-terminated string and converts lowercase ASCII letters to uppercase by subtracting 32, leaving other bytes unchanged.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: When the first argument and a structure pointer are both non-null, calls a library function through a stored base with the argument and a full mask.
- `process_pager_structure_fields`: Calls routines on the pointer fields of a structure when they are non-null, then calls library functions for interrupt control.
- `write_single_value_via_handle`: Writes its argument as a single item through a handle obtained from a routine, passing a constant and a one-item stack buffer, and returns the result.
- `allocate_and_fill_info_block`: Allocates a 36-byte block through a routine and returns zero if that fails.
- `initialize_object_from_template`: Obtains a template identifier and allocates an object when the argument is non-null, then links its fields to the template and source block and copies words from the source address into it.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `execute_indirect_function_4`: Fetches a function pointer from a global table and executes it.
- `invoke_task_switch_function`: Calls the exec Switch function through a stored library base to hand control to another task.
- `execute_indirect_function_5`: Fetches a function pointer from a memory location offset from a global variable and executes it.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `invoke_alloc_mem_function`: Calls the exec AllocMem function through a stored library base, with no arguments set in the shown code.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `execute_indirect_function_6`: Fetches a function pointer from a table using an offset and executes it.
- `execute_indirect_function_7`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_8`: Fetches a function pointer from a specific offset within a global data structure and executes it.
- `execute_indirect_call`: Calls a function pointer located at an offset from a global variable.
- `execute_indirect_call_2`: Performs an indirect call to a routine stored in a global table.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `execute_indirect_call_3`: Executes a subroutine via a pointer retrieved from a global data structure.
- `execute_indirect_call_4`: Triggers a function call using an address stored within a global variable.
- `execute_indirect_call_5`: Loads a function address from a global table and jumps to it.
- `invoke_library_vector_414`: Calls a library function through a pointer read from a global base at a fixed negative offset, with no arguments of its own, and returns nothing.
- `execute_indirect_call_6`: Calls a routine by dereferencing a pointer found in a global variable.
- `execute_indirect_call_7`: Executes a subroutine through a pointer stored at a specific offset in a global variable.
- `forward_args_with_stack_slot`: Passes its two arguments on to another routine together with the address of a fixed stack slot in the caller's frame, then returns.
- `read_single_byte_result`: Asks a scheduling library stub to fill a one-byte stack buffer and expects a return value of one for success.
- `measure_stack_buffer_and_forward`: Calls a library routine through a global pointer, then counts the characters of a 140-byte stack buffer up to its terminator.
- `store_byte_at_pointer`: Writes the low byte of the data register to the address held in an address register, then returns.
- `divide_32bit_with_remainder`: Divides a 32-bit dividend by a divisor and returns the quotient and remainder as a register pair.
- `multiply_words_and_swap_halves`: Computes the low 32 bits of the product of two 32-bit values from 16-bit partial products.
- `recurse_unconditionally`: Calls itself with no arguments and no test before doing so, so the code contains no base case.
- `negate_result_on_flag`: Calls a routine and returns its first result value, negated when the second result held in an address register is non-zero.
- `invoke_global_vector_522`: Calls the function pointer stored at a fixed negative offset in a global data block, with no arguments, and returns.
- `store_byte_via_pointer`: Stores a byte from a data register into a memory location pointed to by an address register.
- `execute_indirect_call_8`: Performs an indirect jump to a routine address held in a global variable.
- `create_message_port_entry`: Allocates a signal bit and a zeroed 34-byte block, fills in the port type, flags, signal number and owning task, and initialises a list header inside it.
- `delete_message_port_entry`: Removes the block from its list when its link field is set, marks it as deleted, releases its signal bit and frees its 34-byte block.
- `find_substring_in_string`: Searches the first string for the second string, trying each starting position in turn.
- `setup_pointer_chain`: Stores the block's own address plus four into its first word, clears the second word, and records the block address in the third word.

### Format

- `program_entry_parse_arguments`: Program entry point.
- `cleanup_task_state_and_reply`: Saves its argument in the current task's state record, then runs the cleanup steps flagged in that record: closing a file handle, releasing a library, forbidding multitasking, freeing memory and replying to a message.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `invoke_library_vector_198`: Calls a library memory allocation function through a pointer read from a global base at a fixed negative offset, with no arguments of its own, and returns nothing.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `invoke_library_vector_54`: Calls a library task-switching function through a pointer read from a global base at a fixed negative offset, and returns nothing.
- `call_forbid_multitasking`: Calls the library function that disables task switching, reached through a pointer at a fixed global offset, and returns nothing.
- `invoke_library_vector_60`: Calls a library dispatch function through a pointer read from a global base at a fixed negative offset, and returns nothing.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `invoke_library_vector_42`: Calls a library scheduling function through a pointer read from a global base at a fixed negative offset, with no arguments, and returns nothing.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `invoke_library_vector_204`: Calls a library function that allocates memory at a fixed address, reached through a pointer read from a global base at a fixed negative offset, and returns nothing.
- `invoke_library_vector_48`: Calls a library rescheduling function through a pointer read from a global base at a fixed negative offset, and returns nothing.
- `run_disk_format_command`: Main routine of a disk formatting utility.
- `release_format_buffers`: Frees the stored buffers that are set, releasing two of them with fixed sizes, and then calls another routine with the argument it received.
- `map_disk_error_code_message`: Maps a disk error code to the text describing it, such as a bad sector, a device that cannot be opened, a changed disk or a seek error.
- `report_format_error_message`: Reports a formatting failure by showing a message about a write-protected disk, a missing disk or a general format error, depending on its argument.
- `match_byte_in_table`: Scans an eight-byte table for the given value and stores the matched entry, or zero if nothing matches, in a field of the structure returned by a routine.
- `find_drive_entry_in_list`: Walks a linked list of drive entries, matching each one either by a numeric key or by its name string.
- `calculate_string_length`: Returns the number of bytes before the terminating zero byte of a string, counted from its start.
- `copy_cstring_to_buffer`: Copies a zero-terminated byte string from the source pointer to the destination pointer, including the terminator.
- `convert_char_to_uppercase`: Returns the byte as an uppercase letter when it is a lowercase letter, and returns every other byte unchanged.
- `compare_strings_via_conversion`: Compares two zero-terminated strings one character at a time, passing each character through a conversion call first.
- `truncate_string_at_colon`: Advances to the first colon or the terminator of a string and writes a zero byte there, cutting the string short.
- `validate_value_with_selector`: Passes its first argument, placed in a one-element array, to a routine together with a fixed selector value.
- `create_linked_request_node`: Allocates a zeroed 68-byte record, makes its list head point to itself, and stores two arguments and up to seven words copied from an array.
- `execute_indirect_call_9`: Calls a function using a pointer retrieved from a global data location.
- `invoke_library_vector_198_2`: Calls a library memory allocation function through a pointer read from a global base at a fixed negative offset, and returns nothing.
- `execute_indirect_call_10`: Executes a routine by dereferencing a pointer stored in a global variable.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `execute_indirect_function_9`: loads a function pointer from a global table and jumps to it.
- `execute_indirect_function_10`: calls a subroutine located at an offset within a global data structure.
- `execute_indirect_function_11`: performs an indirect call to a routine stored in a global table.
- `execute_indirect_function_12`: jumps to a routine address retrieved from a global variable.
- `invoke_library_vector_210`: Calls a library memory release function through a pointer read from a global base at a fixed negative offset, and returns nothing.
- `execute_indirect_function_13`: executes a function pointer found at a specific offset in a global table.
- `execute_indirect_function_14`: calls a routine by dereferencing a pointer stored in a global table.
- `execute_indirect_function_15`: jumps to a subroutine address retrieved from a global data structure.
- `execute_indirect_function_16`: executes a function pointer located within a global table.
- `execute_indirect_function_17`: calls a routine via a pointer stored in a global variable.
- `execute_indirect_function_18`: jumps to a subroutine address retrieved from a global table.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `execute_indirect_pointer`: Calls a function pointer stored at a specific offset within a global variable.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `execute_indirect_pointer_2`: Calls a function pointer located at a different offset within a global variable.
- `forward_arg_with_stack_frame_slot`: Passes its argument to a second routine along with a stored pointer and the address of a stack slot in the caller's frame, then returns.
- `execute_indirect_function_19`: Fetches a function pointer from a location calculated by adding an offset to a global variable and executes it.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_and_verify_disk_cylinders`: Main disk formatter for each cylinder.
- `call_library_sequence_with_arg`: Makes two library calls on a stored device base, the first passing two constants and the second passing one, then calls a further library function with its argument.
- `invoke_global_and_checksum_stack`: Invokes a function pointer stored at a fixed offset from a global variable, then computes a rolling checksum over the local stack buffer until a null terminator is encountered.
- `store_byte_indirectly`: Stores a single byte into a memory location pointed to by a register.
- `divide_32_bit_integers`: Performs 32-bit integer division using a shift-and-subtract algorithm or a 16-bit partial division method.
- `shift_subtract_divide_loop`: Runs sixteen shift-and-subtract steps that move a 32-bit dividend left one bit at a time, subtracting the divisor from the high half whenever it fits and setting a quotient bit.
- `multiply_32_bit_integers`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer_2`: Stores a single byte from a data register into a memory location pointed to by an address register.
- `allocate_and_initialize_block`: Calls a library function to allocate a memory block, fills in fields including a stored parameter and flag bytes, and calls further library functions, returning the block address or zero.
- `reset_object_and_recurse`: Recursively processes a linked structure when a pointer field is non-zero, sets a byte field to all ones and a four-byte field to all ones, then calls another routine on a byte field value and again with a constant second argument.
- `forward_with_fixed_code_48`: Passes its argument to a second routine together with a fixed code of forty-eight, then returns.
- `forward_argument_to_routine`: Passes its single argument on to another routine and returns.
- `initialise_trashcan_defaults`: Fills a set of global default values, copying fields from a looked-up structure when one is found, then registers a named item and returns a status word.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `execute_indirect_pointer_3`: Calls a function pointer retrieved from a global variable using a specific offset.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `allocate_node_with_pointer`: Allocates a node through a library-backed allocator when given a non-null pointer, then stores a fixed type code, the low word of the second argument and the pointer into fixed offsets.
- `reset_node_and_release_it`: If given a non-null node, sets its type byte to all ones and sets two long fields to all ones, then passes the node and its stored word to another routine and returns that result.
- `execute_indirect_pointer_4`: Calls a function pointer stored in a global variable at a specific offset.
- `execute_indirect_pointer_5`: Calls a function pointer located at a specific offset within a global variable.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `cli_entry_parse_arguments`: Program entry point of the command-line shell.
- `invoke_library_vector_30`: Calls a library supervisor-mode function through a pointer read from a global base at a fixed negative offset, with no arguments, and returns nothing.
- `invoke_library_vector_36`: Calls the library function that returns from an interrupt, through a pointer read from a global base at a fixed negative offset, and returns nothing.
- `call_switch_function`: Calls a routine through a pointer retrieved from a global base at a fixed negative offset.
- `execute_indirect_function_20`: Fetches a function pointer from a global table and executes it.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `execute_indirect_function_21`: Fetches a function pointer from a calculated offset within a global table and executes it.
- `invoke_library_vector_96`: Calls a library function that searches for a resident module, reached through a pointer read from a global base at a fixed negative offset, and returns nothing.
- `invoke_library_vector_126`: Calls the library function that re-enables task switching, reached through a pointer read from a global base at a fixed negative offset, and returns nothing.
- `invoke_library_vector_222`: Calls a library function that allocates an entry from a list of memory blocks, reached through a pointer read from a global base at a fixed negative offset, and returns nothing.
- `start_cli_session_and_scripts`: Starts a command-line shell session.
- `build_linked_node_list`: Allocates eight-byte list nodes in a loop, storing each node's value from an input chain and linking it to the previous node.
- `free_linked_node_list`: Walks a chain of list nodes, releasing the value stored in each one and then freeing the node itself with its eight-byte size.
- `parse_signed_decimal`: Parses a signed decimal integer from a zero-terminated string, accepting an optional leading plus or minus sign, and returns its value.
- `execute_indirect_function_22`: Fetches a function pointer from a specific offset within a global data structure and executes it.
- `execute_indirect_function_23`: Fetches a function pointer from a memory location and executes it.
- `execute_indirect_call_11`: Fetches a pointer from a location relative to a global variable and jumps to it.
- `execute_indirect_call_12`: Performs an indirect function call using an address stored in a table offset from a global variable.
- `execute_indirect_call_13`: Calls a subroutine by dereferencing a pointer located at a specific offset from a global variable.
- `execute_indirect_call_14`: Loads a function address from a global table and transfers control to that address.
- `execute_indirect_call_15`: Executes a routine by jumping to a pointer retrieved from a memory location relative to a global variable.
- `execute_indirect_call_16`: Transfers execution to a subroutine address found within a global data structure.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `negate_on_flag`: Calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.

### Disk-Validator

- `verify_track_bit_pattern`: Program entry of a disk validator.

### Ram-Handler

- `copy_memory_then_dispatch`: Entry point that calls the library copy-memory function through a global pointer, then transfers control through an indirect jump whose target could not be recovered.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:AM2_BLIT

- `release_blitter_buffers`: Releases a series of memory blocks through the library's free function in a loop, then clears a word at an address derived from a register.
- `decrement_until_minus_one`: Counts a 16-bit local value down until it reaches minus one, then returns.
- `decode_bitstream_to_memory`: Decodes a variable-length bit stream in seven passes, copying runs of already written data forward within a buffer.
- `decode_bitstream_copy_runs`: Decodes length and offset codes from a bit reader and copies the resulting runs into the output buffer, working backwards until its counter is exhausted.
- `decode_bitstream_literal_block`: Decodes a bit-packed stream block by block, writing literal bytes or copying earlier output backwards into the buffer.
- `emit_decoded_quad_bytes`: Copies bytes from a bit-packed stream into the output buffer four at a time, reading length codes between the copies.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### d

- `stub_bad_instruction_data`: Stub whose recovered code is only bad instruction data, so no behaviour can be recovered from it.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:AM2_CPU

- `release_buffers_and_restore_word`: Runs a loop that calls the library's memory-release function several times, then reads a value from a structure pointed to by a global variable.
- `decrement_until_minus_one_2`: Decrements a 16-bit local variable until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:Ambermoon

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_d6_to_negative_one`: Decrements the value in register D6 until it reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:Ambermoon_install

- `reset_install_state_at_entry`: Program entry for Ambermoon_install that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_register_to_negative_one`: Decrements a 16-bit register until its value reaches negative one and then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a6].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `compare_and_call_routine`: Compares a longword at the pointer in a0 with a constant; if they match, it branches to another routine, otherwise it loads a word and a byte from a0, updates two global pointers with the current stack pointer, sets a0 to a global address, calls another routine, and then branches or jumps based on the resulting sign bit.
