# Game 2359 (spy vs spy - the island caper (1989)(wicked software)[cr vf][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02359). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902144 top-level bytes in 38 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 25 of 25 units identical (bytes and relocations), 124 raw-word lines |
| Functions named and described | pass, 924 of 924 named (287 author symbols), 924 of 924 described (59 by rule, 232 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1287 of 1287 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 22552 of 22596 instructions identical in start and length, 3 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 182619 |
| code | 85412 |
| data | 178355 |
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
| Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF][a2].adf:vf.obj | 2 | 96 | 4 |
| Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF][a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 238 | 9 |

## Cases handled

- runtime segment an address: 181 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### setmap

- `parse_command_line_arguments`: Parses a command-line argument into individual tokens, stripping surrounding whitespace and handling quoted strings with escape sequences, then stores the resulting token pointers in a table.
- `store_arg_and_call_pointers`: Saves the incoming argument on the stack, then calls a function pointer stored in a global variable when a flag is set, and calls two more function pointers at fixed negative offsets from a register when another flag is set, before restoring and returning the saved value.
- `invoke_two_function_pointers`: Invokes two function pointers located at fixed negative offsets from a register and then returns.
- `store_and_call_pointers`: It stores the result of a call into a global variable and returns early when that value is non-zero.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `propagate_map_update_calls`: It forwards stored global values to a helper routine when corresponding flags are set, then forwards the incoming parameter to the same helper.
- `store_pair_and_invoke_helpers`: Saves its two parameters into global variables, then calls a helper routine with a fixed argument list that includes a global address and the constants 440 and 60, and finally calls another helper with the value 20.
- `resolve_keymap_name_and_open`: Resolves a keymap name from icon tool types, a command-line argument, or a default, converts it to lower case, and opens the console device and keymap resource, reporting an error if either cannot be opened.
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
- `pass_params_to_subroutine`: It calls another routine passing a global variable, the incoming parameter, and the address of an eight-byte stack slot.
- `call_library_function_minus`: Calls a library function through a base pointer held in a global variable, using a fixed offset of minus 348 into its jump table, and returns with no result.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### delete

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_words_then_call`: Copies a fixed count of four-byte words from a location derived from a register into a destination address passed on the stack, then invokes a stored routine pointer.

### addbuffers

- `find_max_and_copy_block`: Walks a linked list of records to find the largest value stored in a specific field, then sets up a stack frame and copies a block of sixteen words from one location to another.
- `copy_four_longwords_then_call`: Copies four longwords from a fixed base offset into a destination pointer, decrementing a counter of fifteen until it underflows, but only when the destination is non-null.

### path

- `build_table_and_setup_return`: Walks a linked list of records to find the largest value in a specific field, then builds a fixed-size table of incrementing values.
- `initialize_argument_buffer`: Copies a fixed block of fifteen words from a global table into the caller's argument buffer when that buffer is not already in use, then invokes another routine.

### ex

- `scan_and_copy_blocks`: Walks a linked list of four-byte entries to find the largest stored value, then, if a flag indicates work is needed, fills a region of memory with an incrementing pattern and copies a block of words from one location to another.
- `copy_word_block_into_argument`: Copies a fixed block of fifteen words from a global table into the memory location pointed to by the argument, then invokes another routine.

### info

- `find_max_and_setup_stack`: Walks a table of linked records to find the largest value in their second field, then stores that maximum along with a run of sequential marker words into a fixed structure.
- `copy_block_then_call`: Copies a fixed 15-word block from a global table into the caller-provided destination buffer, then invokes another routine.

### dir

- `build_marker_block_from_list`: Walks a linked list of records to find the largest value in a specific field, then builds a fixed-size block of sequential marker words and copies two 16-word regions into it.
- `copy_table_and_call`: Copies a fixed block of fifteen words from a global table into a caller-provided buffer, then invokes another routine.

### copy

- `find_max_and_setup_table`: Walks a linked list of records to find the largest value in a field, then stores that maximum along with a block of incrementing marker words into a table.
- `copy_defaults_into_buffer`: Copies a fixed block of fifteen words from a global table into the caller's buffer when that buffer is not already in use, then invokes another routine.

### makedir

- `find_largest_directory_entry`: Scans a table of directory entries to find the one with the largest size field, then builds a fixed-size block of four-byte words and copies it into a destination buffer.
- `copy_block_invoke_routine`: Copies a fixed block of fifteen words from a global table into a destination buffer passed as an argument, then invokes a routine stored in a global pointer.

### list

- `build_command_table`: Finds the largest value in a table of indexed entries, then builds a fixed-size block of sequential marker words and copies two 16-word regions into it.
- `copy_and_call_routine`: Copies a fixed block of fifteen words from a global table into the caller's buffer, then invokes another routine.

### cd

- `initialize_command_buffer`: Walks a table of linked records to find the largest stored value, then sets up a fixed sequence of four-byte entries in a buffer.
- `copy_table_block_and_call`: Copies a fixed block of fifteen words from a global table into the caller's memory when the provided pointer is not null, then invokes another routine.

### genim2

- `initialize_structure_and_copy_block`: Copies a block of longwords from a global data structure into another location, zeroes a second block of words, and then clears and initializes a series of fields and pointers within a large structure.
- `initialize_data_and_copy_string`: Clears and initializes a large block of fields in a global data structure, then copies a null-terminated string from one buffer into another.
- `initialize_slot_counter`: Sets a guard flag and returns early if it is already set or another flag is non-zero, then ensures a counter reaches at least ten before incrementing a second counter.
- `invoke_helpers_and_add_values_2`: Invokes four helper routines in sequence, then reads the value stored at one offset of a structure, adds it to the value at another offset, and writes the resulting sum back to that first location.
- `invoke_helpers_and_add_values`: Invokes four helper routines in sequence, then adds the value at one structure offset to the value at another offset and stores the sum back into the second location.
- `increment_conditional_values`: If the low bit of the value pointed to is set, it increments that value and also increments the value eight bytes later, then returns.
- `process_conditional_callbacks_2`: It calls another routine when a pointed-to value is non-zero, again when a nearby character matches a tab, and once more when the following value is non-zero.
- `process_conditional_callbacks`: If a value at a pointer is non-zero, it calls another routine; it then clears a value at an offset of eight bytes and calls another routine if a value at an offset of four bytes is non-zero.
- `advance_string_list`: Walks through a linked list of string entries, comparing each against a target and advancing to the next record until it finds a match or runs out of entries.
- `advance_string_pointer_2`: Advances a newline-delimited string pointer to the next line, wrapping to the next buffer when the current one is exhausted, and records the starting address of that line.
- `adjust_counter_by_sign`: Sets a flag byte to all ones, returns early if a status byte is zero, then adjusts a counter byte by one depending on whether a register holds a plus or minus sign and records whether the result stays non-negative.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 10 into d1, then returns.
- `decrement_status_byte`: It decrements a byte at a structure offset and stores a flag byte that is set when the new value stays non-negative.
- `update_status_flags`: Increments a byte at a specific offset within a structure and updates a byte at another offset based on whether the new value is zero.
- `clear_status_fields`: Checks several conditions within a structure to select a byte, then clears three specific byte offsets within that structure.
- `parse_text_token`: Parses a text token from a buffer, skipping whitespace and handling colon-delimited sections, then looks up the token in a table and stores its length.
- `copy_variable_length_string`: Copies a variable-length string from one source buffer to a destination buffer, handling both a zero-length case and a loop that transfers the remaining characters.
- `store_offset_in_structure`: Adds a fixed offset to a register value and stores the result into a field within a structure, then returns.
- `store_token_string_and_count`: Clears a flag byte and, when a source string is present, copies it into a newly allocated structure field while advancing pointers and decrementing a running length.
- `update_token_counter_and_store`: Updates a running counter and, when a source string is present, copies it into a newly allocated structure field while advancing pointers and decrementing a length counter.
- `copy_data_to_structure`: If a value in a structure is above a threshold, it initializes a new structure with various fields and copies a string from a source pointer into it, then decrements a counter and advances a pointer by the copied length.
- `adjust_structure_fields`: It calls a helper when a structure value is within a threshold, then adjusts two structure fields by subtracting and adding the helper result and the incoming value.
- `save_registers_and_insert_entry`: Clears two words in a global structure and returns early if both are zero, otherwise saves the caller's registers onto a stack frame, sets a flag byte, and walks a linked list to insert a new entry.
- `save_registers_and_insert_value_2`: Sets a flag byte and ensures a counter meets a minimum value, then saves registers onto a stack frame and searches a short array to insert a value.
- `process_command_stream_with_save`: Initializes a buffer by incrementing its length field and clearing the new slot, then processes an encoded command stream that records entries into an eight-slot table.
- `process_command_stream_2_2`: Parses a byte stream of encoded commands and inserts new entries into a fixed-size table of eight-slot records, dispatching on each command byte.
- `save_context_and_return`: Pushes its current register state onto a stack in global memory and sets a return address so it can resume elsewhere.
- `store_command_byte_3`: Pops a command byte from a byte stream and stores it in a table of eight-slot records, advancing a slot counter and recording the associated value.
- `store_command_byte_2`: Pops a command byte from a byte stream and stores it in a table of eight-slot records, advancing a slot counter and recording associated values.
- `dispatch_from_stack_frame_2`: Pops a dispatch byte from a stack frame, looks up a routine in a table, and calls it with the frame's arguments, restoring the frame afterward.
- `store_command_byte`: Pops a command byte from a byte stream and stores it in a table of eight-slot records, advancing a slot counter and recording associated values.
- `process_command_stream_2`: Parses a byte stream of encoded commands, dispatching on each command byte to add new entries to a fixed-size table of eight-slot records, and when a special marker is found it saves the current registers onto a stack frame and inserts a value into a short array.
- `process_command_stream`: Parses a byte stream of encoded commands, dispatching on each command byte to add new entries to a fixed-size table of eight-slot records or to remove the most recently added record.
- `loop_until_index_zero_then_decrement`: Loops by repeatedly reading a value stored at a fixed offset and using it to index the same location until that value becomes zero, then decrements two stored 16-bit values by two and eight respectively.
- `decrement_table_values`: Subtracts two from a 16-bit value stored at one table offset and eight from a 16-bit value stored at another offset within the same table.
- `save_registers_and_insert_value_4`: Returns early if a flag is already set, otherwise sets that flag, ensures a counter meets a minimum value, saves registers onto a stack frame, and searches a short array to insert a value into the first empty slot.
- `dispatch_from_stack_frame`: Pops a pair of values from a stack frame and calls a routine chosen from a table indexed by a byte, restoring the frame afterward.
- `tiny_movea_rts`: Small routine of 2 instructions that copies a4 to a0, then returns.
- `parse_string_token_digits`: Skips whitespace and delimiter bytes, then scans a table of paired marker and value bytes to locate a matching marker.
- `process_string_token`: Examines a byte and, when it falls between the digit and punctuation ranges, copies a run of characters up to a delimiter into a buffer at a fixed location while recording the copied length.
- `empty_routine`: This routine is empty and performs no operations.
- `scan_and_copy_string`: Scans a byte sequence forward to find the first character that is not a lowercase letter, then checks whether it is a dollar sign.
- `jump_jumptable_indexed_byte`: Loads a routine address from a jumptable indexed by the negated contents of a byte and jumps to it.
- `read_and_swap_identifier`: Reads a four-character identifier from a pointed-to location, swaps its byte order when it does not match a known marker, and returns the resulting value.
- `store_hash_compare_flags`: Stores a negated flag byte at a fixed offset through a pointer, then stores another flag byte whose value depends on whether the input byte equals a hash character and whether a distant memory location is zero.
- `process_flag_byte_chain`: Reads a byte at a fixed offset and inspects a nearby flag byte, then branches into a recursive routine depending on whether the flag is set.
- `check_byte_and_flag_then_call`: Reads a byte from a fixed offset and inspects a nearby flag byte, then branches into a recursive routine depending on whether the flag is set or the byte matches one of three specific values.
- `append_byte_sequence_to_buffer`: Appends a two-byte value and a following word to a buffer, advances the buffer pointer by two words, and stores the updated pointer back in the referenced location.
- `shift_table_byte_left`: Loads a byte from a table using an index, shifts it left by one bit, and stores the result back into the same table entry through a pointer.
- `read_byte_check_flag_branch`: Reads a byte from a fixed offset and inspects a nearby flag byte, then branches into a recursive routine depending on whether the flag is set.
- `check_byte_and_flag_then_call_2`: Reads a byte from a fixed offset and inspects a nearby flag byte, then branches into a recursive routine depending on whether the flag is set.
- `call_and_return_3`: Calls another routine and then returns.
- `copy_byte_block_and_store_pointers`: Copies a block of bytes from a source pointer into a destination pointer using a loop, records the source pointer and a length value into nearby memory, then stores a computed difference between two pointers.
- `call_and_return`: Invokes another routine and then returns without modifying any data.
- `call_and_return_2`: This routine invokes another routine and then returns without modifying any data.
- `push_registers_into_stack_frame`: Pushes the caller's registers onto a stack frame in a global structure, records a return address, and increments a counter, then walks a small table to store a value into the first free slot.
- `save_registers_and_insert_value_3`: Sets a flag byte, ensures a counter meets a minimum value, saves registers onto a stack frame, and then searches a short array to insert a saved value before returning.
- `store_constants_in_struct`: Stores a fixed word value at a fixed offset within a global structure, then writes a previously saved register value into the following four-byte field, and returns.
- `process_chunk_trailer_3_2_2`: Increments a counter in a global structure and, when the counter is zero, repeatedly invokes another routine until a matching or terminating marker is found.
- `process_chunk_trailer_3_2`: Increments a counter and, when the input is zero, repeatedly invokes another routine until a matching or terminating marker is found.
- `process_chunk_trailer_3`: Parses a chunk trailer by scanning a newline-delimited buffer for a magic marker, comparing it against expected values, and when a match is found popping a stack entry and adjusting counters and pointers.
- `detect_chunk_trailer`: Detects a chunk completion marker and, when a matching trailer is found, pops an item from a processing stack and adjusts the associated counters and pointers.
- `process_chunk_trailer_2`: Pops a chunk trailer from a processing stack, matching its type against the current chunk and adjusting the stack pointers, counters, and active chunk pointer accordingly.
- `process_chunk_trailer`: Detects a chunk magic string and, when a completion marker is found, pops an item from a stack and adjusts the associated counters and pointers.
- `pack_chars_into_64bit_2`: Reads characters from a buffer, discarding spaces, tabs, newlines and dots, and packs the kept characters into a 64-bit value built from a lookup table.
- `pack_chars_into_64bit`: Reads characters from a buffer and skips spaces, tabs, newlines and dots, packing each kept character into a 64-bit value built from a lookup table, storing the byte through a pointer and advancing both pointers until the source counter reaches one.
- `combine_byte_with_value`: Combines a byte of all ones with a 32-bit value from a table and returns the assembled 64-bit result.
- `init_field_then_store_result`: Initializes a word field in a global structure to a fixed value, then invokes another routine and stores the returned value into the immediately following four-byte field, and returns.
- `store_byte_from_register`: Copies the low byte of a value from a register into a pointed-to location, then returns.
- `process_linked_list_pair`: Pops a pair of values off a linked list, decrements a counter, and either runs a long fixed sequence of routine calls or inserts a new entry into the list depending on the popped values.
- `insert_into_linked_list`: Sets a flag when it is clear, forces a counter to be at least ten, then walks a linked list of short pairs to insert a new entry before the first zero slot.
- `reset_fields_then_dispatch`: Clears two global fields and sets a counter to ten, then runs a long fixed sequence of routine calls before branching on stored values, invoking a routine through a computed pointer when a flag is set and finally returning a byte read from a global field.
- `advance_string_pointer`: Advances a string pointer to the next newline, wrapping to the next buffer when the current one is exhausted, and records the line start.
- `empty_routine_2`: This routine is empty and performs no operations before returning.
- `initialize_and_lookup_token`: Fills a block of words in a data structure with 65535, then repeatedly checks a global value against a table, storing an index offset for each entry.
- `lookup_and_store_token`: Looks up a record in a table by comparing a value against its entries, or falls back to a lookup table indexed by that value.
- `store_16bit_value`: Stores a 16-bit value held in a register into the memory location pointed to by a pointer, then returns.
- `clear_and_jump_jumptable`: Stores a zero at a table slot, then jumps to the routine address held in another slot of the same jumptable.
- `process_and_queue_differences`: Skips its work when the input byte is a newline, tab, space, asterisk, or semicolon.
- `store_difference_and_list`: Computes a difference and stores it, then updates several counters and, when a list is present, writes a 16-bit value into a free slot of that list.
- `save_and_insert_value`: Sets a flag byte, raises a counter to a minimum, saves registers onto a stack frame, then scans a zero-terminated short array to insert a value and record a return address before incrementing a counter.
- `scan_buffer_for_dot`: Calls a routine that classifies a token, then walks a buffer counting characters until it finds a dot, adjusting a stored length and returning a flag set by the first call.
- `parse_string_token`: Looks up a character in a lookup table to classify it, then scans a buffer for a delimiter to extract a substring, storing the result length and a pointer to the extracted data in a destination structure.
- `copy_string_skipping_delimiters`: Copies characters from a source string into a destination buffer, skipping delimiters and whitespace, and stores the resulting length and a flag byte into the destination structure.
- `chain_lookups_while_flag`: Checks a flag field and, while it is set, chains a series of lookups storing each result into successive fields until one returns zero.
- `search_linked_list_matching_byte`: Walks a linked list of structures and compares a byte field against a target value, then compares the associated data until it finds a matching entry or reaches the end.
- `invoke_routine_twice`: It calls another routine twice in a row and then returns.
- `initialize_and_copy_structure`: Clears a field in a structure, then calls another routine repeatedly.
- `store_value_and_offset`: Stores an incoming value into one global field and that value plus sixteen into a nearby global field, then returns.
- `store_value_and_offset_2`: Stores an incoming value into a global field and that value plus sixteen into a nearby global field, then returns.
- `store_snapshot_and_increment_counter`: Copies a running total into a snapshot field, stores a return address, and increments a counter, branching to a shared exit when a specific value is reached.
- `return_when_flags_set`: Checks two condition flags and returns early when either is set, otherwise falling through to the shared exit that increments a counter and stores a flag byte.
- `check_flag_and_match_string`: Returns early without changing anything when a flag in the global data block is already set.
- `reset_memory_pointers`: Writes a zero word at a fixed offset in a global structure, points a pointer at that location, and clears a nearby flag byte.
- `append_two_byte_sequence`: Writes a two-byte value and a following word into a buffer, advances the buffer pointer by two words, and stores the updated pointer back into the global structure.
- `write_pattern_and_advance_buffer`: Reads a byte from a fixed offset and writes a two-byte pattern, a flag byte, and that value into a buffer via a stored pointer, advancing the pointer by two words and zeroing the next entry before returning.
- `seed_instance_and_append_entries`: It writes a byte to a fixed offset in a per-instance data block, then repeatedly invokes a routine that sets a flag and appends an entry, and finally copies a byte from that block into a stack slot before returning it.
- `conditional_dispatch_then_run`: Calls another routine and returns its result when a status flag is set, otherwise clears two global fields, sets a counter to ten, runs a long sequence of routine calls, branches on stored values, invokes a routine through a computed pointer when a flag is set, and finally returns a byte read from a global field.
- `set_flag_and_append_entry`: Sets a flag in the global data block and raises a counter to at least ten before returning.
- `save_registers_and_insert_value`: Sets a flag byte, ensures a counter meets a minimum value, saves registers onto a stack frame, and then searches a short array to insert a saved value.
- `emit_null_terminated_strings_2`: Checks a flag that returns early when already set, then sets it and ensures a counter is at least ten before incrementing another counter.
- `append_string_to_buffer`: Appends a string to a growing buffer, growing the buffer with another routine when the current one is full and inserting a newline when needed.
- `advance_buffer_and_append`: Advances a buffer pointer by a fixed amount, appends a newline when the buffer is empty and the preceding bytes are not already newlines, updates a stored pointer and counter, and returns a fixed value when a flag is clear.
- `append_newline_to_buffer`: Advances a buffer pointer by a fixed amount, appends a newline character when the buffer is empty and the previous byte is not a newline, and updates the length counter.
- `follow_pointer_conditional`: Follows a pointer held in a structure, and when the pointed-to data is present performs a sequence of conditional calls based on a flag, a character value and a following field.
- `clear_field_and_call_routine`: Checks a field in a structure and, when it is nonzero, clears it and calls a routine before returning.
- `process_conditional_callback`: Calls a routine when a pointer is nonzero, invokes a callback when a byte is empty or matches a structure field, and recurses when the next pointer is nonzero.
- `execute_conditional_calls_2`: Checks a pointer field and, when it is set, calls the routine it references, invokes a function pointer, and makes a further conditional call based on a value four bytes past the result.
- `execute_conditional_calls`: Checks if a pointer held in a register is non-zero before calling another routine, then performs a second conditional call based on a value at an offset from that pointer.
- `call_pointer_if_set`: Checks a pointer field at a fixed offset and, when it is set, calls the routine it references, then performs a further conditional call.
- `test_bit_and_call`: Checks if a specific bit is set in a byte at an offset from a pointer and calls another routine if it is.
- `conditional_sequential_calls`: Verifies a pointer at a fixed offset is non-zero, then calls the routine it points to, makes an unconditional call, and finally a conditional call based on a value at another offset.
- `make_sequential_calls`: Checks whether a pointer is set and, when it is, makes an unconditional call, a second unconditional call, and a conditional call based on a value four bytes past the result.
- `test_bit_and_call_2`: Checks if a different specific bit is set in a byte at an offset from a pointer and calls another routine if it is.
- `unlink_list_node`: Removes a node from a linked list by updating pointers and calculating a new offset, or initializes a new node if a specific byte is zero.
- `append_or_search_record_slot`: Computes a value from stored fields, then either appends data to a node and advances its size or walks a linked record structure searching for a matching slot, returning either the computed value or a stored pointer depending on the path taken.
- `reset_list_node_fields`: Calls a routine, then clears a linked list node by writing a pointer into two fields, zeroing its first two fields, and storing a stack value and register into later fields.
- `find_or_advance_record_slot`: Compares an input value against a stored field and, when they differ, searches a linked record structure for a matching slot.
- `release_record_slot`: Reads a pointer stored in a fixed slot of the global data block and, when it points to a live record, writes a zero into that record's field.
- `set_slot_to_minus_one`: Stores the all-ones longword into a fixed slot of the global data block and returns zero.
- `process_null_terminated_strings`: Walks through one or more null-terminated strings, skipping a fixed number of them and then processing any remaining characters until the end is reached.
- `emit_null_terminated_strings`: Walks through a fixed number of null-terminated strings and calls a routine once for each byte of every string until the terminator is reached.
- `handle_char_output`: Handles character output by emitting a newline when a flag is clear and otherwise advancing a step counter, emitting characters with a per-character delay and comparing against a limit before resetting counters.
- `return_space_character_2`: Loads a value into a register, discards a temporary result, and returns the space character code.
- `tiny_moveq`: Small routine of 1 instruction that loads 32 into d1.
- `transform_and_return_pair`: Performs a transformation on two input values and returns the combined result.
- `gather_values_and_combine`: Invokes another routine twice to gather two values, indexes a global table using a masked register value, runs the routine once more, and combines the results into a single value with certain byte positions cleared.
- `index_table_and_combine_values`: Invokes another routine twice to gather two values, then indexes a global table using a masked register value.
- `index_table_and_combine`: Indexes a global table using a masked value from a register, calls another routine with the result, and combines the returned values into a single value.
- `lookup_and_transform_pair`: Indexes a global table using a masked value from a register and calls another routine with the result.
- `rotate_and_switch_value`: Rotates a 32-bit value in register D1 left by four bits across eight iterations, then branches through a jump table using a flag that becomes set once a nonzero nibble is encountered.
- `convert_number_to_ascii`: Walks through a table of word-sized entries, repeatedly subtracting each value from a register until it falls below the entry, then calls another routine to process the remainder.
- `process_table_with_jumptable`: Walks through a table of word-sized entries, repeatedly subtracting each value from a register until it falls below the entry, then invokes a jumptable routine based on the remainder.
- `copy_word_to_indexed_slot`: Copies a single word from the source pointer in register A2 into the indexed location computed from registers A4 and A5.
- `process_record_pointer`: Calls two routines unconditionally, then checks a pointer stored in a fixed slot of the global data block.
- `emit_period_to_text_buffer`: When a flag is clear, waits while a buffer counter equals 130, then writes a period character into the text buffer and advances the counter.
- `return_space_character`: Calls two routines around a third call and returns the constant 32, the space character code, as its result.
- `repeat_action_count_times`: When a flag byte is nonzero, calls a routine once per unit of a count read from a record field, then calls three more routines.
- `init_mask_marker_and_ff_run`: If a pointer is set and the value it refers to is nonzero, writes a 127 marker byte into a fixed buffer followed by 128 bytes of 255, then calls a routine.
- `match_record_prefix_bytes`: Compares bytes from a stored record field against a comparison buffer while they match, stopping at a mismatch or when a count runs out; it then calls a follow-up routine if a pointer field is set.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `release_pending_block_size`: If a pending block pointer is set, clears it, subtracts the block's stored size from a running total and calls a routine through the library base.
- `remove_last_list_node_or_reset`: Walks a singly linked list to its last node and unlinks it, returning the result of a routine.
- `set_default_pointer_field`: Writes the fixed value 10240 into a 16-bit record field, then stores the result of a routine into a second pointer-sized field.
- `release_head_block`: If a list head pointer is set, frees its block by subtracting the block's stored size from a running total and calling a library routine, after first running a routine on its contents when they are nonzero.
- `release_block_at_argument`: Releases the block passed in, running a routine on its contents when they are nonzero, subtracting its stored size from a running total and calling a library routine.
- `initialize_text_output_state`: Clears a set of flags and counters, points two buffer pointers at the start of an output area, sets the width and line-limit words to 132 and 60, sets a reset marker and a stop value, then calls a routine.
- `flush_pending_output_if_set`: When a pending output value is nonzero, calls three routines to process it.
- `consume_pending_value_and_notify`: Takes a nonzero pending value, clears it, and when it differs from a stored reference value calls a callback through the library base.
- `buffer_char_or_flush_when_full`: Stores one character in the text buffer if there is space and returns.
- `reset_text_buffer_pointer`: When the text buffer write pointer is not at its base, resets it to the base and calls a routine.
- `tick_text_output_per_frame`: Advances a frame counter.
- `advance_timed_character_step`: Clears a per-character tick counter and advances a step counter; when the step reaches its target it calls a library routine and resets the step counter to minus one.
- `write_timed_or_buffered_char`: In timed display mode, emits a character with a per-character delay and handles newlines through the same step counters.
- `emit_null_terminated_string`: Calls a routine once for each byte of a zero-terminated string until the terminator is reached.
- `draw_status_line_text`: Draws the status line: it emits decimal digits of a 16-bit counter when the flags allow, then walks the current text string, expanding tab characters into repeated routine calls until a newline.
- `stub_empty_return`: The routine has no effect and returns immediately to its caller.
- `stub_field_guard_return`: Checks a state word against a fixed value and returns in both cases, so it does nothing.
- `append_source_twice_with_comma`: Copies one zero-terminated string to the destination, writes a comma, then copies the source string again after it.
- `walk_hunk_sections_rounding_size`: When the mode allows and a section list exists, runs a fixed reset sequence of routine calls; otherwise walks each AmigaDOS hunk section, rounds its size up to a multiple of four and stores the start and size fields.
- `stub_flag_guard_return`: Returns without action when a flag byte at a fixed record offset is nonzero, and also returns without action when it is zero.
- `stub_flag_guard_check`: Returns without action; the body only tests a flag byte at a fixed record offset and then returns.
- `return_empty_routine`: Empty routine that returns immediately without performing any operation.
- `record_value_in_free_slot`: In one state, latches a flag, raises a minimum delay, bumps a counter and, when a list is present, writes the current 16-bit value into the first free slot of a short table before calling routines.
- `stub_state_guard_return`: Returns without action; it only tests a state byte and then returns.
- `add_count_and_copy_bytes`: Adds the byte count to a running total at a fixed field, then copies that many bytes from the source to the destination buffer.
- `emit_space_or_period_char`: When a state byte equals two, emits a space character through routines; otherwise writes a period into the text buffer as the period routine does.
- `fetch_and_store_when_unset`: When a handle field is zero, calls a routine and stores the supplied value into that field; otherwise returns the stored result.
- `snapshot_running_total`: Clears a byte, increments two counters, and copies the running total into a snapshot field; when a flag is set and another flag is clear, it calls two routines first.
- `clear_pending_count_field`: Sets a 32-bit counter field at a fixed record offset to zero.
- `apply_hunk_relocations`: When a flag is set and a hunk list exists, walks the entries and adds computed offsets to the longword references of entries with a nonzero size; otherwise it walks the list and runs routines.
- `release_handle_and_clear`: If a handle field is nonzero, passes it to an AmigaDOS library routine and then clears the field to zero.
- `step_count_down_by_block`: Repeatedly subtracts a fixed block size from an unsigned count, clamping at zero, and calls a routine on each step until the count reaches zero.
- `lookup_name_table_index`: Extracts a word up to a space, newline or tab, then scans a length-prefixed table of entries comparing each against it through a character map, returning the index of the match or minus one.
- `empty_return_routine`: An empty routine that returns immediately without executing any instructions or touching any data.
- `accumulate_and_copy_bytes`: Adds a register value to a field at a fixed offset, then copies a run of bytes from one pointer to another for the count given in register D1.
- `find_next_entry_index`: Walks a list of entries, incrementing a byte counter taken from a record field on each step, and returns the counter value when it wraps to zero; it returns minus one when the list ends.
- `write_named_record_entry`: For a matching record, writes a longword length rounded up to a multiple of four, a longword address, the length-prefixed name bytes, zero padding to a four-byte boundary and a zero terminator longword.
- `write_padded_name_entry`: Writes a given longword value, an address derived from the record, the length-prefixed name bytes, zero padding to a four-byte boundary and a terminating run of zero bytes.
- `store_field_when_key_matches`: When a key byte matches, waits until a 16-bit counter reaches four and then stores the record's 32-bit value into the output longword.
- `do_nothing`: The routine contains no instructions and returns immediately without reading or writing any data or registers.
- `store_field_when_flags_allow`: When the key byte matches and the record's flags permit it, waits until a 16-bit counter reaches four and stores the record's 32-bit value into the output longword.
- `copy_name_bytes_padded_to_longword`: Copies a length-prefixed name into the output and pads it with zero bytes to a multiple of four, calling a routine first when a counter is too small for the name.
- `store_longword_after_wait`: Waits until a 16-bit counter reaches four, then stores a 32-bit value at the output pointer.
- `dispatch_if_not_flag_set`: Compares an unmodified 16-bit value against the constant 128 and, when they differ, branches into another routine before returning.
- `noop_triple`: The routine executes three instructions that perform a no-op operation, leaving the registers and memory unchanged.
- `run_three_setup_routines`: Calls three routines in sequence and returns, with no other work.
- `loop_while_marker_byte_matches`: Calls a routine and then loops, calling routines and checking a link field, while a byte compares equal to a fixed marker value.
- `empty_routine_3`: This routine contains no instructions and performs no operations.
- `wait_until_minus_one_result`: If a link field is set, calls a routine repeatedly until its result equals minus one, then calls one more routine.
- `set_link_pointer_past_header`: Stores into the link field of a fixed record the address ten bytes past that record's header.
- `init_record_pointer_past_header`: Sets a pointer field of the current record to the address ten bytes beyond the record start.
- `find_matching_entry_in_list`: Walks a table of eight-byte entries in a list, returning the position of the first entry whose leading bytes match the requested pair (and whose word also matches unless the first byte is one), or minus one when the list is exhausted.
- `store_text_entry_and_reset`: Stores a 32-bit value and a padded copy of a byte string in a fixed buffer, records the padded length, then runs a sequence of routine calls and a callback to reset the display state.
- `append_string_copy_with_comma`: Copies the zero-terminated string at one pointer into a destination, appends a comma, then appends a second copy of the same string.
- `flush_text_and_reset_buffer`: Calls a library routine through the base at a fixed offset, then clears the character count and resets the write pointer to the start of the output buffer.
- `invoke_library_then_routine`: Calls a library routine at a fixed offset through the base stored in the state record, then runs a second routine.
- `initialize_program_state`: Null-terminates a command string, opens the DOS library and allocates the program state block, returning distinct error codes if either step fails.
- `parse_command_line_options`: Parses the command-line text byte by byte into program options and flags, and prints the referenced error messages when the command line is invalid or the listing device cannot be opened.
- `check_path_has_directory_separator`: Scans a name for the last slash or colon, returning zero when one is present and minus one when the name has no directory part.
- `finish_timed_step_when_active`: When the display flag is set and the step counter is nonnegative, calls a library routine and, if it returns false, a routine; then resets the tick and step counters.
- `finish_timed_step_unconditionally`: When the step counter is nonnegative, calls a library routine and resets the tick and step counters to start a new step.
- `handle_ctrl_c_signal`: Reads the task signal state through the exec library and, when the Ctrl-C signal bit is set, flags the state byte and clears the signal, then runs a routine if a buffer is present.
- `print_fixed_message_after_cleanup`: Runs a cleanup routine, then emits a zero-terminated message from a fixed string one character at a time through a routine.
- `call_library_functions_in_order`: Calls one library function at a fixed offset from the base, then a second library function with a low-word argument from the same base.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies 32 to (a3)+, then returns.
- `allocate_sized_memory_block`: Allocates a memory block through the library large enough for the requested size plus an eight-byte header, stores a shifted pointer value in the caller's longword and zeroes the header's second field.
- `allocate_block_and_track_bytes`: Allocates a memory block of the requested size plus a four-byte header through the library, stores the size in the header and adds it to a running total of allocated bytes.
- `terminate_name_with_marker`: Writes a zero byte at the end of a length-prefixed name, calls a routine, then writes the marker value 11 at that same position.
- `skip_to_colon_in_name`: Advances through a zero-terminated string until it reaches a colon or the terminator, leaving the pointer on the colon.
- `try_open_path_candidates`: Builds candidate file paths by joining a name with each entry of a search list, tries each through library calls and returns the first that opens.
- `attempt_file_open_sequence`: Runs a fixed sequence of library calls to probe a file; when the probe succeeds it performs a follow-up library call and returns its result, otherwise it returns zero.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `check_library_result_nonnegative`: Calls a library function at a fixed offset and returns zero when its result is nonnegative, otherwise returns that result.
- `wrap_single_routine_call`: Calls one routine and returns, adding no other work.
- `call_library_function_36`: Calls the library function at offset minus 36 from the library base held in the state record and returns nothing.
- `call_library_function_42`: Calls the library function at offset minus 42 from the library base held in the state record and returns nothing.
- `append_name_to_path_buffer`: Appends a name to a path buffer when the buffer is empty or the name starts with a dot, then calls a library function and returns zero on success or minus one on failure.
- `call_library_when_argument_set`: When the argument is nonzero, calls the library function at a fixed offset from the library base.
- `call_library_function_66`: Calls the library function at offset minus 66 from the library base and returns.
- `query_available_memory_when_clear`: When a flag byte is clear, calls the exec library routine that reports available memory.

### spy2

- `START_CODE`: Jump stub: passes control straight on to START.
- `DUMMY_SPRITE`: Spins reading the vertical beam position register until it reaches a computed scanline and then until it passes that line.
- `ATARI_COPY`: Copies a stream of four-word groups into four bitplanes laid out 8000 bytes apart, writing one word per plane per iteration for 4000 words.
- `COLOUR_COPY`: Copies sixteen 16-bit colour words into a palette table, shifting each value left by one bit.
- `LOAD_FILE`: Opens the DOS library and stores its base, then opens a file handle through it and calls two more DOS routines.
- `SETUP_SCREEN`: Selects the two screen buffers, fills the copper list with bitplane pointer words, programs the display window, fetch and bitplane control registers, then enables the DMA channels.
- `WHICH_PLANES`: Writes four bitplane addresses spaced 8000 bytes apart into the copper pointer table, splitting each address into high and low words.
- `BLIT`: Waits until the blitter is idle, then loads its first and last word masks, source and destination pointers, modulo, minterm and size registers from the supplied arguments.
- `WAIT_BLIT`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `SWAP_SCREEN`: Waits for the blitter to finish, then enters an empty endless loop; no screen pointers are written in this code.
- `CLEAR_SCREEN`: Zeroes 8000 bytes of memory, the size of one full bitplane.
- `MYINT2`: Keyboard interrupt handler: records the incoming key in the key matrix, waits a short fixed time, clears the serial handshake bit and sets the keyboard interrupt request.
- `ADD_MATRIX`: Converts a raw keycode into a byte index and bit position in the key matrix, setting the bit when the key is pressed and clearing it when released.
- `DOWN_STROKE`: Clears one bit in the key matrix at a given byte offset and bit position.
- `INIT_KEY`: Sets the four longwords of the key matrix to all ones.
- `INKEY`: Tests one bit of the key matrix for a keycode, returning zero when the bit is set and minus one when it is clear.
- `SCAN_JOY`: Decodes left, up and fire bits for both joystick ports from the port status words, then polls the keyboard for the keys mapped to the left direction.
- `HANDLE_MOUSE`: Updates mouse state by calling the mouse routine twice, then records each of the two mouse buttons as pressed when its bit in the port status is clear.
- `DO_MOUSE`: Decrements three direction counters, compares the new horizontal and vertical position bytes with the previous ones, arms a 20-count timer on each axis that moved and stores the new position.
- `LAST_MOUSE0`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `LAST_MOUSE1`: Allocates two 32 kilobyte screen buffers, clears each of them, and allocates a third block as the back buffer.
- `KLEFT1`: Small routine of 1 instruction that merges 0 into d0.
- `ALLOCATE_MEMORY`: Requests a 32K memory block and stores the returned base into the onscreen buffer slot, then repeats this for the twoscreen and background buffers.
- `CLEARD0`: Zeroes 8000 longwords, a block of 32000 bytes, starting at the pointer held in a data register.
- `FIND32K`: Requests a memory block from the exec library and returns its pointer; if the allocation fails the routine loops forever.
- `DECO_PIC`: Loads a picture file, copies sixteen palette words into a table, then decompresses run-length-coded picture data into three bitplanes of 8000 bytes each, treating a repeat marker byte of 205 as an escape.
- `DOS_BASE`: Forbids multitasking through the exec library, calls the sound start routine, then seventeen times triggers the sound step, sets the music switch and waits for a timing flag to clear and set again; it then re-enables multitasking.
- `DOS_HANDLE`: Invokes a referenced handler, then plays a sound and waits for a status flag to toggle while decrementing a loop counter eight times, before invoking another handler and clearing a register field.
- `ONESCREEN`: Calls a routine stored in a global variable, starts the sound system, then runs a loop of sixteen iterations that each request a new sound, enable the music switch, and spin until a status flag toggles on and off before returning.
- `TWOSCREEN`: It invokes a screen setup routine and then plays a short sound sequence by repeatedly requesting new sounds while waiting for the audio channel to become ready.
- `SCREEN2`: Waits for a music switch flag to clear, then repeatedly toggles a sound counter while polling a status bit, and finally stores a register value and a small constant into a global variable before invoking another routine.
- `SCREEN1`: Waits for a music switch to activate, then repeatedly polls a status flag until it is set and then until it is clear, decrementing a counter each pass.
- `COPPER1`: Invokes a previously registered copper routine, then plays a sound and runs a ten-iteration loop that waits for a music-switch flag to toggle on and off.
- `COPPER2`: Chains a previously stored routine, then repeatedly plays a sound while waiting for a flag bit to toggle, and finally invokes another stored routine.
- `COPPER_LIST2`: Invokes a routine from a fixed slot and starts the sound hardware, then loops ten times waiting for two flag states to toggle.
- `E_KISS`: Plays a short sound effect and then runs a looping animation sequence that alternates between waiting for a flag to be set and waiting for it to clear, repeated several times, before invoking another routine.
- `E_RUMBLE`: Plays a rumble effect by cycling a sound through a short timed sequence, pausing while a flag bit is set and then waiting for that bit to clear.
- `TEST`: It starts a sound effect, then repeatedly plays music notes while waiting for a status flag to flip on and off.
- `START_SOUNDS`: Fills an 8192-byte noise table from a pseudo-random generator, clears four audio channel records and their envelope pointers, initializes the music player and sets the music interrupt handler, then enables the audio interrupt and DMA registers.
- `GO_SOUND`: Fills a sound channel record with the sample pointer, length and priority values, clears its position and sets its active flag.
- `HIGH_PING`: Starts the high ping sound effect by calling the shared sound-queue routine.
- `MEDIUM_PING`: Starts the medium ping sound effect by calling the shared sound-queue routine.
- `LOW_PING`: Starts the low ping sound effect by calling the shared sound-queue routine.
- `DROP_PING`: Starts the drop ping sound effect by calling the shared sound-queue routine.
- `TAKE_PING`: Starts the take ping sound effect by calling the shared sound-queue routine.
- `MEDIUM_BEEP`: Starts the medium beep sound effect by calling the shared sound-queue routine.
- `BANG`: Starts the bang sound effect by calling the shared sound-queue routine.
- `OTHER_SMACK`: Starts the other smack sound effect by calling the shared sound-queue routine.
- `GRAVESOUND`: Starts the gravesound sound effect by calling the shared sound-queue routine.
- `BOOM`: Starts the boom sound effect by calling the shared sound-queue routine.
- `BURY`: Starts the bury sound effect by calling the shared sound-queue routine.
- `SPLASH`: Starts the splash sound effect by calling the shared sound-queue routine.
- `RUMBLE`: Starts the rumble sound effect by calling the shared sound-queue routine.
- `FEET`: Starts the feet sound effect by calling the shared sound-queue routine.
- `KISS`: Starts the kiss sound effect by calling the shared sound-queue routine.
- `SUB`: Starts the sub sound effect by calling the shared sound-queue routine.
- `HANDLE_SOUNDS`: Calls the request handler up to four times in sequence, each call only while the previous one left the zero flag set, to process pending sound requests.
- `HANDLE_REQUEST`: Finds the first pending sound request among four queue slots, marks it active, and calls the sound routine selected by its low byte from a table; returns minus one when nothing is pending.
- `NEW_SOUND`: Adds a sound request to the queue unless an equal one is already queued, applying a priority flag and enabling the master sound switch when the request asks for it.
- `MYINT6`: Audio interrupt handler: reads mouse state, counts down a music delay and plays the next music step when the music switch is on, updates the four channel envelopes, services queued sound requests, and then acknowledges the interrupt.
- `CHANGE_ENVELOPE`: Advances one audio channel's envelope through its table of segments, updating its current volume and period and writing them to the channel registers.
- `X8912`: Updates the pointer, length and period registers of the music audio channels from the current wave, frequency and volume tables, and sets up the temporary music voice when requested.
- `PSGW`: Stores one byte into the sound register shadow buffer at a given offset, representing a write to the programmable sound generator.
- `INITMUSC`: Sets the mixer register to a fixed value, writes three register shadow bytes and switches both music voices off.
- `PLAYMUS`: Plays one music step on each of the two voices by selecting the voice, enabling it and outputting its register values in turn.
- `VOICEISOF`: Clears a voice's gate flag, resets its sequence position and loads the next note entry from the sequence table.
- `NEXTSEQ`: Advances a voice's sequence pointer to the next three-byte note entry, loading its duration, pitch and control bytes.
- `VOICISON`: Counts down a voice's note duration and, when it expires, fetches the next sequence entry, looks up its duration and computes the note period and control values for that voice.
- `DEGATEIT`: Clears the gate byte of a voice record.
- `CHECK2`: Writes the mixer and volume shadow registers for a voice according to its note status bits and the selected channel.
- `CALCNOTE`: Gets the adjusted note byte from the note modifier and applies a pitch step to the 16-bit result, returning the adjusted value.
- `OUTVC1`: Writes the first voice's shadow register bytes to the sound chip through the output routine.
- `OUTVC2`: Writes the second voice's shadow register bytes to the sound chip through the output routine.
- `SNDOUT`: Writes three shadow register bytes to the sound chip.
- `NOTEMODS`: Adds a signed pitch offset to the low nibble of the note byte with wrap-around carry into the upper bits, saving the original byte and storing the combined result in the output register.
- `RESET`: Writes three fixed shadow registers to reset the sound chip.
- `SAVE_INT2`: The listing shows a large body that does not match this entry's few instructions, so no behaviour can be read from it; the entry is treated as a decompiler overlap.
- `TEMP_MUSIC`: Allocates memory, initializes the keyboard, starts the sound system, saves interrupt state, sets up the screen, copies two blocks of data from a back buffer into a screen buffer, sets the screen modes, and then loops forever calling a game routine until an abort flag is set.
- `REG7`: Saves the current interrupt vector, installs two interrupt handlers, copies a screen buffer into another buffer twice, and then loops forever polling a flag while repeatedly initializing and running the main game loop.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `MUSINF_1`: Initializes the game's music and sound system, installs interrupt handlers, and copies a large block of data between two screen buffers.
- `init_game_system`: Initializes the game system, sets up the screen and sound, installs interrupt handlers, and copies screen data between buffers.
- `MUSINF_3`: Initializes the game's sound system and then runs its main loop.
- `MUSIC_DELAY`: Initializes the music subsystem, saves and restores interrupt vectors, copies a screen buffer, and then runs the game loop until aborted.
- `MASTER_ENABLE`: Initializes the game system by allocating memory, setting up keys and sound, then installing interrupt handlers and swapping screen buffers before entering the main game loop.
- `START`: Allocates buffers, resets keyboard and sound state, sets up the display and loads the graphics, installs the interrupt handlers, then loops through mode selection, play initialization and game steps until an abort flag is set.
- `READALL`: Clears the first screen buffer, then runs a long series of read and grab routines that load the back, land, map, spy and rocket picture files into memory and store their data into the screen buffers.
- `MYINT3`: Interrupt handler: increments a frame counter, clears the busy flag and acknowledges the interrupt request bits.
- `GRAB`: Calls the buffer save routine four times in a row.
- `GRABOBJ`: Small routine of 2 instructions that copies 20 to d3, then subtracts 1 from d3.
- `GRABOBJ1`: Calls the buffer fill routine in a loop controlled by a 16-bit counter taken from a register, ending when the counter wraps to minus one.
- `GRABLAND`: Calls the short buffer fill routine twenty times in a row.
- `GRABMAP`: Calls the buffer save routine in a loop controlled by a 16-bit counter taken from a register, ending when the counter wraps to minus one.
- `PLAYINIT`: Resets the menu, flash and refresh flags, picks a bullet count of three to six from a random value, copies the current level's map and terrain tables into working buffers, sets both players' energy to 100 and clears the trap list and state fields.
- `PLAYINIT0`: Copies the level's map and terrain tables into working buffers, sets the choice pointers and rocket positions, resets energy, the trap list and per-player state fields, then calls the per-player setup routines.
- `GETA4`: Small routine of 3 instructions that clears d1, then copies (a4)+ to d1, then returns.
- `XGET_STICK`: Looks up the stick X and Y plane offsets for the current level in two tables and stores them in the first player's stick fields.
- `MY_VERY_OWN_STACK`: The listing shows a large body that does not match this entry's single instruction, so no behaviour can be read from it; the entry is treated as a decompiler overlap.
- `XSTICK1`: Clears several global flags, copies a block of bytes from one table to another, then reads the joystick buttons and direction and uses them to update movement state, a counter, and sound effects.
- `YSTICK1`: Clears several game-state flags and copies a block of memory from one table to another, then repeatedly reads the joystick triggers and movement until a button is pressed or a counter runs out, adjusting a sound level and playing sounds as needed.
- `PARMS`: Clears the demo and automatic-play flags, copies the back buffer to the second screen, reads trigger and joystick input, then runs the selection drawing and paradrop routines.
- `DO_GAME`: Runs the play loop until an abort flag is set: resets death and timing counters, then for each frame updates both players' busy state, moves each spy, checks meetings, redraws the movement and swaps the screens.
- `READPIX`: Decodes the picture into the back buffer, copies its bitplane data into screen memory, copies its colour words, then sets a colour register value.
- `STUFFIT`: While a count remains, picks a random map cell and, where the terrain code is outside the reserved ranges and the neighbouring cells are empty, writes the object value and increments a counter.
- `STUFFSPY`: Picks random start positions for the two spies, converts them to window coordinates and repeats until their horizontal separation reaches a minimum, then sets up the volcano colours, redraws the buffer and plays a sound.
- `SETUP_VOLCANO`: Stores three fixed colour values for the volcano palette entries in two places each.
- `COPY_COLOURS`: Selects one of two three-colour sets depending on the low bit of a parameter and copies it to the top-green value and the colour registers.
- `COL1A`: Calls the random value routine and returns its result as the first colour value.
- `COL2A`: Calls the random value routine and returns its result as the second colour value.
- `COL3A`: Calls the random value routine and returns its result as the third colour value.
- `RNDER`: Calls the random value routine and returns with its result.
- `RNDER2`: Calls the random value routine and returns its result as a 32-bit value.
- `GETRAND`: Repeatedly picks random coordinates within the slab grid until the chosen map cell is empty, then returns a pointer to that cell.
- `SETXY`: Adds a fixed offset to a 32-bit value, then stores its quotient and remainder when divided by 96 as two 16-bit coordinates.
- `CLEAROUTCRAP`: Sets six timer and fuel fields to minus one.
- `L_TENMINS`: It computes a screen position and depth value from map coordinates, points a buffer at a sprite address table, and then draws a sprite along with a pair of hands for the active window.
- `L_ONEMINS`: Selects a window view and, when the current coordinates match a map position, computes a depth-based offset and calls a sprite routine and a hands-drawing routine to render that view.
- `L_TENSECS`: Given a map coordinate pair, it computes screen plot positions and a depth value, points a buffer at a frame table, and then draws a sprite followed by the hands.
- `L_ONESECS`: It computes screen coordinates and a depth value from a map position, sets up a sprite buffer and width, then calls a sprite-drawing routine and a hands-drawing routine.
- `L_S1FUEL`: Selects which of two window views to draw based on a window index, then converts a map coordinate into screen coordinates and a depth value, loads a fuel sprite address, and calls the sprite plotter and hand-drawing routines to render the fuel gauge.
- `L_S2FUEL`: Selects which of two split-screen maps is active and, when the player's map coordinates match that screen's window, computes a depth-based vertical offset and a horizontal position, points a buffer at a fuel sprite table, and draws the fuel sprite plus the character's hands at that location.
- `SLABX`: It converts a map coordinate into screen coordinates and a depth value, then stores the resulting buffer pointer, width, plot position, and depth into global variables and calls the sprite and drawing routines to render the hands.
- `OLD_BUSY`: Computes a screen position and depth value from a map coordinate, sets up a drawing buffer and width, then calls a sprite-plastering routine and a hands-drawing routine to render the correct spy depending on which window is active.
- `SAVE_INT3`: It maps a screen coordinate pair to a map cell and, when the cell matches the current spy's position, computes a depth-based value, sets up a drawing buffer and calls the sprite and hand-drawing routines to render that spy.
- `DRAWSPY`: Draws each spy's sprite into the second screen when it lies in the visible window, computing the plot position from its map coordinates, depth and altitude, then calls the sprite and hand drawing routines.
- `DRAWHANDS`: Draws the held-item hand sprite for the player selected by the state in the registers.
- `DRAWBUTTONS`: Draws six item buttons by calling the single-button routine six times in a row.
- `ONEBUTTON`: Draws one item button at the horizontal position given in a data register, using the on-state image when the flag register is non-zero and the off-state image otherwise.
- `SWAPSCREEN`: Jump stub: passes control straight on to SWAP_SCREEN.
- `DRAWTIME`: Refreshes the four clock digits for minute tens and ones and second tens and ones.
- `HLINE`: Thin wrapper that calls the horizontal line routine and passes the registers through unchanged.
- `METER`: Advances each player's energy count on a frame-counter schedule while that player is alive and not in a blocked state, then redraws both energy bars when they are flagged for refresh.
- `FLASH`: Draws three pieces for the selected player's hand, using the parity of the frame counter and the hand index to choose the piece set.
- `DRAWPIECE`: Draws one piece sprite using the alternate blitter combine mode and then restores the default mode.
- `DRAWSEL`: When the selection flag is set, draws four marker characters and skips one of them, the index being taken from a frame-counter bit in one branch.
- `COMPARETWICE`: Compares a value with a stored word and, if the first word matches, with the word after it, and writes the value into the stored slot when it differs.
- `RSPRITER`: Draws a sprite into four bitplanes, row by row across the given width and height.
- `RBACKER`: Writes a background image into four bitplanes, clearing each destination word and then combining the shifted source words into it when the horizontal position has a sub-word offset.
- `blit_shifted_column_rows`: Loop body used by the background blitter: copies or shift-merges four bitplane words per column, clears the next destination row when a row finishes, and returns once the last row is done.
- `RSAVE_BUFF`: Copies a rectangle of four-plane screen words into a memory buffer, storing the four plane words of each column consecutively.
- `RDRAW_BUFF`: Copies a rectangle of buffered four-plane words back onto the screen, the inverse of the save routine.
- `RSTUFF_BUFF`: Saves a rectangle of screen words into a buffer that begins with a header holding the word count, width and height.
- `RSTUFF_BUFF_SHORT`: Saves a rectangle of screen words into a buffer that begins with a header holding the word count, width and height.
- `RSPRITER_AM`: Draws a sprite with the Amiga hardware blitter, setting the edge masks, shift amount, modulos and source and destination pointers from the sprite header and screen position.
- `SPRITER`: Thin wrapper that calls the masked sprite drawing routine with registers unchanged.
- `SPRITER_AM`: Thin wrapper that calls the hardware-blitter sprite routine with registers unchanged, so callers use a single entry point for hardware sprite drawing.
- `BACKER`: Thin wrapper that calls the background blit routine with registers unchanged.
- `X`: Sets one bit in a byte just before the address in the address register, the bit being chosen by the low three bits of a data register, and then calls the character drawing routine.
- `WIDTH`: Sets a single bit in a byte addressed through an address register, using a shift count taken from a data register, then advances a character stream.
- `BUFFER`: Sets a single bit in a byte addressed through a pointer and then advances a character index.
- `SCREEN`: Sets a single bit in a byte addressed through a pointer, then advances a character counter.
- `MINTERMS`: Sets the bit at position (D7 mod 8) in a byte addressed by A2, then calls NEXTCHAR.
- `PUTCHAR`: Thin wrapper that calls the character drawing routine and returns.
- `NEXTCHAR`: Draws one eight-row character glyph into four bitplanes.
- `CALCAD`: Contains only a return, so calling it has no effect.
- `PLOTPOINT`: Sets one pixel in four bitplanes by clearing that pixel's bit in each plane through a mask, then combining in the colour bits for each plane.
- `LEFT_RIGHT`: Reads the leftward-movement flag and the low bit of a data register, then returns.
- `UP_DOWN`: Tests the leftward-movement flag and returns in both outcomes, so the decompiled code shows no visible effect.
- `DRAW_LINE`: Draws a line between two points by taking the absolute horizontal and vertical distances, recording the direction of each axis, and then stepping one pixel at a time with a running error term.
- `GO_LEFT`: Fills a horizontal run of pixels across four bitplanes between a start and an end column.
- `HORLINE`: Fills a horizontal run of pixels across four bitplanes between a start and an end column.
- `POINT`: Packs the screen position held in the data registers into one value and keeps only the low four bits for the bit position within the word.
- `GET_RANDOM`: Advances three random seed words with additions, a subtraction and a bit rotation, leaving the next pseudo-random value in the first seed.
- `READ_TRIGS`: Reads the fire trigger for each player according to that player's control mode, using either the joystick fire signals or the key-check routine, and stores all-ones or zero in that player's trigger word.
- `MOVE`: Moves each spy every frame, choosing joystick, computer or random direction according to the control mode, and exits early in demo mode when a key is pressed.
- `set_spy_direction_fixed`: Sets a direction value to a fixed number and writes a constant through a pointer register.
- `set_spy_direction_and_store`: Sets the spy direction to a fixed value and writes a constant through a pointer register.
- `CHECKMEET`: Checks whether the two players share a vertical window band and are horizontally close, within forty columns of each other.
- `CHECKMEETEND`: When the value in the data register is greater than one, sets both saved buffer Y positions to all-ones so they are treated as invalid.
- `SETTEMPS`: Copies the selected player's window X and Y and map X and Y into the shared spy position variables.
- `DIRFIX`: Chooses the animation frame image for the selected player from the current spy direction, then advances that player's frame counter and wraps it after four steps.
- `TEST_OPTIONS`: Counts down a delay flag and, once the flag reaches zero, runs the option test routine.
- `TEST_M`: After a key check, resets a delay counter and toggles the music flag, setting or clearing three audio level variables to match.
- `JOYMOVE`: Scans the joystick and waits through key and delay loops, setting the abort flag when a key is held.
- `SETWINDOW`: Loads the shared spy window position from one of the two player windows.
- `GETWINDOW`: Copies the shared spy window position back into the chosen player's window.
- `LITTLE_DELAY`: Burns time by counting a 16-bit counter down through tens of thousands of steps, then returns the data register unchanged.
- `FEET_FLAG`: Draws one frame: fuel gauges, flashing hand pieces and selection markers, then the landscape and objects for each player's view, the buttons and the clock.
- `DRAWMOVE`: Redraws the fuel readout, flashes the current piece, decrements the selection counter and redraws the land and objects for both spies, then redraws the buttons, time readout and meter, plays a sound and draws the ropes.
- `DRAWLAND`: Draws the landscape for one player's window.
- `DRAWLAND2_0`: Clears the working region, draws the landscape slab rows, then programs the blitter to copy the result to the screen with the sub-word shift taken from the window position.
- `DRAWLAND3_0`: Starts a blit using a mode word, waits for it to finish, and draws the land slabs.
- `ONE_SLAB`: Draws one landscape slab by computing its source address from the slab index and the current background buffer, then programming the blitter with full masks, the destination pointer and the slab size.
- `DRAWSLAB`: Draws a landscape slab and, for certain slab indices and only at addresses aligned to eight bytes, draws it a second time to cover the wider slab region.
- `DRAWBACK`: Copies a column of background buffer data into the back-buffer, two long words per row over the given width, stepping the destination by a fixed row stride.
- `DRAWOBJ`: Draws the map objects visible in a player's window by scanning a grid of four rows and twelve columns of map cells.
- `DRAWLAND5_2`: Draws the window cover sprite at a fixed horizontal position, using an upper or lower vertical position according to the player index.
- `DRAWROPES`: Draws the rope line for each player whose pending-line flag is set, then clears that flag.
- `DRAWFUEL`: Redraws a player's fuel gauge when its value has changed.
- `CLEAR_WINDOW`: Clears four plane regions of the screen window with blitter fills, waiting for the blitter to finish between each one.
- `SEL_FLAG`: Runs one frame of a player's action logic.
- `BUSY`: Runs the busy-state logic for a spy.
- `BUSY4_A`: Places the selected player's held item at that player's map position.
- `DELETE_TRAP`: Looks up a map cell in a table of up to one hundred trap entries.
- `ADD_TRAP`: Finds the next usable map cell from a starting cell by skipping over tiles of two kinds, moving forward and backward on alternating steps, and then records the cell's address and value in the first free slot of the trap table.
- `COUNTER1`: Handles the map and menu state for one player.
- `clear_player_flash_and_timers`: Clears both players' flash counters and moves a pending delay value into the chosen player's delay counter.
- `BUSY6_4`: Moves a pending shared delay value into the chosen player's delay counter, clears the shared delay, and then clears that player's state counter.
- `SETSAFE`: Sets the chosen player's safe counter to four.
- `KILLDOUBLE`: Records a time value twenty counter ticks before the current counter value in the chosen player's double-time slot.
- `GETPOS`: Small routine of 2 instructions that shifts d1 right by 6, then shifts d2 right by 4.
- `GETPOS1`: Converts a player's grid position into a screen X value and a screen address offset, using a cell size of sixteen units and a fixed origin offset with an eight-byte row stride.
- `SETSTATE`: Sets the chosen player's state counter to the value in a register, after first moving any pending shared delay into that player's delay counter.
- `GETBTIME`: Both branches return immediately, so the routine has no effect in the decompiled code.
- `SHOWMAP`: Draws the map image for the current level at a fixed horizontal position, with the vertical position taken from a register plus a fixed offset.
- `ABSD2`: Contains only a return and performs no action.
- `DRAWROPE`: Works out the two end points of a player's rope line from its plot position, the difference between two stored positions and the spy's direction.
- `CLEANTRAIL`: Fills the six trail entries of both players with all-ones, marking both trails as empty.
- `DELAY1`: Waits in a loop that swaps screens and redraws a background of horizontal bands, cloud sprites and figure sprites, with the figure images chosen from a state value.
- `SUBCLOUDS`: Draws a cloud sprite at a fixed horizontal origin plus an offset, with the offset reduced by twice the first input when the second input is above six.
- `COPYBACK`: Copies one full screen block from the first screen buffer into the second, moving four long words per step across two thousand steps.
- `PARADROP`: Draws the parachute sprite twice when a demo timing value is large enough, then waits in a long countdown loop.
- `GETMODE`: Runs the title menu loop.
- `DRAWITEMS`: Draws seven item frame outlines, each made of four line segments whose positions step with the item index.
- `SELECTITEM`: An empty routine that returns immediately without touching any data or registers.
- `DRAWLINE`: Thin wrapper that calls the line drawing routine and passes the registers through unchanged.
- `RSPRITERBOTH`: Draws a masked sprite into the first screen buffer.
- `RDRAW_BOTH`: Copies a buffered four-plane rectangle into the first screen buffer.
- `RCLEARBLOCK`: Fills a rectangle of screen words so that the first plane is set to all-ones and the other three planes are cleared, row by row across the given width and height.
- `PX1`: Chooses a steering target for a spy each frame.
- `PX2`: Chooses a steering target for a spy each frame with the same logic as the player-one routine.
- `BRAINMOVE`: Sets the computer-controlled spy's movement direction for the frame.
- `GETD0D1`: Sets the spy direction from a four-bit input mask, checked in a fixed order: bit one gives direction three, bit two gives direction four, bit three gives direction one, bit four gives direction two, and no bit gives zero.
- `BRAINBUSY1`: Clears the frame flag for one of the two spies when its flag is zero and the incoming selector selects it, then writes a fixed value into a frame counter pointed to by an address register.
- `reset_player_two_brain_timer`: Clears player two's frame counter and writes a fixed timer value through the address register.
- `RANDMOVE`: With a small random chance on each call, picks a new spy direction from two random bits and sets a random nudge value between zero and seven, with the nudge going to player one when a random value is zero and to player two otherwise.
- `FIND_A_MOUND`: Scans a small window of map cells around a position, starting from the cell addressed by the two masked position words.

### begin

- `jump_to_init`: Jump stub: passes control straight on to INIT.
- `E_EXPLODE`: Plays the explosion effect: it starts the sound system, then runs eleven cycles of dive and explosion setup, waiting on a timer status bit in each cycle.
- `E_DIVE`: Plays the dive effect with the same eleven-cycle sequence as the explosion routine, waiting on a timer status bit in each cycle and restoring the saved interrupt afterward.
- `TEST`: Test routine that starts the sound system and runs the same eleven-cycle dive and explosion sequence as the effect routines, with the interrupt save and restore around it.
- `START_SOUNDS`: Sets up the sound system: fills an 8192-byte random table, clears the music state, and sets up four sound channel records with envelopes.
- `SHOOT`: Starts the shooting sound effect on the fourth audio channel.
- `SHOOT2`: Sets the fourth audio channel's length register to a fixed value of 4096.
- `EXPLODE`: Starts the explosion sound effect on the fourth audio channel.
- `DIVE`: Starts the dive sound effect on the third audio channel using a triangle waveform table, a short period, a length of 2130 and the dive sequence handler.
- `MYINT6`: Interrupt handler that runs on each tick while sound is enabled.
- `advance_music_and_envelopes`: Advances the music sequence and updates the envelopes of the four channels, resetting the music delay to three.
- `CHANGE_ENVELOPE`: Advances one channel's volume envelope by one step.
- `X8912`: Updates the sound chip's channel registers from the current music state.
- `DO_ENVEL1`: When its restart flag is set, reloads the envelope level and pitch for one channel from its table, then updates that channel's sound registers from the music state and clears the flag.
- `XNOISE1`: Updates a noise source: it sets the noise buffer and pitch from the music state, resets the envelope when the restart flag is set, and then writes the channel's tone and volume registers before clearing the flag.
- `XDONE1`: Writes one channel's pitch and volume registers from the music state, choosing tone or the other voice from the mixer bits, and clears the channel's restart flag.
- `DO_ENVEL2`: When the restart flag is set, reloads the envelope level and pitch for channel two from its table and resets the envelope counters, then clears the flag.
- `XNOISE2`: Updates the second noise source from the music state, setting the noise buffer, its pitch and the restart state as the first noise routine does, and then clears the restart flag.
- `XDONE2`: Clears the restart flag of the channel whose state block is addressed by the address register.
- `PSGW`: Writes one byte to a sound chip register.
- `INITMUSC`: Sets the sound chip's mixer register to enable tone output and disable noise on the three voices, writes it through the chip register routine, then silences the voices.
- `PLAYMUS`: Plays one music step for each of the two voices in turn: it selects the voice, updates its note state, and writes its registers to the sound chip.
- `VOICEISOF`: Silences a voice by clearing its gate and resetting its state, then loads the first sequence entries of its note data.
- `NEXTSEQ`: Moves a voice on to its next sequence entry.
- `VOICISON`: Runs one tick of a voice.
- `NEXTNOTE`: Starts the next note on a voice when the current one ends.
- `NEXTNOTO`: Steps the voice's note counter and sequence position, then looks up the duration and calculates the note value for the sound chip.
- `CONTSONG`: Continues the current note.
- `finish_note_with_check`: Clears the gate and, when the note's control bits are nonzero, runs the channel check and computes the note value for the voice.
- `check_and_store_note`: Runs the channel check and computes the note value, then stores the resulting note and the data register in the voice state.
- `DEGATEIT`: Clears the gate flag in the voice state block so that the voice is no longer gated on.
- `CHECK2`: Chooses the sound chip mixer setting for the current voice, depending on whether it is channel zero or one, and writes the noise or tone settings for the note type.
- `SNAREA`: Sets the mixer for a snare-style noise on the first voice, sets its gate level and writes the chip registers.
- `CHANNELB`: Sets the mixer and volume for the second voice, choosing a tone or noise pattern from the note type, and writes the chip registers.
- `SNAREB`: Sets the mixer for a snare-style noise on the second voice, sets its gate level and writes the chip registers.
- `CALCNOTE`: Calculates the sound chip note value by applying the note modifier and adjusting the octave bits, then returns the combined value.
- `OUTVC1`: Thin wrapper that calls the shared sound chip output routine for the first voice.
- `OUTVC2`: Thin wrapper that calls the shared sound chip output routine for the second voice.
- `SNDOUT`: Writes three sound chip registers for one voice in sequence, through the register write routine.
- `NOTEMODS`: Applies a pitch modifier to a note byte.
- `NOTEMOD5`: Applies a second pitch offset to a note byte by adding the complement of the current note to the stored value, adjusting the octave bits on carry, and storing the combined value in the note register.
- `NOTEMOD6`: Combines the low nibble of the note input with the octave bits saved earlier and stores the result as the note register value.
- `XXMOD5`: Adjusts a note by the low three bits of the input, adding their complement to the note value and moving the octave bits down one step when there is no carry.
- `XXMOD1`: Stores the note input combined with the saved octave bits as the note register value.
- `RESET`: Resets the sound chip by writing three register values through the register write routine.
- `INIT`: Start-up routine.
- `BLIT_ME_HONEY`: Combines one picture block into the screen through a mask.
- `PIXEL_PIECE`: Calls the piece blit routine sixteen times in a row, stepping through the piece data on each call.
- `BLIT_ME_HONEY_PIECE`: Blits a piece of picture data across four planes.
- `MOVE_THE_JETS`: Copies the first screen buffer to the second, then animates two rows of jets across the screen by stepping their horizontal position in fixed increments until the right edge.
- `DO_RESTORE_OF_JET`: Restores the background behind a jet by copying a block of rows across all four planes from the second screen buffer into the first, at the saved horizontal position.
- `SHOW_JET`: Waits for the raster beam position, restores the background behind the previous jet image, then draws the next jet animation frame into the first screen buffer.
- `WAIT_FOR_RASTER`: Busy-waits until the video beam position reaches the target line given in the data register, allowing a short window after it.
- `BLOW_A_HOLE`: Advances the hole index with wrap-around at eight.
- `MOVE_A_HOLE`: Draws a hole pattern across four rows and three columns of screen words, combining the pattern bits with the background under a mask.
- `ATARI_COPY`: Copies a picture from its interleaved source into the four bitplanes of the screen, moving four words per step across four thousand steps and writing each plane to its own offset.
- `COLOUR_COPY`: Converts sixteen palette entries from the source colour table into the screen's colour register format, shifting each value left by one and setting the low bits to a fixed pattern.
- `LOAD_FILE`: Opens the operating system library and a file, storing the library base and the file handle for later reads.
- `SETUP_SCREEN`: Sets up the two screen buffers, copies the display list into the copper list, points the bitplanes at the screen, and writes the display window, fetch and bitplane control registers, then enables display DMA.
- `WHICH_PLANES`: Writes the four bitplane pointers into the copper list entries, each plane offset by eight thousand bytes from the previous one.
- `BLIT_A_SPY`: Blits the spy sprite with the blitter, setting the combine mode once and then running the second blit pass three times.
- `BLIT`: Writes the blitter's mask words, control values, modulo and source and destination pointer registers from the values passed in registers, then waits for the blitter to become idle.
- `BLIT2`: Writes the blitter's source, destination and size registers from the values passed in, then waits for the blit to finish.
- `WAIT_BLIT`: Busy-waits until the blitter busy flag in the DMA status register clears.
- `SWAP_SCREEN`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `CLEAR_SCREEN`: Clears a full screen buffer by writing zero to 2000 blocks of four words each.
- `ALLOCATE_MEMORY`: Allocates six 32K memory blocks and stores their addresses for the load screen, the two screen buffers, and the title, jet and credits graphics.
- `RESCIND_MEMORY`: Releases the six memory blocks obtained at start-up by calling the release routine six times.
- `FIND32K`: Allocates a 32K memory block from the system and enters an endless loop if the allocation returns nothing.
- `LOSE32K`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `DECO_PIC`: Loads a picture file and unpacks its run-length data into the picture buffer.

### port-handler

- `dispatch_request_records_by_type`: Initialises a record and then loops over request records, dispatching each one by its type code and linking or unlinking it in a list.

### disk-validator

- `check_bit_mask_table_entries`: Writes a two-digit number into a text buffer, then loops over a table of bit masks and checks each entry in turn through a called routine.

### ram-handler

- `copy_memory_and_jump`: Copies memory through the operating system copy routine and then transfers control through a jump table that the listing does not recover.

### serial.device

- `empty_stub_2`: The routine contains no instructions; it is an empty stub that performs no operations.
- `initialise_serial_unit_record`: Sets up a device unit record.
- `check_flag_and_call_handler`: The routine tests a byte at an offset from a pointer in A1 against a mask and, if the test passes, calls a handler routine obtained from a table via A6.
- `clear_flag_and_call_handler`: Stores a constant into a flag byte of the request, then calls a handler through a library vector when the request's flag bits are clear.
- `disable_then_reenable_interrupts`: Clears a set of interrupt enable bits, calls the routine that follows, then sets those bits again through the interrupt enable register.
- `remove_request_from_queue`: Removes a request record from one of two linked queues, decrements the matching queue count and clears the request's flag bits.
- `unlink_request_and_clear_flags`: Unlinks a request record from its queue, decrements the queue count and clears the active flag bits.
- `unlink_flagged_request`: When the request's flag bit is set, unlinks it from its queue, decrements the queue count and clears the active flag bits.
- `remove_request_from_unit_queue`: Removes a request from the queue it belongs to.
- `unlink_completed_request`: When a request's flag bit is set, unlinks it from its queue, decrements the queue count, sets its state flags, and calls the library completion routine through the library vector.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `close_device_unit_and_unlink`: Sets a flag bit on the unit when its counter field is nonzero, calls a series of library vectors to finish the device work, then unlinks the unit record from its list.
- `shutdown_unit_via_library_vectors`: Calls a sequence of library vectors through the device base, including a pair of repeated calls, copies a stored word from the unit record, then unlinks the record from its list.
- `queue_request_and_signal`: Clears a count field of the request and, when the request has data, links it onto a pending list, clearing its active bit if the queue is busy.
- `transfer_ring_buffer_into_request`: Takes the next request from the pending list or the free pool, then moves bytes from a ring buffer into the request's buffer one at a time.
- `copy_serial_bytes_until_stop`: Copies bytes from a ring buffer into the current request until the buffer is empty or the position reaches its end, stopping early on a stop-character match.
- `copy_serial_byte_to_request`: Stores one received byte into the current request buffer, advancing the buffer and count fields, and stops when the request count or a stop character is reached.
- `receive_bytes_into_request`: Loops over incoming bytes, storing each in the current request buffer and stopping on a stop-character match or at the end of the request.
- `receive_bytes_with_clear_mask`: Clears a bit of the input value according to the unit's mask bit, stores bytes into the request buffer as the other receive routines do, and takes the next pending request when one completes.
- `stub_return_only_routine`: Has an empty body, so calling it does nothing.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `finish_receive_and_set_status`: Sets the request's status code from its error bits and clears the error flags.
- `set_request_status_and_drain`: Sets the request's status code from the flag bits and clears the error state, then drains bytes from the buffer into the current request.
- `set_status_code_and_drain`: Sets the request's status code to one of two values depending on a flag bit, clears the request's count and state flags, then runs the same byte transfer loop used by the other receive routines.
- `serial_receive_interrupt_handler`: Interrupt handler for received serial data.
- `handle_received_serial_byte`: Handles one received serial byte.
- `store_serial_byte_with_status`: Stores a received byte in the receive buffer when the flag bits allow it, setting the completion status and advancing the buffer position.
- `store_byte_or_flag_error`: Stores the received byte in the buffer when its high flag bit is clear, otherwise it sets the error status and completion flags.
- `store_byte_if_flag_clear`: Checks a flag bit to decide whether the received byte counts as an error, sets the completion status when it does, stores the byte in the buffer and calls the completion routine through a library vector.
- `flag_request_complete_via_library`: Sets the completion flag on the unit and, when the current request is unset, records the request.
- `store_byte_in_receive_buffer`: Stores a byte in the receive buffer and advances the buffer pointer and count, then calls the completion routine through a library vector.
- `signal_receive_complete`: Sets the unit's completion field to the current buffer position when that field is zero, runs the overflow routine, then calls the completion routine through a library vector.
- `update_serial_status_flags`: Copies one status byte of the unit into another and clears a flag bit in the unit control byte.
- `set_serial_status_flags`: Counterpart of the previous status routine: copies a status byte, sets a flag bit in the unit control byte and sets a bit in the global status word when a control bit is set.
- `clear_serial_pending_byte`: Stores zero into one byte of the unit state block and returns.
- `start_serial_transfer_request`: When the request-active bit is clear, it marks the request active, resets a count, sets a buffer pointer offset back by ten bytes and a length of nine, then calls a library routine through the device base.
- `complete_serial_transfer_request`: Clears the busy bit of a unit state byte and, if the request was busy and the global status count bits are zero, marks completion, stores the request pointer when none is set and calls a handler from the unit's routine table.
- `receive_serial_data_byte`: Reads the serial data register through an internal routine, masks it and stores received bytes in a buffer while advancing the write pointer and count.
- `mark_serial_transfer_pending`: Sets the pending-transfer bit of the unit state byte, stores the current buffer pointer into the request slot when it is empty, and calls a handler from the unit's routine table.
- `queue_serial_io_request`: Clears the request's status field and, if a length is set, either makes the request the active one when no transfer is running or links it at the tail of the waiting list and increments the queued count.
- `handle_serial_interrupt`: Tests a status byte for a pending flag and, if a receive buffer holds a character, invokes a handler when the flag mask matches, otherwise sets a flag bit and reconfigures a buffer pointer.
- `return_without_action`: Empty routine that only returns.
- `store_carriage_return_marker`: When the global status word has the flag bits in range set and bit 3 set, writes the decimal value 13 (carriage return) into a byte of the request block.
- `feed_serial_transmit_bytes`: Feeds bytes from the active request buffer into the transmit data register, advancing the pointer and count, honouring a mask for parity-style bits.
- `send_next_serial_byte`: Resets a retry counter to decimal 200, sends the next buffer byte through the global status word with an optional extra bit, advances the pointer and count, and calls the completion handler when the buffer end is reached.
- `transmit_serial_byte_masked`: Checks a count against a limit and, when the transmit condition holds, writes the next buffer byte masked by a register value into the global status word.
- `send_serial_byte_conditional`: Calls an internal routine that is not decompiled meaningfully, then writes the next buffer byte to the global status word with an extra bit only when the condition flag is clear, advances pointer and count, and completes the request when the buffer end is reached.
- `dequeue_serial_io_request`: When requests are queued, takes the first request off the waiting list, decrements the queued count, marks it busy and stores its buffer pointer.
- `set_serial_ready_flag`: Writes the value 1 to the global status word and returns.
- `publish_serial_pending_flags`: Sets the global status word to 1, clears the pending flag byte of the unit and combines the unit flag word into the global word.
- `initialize_serial_unit_defaults`: Clears the unit status fields and writes default configuration values into the unit block, including a packed word, a value of 9600, a buffer size of 512 and a zero-initialised pair of fields.
- `complete_idle_serial_request`: Calls an internal routine and, if the request's flags do not indicate a pending or busy state, calls a handler from the unit's routine table.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `poll_serial_request_state`: Sets a pending bit when a secondary request pointer is present, calls an internal routine when another pointer is set, and calls a handler from the unit table when a status bit is clear.
- `reset_serial_request_flags`: Clears a request-pending bit in the unit flag byte, calls an internal routine when a pointer is set, and writes the ready value to the global status word before calling a handler when the status bits are clear.
- `enable_serial_interrupt_bits`: Writes the interrupt-enable setup into the global word, calls an internal routine, then sets the enable bit and bit 0 and calls a handler when the request status bits are clear.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `drain_serial_request_list`: Repeatedly removes the first entry from a linked list and calls an internal routine with it until the list is empty.
- `return_empty_stub`: Empty routine that only returns.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `save_serial_line_status`: Reads the CIA port register, masks it and sets a bit when two unit flags are set, then stores the result and the unit state byte into the request.
- `queue_or_start_serial_request`: Clears a request flag and, if a control bit is set and the unit already has active or queued requests, appends the request to the waiting list and increments the queue count.
- `start_serial_request_if_idle`: Calls an internal routine, then makes the returned request active if its status byte is zero and writes the in-progress global word.
- `clear_active_serial_request`: Calls a handler from the unit table and clears the active request pointer.
- `finish_serial_request_if_ready`: Clears a request-busy bit and, if it was set with an active request whose type is decimal 10, sets a completion bit, clears the active request, writes the global word and calls a handler.
- `start_serial_transfer_with_length`: Sets the global status word to 1, clears a counter, stores the argument value and a length of nine, then calls a library routine through the device base.
- `update_serial_control_flags`: Sets or clears the control flag bit according to the request's bit 7 and adjusts the unit flags when a related status bit is set.
- `clear_serial_control_flag`: Clears the control flag bit and applies the same idle-or-pending check as the flag update routine: when the unit is idle it runs the internal routine sequence with fixed global values, otherwise it marks the request status as one, then calls a handler when status bits are clear.
- `reenable_serial_interrupt_bits`: Writes the interrupt setup value to the global word, calls an internal routine, restores the enable bit and calls a handler from the unit table when the request status bits are clear.
- `validate_serial_record`: Validates a serial device record by checking that its identifier fields contain printable characters and its clock value is within range, then copies those fields into the device's default state.
- `compute_serial_channel_mask`: Builds a 16-bit channel mask by shifting a bit selected by a register field, doubling it when a mode value is two and shifting once more when a flag is set.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_serial_error_status`: Stores decimal 5 into the request status byte and returns.
- `configure_serial_divisor`: Computes a divisor from a clock value and stores it into a register word that feeds the serial hardware, clearing a mode bit and, when a backspace request is pending, setting control bits.
- `write_serial_period_divisor`: Divides one register value by another, stores the 16-bit quotient as the unit divisor and writes it into the global word that feeds the serial period register.
- `open_serial_device_unit`: Allocates a unit block through a library call, opens the resource names misc resource and serial device, then initialises list heads, buffer pointers and default status values.
- `initialize_serial_device_tables`: Sets two status bytes to decimal 5, stores pointers to two internal lists in the device block, calls an internal routine and stores two handler routine addresses with their returned values.
- `fill_serial_request_block`: Fills a request block with a back-pointer, the decimal value 16, a pointer to the global status word and two saved register values.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `initialize_serial_device_request`: Initialises two request fields of the device block with decimal 5 status bytes and list pointers, calls a library routine through the device base and sets the global status word.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `return_zero_result`: Returns zero with no other effect.

### gb

- `self_xor_memory_locations`: XORs two memory locations with themselves, writing the zero result back to each of them.

### parallel.device

- `update_parallel_status_flags`: Modifies a byte in a device register block and updates a status word based on hardware flags.
- `update_parallel_transfer_state`: Clears a status byte and sets the flag bit when a line bit is set.
- `dispatch_parallel_request_type`: Calls an internal routine when the request is not yet marked, then for request type two sets a global bit and clears a status bit, and for type three clears a global bit and calls a handler unless a flag is set.
- `process_parallel_type_three`: Handles a request of type three: clears a global status bit, calls a handler when the flag bit is clear and not already marked, then marks the request and calls a handler from the unit table.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `dispatch_parallel_request_status`: Sets the request status byte to decimal 253 and then follows the same type dispatch as the other parallel request routine, calling handlers from the unit table.
- `queue_or_start_parallel_request`: If the request has no list pointer it clears a flag.
- `start_parallel_transfer_request`: Marks the request in progress, stores its buffer pointer and length, sets the unit direction flags and writes the global word to zero, or to decimal 255 for request types other than two.
- `begin_parallel_transfer_request`: Marks the request in progress, stores its buffer and length, sets the global word to zero, or to decimal 255 for request types other than two, and tracks the direction flags.
- `advance_parallel_transfer_byte`: Moves one byte between the global word and the active request buffer, advancing the buffer pointer and counter.
- `exchange_parallel_byte_with_buffer`: Transfers one byte between the request buffer and the global word: type-three requests read from the buffer, others write into it.
- `complete_parallel_transfer_request`: Stores the transfer count into the request, clears the busy flags, calls an internal routine, clears the active request and calls a handler from the unit table when the queue is empty.
- `read_parallel_input_byte`: Reads one byte from a source pointer into the global word and advances the buffer pointers when the byte is nonzero or the mode bit is clear.
- `reset_parallel_unit_state`: Calls a library routine through the device base, clears the status byte, sets the line-change flag bit when a unit bit is set, and stores the state into the request before calling a handler.
- `stub_immediate_return`: Empty routine that only returns with no side effects.
- `check_parallel_pending_request`: If a request pointer is set, calls a handler from the unit table with the unit block.
- `signal_parallel_request_pending`: If a request pointer is set, calls a handler from the unit table with the unit block and returns.
- `drain_parallel_request_list`: Repeatedly removes the first entry of a linked list and calls an internal routine with it until the list is empty, then clears the queue counter.
- `reset_parallel_queue_count`: Clears the 16-bit queue counter of the parallel unit and returns.
- `abort_parallel_request`: Sets a completion bit in the unit state, writes decimal 254 into the request status byte and calls a library routine through the device base.
- `update_parallel_port_bits`: Splits the low three bits of the global word, merges them into a unit control byte and stores that byte into the request.
- `copy_parallel_line_settings`: When the busy bit is clear, copies the request's line-mode byte into the unit; if bit 1 of that byte is set it also copies two stored pointers.
- `load_parallel_line_settings`: Copies the request's line-mode byte into the unit and, when bit 1 is set, copies two stored pointers from the request.
- `open_parallel_device_unit`: Opens the ciaa and misc resources through library calls and allocates the unit block.
- `return_zero_value`: A routine that performs no operations and simply returns zero to its caller.

### printer.device

- `allocate_printer_device_unit`: Allocates a device block through a library call.
- `forward_to_helper_routine`: Calls an internal routine and returns without other action.
- `complete_printer_io_request`: Sets the done bit in the request status byte and clears the queued bit when the request is linked.
- `create_printer_device_request`: Allocates a printer request through a library call, stores the result in the global base pointer, copies the caller's argument into the request and calls a library routine.
- `open_printer_libraries_and_buffers`: Stores the device base and opens the dos, graphics and intuition libraries and the timer device through library calls.
- `release_printer_device_resources`: Calls library routines that release what the startup routine opened, including several close calls and a free call.
- `run_printer_cleanup_steps`: Calls three internal routine routines in sequence and returns.
- `forward_printer_to_helper`: Calls an internal routine and returns.
- `set_printer_busy_flag`: Sets bit 0 of a status byte in the printer unit block.
- `clear_printer_busy_flag`: Clears bit 0 of a status byte in the printer unit block.
- `drain_printer_pending_list`: Repeatedly calls an internal routine for each entry while the first pointer in a list head is nonzero, emptying the list.
- `empty_return_routine_2`: An empty routine that returns immediately without performing any operations.
- `call_and_return_routine`: A routine that calls another internal routine and then returns without performing any operations itself.
- `run_printer_event_loop`: Initialises printer queue heads and state using library calls, then loops polling a status register and dispatching queued request entries through a function table.
- `service_printer_status_loop`: Waits on an event through an internal routine; when a status bit is set it dispatches the queued entries through the function table and clears the busy bits of matched request slots.
- `dispatch_printer_queued_requests`: Dispatches queued requests when a status bit is set, then re-reads the status word and clears busy bits for matched slots.
- `store_masked_printer_byte`: It is a printer device routine that clears a bit of an incoming byte using the unit's mask and stores the resulting byte into the request buffer, matching the behavior of the other receive routines.
- `queue_printer_request_slot`: When an internal routine reports no pending work it marks the busy bit, picks one of two request slots based on the previous busy bit, stores the request parameters and sets request type three, then calls two internal routines.
- `empty_routine_4`: This routine is empty; it performs no operations and returns immediately.
- `start_printer_output_request`: Sets the second busy bit, stores the request parameters and sets request type three into a fixed slot, then calls two internal routines to start the output.
- `wait_printer_request_completion`: Repeatedly services the printer request queue, sending commands through internal routine calls and handling completion bits for two slots.
- `service_printer_request_queue_2`: Loops while repeatedly servicing the printer request queue and polling two status bits until a completion condition is met, then inspects completion entries and clears the corresponding status flag bits for known request slots.
- `dispatch_printer_completion_entries`: Walks through the completion entries, handling the two busy bits for known slots and calling an internal routine for other entries.
- `dispatch_printer_status_entries`: Dispatches printer status entries through internal routines: each event clears the matching busy bit and the routine returns the status byte when the request is finished.
- `service_printer_request_queue`: Services the printer request queue.
- `reset_printer_request_parameters`: Sets the request field to decimal 9, copies a value from the device block into the request, clears a counter and a flag, then calls a library routine through the unit base.
- `finish_printer_request_state`: Clears the status byte of the request and calls a library routine through the unit base to finish it.
- `invoke_dispatch_table_routines`: Checks a status byte and returns early if a specific value is present.
- `close_printer_request_handle`: Calls an internal routine with the request's library handle and then calls a library routine with the unit to close it.
- `wait_printer_queue_idle`: Returns the request status byte, servicing queued requests while busy bits are set and polling the status register until the queue is idle.
- `poll_printer_status_until_idle`: Polls the printer status register and dispatches queued entries until the busy bits clear, then returns zero.
- `poll_printer_queue_entries`: Polls the printer entry list in a loop, clearing busy bits for matching slots and calling internal routines for other entries until the busy bits clear.
- `service_printer_entry_loop`: Loops through printer entries, calling internal routines and clearing the busy bit for matched slots.
- `service_printer_busy_bits`: Loops while busy bits are set, servicing entries through internal routines and clearing the bits for matched slots.
- `select_printer_output_slot`: Chooses a request slot from a two-entry table using a global index that toggles on each call, stores it into the device block and calls an internal routine unless the request is a special type with no pointer.
- `advance_printer_output_slot`: Advances to the next printer output slot by toggling the global index and storing the slot pointer in the device block.
- `clear_printer_busy_when_idle`: Calls an internal routine and, when it returns zero, clears the printer busy bits and calls the matching handlers.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `forward_to_two_helpers`: Calls two internal routine routines in sequence and returns.
- `set_printer_result_field`: Clears a result field, calls an internal routine and, when it succeeds, copies a second field into the result.
- `call_two_routines_in_sequence`: Calls two internal routines in sequence and then returns without performing any operations itself.
- `empty_routine_5`: This routine is empty, containing no instructions beyond its return.
- `start_printer_parameter_request`: Calls an internal routine and, when it succeeds, sets the request field to decimal 9, stores two parameters, clears a flag and calls internal routines.
- `send_printer_text_buffer`: Copies the request text into a 256-byte work buffer in chunks and sends each chunk through internal routines; a negative length means the text is NUL-terminated.
- `send_printer_text_fragment`: Sends a text fragment from the request by copying characters into a 256-byte buffer and passing each chunk to internal routines.
- `send_printer_text_counted`: Loops over a counted text run, copying characters into the work buffer and passing each chunk through internal routines until the count is reached.
- `send_printer_text_stream`: Streams text from the request in chunks, calling internal routines for each chunk and stopping when the count is reached or an error is returned.
- `send_printer_text_tail`: Sends the remaining text after the main loop finishes, flushing the work buffer through the internal routines and returning the status byte.
- `send_printer_text_with_terminator`: Sends printer text up to a terminating character.
- `copy_printer_text_remainder`: Copies the remaining bytes of the text buffer forward and stores the final count.
- `clear_printer_text_count`: Sets the request's returned count to zero.
- `translate_printer_characters`: Maps each input character through a lookup table with special cases for a few control codes, then passes the mapped value to an internal routine and advances through the table.
- `translate_printer_character_pair`: Loops over up to thirteen table entries for one input byte.
- `match_printer_character_table`: Matches each input byte against a thirteen-entry character table held in a string literal and writes the mapped output for matches.
- `resume_printer_character_scan`: Resumes the scan of the translation table from a saved index and writes the mapped output character, then shifts the window and calls an internal routine once the scan finishes.
- `finish_printer_character_scan`: Finishes the translation scan from a saved index, writing special-case or table output, shifting the window and calling an internal routine before the next input byte is taken.
- `map_printer_special_characters`: Handles special-case codes: writes a semicolon when code nine is followed by a zero byte and writes a form-feed style code for certain low values, then continues the table scan.
- `map_printer_table_characters`: Maps a code through a seven-entry sub-table when the index is below seven, otherwise copies the table value directly, then continues scanning.
- `scan_printer_character_sequence`: Copies the table value for codes at or above seven, or maps lower codes through a seven-entry sub-table, then continues the scan of the character sequence.
- `translate_printer_character_fragment`: Translates a fragment of characters through the table, handles the form-feed case for a zero code above a threshold, advances the window and calls an internal routine for each completed character.
- `run_printer_translation_state_machine`: Clears the translation state variables and runs the character translation loop.
- `process_printer_text_bytes`: Processes printer text bytes, writing plain characters to the output buffer and interpreting escape sequences (line feeds become carriage-return/line-feed pairs, high-range bytes are remapped through a font table).
- `resume_printer_escape_parsing`: Resumes the printer escape-sequence parser at a saved position.
- `parse_printer_bracket_sequences`: Parses sequences introduced by a bracket byte.
- `parse_printer_text_stream`: Walks printer text input, mapping escape prefixes and their parameters through the code tables, expanding line feeds and high-range bytes into stored escape strings, and writing the output to the buffer.
- `expand_printer_text_bytes`: Writes each printer text byte to the output buffer, expanding line feeds into carriage return and line feed pairs and high-range bytes through the stored escape strings.
- `emit_printer_line_end_pair`: Writes a line feed and a carriage return into the output buffer when there is room, then resumes parsing the printer text.
- `emit_printer_crlf`: Writes a line feed followed by a carriage return into the output buffer and resumes the text parser, which handles the rest of the printer stream.
- `emit_printer_line_feed`: Writes a single line feed byte into the output buffer and resumes the text parser.
- `expand_printer_escape_string`: Expands a high-range byte, at or above decimal 160, through a per-code table of escape strings.
- `copy_printer_escape_string`: Copies a stored escape string into the output buffer, decoding backslash sequences of up to three decimal digits into single byte values and advancing the buffer position.
- `decode_printer_escape_digits`: Decodes a backslash followed by up to three decimal digits into one byte value, then copies the remaining characters of the escape string to the output.
- `copy_printer_decoded_string`: Copies the characters of a stored escape string into the output, decoding backslash-digit sequences, and advances the output and count positions until the string ends or the buffer is full.
- `decode_printer_escape_bytes`: Decodes stored escape strings into output bytes: backslash sequences of up to three decimal digits become single byte values, other characters are copied, and the output count is advanced until the string ends.
- `decode_printer_escape_digit_run`: Reads a run of up to three decimal digits from an escape string and writes the combined value as one output byte, then continues with the following character.
- `parse_printer_decimal_escape`: Converts up to three decimal digits of an escape parameter into a byte value and writes it to the output buffer.
- `emit_printer_escape_first_byte`: Writes the first byte of an escape string to the output buffer, then continues copying the rest of the string with decoding.
- `emit_printer_plain_byte`: Writes one plain byte to the output buffer and resumes the printer text parser.
- `rewind_printer_text_position`: Moves the text position back by the number of bytes not yet consumed and clears the pending count, so parsing can restart from the unread part of the input.
- `append_printer_helper_output`: Calls an internal routine that writes into a 256-byte stack buffer and appends the produced bytes to the destination when they fit.
- `append_printer_stack_buffer`: Appends a counted run of bytes from the stack work buffer to the destination when the total stays within 256 bytes.
- `mark_printer_text_resume`: Stores the current text position as the resume point so the next parser pass can continue from it.
- `copy_printer_stack_bytes`: Copies a counted run of bytes from the stack work buffer to the destination, advancing the destination position for each byte.
- `copy_printer_stack_tail`: Copies the rest of the stack work buffer, from the current index up to the stored count, into the destination.
- `parse_printer_numeric_parameters`: Parses semicolon-separated decimal parameters from escape input into an array of values.
- `parse_printer_parameter_digits`: Accumulates decimal digits into the current parameter and handles the separators, spaces and quotes that end it, returning the parser state when the list ends.
- `scan_printer_parameter_separators`: Scans separators between printer escape parameters: semicolons start a new parameter, spaces and quotes set the state, and other bytes end the list.
- `parse_printer_quoted_parameter`: Reads a double-quoted parameter in the printer escape input, accumulating digits into the current value until the closing quote or another delimiter.
- `parse_printer_parameter_loop`: Loops over the escape parameter input, reading each byte and assembling parameters until a delimiter that is not a digit, semicolon, space or quote is reached, then sets the state result.
- `read_printer_text_byte`: Reads the next byte of the printer text input and advances the position.
- `return_register_value`: It reads a byte from a data register and returns it unchanged to the caller.
- `find_previous_printer_escape`: Scans backward through the printer text for the escape code, ignoring the high bit, and records the position it finds, marking the scan as done.
- `rewind_to_printer_escape`: Moves the text position back to the previous escape code and marks the scan as complete.
- `reset_printer_state_and_init`: Clears the printer state variables and sets a flag in the device block.
- `save_printer_configuration`: Copies six 16-bit configuration values from the device block into a saved configuration area.
- `compare_printer_configuration`: Compares six 16-bit configuration values in the device block with the saved copy.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `send_printer_formatted_buffer`: Builds a four-byte prefix from the device block and calls an internal routine to format it into a 256-byte stack buffer.
- `send_printer_status_codes`: Sends control values through internal routines: decimal 1 when a flag is zero, then a buffer pointer, then decimal 3 when the flag is zero.
- `emit_printer_code_sequence`: Looks up the escape string for a printer code in a table, writes its bytes into the output buffer and handles a special marker that ends the sequence with an error.
- `emit_printer_code_entry`: Writes the escape string for one printer code into the output, substituting a zero byte for the special marker and appending a final byte for the digit-style codes.
- `emit_printer_code_tail`: Finishes an escape sequence by writing the stored closing byte for digit-style codes once the code string is exhausted, and returns an error when a marker is hit before the end.
- `copy_printer_code_string`: Copies a zero-terminated escape code string into the output, replacing a special marker byte with zero and returning an error when the end marker is reached.
- `copy_printer_code_tail_bytes`: Copies the remaining bytes of a printer code string into the output until a terminator, then appends the closing byte for the digit codes and returns the new length.
- `dispatch_printer_driver_command`: Switches on a printer driver command code.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `configure_printer_driver_job`: Copies the print job parameters from the request into the driver's working variables, calls the driver's setup routine and sets output mode flags from the device capability bits.
- `select_printer_driver_options`: Sets driver option globals from the device flags and the selected mode, choosing one of several value sets and clearing or setting option bits in the same way for each option.
- `set_printer_option_flags`: Sets printer driver option flags from the device capability bits and the job settings, choosing defaults for each option and clearing bits that the device does not support.
- `compute_printer_print_area`: Computes the width and height of the print rectangle from the request bounds and checks them against the device margins.
- `compute_printer_bitmap_area`: Derives the print area from a bitmap header: width is taken from the header shifted by three bits, height from the next field, then the same margin check is applied and the sizes are scaled.
- `clamp_printer_print_size`: Clamps the computed width and height to the device's maximum sizes, then sets the output option flags for the job.
- `limit_printer_print_dimensions`: Alternative entry into the print-size clamp: limits the width and height to the device maxima and records the result for the rest of the job.
- `limit_printer_print_bounds`: Alternative entry into the same clamp, reading the device bounds and applying them to the computed size before setting the output flags.
- `limit_printer_print_margins`: Alternative entry into the clamp that applies the device margin limits to the computed size and then sets the output option flags.
- `compute_printer_scaled_extents`: Alternative entry into the print-area sizing code.
- `compute_printer_output_scale`: Computes the output scale and offsets for a print job, choosing the scaling mode from the device type and limiting the result by the page height.
- `select_printer_scaling_mode`: Chooses the scaling mode values from the device type and the scaling flag, then sets the page width and height extents used by the rest of the job.
- `set_printer_raster_extents`: Sets the raster width and height extents for the selected orientation from the job bounds and the device resolution values.
- `swap_printer_raster_extents`: Swaps the width and height extents between the working variables, then sets the output extents for the rotated orientation.
- `normalise_printer_raster_offsets`: Turns negative raster offsets into positive values, clears the offset words when the mode requires it, and recomputes the end extents from the job bounds.
- `scale_printer_raster_coordinates`: Scales a raster coordinate by shifting it right by fifteen bits and adding one, applying this to the high word of the result when the scaling flag is set.
- `render_printer_raster_rows`: Processes the raster rows of the print job: scales coordinates, computes row offsets and calls internal routine routines to render each row and send it to the output.
- `clip_printer_raster_rectangle`: Clips the raster rectangle against the device limits held in the driver state and adjusts the start and end coordinates to fit.
- `compute_printer_clip_extents`: Takes the minimum and maximum of the job and device sizes for the width and height and stores the resulting clip extents.
- `compute_printer_clip_offsets`: Computes the clip offsets from the job and device extents, using the smaller of each pair and storing the differences as working values.
- `load_printer_palette_entries`: Clears a 64-entry working table, then reads each palette entry for the screen depth through an internal routine and stores it in the table.
- `split_printer_palette_channels`: Reads palette colours through the graphics library and splits each 16-bit colour value into three 4-bit channel values stored in the working table.
- `run_printer_raster_pass`: Starts a raster pass by setting a fixed bit mask and calling a driver routine with the row values.
- `retry_printer_raster_call`: Makes the driver call for a raster pass again when the first attempt returned zero, using a fixed mode argument, and chooses the next step from the output flags.
- `dispatch_printer_raster_output`: Selects which driver call to make from the output mode flags and stores its result as the status of the raster output.
- `finish_printer_raster_row`: Calls a driver routine to finish the current raster row, makes a second call when the first returns zero, and passes the resulting status on to a handler.
- `finish_printer_raster_block`: Finishes a raster block with a driver call and a second call when the first returns zero, then passes the status to a handler; the check repeats once the block count is a multiple of the device interval.
- `report_printer_raster_status`: Passes the driver status for a raster block on to the handler, with a second driver call when the first result is zero.
- `report_printer_row_status`: Reports row completion: when the row counter is a multiple of the device interval, or after a driver call returns zero, it reports a zero status to a handler.
- `call_routine_with_zero_arguments`: Calls one other routine with two zero arguments and returns.
- `map_colour_to_level`: With the global mode flag clear, it returns the smallest colour channel when a device flag bit is set, otherwise the value unchanged.
- `pick_minimum_colour_channel`: Returns the smallest of the three colour channel bytes when a device flag bit is set, otherwise returns the input value unchanged.
- `lookup_pixel_palette_value`: Asks a coordinate routine for an index for the current pixel and stores a colour value into a device variable.
- `read_pixel_and_emit_colour`: It reads a pixel value through a library stub and then writes a palette entry to an output pointer.
- `emit_colour_and_advance_accumulators`: Writes the palette value selected by the current index through an output pointer, using a nibble lookup and optional inversion when the global mode flag is set.
- `apply_polarity_and_advance`: Stores the current colour value, inverts the index nibble when a device flag bit is clear, and updates the global mode bits from the index range.
- `update_mode_bits_and_advance`: Updates the global mode bits from the high nibble of the index, calls a routine with the stored value, and adds the global step value into four accumulators.
- `assign_global_byte_and_advance`: Sets the global byte from an input byte depending on a selector code, calls a routine with the stored value, and adds the global byte into four accumulators.
- `assign_byte_on_selector_three`: Copies an input byte into the global byte only when the selector equals three, stores the routine result, and adds the global byte into four accumulators.
- `copy_input_byte_to_global`: Copies an input byte into the global byte, stores a routine result through a pointer, and adds the global byte into four accumulators.
- `store_byte_and_map_colour_2`: Stores a byte received in a register into a global variable, maps it through a colour lookup routine, writes the mapped result back, and adds the original byte to four separate counters.
- `store_byte_and_map_colour`: It stores a byte received in a register into a global variable and passes the pointed-to value through a colour mapping routine before storing the result.
- `store_indexed_global_and_advance`: Loads a value from the global table at an index held in a register, writes it through a pointer, and adds the global byte into four accumulators.
- `compute_step_and_initialise_accumulators`: Multiplies two device step values into a product, using their sum instead when the product is zero.
- `try_handler_candidates_in_order`: Runs a series of stored handler calls with selector codes zero to three, each gated by a routine check.
- `try_three_handler_codes`: Checks three gated values in order and calls a stored handler with codes one, two and three on each success.
- `try_codes_two_then_three`: Calls a stored handler with code two when a routine check passes, otherwise falls through to code three.
- `try_handler_code_three`: Calls a routine check and, when it passes, calls a stored handler with code three and returns its nonzero result.
- `return_immediately`: It executes three instructions and returns without side effects.
- `advance_row_and_notify_aligned`: Increments a row counter and returns zero unless the row count is a multiple of an interval read from the device table.
- `invoke_handler_code_three`: Calls a stored handler with selector code three and returns its result, or zero when the result is zero.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `walk_grid_calling_handlers`: Walks rows of a grid from a start position up to a limit, reading each row size from a routine.
- `walk_grid_with_row_counter`: Walks the grid like the previous routine, but keeps the row size in a counter updated from the routine.
- `walk_grid_reloading_row_sizes`: Walks cells row by row, reloading the row size from a routine after each row.
- `walk_cells_per_column`: Steps through the cells of a column calling a per-cell routine for each, then moves to the next row and reloads a routine value.
- `walk_cells_across_columns`: Steps across the cells of one row and moves to the next row when the column count is reached.
- `walk_rows_until_handler_fails`: Repeatedly advances a row counter and runs a per-cell pass, calling a handler after each completed pass.
- `advance_block_and_walk_cells`: Adds the block size to a running position and returns zero once the position reaches the limit.
- `walk_bands_with_height`: Walks bands from a start position to a limit, reading each band height from a routine.
- `walk_strips_with_width`: Reads a strip width from a routine, walks the cells of each strip with per-cell handlers, and advances through strips.
- `walk_strip_cells_with_handlers`: Calls per-cell handlers for each cell of a strip using a width held in memory, then moves on to the next strip.
- `walk_columns_resetting_accumulators`: Walks columns and resets four accumulators at the start of each column.
- `walk_rows_counting_down`: Counts down over rows, running a per-cell pass and a handler on each row, and advances a position at the end of each band.
- `count_down_rows_with_handlers`: Counts down rows, running a per-cell pass and a handler for each row.
- `advance_block_and_walk_rows`: Adds the block size to a running position and returns zero when it reaches the limit.
- `walk_grid_with_repeats`: Walks a grid of columns and rows, repeating each row a number of times read from a routine.
- `walk_repeated_cell_passes`: Repeatedly walks a grid of cells using a count read from a routine, decrementing it on each pass and calling per-cell handlers.
- `walk_passes_with_countdown`: Runs cell passes while a countdown is nonzero, reloading the count from a routine when it runs out.
- `walk_tiles_with_countdown`: Walks tiles across rows from a start position to a limit, using a countdown to repeat each tile row.
- `walk_cells_with_row_handlers`: Walks cells across a row with a per-cell pass, advancing a column index and a row position.
- `advance_row_position_and_scan`: Advances a row position by a routine stride, runs a per-row pass over cells, and calls a handler after each completed row.
- `decrement_repeat_and_scan`: Decrements a repeat count and, when it reaches zero, advances a grid position and reloads the count from a routine.
- `walk_repeated_tile_rows`: Walks tiles across rows from a start position to a limit, repeating each tile row a count from a routine.
- `repeat_cell_passes_by_count`: Reads a count from a routine and repeats cell passes while the count is nonzero.
- `walk_column_passes_with_repeat`: Walks column passes, decrementing a repeat count and reloading it from the routine.
- `scan_column_cells_with_repeats`: Scans the cells of a column with repeat counts, decrementing and reloading the counts from routines.
- `scan_cells_counting_down`: Counts down cells across a block, calling per-cell and per-row handlers.
- `scan_rows_in_blocks`: Scans rows in blocks of a fixed size, calling per-cell handlers and a block handler.
- `countdown_block_and_scan`: Decrements a block count and reloads the next block from a routine when the count reaches zero.
- `compute_bresenham_error_steps`: Steps a Bresenham-style error term held in a pointed-to accumulator.
- `compare_level_to_dither_threshold`: Compares a colour level with a 4x4 threshold matrix entry chosen by the low two bits of the two coordinates, returning whether the level is greater.
- `check_dither_threshold_below`: Returns whether the 4x4 dither threshold entry chosen by the low two bits of the two coordinate registers is less than the level in another register.
- `call_two_helpers_in_sequence`: Calls two routine routines in sequence and returns.
- `divide_32_bit_quotient`: Divides a 32-bit dividend by a divisor.
- `run_shift_subtract_division`: Performs sixteen steps of shift-and-subtract division on a 32-bit value.
- `multiply_32_bit_integers`: Multiplies two 32-bit values from two registers using 16-bit partial products and returns the low 32 bits of the product.
- `recursive_call`: It invokes itself once and then returns to its caller.
- `call_and_return_register_value`: Calls one other routine and returns whatever it leaves in the D1 data register.
- `negate_result_on_flag`: Calls another routine and returns its result, negated when the routine's second output is nonzero.
- `negate_on_flag`: It calls another routine and takes its first result, negating that result when the second result is non-zero.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `load_named_preferences_entry`: Fetches the system preferences block through the intuition library, then searches a name table for an entry whose characters match a stored name, ignoring case.
- `match_name_and_replace_buffer`: Compares a selected name string with a stored name ignoring letter case.
- `return_minus_one_stub`: Returns minus one and does nothing else.
- `match_name_and_rebuild_entry`: Searches a name table for a matching entry ignoring letter case and rebuilds the stored entry on a match.
- `match_name_and_allocate_entry`: Checks a name against a table ignoring letter case, allocating a new entry for the caller on a match.
- `scan_name_and_allocate_entry`: Walks a name string character by character against stored names ignoring case, and allocates an entry for a match.
- `load_printer_driver_segment`: Locks the printers directory, makes it current, and loads a printer driver segment from it.
- `restore_directory_and_fail`: Restores the previous current directory lock, releasing the directory, and returns minus one as a failure result.
- `unload_driver_segment`: If a loaded segment pointer is set, calls the stored unload handler and then unloads that segment from memory.
- `compare_characters_ignoring_case`: Returns whether two characters are equal after converting lowercase letters to uppercase.
- `return_zero_stub`: Returns zero and does nothing else.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `invoke_loadseg_handler`: Calls a stored library function pointer that loads a program segment and returns its result.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `return_empty_result`: This routine performs no operations and returns immediately without touching any data or registers.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 255 into d0, then returns.

### arp.library

- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 34 into d0, then returns.
- `fill_arp_entry_table`: Stores a pointer into the structure, then calls a library function repeatedly to fill a small table of entries.
- `setup_arp_list_header`: Initialises a list header pointer and calls library functions to set up an entry, falling through to a routine when the table fill succeeds.
- `allocate_arp_buffer_or_cleanup`: Stores a pointer, calls several library functions, and on a successful allocation stores the size and owner into the block.
- `poll_library_until_flag`: Repeatedly calls another routine and a library function until a status flag is set, then increments a counter in the structure.
- `clear_field_and_call_library`: Clears a field in a structure and calls a library function through the library base.
- `allocate_sized_memory_block_2`: When the requested size is nonzero, calls the memory allocation library function and on success stores the size plus a header into the returned block.

### Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF][a2].adf:vf.obj

- `entry_toggle_interrupts_with_helpers`: Program entry that disables interrupts through the interrupt enable register, calls a setup routine, then re-enables interrupts and calls a second routine before returning.
- `decode_bitstream_into_buffer`: Reads bits from a shifting bit buffer that is refilled from a stored word, and writes decoded bytes backwards into a buffer with copy runs.
- `advance_xor_shift_random_state`: Updates a global state word by exclusive-or with a word read from memory, and returns that word shifted right by one with its top bit set.
- `shift_bits_with_refill`: Shifts a value right one bit per iteration for a count, reloading the value from a routine when it becomes zero.

### Spy vs Spy - The Island Caper (1989)(Wicked Software)[cr VF][a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_chips_and_copper_list`: Opens and closes system libraries, clears a block of memory, programs the custom chip registers for the copper list pointer, DMA and interrupts, then waits for a status flag to clear.
- `advance_timed_entry_queue`: Decrements a countdown byte; when it reaches zero it resets the byte and, if the queue has entries, copies the next entry into three slots and moves the queue pointer.
- `fill_offset_step_table`: Fills two parallel eight-entry tables, one with values stepping by a large increment and one by a smaller increment, starting from fixed values.
- `toggle_or_expand_pattern_entries`: Copies a stored pattern word into a table, storing its complement as well, and advances the pointer.
- `fill_block_rows_from_helper`: Resets a source pointer when its byte count is exhausted, then calls a per-cell routine twenty times per row for eight rows, advancing the pointer by twenty each row.
- `start_blitter_table_copy`: Finds the position of a character in a table, waits for the blitter to be idle, and programs the blitter control and size registers to copy from that table offset.
- `update_position_and_blit`: Steps a vertical position down by four and wraps it when negative, toggles a bit, waits for the blitter to finish, then sets the blitter registers and starts a blit.
- `blit_rows_with_mask_loop`: Waits for the blitter, sets its mask and control registers, then calls a routine for each of seven entries.
- `increment_global_counter`: It loads a value from a global variable, adds one to it, and stores the result back to the same global variable.
