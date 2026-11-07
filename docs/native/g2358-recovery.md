# Game 2358 (spy vs spy - the island caper (1989)(wicked software)[cr vf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02358). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902144 top-level bytes in 38 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 25 of 25 units identical (bytes and relocations), 124 raw-word lines |
| Functions named and described | pass, 925 of 925 named (287 author symbols), 925 of 925 described (59 by rule, 401 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1287 of 1287 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 22599 of 22643 instructions identical in start and length, 3 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 182619 |
| code | 85636 |
| data | 178131 |
| padding | 6734 |
| unused | 449024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| setmap | 13 | 627 | 33 |
| delete | 2 | 117 | 2 |
| addbuffers | 2 | 117 | 2 |
| path | 2 | 117 | 2 |
| ex | 2 | 117 | 2 |
| info | 2 | 117 | 2 |
| dir | 2 | 117 | 2 |
| copy | 2 | 117 | 2 |
| makedir | 2 | 117 | 2 |
| list | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| genim2 | 1 | 4256 | 221 |
| spy2 | 3 | 9563 | 227 |
| begin | 4 | 1267 | 71 |
| port-handler | 1 | 306 | 1 |
| disk-validator | 1 | 386 | 1 |
| ram-handler | 2 | 10 | 1 |
| serial.device | 4 | 976 | 83 |
| gb | 1 | 12 | 1 |
| parallel.device | 3 | 322 | 26 |
| printer.device | 19 | 3272 | 218 |
| arp.library | 3 | 85 | 7 |
| Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF].adf:vf.obj | 2 | 96 | 4 |
| Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF].adf:bootblock | 1 | 57 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 238 | 9 |

## Cases handled

- runtime segment an address: 181 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `format_complex_output` (genim2): Performs a sequence of operations including updating a status byte, traversing a list, and executing a loop based on a value in a structure. Kind: sabotage path. Pass path: the routine executes its logic and returns normally, but the 'line malformed' error suggests the routine is part of a parser/loader that triggers a failure state (sabotage) when specific data patterns (likely related to the disk format) are.
- `parse_command_line_options` (genim2): Parses the command line into option letters and file names, handling options P, S, L, C, D and X, and builds a listing file name by replacing the extension. Kind: command line validation/sabotage path. Pass path: the branch where the loop continues or returns the parsed command line (uVar4).
- `SHOOT` (begin): Starts a shot sound on the fourth audio channel by pointing it at the noise sample, setting its length and period to 2000, clearing its envelope state and attaching the shoot envelope routine. Kind: disk hardware/timing check. Pass path: the code continues execution normally after the routine returns.
- `EXPLODE` (begin): Starts an explosion sound on the fourth audio channel by pointing it at the noise sample, setting its length and period to 3000, clearing its envelope state and attaching the explode envelope routine. Kind: sabotage path. Pass path: the routine is not called if the check passes.
- `DIVE` (begin): Starts a dive sound on the third audio channel using the triangle waveform, with its period set to 2130 and the dive envelope routine attached. Kind: disk hardware/sync check. Pass path: the branch an address (the bne.w an address is NOT taken).

## Functions


### setmap

- `parse_command_line_arguments`: Parses a command-line string into individual tokens by handling spaces, tabs, and quoted substrings.
- `invoke_conditional_pointers`: Saves a parameter to a local stack location, then conditionally invokes one or two function pointers based on the values of global variables.
- `invoke_two_function_pointers`: The routine invokes two function pointers located at fixed negative offsets from a register and then returns.
- `invoke_conditional_pointers_2`: Conditionally invokes several function pointers based on the values of global variables and returns a value from the stack.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `propagate_map_update_calls`: Calls a helper with a stored value and, when a flag is set, the address of a global, then calls the helper again with other stored values.
- `store_pair_and_invoke_helpers`: Saves two parameters into global variables, then calls a helper with a fixed argument list that includes a global address and the constants 440 and 60, and finally calls another helper with the value 20.
- `initialize_keymap_and_console`: Resolves a keymap name from icon tool types, command-line arguments, or a default value, then converts the name to lowercase.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_head`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `call_find_name`: Library stub: loads its arguments and the exec library base, calls FindName and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_alloc_signal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_open_resource`: Library stub: loads its arguments and the exec library base, calls OpenResource and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `pass_params_to_subroutine`: Calls another routine while passing a global variable, a parameter, and the address of an eight-byte stack slot.
- `call_library_function_minus`: Calls a library function through a base pointer held in a global variable, using a fixed offset of minus 348 into its jump table, and returns with no result.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### delete

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: Copies a fixed number of words from a source location to a destination pointer if the destination is not null.

### addbuffers

- `initialize_data_buffers`: Calculates the maximum size required for a linked list of data structures and allocates a contiguous block of memory.
- `copy_stack_data`: Copies a sequence of words from a source table to a destination stack location.

### path

- `initialize_dynamic_data_structure_2`: Calculates the maximum value within a linked list of data blocks and allocates a new memory region.
- `copy_data_block_3`: Copies a fixed number of longwords from a source address to a destination address if the destination pointer is not null.

### ex

- `initialize_dynamic_data_structure_3`: Traverses a linked list to find a maximum value and then allocates or expands a memory block.
- `copy_data_block_4`: Transfers a sequence of sixteen bytes from a source location to a destination location.

### info

- `initialize_dynamic_data_structure_4`: Calculates the maximum value within a linked list of data blocks and allocates a new structure.
- `copy_data_block_5`: Performs a block copy of data from a source to a destination pointer.

### dir

- `initialize_dynamic_data_structure_5`: Calculates the maximum value within a linked list of data blocks and initializes a new structure by populating it with a sequence of incrementing values.
- `copy_data_block_6`: Moves a block of data by iterating through a source and destination.

### copy

- `initialize_dynamic_data_structure_6`: Calculates the maximum value within a linked list and allocates a memory block to store a new data structure.
- `copy_data_block_7`: Copies sixteen bytes of data from a source to a destination.

### makedir

- `allocate_and_initialize_data_structure`: Iterates through a linked list to find a maximum value and then allocates a memory block to store a structured data set.
- `copy_data_block_8`: Copies a set of data words from one memory location to another.

### list

- `initialize_dynamic_data_structure_7`: Traverses a linked list to determine a maximum value and then allocates or initializes a memory block with a sequence of incrementing values.
- `copy_data_block_9`: Copies a block of data using a loop.

### cd

- `initialize_dynamic_data_structure`: Traverses a linked list to find a maximum value and then populates a new data block with a sequence of incrementing words.
- `copy_data_block_2`: Transfers a sequence of words from a table to a destination buffer.

### genim2

