# Game 1346 (distant armies - a playing history of chess (1988)(eagle tree software)[cr band]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01346). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902784 top-level bytes in 136 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 49 of 49 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 2975 of 2975 named (0 author symbols), 2975 of 2975 described (491 by rule, 2416 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2486 of 2486 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 50945 of 50965 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 269801 |
| code | 170418 |
| data | 418559 |
| padding | 35302 |
| unused | 8704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| More | 19 | 2290 | 76 |
| Else | 2 | 117 | 2 |
| Binddrivers | 11 | 652 | 35 |
| Mount | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| waitmouse | 2 | 4 | 1 |
| AddBuffers | 2 | 117 | 2 |
| FF | 3 | 751 | 38 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Dir | 2 | 117 | 2 |
| NewCLI | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Failat | 2 | 117 | 2 |
| clr | 3 | 19 | 1 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| CD | 2 | 117 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| LoadWB | 12 | 536 | 30 |
| SetClock | 17 | 1165 | 52 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band].adf:Distant Armies | 40 | 23509 | 1234 |
| serial.device | 4 | 982 | 84 |
| usa1 | 1 | 4 | 1 |
| parallel.device | 3 | 285 | 26 |
| clipboard.device | 10 | 1392 | 130 |
| printer.device | 46 | 6238 | 340 |
| narrator.device | 3 | 3782 | 268 |
| generic | 7 | 11 | 4 |
| ramdrive.device | 3 | 345 | 17 |
| 7 | 1 | 36 | 2 |
| 16 | 1 | 2 | 1 |
| 9 | 1 | 2 | 1 |
| 17 | 1 | 2 | 1 |
| 20 | 1 | 2 | 1 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 10 | 1375 | 109 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2597 | 167 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 15 | 1040 | 61 |
| Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 103 | 103 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 64 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `parse_command_line_arguments` (FF): Attempts to open several libraries and, if successful, parses a command-line string to check for certain flags or switches, skipping spaces and handling hyphens, then calls library functions to open or close libraries based on the parsed input. Kind: command line switch/parameter check. Pass path: The loop continues or exits via the condition `(pcRam01007c1c == (char *)0x0) && ((uRam01007c20 & 8) == 0)` or the `else` branch where `iRam01007c10` is validated, avoiding the `u7__sub_0_0338()` error call..
- `compare_font_properties` (FF): Compares certain properties or pointer-based values between two structures and, if they match, writes a character code to a global location. Kind: baseline mismatch check. Pass path: the branch where the condition (mismatch) is false, or specifically the path where the check does not trigger a jump/mismatch error (e.g., the `else` or non-taken `beq` paths or the `iVar2 != 0` path or `iVar2 == 0` path depending on the sp.

## Functions


### More

- `initialize_file_metadata_and_parse_string`: Attempts to open a file and, if successful, parses its header or content to populate a structure with metadata, including a routine to scan a buffer for a string or a quoted string, replacing certain characters like certain newline or escape sequences, and then calls another routine to process the resulting data or return a status code.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Attempts to open a file and, if successful, parses its header to populate a structure with metadata, manages memory allocation for the file content, and updates internal state flags or pointers to the loaded data or associated error states.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `process_string_character_and_update_state`: Checks if a character or a certain value in a structure matches a set of allowed characters, and if so, updates a field in the structure or calls other routines to handle string-based or character-based logic, potentially clearing a range of memory or updating a pointer-based buffer.
- `process_message_recursive`: Recursively processes a structure by invoking itself on various fields and string arguments, conditionally updates fields at offsets 8, 6, a.
- `initialize_entity_state`: Sets a 32-bit field at offset 60 and a 16-bit field at offset 94 of a structure to specific values, calls another routine with a pointer from offset 52, calls itself recursively if certain conditions are met, and enters a loop that calls another routine while certain conditions regarding offset 60, offset 8 of a pointed-to structure, and offset 94 are met.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field at offset 4 is zero it increments a short located at an offset from a pointer stored in the structure.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on fields in a passed structure and iterates to call another routine using a pointer to a table and a global variable; it also recursively calls itself, calls another routine with a pointer to a certain stack buffer, and calls a library routine to reschedule.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Calls another routine with zero, then with a parameter, then with a pointer from offset 16 of a structure, a global variable, and a parameter, conditionally calls another routine with the same arguments but without the parameter, calls another routine with two arguments, calls another routine with a constant, calls another routine with a parameter, and returns the result of the call involving two arguments.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: The routine calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: The routine calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is encountered.
- `dispatch_recursive_pointer`: The routine calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `recursively_process_buffer_segments`: If a field at offset 120 is non-zero and a field at offset 116 is not 4096, it calls itself recursively with a pointer into the structure and a size from offset 120.
- `update_scroll_window`: Calculates a target position based on provided offsets and compares it against the current window bounds.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a certain location and a provided mask.
- `process_message`: Recursively processes a structure by invoking itself on various fields and string arguments, conditionally updates fields at offsets 8, 6, and 7, and performs a loop based on a return value.
- `build_message_buffer`: The routine obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing theinput parameter, and a size of one.
- `copy_entity_data`: The routine obtains a handle from another routine and, if valid, reads a value from that handle, passes it to another routine together with a small stack buffer, and copies two fields from the handle into the caller-provided structure.
- `initialize_object_from_template`: Obtains a template identifier, allocates an object, links its fields to a template and source block, and copies words from a source address into the new object.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `extract_byte_from_call`: Calls another routine with a pointer to a small stack buffer and a constant, returning a byte from that buffer if the call returns certain a value, otherwise returning negative one.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_modular_result`: Computes a 32-bit result by repeatedly doubling and subtracting the divisor when the running remainder exceeds it, otherwise it performs a 16-bit modulo on the low half of the dividend and returns the combined quotient and remainder.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that or that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: Removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0_and_branch`: Copies the contents of d1 to d0 and then branches to a fixed offset.

### Binddrivers

- `allocate_fast_memory_and_parse_args`: Allocates a block of fast memory and stores the requested size, a pointer to the block, and the address of the caller's stack frame into a structure, then calls a routine that parses the command line from dos.library into a linked list of null-terminated argument strings, handling quoted text and asterisk conversions; if the fast memory allocation fails, it returns twenty.
- `bind_expansion_drivers`: Opens expansion and icon libraries to obtain a list of expansion devices, then iterates through a driver table to disable each device, invoke its driver, and restore its state.
- `find_driver_entry`: Scans a table of short values starting from an offset derived from a parameter, decrementing a counter and advancing a pointer until it finds an entry whose first short equals a constant and whose second short points back to the entry itself; otherwise it returns null.
- `iterate_driver_data`: Iterates through a buffer or list, performing arithmetic on a counter and updating a value via another routine.
- `calculate_string_index`: Uses a series of calls to determine an index and a value, then performs conditional logic to return a result or zero.
- `traverse_handler_chain`: Iterates through a linked list of handlers and updates a pointer in a structure if a certain bit is set in a memory location.
- `follow_parent_chain`: The routine receives two pointers and walks a linked list by repeatedly following a field located at a fixed offset from the current node.
- `skip_non_slash_colon`: The routine checks a character string for a leading slash or colon.
- `parse_decimal_integer`: The routine reads a null-terminated string through a pointer and accumulates a decimal integer from consecutive ASCII digit characters, stopping at the first non-digit or null byte.
- `clamp_value_by_offset`: The routine calls another routine with the first two arguments and receives a result.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_alert`: Library stub: loads its arguments and the exec library base, calls Alert and returns its result to the caller.
- `call_enable_2`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `compare_two_strings`: The routine compares the byte strings pointed to by two registers byte by byte until a zero byte is found.
- `load_address_to_a0`: Loads a 32-bit value from an offset of 4 from the stack pointer into a0.
- `compute_checksum`: Initializes a 32-bit accumulator to all ones and decrements it by one for each byte in a null-terminated string.
- `find_byte_in_list`: Iterates through a list of bytes starting at a pointer in a register and returns a pointer to the first instance of a target byte, or a pointer to the byte following the end-of-list marker.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `compare_and_checksum_strings`: Compares two strings and, if the second string is longer than the first, computes a checksum over the remaining bytes of the second string.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.

### Mount

- `find_max_and_prepare_buffer_11`: Walks a linked list of entries to find the maximum value stored in a field, then if a condition is met, writes a header and a sequence of incrementing words into a buffer, and finally copies a fixed number of words between a temporary area and a global buffer before installing a function pointer and returning success or failure.
- `move_and_branch_4`: Moves a longword from d1 to d0 and branches to another instruction.

### type

- `find_max_and_prepare_buffer_12`: Walks a linked list of entries to find the maximum value stored in a field, and if a condition is met, writes a header and a sequence of words into a buffer, sets up a stack frame, copies sixteen words from a temporary area to a global buffer, installs a function pointer, and copies another sixteen words from a global buffer into the temporary area before returning success or failure.
- `move_and_branch_5`: Moves the contents of d1 to d0 and then branches to a different instruction.

### waitmouse

- `wait_mouse_release`: Polls a bit in the CIAA_PRA register until it is cleared, then clears d0 and returns.

### AddBuffers

- `find_max_and_prepare_buffer_13`: Walks a linked list of entries to find the maximum value stored in a field, and if a condition is met, writes a header and a sequence of words into a buffer, sets up a stack frame, copies sixteen words from a temporary area to a global buffer, installs a function pointer, and returns success or failure.
- `copy_register_and_branch`: Copies the value from d1 to d0 and then branches to another routine.

### FF

- `parse_command_line_arguments`: Attempts to open several libraries and, if successful, parses a command-line string to check for certain flags or switches, skipping spaces and handling hyphens, then calls library functions to open or close libraries based on the parsed input.
- `call_open_library`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `flush_ports_and_reply_pending_message`: Saves the word argument, switches to the saved stack, calls a flush helper on four message pointers, finds the current task, forbids, replies to a pending message and returns the argument.
- `branch_on_zero_pointer`: Library code that, if d0 is zero, copies d0 to a1 and jumps to an address relative to a6.
- `load_address_to_a1`: Loads a longword from a location relative to the stack pointer into a1.
- `scan_string_and_call_library`: Saves registers, increments a pointer while checking for a null terminator, and if a certain global value is zero, calls a library function using a pointer from a structure, then calls another library function using d0 and d1, and returns.
- `initialize_array_swap`: Library code that loads two pointers from the stack, sets a counter, calls a routine that swaps word arrays, and increments the pointers.
- `swap_word_arrays_offset`: Exchanges the word at offset 28 of each a0/a1 pair, advances both pointers by two and repeats d1+1 times before returning.
- `allocate_and_init_fastfont_record`: Allocates a record via two allocators, fills fields at offsets 8, 10 and 14, initialises two sub-structures, copies six bytes into offset 34 and returns the record pointer.
- `process_font_metadata`: Processes a structure or pointer to check bitmask flags and conditional pointer values.
- `select_font_by_size_and_store_handle`: Stores the requested font size in a global, opens a font via the font opener, sets digit characters for size eleven or eight, and then returns.
- `close_font_and_clear_handle`: Checks the font handle at offset four of the structure, closes the font with the given base and handle, then clears that handle field and returns.
- `compare_font_properties`: Compares certain properties or pointer-based values between two structures and, if they match, writes a character code to a global location.
- `release_ff_object_and_three_buffers`: Allocates an object, frees three buffers at offsets 40, 48 and 56, frees the third only if non-null, then releases the object and its memory block.
- `clear_field_and_handle_abort_text`: Copies a 32-bit value from offset 64 of a structure into a global variable, clears that field if it is non-zero, and then calls a routine to print an error message and waits for a keypress or status change in a loop.
- `reset_and_check_field_status`: Copies a 32-bit value from a field in a structure to a global variable, clears that field if it is non-zero, and calls another routine to check a status, returning zero if the status check succeeds or one otherwise.
- `toggle_fasttext_and_init_font`: Checks a flag in a global structure, prints a status message, and then deallocates or updates font pointers at offsets 40 and 56 of a structure, while updating font pointers at offsets 64 and 68 from a global source.
- `print_string_then_free_memory`: Library code that pushes a string argument to the stack, calls a routine to print it, then calls a routine to free memory using a null pointer before returning.
- `build_font_render_and_install_bitmap`: Opens a graphics library base, reads a font structure, allocates chip memory, patches a blit routine with register moves, copies font bytes, allocates a bitmap, fills it via a helper and stores the result.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `copy_plane_rows_to_buffer`: Saves d1, stores its low byte to a4 offset, resets a2 fields, calls a library function, then copies a2's row count of bytes from a1 stepping 128 bytes into a3 and returns.
- `execute_conditional_library_calls`: Checks a pointer, clears a value, and if certain conditions or certain pointer values are met, calls a library function, potentially calls another routine, and may call a library function with a modified offset or a constant offset before returning.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a2, then returns.
- `jump_to_address`: Performs an indirect jump to an address calculated relative to the program counter.
- `tiny_moveq_move`: Small routine of 2 instructions that loads 0 into d1, then copies (a0)+ to d1.
- `shift_and_store_byte`: Shifts a longword in d1, calculates a new address, and copies a byte from that address to a location pointed to by a6.
- `unroll_and_execute_logic`: Unrolls a loop to copy bytes from a2 to a6, updates a pointer, calls a library function, and if a certain condition is met, calls another library function and performs a bitwise NOT on a global variable before jumping to a library function.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `execute_indirect_call`: Loads a pointer from a structure offset, then calls a routine at an offset from that pointer.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_call_short`: Loads a pointer from a structure offset, then calls a routine at a short offset from that pointer.
- `execute_indirect_call_with_args`: Loads a pointer from a structure offset, loads a value from the stack, then calls a routine at an offset from that pointer.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.

### Assign

- `find_max_and_prepare_buffer_14`: Walks a linked list of entries to find the maximum value stored in a field, then, if a condition is met, writes a header and a sequence of words into a buffer, copies a block of 16 words from a temporary area to a global buffer, installs a function pointer, and returns success or failure depending on whether certain checks pass during the loop.
- `copy_register_and_branch_2`: Copies the value from d1 to d0 and then branches to another routine.

### EndCLI

- `find_max_and_prepare_buffer_15`: Walks a linked list of entries to find the maximum value stored in a field, and if a condition is met, writes a header and sequential words into a buffer, sets up a stack frame, copies sixteen words from a temporary area to a global buffer, installs a function pointer, and copies another sixteen words from a global buffer into the temporary area before returning success or failure.
- `move_register_and_branch`: Copies the contents of one data register to another and then performs an unconditional short branch to another routine.

### Dir

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value stored in a field, then if a condition is met, writes a header and sequential words into a buffer, sets up a stack frame, and copies sixteen words between a temporary area and a global buffer before installing a function pointer and returning success or failure.
- `copy_d1_to_d0_and_branch_2`: Copies the contents of d1 to d0 and then branches to a fixed offset.

### NewCLI

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value stored in a field, then if a condition is met, writes a header containing sequential words into a buffer, sets up a stack frame, and copies a block of sixteen words from a temporary area and another block from a source table into the buffer before installing a function pointer and returning.
- `transfer_register_and_branch`: Copies the contents of one data register to another and then performs an unconditional short branch to another routine.

### Echo

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value stored in a field, and if a condition is met, writes a header containing sequential words into a buffer, sets up a stack frame, and copies sixteen words between a temporary area and a global buffer before installing a function pointer and returning success or failure.
- `transfer_register_value`: Copies a longword from register d1 to register d0 and branches to another routine.

### Failat

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value stored in a field, then if a condition is met, writes a header and a sequence of incrementing words into a buffer, sets up a stack frame, and copies a fixed number of words between a temporary area and a global buffer before installing a function pointer and returning success or failure.
- `transfer_register_and_branch_2`: Copies the contents of a data register to another data register and then branches to a different part of the calling routine.

### clr

- `initialize_library_resources`: Accesses a global variable and calls several library functions using offsets from a library base pointer.

### Execute

- `find_max_and_prepare_buffer_6`: Walks a linked list of entries to find the maximum value stored in a field, and if a condition is met, writes a header and sequential words into a buffer, sets up a stack frame, copies sixteen words from a temporary area to a global buffer, installs a function pointer, and returns success or failure depending on the conditions checked during the loop.
- `transfer_register_to_destination`: Copies a longword from register d1 to register d0 and branches to another routine.

### Copy

- `find_max_and_prepare_buffer_7`: Walks a linked list of entries to find the maximum value stored in a field, then if a condition is met, writes a header and a sequence of words into a buffer, sets up a stack frame, and copies a fixed number of words between a temporary area and a global buffer before installing a function pointer and returning success or failure.
- `transfer_register_and_branch_3`: Copies the contents of one data register to another and then jumps to a different part of the calling routine.

### Makedir

- `find_max_and_prepare_buffer_8`: Walks a linked list of entries to find the maximum value stored in a field, and if a condition is met, writes a header and a sequence of incrementing words into a buffer, then copies a fixed number of words between a temporary area and a global buffer before installing a function pointer and returning success or failure.
- `transfer_register_and_branch_4`: Copies the contents of one 32-bit register to another and then jumps to a relative offset in the code.

### CD

- `find_max_and_prepare_buffer_9`: Walks a linked list of entries to find the maximum value stored in a field and, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_long_word_and_branch`: Copies a long-word from d1 to d0 and then branches to another routine.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_prepare_buffer_10`: Walks a linked list of entries to find the maximum value stored in a field, and if a condition is met, writes a header and fills a buffer with sequential words.
- `copy_long_word_and_branch_2`: Copies a long-word from d1 to d0 and then branches to another routine.

### LoadWB

- `parse_file_header_and_string`: Attempts to open a file and, if successful, parses its header to populate a structure with metadata, managing memory allocation for the file.
- `update_and_cleanup_callback_table`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer, then conditionally invokes several cleanup callbacks stored in a table, setting specific values in the structure before each call, and finally returns the saved parameter.
- `start_tool_from_resident_tag`: XOR-decodes a global string with one key applied twice, allocates a 160-byte block, and uses icon, intuition, and workbench library calls to start a tool.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte and returns minus one when the first is lower, one when it or higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `call_library_vector_96_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `compare_string_buffers`: Checks if a pointer in a2 is valid, then loops to compare a string or buffer pointed to by a2 with a global string, or if a certain byte at offset 20 of the structure in a2 is 50, it compares a string or buffer pointed to by a2 with a global string using a loop.
- `initialize_string_padding_bytes`: Writes the value 46 to the byte at offset 17 of the structure pointed to by a2, then writes the value 32 to the next four consecutive bytes starting at offset 18 of that structure, and otherwise returns to a different part of the calling routine.
- `compare_string_bytes`: Compares a string or buffer pointed to by a2 with a string or buffer located at a certain offset from the program counter, using a loop that checks if the bytes match or if a null terminator is encountered, otherwise it returns.
- `load_stack_and_return`: Loads a pointer from a relative offset into a register, restores two registers from the stack, and returns.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `recurse_with_stack_slot_2`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `verify_and_dispatch_buffer_checksum`: Invokes a function pointer from a global variable, then iterates through a stack-allocated buffer to compute a rolling checksum until a null terminator is found; if the bitwise NOT of the checksum is non-zero, it calls another routine passing the buffer and the inverted checksum.
- `store_byte_to_pointer`: Stores a byte from a register into a memory location pointed to by a register and then returns.

### SetClock

- `parse_file_string_metadata`: Attempts to open a file and, if successful, parses its header or string content, handling quoted strings and replacing certain characters like certain letters with newlines or escape codes, then calls another routine to process the resulting string or metadata.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `process_string_table`: Iteratively checks pointer-based string or data lookups and updates a global variable or structure field based on the results.
- `execute_multiple_calls`: Calls another routine or a function pointer stored in a global variable multiple times with different arguments.
- `poll_clock_register_change`: Reads the clock register low nibble, delays briefly, reads it again and returns one if the nibble changed, otherwise zero.
- `write_global_and_check`: Writes several constant values to a global array and returns a comparison of a global variable's bits.
- `set_clock_type_ten`: Allocates a structure, sets its type field to ten, calls a setup routine, and then calls a release routine, returning the value stored at offset 32.
- `start_clock_measurement_store_result`: Allocates a structure, sets its type field to eleven, calls a routine to calculate a result and stores it at offset 32, clears offset 36, then calls a release routine and returns zero.
- `open_timer_and_set_clock`: Opens the timer device, requests a timer unit, and calls a routine to set the timer or closes the device and returns zero if any step fails.
- `free_clock_request_and_close_timer`: Frees the request's timer message if present, deallocates the request block, closes the timer device unit and returns zero.
- `calculate_total_seconds_from_clock`: Reads packed BCD digits from a clock structure, converts them to decimal, and calculates a total number of seconds by adding offsets from a table and accumulating values from a pointer-based list, then returns the final calculated value.
- `set_clock_from_seconds`: Calculates the remaining seconds from a value, then iterates through a table of time-unit durations to subtract or assign the resulting remaining seconds into a clock record, storing the decimal digit pairs or zero terminator into the record structure.
- `branch_to_middle_of_routine`: Saves registers a2 and a3 to the stack, then loads a pointer from an offset relative to the stack pointer into a3 and another pointer from an offset relative to the stack pointer into a2, before branching to another routine.
- `validate_and_compare_sequence`: Iterates through a sequence of bytes using two pointers, calling another routine to process each byte, and checks if the returned value matches a byte at a second pointer location.
- `compare_and_set_negative`: Compares a byte in a register to another and if it is not greater or equal, sets a register to a negative constant and branches.
- `return_constant_one`: Sets a register to one and returns.
- `extend_and_offset_byte`: Checks if a byte from a stack offset is within a certain range, and if so, sign-extends it to a long and adds a constant, otherwise it sign-extends it to a long and returns.
- `tiny_ext_ext_rts`: Small routine of 3 instructions that sign-extends d0, then sign-extends d0, then returns.
- `pulse_clock_lines_and_wait_for_ack`: Writes three clock control patterns to offsets 60 of the structure pointed to by a global pointer, sets bits at offset 52, then polls bit 1 of offset 55 up to 200 times before returning.
- `update_color_structure_loop`: Writes a constant to an offset in a structure, then loops a certain number of times while checking a bit, potentially writing a different constant to a different offset if a bit is clear.
- `write_color_registers_via_a1_pointer`: Loads the pointer stored at a1 into a0 and writes two immediate color values to offsets 60 and 56 of that structure, then returns 1 in d0 and pops d2.
- `write_color_registers_from_global_pointer`: Loads the global pointer at absolute address 324 into a0 and writes the same immediate color value to offsets 60 and 52 of that structure, then returns.
- `set_clock_signal_bits_2`: Writes a constant to an offset in a structure, then loops a certain number of times while checking a bit, potentially writing a different constant to an offset if a bit is clear.
- `set_clock_signal_bits`: Writes 4 to offset 60 of the structure pointed to by a1, then writes the immediate patterns to offsets 60 and 56 of that structure and returns 1 in d0.
- `call_reschedule_3`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_3`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_alloc_mem_3`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_entry_2`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `recurse_with_stack_slot_3`: Loads two pointers from the stack and calls another routine passing those pointers and a pointer to a specific location on the stack.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `checksum_buffer_and_call_subroutine`: Invokes a function pointer from a global variable, then iterates through a stack-allocated buffer to compute a rolling checksum until a null terminator is found; if the bitwise NOT of the checksum is non-zero, it calls another routine passing the buffer and the checksum.
- `store_byte_to_pointer_2`: Stores the byte from d0 into the address pointed to by a3 and increments a3, then returns.
- `compute_modular_result_2`: If a 32-bit value's high word is present, it uses a bit-shifting loop to compute a 32-bit remainder and quotient; otherwise, it calculates a 16-bit modulo and quotient using the low word of a 32-bit register and a 16-bit divisor, returning the result in a single 32-bit register.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper_2`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `negate_result_on_nonzero_flag`: Calls a helper routine and returns its first result, or returns the negation of that result if the helper's return address or a secondary register-based check is non-zero.
- `allocate_and_populate_message_buffer`: Attempts to allocate a message buffer using a helper routine; if successful, it populates several fields of the buffer, including a value from a previously called routine, and then calls another routine to either dispatch or skip the message based on a parameter or a check of a returned value.
- `remove_port_from_list`: Checks if a field at offset ten of a structure is non-zero, then sets a byte at offset eight to any value, sets a dword at offset twenty to all ones, and calls another routine using a byte loaded from offset fifteen, followed by a call to another routine with a constant value passed via the stack.
- `create_ext_io`: The routine allocates memory for an I/O request structure when the first argument is non-zero, initializes specific fields with constants and arguments, and returns the pointer to the block or zero.
- `clear_and_free_io_request`: Checks if a pointer passed in a register is non-zero, then sets a byte at offset 8 and two dwords at offsets 20 and 24 of that structure to certain values, and calls another routine using a size or pointer stored at offset 18 of that structure before returning.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Port-Handler

- `initialize_relocation_table`: Initializes a structure with pointers and size limits, then iterates through a table of relocation or hunk-type offsets to update or clear specific memory locations or pointers in a global structure, calling other routines to handle or skip certain relocation types.

### Disk-Validator

- `update_string_buffer_and_bitmask`: Updates a string buffer with a character or a digit, modifies a counter in a global structure, and performs a loop that sets bits in a bitmask or updates a bitset-like structure based on a range of values.

### Ram-Handler

- `execute_indirect_jump`: Sets bit 2 of d0, copies a pointer from d1 to d2, and calls a routine at an offset from a pointer in a6, then jumps to the address in a6.

### Distant Armies - A Playing History of Chess (1988)(Eagle Tree Software)[cr Band].adf:Distant Armies

- `jump_to_entry`: Performs a long jump to a target address.
- `initialize_string_and_table_pointers`: Tests for the presence of certain resources and, if successful, populates a table with pointers to string or image data, updates state flags, and executes a loop to assign pointers or data-related offsets to a temporary array or list.
- `execute_conditional_messages`: Executes a series of indirect function calls with hardcoded arguments, then checks if a provided parameter is at least six; if so, it fetches a pointer from a table and calls another routine using that pointer and a subsequent table entry, otherwise it calls a routine with a null pointer and a zeroed value, finally calling a separate routine before returning.
- `call_multiple_routines`: Calls a routine four separate times using different pointer arguments derived from a register.
- `execute_conditional_calls`: Executes a series of conditional calls to various routine pointers based on checking bits in a byte or checking if pointers or certain values are non-zero.
- `execute_conditional_branch`: Sets up a stack frame, loads a pointer from a structure, pushes a value, calls a routine, and if d0 is not zero, pushes another value and calls a second routine before branching back.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `update_structure_members`: Copies multiple 16-bit and 32-bit values from a source table into a destination structure and calls a routine with the destination address.
- `conditionally_update_pointer`: Updates a global variable with a value from another global variable or a bitwise-or-ed pointer value, depending on a conditional check.
- `process_structure_data`: Calls a routine using a pointer from a structure, then updates several fields in a structure based on the result, including conditional logic for certain values and an extension of a signed value.
- `initialize_resource_pointers`: Sets a bit in a global byte, calls a routine with a pointer, and then checks a table or global value to conditionally call another routine or update a pointer.
- `tiny_unlk_rts_2`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `clear_bit_in_byte`: Calls a routine, clears the least significant bit of a byte at a specific offset in a structure, and calls another routine.
- `set_bit_in_byte`: Calls a routine, sets the least significant bit of a byte at a specific offset in a structure, and calls another routine.
- `call_routine_with_constants`: Calls a routine multiple times with specific constant or pointer-based arguments.
- `invoke_subroutine_with_arguments`: Calls a routine using a pointer from a global variable and passes three arguments from a global variable.
- `execute_multiple_subroutines`: Executes two different subroutines via pointers and calls a third routine with a specific constant value.
- `save_registers_and_jump`: Stores several registers into a memory location pointed to by a stack pointer and then jumps to a destination address stored in a register.
- `branch_to_indirect_address`: Checks a value, and if it is non-zero, jumps to an address stored in a pointer, otherwise it sets a register and jumps to an address stored in a register.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `increment_pointer_on_match`: Sets up the stack and a2, loads a pointer from offset 8 of a5 into a2, and if the byte at a2 is 32 or 9, increments a2 and branches back to the start of the routine.
- `check_byte_and_increment_pointer`: Compares a byte at a pointer in a2 to a constant, and if it matches, increments the pointer and sets a register to one.
- `check_byte_and_clear`: Compares a byte at a pointer in a2 to a constant, and if it matches, increments the pointer and clears a value in a structure.
- `update_counter_loop`: Iterates through a buffer, calls a function, and updates a global variable with the result plus an offset, continuing if a bit in a table is set, then returns a signed or unsigned version of that variable.
- `load_indirect_longword`: Returns a longword stored at a specific offset relative to a register.
- `load_registers_and_branch`: Loads a pointer from the stack into a0 and saves d0 and d1 to the stack before branching.
- `fill_memory_with_bytes`: Repeatedly writes bytes to memory using a pointer and a counter until a termination condition is met.
- `update_buffer_loop_2`: Decrements a value at offset 12 of a structure in a5 and, if it is not negative, calls another routine, then checks if the return value is not negative, and if so, increments a pointer and writes the return value to the new pointer location, continuing a loop if the return value is not 10.
- `validate_and_restore_registers`: Clears a byte at a pointer in a2, checks if d4 is not certain, and if a pointer in a5 matches a certain address, restores registers from the stack and unlinks the stack frame before returning.
- `load_and_branch_4`: Loads a longword from a global variable into d0 and branches.
- `modify_structure_bit`: Clears bit 3 of a byte at offset 12 of a structure in a2, and if bit 2 is clear, calls another routine with a pointer and a value, otherwise it sets d0 to negative one and returns.
- `branch_to_target_routine_3`: Performs a short branch to a target address located in another routine.
- `verify_and_call_routine`: Checks if a value at offset 16 of a5 is 1 and if a pointer in a2 is not null, then calculates a difference between a pointer in a2 and its next element, updates offset 12 of a5, clears certain memory at a2, pushes several values onto the stack, and calls another routine with a value loaded from a2 offset 13; if the returned d0 is not greater than or equal to 0, it sets d0 to -1 and branches forward.
- `clear_d0_and_branch_4`: Clears register d0 and branches to another routine.
- `update_array_difference`: Calls a routine, then if a certain bit is not set in a structure, subtracts the difference between two array elements from a value in a global variable and returns.
- `subtract_array_difference`: If the first element of an array is non-zero, subtracts the difference between the first two elements from a stored longword.
- `compare_pointer_address_2`: Loads a pointer from offset eight of a structure in a5, calls a routine to compare that pointer with another address, and then checks if a longword in d0 is zero, otherwise it decrements d0 by four and branches if d0 is not zero, otherwise it branches to another location.
- `decrement_and_set_bit`: Decrements a longword at a pointer and sets bit 3 of a byte located at an offset of 3 from that pointer, then returns.
- `branch_only`: Performs a branch to another location.
- `branch_on_register_value_2`: Checks if a longword in d0 is zero, otherwise decrements d0 by 4 and branches if d0 is not zero, otherwise it branches to another location.
- `compare_pointer_address`: Loads a pointer from an offset 8 of a structure in a5, compares the address at that pointer with an address at offset 4 of the same structure, and calls another routine if the first address is not less than the second.
- `extract_byte_and_branch`: Loads a pointer from a memory location, increments that pointer, and extracts a byte into d0 as a longword masked to 8 bits before branching.
- `test_bit_and_branch_on_structure`: Loads a pointer from an offset 8 of a structure, checks if bit 3 or 4 of a byte at offset 12 of that structure is set, and if neither bit is set, loads a constant into a register and returns; otherwise, it returns without further action.
- `traverse_and_update_list_5`: Clears bit 2 of a byte at offset 12 of a structure in a2, and if the pointer at offset 8 is not null, calls another routine.
- `set_byte_and_branch`: Sets a bit in a byte at an offset from a pointer, clears d0, and branches.
- `update_pointer_and_extract_byte`: Updates a pointer with a value from its own offset, modifies a value at an offset, and extracts a byte from a new pointer location into d0 as a masked longword before branching.
- `push_and_return_pointer`: Pushes a pointer from a7 onto the stack, calls a routine, and if the return value in d0 is not zero, it clears d0, restores a pointer from the stack, and returns.
- `push_structure_pointers`: Pushes a pointer from offset 12 of a structure in a5 and a pointer from offset 8 of the same structure onto the stack, calls another routine, and then jumps to another routine.
- `compare_string_buffers_3`: Calls another routine, then checks if a byte at a pointer in a2 is null; if not null, it continues a loop that compares strings and, if a match is found, calls another routine and updates a structure member, otherwise it sets a global variable to 5 and returns.
- `compare_string_buffers_2`: Compares a string or buffer starting at a2 with a string or buffer starting at a pointer or offset 12 of a5, and if the strings match and a byte is zero, it calls another routine with a pointer and a value, otherwise it increments a2 by 8 and loops.
- `process_structure_list`: Pushes a longword from a structure offset 4 and a longword from a structure offset 8 onto the stack, calls another routine, and if d0 is not equal to a certain value, continues a loop by adding 8 to the stack pointer and checking if a certain value was reached, otherwise it clears d0 and loops back to a previous instruction.
- `modify_global_data`: Writes d4 to an offset in a structure, writes a byte to another offset in the same structure, and loads a pointer from that structure into d0 before branching.
- `setup_and_verify_pointer`: Calls a routine, then uses the result to calculate a pointer, checks if that pointer or a value in a structure is valid, and if certain conditions are met, writes a value to a structure and returns.
- `setup_and_call_seek`: Pushes a value from a pointer, a value from a structure, and a pointer from a2 onto the stack, calls setup_stack_and_seek, and if the return value is not negative, adjusts the stack and calls another routine, otherwise it adjusts the stack, writes a value to a global variable, and sets d0 to negative one before branching.
- `setup_stack_and_seek`: Clears two stack slots, pushes a pointer from a register onto the stack, calls a seek routine, and then adjusts the stack pointer before branching to another routine.
- `setup_stack_and_call`: Sets up a stack frame, pushes a longword from a structure offset 12 onto the stack, pushes a longword from a structure offset 8 onto the stack, calls another routine with a constant, and then cleans up the stack before returning.
- `call_and_store_result`: Saves registers to the stack, moves a pointer from a structure into a register, calls another routine, and then moves a pointer from a structure into a register before branching.
- `compare_structure_value`: Calls a routine with a value in d0, then if a certain memory location or pointer is not null, increments d4 and compares d4 against a value loaded from a global address, looping if d4 is less than that value.
- `process_structure_flags`: Checks bit 1 of a byte at offset 14 of a structure in a5, and if set, calls several routines while managing a pointer in a2, potentially updating a global variable with a value or calling another routine with a constant.
- `execute_library_calls_3`: Pushes a pointer onto the stack, calls a routine, moves the result to d6, adjusts the stack, and if d0 is not zero, calls another routine, moves the result to d5, and branches forward.
- `execute_library_seek`: Pushes a pointer and a constant onto the stack, calls a library function, then copies a word from a structure into a register and calls another routine.
- `execute_seek_and_store_word`: Pushes a pointer and a constant onto the stack, calls a routine, then calculates a new address and stores a word from a structure at that address, and calls another routine if a bit is set.
- `check_and_update_value`: Checks if a certain bitmask is present in a value from a structure, calls another routine, and writes a constant value to a global variable if the check passes, otherwise it restores registers and returns.
- `execute_seek_and_update_pointer`: Calls a routine using a pointer from a4, calculates a new address using d0 and a3, and writes a word from a structure to that address; it then calls the same routine again, checks bit 3 of a byte in a structure, and if set, pushes a constant onto the stack and calls another routine.
- `traverse_linked_list_2`: Calls a routine, then calls another routine using a pointer derived from the first result, and if certain conditions or a null pointer check pass, sets a global variable to a constant and returns.
- `check_mask_and_set_global`: Extends a word from a pointer, masks it, and if it does not equal one, sets a global value to a constant and sets a register to a constant.
- `execute_read_and_store_result`: Pushes a pointer, a pointer from a structure, and a pointer from a2 onto the stack, calls a routine, and if the return value is not negative, adjusts the stack and branches to another routine, otherwise it adjusts the stack, writes a value to a global variable, and sets d0 to negative one before branching.
- `move_and_branch_2`: Moves the contents of one register to another and then branches to another routine.
- `copy_byte_block`: Copies a number of bytes from one memory block to another, moving forward when the destination precedes the source and backward when it follows it, with a zero-length early return.
- `initialize_global_state`: Calls a routine, zeroes out a range of memory, updates a global pointer, and calls a library function or another routine based on a bit test or a return value.
- `set_pointer_to_constant`: Loads a constant value into a register and returns immediately.
- `initialize_file_buffer`: Initializes a file or buffer structure, setting header or metadata, and potentially executing indirect functions or library calls.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_string_into_table`: Parses a source string by searching for whitespace or quoted delimiters, copies the resulting tokens into a destination buffer, and then populates a table with pointers to these strings.
- `set_max_limit`: Sets a register to a specific constant value and then branches to another routine.
- `insert_string_at_end`: Finds the end of a destination string and writes a source string into that position for a limited count, ensuring a terminator is present.
- `copy_limited_string_with_pad`: Copies bytes from a source pointer to a destination pointer for a given count, stopping on a zero byte, and fills any remaining space with zero bytes.
- `update_window_structure`: Calls a routine to check a condition, then if successful, calls several other routines to update a structure, including setting a window or icon property and updating a global pointer, before clearing a global variable.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `execute_recursive_function`: Executes a function pointer using a stack-passed parameter and recursively calls itself if the result is non-zero.
- `call_function_with_stack_param`: Sets up a stack frame, pushes a pointer to a value onto the stack, pushes a pointer to the current routine, and calls a function pointer passing the current routine and a pointer to a value on the stack.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: Clears d4, checks the sign of d0 and d1, negates d0 or d1 if negative, and calls a routine to perform a division.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1, employing a bit-shifting algorithm or standard modulo arithmetic, and returns the result.
- `call_function_via_pointer`: Calls a function pointer using a value from a global address and a provided parameter.
- `process_structure_list_2`: Saves d4, loads a pointer from a structure, pushes a pointer from a structure, and calls another routine; if the loaded pointer equals 10, it checks bit 7 of a byte at an offset of a pointer, and if bit 7 is set, it calls another routine with a certain value and a pointer from a structure, otherwise it continues a loop or returns.
- `jump_to_target`: Performs a short branch to a fixed location in another routine.
- `update_pointer_target`: Loads a pointer from a structure offset 12, compares a value at that pointer with a value at offset 4 of that pointer, and if the value is less than the comparison value, calls another routine with a masked value from a structure offset 8, otherwise it calls another routine recursively.
- `update_byte_from_source`: Loads a pointer from a register, increments the value at that pointer, reads a byte from a structure, writes that byte to the new pointer location, and then branches to another routine.
- `traverse_memory_range`: Iterates through a range of memory by stepping through a sequence of addresses and calling another routine at each step.
- `load_pointer_and_check`: Loads a pointer from an offset of 8 in a5 into a2, then checks if a2 is null, setting d0 to a certain value if null or a different value otherwise, before returning.
- `clear_structure_and_call_routines`: Checks a byte at offset 12 of a2, and if certain bits are set, calls several routines with parameters pushed from a2, then clears several offsets in a2 and a byte at offset 12, and finally returns a value from d4.
- `check_bit_and_set_address`: Sets up a stack frame, saves d4 and a2, loads a pointer from a structure offset 8 into a2, loads a constant address into a0 and stores it in a global variable, checks bit 4 of a byte at a structure offset 12, and if the bit is not set, clears d0 and restores registers before returning.
- `update_linked_list_node_4`: Checks a bit in a structure, and if set, calculates a pointer, calls another routine, and if certain conditions pass, clears a longword and certain bytes in a structure before branching.
- `clear_node_data`: Checks if a value at an offset of twelve in a5 is negative one, and if not, clears a bit in a memory location, clears a longword, and clears another longword.
- `update_linked_list_node_3`: Checks if a pointer at offset 8 of a2 is null, and if so, calls another routine, checks a value at offset 16 of a2, and if it equals 1, modifies a byte at a certain offset in a5 and calls another routine with a value from a2 and a pointer, otherwise it updates a pointer and a byte in a structure and calls another routine.
- `update_linked_list_node`: Updates a pointer in a structure to point to a new location, calculates a new value based on a short and a pointer, sets bit 2 of a byte at offset 12, increments a pointer, and copies a byte from a source offset 15 to a destination pointer before returning a masked version of that byte.
- `traverse_structure_list`: Traverses a list of structures, checking a byte at offset 12, and clears certain fields if a condition is met or if the end of a range is reached.
- `branch_to_target_8`: Performs a short branch to another location.
- `clear_structure_fields`: Clears a long-word at offset 0, a long-word at offset 4, and a long-word at offset 8 of a structure in a2, then copies a2 to d0 and branches to another location.
- `insert_node_and_set_flag`: Calls a routine with a constant, then if the return value is zero, sets a word at offset 16 of a structure and updates a pointer at offset 8, otherwise it sets a bit in a structure and updates a pointer.
- `set_node_flags`: Sets a word at an offset in a structure, sets a bit in a structure, and calls a routine, then sets a bit in a structure if the routine returns a non-zero value.
- `setup_list_traversal`: Links a5, saves registers a2 and a3 to the stack, and loads a pointer from a global variable into a2 before branching to another location.
- `traverse_and_call_list`: Iterates through a linked list of pointers, calling another routine with an offset, and clears a global variable upon completion.
- `set_list_head_and_increment_2`: Sets a pointer in a structure, calls a routine with an incremented pointer, and updates a global pointer if the result is non-null.
- `set_list_head_and_increment`: Copies a pointer from a global variable to a structure, copies a long-word from a structure into a structure, updates a global variable with a new pointer, and branches to another location while calculating an offset in d0.
- `forward_list_insertion`: Passes its argument directly to a linked list insertion helper and returns.
- `clear_register_and_branch`: Clears a register and branches to another instruction.
- `copy_pointer_if_not_zero_2`: Checks if a pointer or address is equal to another, and if not, copies a long-word from the address in a2 to a2 and updates d0 before returning.
- `copy_pointer_if_not_zero`: If d3 is not zero, copies the long-word at the address in a3 to the address in a2, otherwise it branches away.
- `call_library_with_offset`: Loads a library base from a global variable, calculates an offset, pushes a pointer and a value onto the stack, calls a library function, and cleans up the stack before branching to another location.
- `check_list_node_validity`: Calls a routine, then checks if a pointer or a value in a structure is within a certain range, and if certain conditions or a null pointer check fail, sets a global variable to a constant and returns a certain value.
- `call_is_interactive_2`: Loads a value from a global variable, calls a routine, and checks the result to either branch or return a specific value.
- `clear_d0_and_branch_5`: Clears register d0 and jumps to another routine.
- `call_delete_file_and_check_error`: Pushes a value from a global variable onto the stack, calls a routine, and if the result is non-zero, calls another routine and updates a global variable with the result.
- `clear_d0_and_branch_6`: Clears register d0 and jumps to a subsequent instruction in another routine.
- `validate_index_and_pointer`: Calls a routine to retrieve a value, then checks if that value is within a certain range and if a pointer at a calculated offset in a structure points to a non-zero or validly flagged memory location; if any check fails, it updates a global variable with a constant and returns -1, otherwise it calls another routine with the pointer and parameters.
- `check_bit_and_set_error`: Checks if bit 1 of a word at an offset in a2 is set, and if not, sets a global variable to a constant and returns a certain value.
- `process_io_error_and_branch`: Pushes three values onto the stack, calls a routine, and if the result is not negative one, calls another routine and updates a global variable.
- `copy_d5_to_d0`: Copies the contents of d5 to d0 and branches to another location.
- `call_function_and_check_bit`: Calls a routine with a pointer, checks bit 12 of the result, and if certain conditions or a global pointer check pass, moves the result to a local variable and returns.
- `call_function_and_clear_d0`: Calls a routine using a pointer from a global variable, clears d0, and branches to another location.
- `execute_callback_if_set`: Checks if a pointer in a structure is non-zero and, if so, executes the routine pointed to by that address.
- `check_pointer_and_branch`: Pushes a register to the stack, copies a longword from a pointer, and if a certain memory location is zero, branches to another instruction; otherwise it clears a register and branches.
- `execute_conditional_chain`: Executes a loop that calls a routine while incrementing a counter, then calls several routines or updates pointers and values in a structure if certain pointers or bit conditions are met, and finally calls a routine using a pointer from a structure.
- `execute_conditional_function_4`: Checks if a pointer at offset 4898 of a structure in a4 is null, and if not, calls a library function using a pointer from a structure, then calculates a new value by adding one and shifting left by two, calls another function with this value, and returns a value from a pointer-based memory location.
- `call_indirect_with_offset`: Executes a function pointer retrieved from a table via a global offset and returns a value from a pointer-based memory location.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `clear_and_branch_on_bit_2`: Calls a routine, checks if a value is within a certain range, and if a pointer or bit condition is met, clears a long-word at a certain address and returns a specific value.
- `clear_and_branch_on_bit`: Checks if bit 15 of a word at offset 4 of a2 is set, and if not, clears a long-word at a2, calls a routine, increments the stack pointer, and clears a long-word at a2 before branching.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `jump_to_close_handler`: Performs an indirect jump to a function pointer retrieved from a structure at an offset relative to a global pointer.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `jumps_to_indirect_routine`: Performs an indirect jump to a routine located at an offset relative to a pointer retrieved from a memory location or a structure.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_ex_next`: Wrapper that calls the dos library function ExNext and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `jump_to_indirect_target`: Loads a pointer from a memory location calculated using a base address and an offset, then jumps to the address stored at that location minus a constant offset.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `jumps_to_indirect_routine_2`: Loads a function pointer from a table located at an offset from a pointer stored in a global variable and jumps to that address.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_parent_dir`: Wrapper that calls the dos library function ParentDir and returns its result to the caller.
- `executes_indirect_jump`: Performs an indirect jump to a routine address retrieved from a pointer-based table or structure located at an offset from a base address in a register.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `jumps_to_indirect_handler`: Loads a function pointer from a table or structure and jumps to it, calling the seek library function via this indirect jump.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `jump_to_indirect_handler`: Loads a function pointer from a table located at an offset from a pointer stored in a global variable and jumps to it, which subsequently calls the library function for unlocking a file or resource.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_450_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `jumps_to_indirect_address`: Performs an indirect jump to an address calculated by adding a constant offset to a pointer retrieved from a global variable.
- `call_library_vector_414_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `update_pointer_chain`: Calls a routine to check a value, then if not negative, calls another routine to check a pointer, and if valid, populates a structure with a pointer, a byte, and a byte, then calls a third routine to update a pointer, and finally calls a fourth routine to update a pointer or calls itself recursively.
- `reset_structure_members`: Recursively calls itself if a certain field in a structure is non-zero, then sets a byte and a dword in that structure to certain values before calling another routine and calling a library function.
- `call_library_vector_354_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_function_with_argument`: Calls a function pointer derived from a global variable and passes a constant value as an argument.
- `call_function_single_argument`: Calls a function pointer derived from a global variable and passes an argument.
- `update_structure_from_pointer`: If a pointer or a value is non-zero, it calls another routine with a constant, and if that routine returns a non-zero value, it writes a constant byte to offset 8 of a structure, and copies a 16-bit value and a 32-bit pointer to offsets 18 and 14 of that structure.
- `load_and_check_pointer`: Loads a pointer from a certain offset of a5 into a2, then checks if a2 is non-zero before returning or otherwise returning.
- `initialize_structure_members`: Sets a byte at offset 8 of a2 to certain value, sets two long-words at offsets 20 and 24 of a2 to certain value, and calls another routine using a pointer-based address before branching back to an earlier instruction.
- `call_library_vector_198_5_2`: Calls a library function at an offset relative to a library base loaded from a global variable.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `process_pointer_chain`: Calculates a 32-bit aligned address, copies a block of 8 or fewer words from a library-relative source to a local stack buffer, and if a routine returns a non-zero value, it updates a structure with several pointers and a byte, then calls other routines recursively.
- `call_with_frame_pointer`: Pushes the pointer stored at offset 8 of the frame onto the stack, calls a routine via a PC-relative address, removes the pushed value, and returns.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `jumps_to_indirect_target`: Performs an indirect jump to an address calculated by adding a negative offset to a pointer retrieved from a global variable.
- `call_library_vector_294_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `jump_to_next_instruction`: Performs an indirect jump to an address calculated by adding a constant offset to the current program counter.
- `call_library_vector_132_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_to_indirect_address`: Performs an indirect jump to an address calculated by adding a small offset to a pointer retrieved from a global variable.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `jumps_to_indirect_address_2`: Loads a pointer from a global variable, applies a negative offset, and jumps to the resulting address.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain_3`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_138_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_360_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `jump_to_library_function`: Performs an indirect jump to a library function address calculated from a pointer in a global variable.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `jump_to_library_function_offset`: Performs an indirect jump to a library function address calculated from a pointer in a global variable with a different offset.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_258`: Library stub: loads a library base from a global variable and calls the library function at offset minus 258 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_30_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_606`: Library stub: loads a library base from a global variable and calls the library function at offset minus 606 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `call_library_vector_552_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_582`: Library stub: loads a library base from a global variable and calls the library function at offset minus 582 of that base, returning its result to the caller.
- `call_library_vector_282_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_318_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_306_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_96_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_438`: Library stub: loads a library base from a global variable and calls the library function at offset minus 438 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_54_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_120_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_330_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_468_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_432`: Library stub: loads a library base from a global variable and calls the library function at offset minus 432 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_444_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_234_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_240_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_390_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_252_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_264_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_282_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `update_scaled_value`: Decrements a value from a register, loops while a counter is non-zero, potentially XORs a value with a constant, and if a certain condition is met, calculates a high-bit product and stores a value in a global variable.
- `multiply_high_bits`: Multiplies two 16-bit values in d0 and d5, then returns the upper 16 bits of the 32-bit product.
- `transform_input_value`: Doubles a value in d0 and, if the carry flag or a zero-check passes, XORs the result with a constant before returning.
- `call_library_vector_30_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_30_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `check_and_return_status`: Calls a routine with a provided parameter and certain flags, then returns a value based on the or result of a second routine call.
- `process_parameter_array`: Initializes several elements of an array with zero, calls a routine using a provided parameter, and returns a status or a specific constant based on the result or comparison or calls another routine if a certain condition is met.
- `copy_struct_and_validate`: Copies several words from a source structure to a destination buffer and checks if a certain value exceeds a limit or if a certain bit in a byte is set.
- `update_buffer_bounds`: If a pointer is not null, it updates a destination memory location using a difference or sum and updates a pointer or counter.
- `increment_pointer_on_success`: Calls a function via a pointer and, if the result is not a failure code, increments a value in a structure by a given amount.
- `validate_and_update_range_2`: Calculates a range or pointer offset, calls a function via a pointer, and updates a structure member or a global variable based on the return value and certain string-like constants.
- `validate_and_update_range`: Checks if a value is within a valid range and calls a function via a pointer, updating two different memory locations if the call succeeds.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `dispatch_string_function`: Calls a routine to initialize a parameter table, then compares the result against four specific 32-bit constants to call a corresponding function pointer from a table provided in a5.
- `update_structure_member_3`: Writes a one to a certain offset in a structure in a5, pushes a pointer to a certain offset in a5 onto the stack, calls a routine that copies a structure and validates a value, and then stores the returned dword into a certain offset in the structure in a5 before returning.
- `check_and_clear_structure_member`: Loads a pointer from a structure in a5 into a0, checks if a certain dword at an offset of that pointer equals a specific constant, and if so, clears a 16-bit value at an offset in the structure in a5 before branching to another routine.
- `update_pointer_from_structure_5`: Copies a 32-bit value from a structure member to a stack location, calls another routine, and then stores the returned value back into a stack location before branching to another routine.
- `set_structure_value_2`: Checks if a value at an offset in a structure is non-zero, then calls a routine using a pointer from a structure and stores the return value in a location relative to a5.
- `set_structure_value`: Writes a constant value to a location relative to a5 and then branches to another routine.
- `call_pointer_from_structure`: Loads a pointer from a structure, calls it with a pointer to a structure member as an argument, and then stores the return value in a structure member before branching.
- `call_function_from_pointer`: Loads a pointer from a structure, calls the function pointed to by a pointer stored in that structure, and then stores the return value in a structure member before branching.
- `call_indirect_function`: Loads a pointer from a structure, loads a second pointer from a location relative to that pointer, calls the second pointer, and then stores the return value in a structure member before branching.
- `process_string_list_2`: Checks d0 against several constant values and, if a match is found, calls a routine using a pointer from a structure and stores the result in a structure member.
- `load_and_branch`: Loads a value from a structure member into d0 and then branches to another routine.
- `pass_argument_to_routine`: Passes a single parameter from offset 8 of a structure in a5 to another routine while setting a second, zero-valued parameter on the stack.
- `validate_and_set_error`: Calls another routine and, if the returned value is a fixed address, sets a longword at offset 20 of a structure in a1 to negative one and returns -9.
- `update_structure_member`: Saves registers to the stack, loads a pointer and a secondary pointer from a structure, calls another routine, and then stores the return value in a structure member before branching back.
- `set_d4_and_branch`: Loads a constant into d4 and then branches to another routine.
- `call_function_and_store_result`: Pushes a pointer from a structure onto the stack, calls a function via a pointer found in a structure, and then stores the return value in d4 before branching.
- `pushes_pointer_and_calls_routine`: Pushes a 32-bit value from a global variable offset 8 into the stack, calls a routine using a pointer stored in a2, adjusts the stack pointer, and moves the return value into a register before branching to another routine.
- `call_function_from_offset_and_store`: Pushes a pointer from a structure onto thestack, calls a function via a pointer found at an offset in a structure, and then stores the return value in d4 before branching.
- `update_table_entry_3`: Compares d0 against several constants, and if a match is found, copies d4 to a structure member and updates a pointer or counter.
- `parse_move_notation_string`: Iterates through a string, masking characters to calculate an index into a table, and updates a structure or calls other routines based on the character or a subsequent character.
- `checks_and_jumps_to_table_entry`: Subtracts 65 from a value in d0 and compares it to 23; if the value is less than 23, it branches to another routine, otherwise it shifts d0 left by one, loads a word from a table using d0 as an offset, and jumps to the address or offset contained in that word.
- `branch_to_target_6`: Performs a short branch to a specific address.
- `return_immediately`: Returns immediately after unlocking a5.
- `initialize_audio_buffers_and_setup`: Clears a range of memory in a global table, then calls a routine to populate or check audio buffers, and if successful, populates a structure with certain values before calling a routine to update a field and another routine to check a status bit or call a handler.
- `process_handler_list`: Iterates through a list of bytes, calling a routine and passing a pointer to a structure if a byte equals one, then resets a counter and processes a second list of pointers, calling a routine with a pointer from a table and potentially calling another routine if a certain value is zero.
- `execute_list_and_update_pointer`: Pushes two values onto the stack, calls a routine, and then checks a value in a table; if it is zero, it increments a local counter and potentially calls another routine and a branch-to-subroutine.
- `execute_list_of_routines`: This routine checks if a pointer is non-null before calling a routine, then iterates through two different tables to call routines with certain pointers or parameters, and finally calls a routine with a pointer from a table.
- `update_structure_field`: Copies a value from a global address into a structure and updates a flag or field within that structure before calling two other routines.
- `execute_branch_routine`: Creates a new stack frame, calls another routine, and then checks if a value at a certain offset from the stack pointer equals a certain value, otherwise it clears d0 and branches to another instruction.
- `update_stack_value`: Copies a 32-bit value from a stack offset to d0, copies d0 to a different stack offset, clears a word at another stack offset, and then branches.
- `set_table_entries`: Checks a byte in a table and, if zero, sets a byte to one and populates several table entries with values from a global pointer and a calculated offset, otherwise it loops through a range.
- `increment_and_check_table_value`: Increments a word at a certain offset in a structure, compares it to a constant, and if it is less than that constant, calls another routine with a pointer to a structure and a constant, then updates a global variable with a value from the structure and loops if that value is not zero.
- `update_table_entries_4`: Updates several fields in a structure pointed to by a pointer in a5, including a longword from a table, a longword from a table, a word from a table, and increments a word in a table, then loops back to a previous instruction.
- `push_and_call_handler`: Clears a local stack value, calculates an offset, pushes a value from a table onto the stack, calls a routine, and then pushes two more values onto the stack before calling another routine and branching.
- `increment_and_compare`: Increments a value at an offset in a stack-based pointer and otherwise returns if the value is less than two.
- `compare_and_decrement_table_entry`: Compares a longword from a structure to a longword in a table, and if they match, calls another routine with a pointer to a structure and a constant, then checks a value at an offset in a table and, if it is not zero, decrements a byte at a calculated offset in that table before returning.
- `compare_and_clear_byte`: Compares a longword from a structure to a longword in a table; if they match, it updates a word in a structure and, if that word is less than 20, clears a byte in a table, otherwise it returns or branches.
- `set_register_and_branch_2`: Sets d0 to negative one and branches to another address.
- `set_register_and_branch_7`: Sets d0 to zero and then branches to another location.
- `copy_global_to_struct`: This routine copies several values from a global table or address into a provided structure or memory block.
- `initialize_structure_and_call_routines`: Initializes a local structure with values from a table, calls copy_global_to_struct, sets a word and a byte in the structure, calls two other routines, and if a bit in the structure is clear, calls another routine before returning or otherwise returning a value.
- `set_register_and_branch_3`: Sets d0 to zero and branches to another address.
- `update_structure_fields_2`: Creates a stack frame and calls a routine to check a value, then if non-zero, calls another routine and several others to fetch values from a table using an offset, updates several fields in a structure, and finally calls a routine to update a value and returns.
- `set_d0_to_negative_one_and_branch`: Sets register d0 to negative one and branches to another routine.
- `initialize_pattern_tables`: This routine loops one hundred and twenty-eight times to fill three different memory buffers with a sequence of incrementing or static byte values.
- `populate_buffer_with_repeating_data`: This routine repeatedly copies a portion of a source buffer into a destination buffer, doubling the amount of copied data in each pass.
- `initialize_and_populate_buffers`: Calls a routine to fill three buffers, then checks if any buffer pointer is null; if not, it calls a routine with three arguments, then calls another routine or copies values to a global table.
- `parse_game_command_and_update_state`: Checks if a structure contains a specific magic signature, copies a sequence of bytes from a source pointer into a local buffer, and then dispatches or executes different logic based on the value of a command identifier, which may involve updating a local counter or copying data from a table or a global variable.
- `parse_resource_header`: Initializes local variables, copies a pointer to a local buffer, and calls a routine that parses a resource header using a provided pointer.
- `parse_resource_header_2`: Initializes local variables, copies a pointer to a local buffer, and calls a routine that parses a resource header using a provided pointer, then checks a return value.
- `call_function_with_bounds`: Calls a function with a new length or size parameter, then calls another routine to write a value to a table using an incremented return value.
- `populate_grid_with_offsets`: Populates an array with values from a function, then uses nested loops to call a routine that calculates or sets certain coordinates, and finally calls a routine to reset two different table entries.
- `apply_decreasing_offsets`: Iterates through a table of ushorts and calls a routine that calculates or sets coordinates, decrementing an offset value in each pass.
- `copy_table_and_set_flag`: Copies a range of words from one table to another, calls a function, and updates a global variable and a fixed constant.
- `reset_global_flag`: Calls a function and then sets a certain global variable to zero.
- `execute_complex_branching`: Checks a pointer in a structure and calls a routine if non-null, then proceeds to call several other routines or enter loops that call routines with calculated offsets or indices, and finally checks a bit in a global variable to conditionally call more routines.
- `calculate_negative_offset_from_d5`: Subtracts the value in d5 from 7, stores the result in a pointer in a2, and if the resulting value is negative, negates it and stores it in d0; otherwise, it returns or branches to another routine.
- `load_and_compare_d6`: Loads a value into d6, then checks if d5 is less than 4, and if so, sets d0 to a constant and branches.
- `iterate_and_call_subroutines_2`: Calls a routine using a pointer from a4 and an argument in d4, then calls another routine with the sum of the previous return value and d4, and loops while d4 is less than 4 or d7 is less than 16 or d5 is less than 8.
- `clear_d5_and_branch`: Clears register d5 and branches to a different location in the code.
- `check_and_move_register`: Tests a value at an offset in a global variable and, if it is not zero, moves a register to another register and branches to another routine.
- `calculate_and_call`: Subtracts a value from a global variable, calls another routine, and stores the result in a register.
- `branch_with_subtraction`: Pushes a value from a pointer onto the stack, checks a global variable, and if non-zero, calculates a difference and branches.
- `iterate_and_call_subroutines`: Adds d7 to d4, pushes the result to the stack, calls a routine, and then checks if d4 or d5 (incremented or otherwise) exceed certain values in a structure, potentially calling multiple other routines or checking a byte in a table before returning.
- `process_conditional_drawing`: Checks a byte in a table, and if non-zero, calls a routine with values from another table, potentially calling further routines and updating a drawing or windowing routine with new bounds.
- `initialize_structure_and_check_bit`: Clears a 16-bit value at an offset in a5, copies a word from a structure, and checks if a specific bit is set in a value loaded from a structure before returning.
- `iterate_and_update_structure_elements`: Iterates through a list of pointers, updating a counter and a pointer in a structure, and if certain conditions regarding a value or a pointer-to-table-entry are met, calls a routine to process an element, calls another routine with a range, and potentially sets bit 6 of a byte in a global table.
- `update_and_call_with_loop`: Updates a local variable, checks a value against a constant, and if equal, calls another routine with several arguments before updating a global pointer and branching.
- `execute_conditional_event`: If a certain long-word or a certain word in a structure in a5 is non-zero, it pushes a constant, calculates a value from a word in a5, and calls a routine using that value as an argument.
- `process_list_and_set_values`: Calls a routine with several parameters, then iterates through a list to assign values or clear entries based on a condition.
- `clear_array_elements_3`: Sets up a stack frame, loads a value from a register, clears a register, and branches.
- `clear_array_elements`: Iterates through a table and, if a certain value is non-zero, calls a routine using a calculated address before setting that table entry to zero.
- `pass_split_bit_fields`: Calls a routine and passes a value from a global variable along with certain bit-shifted fragments of a provided parameter.
- `clear_specific_bits`: Performs a bitwise AND with a bitwise NOT of a parameter on a global variable to clear certain bits.
- `execute_two_subroutines`: Calls two different subroutines using a pointer from a global variable and passed parameters.
- `execute_sequential_subroutines`: Calls a series of different subroutines using a pointer from a global variable and passed parameters.
- `process_pointer_range`: Calculates a new offset or a decremented value and calls another routine twice with different pointer-based arguments, then returns.
- `update_pointer_index_2`: Checks a longword in a structure, and if it is not negative, calculates a new offset by shifting a value from a previous pointer and adding a constant, then updates a pointer in the structure before branching to another routine.
- `update_pointer_index`: Calculates a new value based on a difference between two pointers, shifts it, and updates a pointer in a structure before branching.
- `branch_to_offset`: Performs a relative branch to a different location.
- `update_counter_and_pointer_2`: Updates a counter in a structure using a value from a pointer, then conditionally updates a different counter using a pointer or a secondary.
- `update_counter_and_pointer`: Updates a counter in a structure using a value from a pointer, then conditionally updates a different counter using a pointer or a secondary pointer, and finally updates a pointer or a constant.
- `decrement_counter_and_dispatch_routine`: Decrements a counter in a structure, checks a bit in a structure, and calls another routine with arguments or updates a pointer and a constant.
- `update_state_and_call_routine_2`: Copies a value from a structure, writes a constant to a structure, and conditionally clears a word in a structure before branching.
- `update_state_and_call_routine`: Updates a byte in a structure, calls a routine with a large number of arguments from a structure, and conditionally clears a bit or calls another routine.
- `update_structure_and_call_handler`: Copies several words and a byte from a structure, updates a global variable with a bit-shifted value, and calls a routine using a pointer from a structure before potentially branching.
- `calculate_scaled_index`: Copies a pointer, loads a pointer from a structure, reads a byte, calculates a scaled value, and writes it to a global variable before branching.
- `update_handler_state`: Checks if a pointer is null, then copies two shorts from a table or structure to global memory, calls a function via a pointer, calls another function, and resets a pointer and a global variable.
- `store_registers_to_memory`: Stores several passed parameters and a byte into a contiguous block of memory and updates a pointer.
- `toggle_bit_and_load_address`: Checks a bit in a global variable and sets or clears it, then checks another bit in a global variable and, if clear, loads a pointer into a register.
- `execute_conditional_function_5`: Calls a routine with a pointer and then conditionally calls another routine based on a bitmask check.
- `invoke_function_with_arguments_2`: Sets up a stack frame, loads a value from a pointer, and branches.
- `invoke_function_with_arguments`: Calls a function using a pointer from a global or relative address, passing several arguments.
- `execute_conditional_update`: Calls two different routines, updates a value at a pointer, and if that value is zero, clears several stack locations and calls a third routine before returning.
- `set_stack_value`: Writes a constant value to a location on the stack.
- `update_table_with_offset`: Loops through a range, calculating a new address based on a value from a structure and a previously read value, then stores that address into a table, and increments a counter in a structure until a certain value is reached.
- `clear_array_elements_4`: Checks if a pointer or a global variable is null, calls a routine, and if not null, sets a certain number of array elements to zero using a loop.
- `clear_array_elements_2`: Iterates through an array, clearing memory or setting it to zero, by using a pointer from a structure and an incrementing counter, until a certain limit or end-address is reached.
- `update_event_parameters`: Checks a value in d0 against several constants and, depending on the match, writes a set of three longwords to a stack-based structure, potentially updating a global variable or a pointer, and then calls a routine to update a stack frame or a structure or calls another routine with a new value or a pointer or otherwise returns.
- `setup_and_copy_data`: Writes three longwords to a stack-based structure, calls a routine, updates a global variable, and then calls two other routines using a pointer and a calculated value.
- `update_table_value_and_call_subroutine`: Writes three longwords to a structure or stack frame, calls a routine to retrieve a value, increments that value by sixty-five, stores it back into a global variable, and then calls another routine passing a pointer and a value, followed by a call to move a longword from a table into a pointer-based location.
- `calculate_and_set_index`: Writes three longwords to a stack-based structure, calls a routine, and if a certain condition is met, updates a global variable with a value derived from a table lookup and a constant, then calls two other routines.
- `divide_and_store_result`: Writes three longwords to a stack-based structure, performs a division on a value from a global variable, stores the quotient in a global variable, and calls two other routines.
- `load_and_set_pointer_2`: Loads a longword from a pointer, calls a routine, and then writes a value to a stack-based location.
- `load_and_set_pointer`: Loads a longword from a pointer, calls a routine, stores the result back to a stack-based structure, calls another routine, and then writes three longwords to a stack-based structure.
- `update_pointer_from_table`: Calls a routine, uses the result to index a table, and writes a value to a stack-based structure, then calls another routine, updates a stack-based structure, and calls two other routines.
- `update_stack_frame_and_call_subroutine`: Checks a value in d0 against several constants and, depending on the match, writes a set of three longwords to a stack-based structure, potentially updates a global variable or a value in a table, and calls a routine with a pointer or a calculated value.
- `move_memory_to_pointer`: Moves a longword from a table indexed by a register into a location pointed to by another register, then decrements that pointer.
- `move_memory_to_pointer_decrement`: Moves a longword from a table indexed by a register into a location pointed to by another register, then decrements that pointer.
- `decrement_pointer_and_load_table_entry`: Decrements the pointer in a2 and loads a longword from a table or array indexed by a d6-scaled offset from a5 into d6.
- `move_table_to_pointer_self`: Moves a longword from a table indexed by a register into a location pointed to by a register, then decrements that register.
- `decrement_pointer_and_load_table_entry_2`: Decrements a pointer in a2 and loads a longword from a table or array indexed by a register-scaled offset (d6 multiplied by 4) into d6, using a pointer held in a3.
- `move_table_to_pointer_v3`: Moves a longword from a table indexed by a register into a location pointed to by a register, then decrements that register.
- `execute_conditional_branch_2`: Loads a pointer from a structure, calls a routine, and if the result is not zero, calls another routine before branching to a different location.
- `unlock_interrupt_controller`: Clears the interrupt disable bit in the status register and returns immediately.
- `process_buffer_with_bounds_check`: Clears a value, calls a routine, and if the result is not zero, checks if a certain buffer size or certain value matches a constant, then calls another routine if certain conditions are met.
- `load_pointer_and_return`: Loads a 32-bit value from an offset of minus 4 from a5 into d0 and returns.
- `check_display_index`: Clears two 32-bit values at offsets -32 and -36 from a5, loads a 32-bit value from a4, and if the lower 23 bits are non-zero, otherwise it returns.
- `update_display_buffer_index_3`: Calculates a new display buffer index or wraps a pointer-based index within a certain range, then updates a display buffer byte by masking and bit-shifting, and calls several other routines, in.
- `update_display_buffer_index_2`: Calculates a new index or wraps a pointer-based index within a certain range, then updates a display buffer byte by masking and bit-shifting values from a table, and calls another routine with the new index.
- `set_d0_and_branch_3`: Sets d0 to 13 and branches to a nearby instruction.
- `update_display_buffer_index`: Calculates a new index, updates a byte in a display buffer using a bit-shifted value from a table, and then calls several other routines, including a recursive call and a routine that clears a bit in a global variable, before returning.
- `check_bit_and_return`: Checks if bit 31 of a value loaded from a pointer-relative offset is set, and returns if it is not.
- `iterate_and_process_table_entries`: Checks if a range or list exists, then enters a loop that extracts a byte from a table, passes its high and low bytes as arguments to another routine, and clears a value in a structure if a certain condition is met.
- `execute_multiple_routines`: Sequentially calls several routines with specific or pointer-based arguments and then calls a final routine.
- `modify_attribute_and_call_function`: Calls a function with a pointer, calculates a new pointer, sets bit 7 of a byte at offset 13 of that pointer, and calls another function with certain parameters before clearing a bit in a word at offset 12 of that pointer.
- `transform_string_and_invoke`: Calls a routine, clears a value, and then loops through a string, calculating a new byte value for a local buffer based on the original byte and an incrementing counter, before calling two other routines.
- `clear_byte_and_invoke_functions`: Clears a byte at a calculated offset from a5, then calls two different functions using values from a5 and a pointer.
- `initialize_stack_frame`: Clears a long-word at a negative offset from a5 and branches to another routine.
- `populate_table_with_function_results`: Iterates through a loop, calling a function to retrieve a value and storing that value into a table, while incrementing a counter.
- `transform_string_buffer`: Executes a series of routines that pass a buffer and various constant or pointer-based arguments to modify or access memory or strings.
- `load_pointer_to_a4`: Loads a pointer into a register from a relative offset and then copies the value from that pointer into a register.
- `load_pointer_to_a4_2`: Loads a pointer into a register from a relative offset and then copies the value pointed to by that pointer into a register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `reset_move_list`: Clears a global variable to zero and calls a routine that uses a pointer and a constant to update a list and a temporary buffer, then stores the return value in a global variable.
- `clear_global_pointer_2`: Sets up a stack frame, checks a global variable, and if it is not zero, sets a global variable to one, then checks that same global variable and branches.
- `clear_global_pointer`: Reads a pointer from a global variable, calls the routine pointed to by that pointer, and then sets that global variable to zero before returning.
- `update_move_list_and_evaluate_position`: Checks move validity and shifts a temporary buffer or array when a move is validated, while also updating a bitmask or pointer-based lookup table related to a move or position state.
- `invoke_subroutine_with_parameters`: Calls another routine using two provided parameters from the stack.
- `call_if_match`: Compares a parameter against a global variable and, if they match, calls another routine with provided parameters.
- `execute_conditional_function`: Checks if three global or external pointers or values are non-zero and non-empty, then calls a function using those values before zeroing two global variables.
- `update_table_values`: Updates a pointer-referenced value and, if certain conditions or value ranges are met, clears three global variables or copies new values into a table.
- `copy_values_to_table`: Copies three dwords from one memory location to another and returns a value from a third location.
- `increment_and_call_if_valid`: Checks if a parameter matches a global variable and if a secondary global value is non-zero, then potentially increments a global counter or calls another routine.
- `load_pointer_and_call`: Checks if a value at offset 336 of a4 is non-zero; if it is between 1 and 5, it loads d0 with a value from a5, calls a routine, and adds a value from a4 to d0.
- `copy_pointer_data`: Copies a pointer or value from a global variable or table and passes it as an argument to another routine.
- `update_table_entry_and_call_handler`: Calculates a table index from a value in a structure, negates it if negative, and uses it to fetch a pointer to a local buffer or a handler.
- `format_win_messages`: Uses a string-formatting routine to prepare and display a message containing a player or score, then updates a global pointer or table-based value and calls a routine to handle event sequence logic.
- `set_constant_value`: Sets a long-word value of 4 to a location relative to a5 and then jumps to a nearby instruction.
- `branch_on_d0_and_jump`: Compares d0 to a constant, and if d0 is not less than that constant, shifts d0 left and jumps to a target address calculated from d0.
- `update_event_sequence_logic`: Compares a pointer to a value, and if not equal, calculates an offset, pushes a value from a table to the stack, and calls a routine; otherwise, it updates a global variable and calls another routine.
- `update_event_sequence`: Copies a value from a structure offset to a global variable, updates a global variable with a constant, and calls a routine that may push a value onto the stack and call another routine or call a routine with a constant or a pointer-based value.
- `process_event_sequence`: Checks a global variable and calls a routine if it equals two, then if a certain value in a5 is one, it calls several routines, including one that uses a value from a table, and otherwise calls a routine with a constant or a pointer from a global variable.
- `execute_event_loop`: Executes a routine, sets a value in a table, and enters a loop that checks for specific return codes from a function, potentially terminating or updating a flag or counter, before cleaning up a table entry.
- `initialize_table_elements`: Runs a loop twice to query a function for a pointer, then calls routines to assign values or pointers from a table to a destination address.
- `calculate_string_offsets_2`: Calculates several offsets using values from a table and a pointer, stores these offsets in a structure, and if a bit in a global variable is set, performs a series of XOR operations on those stored offsets before branching.
- `calculate_string_offsets`: Checks if a global variable is non-zero, then updates several offsets in a structure by subtracting a value from a global pointer, and finally calls a routine to format a string using a calculated character and certain pointer-based offsets.
- `format_string_output_2`: Prepares arguments including a constant, a pointer, and a length, then calls a routine to format a string, using a specific character if certain conditions in a5 are met, otherwise using another character.
- `format_string_output`: Calls a routine to write a formatted string using a certain or a global variable and a calculated length or pointer.
- `setup_stack_and_test_pointer`: Sets up a stack frame, clears a long-word at offset -4 of a5, and checks if a long-word at offset 308 of a4 is or contains a negative or zero value, otherwise it unlinks the stack and returns.
- `execute_conditional_calls_3`: Checks if a value at offset 8 of a5 is non-zero; if so, it loads a pointer from a4, calls a routine, pushes a constant, calls another routine, adjusts the stack, pushes two or more addresses, calls a routine, adjusts the stack, and stores d0 into a global variable.
- `initialize_string_buffer`: Clears a range of memory in a structure, calls a routine with a pointer to that memory, and then updates a counter in a structure before branching.
- `execute_branching_logic`: Pushes a pointer from a table, calls a routine, and then checks a certain word in a5, otherwise it sets d0 to a constant and branches.
- `process_string_and_branches`: Performs a loop that calls a routine with a pointer and a constant, increments a counter in a structure, and then conditionally calls other routines or calls a routine with a negated value from a table or a pointer from a table, or calls a routine with a pointer and a calculated value, before potentially returning or branching.
- `execute_conditional_calls_2`: If a certain long-word in a5 is non-zero, it clears the stack, pushes a pointer, clears the stack, calls a routine, adjusts the stack, and calls another routine before branching.
- `register_callback_with_parameter`: Stores a parameter in a global variable and calls a function that takes a function pointer and a parameter.
- `update_pointer_and_store`: Increments a global pointer and stores a provided byte at the location pointed to by the original pointer.
- `store_parameter_and_call_function`: Saves a parameter to a global variable before calling another routine with a specific argument.
- `call_function_with_pointer`: Calls a function pointer using a parameter and a value from a global pointer.
- `load_string_pointer`: Sets up a stack frame, saves registers, and if a certain value in a structure is not 4, loads a pointer from a structure into a2 and a value from a pointer into d4.
- `load_string_from_table`: If a value in a structure in a5 is not non-negative, loads a value from a pointer in a structure into d4.
- `fill_string_from_table`: Iterates through a table or list, copying bytes into a buffer, while checking a sign or condition and potentially appending a character at the end.
- `parse_string_control_commands`: Parses a string to update a structure, checking a bit in a lookup table to decide whether to continue or branch, and handles certain control characters to modify string length, offsets, or pointer values in a structure.
- `clear_long_variable_2`: Clears a longword in a global variable at an offset of minus twenty-four from a5 and then branches to another location.
- `parse_string_control_characters_2`: Parses a string to update a length or offset in a structure, checking a bit in a lookup table to decide whether to continue or branch.
- `clear_structure_member_and_branch`: Clears a longword at a negative offset of 20 from a5 and branches to a different location in the string parsing routine.
- `parse_string_control_characters`: Iterates through a string, updating a length or offset in a structure, and checks a bit in a lookup table to decide whether to continue or branch to a different routine.
- `load_next_string_byte`: Checks if a register contains a specific constant, and if so, increments a pointer and loads the next byte from that pointer into a register, sign-extending it to a 32-bit value before jumping to a new address.
- `update_string_pointer_and_store_char`: Loads a pointer from a local structure offset into a0, updates a pointer in the structure to point to that address, and writes the byte from d4 to the memory location pointed to by a0 before branching back.
- `calculate_jump_offset_2`: Calculates a value, compares it to a constant, and if it is not less than that constant, shifts the value and uses it to jump to a new address.
- `parse_string_and_execute_commands`: Parses a string or a sequence of characters, checking for certain symbols, and calls another routine for each valid character or command, while updating a counter in a structure and potentially resetting or modifying certain flags and pointers in that structure.
- `branch_to_target_address_6`: Performs a short branch to a fixed address in another routine.
- `loop_and_call_function`: Pushes a value onto the stack, calls a routine, and loops if d0 does not equal a certain value.
- `increment_and_clear_pointer`: Increments a value at offset -8 of a5, loads a longword from offset -18 of a5 into d0, decrements the value at offset -18 of a5, and if d0 is greater than the value at offset -1c of a5, clears the value at offset -20 of a5 and branches forward.
- `loop_and_call_with_extended_value`: Increments a pointer in a structure, sign-extends a byte from that pointer, pushes it to the stack, calls a routine, and loops if d0 does not equal a certain value.
- `increment_pointer_and_update_structure`: Increments a value in a structure, loads a pointer from a structure, checks a byte at that pointer, and if not zero, calculates a difference, adds it to a structure value, and branches.
- `loop_and_call_with_parameter`: Pushes a parameter onto the stack, calls a routine, and loops if d0 does not equal a certain value.
- `decrement_and_compare_pointer`: Increments a value at an offset in a structure, decrements a value at another offset in a structure, and compares the new value against a constant.
- `execute_string_commands`: Calls a routine with a pointer saved in a register, then loops if the return value in d0 is not negative one, or otherwise sets d0 to negative one and branches.
- `increment_global_pointer`: Increments a value at an offset in a structure and branches back to an earlier instruction.
- `branch_to_target_address_7`: Loads a longword from an offset of minus eight from a5 into d0 and branches to a target address.
- `execute_conditional_function_2_2`: Pushes a constant onto the stack and jumps to another routine.
- `format_float_string`: Converts a floating-point or integer value into a string representation, handling signs, decimal points, and scientific notation, while updating a counter in a structure and writing characters to a buffer.
- `compare_and_update_structure`: Loads a longword from a pointer in a3 and a longword from a structure offset 8 into d0, calls another routine, and if the result is not negative, repeats the process by loading a longword from a3 into d1, loading a longword from a structure offset 8 into d0, calling another routine, and updating a structure offset 8 with d0 before incrementing a pointer in a structure offset minus 4 and looping.
- `check_range_and_clear`: Compares a value at an offset of a structure to a constant, and if not equal, moves a value from a structure to another location in that structure, then checks a value against a constant and potentially clears a value in a structure.
- `check_numeric_range`: Compares a value at an offset of one in a5 to a constant, and if not equal, loads a value from a5, adds it to a value at an offset of minus sixteen from a5, and branches based on whether that result is negative or within a certain range.
- `format_numeric_string`: Formats a signed integer and its fractional part into a string buffer, prepending a sign or a decimal point, and calls routines to write digits or update a counter or a pointer-based structure.
- `clear_offset_value`: Clears a longword at offset -12 of a5 and branches forward.
- `increment_and_store_value`: Loads a longword from offset -4 of a5 into d0, increments d0, and stores d0 at offset -c of a5 before branching forward.
- `format_integer_to_string_2`: Sets a value in a structure, checks a value in a structure, and if not zero, calls several routines to write characters to a buffer and update a structure, using a loop that decrements a counter.
- `format_integer_string_2`: Writes a character to a pointer, decrements a counter in a structure, and if a certain value is not zero, decrements another counter and branches.
- `format_integer_string`: If a certain value at offset 24 of a5-relative structure is not zero, it increments a pointer in a2, writes a byte with value 101 to that pointer, and otherwise checks if a certain value at offset -4 of a5-relative structure is negative, potentially negating it, incrementing a pointer in a2, and writing a byte with value 45 to that pointer before branching.
- `format_integer_to_string`: Converts an integer value to a string format by appending a plus sign and digit characters, or a certain amount of digits if a threshold is met.
- `initialize_and_call_function_2`: Pushes a constant onto the stack and jumps to a relative offset.
- `initialize_and_call_function_3`: Pushes a constant onto the stack and jumps to a relative offset.
- `initialize_and_execute_handler`: Pushes a constant onto the stack and jumps to a relative offset.
- `setup_stack_and_jump_to_handler`: Pushes a constant value onto the stack and jumps to a relative offset to execute a handler that initializes a global variable and calls a function or a fallback routine.
- `initialize_and_execute_handler_2`: Checks if a global variable is null, and if so, initializes several pointers and calls a function, otherwise it calls a fallback routine or executes a function pointer or a secondary fallback routine.
- `initialize_and_execute_handler_3`: Checks if a global variable is null, and if so, initializes several pointers and calls a function or a fallback routine, then executes a function pointer or a secondary routine using a value or pointer stored in a stack-allocated structure.
- `initialize_and_call_function_4`: Pushes a constant onto the stack and jumps to a relative offset to check if a global variable is null, otherwise it initializes several pointers and calls a function or a fallback routine, and finally executes a call to a function pointer or a secondary routine using a value from a stack-allocated structure.
- `initialize_and_call_function_5`: Pushes a constant onto the stack and jumps to a relative offset to check if a global variable is null, otherwise it initializes several pointers and calls a function or a fallback routine.
- `initialize_and_call_function_wrapper`: Pushes a constant onto the stack and jumps to a relative offset to execute a function that checks if a global variable is null, initializes pointers, and calls a function or a fallback routine.
- `initialize_and_call_function`: Checks if a global variable is null, then initializes several pointers and calls a function, or executes a fallback routine if an initialization certain value is zero.
- `execute_conditional_function_3`: Pushes a word onto the stack and jumps to an address or a function pointer relative to a pointer in a global variable.
- `execute_conditional_function_2`: Checks if a global variable is non-zero, and if so, calls a function using an offset from a pointer, otherwise it calls a routine that sets up a jump table and calls a function.
- `update_global_and_branch`: Calls a function to get the size of a global variable, passes that size to another routine, and then calls a routine with a constant value.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_552_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `update_structure_offset_table_2`: Updates a structure's offset or pointer table by calculating new pointer or offset values based on certain index or constant checks, and calls another routine to update the table.
- `update_structure_offset_table`: Calculates and stores new pointer or offset values into a table based on certain index or constant checks, and calls another routine to update a structure or pointer.
- `update_structure_values_with_pointer`: Iterates through a range of indices, checking if a value in a table matches certain constants, and if so, calculates a new offset or pointer and stores it into a destination structure.
- `update_structure_values`: If a certain value in a structure is one, it copies a pointer to a new location.
- `update_structure_members_2`: Clears a word in a structure, updates a pointer or value from a nested table, and calls a function conditionally based on a register value.
- `evaluate_and_sort_structure_pointers`: Calls a routine via a pointer in a5 to retrieve or update values, then compares or sorts three retrieved long-words.
- `compare_structure_values_2`: Loads a long-word from an offset of minus twenty-four from a5 into d0, compares it to a long-word from an offset of minus twenty from a5, and branches if it is less than or equal or otherwise branches to a different location.
- `update_pointer_offset_2`: Calculates a new pointer value by multiplying a value from a structure offset by eight and adding twenty, stores this new pointer back into the structure, and then checks if it has reached a certain limit before either continuing or updating a register with the new pointer.
- `update_pointer_loop`: Sets a value at offset -12 of a5, shifts it, and stores it at offset -16 of a5; then it performs a loop that updates a global array using a pointer, calls a routine, and updates a global array with an incremented value; finally, it copies several 32-bit values from a5 to a4 and sets a pointer in a4 based on a test of a value in a5, then calls another routine.
- `update_pointer_value_2`: Stores a pointer or value from a global location into another global location, increments it, calls another routine, and then updates a global variable with the return value before returning.
- `process_event_id_and_update_state`: Checks a value from a global pointer, and if it matches certain IDs, calls other routines or updates a global variable with a new value or a calculated offset.
- `execute_subroutine_with_stack_setup`: Clears a long-word at a negative offset from a5, pushes a constant onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `load_table_entry_from_offset`: Clears d0 and loads a 16-bit value from a location 14 bytes before the address held in a5, then branches to a nearby instruction.
- `calculate_target_address`: Subtracts a constant from a register, and if the result is less than four, shifts the register left, uses it to load a word from a table, and jumps to that address.
- `branch_to_target_address`: Performs a short branch to a target address located at a certain offset in a data pointer or table.
- `load_event_id_and_branch`: Loads a 16-bit value from a pointer-based offset in a4 and a 32-bit value from a nearby offset in a4 into a5 and d0, then branches to a fixed location.
- `calculate_and_store_offset`: Calculates a value by subtracting a constant from a value loaded from a global variable, stores this value in a global variable, and branches to another routine.
- `decrement_and_check_value`: Subtracts a constant from a register and, if the result is not zero, or if the register minus a constant or a further constant is not zero, otherwise it branches to another routine.
- `dispatch_instruction_opcode`: Loads an opcode from a pointer-relative offset in a4 and branches to a handler or updates a pointer in a structure based on whether the opcode matches a series of predefined constants or ranges.
- `setup_and_call_routine`: Sets a long-word to one at a negative offset from a5, clears a long-word at a negative offset from a5, pushes a constant onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `setup_pointer_and_call`: Loads a pointer from a table, pushes a pointer and two other pointers onto the stack, and calls another routine before adjusting the stack and branching to another routine.
- `pushes_pointer_arguments_and_calls`: Loads a pointer from a global variable into a0, pushes that pointer and two other pointers from a structure in a4 onto the stack, and calls another routine.
- `push_and_call_value`: Clears d0, loads a word from a negative offset of a5 into d0, pushes d0 onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `setup_recursive_pointer_update`: Loads a pointer from a global variable, pushes a new value and two existing pointers onto the stack, and calls a routine to update a pointer-referenced value recursively.
- `setup_recursive_pointer_update_2`: Loads a pointer from a global variable, pushes that pointer and two other pointers from a structure in a4 onto the stack, and calls a routine to update a pointer-referenced value.
- `execute_multiple_calls_2`: Pushes several values onto the stack, calls another routine, restores the stack pointer, stores d0 into a negative offset of a5, calls another routine, pushes a value from a global variable onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `initialize_and_iterate_data_table`: Clears or initializes several words in a structure pointed to by a4, calls a routine to initialize a pointer, and then enters a loop that calls another routine for every entry in a range of memory or a table, or otherwise returns.
- `process_instruction_dispatch_table`: Checks a value in d0 against a series of constants to decide whether to update pointers in a structure, call other routines with new pointer arguments, or execute a loop that calls a routine using a sequence of values or pointers loaded from a table.
- `decrement_and_branch_on_zero`: Decrements d0 by a constant and branches if the result is zero, repeating this for several different constant subtractions before a final branch.
- `branch_to_offset_2`: Performs a branch to a fixed offset.
- `load_value_and_return`: Loads a 32-bit value from an offset of minus 8 from a5 into d0 and returns.
- `toggle_bit_and_call_function`: Toggles bit 7 in a byte at offset 13 of a pointer and calls another routine with several arguments.
- `update_pointer_value`: Updates a pointer-referenced value with a new value and calls another routine recursively if the new value differs from the old one.
- `increment_and_store_parameter`: Increments a value passed via a pointer, then calls a routine to store the incremented value and the original parameter value into a global variable.
- `update_global_variables`: Calls another routine to store a local stack variable into a global variable, then calls another routine to store a parameter value into a global variable, and returns.
- `initialize_structure_members_3`: Clears a longword, loads two longwords from a pointer-relative offset, checks if a value at an offset equals a constant, and if so, loads a constant into d0 and branches.
- `initialize_and_pack_structure_members`: Checks a pointer-based value, and if it equals a specific constant, initializes a string, copies several bytes from a table into a structure, and then packs multiple bit-shifted values from a global table into a single long-word in a structure before executing a series of recursive calls and nested loops that populate or check structure members.
- `initialize_and_pack_structure`: Calls a routine with several hardcoded pointer offsets, then executes a nested loop that increments and stores values into a structure, followed by a loop that populates a structure with values from a table.
- `clear_struct_long_word`: Clears a long-word at a negative offset of 20 from the pointer in a5 and branches to another routine.
- `populate_and_pack_structure`: Iteratively allocates memory and copies values into a structure, then packs multiple bit-shifted values from a global table into a single long-word and stores it at an offset in a structure, before clearing a field and returning a boolean status.
- `copy_and_pack_structure_members`: Iteratively calls a routine to allocate or check memory, then packs several values from a source structure into a single long-word and stores it in a destination structure, followed by copying several words to a destination structure and calling a routine.
- `validate_and_update_structure_state`: Pushes a pointer and a value onto the stack, calls another routine, and if the return value does not equal thirty-six, updates a field at offset negative twenty-eight of a structure in a5 to a constant value of three before branching.
- `compare_constant_and_set_value`: Compares a long-word at offset -12872 of a4 to a constant, and if not equal, stores 4 in a structure at offset -28 of a5, otherwise it branches to another routine.
- `compare_and_set_pointer`: Loads a byte from a4, compares it to a value in a4, and if they are not equal, sets a 32-bit value at offset -28 of a5 to 5; otherwise it branches away.
- `update_game_state_and_extract_flags`: Calls several routines to update pointers, updates a global variable with an extended byte, and if certain conditions are met, executes a loop that extracts bit-masked flags from a value and stores them into various global variables or a structure.
- `clear_long_and_branch`: Clears a longword in a structure or global variable at a negative offset from a5 and branches to another routine.
- `update_pointer_list_and_extract_flags`: Iterates through a list of pointers, updating a value in a structure with a result from a called routine, and then performs a secondary loop that copies data from a table and extracts or sets various bits in a global status or flag variable based on bit-shifted values from a source pointer.
- `sync_and_extract_bit_flags`: Iterates through a list of pointers, copying or updating values from a source table, and if a certain condition is met, extracts specific bits from a pointer value into a series of global or structure-based flags.
- `copy_structure_members`: Compares a value from a pointer in a4 to a value in a5, and if not equal, calls a routine; otherwise, it copies several words from a structure in a5 to a different structure in a4, calls several routines, and then branches.
- `set_branch_condition`: Sets a value in a structure to one, checks if it is zero, and if not, calls a routine using a pointer-based value from a table before returning.
- `set_d0_and_branch_2`: Sets d0 to one and branches to another routine.
- `clear_and_branch_3`: Clears a long-word at an offset in a5 and branches to another location.
- `update_struct_via_call`: Sets d1 to 13, loads a long-word from a negative offset of 8 from the pointer in a5 into d0, calls another routine, stores the returned value from d0 into a negative offset of 12 from the pointer in a5, clears a long-word at a negative offset of 4 from the pointer in a5, and branches to another routine.
- `update_piece_index_and_metadata`: Reads a byte from a table using a pointer stored in a stack-relative offset, stores that byte back to a stack-relative location, and updates a secondary table or structure using a bit-masked and bit-shifted version of that byte.
- `update_piece_metadata_and_reset_index`: Updates a table or structure with a constant value and a bitwise-modified byte, increments certain pointers, and if certain bounds are met, resets an index, copies a sequence of values from a global table, and calls another routine; otherwise, it continues a loop that updates a piece index and metadata or calls another routine to update the index.
- `initialize_stack_frame_and_branch`: Creates a new stack frame, clears a longword at a negative offset from the stack pointer, and branches to another routine.
- `update_pointer_and_clear_counter`: Loads a value from a pointer at offset -8 of a structure in a5, calls another routine, stores the returned value at offset -12 of a5, clears a longword at offset -4 of a5, and branches to another location.
- `calculate_bit_mask_from_table`: Loads a value from a structure in a5, shifts it left, checks a word in a table, and if non-zero, calculates a bitmask by oring a value from a table with a bit-shifted value from another table location, then branches.
- `update_table_and_sync_pointers`: Updates a byte in a table using a pointer from a structure, increments a counter, and checks a limit; if the limit is exceeded, it copies several words and bytes from a global structure to a local structure, copies a word-table-based value, and resets a counter before returning.
- `calculate_offset_and_iterate`: Calculates an offset-based value, calls a routine with several arguments, and then enters a loop that calls another routine with a calculated address and a constant value.
- `check_pointer_and_set_d0_2`: Checks if a pointer at offset -96 of a5 is null, otherwise it sets d0 to zero, and if the pointer is not null, it sets d0 to 128 and branches to another location.
- `set_register_constant`: Loads a constant into d0 and branches to another location.
- `check_pointer_and_set_d0`: Tests if a pointer at offset -96 of a5 is null, and if not, sets d0 to a constant and branches.
- `iterate_and_call_with_bounds`: Sets a value in a structure, calculates an offset, and calls a routine; it then increments a counter in a structure and checks if it exceeds a limit, potentially calling the routine again with different parameters before returning.
- `calculate_and_store_result`: Multiplies two values from a structure, calls a function with the product, stores the result in a calculated offset within that structure, and calls itself if the result is non-zero.
- `parse_move_string`: Initializes a local buffer or structure, resets certain global or structure-based counters, and enters a loop that parses a string or command sequence, updating a pointer or value based on certain characters like 'c', 'f', 'g', or 's', and calls other routines to handle or validate these command-driven updates or state changes.
- `format_string_with_padding`: Uses a routine to write a string or integer with a specific width or padding, then checks for a period terminator in a string and updates a pointer or a flag.
- `calculate_jump_offset`: Subtracts 99 from d0, compares it to 24, and if d0 is less than 24, shifts d0 left by one and uses a value from a table to jump to a new address.
- `branch_to_target_address_2`: Performs a short branch to a fixed memory address.
- `parse_string_and_update_state`: Calculates the length of a string, checks for a semicolon terminator, and if found, resets a flag and decrements a pointer.
- `process_conditional_flag_and_branch`: Checks if a value at an offset in a4 equals ten and if a value at an offset in a5 is zero, then if certain conditions are met, pushes a pointer from a5 onto the stack and calls another routine.
- `set_flag_and_branch_2`: Sets bit 0 of a global variable, checks a value in a structure, and if certain conditions are met, calls a routine with several arguments and clears a bit in a global variable before branching.
- `set_flags_and_branch`: Sets a bit in a global variable and checks if certain values in a structure are non-zero, branching or otherwise continuing.
- `execute_sequence_of_calls`: Pushes several addresses onto the stack and calls a routine with certain arguments, modifies a global word, and calls several other routines while checking certain pointers in a5 or a global variable, otherwise it returns.
- `clear_d0_and_branch_3`: Clears d0 and branches to another location.
- `initialize_move_history_and_parse_input`: Initializes a local array with a starting value, clears a local variable, and sets a limit or sentinel value in a local stack frame before calling a routine to parse a move string.
- `update_move_history_table`: Updates a pointer-based table or structure by replacing an entry with the result of a string parsing routine, then checks a global variable to decide whether to branch or call further subroutines.
- `validate_and_traverse_move_list`: Checks if a move's coordinates fall within a valid range, then traverses a linked list of move structures, updating a counter and calling other routines to update a move string table and potentially recurse or reset a global state variable.
- `update_move_string_table_2`: Updates a table entry with a value from a structure, then checks if the result is zero to either reset a global variable and call another routine or continue a loop that updates a table entry with a constant value and checks a pointer or a global variable.
- `update_move_string_table`: Calculates an offset into a table using a value from a structure, calls another routine to write a result into the next entry of that table, and then updates a pointer or index in the structure.
- `copy_pointer_and_branch`: Loads a pointer from offset -4 of a5 into a0, copies a longword from the address pointed to by a0 into offset -4 of a5, and branches.
- `branch_to_target_address_3`: Performs a short branch to a fixed instruction or a location within another routine.
- `reset_and_dispatch_event`: Clears a long-word in a structure, loads a new value from a table into a register, and branches to a dispatcher routine that handles different event types or state-dependent logic.
- `set_value_and_call_routine`: Sets a value at an offset in a4, checks a flag in a5, and if not equal to all ones, clears the stack, calls another routine, adjusts the stack, and branches.
- `set_value_and_call_with_stack_push`: Sets a value at an offset in a4, checks a pointer in a5, checks a value in a4, and if not zero, pushes a constant onto the stack, calls another routine, adjusts the stack, and branches.
- `set_pointer_and_call`: Sets a long-word value in a global variable, pushes a constant onto the stack, calls another routine, adjusts the stack, and branches to another routine.
- `set_pointer_and_call_alt`: Sets a different long-word value in a global variable, pushes a constant onto the stack, calls another routine, adjusts the stack, and branches to another routine.
- `decrement_and_branch_on_value`: Subtracts constants from d0 and branches if certain values are reached, eventually loading a long-word from a table into d0 and branching.
- `branch_on_table_index`: Subtracts a constant from d0, and if the result is less than 4, shifts d0 left by 1 and uses it as an offset to load a word from a table into d0 before jumping to a target address.
- `jump_to_target_routine`: Performs a short branch to a different location within another routine.
- `decrement_and_branch_on_value_2`: Subtracts 8 from d0 and branches if zero, otherwise subtracts 56 from d0 and branches if zero, otherwise subtracts 960 from d0 and branches if zero, otherwise it branches to an earlier instruction in the routine.
- `branch_to_target_routine`: Performs a short branch to a location within another routine.
- `reset_and_invoke_subroutine`: Sets a global variable to a specific negative value, checks if another global variable is non-zero to call a routine with that variable as an argument, clears that variable, and then calls a sequence of other routines while clearing a local stack-based structure.
- `initialize_structure_and_pointers`: Updates a pointer-based structure with a new address, sets a flag, and calls several subroutines with specific or calculated arguments.
- `update_game_state_and_parse_move`: Checks if a provided value matches a global state variable; if not, it clears a certain flag, resets the state variable to a sentinel value, and calls a handler routine.
- `modify_and_branch_d0`: Sets d0 to a large negative value, compares d0 to twelve, and if d0 is less than twelve, shifts d0 left by one and loads a word from a table using d0 as an offset before jumping to a location determined by a word from that table.
- `update_d0_and_branch`: Stores the value of d0 into a location relative to a4, then branches to another routine if d0 is not zero, otherwise it loads a value from a5 and branches to another routine.
- `call_subroutine_with_argument`: Pushes a constant onto the stack and calls a routine with that argument before branching.
- `branch_to_address`: Performs a branch to a fixed address.
- `clear_and_call_routine`: Clears several stack locations, pushes a pointer to a global variable, and calls a routine.
- `modify_d0_and_branch`: Subtracts a constant from d0, and if d0 is zero, branches to another routine; otherwise, subtracts a different constant from d0 and if d0 is zero, branches to another routine; otherwise, branches to another routine.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `parse_move_string_and_update_state`: Iterates through a string or array, checking for specific character sequences like a dot followed by 'n' or 'd', and updates a table or structure field with values or pointers retrieved from other routine calls.
- `process_string_and_store`: Compares a byte at an offset of a5 to a constant, and if not equal, branches to another routine; otherwise, pushes a pointer, calls another routine, adjusts the stack, decrements d0, pushes d0, pushes a pointer, calls another routine, adjusts the stack, shifts d0 left, loads a pointer into a0, and stores a value from the stack into a table using d0 as an offset before branching to another routine.
- `push_and_call_routine`: Pushes a value from a global variable onto the stack, calls a routine, adjusts the stack, and then sets a register to a constant before branching.
- `clear_structure_fields_2`: Clears a 4-byte field at a negative offset from a5, then checks if a 4-byte field at offset 8 of a5 is equal to 1, or if a 4-byte field at offset 12 of a5 or a 4-byte field at offset 12 of a5 is zero, otherwise it clears a 4-byte field at a negative offset from a5 and branches back to the start of the routine.
- `iterate_and_call_with_data`: Increments a value at an offset of a5, and if it is less than a constant, checks a table, and if a certain value is not found, continues a loop; otherwise, clears the stack, loads a pointer, and calls another routine; if certain conditions or certain characters are met, it calls another routine, increments a value, and stores a value from a routine call into a table.
- `set_register_and_branch`: Sets d0 to one and branches to a fixed address.
- `push_address_to_stack`: Pushes a constant address onto the stack.
- `update_table_entries_2`: Calculates a value based on a global variable, calls a library function, and stores the result in a global variable; it repeats this for a second value and checks a global variable against a constant before potentially calling another library function and storing a result.
- `update_table_entries`: Calls a routine and stores the result in a global variable before performing a calculation and storing the result in another global variable.
- `setup_pointer_address`: Sets up a new stack frame and loads an address relative to a pointer in a5 into a0, then branches to another routine.
- `modify_pointer_list`: Loops through a list of pointers, clearing a bit in the value at each pointer, and then updates a global pointer or checks a loop termination condition, finally calling another routine.
- `restore_saved_values`: Sets up a stack frame, saves several words from a4-relative addresses into a5-relative stack locations, calls a routine, sets up a new stack frame, pushes a constant and a pointer, calls a routine, and if a certain condition or return value or error check fails, restores a sequence of previously saved words to a global memory location.
- `iterate_and_call_handler`: This routine reads a byte from a table using an offset, and if the byte is non-zero, it checks certain bits in a global variable before calling another routine with a calculated offset and a value, or otherwise calls another routine with an incremented offset.
- `update_stack_and_branch`: Clears the stack, pushes a value from a local variable onto the stack, calls another routine, adjusts the stack, increments a local variable, and branches to another routine.
- `unl_a5_and_return`: Unlinks a5 and returns immediately.
- `modify_structure_and_branch`: Pushes two pointers from a structure in a5 onto the stack, calls a routine that initializes structure members, stores the returned dword into a certain offset in the structure in a5, and then checks a bit in a global variable before either moving a constant into d0 or branching to another routine.
- `initialize_structure_members_2`: Sets a value of negative fifteen in d0, stores that value at an offset in a structure in a5, and clears two dwords at certain offsets in that same structure before branching to another routine.
- `update_table_with_offset_and_bounds_check`: Iterates through a range, calculating a new value based on a table lookup and a local offset, then writes this value to a structure member if it falls within a certain range.
- `update_table_with_offset_4`: Compares a value in a structure against a constant, and if it is not less, writes a constant to a structure member, increments a pointer, and writes a value to a table at an offset calculated from a register.
- `modify_local_pointer`: Clears a local variable, sets a local variable to a constant, and if a certain condition is met, loads a value from a global variable, adds it to a local variable, clears another local variable, and branches to another routine.
- `modify_local_pointer_2`: Performs a loop that checks if certain values are within a range, and if so, updates a local counter and writes a constant value to a table location.
- `test_structure_bit_flag`: Calculates a pointer to a structure, loads a longword from a global variable, and checks if certain bits are set; if the bits are not set, it clears a register and otherwise returns or branches to another routine.
- `update_structure_and_search_range`: Initializes a structure with several constants and clears a range of memory, then enters a loop that iterates through a table of pointers to check if certain memory-mapped or table-based ranges (calculated via offsets) overlap with a target range, updating a stored range or pointer if a valid condition is met and calling other routines to perform further calculations or checks.
- `check_pointer_and_clear_a5`: Checks if a long-word pointer at a negative offset from a5 is equal to all ones; if not, it clears a register and unlocks a5, otherwise it returns immediately.
- `calculate_and_call_subroutine`: Checks a bit in a global variable, and if set, calculates a pointer-based index using a table, then calls another routine with a constant and a value retrieved from a table-based lookup.
- `check_and_call_subroutine`: Checks if a longword at offset 336 in a4 points to zero; if not, it pushes a constant onto the stack, calls another routine, and then jumps to a subsequent instruction.
- `check_coordinate_limit_and_set_flag`: Compares a value in a structure at offset minus one hundred eighty-seven with two hundred or fewer, and if so, sets a register to one and jumps to another routine; otherwise it jumps to a different routine.
- `update_coordinate_bounds_and_validate_2`: Reads a byte from a table using an index stored in a structure, updates a corresponding value in that structure and a global variable, then calculates a new boundary or limit using a second table lookup and calls a validation routine or another routine with a new coordinate or flag.
- `update_coordinate_bounds_and_validate`: Calculates new coordinate boundaries by reading from a table and updating a structure, then calls a routine to validate or clear move flags and potentially updates a global counter or structure member.
- `initialize_and_update_coordinate_bounds`: Clears a byte in a table, calls a routine twice with different parameters, and then enters a loop that updates coordinate bounds in a structure while checking certain flags and calling other routines.
- `update_position_bounds_and_call_routine`: Checks if a position or move-related value has changed, and if so, calculates a new position by adding an offset to a previous value, clamps the resulting coordinates within a certain range, and calls another routine with the new coordinates.
- `clear_local_on_match`: Compares a value at an offset in a global variable against a constant and, if they match, clears a local variable before branching to another routine.
- `decrement_and_branch_4`: Decrements a register and branches if it equals zero, otherwise it branches to a third location.
- `branch_to_target_4`: Performs a relative branch to a specific address.
- `validate_and_update_coordinate_bounds`: Calls a routine to calculate a new index, then checks a bit in a structure and a null pointer in a global variable to update a value in a structure.
- `calculate_coordinate_range_shift_2`: Checks if a certain bit is set in a value retrieved from a structure in a4, and if so, calculates a bit-shifted difference between a value from a table and a value from the structure, storing the result in d1 via a temporary storage in a5.
- `calculate_coordinate_range_shift`: Calculates the bit-shifted difference between two values retrieved from a table using an offset from a structure in a4, and stores the result in a local variable at an offset in a5, otherwise it returns.
- `calculate_and_update_coordinate_bounds`: Calculates the absolute difference between two coordinate-related values retrieved from a table using an index from a structure, updates local stack-based variables with these absolute differences, and then conditionally calls another routine with a new stack frame to update a table entry or clear a value if certain bounds or status flags are met.
- `update_table_entries_3`: Calls two routines, checks if a local variable is less than a constant, and if so, clears a word in a table and writes a word to another table using an offset derived from the local variable, then calls two more routines and branches to another routine.
- `update_piece_position_and_state_2`: Updates a piece's move index and secondary counter in a structure using a table-based lookup, clears a bit in a global status byte, and then calls several routines to update piece data and check move validity.
- `update_move_state_and_validate_bounds_2`: Updates a move index and a secondary counter in a structure, then calculates the difference between two table-based values (retrieved via an index-based lookup) and resets certain counters in the structure if a specific condition is met, otherwise it continues with a loop that iterates through a range, calls another routine, and updates several structure members based on table lookups.
- `clear_local_variable`: Clears a local variable and branches to another routine.
- `update_move_index_and_validate_state`: Updates a move index in a structure by adding the return value of another routine to a value at an offset, then checks a table to update a secondary counter and potentially sets a state or move index.
- `check_counter_value`: Compares a value at an offset in a5 against a constant, and if it is not equal, sets d0 to a constant and branches.
- `update_move_counter_and_validate_state`: Increments a counter at an offset in a structure, then checks a value in a table to determine if a certain state or move index should be updated or if a specific status code should be written to a structure offset.
- `update_move_state_and_validate_bounds`: Updates a move index and a secondary counter in a structure, then calculates the difference between two table-based values (retrieved via an index) and updates a third offset in the structure.
- `update_table_value_on_condition`: Checks if a longword at a negative offset of a5 is equal to one; if so, it overwrites a longword at a different negative offset of a5 with a constant value and otherwise returns immediately.
- `modify_value_on_bit_test`: Loads a longword from a negative offset of a5 into d0, compares it to a longword at an offset of a4, and if it is not greater than or equal to zero, tests bit 2 of a byte at a negative offset of a4; if that bit is clear, stores the constant 46 into a longword at a negative offset of a5 and branches.
- `test_bit_and_set_constant_2`: Tests bit 1 of a byte at a negative offset from a4, and if clear, stores 55 into a global variable at a negative offset from a5 and branches to another routine; otherwise it branches to another routine.
- `test_bit_and_set_constant`: Tests bit 4 of a byte at a negative offset from a4, and if clear, stores 56 into a global variable at a negative offset from a5 and branches to another routine; otherwise it branches to another routine.
- `set_global_constant`: Stores the constant 39 into a global variable at a negative offset from a5, then branches to another routine.
- `update_value_on_condition`: Loads a longword from a negative offset of a5 into d0, compares it to a longword at an offset of a4, and if it is not greater than or equal to zero, stores the constant 57 into a longword at a negative offset of a5 before branching.
- `negate_longword_on_negative`: Tests a longword at a negative offset of a5, and if it is not greater than or equal to 0, loads that longword into d0, negates it, and branches.
- `check_longword_value`: Loads a longword from a negative offset of a5, compares it to 2, and if the result is not greater than or equal to 2, tests a bit in a byte at a negative offset of a5, and if clear, negates a longword loaded from a negative offset of a5 before branching.
- `compare_and_set_constant_2`: Loads a longword from a5 into d0, compares d0 against 2, and if d0 is not greater than or equal to 2, stores 35 into a global variable at a negative offset from a5 and branches to another routine; otherwise it branches to another routine.
- `check_bits_and_set_constant`: Tests bit 1 of a byte at a negative offset from a4, and if clear, tests bit 5 of a byte at a negative offset from a4, and if clear, tests if a word at a negative offset from a4 is non-negative, and if a longword at a negative offset from a5 is not equal to 1, stores 37 into a global variable at a negative offset from a5 and branches to another routine; otherwise it branches to another routine.
- `compare_and_set_constant`: Loads a longword from a5, adds a longword from a5, compares the result against a value in a4, and if not less than or equal, stores 46 into a global variable at a negative offset from a5 before branching to another routine.
- `test_and_set_pointer`: Tests if bit 9 of a longword at an offset from a pointer in a4 is set, and if bit 1 of a word at another offset from a4 is clear, it sets a longword in a5 to a constant value before branching to another routine.
- `test_bit_and_set_pointer`: Tests bit 1 of a byte at a certain offset from a pointer in a4, and if it is zero, sets a longword in a5 to a constant value before branching to another routine; otherwise, it branches to another routine.
- `calculate_index_difference_loop_2`: Loads a value from a global address, masks it, and if the result is zero, clears a local variable before branching.
- `calculate_index_difference_loop`: Calculates the difference between two values from a table using an index-based loop, updating a local variable with the result, and otherwise returns.
- `increment_and_check_table_index`: Increments a longword in a table, then checks if the new value equals a constant and verifies if a certain longword in a table is negative; otherwise, it updates a longword in a table with a constant and branches to another routine.
- `validate_piece_move_range`: Tests if bit 2 of a byte at a negative offset from a4 is set; if so, it checks if a longword at a certain offset from a5 equals 8, and if not, it checks if a longword at a certain offset from a5, after being right-shifted by 6, equals a longword at another offset from a5, then writes the constant 33 to a certain offset in a5-based structure before branching to another instruction.
- `validate_and_update_game_state`: Checks a value in a global variable against certain constants and, if certain conditions regarding a table lookup and a status flag are met, sets a longword in a structure to a constant value.
- `update_structure_pointer`: Clears a longword at a negative offset of a structure, checks if a longword at another negative offset is non-zero, and if so, adds 13 to a longword at a negative offset of a structure before jumping to another routine.
- `iterate_and_update_status_flags`: Iterates through a table, checking if a certain bit is set in a status byte and setting a flag in a structure if so, while incrementing a pointer or counter.
- `test_bit_and_branch_4`: Tests bit 4 of a byte at a negative offset from a4 and, if clear, returns; otherwise, it masks a value from a pointer in a4, checks if it equals one, and if so, loads a constant into d0 and branches to a nearby instruction.
- `validate_and_update_move_state`: Calculates a new index, checks if it falls within a valid range, and verifies if the move or piece state matches a value in a lookup table; if certain conditions are met, it updates a global status value and calls other routines to update piece coordinates and trigger conditional logic or further routine calls.
- `update_piece_data_on_bit_test`: Tests bit 3 of a byte at a negative offset from a4 and, if the bit is clear, otherwise it returns; if the bit is set, it compares a longword at a negative offset from a5 with a longword at a negative offset from a4 and, if they are not equal, overwrites a longword at a negative offset from a5 with the constant 42 and otherwise it returns.
- `test_and_update_longword`: Tests bit 5 of a byte at a negative offset of a4 and, if a longword at a negative offset of a5 is non-zero and a longword at a certain offset of a4 equals one, writes the constant 51 to a negative offset of a5, otherwise it returns.
- `test_bit_and_update_longword`: Tests bit 5 of a byte at a negative offset of a4 and, if a longword at a negative offset of a5 is non-zero, writes a longword containing a constant to a negative offset of a5 before branching to another routine; otherwise it branches to another routine.
- `calculate_d0_offset_2`: Tests if a longword at a negative offset of a5 is zero, otherwise loads a constant into d0 and subtracts a value from a negative offset of a5 from d0 before branching to another routine.
- `sync_longword_and_check_status`: Copies a longword from a negative offset of a5 to another negative offset of a5, tests bit 4 of a byte in a global variable, and if the bit is clear, branches to another routine; otherwise, it compares a longword in a global variable to one, and if not equal, branches to another routine, writes a longword of one to a negative offset of a5, and branches to another routine.
- `check_longword_range`: Compares a longword at a negative offset of a5 against certain constants and, if the value is within a specific range, sets d0 to one and branches to another routine; otherwise, it calls a routine that clears d0 and compares a longword at a negative offset of a5 against a different constant.
- `check_longword_threshold`: Clears d0 and compares a longword at a negative offset of a5 against 33, otherwise it clears d1 and branches to another routine.
- `check_and_set_longword`: Checks if a longword at a negative offset of a5 is zero, otherwise it writes the constant 51 to a longword at a negative offset of a5 and branches to another routine.
- `set_status_value_and_branch`: Checks if a longword at a negative offset of a structure is zero, and if not, writes the constant 51 to a longword at a negative offset of a structure before jumping to another routine.
- `reset_status_bit_and_call_update`: Clears a bit in a global status byte via a pointer in a4, calls a routine to update piece position and state, and then jumps to a fixed location.
- `update_status_and_branch`: Pushes a constant onto the stack and calls another routine, adjusts the stack pointer, checks a longword at a negative offset from a pointer in a5, and if that longword is zero, writes a constant to a memory location relative to a5 before branching to another routine.
- `check_longword_range_2`: Checks if a longword at a negative offset from a pointer in a5 is between 39 and 42 inclusive, otherwise it returns immediately or sets d0 to one.
- `test_longword_threshold`: Clears d0 and checks if a longword at a negative offset of a5 is less than or equal to a certain constant, otherwise it sets d1 to one and branches to another routine.
- `update_longword_value`: Clears d1 and performs a logical or on d0, otherwise it writes the constant 48 to a longword at a negative offset of a5 and branches to another routine.
- `compare_and_update_longword`: Compares a longword at a negative offset of a5 against 1, otherwise it writes the constant 51 to a longword at a negative offset of a5 and branches to another routine.
- `set_status_bit_on_range`: Compares a longword at a negative offset of a5 against 1, and if not equal, checks if a longword at a negative offset of a5 is within the range of 26 to 46, and if so, sets bit 5 of a global variable at a negative offset of a4 before jumping to another routine.
- `set_status_flag_and_branch`: Writes the constant value fifty-one to a location in a global variable and branches to another routine.
- `update_piece_position_and_state`: Toggles a bit in a global status byte, updates a table with new coordinate or piece data if certain conditions or piece-move flags are met, and calls a routine to calculate or validate a new position, potentially updating a global counter or clearing a cell in a board-related table.
- `push_and_call_state`: Pushes three pointers from a structure onto the stack, calls another routine, and then updates the stack pointer; it also checks a bit in a global variable to conditionally call a routine with a constant or a zeroed stack, and finally sets a register to one before branching.
- `update_recursive_members`: Initializes two local variables to zero, calculates a difference between a value in a structure and a pointer, and calls a routine; it then calls another routine and potentially calls a third routine if a certain condition is met, or branches to a different routine.
- `update_recursive_structure_members`: Updates several fields in a structure, including resetting certain values to one or zero based on comparisons, and executes a loop that recursively calls another routine while decrementing a counter-based value in the structure.
- `process_recursive_data_step`: Clears a 32-bit value in a structure, then calls another routine twice with a constant value in d1, and if a comparison fails, it calls that routine again while updating a pointer in the structure and then returns.
- `process_recursive_data`: Clears a 32-bit value in a structure, then calls another routine twice while updating a pointer in a structure, and decrements a counter in a structure, otherwise it returns.
- `traverse_piece_path_recursively`: Checks a byte in a table using a pointer, and if it is zero, calls a routine with a new pointer; otherwise, it recursively traverses a path or checks path validity by updating a global table and calling another routine.
- `update_piece_path_and_validity_2`: Calculates and stores new pathing or move-validity values in a structure, updates a global table, and then calls another routine to check or apply these pathing/validity updates.
- `update_piece_path_and_validity`: Updates a structure member with a value, clears a byte in a global table, and then checks move validity or pathing by comparing values in a global table.
- `update_pointer_offset`: Loads a 32-bit value from a structure offset into d0 and adds d0 to a value in a structure, then branches to another routine.
- `jump_to_target_address`: Performs a short branch to a fixed location in the code, which redirects execution into the middle of another routine.
- `modify_longword_offset`: Calculates the difference between a longword at a positive offset of a5 and a longword at a different positive offset of a5, stores the result in a longword at a negative offset of a5, and if that value is not 43, subtracts a constant from that longword and branches to another routine.
- `update_structure_and_call_2`: Compares a longword at a negative offset of a5 against a negative constant, otherwise it adds a constant to a longword at a negative offset of a5, loads a longword from a negative offset of a5 into d0, calls another routine, and if d0 is not zero, sets d0 to 12 and branches to another routine.
- `traverse_structure_and_recurse`: Iterates through a linked list or structure-based chain, calculating a new index or pointer value and recursively calling itself with updated parameters derived from a table, then updates a pointer in the structure to the next element before returning.
- `branch_to_target_address_8`: Performs a short branch to a fixed instruction or location in the code.
- `push_structure_members`: Pushes five longwords from a structure in a5 onto the stack, calls another routine, and if a certain value in the structure is one, updates a structure member with a value from another member before calling that routine again with a different stack state.
- `update_move_validity`: Reads a byte from a table using an offset, stores it in a local variable, and clears a byte in that same table; it then calls a routine with several pointers pushed onto the stack and branches to another routine.
- `update_state_bitmask`: Compares a bitmask from a global array against a value retrieved from a table.
- `update_move_validity_and_bounds_2`: Checks if a piece's move flags or certain bits in a structure are set, and if so, updates a global counter, calls a routine to calculate move offsets, and iterates through a table to validate move bounds or piece movement permissions.
- `check_piece_move_flags`: Tests bit 6 and bit 7 of a byte at offset 23 of a structure in a5, and if either bit is clear, branches to different locations; otherwise, it loads a constant into d0 and branches to another routine.
- `update_move_validity_and_bounds`: Initializes a structure and calls a routine to modify structure members, then iterates through a table of move offsets to check if the resulting move coordinates fall within a certain threshold.
- `normalize_coordinate_offset_value`: Compares a value from a structure in a5 to a global variable in a4, and if the value is less than the threshold, calculates the absolute value of a second field from the structure in a5 and stores it in d0 before returning or branching otherwise.
- `normalize_coordinate_offset`: Checks if a value in a structure is less than a certain threshold in a global structure, and if so, calculates its absolute value and stores it in a different field within the same structure, otherwise it returns immediately.
- `update_move_coordinates`: Copies a value from a structure offset to another location in the same structure, clears a field, and then checks if a masked bitmask or a sign-flipped coordinate value exists to trigger a call to a routine that calculates a move or a jump or a branch.
- `calculate_difference_and_branch`: Calculates the difference between a value from a structure in a5 and a value from a structure in a4, pushes the result onto the stack, and then checks if a value at an offset in a5 is negative, otherwise it negates a value from a structure in a5 and branches to a previous instruction.
- `calculate_dispatch_argument`: Calculates a new pointer or value by subtracting a value from a structure member and pushes several other structure members onto the stack, then calls a routine with a pointer or value determined by checking if a certain bit is set in a structure member.
- `dispatch_table_indexed_call`: Calculates a table offset by masking a value from a structure, shifts it left by two, and adds a base pointer from another structure, then calls a routine with a pointer-based argument.
- `calculate_index_difference`: Subtracts a longword at an offset from a pointer in a5 from a longword at an offset from a pointer in a4, pushes the result onto the stack, and if a longword at an offset from a5 is negative, negates that value and branches to another routine.
- `calculate_table_offset_and_call`: Calculates a new pointer by subtracting a value from a structure in a5 from a value in a4, pushes several arguments onto the stack, and calls a routine using a pointer or an incremented offset or a conditional branch depending on a bit-shifted value from a structure in a4.
- `get_table_offset_and_call`: Calculates a table offset by masking a value from a structure, shifts it left by two, and adds a base pointer from another structure, then calls a routine using this calculated address as an argument.
- `access_table_with_offset`: Loads a value from a structure, shifts it left, and uses it as an offset to load a value from a table into a register, then decrements a counter in a structure and branches to another routine.
- `evaluate_move_and_iterate_table`: Checks a flag in a structure, then calls a routine with a pointer-based argument or a table-based argument, and subsequently performs a loop that iterates through a table to call another routine with modified parameters or otherwise returns.
- `update_position_and_iterate_table`: Calls a routine with several arguments, then if a certain flag in a structure is set, it clears a longword and loops through a table to read bytes, checking if they match certain values, and calls another routine for each valid entry.
- `process_position_data`: If a value at an offset of 20 from a5 is 2, it clears a longword at offset -28 of a5, then loops through a table, reading a byte, checking if a bit is set, and calling another routine with a calculated pointer and a value from a table, or otherwise calls another routine with a pointer and a constant or a value from a table.
- `prepare_argument_list`: Sets bit 6 of d0, pushes d0 and two values from a table onto the stack, and calls another routine, then cleans up the stack and branches to another routine.
- `store_position_data`: Clears the stack, pushes a pointer and two values from a structure, calls another routine using a value from a table, and then stores two values from a structure into a global variable.
- `swap_values_using_xor`: Swaps the contents of two memory locations using the XOR operator.
- `call_function_with_arguments`: Calls a function pointer using several arguments and values from a global table.
- `modify_structure_value`: Writes a constant value to a memory location, then loads a value from a pointer in a4, shifts it right, and subtracts it from a value at offset 8 of a5, otherwise it branches or sets d0 to 87 and jumps.
- `set_and_compare_constant`: Sets d0 to 99, copies it to d6, and compares a value at offset 8 of a5 to 159, otherwise sets d0 to 156 and branches.
- `set_constant_d0`: Sets d0 to a constant value and branches to another routine.
- `compare_and_branch`: Loads a 32-bit value from a structure, compares it to a register, and if it is not less than or equal, sets d0 to zero and branches to another routine.
- `calculate_distance_and_update_table`: Iterates through tables to calculate or compare distances and updates a global variable with a conditional result.
- `initialize_address_registers`: Loads an address from a pointer in a4 into a0 and a2, then clears d4 and d6 before branching to another routine.
- `calculate_signed_difference`: Loads a 16-bit value into d0, loads a 32-bit value from a structure into d7, calculates the difference, and if the result is negative, negates it and branches to another routine.
- `update_pointer_and_compare_logic`: Updates a 32-bit value in a structure, and if a certain condition is met, updates a value in a structure, increments a register, and checks it against a value in a table before branching or calculating a new pointer address.
- `calculate_difference_with_pointer_advance`: Copies a pointer to a0 and increments it, loads a 16-bit value, calculates the difference between a value from a structure and d0, and if negative, negates the result and branches to another routine.
- `update_pointer_and_compare`: Moves d7 to d0, stores d0 at offset -16 of a5, and if d0 is not greater than or equal to a value at offset -12 of a5, it moves d6 to d5, updates a value at offset -12 of a5, and checks if d6 is less than a value at offset -14214 of a4, otherwise it adds d5 to d0 and branches.
- `update_stack_and_call_routine`: Updates a longword in a structure using a value loaded from a pointer in a4, tests bit 2 of a byte at a negative offset from a4, and if certain conditions are met, pushes that longword onto the stack and calls another routine.
- `check_bit_and_clear_stack`: Tests bit 3 of a byte at a negative offset from a4 and, if not set, clears the stack pointer and returns; otherwise, it clears the stack pointer and returns.
- `clear_stack_long`: Clears a longword at a negative offset from a5 and then branches to another routine.
- `clear_and_branch`: Clears a 32-bit value in a structure and branches to another routine.
- `process_move_sequence`: Executes a loop that calls a routine, updates a counter in a structure, and checks a value against a global constant.
- `process_move_sequence_bounds`: Pushes a constant and several values onto thestack, calls a routine, and then checks if a value in a structure is within certain bounds before calling two more routines and branching.
- `initialize_stack_frame_and_branch_2`: Creates a new stack frame, clears a longword at a negative offset relative to the stack pointer, and branches to another routine.
- `clear_offset_long`: Clears a longword at a negative offset of 4 bytes from the pointer in a5 and branches to another routine.
- `calculate_bounds_and_branch`: Calls a routine, updates a local variable, and calculates new values from a table; it then checks a bit in a table, and if certain conditions involving a register and a global constant are met, it sets a register to a constant and branches.
- `push_parameters_and_call`: Sets bit 2 of a register, pushes that register and three other values from a structure onto the stack, calls another routine, and then branches to another instruction.
- `iterate_and_call_routine`: Calculates an address using a value from a structure, updates certain flags, pushes several values onto the stack, and calls another routine; it then checks a table, and if a condition is met, clears the stack, pushes more values, and calls another routine before potentially incrementing structure members and returning or branching.
- `initialize_board_layout_and_pieces_2_2`: Initializes a board structure by loading piece or empty-space identifiers from a table, then populates a secondary table with piece or empty-space identifiers using a loop that runs up to 30 times.
- `initialize_board_layout_and_pieces_2`: Initializes a structure with piece or empty-space identifiers by calling a routine to populate a table, then sets or clears certain structure offsets based on flags in a global variable.
- `set_structure_constants`: Tests bit 6 of a byte at a negative offset of a4 and, if set, writes four different longword constants to negative offsets of a5, otherwise it returns.
- `initialize_structure_constants`: Tests bit 0 of a byte at a negative offset of a4 and, if not set, writes four different longword constants to negative offsets of a5 before branching to another routine; otherwise it branches to another or returns.
- `initialize_structure_fields_2`: Sets four different longword values at negative offsets of a5, clears a longword at a negative offset of a5, and then branches to another routine.
- `update_structure_and_call`: Loads a longword from a negative offset of 144 bytes from the pointer in a5 into d0, calls another routine, stores the returned value back to a negative offset of 172 bytes from the pointer in a5, clears a longword at a negative offset of 140 bytes from the pointer in a5, and branches to another routine.
- `update_lookup_table_loop_2_2`: Iterates through a table, calling another routine to retrieve a value, adding it to a base value, and storing the result in a lookup table at an offset or index derived from a pointer.
- `initialize_board_layout_and_pieces`: Reads a value from a global pointer to determine a layout offset, then enters a loop that populates a table with piece or empty-space identifiers while updating a secondary table with corresponding move or piece data.
- `initialize_table_and_populate_data`: Initializes two longword values in a structure relative to a5, then enters a loop that calculates a new table index, populates a table with values derived from a global variable, and performs further conditional data writes and calls to other routines.
- `initialize_table_bounds_and_populate`: Sets a constant value in a local variable, initializes a secondary counter, and enters a loop that populates a table with values derived from a global source, while also performing a calculation to set a table index based on a right-shifted, incremented global longword.
- `initialize_scaling_table`: Initializes a scaling table by setting a starting index and a limit, then enters a loop that calculates and stores scaled or offset values into a table or structure using a pointer in a5.
- `reset_table_indices`: Clears two longwords at negative offsets from a pointer in a5 and then branches to a nearby instruction.
- `decrement_and_branch_on_zero_4`: Decrements d0 by 6, 2, or 1 and, if the result is zero, branches to another routine; otherwise it branches to another routine.
- `increment_and_scale_table_index`: Reads a longword from a global variable, increments it, performs a right shift, and stores the result into a longword at a negative offset from a5, then clears a longword at a negative offset from a5 before branching to another routine.
- `update_table_with_offset_2`: Sets d1 to 13, loads a value from offset -144, calls another routine, and then updates a word at an offset in a table using a value from a register and an offset, then clears a longword at offset -140 of a5.
- `update_lookup_table_loop`: Iterates through a range of indices, reading a byte from a table and storing it as a word into a destination table, while incrementing a counter in a structure.
- `update_lookup_table_loop_2`: Iterates through a table, reading a byte from a source address, and updates a lookup table with a value derived from that byte and a pointer offset.
- `populate_and_reset_structure_state`: Iterates through a table to copy bytes from a source table to a destination table, then resets a counter in a structure, updates a pointer in a structure, and executes a series of calls to update or clear specific fields or table-related data.
- `reset_and_update_structure_state`: Resets a field in a structure, sets a bit in a status byte, and then executes a series of calls to update or clear specific fields or table-based pointers based on indices or offsets derived from the structure, before entering a loop that updates structure fields or clears table-based pointers.
- `set_state_and_call`: Sets d1 to 2, loads a value from offset -144 of a5, calls another routine, and if the result is not zero, sets d0 to 2 and branches.
- `update_structure_state`: Writes a constant to a location in a structure, loads a value from a structure offset into a register, calls another routine, and then updates a structure field and clears a longword in a structure before branching to another routine.
- `update_structure_state_logic`: Iterates through a range of indices, calling a routine to check a condition and then updating or clearing structure fields or table-based pointers.
- `clear_structure_field`: Clears a longword in a structure and branches to another routine.
- `update_position_and_check_bounds`: Calculates a new index by adding a value from a function call to a counter in a structure, then checks this index against a table or range.
- `iterate_and_update_structure_fields`: Iterates through a range of indices, calling a routine to check a condition and then updating or clearing structure fields or table-based pointers based on the result.
- `clear_long_value`: Clears a long-word at a negative offset from a pointer in a5 and branches to another routine.
- `update_structure_and_bounds`: Iterates through a list of pointers, updating a structure field with a calculated offset and calling two other routines, then clears a long-word at a negative offset from a pointer in a5 and branches to another routine.
- `reset_structure_counter_and_branch`: Sets a value of 4 in a local structure field at offset 188, clears a 32-bit value at offset 144 of the structure in a5, and branches to another routine.
- `shift_and_branch`: Tests if a long-word at a negative offset from a4 is zero, otherwise loads a long-word from a negative offset of a5 into d0, shifts it right by one, and branches to another routine.
- `iterate_and_update_structure_2`: Iterates through a list of pointers, updating a value in a structure with a calculated offset and calling two other routines, then clears a field in a structure and calls a routine with several arguments before calling a sequence of other routines and finally updating a global variable with a return value from a routine.
- `update_piece_index_and_structure`: Validates and updates a piece index or pointer, then updates a structure or table with a new value derived from a pointer-based lookup, and finally executes a loop that calculates and stores new offsets or values into a table or structure based on a loop counter or index.
- `validate_and_update_index`: Checks if a certain value plus one is valid by calling a routine, and if the result is non-zero, calls another routine to update a pointer in a structure and then enters a loop that updates a table of indices or offsets.
- `update_structure_indices`: Calls a routine using a pointer from a structure, then if the return value is non-zero, calls another routine and enters a loop that updates a table or array using a loop that increments a struct member, or otherwise returns immediately.
- `recalculate_structure_indices`: Updates or clears certain members of a structure pointed to by a5, including updating a table or array using a loop that increments a struct, or otherwise resets a counter or pointer-based index.
- `clear_long_word_and_branch`: Clears a long-word in a structure at a negative offset from a5 and branches to another routine.
- `update_game_state_and_table`: Calculates a new index or value based on a previous routine's return, updates a global counter, and then writes a value to a table or array using a calculated offset.
- `update_structure_member_indices_2`: Updates or clears certain members of a structure pointed to by a5, including updating a table or array using a loop that increments a structure member.
- `check_and_clear_structure_offset`: Tests if a longword at a certain offset from a pointer in a4 is zero, and if not, clears a longword at a certain offset from a pointer in a5 before branching to another routine.
- `clear_long_word_at_offset`: Clears a long-word at a negative offset of 140 from a5 and branches to another routine.
- `update_structure_member_indices_3`: Updates a table of indices or pointers within a structure using a loop that increments a structure member, while also performing arithmetic-based updates to other structure members and a global table or array.
- `clear_structure_member_offset`: Clears a longword at a fixed offset relative to a pointer in a5 and branches to another routine.
- `clear_long_word_at_offset_2`: Clears a long-word at a negative offset of 1402 from a5 and branches to another routine.
- `update_structure_member_indices`: Calculates and stores new index or pointer values into a structure, updating a table or array using a loop that increments a structure member and checks against a limit, then conditionally clears a structure member and modifies other structure members or tables based on a bit test.
- `set_negative_constant_to_structure`: Sets a value of negative one to a local structure member, then sets another member of that same structure to the two's complement negation of that value, and otherwise returns.
- `update_structure_values_2_2`: Iterates through a range of memory, calling another routine to retrieve a value, adding it to a value from a pointer, and writing the result into a table.
- `update_structure_values_2`: Loops through a range of memory to write a constant, then calls another routine to update a table with a value from a pointer, and finally performs a series of pointer-based writes and calls to other routines.
- `update_pointer_and_clear_memory`: Checks a bit in a structure, otherwise loads a value from a structure into d0, calls another routine, stores d0 back into a structure, clears a longword in a structure, and jumps.
- `iterate_and_update_values_2`: Increments a value from a structure, calls a routine using a pointer from a4, and pushes the result to the stack.
- `set_initial_value_and_branch`: Sets a 16-bit value at an offset in a structure, checks a value in a table, and either calls another routine or branches to a different location.
- `copy_table_elements_conditional`: Sets up a stack frame, checks a bit in a table, and if set, executes a loop that calls another routine to copy or move data between two table locations, or otherwise executes a nested loop that calls another routine to copy data between table locations or updates a counter.
- `update_table_element_size`: Loads a value from a negative offset of 14370 from a4, calls another routine, adds a value from a negative offset of 14374 from a4 to the result, stores the result at a negative offset of 4 from a5, then calculates a new value by adding 1 and right-shifting the result before storing it at a negative offset of 8 from a5, and finally branches to another routine.
- `copy_table_elements`: Iterates through a table and calls a function to move or copy data between two memory locations.
- `update_table_with_calculated_value`: Iterates through a table and writes a value derived from a function call and another routine to a memory location.
- `initialize_table_and_set_flags`: Checks a bit in a status byte and a short value in a structure, then sets a bit in a status byte and updates a pointer.
- `populate_table_index_and_set_value`: Iterates through a loop to store an incrementing counter into a table, where each entry is determined by an offset-scaled index read from a global byte.
- `initialize_table_index_and_branch`: Sets a value of 8 in a structure member, checks a bit in a global byte, and if certain conditions are met, sets another structure member to 7 before using a value from a table to determine a branch or a new pointer destination.
- `select_next_table_entry`: Compares a value in d0 against a constant, and if it is not less than that constant, shifts d0 left by one and uses the new value as an offset to load a new jump target from a table into d0 before jumping to that target; otherwise, it jumps to a different location.
- `reset_and_populate_move_history`: Resets several tables in memory, including clearing a 16-bit or 32-bit area, and then enters a nested loop that calls another routine to read a value, increments a pointer in a structure, and writes that value into a move history table while updating bitmasks and other state-related tables.
- `zero_d6_and_branch`: Sets d6 to zero and branches to another routine.
- `update_move_history_and_board`: Calls another routine to calculate a new index, then checks a pointer-based structure to validate a move, updates a move-tracking table, and modifies a global counter or board-related state if certain conditions or move-type flags are met.
- `update_move_history_and_board_2`: Updates move history and board state by writing certain values to a table, updating a bitmask, and then calls another routine to update a pointer-based structure and potentially calls a third routine to update a bitmask or clear a value.
- `sync_board_state_and_update_table_2`: Iterates through a range of indices, writing a value from a pointer-based source to a table at a calculated offset, and then updates a secondary table or pointer-based source.
- `sync_board_state_and_update_table`: Iterates through a range of indices, writing a value from a pointer-based source to a table at a calculated offset, and then updates a secondary table by adding or copying values.
- `set_d6_and_branch`: Sets register d6 to negative one and branches to another routine.
- `update_board_and_state`: Updates a table of values by adding a value from one memory location to another within a loop, then potentially copies a block of data from a source table to a destination table.
- `update_structure_value`: Tests bit 5 of a byte at a negative offset from a4, and if clear, compares a word at another negative offset from a4 to one, otherwise it writes a longword with value 3 to a certain offset in a4 and then jumps to another routine.
- `check_structure_bits`: Tests bit 5 of a byte in a structure and, if bit 5 is clear, checks if a word in a structure equals two, otherwise it writes a longword to an offset in a structure and branches.
- `set_structure_constant`: Checks a bit in a structure, otherwise checks if a value in a structure is non-zero, and if so, writes a constant to an offset in a4 before jumping.
- `check_bit_and_set_pointer`: Checks a bit in a structure, and if a certain value in a structure matches a constant, sets a longword at an offset in a4 before branching to another routine.
- `set_structure_value_conditionally`: Checks a bit in a memory location, and if it is not cleared, sets a longword at an offset in a structure before branching.
- `set_pointer_offset_value`: Sets a longword at offset 420 of a4 to 1, and if a bit in a value masked with 121 is not zero, sets a longword at offset 424 of a4 to 1, otherwise branches.
- `check_and_update_pointer_data`: Tests bit zero of a byte at a negative offset from a pointer in a4 and branches if it is clear; if it is set, it compares a word at a different negative offset from a4 to one and, if equal, writes a longword of one to an offset of a4, then branches to another routine.
- `reset_pointer_offset_value`: Resets a longword at a fixed offset from a pointer-based address to zero, then restores registers and returns.
- `calculate_and_store_offsets`: Calculates a value using a pointer and a structure member, calls a routine, and then performs a loop that writes certain values to a table or structure based on checks of a global variable and a structure member, otherwise it returns.
- `clear_structure_longword`: Clears a longword at an offset in a structure and then branches to another routine.
- `calculate_offset_value`: Tests if a longword in a structure is zero, otherwise it calculates a value by subtracting a value in a structure from a constant and branches to another routine.
- `set_structure_offset_value`: Writes a constant to a longword at an offset in a5, then checks a value in a structure and either jumps or sets d0 to a constant and jumps.
- `update_structure_data`: Loads a value from a structure, increments it by 5, stores the result in a structure, clears a longword in a structure, and branches to another routine.
- `update_structure_elements_2`: Checks a bit in a table and checks if a value is zero, then if certain conditions are met, copies a byte from a table to a table or copies a word from a table to another table using a calculated offset, and finally updates a longword in a structure using a value from a table or a calculated difference.
- `update_structure_elements`: Calculates an index or offset based on a value in a structure, then writes two 16-bit values into a table or array using that calculated offset.
- `update_structure_pointers`: Updates a pointer in a structure, increments a value in a structure, and checks a value against a table entry before potentially unlocking a register and returning.
- `initialize_game_state_and_buffers_2`: Initializes a global structure by setting a boolean flag, calling several subroutines to reset or configure certain pointers, and clearing or updating various tables and buffers.
- `update_jump_target_pointer`: Clears a 32-bit value at a certain offset in a structure, checks if a certain 16-bit value in that structure falls within a specific range, and if so, calculates a new pointer or value and stores it back into the structure before returning.
- `calculate_jump_address`: Subtracts a constant from a register, compares it to a constant, and if certain conditions are met, uses a table-based lookup to jump to a new address.
- `update_piece_position_and_distance_2`: Checks if a certain value in a structure is within a specific range, then updates a global offset and a bitmask-based flag, and recursively calls itself or calls other routines to update or clear certain memory-mapped or structure-based values.
- `branch_to_target_5`: Performs a short branch to a target instruction located at a relative offset.
- `decrement_and_branch_3`: Subtracts a constant from d0 and branches if the result is zero, otherwise subtracts another constant from d0 and branches if the result is zero, otherwise it loops back to an earlier instruction.
- `branch_to_target_routine_2`: Performs a short branch to a fixed location within another routine.
- `initialize_game_state_and_buffers`: Initializes several global variables, clears a table, and calls multiple functions to configure hardware or system state.
- `initialize_system_state`: Initializes several global variables, clears a table, and calls multiple functions to configure hardware or system state.
- `update_piece_position_and_distance`: Updates a global piece or coordinate or index and calculates wrapped distances or or relative offsets.
- `move_d0_to_d5`: Copies the contents of d0 to d5.
- `update_move_validity_table`: Calculates a range of possible moves or piece positions and updates a table in a structure, checking certain bitmask conditions and bounds, and calls another routine to clear a longword or update a pointer-based value.
- `clear_long_variable`: Clears a longword at offset minus 52 of a5 and then branches to another routine.
- `fetch_move_metadata`: Retrieves two 16-bit values from tables using an index stored at an offset in a structure, stores the first value into a structure field, and then stores the second value into a register before performing a conditional check on a bit in a global variable.
- `update_move_list_and_reset_register_2_3_2`: Updates a move list by iterating through a table of move indices, checking if a move's destination or certain flags meet specific criteria, and if so, updates a list.
- `update_move_list_and_reset_register_2_2`: Iterates through a table of move indices, checking if a move's destination or certain flags meet specific criteria, and if so, updates a list or counter with a value from a structure.
- `set_register_and_branch_4`: Sets a2 to a full 32-bit mask and branches to another address.
- `update_move_list_and_reset_register_2`: Checks a bit in a structure, then iterates through a table to find a specific value, updating a list or counter if a condition is met, and calls another routine to reset a register or branch if certain criteria are met.
- `update_move_list_and_reset_register`: Checks if a value from a table, indexed by a register-based offset, matches a value in a structure, and if so, updates a list or counter and sets a register to all ones.
- `set_register_to_max_and_branch`: Sets register a2 to a value of all ones and branches to another routine.
- `update_move_list_and_reset_register_2_3`: Iterates through a table of move indices, checking if a move's destination or certain flags meet specific criteria, and if so, updates a list.
- `update_pointer_from_table_2`: Loads a longword from a negative offset of a5, masks it, and if the result is not zero, updates a longword at a negative offset of a5 using a value from a table or another register, otherwise it returns.
- `update_move_list_and_validate_position`: Iterates through a list of possible moves, checking if a target square or piece type satisfies certain conditions, and if so, updates a value in a structure pointed to by a register.
- `check_bit_and_load_data`: Tests bit 2 of a byte at a negative offset from a4, and if bit 2 is clear, loads a longword from a negative offset of a5 into d0 and checks if certain bits are set, otherwise it loads a longword from a negative offset of a5 into d5 and branches.
- `evaluate_move_validity_and_update_board`: Iterates through a list of possible move indices, checking if a move or piece placement satisfies certain boundary and value constraints, and if valid, updates a piece or value in a board structure pointed to by a pointer in a5.
- `load_and_branch_2`: Loads a longword from an offset of minus twenty enough from a5 into d5 and then branches to another routine.
- `check_and_negate_index`: Checks if a longword at an offset of eight from a5 or a negative offset of eight from a5 equals one, then if so, calculates a negative index or a negated value from a table and branches.
- `update_move_list_and_apply_move`: Iterates through a list of move indices, calculates a new target index using an offset from a table, and updates a move-list or board-state array if the new index or a certain value is valid or certain conditions are met.
- `update_score_and_status_flags_logic_2_2`: Increments a counter in a global variable, clears two 16-bit fields in a structure, and calculates a new score or status bitmask based on a table lookup and several conditional checks involving a pointer or a value in a global variable.
- `set_global_byte`: Stores the value one into a global variable at a specific offset.
- `update_score_and_status_flags_logic_2`: Clears two 16-bit fields in a structure, calculates a score-related value from a table using an offset, and updates a status bitmask or score if a certain value matches a global constant.
- `compare_scaled_offset_index`: Loads a longword from an offset of 8 in a structure, shifts it left by 1, and compares a word from an offset of 30 before the structure in a5 against a word at the calculated offset in a table, otherwise it returns.
- `compare_scaled_offset`: Loads a longword from an offset of 8 in a structure, shifts it left by 1, and compares a word from an offset of minus 30 in a structure against a value at a calculated address in a structure.
- `check_scaled_offset_match`: Loads a longword from an offset of 8 in a structure, shifts it left by 1, and compares a word from an offset of minus 30 in a structure against a value at a calculated address in a table; if they match, it loads 50 into a register and jumps to another routine.
- `compare_and_set_score_constant`: Compares a 16-bit value from a structure offset to a global constant, and if they match, loads a constant into a register and jumps to another routine; otherwise, it branches to a different location.
- `update_score_and_status_flags_logic`: Checks if a value in a structure matches a global constant, then calculates a score difference and updates a status bitmask in a structure.
- `update_score_and_status_flags`: Calculates a score difference based on values in a structure and updates a status byte or bitmask in a structure, depending on certain conditional checks or piece-related values.
- `update_status_flags_and_score`: Checks a value in a structure against certain constants and, depending on the result, sets or clears specific bits in a status byte located at an offset from a pointer in a5, or updates a score or counter in a global variable based on certain conditions; otherwise it returns.
- `set_flag_on_match`: Checks if a longword at offset minus 8 of a5 equals 5; if so, it sets bit 1 of a byte at offset 3 of a pointer stored at offset minus 34 of a5 and then branches to another routine.
- `set_status_flag_on_specific_value`: Checks if a 32-bit value at a certain offset from a pointer in a5 equals 1 or 4; if so, it sets bit 7 of a byte located at offset 3 of a structure pointed to by a pointer at offset 34 of the a5-relative structure, then returns.
- `set_flag_on_seven`: Checks if a longword at offset minus 8 of a5 equals 7; if so, it sets bit 1 of a byte at offset 3 of a pointer stored at offset minus 34 of a5 and then branches to another routine.
- `update_score_and_state_flags_2`: Checks if a value at a certain offset in a structure is one or six, and if so, sets bit seven of a byte at offset three of a pointer-referenced structure.
- `branch_on_value_and_jump_table`: Compares a longword in d0 to ten; if it is less than ten, it shifts d0 left by one and uses the result to load a jump target from a table and jumps to it; otherwise, it jumps to a different routine.
- `update_score_and_state_flags`: Updates a score or counter in a global variable based on certain conditions, and if a specific bit in a global flag is set, it updates a pointer-referenced structure with a new value and potentially sets bit 5 or bit 6 of a byte at offset 3 of a structure, then checks for a loop or otherwise returns.
- `increment_pointer_target_conditionally`: Loads a value from a pointer-referenced memory location, stores it back to a local stack-based temporary, and if the value is not negative, adds a constant to that temporary value and stores it back to the stack-based temporary; otherwise it returns.
- `update_table_entry_bit_set`: Checks if a value in a structure equals one or two, then sets bit 5 or bit 6 of a byte at offset 3 of a pointer-referenced structure, otherwise it returns.
- `update_state_and_table_entry`: Checks if a value in a structure equals two, then sets bit 5 of a byte at offset 34 of a pointer-referenced structure and calls another routine.
- `update_table_and_counter`: Calls another routine, then checks if certain values in a table match a register, and if so, clears a word in a table, writes a word from a5 to a table, and increments a longword at a negative offset of a5 in a loop until it reaches a certain value or a condition is met.
- `update_counter_and_branch`: Increments a longword at a negative offset of a5 by 13, then enters a loop that compares a longword at a negative offset of a5 with another longword at a negative offset of a5, and if they are not equal, checks a word in a table using a value from a5, and if that word is non-zero, sets a longword at a negative offset of a5 to 1 before branching to another routine.
- `add_and_branch`: Adds a longword from an offset of minus four from a5 to d4 and then branches to another routine.
- `update_table_and_values`: If a certain value is zero, it increments a value in a table, then writes two words from a5 to a table at an offset of enough from a4, and otherwise it returns.
- `propagate_move_to_neighbor`: Updates a pointer in a table, checks a global flag and a short value, and if certain conditions are met, clears a 32-bit value in a structure.
- `reset_pointer_and_clear_flag`: Clears a 32-bit value in a local structure or stack frame and resets a 32-bit flag or counter to zero, then jumps to another routine.
- `update_bitset_and_pointers`: Calls a routine to calculate an offset, then uses that offset to load a new pointer into a local structure, updates a secondary pointer from a table, and checks certain bitflags and value ranges before potentially updating further pointers or entering a loop that calls another routine recursively.
- `calculate_bitmask_and_load_pointers`: Calculates a bitmask-based bitset, updates a counter in a local structure, and loads two new pointers from a table or array based on an offset derived from that counter.
- `traverse_table_and_update_index`: Calculates a new index or pointer by reading a value from a table using an offset, then checks if this new index falls within a specific range or if a certain bit in a structure is set, potentially calling another routine or updating a global counter or status bitmask.
- `validate_and_dispatch_index`: Compares a value retrieved from a table using a pointer and an offset to a value in a structure, and if they match, calls another routine with parameters loaded from the stack, otherwise it returns immediately.
- `set_d7_and_branch`: Sets d7 to negative one and then branches to another routine.
- `traverse_table_and_validate_index`: Checks a bit in a structure, then calculates a scaled index to retrieve a value from a table, and if that value is non-negative, checks a second table for a specific constant before potentially repeating the lookup or calling another routine.
- `check_table_index_and_call_handler`: Checks if a value retrieved from a table using a scaled index in d0 matches a value in a structure, and if so, pushes three values onto the stack and calls another routine before setting d7 to negative one.
- `set_d7_and_branch_short`: Sets d7 to negative one and then branches a short distance to another routine.
- `iterate_table_and_call_subroutine`: Iterates through a table-based lookup, checking if certain bits are set in a structure, and calls another routine with parameters derived from table lookups and pointer offsets, otherwise it returns.
- `test_table_bitmask_and_load_offset`: Loads a longword from a pointer-indexed location, checks if certain bits are set, and if not, loads another longword from a different offset and branches back to a previous instruction or otherwise returns.
- `iterate_and_validate_table_indices`: Iterates through a range of indices, checking if certain values in a table (indexed via a pointer in a5) match or satisfy specific bitmask conditions, and calls another routine with a calculated index if the conditions are met.
- `evaluate_piece_move_validity`: Checks certain bit flags in a structure and a table, then if certain conditions are met, iterates through a range of values to check if a piece or move exists in a table, calling another routine with a pointer to a structure and a calculated index if a match is found; otherwise it returns.
- `update_score_and_status_flags_logic_2_2_3`: Checks a value in a table using an offset derived from a pointer in a5, and if certain conditions or flag bits in a4 are met, calls another routine with a pointer to a structure or a modified pointer.
- `update_score_and_status_flags_logic_2_2_2`: Checks a condition based on a value in a table, and if certain flags or value ranges match, calls another routine with a pointer to a structure and a calculated index.
- `iterate_and_update_score`: Iterates through a list of indices, checking if a value in a table matches a certain condition, and if so, calls a routine with a pointer to a structure and a calculated index.
- `test_bit_and_load`: Tests bit 2 of a byte at a negative offset from a pointer in a4 and, if clear, branches; otherwise, it loads a longword from a negative offset from a5, masks a bit, and branches if the result is non-zero.
- `iterate_and_validate_index`: Iterates through a range of indices, calculating a pointer-based index and checking if a value retrieved from a table falls within a specific range, then calls another routine with certain structure members if the conditions are met.
- `check_bit_and_load_value`: Tests bit 7 of a byte at a negative offset from a pointer in a5, and if the bit is clear, loads a longword from a different negative offset in a5 into d4 before branching to another routine.
- `iterate_and_update_score_status`: Iterates through a table of values, using an index or a secondary loop, and calls another routine if certain conditions or status flags in a structure are met.
- `validate_and_update_score_index`: Checks specific bit flags and zero-status in a global structure, then iterates through a table to check if a value at an offset matches a condition, and if so, loads a longword from a pointer in a5 and calls another routine or branches.
- `iterate_and_update_score_status_2`: Iterates through a table using an index, checks if a value at an offset in a structure matches a certain condition, and calls another routine with parameters from a structure.
- `load_d4_and_branch`: Loads a longword from an offset of minus twenty enough from a5 into d4 and then branches to another routine.
- `calculate_negative_index`: Checks if a longword at an offset of eight from a5 equals one, then if so, calculates a negative index or a negated value from a table and branches.
- `iterate_table_and_update_pointers`: Iterates through a table using an index, checks if the resulting value points to a valid entry, and if so, calls another routine with a pointer and a value from a structure.
- `update_table_entry_complex`: Updates a table entry by incrementing a value, clearing certain words in a table, and calculating a difference or a value from a table based on certain conditions and then branches.
- `update_table_entry`: Calculates a difference between a value from a table and a stack value, then stores the result into a table entry.
- `update_table_and_reorder_list_2`: Updates a value in a structure, clears a byte in a table, and then iterates through a list of pointers to call another routine if certain conditions regarding table-based values and bitmasks are met; otherwise, it returns or branches to a subsequent routine.
- `reset_context_offset_to_zero`: Clears a 32-bit value at offset negative fifty-two in a structure pointed to by a5 and then branches to another routine.
- `update_move_validation_context_2`: Updates a local structure with values retrieved from a table using an index derived from a pointer, then checks a bitmask in a global structure to decide whether to continue or branch to another routine; otherwise, it updates a local value and returns or branches to a final cleanup label.
- `update_move_validation_context`: Calculates a bitmask-based index and updates a local structure or stack frame with a new move-related value, then checks a bitmask in a local variable to decide whether to branch or continue a loop that validates move targets or calls a routine with a new move index.
- `validate_and_execute_move_sequence`: Calculates a target index from a pointer-based table, checks if the target or its neighbors match a specific type or value, and if certain conditions are met, calls a function with a pointer to a structure and a target index.
- `check_and_execute_move_validation`: Checks if a value at an offset in a table matches a value in a structure, and if that value is not ten, then pushes several pointers onto the stack and calls another routine.
- `traverse_callback_table`: Checks a bit in a structure, then iterates through a table of pointers, checking if a value in a secondary table matches a target value, and if so, calls a callback routine with several arguments.
- `check_and_execute_callback`: Checks if a value in a table, indexed by a bit-shifted version of a register, matches a value in a structure, and if so, pushes several pointers and a register onto the stack before calling another routine and resetting a register to all ones.
- `set_address_register_and_branch`: Sets a longword in a3 to all ones and branches to a different routine.
- `iterate_move_validation_loop`: Iterates through a sequence of possible move indices, checking if a target or its neighbors match a specific type or value, and calls a routine to execute the move sequence if certain conditions are met; otherwise, it continues or exits via a branch.
- `update_board_index_value`: Checks if certain bits are set in a longword at a negative offset from a5, and if so, calculates a new value by masking, shifting, and storing it back to a longword at a negative offset from a5, then branches to another routine.
- `iterate_and_validate_board_position`: Iterates through a range of indices, checks if a value retrieved from a table (based on an offset and a pointer in a structure) satisfies certain range and equality conditions, and if so, calls another routine with parameters passed via the stack.
- `validate_move_bit_mask`: Checks if bit 2 of a byte at a negative offset from a4 is set, and if so, checks if bit 9 of a longword at offset minus 8 from a5 is clear, otherwise it loads a longword from offset minus 40 from a5 into d5 and branches to another routine.
- `iterates_and_validates_piece_moves`: Iterates through a list of possible moves, checking if a piece's destination or move type falls within certain bounds or matches a specific value in a table, and if so, calls another routine with parameters loaded from a structure in a5.
- `load_offset_value`: Loads a longword from an offset of minus 40 from a5 into d5 and branches to another routine.
- `calculate_negative_offset`: Checks if a longword at offset 8 or offset minus 36 of a structure in a5 equals 1, and if not, calculates a negative value based on a word read from a table and branches to another routine.
- `update_move_validation_context_2_2`: Iterates through a list of move-related data, calculating new offset values and updating a table or structure with new indices and values, and calls another routine to update the move validation context or a table entry.
- `update_table_entry_4`: Compares a longword at a certain offset from a5 to a constant, and if not equal, copies a word from a negative offset in a5 to an offset in a table pointed to by a0, then branches to another routine.
- `update_table_and_reorder_list`: Updates several table entries by copying or overwriting values from a source array, calls a routine to calculate an offset, and then performs a conditional update of a list or table, potentially reordering or replacing elements based on a comparison or a recursive call to another routine.
- `copy_and_update_table_entry`: Copies a sequence of 18 words from a source array (offset -128 from a5) to a destination array (offset 36 from a5) using a loop, then updates a table entry at a pointer-based destination with a value constructed from a register and a previously stored byte.
- `update_table_entry_2`: Checks if a value at a certain offset from a pointer in a5 is non-zero, then calculates a new index by shifting that value, loads a word from a table at that index into d0, and jumps to another routine.
- `set_table_to_max`: Stores a maximum unsigned short value into a table entry using an offset from a pointer.
- `update_history_and_bounds`: Updates a stack frame, modifies a longword at an offset in a5, and performs a series of conditional checks and writes to a table or structure based on a value retrieved from a pointer in a5, eventually calling another routine and branching back to a previous instruction.
- `branch_on_value_range`: Compares a value in d0 against a constant, and if the value is less than that constant, shifts d0 left by one and jumps to a destination calculated from a table, otherwise it jumps to a different destination calculated from a table.
- `update_pointer_and_flag`: Checks a range for a value in a table and sets a bit in a byte, then updates a table entry with a value from another table.
- `update_structure_value_3`: Sets up a stack frame, pushes d4 onto the stack, copies a longword from an offset in a5 to d0, clears all bits in d0, and stores d0 back to a different offset in a5; if a longword at an offset in a5 is not equal to one, it continues, otherwise it clears d0 and branches to another routine.
- `update_move_index_and_clear_flags`: Calculates a new index by decrementing a value from a global variable and left-shifting it, then loads a value from a table using this new index and stores it into a field in a structure.
- `update_move_history_and_undo_3`: Updates move history pointers and copies move data from a table, then enters a loop that updates a pointer in a structure, clears a 16-bit entry in a table, and conditionally calls another routine and updates a global pointer or clears a longword in a structure, otherwise it returns.
- `check_and_clear_longword`: Compares a longword at a negative offset from a5 against twenty, and if it is not greater or equal, clears a longword at an offset of 372 in a4 and branches to another routine.
- `update_scaled_pointer_conditional_2`: Compares a longword at a negative offset from a5 against a constant, and if it is not greater or equal, copies a bit-shifted longword from a negative offset in a4 to an offset in a4, then jumps to another routine.
- `update_scaled_pointer_conditional`: Compares a longword at a negative offset from a5 against a constant, and if it is not greater or equal, copies a longword from a negative offset in a4 to an offset of 372 in a4, otherwise it returns.
- `update_scaled_pointer`: Loads a longword from a memory location, shifts it left by 1, and writes it to an offset of 372 in a structure, then checks bit 7 of a byte at an offset of minus 4 in a structure and branches or moves a constant into a register.
- `update_table_with_scaled_offset`: Calculates an offset by shifting a longword from an offset of 12 in a structure left by 2, writes zero to a table at that offset, and if bit 0 of a byte at offset minus 4 of a structure is clear, it loads 1 into a register and jumps to another routine.
- `update_move_metadata_and_state_3`: Calculates a new move index or constant, updates a value in a global table, and modifies a bit in a structure's status byte.
- `push_and_validate_move_index`: Pushes a pointer from a structure onto the stack, calls another routine, and then checks if a value from a structure is less than or equal to a value in a global table, otherwise clearing a register and exiting.
- `update_move_metadata_and_state_2`: Calculates a new move offset or index, updates a pointer in a structure, and calls a routine to reorder a list.
- `update_move_metadata_and_state`: Updates a pointer in a structure, calls a routine to reorder a list, and then modifies flags and values in a structure, including clearing or setting bits in a status byte and updating a counter or flag in a global table based on certain conditions, then calls another routine if certain criteria are met.
- `set_undo_state_flag`: Sets a flag in a table or array based on whether a move counter or a certain value in a structure has exceeded a certain limit, then returns.
- `update_move_history_and_undo_2`: Checks a bit in a structure, then recursively calls itself to advance a move counter and check move history or undo-state flags in a table, before updating a move history array and a pointer or counter in a structure, and finally copies a range of move data from a source array to a destination array.
- `update_move_history_and_undo`: Iterates through a list of move data, copying values from a source array to a destination array, then checks a condition or counter to decide whether to call a routine that potentially updates a game state or undoes a move by restoring a previous state from a pointer-based stack or history list.
- `update_table_entry_pointer`: Decrements a counter in a structure if a certain flag is non-zero, then either calls a routine using a pointer or copies a longword from a source table to a destination table, and finally updates a pointer in the structure to point to a new table entry.
- `swap_table_values_2`: Calculates a pointer to a table entry, copies a longword from a source pointer to a destination pointer, updates a pointer in a structure, and then conditionally updates a table entry with a value from a source or destination if certain conditions or comparisons are met.
- `swap_table_values`: Compares two values in a table and swaps them if they differ, then potentially updates a table entry with a value from a local variable.
- `load_pointer_and_byte`: Sets up a stack frame, loads a pointer from a table into a2, reads a byte from that pointer into d4, and then branches.
- `shift_memory_left`: Shifts a sequence of bytes left by one position and terminates the sequence with a zero byte.
- `update_list_node`: Modifies a linked list or a similar data structure by updating a pointer or value, potentially deleting or inserting a node based on a returned length or count.
- `update_tree_recursively`: Recursively calls itself while updating a global pointer or value.
- `load_indirect_address`: Loads a long-word from a table into a register using an index and a base pointer.
- `traverse_tree_nodes_2`: Sets up a stack frame, saves a register, clears a local variable, and if a global pointer is non-null, calls a routine with a pointer and a constant, clears a global, then loads a pointer from a table, updates local stack pointers, and calls another routine or jumps to another routine.
- `traverse_tree_nodes`: Iterates through a linked structure, updating a pointer or a value in a local stack frame, and checks if a child node's pointer or a specific value matches a previously stored pointer.
- `shift_pointer_left`: Loads a pointer from a local variable, reads a long-word from that pointer, shifts it left by two bits, and writes the result back to a local variable before branching.
- `process_tree_node`: Loads a pointer and a byte from a table, then recursively calls another routine while updating a global variable with a pointer or a value, and potentially updates a pointer at an offset in a structure.
- `traverse_tree_nodes_child`: Updates local stack-based pointers by copying a value from a previous stack offset, then checks if a pointer-based structure or child node exists by masking and shifting a pointer in a2; if a certain condition is met, it calls another routine and returns a value from the stack, otherwise it continues a loop or exits.
- `initialize_hunk_type_loop`: Initializes a local stack-based structure, clears several global or heap-based variables, and enters a loop that checks if a hunk type or counter has reached a certain limit, otherwise it continues to a subsequent routine.
- `load_game_address`: Loads an address into a local variable and branches.
- `load_address_pointer_and_branch`: Loads the address of a constant or data or instruction into a register, stores that address into a global pointer at a negative offset from the stack pointer, and branches to another routine.
- `save_game_address`: Loads an address into a local variable, loads another address into a global variable, and branches.
- `parse_hunk_type_and_load_data`: Checks if a value in d0 matches specific hunk type constants and, if so, updates a global pointer; it then calls a routine to parse a structure, calls another routine to handle a pointer, and calls a third routine to perform a memory-related operation before storing the result in a global variable and returning or branching.
- `initialize_string_and_parse_hunk`: Initializes several string-related or structure-related pointers, checks for specific character or hunk type constants, and traverses a linked list of pointers while performing string comparisons or length checks.
- `check_hunk_type_and_clear`: Checks if a byte at a certain offset in a structure pointed to by a4 is zero, and if so, clears d0 before branching to another routine; otherwise it returns immediately.
- `validate_hunk_header_buffer`: Clears a 35-byte buffer in a structure, then checks a pointer or a longword in that structure to decide whether to call a routine that performs a recursive tree update or a routine that clears a buffer or checks a pointer.
- `initialize_hunk_header_buffer`: Clears a buffer of 35 bytes in a structure, then checks a pointer or a longword in that structure to decide whether to call a routine that processes string buffers or calls another routine to handle relocation or executable loading.
- `calculate_index_range`: Compares a register against 4, and if not less or equal, copies that register to another register and subtracts 4 before branching to another routine.
- `validate_hunk_type_codes`: Iterates through a range of five memory locations, comparing a masked byte from a structure field against a byte from a lookup table, and sets a flag in a structure if a mismatch is detected.
- `merge_hunk_list_nodes`: Updates a linked list of structures by comparing type or size fields, potentially replacing a node with a new pointer and traversing or updating the list if certain conditions or hunk type codes match.
- `compare_and_traverse_hunk_list`: Compares the size or type field of two structures pointed to by offsets in a5, and if they match, calls another routine with pointers to the next items in the list, potentially traversing a linked list of structures while updating a pointer in the structure or a global variable.
- `update_hunk_pointer_2`: Checks if a longword at a negative offset of a structure in a5 is zero, and if not, loads that longword into a0 and writes a longword from a negative offset of the same structure to the address in a0, then branches to another routine.
- `update_hunk_pointer`: Copies a longword from an offset of a structure in a5 to another offset in the same structure, copies that same longword to a new destination pointed to by a pointer in the structure, and increments a longword at a different offset in the structure before branching to another routine.
- `check_hunk_index`: Clears a bit in a byte at an offset of a structure in a5 and compares a longword at an offset of that structure against a constant; if the value is greater or equal, it branches to another routine, otherwise it loads a longword with all bits set into d0 and branches to another routine.
- `process_amigadot_hunk_headers_2`: Iterates through a list of hunk headers, checking for specific type codes and updating a status byte or string buffer in a structure, or calling other routines to handle certain relocation or data types, and may clear a memory range if a certain type is encountered.
- `traverse_linked_list_nodes`: Traverses a linked list of nodes by following pointers, updating a pointer in a local structure with the address of the next node, and calls another routine to check if a certain value exists or if a node should be processed, otherwise it returns.
- `check_node_existence`: Checks if a pointer or value exists at a certain offset, and if a subsequent routine returns a non-zero value, calls another routine to update a flag or value in a structure and then calls a third routine to reset a value to one.
- `push_and_call_subroutine`: Pushes two longwords from a structure in a5 onto the stack, calls check_pointer_limit, adjusts the stack pointer, pushes a longword from a4 onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `validate_hunk_type_range_2`: Checks if a hunk type code loaded from a pointer in a4 is less than a value in a5 and less than 9, then sets bit 1 of a byte in a5 if these conditions are met; otherwise it returns.
- `push_and_call_subroutine_2`: Pushes two longwords from a structure in a5 onto the stack, calls check_pointer_limit, adjusts the stack pointer, and branches to another routine.
- `check_hunk_type_validity`: Loads a pointer from a structure offset, checks if the pointed-to value is null, and if not, loads the value at that address into a register before branching to another routine.
- `update_global_pointer_2`: Copies a 32-bit value from a structure offset to another offset in the same structure, pushes a constant onto the stack, calls another routine, and then updates a global variable with a constant before branching to another instruction.
- `validate_hunk_type_range`: Subtracts a constant from a register and branches to another routine if the result is zero, or subtracts a different constant and branches to another routine if the result is zero; otherwise it branches to a third routine.
- `parse_amigadot_hunk_headers`: Checks if a value loaded from a pointer in a4 matches specific AmigaDOS hunk type codes, then performs table lookups or updates a string buffer with path or name information, and calls another routine or a string-copying routine depending on the hunk type.
- `parse_hunk_type_headers_2`: Calculates an offset into a table, checks if a pointer or a certain length or type code is valid, and if so, clears a range of bytes in a global buffer, then either scans backwards through a buffer to clear bytes until a slash or colon is found or copies bytes from a source table into a destination buffer until a null terminator or a certain limit is reached.
- `parse_hunk_type_headers`: Iterates backwards through a memory buffer, clearing bytes at a certain offset, and checks if any byte equals a slash or a colon; if a match is found, it checks a hunk type code and potentially clears or modifies characters in the buffer or calls another routine to handle the header or sanitize characters.
- `scan_hunk_type_headers`: Iterates backwards through a memory buffer, clearing bytes at a certain offset, and checks if any byte equals a slash or a colon; if a match is found or the counter reaches zero, it calls a routine to copy a buffer from a table and then calls another routine with a pointer to the buffer, a source address, and a length.
- `sanitize_hunk_type_chars`: Checks if a byte at a certain offset from a pointer equals a specific character, and if not, overwrites it with a slash or a forward slash, then increments a counter and continues a loop that copies bytes from a source table into a destination buffer until a null terminator or a maximum length is reached.
- `copy_buffer_from_table`: Iterates through a source table, copying bytes to a destination buffer and incrementing a counter, until a null byte is encountered or a maximum length of 35 is reached; otherwise, it updates a counter in a structure and jumps to another routine.
- `scan_and_populate_buffer`: Iterates through a buffer, copying bytes from a source table until a null byte or a certain limit is reached, then updates a counter in a structure and calls another routine with a pointer to a structure and a value.
- `clear_and_branch_to_target`: Clears a longword in a structure at a negative offset from a5 and performs an unconditional short branch to another routine.
- `set_long_word`: Sets a long-word to one in a structure at a negative offset from a5 and branches to another routine.
- `clear_and_set_long_word`: Clears a long-word in a structure at a negative offset from a5, sets a long-word to one at a different negative offset from a5, and branches to another routine.
- `process_hunk_type_headers`: Checks if a value in d0 matches specific AmigaDOS hunk type codes, and if so, performs pointer-based table lookups, clears or updates certain fields in a structure pointed to by a5, and copies or modifies string data or length-prefixed strings in a structure pointed to by a4 before calling other routines.
- `process_amigadot_hunk_headers`: Checks a hunk type code against several constants and, if a match is found, updates a status byte in a structure, clears a string buffer, or performs string manipulation such as stripping path separators or finding a colon or slash, and calls another routine or a function pointer.
- `initialize_string_buffer_and_increment_2`: Clears a structure or buffer, initializes a string-related buffer or pointer, and then performs a series of conditional checks and updates to a status byte and a pointer-based value, potentially calling other routines to allocate or locate a buffer or handle string-related data.
- `initialize_string_buffer_and_increment`: Checks if a pointer-based value is zero, sets a bit in a status byte, and if so, calls a routine to allocate or locate a buffer, then writes a constant value to a new memory location and increments a counter in a structure.
- `branch_to_address_3`: Performs a short branch to a fixed address.
- `trim_string_buffer`: Calls several subroutines and then loops backwards through a buffer to clear any space or null characters, and if a certain longword in a structure equals 5, loops backwards through another buffer to clear space or null characters.
- `clear_string_suffix`: Checks if a longword at an offset of 8 from a pointer in a5 equals 5, then loops backwards through a buffer, clearing bytes to zero if they match a space or a null terminator, or if they match a space character.
- `load_and_branch_3`: Loads a longword from a location relative to a5 into d0, then performs a short branch to a fixed address.
- `check_pointer_limit`: Decrements a pointer in a5 by 65532 and, if the value at offset 8 of a5 is not greater than 8, otherwise returns.
- `iterate_pointer_list`: Loads a pointer from a structure in a5, then enters a loop that iterates through a linked list of structures, updating a bit in a global flag, calling other routines to manipulate data, and storing the current pointer into a table or array located at an offset from a5.
- `initialize_register_values`: Loads a longword from a pointer in a5 into a0, then copies a longword from a0 into a location relative to a5, increments d4, and if d4 is less than d5, branches back to the start of the routine; otherwise, it checks a value at an offset in a5 and, if it is greater than or equal to 8, copies a longword from a5 into d0 and branches forward.
- `initialize_registers_and_branch`: Loads certain constant values into registers d5, d4, and d6, then branches to another routine.
- `execute_complex_logic`: Calls a function, modifies a bit in a pointer-based address, and performs several conditional calls and stack-based argument passing involving a global variable and a pointer from a5.
- `update_state_and_buffer`: Performs a series of calls to update a global pointer and its associated data, modifies a bit in a memory location, and copies a longword from a table into a destination pointer or a buffer, then loops until a certain condition is met.
- `call_function_with_args`: Calls a function pointer using a pointer from a global variable with several arguments, including a long or integer value.
- `compare_case_insensitive_strings`: Compares two strings byte-by-byte, converting lowercase ASCII characters to uppercase, and returns a value indicating if the first string is less than, equal to, or greater than the second string.
- `compare_buffer_values`: Loads a longword from a pointer in a5 into d0, compares it to another longword at a different offset in a5, and either branches or sets d0 to 1 and branches.
- `branch_on_null_check`: Tests if a longword at an offset of negative 4 from a pointer in a5 is non-zero, otherwise it clears d0 and branches to another instruction.
- `branch_to_target_address_4`: Performs a short branch to a fixed address in the code.
- `set_zero_d0`: Clears d0 and branches forward.
- `execute_with_stack_setup`: Sets up a stack frame, pushes a pointer, calls another routine, adjusts the stack pointer, and if d0 is not zero, calls a routine before returning.
- `push_and_call_pointer`: Pushes a pointer onto the stack, calls a routine, restores the stack pointer, checks d0, and if d0 is not zero, calls another routine, clears d0, and branches forward.
- `set_structure_values`: Compares a value at an offset relative to a4 against 16 and, if it is less than 16, checks if a value at offset c of a5 is 2 and, if not, writes 15 to offset -10 of a5 and 1 to offset -14 of a5, then branches forward.
- `rebuild_stack_node_properties`: Iterates through a list of structures, updating or creating new nodes by calculating new pointer offsets and property values, and calls other routines to retrieve or update certain data or pointers.
- `update_stack_values`: If a value at a5 plus offset 12 is not all ones, it loads values from a5, calls a function, and writes the result back to a5, then repeats a similar process with another function call before branching.
- `update_stack_node_properties`: Updates a linked list or array of structures by calculating new offset or pointer values, calling other routines to retrieve or update certain properties, and updating a global counter or pointer.
- `write_to_memory_ranges`: Calls a function pointer twice to write a certain amount of data to two different memory locations.
- `execute_nested_function_calls_2`: Pushes a pointer from a structure or global variable onto the stack, calls another routine, and if the return value is non-zero, calls another routine twice with a pointer from a structure, otherwise it returns.
- `execute_nested_function_calls`: Pushes a pointer from a structure or global variable onto the stack, calls a routine, and cleans up the stack, repeating this process for a second pointer and then calling two additional routines using pointers retrieved from the stack, before returning a value of one in d0.
- `initialize_engine_state`: Clears several local stack variables and certain global memory locations, then calls a routine with a pointer to a stack-allocated structure and several integer arguments before checking a return value to decide whether to continue or return.
- `process_nested_function_calls`: Calls a function with several arguments, then checks if the return value is non-zero to enter a loop that executes a sequence of nested function calls.
- `process_node_and_update_structure`: Checks if a pointer in a structure is null and if a certain value in a global structure equals 104; if so, it pushes several pointers from a global structure onto the stack, calls a routine with those pointers, and then calls another routine with a pointer or a value from a register before branching to a final instruction.
- `compare_and_extract_bits`: Compares a value from a pointer in a4 to another value in a4, and if they are equal, pushes a pointer, calls another routine, pushes another pointer, calls another routine, and branches to another routine.
- `load_loop_counter`: Loads a 32-bit value from a pointer located at a negative offset from a4 into d0 and branches to a different routine if the branch condition is met.
- `push_and_call`: Pushes a pointer relative to a5 onto the stack, calls a function, adjusts the stack pointer, and then branches to a fixed address.
- `decrement_and_branch`: Subtracts 4000 from a value in d0, then decrements d0 twice and branches if d0 is not zero or if a certain condition is met.
- `process_event_and_update_structure`: Checks if a flag at offset -60 of a structure in a5 is non-zero, clears it, and calls another routine with a pointer from a4.
- `check_structure_value`: Loads a longword from an offset of minus 4 in a structure, compares it to 1, and if not less or equal, performs a series of register loads and comparisons against a value at a large negative offset in a structure before branching.
- `load_and_compare_pointer`: Loads a longword from a pointer in a4 into d0, then copies that value to an offset of minus 8 in a5, and then compares d0 with the value at a certain offset in a4 before branching.
- `initialize_and_iterate_structure_list`: Initializes a structure field with a value from a pointer in a4, then enters a loop that calls a routine to update structure offsets and recursively calls itself while incrementing a counter in a5, otherwise it returns.
- `push_and_call_handler_2`: Pushes a longword from a negative offset of a structure in a4 onto the stack, calls a routine, adjusts the stack pointer, clears a longword at a negative offset of a structure in a5, and then branches to another routine.
- `load_event_type_to_d0`: Loads a longword from a pointer-relative offset into d0 and then branches to a fixed address.
- `push_pointers_and_call`: Pushes a pointer from a4 onto the stack, calls another routine, pushes another pointer from a4 onto the stack, calls another routine, and branches to another routine.
- `setup_and_call`: Pushes a pointer from a4 and a pointer from a5 onto the stack, calls a function, clears a value at a5 plus offset minus 56, pushes another pointer from a4, calls another function, adjusts the stack, and branches.
- `push_and_call_subroutines`: Pushes a pointer from a structure in a5, calls another routine, adjusts the stack, pushes a pointer from a4, pushes a pointer from a4, calls another routine, adjusts the stack, pushes a pointer from a4, and calls another routine before branching to a different address.
- `push_and_call_subroutines_alt`: Pushes a pointer from a5, calls another routine, adjusts the stack, pushes a pointer from a4, pushes a pointer from a4, calls another routine, adjusts the stack, pushes a pointer from a4, and calls another routine before branching to a different address.
- `clear_structure_member`: Clears a long-word value at a negative offset from a5 and then branches to another routine.
- `iterate_and_update_structure_members`: Iterates through a list of pointers, calling a routine to extract a value and store it in a structure member, then updates a secondary structure member using a calculated offset, and increments a counter in a structure until a certain limit is reached.
- `initialize_copy_params`: Sets a longword at a5 minus 20 to a constant, copies a word from a4 to a5, pushes a constant onto the stack, calls a function, adjusts the stack, sets a value at a5 minus 60, and branches.
- `update_color_exchange_state`: Loads a constant into a field of a structure, updates a 16-bit value in a structure using a value from a pointer, and calls a routine using a relative address before updating the stack pointer and setting a global variable to one before branching to another routine.
- `update_color_pointer_and_call_handler`: Updates a pointer in a structure, modifies a value in a structure, and calls a routine using a relative address before updating the stack pointer and setting a global variable to one.
- `update_structure_pointers_and_loop`: Toggles a bit in a global variable, calculates a new pointer or address, and enters a loop that updates a structure member with a new address or pointer, then calls another routine with a pointer or a specific value, and finally calls a third routine recursively.
- `dispatch_event_handler_2`: Checks a value in d0 against several constants to decide whether to execute a branch or a loop that updates a structure member, or to call other routines that modify structure members or global variables.
- `decrement_and_test_d0`: Decrements a value in d0 by eight and checks if it is zero, then decrements d0 by eight again and checks if it is zero, then decrements d0 by sixteen and checks if it is zero, then decrements d0 by thirty-two and checks if it is zero, otherwise it branches to an earlier instruction in the routine.
- `branch_to_target_7`: Performs a short branch to a target address.
- `write_memory_with_calls`: Pushes a value onto the stack, calls a function pointer, adjusts the stack, calls another function, clears d0, and then performs a short branch.
- `check_pointer_validity`: Checks if a longword at a certain offset relative to a4 is non-zero, otherwise it unlinks the stack frame and returns.
- `update_structure_and_loop`: Loads a pointer from a structure, increments values from that structure, and stores them in a local structure; it then calls a function, updates a global variable, and enters a loop that calls a function and increments a counter until a certain value is reached, finally calling another function and returning.
- `update_structure_fields`: If a value at offset 8 of a5 is less than 2, it pushes a pointer from a structure, calls a routine, and updates several offsets in a5 by subtracting a masked value derived from the return or a value in a5, then checks if a certain value is negative and potentially negates it.
- `negate_structure_value`: Loads a longword from offset -4 of a5 into d0, copies d0 to the stack, checks if the value at offset -8 of a5 is negative, and if so, loads that value into d0 and negates it before branching forward.
- `negate_and_load_value`: Loads a longword from a structure, adds it to a value from the stack, then checks a value at an offset in a structure and either negates a value or branches.
- `sum_and_compare`: Adds a longword from a structure in a5 to d1, then compares d1 against a constant and branches if d1 is not less than 4.
- `push_and_call_params`: Pushes four values from a structure onto the stack, calls a function, restores the stack pointer, and performs a short branch.
- `call_function_if_nonzero`: Checks if a global variable is non-zero, then calls a function pointer using that variable and a provided parameter.
- `extract_bits_and_store`: Checks a global variable, then calls another routine to extract bits from a memory location, stores the result in a structure, and performs a bit-shift or right-shift on a value before storing it in a structure, then calls another routine.
- `extract_and_pass_bits`: Extracts certain bits from three different memory locations and passes them as arguments to another routine.
- `update_structure_values_3`: Clears a value in a structure, calls a routine to perform arithmetic on a value from a structure, and then compares or updates several values in a structure, branching if certain comparison conditions are met.
- `compare_structure_values`: Loads a 32-bit value from offset -8 of a5 and compares it to a 32-bit value from offset -12 of a5, then branches if the first value is less than or equal to the second.
- `sync_structure_offsets`: Copies a value from offset -12 of a5 to offset -16 of a5, then copies a value from offset -4 of a5 to offset -8 of a5, and if certain conditions are not met, copies a value from offset -4 of a5 to offset -8 of a5.
- `compare_offset_values`: Compares a value at offset -8 of a5 with a value at offset -12 of a5, and if the first is not greater or equal, loads it into d0 and branches forward.
- `update_structure_offsets`: Calculates differences or new values based on existing structure offsets and updates a third offset in the structure, then calls other routines to store or reset values.
- `update_offset_with_difference`: Compares two values at offsets in a structure, and if they are equal, calculates a difference, adds a constant, and stores it back to a structure.
- `update_offset_value`: Calculates a new value by subtracting a value at an offset from another and adding 4000, stores it in a structure, and if the value is negative, calls two functions and updates a structure offset with a new value.
- `reset_and_set_value`: Checks if a value at a certain offset is negative; if not, it sets d1 to 60, loads a value from a structure into d0, calls a function, sets d1 to 1000, calls another function, and stores d0 into a local variable before branching.
- `reset_global_variable`: Sets a global variable to zero and returns that value.
- `update_structure_on_condition_2`: Checks if a value at offset 12 of a5 is less than 3 or greater than or equal to 392, otherwise it clears offset 12 of a5 and performs a series of calls to update various offsets in a structure, including calculating new values using subtraction and addition, and finally branches to another routine.
- `update_local_pointers`: Sets d1 to 15, loads a value from a structure into d0, calls a function, adds 50 to d0, sets d1 to 100, calls another function, stores d0 into a local variable, copies a value from one local variable to another, copies another local variable, and branches.
- `update_pointers_and_call`: Copies a value from a local variable to another, sets d1 to 15, loads a value from a structure into d0, calls a function, adds 50 to d0, sets d1 to 100, calls another function, stores d0 in a local variable, copies a local variable, and branches.
- `update_pointer_and_increment_counter`: Updates a pointer in a structure and calls another routine, then increments a value in a register and calls another routine before storing the result back to a structure and branching to a new location.
- `update_multiple_pointers`: Copies two local variables to other local variables, sets d1 to 15, loads a value from a structure into d0, calls a function, adds 50 to d0, sets d1 to 100, calls another function, stores d0 in a local variable, and branches.
- `update_pointers_and_branch`: Copies two local variables to other local variables, sets d1 to 15, loads a value from a structure into d0, calls a function, adds 50 to d0, sets d1 to 100, calls another function, stores d0 in a local variable, and performs a short branch.
- `update_pointer_and_call_function`: Adds a constant to a value in d0, calls another routine, then updates a global variable with the new d0 value and shifts several values in a structure in a5.
- `update_structure_on_condition`: Checks d0 against several constants, and if a match is found, updates certain offsets in a5 with values from other offsets or from the return of another routine, then calls another routine.
- `extract_bits_and_update_values`: Pushes a pointer, calls another routine, and then extracts bit-masked values from a structure member into other structure members before calling another routine with certain values, and finally performs a series of comparisons and conditional branches or jumps to other routines.
- `compare_pointer_range`: Compares a value at offset -20 of a5 with a value at offset -24 of a5, and if the first is not less or equal, loads it into d0 and branches forward.
- `check_pointer_range_2`: Copies a longword from offset -24 of a5 to offset -10 of a5, then compares the value at offset -1c of a5 with the value at offset -20 of a5 and, if it is not greater or equal, compares the value at offset -1c of a5 with the value at a5 offset -24 and, if it is not greater or equal, copies the value at offset -1c of a5 to d0 and branches forward.
- `compare_and_branch_on_value`: Compares a longword from a structure against another longword from a structure and branches if the first is not greater than or equal to the second.
- `update_global_with_loop`: Calculates a difference between two values in a structure, calls a function, and then executes a loop that calls a function and increments a value in a structure until a certain limit is reached.
- `compare_and_update_global`: Compares a value from a structure in a5, and if not less than or equal, calls another routine and updates a global variable with the result.
- `populate_memory_with_loop`: Calls a function to retrieve values and stores them in a list of global variables, then executes a loop that increments a value and calls a function repeatedly.
- `update_move_indices_and_validate_board_2`: Initializes a local array with values from a global table, then iterates through a range of indices to check a table and potentially decrement values in that array.
- `clear_and_branch_2`: Clears a longword in a structure and then branches to another routine.
- `update_move_indices_and_validate_board`: Updates a pointer in a structure by adding a value returned from a function call, then iterates through a range of indices to check a table for piece presence and valid move conditions, updating a counter or pointer in the structure if certain conditions are met, and finally enters a nested loop that updates a move-tracking variable and checks a bitmask or table for valid move indices.
- `scan_board_for_valid_moves`: Iterates through a range of possible move indices, checking a board or piece table for valid piece presence, and if a valid piece is found, it executes a nested loop to evaluate potential move coordinates, updating a move history or score-related structure and potentially calling other routines to validate or store move-related data.
- `clear_and_branch_offset`: Clears a longword in a structure and then branches to a relative offset.
- `update_structure_and_validate_index`: Calculates a new value for a longword in a structure by adding a returned value from a routine to an existing value, then checks a condition to either update a pointer or a secondary value in the structure.
- `move_and_branch`: Copies a longword from one location in a structure to another location in the same structure and then branches to a relative offset.
- `calculate_d0_offset`: Tests a longword in a structure, and if it is not zero, calculates a new value for d0 by subtracting a value in a structure from a constant and then branches.
- `update_move_index_and_validate_board`: Updates a move index in a structure, checks if a move or piece index is within a valid range, and if certain conditions are met, searches a table to validate a move or piece, potentially updating a global state and calling other routines to handle game state or board updates.
- `initialize_position_state_table`: Clears several memory locations and a table of 24 entries, then enters a loop that clears or updates specific array indices or state flags based on a pointer-driven index.
- `extract_and_update_position_state`: Extracts piece or position-related data from a structure, updates a state variable or table with a modified index, and calls a routine to evaluate the position.
- `set_constant_register`: Loads the constant 13 into a register and branches to another routine.
- `update_position_and_evaluate_state`: Updates a state variable in a structure by packing or or-ing values from a table, calls a routine to evaluate the position, and modifies a global table or structure member based on certain bit-flags or conditional checks, then calls several other routines to update game state or clear entries.
- `update_game_state_and_evaluate_position`: Updates a state variable in a structure, clears certain entries in a global table, and executes a sequence of conditional checks and functions.
- `reset_game_state_and_evaluate_position`: Updates a state variable in a structure, clears certain entries in a global table, and executes a sequence of conditional checks and function calls that update a global state or error code based on the current game status or piece position.
- `reset_state_variable`: Clears a 32-bit value in a structure pointed to by a5, loads a new value from a pointer-offset in that structure into d0, and branches back to the start of the routine, forming a loop.
- `update_structure_state_3_2`: Writes a longword containing one to an offset of a structure in a4, then writes a longword from a pointer in a5 to another offset in a4, and otherwise writes a longword containing three to an offset in a4 before branching.
- `update_structure_state_3`: Calls a routine using a pointer from a structure, calculates an offset, and checks a value in a table; if certain conditions are met, it updates a longword in a structure and calls another routine.
- `check_and_set_value`: If a longword at offset or a certain value is not equal to two, it sets a longword at offset or a certain value to six; otherwise, if a longword at offset minus twelve is not zero, it sets d0 to one and jumps to another routine.
- `set_zero_at_offset`: Stores a zero-filled longword into a structure at offset minus sixteen using a pointer in a5, then jumps to another routine.
- `check_bit_and_call_routine`: Checks bit two of a byte at offset minus forty-five in a5; if set, it calls a routine with a constant, then clears a longword on the stack and calls another routine before jumping to another routine.
- `set_structure_index_2`: Writes a longword containing one to an offset of a4, then writes a longword from a pointer in a5 to another offset in a4, and if certain bits are set, calls another routine twice while performing pointer arithmetic and checking a table, otherwise it writes a longword containing either three or five to an offset in a4 and calls another routine.
- `set_structure_index`: Sets a longword at offset 336 of a structure to 3 and then branches to another routine.
- `update_structure_state_2`: Calculates an index using a function return and a pointer-based offset, checks a value in a table, and updates a longword in a structure.
- `set_zero_at_offset_alt`: Stores a zero-filled longword into a structure at offset minus sixteen using a pointer in a5, then jumps to another routine.
- `test_bits_and_set_long`: Tests if bit 1 or bit 3 of a byte at offset 46 of a structure in a5 is clear; if bit 1 is clear, it writes a longword containing 4 to offset 336 of a structure in a4 and branches to another routine; if bit 3 is also clear, it branches to another routine; otherwise it writes a longword containing 4 to offset 336 of a4 and branches to another routine.
- `update_long_if_bit_set`: Tests if bit 4 of a byte at offset minus 46 of a5 is clear; if so, it writes a longword containing 26 to an offset 336 of a4 and then branches to another routine.
- `update_structure_value_2_2`: Tests bit 2 and bit 4 of a byte at an offset in a5, and if bit 4 is clear, writes a longword containing twenty-five to an offset in a4 before branching.
- `update_structure_value_2`: Tests bit 2 of a byte at offset 46 of a structure, and if set, sets a longword at offset 336 of a structure to 5 before branching to another routine.
- `call_routine_with_parameter_2`: Calls another routine, then if a register contains zero, writes a longword containing twenty-seven to an offset in a4 before branching.
- `call_routine_with_parameter`: Tests bit 2 of a byte at offset 47 of a structure, and if set, pushes a constant onto the stack, calls a routine, adjusts the stack pointer, and branches to another routine.
- `update_structure_with_value`: Checks if a longword at a certain offset is one, and if so, calls a routine with a certain value; then, if a longword at a certain offset is not zero, it copies a longword from a certain offset of a structure in a5 to a certain offset of a structure in a4, and sets a longword at offset minus sixteen of a structure in a5 to zero or two, and finally checks certain conditions to copy a longword or call another routine.
- `clear_frame_counter_and_dispatch`: Sets a 32-bit local slot to zero, then conditionally invokes a callback through a function pointer in the global data area when a flag bit is set, and another callback when a structure pointer is non-null, before updating a timing-related global and calling a final routine with a fixed value of 1024.
- `decrement_and_branch_on_zero_3`: Subtracts a large constant from a register and branches if the result is zero, then subtracts a different constant from the same register and branches if the result is zero, then decrements the register by two and branches if the result is zero, otherwise it jumps to another routine.
- `update_move_index_and_validate_board_2`: Checks a bit in a global variable and calls a routine, then checks if a pointer in a structure is non-null to call another routine with a pointer from a second structure, and if a certain condition involving a table or move index is met, updates a value in a global structure before calling a routine and potentially calling another routine if a certain value equals 499.
- `validate_and_update_move_index_2`: Searches a table for a matching move index, and if found, updates a pointer or counter, calls a routine to check move validity, and then iterates through a range of move indices to check for further move validity or state changes, updating status bits in the move structure or returning a status code.
- `validate_and_update_move_index`: Iterates through a table to check if a move index or pointer matches a value derived from a table, updates a pointer or counter, and then checks a flag or range to update a structure or call other routines.
- `load_constant_and_branch_2`: Loads the constant 13 into d0 and then branches to another routine.
- `update_move_state_and_evaluate`: Updates a state-tracking longword in a structure using a value derived from a table, then calls a routine to update a structure, followed by a routine that updates a global state.
- `update_game_state_and_evaluate_move`: Checks certain flags in a structure and a global table to update a state-tracking longword, potentially calls a routine to update a structure member, and updates a global state or pointer based on certain conditions or a check of a move's validity or a timer-related value.
- `set_constant_value_2`: Sets a longword at offset minus forty-four of a structure in a5 to three and then jumps to another routine.
- `update_structure_member_2`: Checks if a longword at offset minus forty-four in a5 is zero; if not, it checks bit two of a byte at offset minus fifty-three in a5; if not set, it writes a constant four to a longword at offset minus forty-four in a5, then compares that value to a constant and branches.
- `set_and_branch_two`: Sets a longword in a structure to a constant and then branches to another routine.
- `updates_pointer_value`: Writes the constant value 6 to a 32-bit location stored at an offset of 44 from the pointer in a5, then branches to another routine.
- `updates_structure_member_and_jumps`: Writes the constant value four to a longword located at offset negative forty-four from the pointer in a5 and jumps to another routine.
- `update_board_and_toggle_state_2_2`: Updates a structure member with a constant or a value derived from a calculation, modifies a board-related table or index, modifies a global state or pointer, and conditionally clears or updates entries in a table or struct.
- `update_board_and_toggle_state_2`: Updates a board-related table or index, modifies a global state or pointer, and conditionally clears or updates entries in a table or structure based on certain flags, then calls several other routines and returns.
- `update_board_and_toggle_state`: Calculates a wrapped index, clears two bytes in a table, and calls an external routine.
- `call_external_and_branch`: Clears the stack, calls another routine, increments the stack pointer, and then branches to another routine.
- `toggle_and_check_state`: Toggles a global byte using an XOR operation, performs conditional external routine calls, and copies a value to a global variable based on table lookups.
- `toggle_global_state`: Pushes several registers onto the stack, calls a routine that toggles a global byte and updates a pointer, then cleans up the stack and moves a value from a local variable into d0 before branching.
- `reset_and_validate_buffer_state`: Clears several fields in a structure, resets a pointer-based counter, and checks a table to validate or update buffer entries with specific values.
- `update_buffer_and_validate_state`: Iteratively writes zero to a buffer or updates buffer entries with a value of seven or five based on a comparison with a table, then checks a global flag to potentially reset a counter and execute a series of other routines or clear a memory range.
- `update_buffer_loop`: Sets d0 to 5, shifts a pointer, and writes d0 to a memory location using a loop that decrements a pointer and checks if it reaches a certain value; otherwise, it checks a bit in a global variable and branches.
- `reset_buffer_and_update_pointer`: Executes several external routine calls, updates a global variable with a return value, and clears a memory buffer or array using a loop.
- `update_score_and_clamp_range`: Calculates a difference between two values in a table, stores it, and then clamps a value in a global variable to a range between zero and ten.
- `clamp_value_to_range`: If a longword at offset minus sixty or a certain value is less than or equal to ten, it sets that longword to ten, clears a longword at offset minus sixty-four, and then checks if that cleared longword is zero; otherwise, it sets a longword at offset minus eighty-eight to ten and a longword at offset minus eighty-eight to thirteen, then jumps to another routine.
- `update_score_and_table_indices_2_2_2`: Iterates through a table of player or game statistics, checking if certain values fall within a valid range and if a difference between two structure members is non-negative, then updates a counter and certain score-related or index-related values in a structure passed via a5.
- `update_score_and_table_indices_2`: Calculates a difference between a global value and a structure member, then uses that value to update a score or index in a structure, following the logic of updating score or index in a structure.
- `update_score_and_table_indices`: Calculates a difference between a global value and a structure member, then uses that value to update a score or index in a structure, followed by several lookups in a table and calls to another routine to update or check other structure members or global values.
- `branch_on_d0_value`: Compares a longword in d0 to a constant, and if it is less than or equal to that constant, shifts d0 left by one and jumps to a location determined by a table lookup.
- `update_score_and_table_indices_2_2`: Checks if a value in a structure certain range, then calculates the absolute difference between two other structure members and updates a score or index.
- `calculate_and_assign_pointer`: Performs complex arithmetic involving multiple global variables and table lookups to update a pointer, while also checking conditions to reset certain pointers to zero or decrementing a value.
- `update_board_and_move_index`: Updates a board state and move index by copying values between tables, decrements a counter in a structure, and calls a routine to handle piece removal or movement, while also updating a global counter and potentially adjusting a move limit or index based on certain conditions.
- `update_move_index_and_board_state`: Checks if a certain range of values is met, then updates a pointer in a structure with a decremented value and modifies a value in a structure based on a lookup table.
- `branch_to_table_entry`: Decrements a longword in d0, and if it is less than eight, shifts d0, loads a word from a table using d0 as an offset, and jumps to an address or another routine.
- `update_board_state_and_counters`: Updates a table or global variable with a constant or a calculated difference, modifies a pointer or index in a structure, and calls another routine to update or clear certain table entries or counters.
- `update_table_and_adjust_counters`: Calculates a new index, updates a table with a constant value, and clears or sets values in a table or global variable.
- `modify_multiple_table_values`: Performs several operations including zeroing or incrementing/decrementing values in a table or a global variable based on certain conditions and pointer-based lookups.
- `update_game_state_indices_and_indices_2`: Updates a table of game state indices and counters, modifies a global counter, and calls other routines to update or decrement counters or flags based on certain conditions or table values.
- `update_game_state_indices_2`: Checks if a value in a table at a certain offset is equal to five and if certain bits in a global variable are set; if so, it updates that table entry to seven, otherwise it updates the table entry to one and modifies other counters or pointers in the structure.
- `update_game_state_indices`: Updates a table or flag or pointer based on a structure member, modifies a counter or flag, and calls another routine with parameters from a structure, and conditionally modifies several global counters or updates a table-based counter or flag, and calls a routine with parameters from a structure, and conditionally modifies several global counters.
- `update_game_state_values_2_2`: Calculates a new index or pointer, clamps it within a certain range, and updates a table or flag based on a structure member.
- `update_game_state_values_2`: Updates a table-based counter or flag, calls a routine with parameters from a structure, and conditionally modifies several global counters or table values based on a bit test and a comparison, then calls another routine multiple times to increment or decrement values in a table or global memory.
- `update_game_state_values`: Updates several global or table-based values, including incrementing or decrementing certain counters and setting a global variable to a value from a local variable.
- `initialize_d0_register`: Tests a longword at a negative offset from a4, and if zero, loads the value 8 into d0 and then jumps to another routine, otherwise it returns immediately.
- `check_bit_and_unlk`: Clears a 32-bit value at a certain offset in a structure, checks a bit in a global variable, and if bit 7 is not set, unlinks a5 and returns.
- `update_string_buffer_and_loop`: Initializes a string buffer with a character, then enters a loop that calculates string offsets or values from a table, updates a character or value in a structure, and calls other routines to write or update these values.
- `clear_local_variable_2`: Clears a 32-bit value at an offset in a stack-based pointer and then branches to another location.
- `update_string_buffer_and_counters`: Iterates through a loop to update a global counter and a string buffer by reading a byte from a table, decrementing it, and writing a character or value back to a buffer, then executes a second loop that calculates an offset from a table and calls another routine to write characters to a string buffer.
- `modify_global_values`: Decrements two global variables, sets a third global variable to a constant, and then branches or moves a value from a global variable into a register.
- `format_string_and_update_counters`: Iterates through a range of values to update a string buffer with ASCII characters and a global counter, while also calculating and writing offset-based values from a table into a global structure and calling other routines to update string buffers and counters.
- `iterate_and_update_values`: Loops through a range, calculating an offset-based value from a table, updating a global variable, and calling two other routines while updating a character or integer value.
- `calculate_difference_and_clamp_2`: Calculates the difference between two 16-bit values retrieved from a table, stores the result in a global variable, and then returns either the absolute difference or a value determined by a function call and a conditional check.
- `calculate_difference_and_clamp`: Calculates the difference between two 16-bit values retrieved from a table, stores the result in a global variable, and then returns either that difference or a value from a global variable depending on whether the difference is negative or smaller than another global variable.
- `get_stack_value_and_return`: Loads a longword from a location relative to a5 into d0 and returns.
- `find_max_and_swap`: Sets up a stack frame, compares a value from a structure against a constant, and if a certain condition is met, iterates through a table to find a maximum value and swaps elements in a table.
- `load_table_index`: Loads a 16-bit value from a table using an index, extends it to a 32-bit value, and sets up a range for a subsequent loop.
- `update_table_with_max_value`: Iterates through a range, updating a global variable with the maximum 16-bit value found in a table, and then copies several 32-bit or 16-bit values from a table to a destination buffer while incrementing a counter.
- `restore_registers_and_return`: Restores d4-d6 from the stack, unlocks a5, and returns.
- `decrement_global_values`: Calls a routine three times with certain offsets and decrements three global integers, ensuring one does not drop below zero, before calling another routine.
- `call_modify_table_logic`: Clears several registers, pushes two pointers onto the stack, calls a routine, and then loads a value from a global variable into d4 before branching.
- `update_table_with_offset_3`: Calculates an offset based on a table value, increments a value at that offset in a table, and then pushes a value onto the stack before branching.
- `decrement_table_entry`: Decrements a 16-bit value at a calculated offset in a table and then pushes a value onto the stack before branching.
- `modify_table_entry_logic`: Decrements a register and checks it against a table value, and if certain conditions are met, updates a value in a table or modifies a bitmask in a structure before branching or returning.
- `add_to_table_entry`: Adds a constant value to a 16-bit word at an offset in a table and then branches to another location.
- `modify_table_entry`: If a certain value equals two, this routine clears a byte in a table and performs a bitwise OR on another or a nearby value in that table.
- `initialize_and_update_tables`: Clears several arrays in a structure, then iterates through a range of indices to update counters and pointers in a table based on values retrieved from another routine and a secondary table.

The function list continues in: g1346-recovery-functions-2.md.