- `initialize_text_buffer_state`: Clears and initializes a large data structure by copying a global block, zeroing out specific memory ranges, and resetting various status flags and pointers.
- `initialize_object_state_and_copy_string`: Clears and sets various fields within a large data structure using offsets from a base pointer, including setting specific byte values.
- `update_complex_structure`: Modifies several fields within a structure, including status bytes and list pointers, based on specific conditional checks.
- `reset_structure_field`: Clears a four-byte value at a specific offset within a structure pointed to by a register.
- `accumulate_structure_value`: Adds the value of one structure field to another field within the same structure.
- `increment_conditional_values`: If the first byte of a pointer is non-zero, it increments the value at that pointer and also increments the value at an offset of eight bytes from that pointer.
- `process_conditional_callbacks_2`: Checks if values at specific offsets within a structure are non-zero or match a specific character, then calls other routines accordingly.
- `process_conditional_callbacks`: If a value at a pointer is non-zero, it calls another routine; it then clears a value at an offset of eight bytes and calls another routine if a value at an offset of four bytes is non-zero.
- `fetch_linked_list_string`: Traverses a linked list of data structures to retrieve a pointer to a character string.
- `process_text_buffer_input`: Parses and manages a text buffer by searching for newline characters and updating internal pointers and line counters.
- `adjust_status_by_character`: Increments or decrements a byte in a structure based on whether a character is a plus or minus sign, and updates a flag byte.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 10 into d1, then returns.
- `decrement_status_byte`: Decrements a byte within a structure and sets a flag byte based on whether the new value is non-negative.
- `update_status_flags`: Increments a byte at a specific offset within a structure and updates a byte at another offset based on whether the new value is zero.
- `clear_status_fields`: Checks several conditions within a structure to select a byte, then clears three specific byte offsets within that structure.
- `parse_command_string_2`: Iterates through a character buffer to identify command tokens, handling delimiters such as colons, spaces, tabs, and equal signs.
- `copy_memory_block_4`: Copies a block of memory from a source to a destination based on a length byte.
- `store_offset_value`: Stores the sum of a register and a constant into a specific offset within a structure.
- `process_command_token`: Validates a command string against a set of criteria and, if successful, copies the string data into a structured buffer while updating internal counters and pointers.
- `process_command_string_buffer`: Parses a command string by validating pointers and copying character data into a structured buffer.
- `copy_data_to_structure`: If a value in a structure is above a threshold, it initializes a new structure with various fields and copies a string from a source pointer into it.
- `adjust_structure_values`: Modifies a longword and two shortwords within a structure using a value from a register and the result of another routine.
- `push_event_context_to_stack`: Saves a series of registers and constants into a memory block pointed to by a global variable.
- `save_context_and_update_list`: Saves several registers to a memory location, updates a status byte and a counter, and traverses a list to insert a value.
- `process_event_stack_recursively`: Iterates through a nested event table by pushing command data onto a stack and recursively processing entries.
- `process_recursive_event_table`: Iterates through a table of event data using a recursive structure to process command packets.
- `enqueue_event_data`: Stores a set of parameters including a value, a type, and a pointer into a structured queue or list.
- `process_event_queue_2`: Iterates through a linked list of event structures, extracting data based on event type and appending processed results to a secondary buffer.
- `dispatch_event_commands`: Iterates through a linked list of event structures and processes them based on their type.
- `process_event_queue`: Iterates through a linked list of event structures, comparing values and updating pointers.
- `enqueue_command_arguments`: Stores a pair of integer and unsigned integer values into a circular command buffer.
- `enqueue_command_packet`: Stores a sequence of register values, pointers, and constants into a structured data block within a memory buffer.
- `enqueue_command_parameters`: Stores a set of command arguments, including integer values and pointers, into a circular buffer or queue.
- `recursively_decrement_table_entries`: Recursively calls itself while a value at a table offset is non-zero, then decrements two specific table entries by two and eight respectively.
- `decrement_table_values`: Decrements two values stored at specific offsets within a table by two and eight respectively.
- `save_context_to_list`: Saves a set of registers into a memory block and then inserts that block into a linked list structure.
- `manage_context_stack`: Pops a data block from a linked list and executes a function pointer retrieved from that block.
- `tiny_movea_rts`: Small routine of 2 instructions that copies a4 to a0, then returns.
- `parse_command_string_3`: Parses a string of characters to identify specific command tokens and delimiters.
- `parse_and_copy_token`: Searches a memory buffer for a specific delimiter and copies the resulting substring to a new destination.
- `empty_routine`: This routine is empty and performs no operations.
- `search_and_copy_string`: Searches for a specific byte in a memory block and, if found, copies a sequence of bytes to a new location using a calculated length.
- `execute_indirect_call`: Performs an indirect call to a location calculated by subtracting a byte from a constant.
- `check_identifier_string`: Compares a 32-bit value against specific constant patterns and returns zero if certain string-based identifiers or flags are matched.
- `evaluate_conditional_logic_2`: Performs complex conditional checks on bytes within structures and a global variable to determine control flow.
- `evaluate_conditional_logic`: Performs complex conditional checks on bytes within structures and a global variable.
- `check_byte_condition`: Checks a byte at a specific offset within a structure and performs a recursive call if the byte does not match certain values or if a bitwise flag is set.
- `append_buffer_data`: Writes a two-byte sequence and a zero value into a buffer pointed to by a global variable.
- `nop_padding_sequence`: This routine is empty and consists only of instructions that perform no operation.
- `process_conditional_data`: Compares bytes from a structure against specific values and increments a global counter if certain conditions are met.
- `evaluate_byte_condition`: Checks a bit within a byte at a specific offset and performs conditional comparisons.
- `append_structure_data`: Appends a two-byte sequence, a byte from a structure, and a zero value into a buffer pointed to by a structure member.
- `copy_memory_block_5`: Copies a block of data from one memory location to another and performs conditional checks on structure members.
- `execute_empty_call`: This routine performs no operations and immediately returns to its caller.
- `call_one_helper_and_return`: Calls one helper routine and returns; part of an unreached gap decoded as code.
- `snapshot_event_context`: Saves the current CPU register state and several constants into a memory block allocated via a pointer found in a structure.
- `log_execution_trace`: Saves the current CPU register state and local data into a memory block before executing a sequence of subsequent routine calls.
- `store_register_state_to_structure`: Stores a byte and a longword from registers into a structure using an offset from a pointer held in a register.
- `process_event_markers_3`: Compares a value against specific four-byte markers to determine whether to decrement a counter, update a pointer to the next node, or adjust state.
- `process_event_markers_2`: Compares a value against specific four-byte markers to determine whether to decrement a counter, update a pointer to the next node, or adjust internal state variables.
- `process_event_markers`: Compares a value against specific four-byte markers to determine whether to decrement a counter, update a pointer to the next node, or adjust internal state.
- `process_linked_list_markers`: Compares a value against specific four-byte markers to determine whether to decrement a counter, update a pointer to the next node, or adjust list offsets.
- `traverse_linked_list_nodes`: Iterates through a linked list of structures by comparing a target value against a field in each node.
- `manage_linked_list_nodes`: Iterates through a linked list of structures to find a specific marker and updates a counter.
- `parse_string_table`: Parses a string to extract values by using characters as offsets into a table and stores the resulting 32-bit values into a destination buffer.
- `fetch_table_value`: Reads characters from a string and uses them as offsets into a table to retrieve and store 32-bit values into a destination buffer.
- `return_extended_value`: Returns a 32-bit value from a table offset concatenated with a 32-bit constant of all ones.
- `wrap_subroutine_call_with_value`: Stores a constant value into a structure offset, calls another routine, and then stores the returned value into the subsequent offset of the same structure.
- `store_byte_to_pointer`: Stores a single byte from a register into a memory location pointed to by a register.
- `update_linked_list_node_2`: Updates a linked list by decrementing a counter and adjusting pointer offsets within a structure.
- `update_state_and_traverse_list`: Modifies several bytes in a structure, increments a value, and conditionally traverses a list of pointers to update memory contents.
- `process_complex_structure`: Performs multiple operations on a structure, including clearing bytes, calling other routines, and executing a function pointer retrieved from a table.
- `advance_string_pointer`: Searches for a newline character in a string and updates two pointers to track the current position and the start of the line.
- `empty_routine_2`: This routine is empty and performs no operations before returning.
- `initialize_and_process_command_table`: Initializes a memory range with a specific value and iterates through a table to update short integer entries.
- `parse_command_string`: Iterates through a character buffer to identify specific command characters and associated parameters.
- `store_word_to_pointer`: Stores a 16-bit value from a register into a memory location pointed to by another register.
- `call_and_clear_pointer`: Copies a function pointer from a table, clears the original entry in the table, and then calls that function.
- `process_event_queue_and_state`: Checks a table for pending function pointers to execute or pending integer values to add to running totals.
- `update_running_totals`: Calculates a difference between two values, updates several running totals in a table, and performs conditional logic involving a loop through a list of shorts.
- `save_context_and_update_list_2`: Saves a large block of register data to a memory location and updates a linked list of entries.
- `find_period_in_string`: Searches for a period character within a string and updates a length field based on its position.
- `parse_string_token`: Searches through a memory buffer for a specific delimiter character to extract a substring.
- `parse_string_data`: Parses a source string by looking for delimiters or specific characters and stores the resulting characters into a destination buffer.
- `verify_multiple_pointers`: Checks a series of memory offsets to ensure they contain non-zero values, returning a specific constant if all checks pass.
- `search_linked_list`: Traverses a linked list of structures to find an entry where a specific byte value matches a target value.
- `manage_object_lifecycle_2`: Initializes a structure field, performs a series of conditional routine calls, and manages pointer updates within a linked list or object structure.
- `manage_object_lifecycle`: Initializes a structure field, performs a series of conditional routine calls, and manages pointer updates within a linked list or object structure.
- `set_range_values`: Stores two integer values into a structure at specific offsets based on an input value.
- `process_amigados_hunk_data`: Parses and manages memory structures by comparing values against AmigaDOS hunk type codes.
- `execute_complex_logic`: Performs a series of conditional checks, updates a counter, and executes multiple subsequent routines based on various global flags.
- `check_status_flags`: Checks two specific bytes in a global structure and returns if certain conditions are not met.
- `check_status_and_text`: Evaluates specific byte and word values within a structure to return constant integer values.
- `reset_memory_pointers`: Sets a word at a specific offset in a global structure to zero and updates a pointer to point to that location.
- `append_to_buffer`: Writes a specific two-byte sequence and a zero value into a buffer pointed to by a global variable.
- `append_buffer_data_2`: Writes a two-byte sequence, a byte from a structure, and a zero value into a buffer pointed to by a structure member.
- `update_state_and_dispatch_callback`: Sets specific bytes in a structure, performs conditional checks on various memory offsets, and executes a function pointer retrieved from a table.
- `execute_conditional_logic`: Checks a zero flag and, if not set, performs a series of structure updates and function calls including a call via a pointer offset.
- `update_buffer_state`: Modifies bytes within a structure and iterates through a linked list of short integers to update values based on a terminator.
- `save_register_context`: Saves a large block of register values and local data into a memory location and updates a linked list of entries.
- `format_complex_output`: Performs a sequence of operations including updating a status byte, traversing a list, and executing a loop based on a value in a structure.
- `manage_string_buffer_nodes`: Manages a linked list of string buffers by allocating or unlinking nodes, updating buffer lengths, and appending newline characters.
- `append_newline_conditionally`: Appends a newline character to a buffer if certain conditions regarding the buffer content and a global flag are met.
- `append_newline_to_buffer`: Appends a newline character to a buffer and updates a pointer and a length counter if a specific condition is met.
- `clear_nested_pointers`: Traverses a pointer structure and clears values if they meet specific non-zero or character-based criteria.
- `clear_global_pointer`: Checks if a pointer at a specific offset in a structure is non-zero and sets it to zero if so.
- `process_conditional_callback`: Checks a pointer for non-zero values and executes a callback routine if a character matches a specific value.
- `execute_conditional_callbacks`: Checks a pointer in a structure and executes a sequence of potentially nested function calls based on the contents of that pointer.
- `execute_conditional_calls`: Checks if a pointer in a register is non-zero before calling another routine, then performs a second conditional call based on a value at an offset from a pointer.
- `execute_conditional_callbacks_3`: Performs a sequence of conditional function calls by checking a pointer in a structure and evaluating specific bits or values.
- `test_bit_and_call`: Checks if a specific bit is set in a byte at an offset from a pointer and calls another routine if it is.
- `process_nested_pointers`: Traverses a nested pointer structure to execute a series of conditional routines based on non-zero values.
- `process_conditional_calls`: Checks if a pointer is non-zero before executing a routine and subsequently performs another conditional call based on a value at an offset.
- `test_bit_and_call_2`: Checks if a different specific bit is set in a byte at an offset from a pointer and calls another routine if it is.
- `unlink_list_node`: Removes a node from a linked list by updating pointers and calculating a new offset, or initializes a new node if a specific byte is zero.
- `update_data_node`: Updates a data structure by calculating an offset, checking a size value, and potentially reinitializing a pointer or updating a status value.
- `initialize_node_structure`: Unlinks a node from a list and initializes its members with specific values.
- `update_data_slot`: Iterates through a data structure to update a status value, potentially resetting a global slot to all-ones if a specific condition is met.
- `update_slot_pointer`: Checks a pointer in a global slot and either updates a value from that pointer or sets the slot to all-ones.
- `set_slot_to_minus_one`: Stores the all-ones longword into a fixed slot of the global data block and returns zero.
- `emit_chars_after_skipping_strings`: Skips a counted number of NUL-terminated strings in a global buffer, then calls an unnamed routine once for each character of the following string.
- `skip_counted_strings_from_pointer`: Skips a counted number of NUL-terminated strings starting from a pointer register, then calls an unnamed routine once per character of the next string.
- `append_newline_and_flush_buffer`: When the mode byte is clear, it waits for a counter to leave a fixed value, appends a newline to a text buffer and calls a library routine through a vector.
- `call_twice_return_constant_32`: Calls an unnamed routine twice and returns the constant 32 in the low longword, with the first call's result in the high longword.
- `tiny_moveq`: Small routine of 1 instruction that loads 32 into d1.
- `pass_registers_through_call`: Calls an unnamed routine and then returns the data registers D0 and D1 unchanged as one combined 64-bit result.
- `lookup_table_nibble_three_calls`: Makes three unnamed routine calls, reads a byte from a global table at the low four bits of the data register, then makes one more call.
- `lookup_table_nibble_two_calls`: Makes one unnamed call, a second call that yields a result, reads a byte from a global table at the low four bits of the data register, then makes a final call.
- `lookup_table_nibble_one_call`: Calls an unnamed routine, reads a byte from a global table at the low four bits of the data register, then calls another routine.
- `fetch_table_nibble_then_call`: Reads a byte from a global table at the low four bits of the data register and then calls an unnamed routine.
- `emit_nibbles_skipping_leading_zeros`: Walks the eight four-bit groups of a longword from the top, sending each non-leading group to a jump-table target.
- `format_unsigned_decimal_digits`: Converts an unsigned value to decimal by repeated subtraction against a table of descending powers of ten, calling unnamed routines for each digit.
- `emit_decimal_digits_jumptable`: Performs the same subtraction-based decimal conversion as its neighbour, but the output step is an unrecovered jump-table call and the routine returns nothing.
- `copy_word_to_indexed_slot`: Copies one word from a pointer register into a slot addressed by a base register plus a scaled index register.
- `process_linked_record_fields`: Follows a two-level pointer stored in the global block and, when the record is valid, calls unnamed routines depending on its first longword, the byte at offset 13 equalling 9, and its second longword.
- `emit_space_or_period_field`: When the field byte equals one, it writes a period into the text buffer if the buffer mode is clear, or waits on a column counter before advancing it.
- `call_wrapped_value_return_32`: Calls an unnamed routine before and after a middle call whose result is kept, then returns that result in the high longword with the constant 32 in the low longword.
- `repeat_call_per_count_field`: Calls an unnamed routine once, then repeats the call as many times as a count byte in a pointer block when a flag byte is set, and finally makes three more calls.
- `fill_buffer_with_ff_bytes`: When the pointer argument is set and its first longword is nonzero, it writes the byte 127 followed by 128 bytes of value 255 into a fixed buffer, then calls an unnamed routine.
- `match_bounded_byte_sequence`: Under a flag and a threshold check, it compares bytes from two pointer blocks until a mismatch or a count runs out, then passes the second pointer to an unnamed routine when that pointer is set.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `release_tracked_block_size`: If a tracked block pointer is set, it clears that pointer, subtracts the size word stored just before the block from a running total, and calls an unnamed routine through a vector.
- `reset_output_and_return_status`: Unlinks a node from a list rooted in a fixed slot and returns its result.
- `preset_word_and_store_result`: Stores the fixed word value 10240 in a fixed slot, calls an unnamed routine, and saves its longword result in another slot.
- `subtract_head_block_size_total`: If the list head pointer is set, it calls an unnamed routine when the block's first longword is nonzero, subtracts the size word stored before the block from a running total, and calls a vector.
- `subtract_block_size_from_total`: Given a block pointer, it calls an unnamed routine when the block's first longword is nonzero, subtracts the size word stored before the block from a running total, and then calls a vector.
- `run_three_calls_if_pending`: When the pending longword is nonzero, it makes three unnamed calls.
- `clear_pending_and_notify`: When the pending longword is nonzero, it clears it and, if the old value differs from a stored reference longword, calls a library routine through a vector.
- `initialise_text_output_state`: Clears the output flags and counters, points the text buffer pointers at a fixed buffer block, sets limit values of 132 and 60 and a counter of 8, then makes one unnamed call.
- `put_char_into_text_buffer`: Stores a character in a text buffer when there is room and returns at once.
- `reset_text_buffer_pointer`: When the text buffer write pointer is not at the start of its buffer, it resets the pointer to the start and calls an unnamed routine.
- `tick_text_output_state`: Advances a counter, makes several unnamed calls including a loop whose count comes from a record's byte, then either appends a newline and flushes through a library vector or advances a column counter and calls the library routine at its target.
- `advance_column_counter_and_flush`: Clears a sub-counter and advances a counter.
- `write_char_to_line_buffer`: Writes one character into a line buffer in one mode, waiting while the count is at its limit and flushing through a library vector at each newline.
- `emit_string_bytes`: Calls an unnamed routine once for every byte of the zero-terminated string addressed by the address register, stopping at the terminator.
- `format_status_line_fields`: Prints a status line from bit-selected fields: a decimal number from a word, a counted value, a run of up to nine repeated output calls, and a text string that stops at a newline and expands tab characters through repeated output calls.
- `stub_return_only`: Does nothing and returns, since the decoded body is only a return.
- `stub_mode_word_check`: Compares a fixed mode word with three and returns in both cases, so no action results.
- `write_string_twice_with_comma`: Copies the zero-terminated string at one pointer into a destination, appends a comma, then appends the same string again.
- `walk_hunk_list_aligning_sizes`: Walks a linked list of loaded hunks, rounding each recorded size up to a four-byte multiple and storing the aligned value with its base.
- `check_byte_zero`: Checks if a byte at a specific offset from a pointer is zero.
- `check_byte_zero_2`: Checks if a byte at a specific offset from a global variable is zero.
- `stub_immediate_return`: Returns immediately with no work.
- `register_format_slot_once`: In one mode it adds a value to a running total and returns.
- `stub_mode_check_return`: Compares a mode byte with a fixed value and returns in both outcomes, so no action results.
- `copy_bytes_add_running_total`: Adds the count to a running total slot, then copies that many bytes from the source pointer to the destination pointer, one byte at a time.
- `emit_space_or_period`: In one mode it passes the space code 32 to an unnamed routine and returns that call's result.
- `store_slot_if_empty`: When a slot at a fixed offset is zero, it calls an unnamed routine and stores the data register value into that slot, returning the call's result.
- `reset_counters_and_save_total`: Clears a byte, increments two counters, and calls unnamed routines when one flag is set and a second flag is nonzero.
- `reset_count_longword`: Sets a fixed longword counter to zero and returns.
- `apply_hunk_relocation_entries`: When the flag is set, it walks the hunk list and adds each hunk's base address to the longwords named by its relocation table.
- `release_slot_via_library`: When the longword in a fixed slot is nonzero, it calls the library function at a vector offset of minus 36 from the library base, then clears the slot.
- `subtract_8966_until_zero`: Subtracts 8966 from a value repeatedly, setting it to zero when it would go below, and calls an unnamed routine on each pass until the value reaches zero.
- `find_name_index_in_table`: Measures a word in the text up to a space, newline or tab, then searches a table of length-prefixed entries for a match through a character translation table.
- `stub_empty_return`: Returns without doing anything.
- `copy_bytes_update_total_slot`: Adds the count to a running total slot and copies that many bytes from the source pointer to the destination pointer.
- `find_wrap_index_in_list`: Walks a list from a fixed head, incrementing a byte taken from each record, until the byte wraps to zero.
- `emit_padded_name_record`: When a record's kind byte and tag byte both match, writes its length and base address longwords, copies its name bytes, pads them to a four-byte boundary, and writes a terminating zero longword.
- `write_name_record_unchecked`: Writes a length longword taken from the data register and the base address, then copies the name bytes padded to a four-byte boundary and four zero bytes, without checking the record type.
- `store_matching_record_longword`: When a record's kind byte matches, calls an unnamed routine, repeats another call until a counter reaches four, then stores the record's longword at offset 8 into the output longword.
- `stub_bare_return`: Returns immediately with no action.
- `store_record_longword_conditionally`: When a record's kind and type bytes match and its flag bits pass a check on the mode word and flag byte, it waits for a counter to reach four and stores the record's longword at offset 8 into the output.
- `copy_padded_record_name`: Copies a record's name bytes to a destination, first making an unnamed call when a counter is below the padded name length, then zero-fills the tail to a four-byte boundary.
- `store_longword_after_wait`: Repeats an unnamed call until a counter reaches four, then stores the data register longword at the output address.
- `call_unless_counter_is_128`: Calls an unnamed routine unless a counter equals 128, then returns.
- `stub_plain_return`: Returns with no work.
- `call_three_routines_in_order`: Makes three unnamed routine calls in sequence and returns.
- `walk_chain_with_marker_kind`: When the zero condition flag is clear, it compares a kind byte with the marker value minus 127, passes the data register value to an unnamed routine, then repeats a call while the next link in the chain is nonzero.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `scan_until_minus_one_result`: When a field of a pointer block is nonzero, it calls an unnamed routine, repeats calls until the returned value is minus one, then makes one final call.
- `store_record_plus_ten`: Stores into a field of a record a pointer ten bytes past the record's own address, with the record pointer read from a block at a fixed offset.
- `set_block_field_plus_ten`: Stores a pointer ten bytes past the block's own address into the block's field at offset 6.
- `find_matching_record_index`: Searches a list of eight-byte records for one whose first two bytes match the given kind and second value, also checking a word field unless the kind is one.
- `store_padded_name_header`: Stores a value and a name of counted length into a fixed header block, pads the name to a four-byte boundary, and calls an unnamed routine.
- `append_string_copy_with_comma`: Copies the zero-terminated string at one pointer into a destination, appends a comma, then appends a second copy of the same string.
- `flush_line_buffer_via_library`: Calls the library routine at vector offset minus 48 to flush output, then calls an unnamed routine and resets the buffer count and write pointer.
- `flush_output_then_call`: Calls the library routine at vector offset minus 48, then calls an unnamed routine.
- `open_dos_and_init_state`: Opens dos.library through the system vector and returns an error code if that fails.
- `parse_command_line_options`: Parses the command line into option letters and file names, handling options P, S, L, C, D and X, and builds a listing file name by replacing the extension.
- `check_path_has_directory`: Scans a name string for its last slash or colon separator and returns zero when one is present, otherwise minus one.
- `flush_column_counter_when_ready`: When a mode byte is set and a counter is non-negative, it calls an unnamed routine, calls a library routine at vector offset minus 216, and resets two counters.
- `flush_counter_when_nonnegative`: When a counter is non-negative, it calls an unnamed routine, calls a library routine at vector offset minus 216, and resets two counters.
- `handle_break_signal_flag`: Polls task signals through a system vector and, when the break bit (bit 12) is set, ORs 127 into a status byte, issues a second signal call, and runs an unnamed routine if a stored pointer is set.
- `emit_exit_message_string`: Calls an unnamed routine, then emits the NUL-terminated message at a fixed global address one character at a time through another unnamed routine, the text including a prompt to press a key to exit.
- `call_two_library_functions`: Calls two library functions in sequence through the library base, passing the low word of the state pointer to the second call.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies 32 to (a3)+, then returns.
- `allocate_segment_block`: Allocates a block of the requested size plus eight bytes, records that size in the block's first longword, stores the block address shifted and plus one in an output longword, and clears its link field.
- `allocate_counted_block`: Allocates a block of the requested size plus four bytes through the system vector.
- `terminate_name_around_call`: Writes a zero byte just past a counted name, calls an unnamed routine, then writes a byte of value 11 at the same position.
- `scan_to_colon_or_end`: Advances through a string until it reaches a colon or the terminating zero byte, then returns.
- `copy_name_into_work_buffer`: Copies a name into a work buffer.
- `run_library_lookup_sequence`: Runs a library lookup sequence: a call at vector offset minus 84, then a call at minus 102 when that succeeds, a close at minus 90, a call at minus 30, and a final call at minus 36 when a state field is empty.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `normalise_library_result`: Calls the library routine at vector offset minus 42 and returns zero for a non-negative result, otherwise passing the negative result through.
- `stub_single_call_return`: Calls one unnamed routine and returns, with no other effect.
- `call_library_vector_minus_36`: Calls the library function at vector offset minus 36 from the library base and returns its result.
- `call_library_vector_minus_42`: Calls the library function at vector offset minus 42 from the library base and returns.
- `copy_name_and_check_library`: Copies a name into a fixed buffer, appending it after the existing path when the name starts with a period or is empty, and stops at a limit character.
- `flush_when_nonzero`: When the data register value is nonzero, it calls the library routine at vector offset minus 48.
- `call_library_vector_minus_66`: Calls the library routine at vector offset minus 66 and returns.
- `query_available_memory_if_clear`: When a flag byte is clear, it calls the system available-memory routine through a global pointer and returns.

### spy2

- `START_CODE`: Jump stub: passes control straight on to START.
- `DUMMY_SPRITE`: Busy-waits on the vertical beam position read from the VPOSR register until it passes the value in the data register, then repeats the wait against a derived value offset from it.
- `ATARI_COPY`: Reads four words at a time from the source and writes each one into one of four separate blocks spaced 4000 words apart, copying 4000 groups in total.
- `COLOUR_COPY`: Copies sixteen words from the source pointer into a global table, shifting each word left by one bit.
- `LOAD_FILE`: Opens dos.library through the system vector, then makes DOS calls to obtain a file handle, storing the library base and handle in fixed variables.
- `SETUP_SCREEN`: Points the screen pointers at the two screen buffers, copies a plane-pointer table into the second copper list, and writes display window, fetch, modulo, copper and DMA control registers, with the DMA enable bit set.
- `WHICH_PLANES`: Writes four bitplane addresses into a pointer table in steps of 8000 bytes, storing the low and high words of each address separately.
- `BLIT`: Waits for the blitter, then stores the blitter control, mask, pointer, modulo and size values into fixed variables.
- `WAIT_BLIT`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `SWAP_SCREEN`: Waits for the blitter and then enters an endless loop in the decompiled body.
- `CLEAR_SCREEN`: Zeroes 8000 longwords, writing four per loop pass over 2000 passes, starting at the address register.
- `MYINT2`: Keyboard interrupt handler: it updates the key matrix, runs a short countdown of 80 steps, then writes bits to the keyboard control register to complete the serial handshake.
- `ADD_MATRIX`: Converts the latched keyboard code into a matrix byte index and bit number, setting that bit when the key is released and clearing it when the key is pressed.
- `DOWN_STROKE`: Clears one bit in a byte addressed by a base register plus a data register offset, with the bit number taken from another register.
- `INIT_KEY`: Sets the four longwords of the keyboard matrix to all ones, so that no key is recorded as pressed.
- `INKEY`: Returns minus one when the key's bit in the keyboard matrix is clear, meaning the key is down, and zero otherwise.
- `SCAN_JOY`: Derives left and up flags for both joystick ports from the joystick data word, sets fire flags from the port button bits, and stores keyboard fallback directions by calling the key test.
- `HANDLE_MOUSE`: Runs the mouse movement update twice, then stores two button flags as all ones when the matching active-low button bits are clear.
- `DO_MOUSE`: Decrements two per-axis counters toward zero, sets each hold counter to 20 when the new position byte differs from the previous one in the matching direction, and stores the new position word.
- `LAST_MOUSE0`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `LAST_MOUSE1`: Allocates and clears two screen buffers and a third back buffer, storing their addresses in fixed variables.
- `KLEFT1`: Small routine of 1 instruction that merges 0 into d0.
- `ALLOCATE_MEMORY`: Allocates and clears two screen buffers and a back buffer, storing their addresses in fixed variables.
- `CLEARD0`: Zeroes 8000 longwords from the address held in the data register, one at a time.
- `FIND32K`: Calls the memory allocation vector and enters an endless loop if it returns null.
- `DECO_PIC`: Loads a file, copies sixteen header words from the back buffer into a table, then unpacks run-length data into four planes.
- `DOS_BASE`: Forbids task switching, starts the sound system, then plays sixteen sound steps, each waiting for a control bit to toggle, and permits again.
- `DOS_HANDLE`: Executes a function via a pointer, then enters a loop that plays a series of sounds while waiting for a bit in a global variable to toggle.
- `ONESCREEN`: Calls a routine via a pointer in a global variable, starts sounds, and enters a loop that triggers new sounds while waiting for a specific bit in a global variable to toggle.
- `TWOSCREEN`: Calls a routine via a pointer in a global variable and enters a loop that repeatedly triggers sound effects while polling a bit in a global variable.
- `SCREEN2`: Calls a routine via a pointer, starts sound playback, and enters a loop that repeatedly triggers new sounds while waiting for a specific bit in a global variable to toggle.
- `SCREEN1`: Triggers a sound effect, then repeatedly plays music notes while waiting for a status flag to flip on and off.
- `COPPER1`: Calls a routine via a pointer, starts sounds, and enters a loop that triggers new sounds while waiting for a specific bit in a global variable to toggle.
- `COPPER2`: Calls a function via a pointer, starts audio playback, and enters a loop that triggers a sound and waits for a specific bit in a global variable to toggle sixteen times.
- `COPPER_LIST2`: Executes a sequence of sound initializations and loops while waiting for a specific bit in a global variable to change.
- `E_KISS`: Executes a routine via a pointer, then enters a loop that triggers a sound and waits for a specific bit in a global variable to toggle before repeating sixteen times.
- `E_RUMBLE`: Calls a function via a pointer, initiates sound playback, and enters a loop that repeatedly triggers new sounds while waiting for a specific bit in a global variable to toggle.
- `TEST`: Triggers a sound effect and then repeatedly plays music notes while polling a global variable for a bit to toggle.
- `START_SOUNDS`: Fills an 8192-byte area with pseudo-random bytes from a shift-and-add generator, sets up four audio channel records with envelope and sample pointers, calls the music initialiser, stores a handler address in a vector, and writes control and interrupt enable registers.
- `GO_SOUND`: Fills a sound channel record with a pointer, two values taken from the data registers and a zero-cleared counter set, then marks the record active.
- `HIGH_PING`: Calls the sound channel setup routine and returns.
- `MEDIUM_PING`: Calls a routine to play a medium-frequency sound effect.
- `LOW_PING`: Calls a routine to play a specific low-frequency sound effect.
- `DROP_PING`: Performs an indirect call by loading a function pointer from a data structure and jumping to it, serving as a dispatch mechanism for a handler stored in a data table.
- `TAKE_PING`: Calls a routine to populate a two-part record with a pointer, two values, and initialized counters.
- `MEDIUM_BEEP`: Fills a record structure with a pointer, two values, and specific flags.
- `BANG`: Invokes a routine to initialize a channel record with a pointer, two values, and a flag set to one.
- `OTHER_SMACK`: Performs a call to a routine that populates a data structure with a pointer and two 16-bit values.
- `GRAVESOUND`: Requests the filling of a two-part record containing a pointer, two values, and zeroed counters.
- `BOOM`: Calls a routine to set up a record with a 32-bit pointer, two 16-bit values, and a flag.
- `BURY`: Plays a sound effect by calling a routine that configures a sound channel.
- `SPLASH`: Invokes the sound system and then returns without performing any other action.
- `RUMBLE`: Calls a sound routine and returns without performing any further operations.
- `FEET`: Calls a routine to play a sound effect.
- `KISS`: Calls the sound routine and then returns without doing anything else.
- `SUB`: Calls a routine and then returns without performing any other operation on its own.
- `HANDLE_SOUNDS`: Calls the sound request handler up to four times, repeating only while the previous attempt's condition holds.
- `HANDLE_REQUEST`: Checks four pending sound request words in order and, when one is positive, flags it, stores it as the current request, and calls the sound routine selected by its low byte.
- `NEW_SOUND`: Queues a sound request identifier in a four-slot list, marking an identifier already queued instead of duplicating it.
- `MYINT6`: Interrupt tick handler: it runs the mouse update, then when sound is enabled it counts down a music delay and, at zero, resets it to three and plays a music step.
- `CHANGE_ENVELOPE`: Advances one channel's volume envelope: when the segment counter reaches zero it loads the next segment from a sequence, otherwise it steps level and period by that segment's increments.
- `X8912`: Selects waveform pointer, length and frequency for two audio channels from wave and frequency tables, or from a noise buffer, according to status flags, and writes them to the audio registers.
- `PSGW`: Stores a byte into a music shadow table at the index held in the data register word plus two.
- `INITMUSC`: Sets a sound-chip register to the value 56, makes three shadow register writes, and runs two voice initialisations.
- `PLAYMUS`: Turns on channel zero, outputs its voice, then does the same for channel one, sending one music step to each of two voices.
- `VOICEISOF`: Silences a voice through a gate routine, then scans its sequence data for the next entry.
- `NEXTSEQ`: Steps a voice's sequence pointer forward three bytes and loads the next entry's volume and offset values.
- `VOICISON`: Ticks one voice: counts its duration down and, when it expires, steps its sequence and loads the next duration from a duration table.
- `DEGATEIT`: Clears the gate flag byte at offset 13 of the voice record.
- `CHECK2`: Chooses the mixer register value for the current channel from the voice's status bits, writes it through the sound chip shadow, and sets the voice's level byte to one of several fixed values.
- `CALCNOTE`: Calls the note modifier, takes the high nibble of its result, and subtracts a correction of one or two from the value, returning the adjusted value.
- `OUTVC1`: Sends voice one's register values to the sound chip by calling the shared output routine.
- `OUTVC2`: Sends voice two's register values to the sound chip by calling the shared output routine.
- `SNDOUT`: Makes three calls to the sound chip shadow write routine, sending three register values in turn.
- `NOTEMODS`: Adjusts a note's four-bit value by the low three bits of a second value, carrying into the octave bits, then stores the combined result in a status byte.
- `RESET`: Makes three shadow register writes in sequence and returns.
- `SAVE_INT2`: Allocates and clears screen buffers, initialises keys and sound, sets up the display, loads picture assets and installs interrupt handlers, then copies the back buffer into the screens.
- `TEMP_MUSIC`: Allocates memory, initializes hardware, copies data between two buffers, and enters a loop that manages screen modes and game logic.
- `REG7`: Performs memory allocation, hardware initialization, screen setup, and enters a main loop containing game logic.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `MUSINF_1`: The provided pseudo-code is identical to the main initialization routine.
- `clear_test_flag`: The routine is called via clear and move instructions, but the pseudo-code provided is identical to the main initialization routine.
- `MUSINF_3`: Sets up hardware interrupts, copies data from a buffer to a screen memory area, and enters a nested loop that repeatedly executes the main game logic and sound initialization until an abort condition is met.
- `MUSIC_DELAY`: Sets up sound parameters, configures screen buffers by copying data from a back buffer, and enters a loop that alternates between game logic and audio playback.
- `MASTER_ENABLE`: Allocates memory, initializes audio, and sets up display screens by copying data from a buffer to a secondary screen.
- `START`: Performs memory allocation, hardware initialization, screen setup, and enters a main loop that manages game state and screen modes.
- `READALL`: Clears the first screen and then runs a long fixed sequence of picture reads, grabs and buffer saves and stuffs.
- `MYINT3`: Increments a counter, clears a busy flag, and writes the value 112 to the interrupt request register.
- `GRAB`: Calls the buffer save routine four times in succession and returns.
- `GRABOBJ`: Small routine of 2 instructions that copies 20 to d3, then subtracts 1 from d3.
- `GRABOBJ1`: Calls the buffer stuffing routine repeatedly, with the count taken from a data register and decremented until it reaches minus one.
- `GRABLAND`: Calls the short buffer stuffing routine twenty times in succession and returns.
- `GRABMAP`: Calls the buffer save routine repeatedly, with the count taken from a data register and decremented until it reaches minus one.
- `PLAYINIT`: Resets round state for both players: menu and flash flags, a bullet count from three to six chosen at random, the map and terrain copied for the current level, energy set to 100, and the trap list and counters cleared.
- `PLAYINIT0`: Performs the same round reset as its neighbour, then writes a marker value at a level-dependent map position, reads two coordinate values through helper calls, and runs setup routines.
- `GETA4`: Small routine of 3 instructions that clears d1, then copies (a4)+ to d1, then returns.
- `XGET_STICK`: Copies the X and Y stick start values for the current level from two tables into the stick variables.
- `MY_VERY_OWN_STACK`: Clears the demo, autoplay and dead flags, then loops copying the back buffer to the second screen, reading triggers and joystick input and playing sounds, until either joystick trigger is pressed.
- `XSTICK1`: Shares the body of the neighbouring loop: clears the demo, autoplay and dead flags, then copies the back buffer to the second screen while reading triggers until a joystick trigger is pressed, clearing the demo flag on exit.
- `YSTICK1`: Copies a block of data from one memory location to another and processes joystick input.
- `PARMS`: Shares the body of the neighbouring loop, and its call list adds joystick movement, sound and the play-setup routines.
- `DO_GAME`: Runs the per-frame game loop for both players until the abort flag is set.
- `READPIX`: Decodes a picture file, copies its plane data and colour words, then writes two fixed values to hardware-facing globals.
- `STUFFIT`: Repeatedly picks a random terrain cell and, when the cell's code falls outside two excluded ranges and its neighbouring cells are empty, writes a fixed value there and counts it.
- `STUFFSPY`: Places both players at random map positions with their window values, rejecting starts that overlap, then copies colours, sets up the volcano values, redraws the buffer and sprites, and swaps screens.
- `SETUP_VOLCANO`: Stores three colour values, 1971, 1424 and 1397, in the colour variables and copies them to a second set of locations.
- `COPY_COLOURS`: Chooses one of two colour sets by the low bit of the data register and copies its three colour values to the colour registers and the top colour variable.
- `COL1A`: Calls the random value routine and returns its result.
- `COL2A`: Calls a routine to generate a random number.
- `COL3A`: Executes a routine that produces a random value.
- `RNDER`: Calls the shared random number routine once and discards the value, so callers only use it to step the random sequence forward.
- `RNDER2`: Calls the shared random number routine and passes its result back to the caller as the returned value.
- `GETRAND`: Repeatedly draws random values to build a candidate position in a table and retries until the byte at that position is zero, then returns a pointer to that empty entry.
- `SETXY`: Adds a fixed bias to its input and divides the sum by 96, storing the quotient in one word and the remainder in another, which turns a linear index into row and column values.
- `CLEAROUTCRAP`: Sets the ten-minute, one-minute, ten-second and one-second clock values and both spies' fuel counters to the maximum value, marking them as cleared.
- `L_TENMINS`: Checks whether the current map cell matches a spy's map position and, if so, computes plot coordinates from that spy's window offsets, then calls the sprite and hand drawing routines and returns a flag showing whether a spy was drawn.
- `L_ONEMINS`: Calculates screen coordinates and buffer offsets for two entities based on their map positions and altitude, then updates global sprite parameters and calls drawing routines.
- `L_TENSECS`: Calculates screen coordinates and buffer offsets for two possible entities based on their map positions and window visibility.
- `L_ONESECS`: Calculates screen coordinates and depth offsets for two entities based on their map positions and window visibility.
- `L_S1FUEL`: Uses the same spy-window placement and sprite drawing logic as the clock digit routines to draw the spy one fuel indicator when the map cell matches, then returns a flag showing whether it drew.
- `L_S2FUEL`: Uses the same spy-window placement and sprite drawing logic as the clock digit routines to draw the spy two fuel indicator when the map cell matches, then returns a flag showing whether it drew.
- `SLABX`: Tests each spy's map cell against the current position and, on a match, computes plot coordinates from that spy's window offsets, draws the sprite and hand overlay, and returns a flag showing whether a spy was drawn.
- `OLD_BUSY`: Checks if specific map coordinates match the current viewport and, if so, calculates screen-space coordinates and depth offsets to update sprite rendering parameters.
- `SAVE_INT3`: Calculates screen coordinates and buffer offsets based on map positions and altitude, then updates sprite and hand graphics for one or two active windows.
- `DRAWSPY`: Checks each spy's map cell against the current position in the split-screen windows and, on a match, computes plot coordinates, draws the spy sprite and then the hand overlay.
- `DRAWHANDS`: Draws the active spy's hand or carried item sprite, choosing the frame and horizontal offset from the spy's hand value and which frame buffer is in use.
- `DRAWBUTTONS`: Draws six button sprites by calling the single-button routine six times in sequence, with no other logic of its own.
- `ONEBUTTON`: Draws one button sprite at the given horizontal position, using the on image when the state value is nonzero and the off image when it is zero, with the drawing mode set before and restored after the sprite is drawn.
- `SWAPSCREEN`: Jump stub: passes control straight on to SWAP_SCREEN.
- `DRAWTIME`: Steps through the four clock digit values in order and, for each nonzero digit, advances to the next character step, calling a comparison helper before each step.
- `HLINE`: Calls the horizontal line drawing routine and returns, passing the screen and position state through unchanged.
- `METER`: Every eighth frame, restores one unit of energy to a living spy that is below full energy and outside a given timer range, then redraws each spy's energy bar as horizontal lines whose length reflects the remaining energy.
- `FLASH`: Computes the active spy's hand index and the frame parity, then calls the piece drawing routine three times.
- `DRAWPIECE`: Sets the blitter minterm to a fixed mode, draws a sprite through the restore-sprite routine, then resets the minterm to its default value.
- `DRAWSEL`: When the selection flag is set, calls the character output routine for four markers, drawing all four on one blink phase and omitting the one that matches the caller's index on the other.
- `COMPARETWICE`: Compares the incoming word with the stored word at the pointer and, if it matches, compares the following word too.
- `RSPRITER`: Draws a four-plane sprite into the screen buffer at a bit-shifted horizontal position.
- `RBACKER`: Restores a background block into four bitplanes.
- `or_shifted_rows_into_planes`: Shifts source words across four bitplanes and ORs them into the destination, zeroing the trailing words at the end of each row, and returns once the last row is done.
- `RSAVE_BUFF`: Copies a rectangular block of the four screen bitplanes into a save buffer, row by row, storing each plane's words consecutively.
- `RDRAW_BUFF`: Copies saved bitplane words from the buffer back into the matching rectangle of the screen, reversing the save routine.
- `RSTUFF_BUFF`: Writes a header with the block width and height, then packs a rectangle of the screen into the buffer: the combined OR of the four planes followed by each plane's words, with a zero word at the end of each row.
- `RSTUFF_BUFF_SHORT`: Writes a header with the block dimensions, then copies each of the four bitplanes of the rectangle into the buffer one after another without combining them.
- `RSPRITER_AM`: Programs the Amiga blitter to draw a sprite: it sets the blit masks, modulo, shift and pointer registers from the sprite data and its position, enables DMA, starts the blit and waits for each pass to finish.
- `SPRITER`: Calls the sprite drawing routine and returns, passing the screen and position state through unchanged.
- `SPRITER_AM`: Calls the blitter sprite drawing routine and returns, passing the screen and position state through unchanged.
- `BACKER`: Calls the background restore routine and returns, passing the screen and position state through unchanged.
- `X`: Sets one bit, chosen by the low three bits of a counter, in the byte just before the current pointer and then calls the next-character routine.
- `WIDTH`: Sets a specific bit in a byte located at a memory address derived from a pointer and a register.
- `BUFFER`: Performs a bitwise OR operation to set a bit within a byte at a calculated memory location.
- `SCREEN`: Uses a bit shift and OR operation to modify a single bit in a byte stored in memory.
- `MINTERMS`: Sets a bit in a byte at a memory location determined by a pointer and a data register.
- `PUTCHAR`: Calls the character drawing routine and returns, with no other logic of its own.
- `NEXTCHAR`: Copies an eight-row glyph from the font table into the four bitplanes at the current screen position.
- `CALCAD`: The body contains only a return, so calling it has no effect.
- `PLOTPOINT`: Clears the selected pixel bits in each of the four bitplanes at the target word using a clear mask, then sets them to the colour bits supplied for each plane.
- `LEFT_RIGHT`: Tests a leftward movement flag together with the low bit of an input value, but both outcomes return immediately, so the routine has no observable effect.
- `UP_DOWN`: Compares a global variable to zero and returns without performing any other actions.
- `DRAW_LINE`: Draws a line between two screen points with a Bresenham-style loop.
- `GO_LEFT`: Sets or clears a horizontal run of pixels across the four bitplanes.
- `HORLINE`: Draws a horizontal run of pixels across the four bitplanes between two column limits, setting or clearing each bit in turn.
- `POINT`: Combines the upper input word with the lower one and keeps the whole high word together with only the low four bits of the lower word, returning that masked value to the caller.
- `GET_RANDOM`: Advances the three random state words with additions, a subtraction and a rotation by a value taken from the low bits of an intermediate result, then stores the new state so the next call gives a different value.
- `READ_TRIGS`: Scans the joystick ports, then sets each player's trigger flag to all ones when that player's fire input is pressed and to zero otherwise.
- `MOVE`: Chooses the active spy's movement source: the joystick when it is player-controlled, or brain or random logic when it is computer-controlled, after demo timeout checks.
- `set_spy_direction_four`: Sets the spy direction variable to four and stores a constant through an address register.
- `set_spy_direction_three`: Sets the spy direction variable to three and stores a constant through an address register.
- `CHECKMEET`: Checks whether the two spies share a window row and are close enough in column to share a view.
- `CHECKMEETEND`: When the input counter is greater than one, sets both window buffer row markers to all ones, marking them as invalid.
- `SETTEMPS`: Copies the window origin and map position of the chosen spy into the shared working variables used by drawing.
- `DIRFIX`: Picks the frame buffer for the chosen spy from its direction value of one to four, then advances that spy's frame counter and wraps it to zero after four frames.
- `TEST_OPTIONS`: Decrements the feet flag when it is nonzero.
- `TEST_M`: When no key is pressed, sets a feet timer count and toggles the music switch.
- `JOYMOVE`: Scans the joystick and keyboard, waiting through a press and release sequence with short delays, and sets an abort flag when a key is pressed.
- `SETWINDOW`: Sets the shared window origin from window two when the window state is two, or when the window state is not one and the input is nonzero.
- `GETWINDOW`: Stores the shared window origin into window two when the window state is two, or when the window state is not one and the input is nonzero.
- `LITTLE_DELAY`: Counts a 16-bit value down from a fixed starting count to minus one as a short delay, then returns the input unchanged.
- `FEET_FLAG`: Draws one full game frame: the fuel gauges, flashing hands, selection marker, land and objects for each spy's window, the buttons and clock, the ropes and the energy meter.
- `DRAWMOVE`: Draws one movement frame with the same layering as its sibling routine: fuel gauges, hands, selection marker, land and objects per window, buttons, clock, ropes and energy meter.
- `DRAWLAND`: Selects the background buffer and upper-left origin for the chosen spy's window, then redraws slab tiles when the window has moved to a different cell.
- `DRAWLAND2_0`: Runs a blitter copy, redraws the slab tiles, then programs the blitter with the background position and the mask bits from its parameter.
- `DRAWLAND3_0`: Runs a blitter copy, redraws slab tiles, then sets up a second blitter pass from the background position and the mask bits in its parameter.
- `ONE_SLAB`: Programs the blitter to copy one slab tile into the background buffer, computing the source address from the slab index and the row offset.
- `DRAWSLAB`: Draws one slab tile, then draws it a second time when the slab index is zero or falls in certain ranges and the address is aligned to eight bytes.
- `DRAWBACK`: Copies a column of saved sprite data from the buffer into the background at the current column, writing two longwords per row for the buffer width and advancing a fixed row stride.
- `DRAWOBJ`: Draws the map objects for the chosen spy's window by scanning a grid of cells and drawing an item sprite for each non-empty cell, with special sprites for certain cell codes.
- `DRAWLAND5_2`: Draws the window cover sprite at a fixed horizontal position, with the vertical placement chosen by which window is in use.
- `DRAWROPES`: For each spy, if its rope line flag is set, draws the rope line and then clears the flag.
- `DRAWFUEL`: Records the spy's fuel value through the comparison helper and returns when it is zero.
- `CLEAR_WINDOW`: Clears a window-sized region of the four bitplanes with the blitter, running four fill passes that each start at a different plane offset and waiting for each pass to finish.
- `SEL_FLAG`: Runs the computer-controlled spy's decision step each frame, honouring the delay counters and reading the triggers.
- `BUSY`: Runs the computer-controlled spy's busy-state logic, honouring the delay counters and reading the triggers, then for an automatic spy chooses between trap placement, item search and movement.
- `BUSY4_A`: When the chosen spy holds an item, it either stuffs the item into a particular cell type or finds an empty map cell and marks it with a placement code.
- `DELETE_TRAP`: Looks up the given map cell in the 100-entry trap list.
- `ADD_TRAP`: Walks past a run of trap-type map cells, alternating the step direction, to reach a cell that is not a trap, then records that cell and its original value in the first free slot of the 100-entry trap list.
- `COUNTER1`: When no window is split, reads the joystick and, unless the spy is in swamp or depth, shows the map and draws marker sprites for each rocket, nose, midsection, tail and other item location that is set.
- `clear_flash_and_countdown_state`: Clears both spies' flash flags, then for the chosen spy moves the shared delay value into that spy's delay slot when it is set and clears its countdown state.
- `BUSY6_4`: Moves the shared delay value into the chosen spy's delay slot when it is set, clears that spy's countdown state and returns.
- `SETSAFE`: Sets the chosen spy's safe counter to four, using player two when the input is nonzero and player one otherwise.
- `KILLDOUBLE`: Stores the frame counter minus twenty as the time mark for the first spy when the input is zero, and for the second spy otherwise.
- `GETPOS`: Small routine of 2 instructions that shifts d1 right by 6, then shifts d2 right by 4.
- `GETPOS1`: Sets the horizontal plot position from the first index times sixteen plus a fixed offset, and sets the screen offset from the second index times eight plus a base value.
- `SETSTATE`: Sets the chosen spy's state counter to the value supplied in a register.
- `GETBTIME`: Tests whether the input is zero and returns in both cases, so it has no effect.
- `SHOWMAP`: Draws the level map sprite at a fixed horizontal position, with the row taken from a supplied register.
- `ABSD2`: This routine consists of four instructions that perform no functional operations before returning to the caller.
- `DRAWROPE`: Works out the start and end points of a spy's rope line from its plot position, its direction value and two linked positions.
- `CLEANTRAIL`: Sets all six entries of each spy's trail array to all ones, marking the trails as empty.
- `DELAY1`: Runs a timed wait sequence: while no key is pressed it swaps screens, draws clouds and sprite frames, and plays sound at set points, then returns once a key is pressed.
- `SUBCLOUDS`: Draws one cloud sprite at a horizontal position offset by a fixed amount, shifted by minus twice the first input when the second input is above six.
- `COPYBACK`: Copies the front screen buffer into the back screen buffer, four longwords at a time over 2000 blocks, which covers the whole four-plane screen image.
- `PARADROP`: Computes a count from the demo timer, capped at sixteen, and when it is above one draws the sprite twice with the restore-sprite routine.
- `GETMODE`: Shows the mode selection screen: it copies the background, clears blocks, draws the menu graphics, then waits for key or joystick input to move the highlighted option.
- `DRAWITEMS`: Draws the outlines of seven item boxes by calling the line routine four times for each box, with each box's row position set by its index.
- `SELECTITEM`: this routine is empty and performs no operations before returning.
- `DRAWLINE`: Calls the line drawing routine and returns, passing the position and state through unchanged.
- `RSPRITERBOTH`: Draws a sprite with the restore routine, then repeats a masked, shifted four-plane write into the back screen buffer.
- `RDRAW_BOTH`: Copies saved bitplane words from the buffer into the back screen buffer, reversing the save routine, with the region offset taken from the position supplied.
- `RCLEARBLOCK`: Fills a rectangle of the four bitplanes: the first plane is set to all ones and the other three are cleared to zero, one byte per column of each row.
- `PX1`: This label is used as a data address by the draw-items code, not as a callable entry.
- `PX2`: Updates player state variables including energy, position, and movement direction based on joystick input.
- `BRAINMOVE`: Chooses the computer spy's next movement target by reading the joystick, updating the trouble and bastard counters for each spy, and setting the target cell from the held item and item positions with random and mound-finding fallbacks.
- `GETD0D1`: Converts a four-bit direction mask into the spy direction value: the first bit gives three, the second gives four, the third gives one and the fourth gives two, and zero is used when no bit is set.
- `BRAINBUSY1`: Checks if a value at a pointer is zero, then clears one of two global frame counters based on a register value and writes a fixed code to that pointer.
- `reset_frame_and_flag_code`: Sets the second spy's frame counter to zero and stores a fixed flag code in the word at the pointer, unconditionally.
- `RANDMOVE`: Occasionally, when the low five bits of a random value match a fixed pattern, picks a new spy direction from two further random bits and stores a random nudge value from zero to seven for one of the spies.
- `FIND_A_MOUND`: Scans a four-row by sixteen-column block of map cells around the spy's position for a cell with the mound marking bits, stores the position of the first match, and sets a flag for the chosen spy's bad-move state.

### begin

- `jump_to_init`: Jump stub: passes control straight on to INIT.
- `E_EXPLODE`: Sets the explosion flag bits in a byte, calls a system library routine through a pointer and starts the sound system, then runs eleven rounds of the dive and explode routines, waiting on a hardware status bit between rounds, and finally restores the saved interrupt.
- `E_DIVE`: Runs the same sequence as its sibling explosion routine: sets flag bits, starts the sound system, then runs eleven rounds of the dive and explode routines, waiting on a hardware status bit between rounds, and finally restores the saved interrupt.
- `TEST`: Calls a system library routine through a pointer, starts the sound system, then runs eleven rounds of the dive and explode routines, waiting on a hardware status bit between rounds, and finally restores the saved interrupt.
- `START_SOUNDS`: Initialises the sound system: fills a random table from a three-word generator, sets up the four audio channel records with their envelope and sample pointers, calls the music initialiser, saves the current level-six interrupt vector and installs the music interrupt handler, then writes the audio hardware registers.
- `SHOOT`: Starts a shot sound on the fourth audio channel by pointing it at the noise sample, setting its length and period to 2000, clearing its envelope state and attaching the shoot envelope routine.
- `SHOOT2`: Sets the length register of the fourth audio channel to the fixed value 4096 and returns.
- `EXPLODE`: Starts an explosion sound on the fourth audio channel by pointing it at the noise sample, setting its length and period to 3000, clearing its envelope state and attaching the explode envelope routine.
- `DIVE`: Starts a dive sound on the third audio channel using the triangle waveform, with its period set to 2130 and the dive envelope routine attached.
- `MYINT6`: Interrupt handler that, when sound is enabled, counts down a music delay and plays the next music step each time it reaches zero, advances any active volume envelopes, and then acknowledges the interrupt.
- `advance_music_tick_and_envelopes`: Resets the music delay to three, plays the next music step when music is switched on, advances the active volume envelopes, then acknowledges the interrupt.
- `CHANGE_ENVELOPE`: Advances one channel's volume envelope.
- `X8912`: Updates the music voices: for each channel that is not disabled it loads the waveform length, waveform pointer and frequency from tables indexed by the current music values, or uses the noise sample and a temporary period, then writes the channel registers and clears the update flag.
- `DO_ENVEL1`: Handles one music channel's envelope and register update.
- `XNOISE1`: Prepares a noise voice by setting its sample pointer and length and computing a temporary period from the music value.
- `XDONE1`: Loads the waveform length, waveform pointer and frequency for one music channel from the tables, or the noise sample when the noise bit is set, writes them to the audio channel registers, and clears the channel's restart flag.
- `DO_ENVEL2`: When the channel's restart flag is set, reinitialises that channel's volume and period from the envelope table and clears its counters.
- `XNOISE2`: Sets up the second noise voice: it points the noise sample at a temporary period value computed from the music value, sets or resets the volume according to the envelope value and restart flag, then clears the restart flag.
- `XDONE2`: Clears the restart flag of the channel and returns.
- `PSGW`: Writes one byte to the sound chip register selected by the index value supplied, through the register array.
- `INITMUSC`: Sets the sound chip mixer register to the value 56, which turns off the tone and noise outputs of the three voices, then silences the voices through the voice helper routine.
- `PLAYMUS`: Plays one music step on the first two voices: for each one it selects the voice, enables it and outputs its volume registers.
- `VOICEISOF`: Resets one voice's sequence state and loads its first note values from the sequence data.
- `NEXTSEQ`: Steps a voice to its next sequence entry: when the count runs out it moves forward three bytes, reloads the count and loads the note, pitch offset and volume fields.
- `VOICISON`: Counts down a voice's note timer and, when it expires, starts the voice's next note.
- `NEXTNOTE`: Handles a voice's next note.
- `NEXTNOTO`: Advances a voice to its next note by stepping the sequence, taking the duration from the duration table and computing the note value to store in the voice record.
- `CONTSONG`: Takes the duration for the current sequence entry from the duration table, then computes the note value and stores it in the voice record so the note continues.
- `compute_voice_note_value`: Clears the gate flag, then returns the low bits as the note value when the note's high bits are zero.
- `store_voice_note_and_volume`: Runs the channel check, computes the note value, and stores it and the volume value in the voice record.
- `DEGATEIT`: Clears the gate flag in the voice record.
- `CHECK2`: Sets the mixer register bits for the current channel from the voice's type bits, writes them to the sound chip, and sets the voice's gate count.
- `SNAREA`: Sets the mixer bits for the first channel's noise output, writes the sound chip registers, and sets the voice gate count to sixteen.
- `CHANNELB`: Chooses tone or noise output for the second channel from the voice's type bits, writes the mixer and volume registers, and sets the voice gate count to sixteen or twelve depending on the note type.
- `SNAREB`: Sets the mixer bits for the second channel's noise output, writes the sound chip registers, sets the voice gate count to sixteen, and writes three more registers.
- `CALCNOTE`: Takes the modified note value, reads its high nibble, and when the low three bits of that nibble are set it reduces the count by one or two.
- `OUTVC1`: Calls the sound output routine for the first voice and returns.
- `OUTVC2`: Calls the sound output routine for the second voice and returns.
- `SNDOUT`: Writes three consecutive sound chip registers through the register write routine.
- `NOTEMODS`: Adjusts the note's pitch by a signed modifier taken from the low bits of its input.
- `NOTEMOD5`: Adds a negated pitch step to the note's low nibble, applies a borrow into the octave bits when there is no carry, and writes the result to the note output variable while keeping the saved octave bits.
- `NOTEMOD6`: Combines the low nibble of its input with the saved octave bits and stores the result in the note output variable.
- `XXMOD5`: Works like the negated pitch adjustment but uses only the low three bits of its input as the step, then stores the adjusted nibble with the saved octave bits in the note output variable.
- `XXMOD1`: Combines the input value with the saved octave bits and stores the result in the note output variable.
- `RESET`: Writes three sound chip registers through the register write routine and returns.
- `INIT`: Runs the startup sequence: it allocates memory, starts the sound system, sets up the screen, draws and copies the picture resources, builds the fade mask table from its template, runs the blitter and pixel effects with timed pauses, releases the memory, restores the saved interrupt and then calls the system exit routine.
- `BLIT_ME_HONEY`: Copies a four-plane picture into the screen through a mask: for each word it ANDs the picture data with the mask and ORs the result into the screen across the four bitplanes.
- `PIXEL_PIECE`: Calls the piece blit routine sixteen times in a loop, with no other work of its own.
- `BLIT_ME_HONEY_PIECE`: Blits a small block one row at a time.
- `MOVE_THE_JETS`: Copies the first screen to the second, then sweeps a jet sprite across the screen twice, once per raster value, blowing a hole every eighth step.
- `DO_RESTORE_OF_JET`: Copies a 32-row block of background from the second screen buffer back to the first at the saved horizontal offset, moving a fixed number of words per row.
- `SHOW_JET`: Waits for the raster position, restores the background under the jet, then draws the current jet animation frame over 32 rows, masking each word before ORing in the frame's pixels.
- `WAIT_FOR_RASTER`: Busy-waits on the video beam position until it reaches the requested raster line, within a short window of lines.
- `BLOW_A_HOLE`: Advances the hole index, wrapping after eight, and when the position is inside the visible range moves the hole animation twice.
- `MOVE_A_HOLE`: Draws a four-by-four hole pattern into the four bitplanes using masks.
- `ATARI_COPY`: Copies a picture from interleaved source words into the four separate bitplanes, taking four words per step and writing each to its own plane, over 4000 steps.
- `COLOUR_COPY`: Converts sixteen palette entries into colour register values by shifting each entry left and setting a fixed set of low bits, then stores the results in the colour table.
- `LOAD_FILE`: Opens the DOS library through the system library, opens the file handle from it, and then calls two further library routines to finish setup.
- `SETUP_SCREEN`: Sets the two screen buffer pointers, copies a display list template into the copper list, fills the bitplane pointers, and programs the display window, bitplane modulo and control registers before starting the copper list.
- `WHICH_PLANES`: Writes the addresses of the four bitplanes into the plane pointer table, each one spaced 8000 bytes after the previous.
- `BLIT_A_SPY`: Sets the blitter mode, then runs one full blit followed by three follow-up blit passes to draw a spy image.
- `BLIT`: Loads the blitter masks, modulo, pointer, control and size registers from the values supplied, then waits for the blit to finish.
- `BLIT2`: Loads the blitter pointers and size for a follow-up pass, then waits for the blit to finish.
- `WAIT_BLIT`: Polls the blitter status until the busy bit clears, then returns.
- `SWAP_SCREEN`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `CLEAR_SCREEN`: Zeroes 2000 blocks of four longwords each, clearing the whole four-plane screen buffer at the address it is given.
- `ALLOCATE_MEMORY`: Allocates six 32-kilobyte memory blocks through the 32K allocator, one each for the loading screen, the two screen buffers and the title, jet and credits pictures, storing each address in its own variable.
- `RESCIND_MEMORY`: Releases the six 32-kilobyte blocks by calling the release routine six times.
- `FIND32K`: Allocates a 32-kilobyte block through the system memory allocator and enters an endless loop if the allocation fails.
- `LOSE32K`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `DECO_PIC`: Loads a picture file, copies its sixteen palette words, then unpacks run-length encoded picture data into the bitplane buffers over four passes.

### port-handler

- `process_hunk_relocation_records`: Processes a program's hunk and relocation records, comparing each record type against the executable hunk codes and patching address words as it goes.

### disk-validator

- `validate_disk_bit_table`: Formats a two-digit count into a text line, sets a table of 32-bit entries to all ones, then walks a bitmap comparing bits and counting differences, calling library routines along the way.

### ram-handler

- `copy_memory_then_dispatch`: Calls the system memory copy routine through the library base, then makes an indirect jump through a jump table whose targets could not be recovered.

### serial.device

- `stub_bad_instruction_data`: The entry decodes to no valid instructions and the decompiler reports bad instruction data here, so there is no behaviour to describe.
- `initialise_serial_device_buffers`: Frees a supplied block through the library when one is given, and allocates a block of the requested size when the size is nonzero.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `clear_request_status_and_signal`: Clears the request's status byte, then if its flag bits are all clear, calls a vector through the device base to signal completion.
- `toggle_interrupt_enable_bits`: Writes the interrupt enable register with the set bit clear, calls a helper routine, then writes the same value with the set bit raised, so the interrupt bits are disabled around the helper and enabled again afterwards.
- `remove_serial_request_from_queue`: Takes a serial request off its queue: if it is the head of the active list the list head and busy bit are cleared, otherwise it is unlinked from its neighbours and the queue count is decremented.
- `unlink_serial_request_node`: Unlinks a serial request from its list when it is not the head, decrementing the list count, or clears the head pointer and busy bit when it is.
- `complete_serial_request_if_flagged`: When the request's flag bit is set, unlinks it from its list and decrements the list count, then marks it done and calls a device vector when its completion bit is clear.
- `finish_serial_request_chain`: Finishes a serial request: at the head of the active list it clears the list head and busy bit, first calling the interrupt helper with a fixed mask if a flagged type-ten request was waiting; otherwise it unlinks the request and decrements the count.
- `release_flagged_serial_node`: When the node's flag bit is set, unlinks it from its list and decrements the count, marks it done and clears its status byte, then calls a device vector when its completion bit is clear.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `shut_down_serial_unit`: If the unit still has pending requests, sets its pending flag and returns zero.
- `release_serial_unit_and_unlink`: Calls the same series of device vectors as its sibling routine, releases the unit's block, unlinks its node from its list and returns the stored value, without the pending-request check.
- `queue_serial_request_node`: Clears the request's count, then either queues the request at the head of the active list with its flags set, after checking the unit's state and calling a device vector, or clears its active bit.
- `transfer_serial_buffer_bytes`: Moves bytes between a ring buffer and the serial port, one per pass, wrapping the buffer pointers and applying the flow-control flag; it calls a flow-control helper and, when a request finishes, a device vector.
- `continue_serial_byte_transfer`: Continues the byte transfer loop: while the transfer count remains it moves one byte between the ring buffer and the port, wrapping pointers and handling the flow-control flag.
- `update_serial_status_and_queue`: Updates serial port status flags and manages a linked list of data buffers by popping elements from a queue.
- `process_serial_buffer_queue`: Manages a linked list of data buffers by popping elements from a queue and updating status flags.
- `process_serial_data_transfer`: Manages the transfer of bytes between a ring buffer and a serial port while handling flow control and buffer pointer wrapping.
- `return_from_subroutine_2`: This routine performs an empty return operation.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `update_serial_status_flags`: Updates status flags based on bitwise operations on a control byte and manages a linked list of data buffers.
- `process_serial_data_buffer`: Iterates through a linked list of data buffers to extract bytes and update status flags.
- `process_serial_queue`: Iterates through a linked list of data buffers to process queued bytes.
- `update_serial_input_buffer`: Checks serial status flags and compares incoming data against specific byte values to update status bits.
- `handle_serial_interrupt`: Manages serial data transfer by storing a byte into a buffer and updating pointer offsets.
- `update_serial_buffer_3`: Manages a circular buffer by storing a byte from a register into a memory location and incrementing various pointers and counters.
- `insert_buffer_byte`: Checks a bit in a register and either inserts a byte into a buffer while managing pointer wrapping or sets a status bit in a structure.
- `update_serial_buffer`: Manages a circular buffer by storing a byte from a register into a memory location and incrementing various pointers and counters.
- `set_status_flag`: Sets a specific bit in a byte within a structure and conditionally copies a pointer to another location in that structure before calling a library routine.
- `manage_buffer_insertion`: Inserts a byte into a buffer at a pointer location, increments counters, and handles buffer wrapping or pointer resetting.
- `queue_entry_and_call_library`: Fills an empty pointer field if needed, calls another routine, and then calls a library routine through a global base.
- `update_serial_status`: Updates a byte in a data structure, modifies a hardware register, and conditionally updates a global variable.
- `modify_hardware_status`: Updates a byte in a data structure and modifies a global variable based on specific bit flags.
- `clear_structure_byte`: stores zero at an offset of 29 bytes from a pointer held in a register.
- `initialize_serial_state`: Sets a bit in a status byte, clears a value, and initializes a structure with a decremented pointer and a constant value before calling a routine.
- `clear_status_bit_2`: Clears a bit in a structure and, if specific conditions are met, updates a pointer and calls another routine.
- `process_serial_input_buffer`: Manages a circular input buffer by reading bytes and comparing them against specific target values to update status flags.
- `set_status_bit_2`: Sets a bit in a structure, updates a pointer if necessary, and calls another routine.
- `dispatch_interrupt_handler_2`: Tests a byte at an offset from a pointer against a mask and conditionally calls a handler routine or inserts an entry into a linked list.
- `dispatch_interrupt_handler`: Tests a byte at an offset from a pointer against a mask and conditionally calls a handler routine.
- `return_from_handler`: This routine is an empty function that performs a return operation.
- `set_entry_state_thirteen`: sets a byte at an offset of 31 bytes from a pointer to thirteen if two specific bits are set in a global variable.
- `manage_serial_buffer_and_callbacks`: Manages a linked list of data buffers and processes a queue of callback functions.
- `update_serial_buffer_state`: Updates internal counters and status flags based on incoming serial data and compares the resulting bitmask against a lookup table.
- `process_serial_event_2`: Updates status bits and counters based on hardware register values and performs a comparison against a table of values.
- `process_serial_data`: Combines a value with a bitmask, increments counters, and performs a loop to compare data against a table before calling a library routine.
- `process_callback_queue`: Checks a counter to determine if a callback queue needs processing.
- `set_global_flag`: stores the value one into a global variable.
- `update_serial_state`: Sets a global variable and modifies a byte at an offset from a pointer.
- `configure_serial_parameters`: Sets a global variable, clears several fields in a structure, and populates a structure with multiple 32-bit constants before calling a routine.
- `call_conditional_handler`: calls a handler routine obtained from a table if a specific bitmask check on a byte at an offset from a pointer passes.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `process_serial_event`: Updates a status byte in a data structure, conditionally calls another routine, and may call a handler routine based on a bitmask test.
- `clear_interrupt_request`: Clears a bit in a status byte and conditionally calls a routine based on a value in a structure and a global variable.
- `update_flags_and_call_handler`: sets a global variable to two different values and calls a handler routine if a bitmask check on a byte at an offset from a pointer passes.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `process_linked_list`: Iterates through a linked list of pointers, updating the list structure and calling another routine for each non-zero element.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_serial_data`: performs several bitwise operations and transfers data between various offsets of two different structures before conditionally calling a handler routine.
- `enqueue_interrupt_request`: Clears a bit in a status byte and, if certain conditions are met, inserts a structure into a linked list and increments a counter.
- `handle_callback_result`: Calls a routine and then either updates a pointer and a status byte in a structure or calls a handler routine and clears a pointer.
- `clear_entry_pointer`: calls a handler routine and then sets a long value at an offset of 78 bytes from a pointer to zero.
- `modify_status_flags`: Clears a bit in a status byte and conditionally updates a pointer and a flag if a specific value is found in a linked structure.
- `initialize_structure_fields`: Sets several fields in a structure to specific values and calls another routine to retrieve a return value.
- `manage_serial_device_state`: Updates device status flags and bitmasks within a data structure based on current hardware or software state.
- `reset_device_status`: Clears a bit in a structure and either calls another routine with specific values or sets a byte in a structure.
- `reset_serial_status`: Updates a global variable and conditionally calls a routine based on a bitmask check in a structure.
- `validate_and_update_serial_parameters`: Validates a set of input parameters including character bounds and integer ranges, then copies these values into a destination structure.
- `calculate_bitmask_value`: Computes a bitmask based on a byte value and various flags, then stores the result and two other values into a table.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_byte_value`: Stores the constant value five into a byte at a specific offset from a pointer.
- `calculate_timing_value`: Performs a division based on a value in a structure, stores the result, and conditionally modifies status bits in a byte.
- `update_division_result`: Performs a division, stores the result in a structure, and modifies status flags based on a character comparison.
- `initialize_serial_device_structure`: Populates a device structure with configuration values, memory pointers, and function pointers.
- `configure_serial_structure`: Writes several constant values and memory pointers into a structure and assigns a routine to a function pointer within that structure.
- `store_structure_data`: Stores several values from registers into a structure pointed to by a register.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `configure_serial_device`: Initializes a structure with specific values, sets up callback pointers, and enables interrupts via the hardware.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `return_zero`: This routine is empty and simply returns zero to the caller.

### gb

- `xor_memory_values`: Performs bitwise XOR operations on two values stored in memory.

### parallel.device

- `process_state_logic`: Modifies several bytes and words in memory based on conditional checks of structure members and flags.
- `sync_state_data`: Copies data from one memory structure to another and increments a counter if certain conditions are met.
- `execute_conditional_callbacks_2`: Checks a byte against a mask to call a routine, then performs conditional logic based on a short value to modify a global variable and a status byte.
- `execute_conditional_callback`: Checks a value in a structure and conditionally calls a routine via a pointer while modifying a status byte.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `process_conditional_callbacks_3`: Checks a byte against a mask to conditionally call a routine and modifies a global variable based on a short value.
- `initialize_device_context_3`: Updates structure fields and status bytes, potentially inserting a node into a linked list and modifying a global variable.
- `initialize_device_context_2`: Modifies status bytes in a structure and conditionally performs a linked list insertion or updates several fields in a structure and a global variable.
- `initialize_device_context`: Sets several fields in a structure, modifies a global variable, and performs bitwise operations on memory values.
- `process_parallel_request_queue`: Manages a linked list of pending requests by popping elements and updating status flags.
- `update_parallel_status`: Checks a status bit, updates a global variable from a pointer, increments counters, and performs a table comparison before calling a routine.
- `clear_request_status`: Copies a value from one structure to another, clears specific bits in status bytes, calls a routine, and conditionally calls another routine.
- `update_request_status`: Increments counters in a structure and conditionally clears bits in a status byte before calling another routine.
- `synchronize_state_data`: Updates a status byte and conditionally copies data from one structure to another.
- `return_immediately_2`: This routine performs no operations and returns immediately to its caller.
- `call_if_not_zero`: Checks if a value in a structure is non-zero and calls a function through a pointer if it is.
- `invoke_callback_pointer`: Executes a function through a pointer stored in a structure if the associated index is not zero.
- `traverse_and_clear_list`: Iterates through a linked list of pointers to update the list structure and then clears a word at an offset from a global pointer.
- `clear_word_field`: Stores a zero word into a specific offset within a structure.
- `modify_status_byte`: Sets a specific bit in a byte and stores a value into another byte before calling a function via a pointer.
- `update_global_and_field`: Manipulates bits within a global variable and updates a byte in a structure using a library function call.
- `transfer_conditional_data`: Copies a byte and potentially several long words from one structure to another based on a bit flag.
- `copy_conditional_structure_fields`: Copies a byte from a source structure to a destination structure, then conditionally copies two long words from the source to the destination if a specific bit is set in the copied byte.
- `process_parallel_request`: Manages a linked list of pending requests by popping elements and updating status flags.
- `return_zero_2`: This routine performs no operations and returns zero.

### printer.device

- `initialize_printer_device`: Calls a routine via a pointer in a structure, stores the returned value in a global variable, and updates a structure member using a value from a register.
- `update_linked_list_node`: Sets a specific bit within a byte and performs a linked list insertion if a condition is met.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls a function through a pointer.
- `execute_printer_initialization`: Calls a routine through a pointer, stores the result in a global variable, and updates a structure member with a value from a register.
- `initialize_printer_buffer`: Checks multiple pointers via library calls, fills a memory range with a constant value, and initializes several structure fields.
- `execute_indirect_calls`: Calls a function through a pointer multiple times using values from a table.
- `execute_recursive_calls`: Performs three consecutive calls to another routine.
- `execute_indirect_call_2`: Performs a recursive call to itself.
- `set_status_bit`: Sets a specific bit in a byte located at an offset within a structure.
- `clear_status_bit`: Clears a specific bit in a byte located at an offset within a structure.
- `traverse_linked_list`: Iterates through a linked list by following pointers until a null pointer is encountered.
- `return_immediately_3`: This routine is empty and performs no operations before returning to its caller.
- `execute_recursive_call`: An empty routine that immediately calls itself.
- `initialize_event_structures`: Initializes two separate structures with pointers and values, then enters a loop to process bitmasks and linked list nodes.
- `process_event_loop`: Iterates through a linked list to call functions via a jump table while updating status bits in a byte based on specific values.
- `handle_status_updates`: Checks bitmasks to iterate through a linked list and update status bits in a byte based on returned values.
- `call_transfer_routine`: Calls a routine that manages the transfer of bytes between a ring buffer and a serial port.
- `update_serial_buffer_2`: Checks a return value to decide whether to set bits in a global status byte and store data into a table.
- `return_from_pointer`: An empty routine that performs no operations and returns immediately.
- `update_state_data`: Sets a bit in a byte and stores several values into a table.
- `poll_device_status`: Checks a byte for specific bit patterns and calls multiple functions through pointers while updating status bits.
- `process_device_status`: Executes a sequence of routines through pointers stored in a structure and modifies specific bits in a status byte based on comparison results.
- `update_device_status_flags`: Checks a status byte and clears specific bits in a global variable based on comparison results.
- `process_device_status_flags`: Checks a status byte and clears specific bits in a global variable based on comparison results.
- `poll_device_status_2`: Polls a status byte and executes a sequence of routines through pointers while checking for specific termination conditions.
- `initialize_structure_data`: Writes several values to a structure and then calls a routine through a pointer found within that structure.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `execute_callback_sequence_2`: Checks a character at an offset within a structure and then executes two routines through pointers.
- `execute_callback_sequence`: Calls a routine through a pointer in a structure and then calls another routine using a value from a global base.
- `process_device_loop`: Uses a global pointer to access a structure and enters a loop that executes several routines through pointers based on status checks.
- `process_printer_status`: Enters a loop that executes routines through pointers and modifies bits in a status byte based on returned values.
- `process_device_status_2`: Checks a value against specific offsets and performs bitwise clearing on a byte within a structure.
- `loop_device_status`: Repeatedly executes routines via pointers from a structure while checking status bits and performing conditional bitwise operations on a byte.
- `process_device_status_flags_2`: Checks a bitmask within a structure to determine if a loop should continue or exit.
- `update_device_state_2`: Modifies a global pointer and status bits based on a conditional check, then executes another routine if specific bits are set in a byte.
- `manage_interrupt_status`: Modifies a global variable and status bits, potentially enabling or disabling interrupts via a hardware register.
- `clear_status_bits`: Clears specific bits in a byte at a given offset and recursively calls itself if certain bits were previously set.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_list_and_copy_string`: Performs a linked list insertion, sets a status bit in a byte, and copies a null-terminated string into a lookup table.
- `reset_structure_field_2`: Sets a 32-bit field in a structure to zero and conditionally updates it from another field based on a routine return value.
- `update_list_and_lookup_table`: Sets a status bit in a byte and performs a linked list insertion if a flag is set.
- `update_linked_list_and_lookup_table`: Sets a status bit in a byte, performs a linked list insertion if a flag is set, and copies a null-terminated string into a lookup table.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `copy_and_filter_string`: Copies a string from a source buffer into a local buffer and performs a pattern-matching search to identify specific byte sequences.
- `reorder_buffer_by_pattern`: Iterates through a data buffer to locate a specific byte sequence and reorders or copies elements based on the match results.
- `search_buffer_for_sequence`: Iterates through a data buffer to locate a specific byte sequence.
- `search_and_match_buffer_sequence`: Iterates through a data buffer to identify and match specific byte sequences or control characters.
- `search_and_reorder_buffer`: Iterates through a data structure to find specific byte matches and performs conditional reordering of elements within a table.
- `search_and_reorder_buffer_sequence`: Iterates through a data buffer to locate a specific byte sequence and reorders or copies elements based on the match results.
- `copy_and_clear_memory`: Copies data from one memory location to another using an incrementing index and then stores either a specific value or zero at a given offset.
- `clear_memory_offset`: Stores a zero at a specific offset within a memory location pointed to by a register.
- `transform_glyph_buffer`: Iterates through a buffer to map input bytes to specific characters using a multi-dimensional table lookup.
- `map_glyph_buffer`: Iterates through a buffer and compares each byte against a table to map input values to specific characters.
- `translate_glyph_stream`: Iterates through a buffer to translate input bytes into characters using a multi-dimensional table lookup.
- `process_input_glyph_stream`: Iterates through an input buffer, translating each byte into a character via a multi-dimensional table lookup or conditional logic.
- `map_input_to_glyph`: Translates an input byte into a character value by performing a multi-dimensional table lookup.
- `translate_input_to_glyph`: Translates an input byte into a character value by searching through a multi-dimensional table.
- `map_input_to_character`: Iterates through a table to translate an input byte into a new character value based on specific matching criteria.
- `process_character_data`: Performs a complex search through a table to map input values to new characters and modifies a global array by shifting its elements.
- `translate_glyph_to_character`: Iterates through a buffer to map input bytes to characters using a multi-dimensional table lookup.
- `expand_ansi_control_sequences`: Iterates through an input character stream to identify and expand ANSI escape sequences and control codes into a destination buffer.
- `parse_ansi_control_sequences_2`: Iterates through a character stream to identify and expand ANSI escape sequences and control codes into a destination buffer.
- `parse_ansi_control_sequences`: Iterates through a character stream to identify and expand ANSI escape sequences and control codes into a destination buffer.
- `process_ansi_escape_sequences_2`: Parses a character stream to identify and expand ANSI escape sequences and control codes into a destination buffer.
- `parse_escape_sequence_stream`: Parses a stream of characters to handle escape sequences and control codes.
- `expand_ansi_control_codes`: Parses a character stream to identify and expand ANSI escape sequences and control codes into a destination buffer.
- `parse_ansi_escape_sequences_5`: Processes a stream of characters to identify and handle escape sequences, control codes, and special formatting commands.
- `parse_escape_sequences`: Processes a stream of characters to handle escape sequences and control codes.
- `parse_escape_sequences_2`: Processes a stream of characters to handle escape codes and special control sequences.
- `expand_ansi_escape_sequences`: Parses a character buffer to identify and expand ANSI escape sequences and control codes into a destination buffer.
- `process_ansi_escape_sequences`: Iterates through a character buffer to identify and process ANSI escape sequences and control codes.
- `parse_escape_sequences_3`: Parses a string for backslash-prefixed escape sequences and converts them into specific character values.
- `parse_ansi_escape_sequences_4`: Iterates through a character buffer to identify and process ANSI escape sequences and control codes.
- `parse_ansi_escape_sequences_3`: Iterates through a character buffer to identify and process ANSI escape sequences and control codes.
- `parse_ansi_escape_sequences_2`: Iterates through a character buffer to identify and process ANSI escape sequences and control codes.
- `parse_ansi_escape_sequences`: Parses a string of characters to identify and process ANSI escape sequences and control codes.
- `parse_ansi_control_codes`: Iterates through a character buffer to identify and process ANSI escape sequences and control codes.
- `expand_ansi_escape_sequences_2`: Iterates through a character stream to identify and expand ANSI escape sequences and control codes into a destination buffer.
- `update_global_values`: Subtracts the difference between two global variables from a third global variable and clears a fourth global variable.
- `buffer_data_transfer`: Copies a sequence of bytes from a local stack buffer to a destination memory location using a global offset.
- `copy_memory_block_3`: Copies a sequence of bytes from a source address to a destination address using a loop.
- `decrement_global_pointer`: Decrements a global variable by one.
- `copy_memory_block_2`: Copies a sequence of bytes from a source address to a destination address using a loop and an incrementing global pointer.
- `copy_memory_block`: Copies a sequence of bytes from a source address to a destination address, incrementing a global pointer for each byte.
- `parse_numeric_stream`: Converts a stream of numeric characters into an integer value and stores the result in a buffer.
- `parse_input_string_2`: Iterates through a character stream to convert numeric characters into an integer value stored in a destination buffer.
- `parse_numeric_string`: Processes characters from an input stream to build a numeric value in a destination buffer.
- `parse_input_string`: Processes characters from an input stream, performing arithmetic on numeric characters and handling specific delimiters to build a string.
- `parse_numeric_input`: Iterates through a character stream to convert numeric characters into an integer value.
- `fetch_table_element`: Retrieves a byte from a table using an index stored in a pointer.
- `return_register_value`: Returns the value currently held in a data register.
- `decrement_and_check_limit`: Decrements a value and checks if it has reached a specific bit pattern or a minimum threshold.
- `decrement_and_check_table`: Decrements a value pointed to by a register and continues looping as long as the value is positive and a specific bit pattern is found in a table.
- `initialize_global_state`: Resets several global variables and performs a comparison check before setting a global flag and specific byte values.
- `copy_data_to_globals`: Copies six 16-bit values from a structure into a set of global variables.
- `compare_global_data`: Compares values in a structure against a set of global variables and returns zero if they match or all ones if they do not.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `dispatch_command_sequence`: Extracts a sequence of bytes from a structure and passes them to another routine, conditionally calling different versions of that routine based on the results.
- `dispatch_conditional_routines`: Calls various versions of another routine with different parameters based on the state of registers and return values.
- `process_printer_command_string`: Parses a command identifier to select a string from a table and copies it to a destination buffer.
- `copy_formatted_string`: Copies characters from a source to a destination while handling specific byte values as termination or replacement markers.
- `copy_terminated_string`: Copies characters from a source to a destination, handling specific termination values and adjusting the pointer based on the character encountered.
- `copy_string_with_termination`: Copies a null-terminated string to a destination and appends a byte from a pointer if a specific value is matched.
- `copy_string_with_special_termination`: Copies a string from a source to a destination while checking for specific character values to trigger a termination or an additional byte copy.
- `update_device_state`: Checks a byte in a structure and sets it to one if a condition is met, then calls a routine using a global pointer.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `validate_clipping_bounds`: Checks if a specified rectangular area fits within the dimensions of a source buffer or object.
- `calculate_clipping_rectangle`: Determines the dimensions and position of a clipping rectangle by comparing object bounding boxes against a defined viewport area.
- `calculate_collision_bounding_box`: Determines the width and height of a bounding box by calculating the difference between coordinate pairs in a data structure.
- `update_collision_bounding_box_6`: Calculates the minimum and maximum axis-aligned coordinates from a data structure to populate a bounding box.
- `update_collision_bounding_box_5`: Calculates the minimum and maximum axis-aligned coordinates from a data structure to populate a bounding box.
- `update_collision_bounding_box_4`: Determines the minimum and maximum axis-aligned coordinates from a data structure to populate a bounding box.
- `update_collision_bounds`: Calculates the minimum and maximum boundary coordinates by comparing values from a data structure and updates a bounding box structure.
- `update_collision_bounding_box_2`: Calculates the minimum and maximum boundary coordinates by comparing values from a data structure and updates a bounding box structure.
- `update_collision_bounding_box_3`: Calculates the minimum and maximum boundary coordinates by comparing values from a data structure and updates a bounding box structure.
- `update_collision_bounding_box`: Calculates the minimum and maximum boundary coordinates by comparing values from a data structure and updates a bounding box structure.
- `update_bounding_box_limits_2`: Calculates and updates minimum and maximum boundary coordinates by comparing current values against a range and swapping them to maintain consistency.
- `update_bounding_box_limits`: Updates a set of boundary coordinates by comparing current values against a range and swapping them to maintain a consistent minimum and maximum order.
- `clamp_coordinate_bounds`: Clamps two coordinate values within a specified range and calculates the resulting minimum and maximum bounds.
- `clamp_and_clear_raster_window`: Clamps coordinate values within specified bounds and calculates the center point of the resulting window.
- `clip_print_window_and_setup`: Clamps the source and destination offsets of a printed raster window, then clears a 64-entry colour table and refills it from per-plane palette lookups through helper calls.
- `load_palette_into_colour_table`: Reads the bitmap depth from the printer's source descriptor and loops over every colour index, storing each packed four-bit plane value from a palette helper into the shared 64-entry colour table.
- `scale_print_region_colours`: Applies a rounded shift by sixteen bits to a coordinate when its flag bit is set, then rebuilds the 64-entry colour table from palette lookups, complementing nibbles when the inversion setting is clear.
- `compare_and_clip_print_edges`: Compares two computed row limits and replaces the upper bound with the smaller one, then runs the same colour-table rebuild as the neighbouring routines.
- `build_plane_colour_table`: Computes the horizontal and vertical span of the printed window, clears the 64-entry table and fills per-plane colour nibbles from a palette lookup over each index of the bitmap depth.
- `build_colour_table_from_row`: Takes its horizontal offset from a word in the source record, recomputes the span, clears the 64-entry table and refills it from palette lookups.
- `build_colour_table_from_column`: Takes its vertical offset from a word in the source record, recomputes the span, clears the 64-entry table, refills it from palette lookups and complements each nibble when the inversion setting is clear.
- `map_row_pixels_to_colours`: Loops over a run of pixels, looks up a colour nibble for each one through a palette helper, rebuilds the 64-entry table with optional nibble complement and calls the printer's status function pointer to emit the row.
- `request_printer_colour_mapping`: Clears the colour index, calls the printer status function pointer with the source row and destination position, and when that returns zero tries a second output code chosen by the mode flags.
- `query_printer_device_status`: Calls the printer status function pointer with a fixed mode code and, when it returns zero, chooses a fallback result from the mode flags before passing the status to a helper.
- `select_colour_fallback_code`: Chooses a printer output code from the mode flags, then forces the result to zero when the column counter is a multiple of the row width or a follow-up status call returns zero.
- `select_dither_mode_code`: Picks one of several printer output codes from the mode flags, then forces the result to zero when the column counter is aligned to the row width or a follow-up status call returns zero.
- `select_plain_output_code`: Passes a single printer output code from the mode flags to the output helper, zeroing it when the column counter is aligned or a follow-up status call returns zero.
- `emit_pixel_result_code`: Calls the output helper with the computed pixel result, zeroing it first when the column counter is a multiple of the row width or a second status call returns zero.
- `emit_zero_or_status`: Zeroes the status result when the column modulo test passes or the follow-up status call returns zero, then hands the value to the output helper.
- `reset_printer_output_state`: Calls the output helper with two zero arguments and returns, resetting the printing state without any colour work.
- `select_luminance_or_minimum`: With the mode flag clear, returns the input unchanged unless a bit is set, in which case it returns the smallest channel.
- `select_minimum_channel_value`: Returns the smallest of three byte colour channels when a particular mode bit is set, otherwise returns the packed value unchanged.
- `fetch_pixel_colour_index`: Computes a pixel position from the stored offsets and sign-adjusted step, looks it up through a helper, and maps the result to a colour from the 64-entry table, using a sentinel for unmapped pixels and nibble inversion in mode one.
- `store_pixel_colour_at_position`: Looks up the pixel code at a computed position and writes the matching colour into the destination through the output pointer, handling the sentinel value and nibble inversion for the mode bit.
- `store_colour_and_accumulate_sums`: Selects a colour index from the stored plane table, complements it when the inversion setting is clear, passes it through a helper and stores the result, then adds the current weight to four running sums.
- `apply_colour_and_accumulate`: Applies the selected colour index through the same plane-selection and inversion logic, stores the helper result as the current colour and adds the weight to the four running sums.
- `select_colour_weight_and_accumulate`: Chooses a weight from the plane index and mode bits, converts the colour through a helper and adds the weight to four accumulators used for the row totals.
- `store_weight_and_accumulate`: Stores a weight taken from the mode argument, converts the current colour through a helper and adds the weight to the four running totals.
- `set_weight_for_mode_three`: Sets the weight from the mode argument only when it equals three, then converts the current colour through a helper and adds the weight to the four running totals.
- `set_weight_and_accumulate`: Copies the weight argument into the weight variable, converts the current colour through a helper and adds the weight to the four running totals.
- `convert_index_and_accumulate`: Copies a weight into the weight variable, converts the colour index through a helper and adds the weight to the four running totals.
- `set_weight_and_update_totals`: Sets the weight from the argument, converts the colour through a helper and adds the weight to the four running totals, as one of five callers of this totals update.
- `copy_colour_and_accumulate`: Copies a stored colour value from the plane table to the destination, then adds the current weight to the four running sums.
- `compute_region_area_totals`: Multiplies the two stored region dimensions to get an area, falling back to their sum when the product is zero, then resets the four running totals from helper results.
- `query_printer_status_sequence`: Runs a sequence of printer status queries with mode codes zero to three, checking each against the stored region values and returning the first non-zero status.
- `try_printer_codes_in_order`: Calls the printer status function with codes one, two and three in order, copies the first non-zero result to the output and returns it, otherwise returns zero.
- `try_later_printer_codes`: Calls the printer status function with code two and, when that is zero, falls back to code three, returning the first non-zero result.
- `request_printer_code_three`: Calls the printer status function with code three for the stored region and returns the result when it is non-zero, otherwise zero.
- `return_zero_status`: Returns zero unconditionally; it is the empty status result used when no output code applies.
- `advance_column_and_check_status`: Increments the column counter and, when it is a multiple of the row width, queries status codes zero and three, returning zero if neither reports a result.
- `query_status_code_three`: Calls the printer status function with code three and returns its result, or zero when the result is zero.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `emit_region_row_by_row`: Walks the region one row at a time, computing a span per row with a helper, clears the running totals and loops over each column calling the output routine, stopping early when a status call returns non-zero.
- `emit_region_with_status_return`: Repeats the row and column emission loop, storing each non-zero status into the output pointer and returning it as soon as the output routine reports one.
- `advance_region_row_cursor`: Steps the row cursor by the row stride, checks the region limit, and keeps emitting rows through the output routine until the limit is reached or a status call fails.
- `emit_pixel_run_inner_loop`: Calls the output routine once per column position across the current run, then advances the row counter and keeps looping while the row count is short of its limit.
- `advance_column_and_emit_row`: Steps through the columns of the current row calling the output routine for each, then moves to the next row and repeats until the region row limit is reached.
- `reload_row_and_restart_span`: Clears the four running totals, reloads the row width from a helper and restarts the span loop, returning zero once the region limit is passed.
- `advance_span_and_reload_width`: Advances the position by the span width, checks it against the region limit, reloads the width from a helper and restarts the column loop.
- `emit_region_span_pixels`: Walks the region in spans, reloading the span count from a helper for each row, and calls the output routine for every pixel position within each span.
- `emit_span_with_status_checks`: Emits the pixels of each span and checks the status of every output call, returning the first non-zero status and otherwise continuing to the next span.
- `reset_totals_and_emit_column`: Clears the four running totals, walks the column of the current row through the output routine and then advances to the next span.
- `advance_row_and_emit_column`: Increments the row position and emits the column of pixels through the output routine, returning the first non-zero status it reports.
- `emit_columns_until_row_end`: Calls the output routine for each remaining column in the row, then reloads the row width and continues until the region limit is reached.
- `decrement_column_and_restart_row`: Decrements the remaining column count and, once it reaches a non-positive value, advances the row and restarts the span with a fresh width.
- `advance_row_and_restart_span`: Advances the position by the row width, checks the region limit and reloads the span width from a helper before restarting the column loop.
- `scan_region_outer_loop`: Runs the outer loop over the region rows, reloading the row count from a helper and resetting the column counters for each row.
- `emit_rows_while_count_nonzero`: Repeatedly loads a row count from a helper and, while it stays non-zero, emits the columns of each span through the output routine.
- `emit_span_until_status_error`: Emits each span through the output routine and stops with the status code as soon as the output call returns non-zero, otherwise moves to the next span.
- `finish_span_and_advance_row`: Finishes the current span with the output routine, decrements the span counter and advances the row once the span is exhausted.
- `decrement_row_counter_and_emit`: Decrements the row counter, emits the column data through the output routine and reloads the span width when the row count reaches zero.
- `reload_span_and_restart_row`: Advances the span position, reloads the span width from a helper when the row count reaches zero and restarts the row loop.
- `decrement_span_and_advance`: Decrements the span count and, once it reaches zero, advances the position and reloads the count from a helper, returning zero at the region limit.
- `scan_region_per_row_totals`: Walks the region row by row, clearing the running totals per row and emitting the columns of each row through the output routine.
- `emit_rows_with_span_reload`: Emits each row's columns through the output routine, reloading the span width from a helper at each row boundary until the region limit is reached.
- `emit_block_rows_with_totals`: Resets the four running totals for each block, calls the output routine once per column of the block and returns the first non-zero status it reports.
- `emit_block_span_and_advance`: Calls the output routine for each pixel of the current block span, counts the emitted pixels and advances the span position with the block stride.
- `advance_block_span_counter`: Advances the block span counter and steps the position through the block, calling the output routine and returning its status when it is non-zero.
- `decrement_block_and_restart_row`: Decrements the block column count and, when the row is complete, moves to the next row and restarts the span from the stored start position.
- `reload_block_count_and_restart`: Advances the row position, reloads the block count from a subroutine when it runs out and restarts the row loop until the region limit is reached.
- `accumulate_doubling_until_positive`: Repeatedly adds twice the first argument to a running value until it becomes positive, counting the steps, then subtracts twice the second argument and returns the step count.
- `compare_plane_entry_against_value`: Returns whether the given value is strictly greater than a four-bit plane entry looked up from the table, or than fifteen minus the stored value when the mode bit is set.
- `test_plane_entry_below_limit`: Looks up a plane entry from the table indexed by the two low coordinate bits and returns whether it is less than the supplied limit.
- `call_two_printer_routines`: Calls two subroutines in sequence with no arguments and returns without a value.
- `divide_unsigned_long_by_word`: Divides a 32-bit unsigned value by a 16-bit divisor, using a shift-and-subtract loop when the divisor exceeds 16 bits and direct 16-bit division otherwise, returning quotient and remainder.
- `divide_long_remainder_loop`: Runs the restoring shift-and-subtract division loop over sixteen bit positions, leaving the remainder and quotient in the register pair.
- `multiply_longs_to_64bit`: Multiplies two 32-bit values by combining 16-bit partial products into a 64-bit result, with the operand halves swapped in the low word.
- `forward_to_subroutine_call`: Calls a single subroutine and returns, passing nothing else through; it is a thin wrapper around another routine.
- `forward_subroutine_result`: Calls one subroutine and returns the value it produces in the data register.
- `negate_result_when_flagged`: Calls a subroutine to obtain a value and returns its negation when the address-register flag is non-zero.
- `negate_helper_result_when_flagged`: Obtains a value from a subroutine and negates it when the flag register is set, returning the signed result.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `match_printer_name_table_entry`: Initialises the device record, selects a name from the printer's name table by a stored index and compares the requested name character by character against it, returning zero on a match and one-based error code otherwise.
- `replace_printer_name_handle`: Compares a name against the table entry, closes any open printer handle by writing the sentinel value, then opens a new handle for the matched entry.
- `return_error_minus_one`: Returns the all-ones value, minus one, which is the failure sentinel for a printer operation.
- `compare_name_then_replace_handle`: Compares a name string with the table entries and, when it matches, closes the current printer handle and reopens it with the matched device number.
- `reopen_printer_handle_for_name`: Matches the name against the stored printer table and, when it is unmatched, reports an error; otherwise it closes the current handle through the library and reopens the printer.
- `copy_name_and_reopen_handle`: Compares a name character by character while copying it into a buffer, then closes any open printer handle and reopens the printer when the name is found.
- `open_printers_directory_handle`: Locks the DEVS:printers directory through the DOS library, changes the current directory into it and unloads a segment, recording the resulting handle in the device record.
- `return_error_after_two_calls`: Makes two subroutine calls and returns minus one as the error result.
- `release_printer_handle`: If a handle is stored in the device record, calls its shutdown vector and then releases the stored handle.
- `compare_chars_ignore_case`: Folds lowercase letters to uppercase in both characters and returns whether they are equal.
- `return_success_status`: Returns zero, reporting success for an operation that needs no further work.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `invoke_library_vector_call`: Calls a library vector with no arguments and returns, leaving the library state changed by that call.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `stub_return_zero_value`: Returns zero immediately; the routine has no other instructions.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 255 into d0, then returns.

### arp.library

- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 34 into d0, then returns.
- `build_arp_entry_list`: Library code that stores its argument in the record and repeatedly calls a library vector to obtain entries, linking the non-empty ones into a list.
- `initialise_arp_list_header`: Library routine that sets up a list header whose first node points at itself, calls library vectors to lock and allocate, and fills in the record when allocation succeeds.
- `finish_arp_record_setup`: Library routine that stores the allocated pointer, calls library vectors to lock and allocate, fills in the flags and owner fields of the record and then calls the release vector.
- `increment_arp_base_counter`: Library routine that calls a library vector repeatedly until the zero flag is set, then increments a 16-bit counter in the library base.
- `clear_arp_record_field`: Library routine that clears one field of the argument record and calls a library vector through the stored base.
- `allocate_arp_memory_block`: Library routine that calls the exec memory allocator when the requested size is non-zero and stores the block pointer, offset, into the caller's record.

### Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF].adf:vf.obj

- `run_entry_with_interrupt_toggle`: Program entry that clears the master interrupt enable bit, calls one subroutine, sets the master enable bit again, points two globals at a self-referencing table and calls another subroutine before returning.
- `decode_bitstream_copy_run`: Reads bits from a shifted word stream and uses a lookup table to choose between literal bytes and short or long run copies, calling subroutines to refill the stream and copy each run.
- `mix_and_shift_state_word`: XORs the stored word into a global accumulator and returns that word shifted right by one with its top bit set, a per-step state update.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right by the requested count, refilling it from a subroutine whenever it runs empty.

### Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `open_intuition_and_init_display`: Checks for enough memory, opens the intuition library through exec and, on success, initialises the display, DMA and interrupt registers with colour zero and reads the CIA port for button and disk status.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_runtime_state_and_chips`: Opens an exec library, writes the interrupt, DMA and copper pointer registers, waits for the blitter to go idle, clears a block of memory word by word and then calls several subroutines to finish setup.
- `advance_countdown_and_pop_queue`: Decrements a four-step countdown and, when it reaches zero, reloads the count and copies the head word of a queue into three slots, advancing the queue when it is non-empty.
- `initialise_eight_entry_table`: Fills an eight-entry table with pointers that step by a fixed stride and matching word offsets that step by eighteen.
- `fill_mirrored_word_pairs`: Reads a word from a pointer-chained table, writes it and its complement offset by a fixed amount into four paired slots, and advances the pointer, wrapping to the start at a sentinel value.
- `run_twenty_calls_eight_blocks`: Makes twenty calls to a subroutine for each of eight blocks, advancing a pointer by twenty after every block and resetting it when the stored count is exhausted.
- `search_table_and_setup_blit`: Finds the index of a value in a byte table, waits for the blitter to go idle and then programs its registers with mask words, a source address derived from the index and a fixed size.
- `setup_blitter_scroll_step`: Steps a scroll offset down by four, wrapping it to a fixed value and toggling a bit pattern when it goes negative, then waits for the blitter and programs its size and pointers for the copy.
- `run_blitter_copy_loop`: Waits for the blitter to go idle, programs its control and size registers with a constant, then runs eight copy passes that transfer source words to the destination.
- `clear_blitter_control_registers`: Clears or resets specific values within a data structure used to configure hardware blitter operations.
