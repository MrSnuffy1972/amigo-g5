# Game 948 (cruise for a corpse (1991)(u. s. gold)(sp)[cr sparrows]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00948). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4505664 top-level bytes in 40 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 104 of 104 named (0 author symbols), 104 of 104 described (11 by rule, 90 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 585 of 585 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4343 of 4343 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3265413 |
| code | 13212 |
| data | 1086209 |
| padding | 8734 |
| unused | 132096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:The_SparrowS | 2 | 119 | 3 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:endcli | 2 | 117 | 2 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:cruise | 2 | 233 | 8 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:cd | 2 | 117 | 2 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:add21k | 3 | 653 | 17 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 5 of 5).adf:makedir | 1 | 111 | 1 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 5 of 5).adf:InstallHD | 3 | 2941 | 65 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:bootblock | 1 | 10 | 1 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 2 of 5).adf:bootblock | 1 | 10 | 1 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 3 of 5).adf:bootblock | 1 | 10 | 1 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 4 of 5).adf:bootblock | 1 | 10 | 1 |
| Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 5 of 5).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- cruise for a corpse (1991)(u. s. gold)(sp)[cr sparrows](disk 1 of 5)/Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:d1: high-entropy data file (7.69 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- cruise for a corpse (1991)(u. s. gold)(sp)[cr sparrows](disk 1 of 5)/Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:The_SparrowS: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- cruise for a corpse (1991)(u. s. gold)(sp)[cr sparrows](disk 1 of 5)/Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:cruise: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:The_SparrowS

- `decompress_bitpacked_data`: Decompresses a bit-packed data block into a destination buffer, writing one byte at a time and copying the color register at address a fixed address after each byte.
- `fetch_shifted_bit_word`: Loads a longword from the stack into d0, sets the carry flag, rotates d0 right by one bit, and returns.
- `shift_and_rotate_loop`: Decrements the 16-bit value in d1 and clears d2, then shifts d0 right by one bit and returns if the result is zero.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0`: Copies the value of register d1 into register d0 and then branches to another location.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:cruise

- `process_hunk_table`: Parses a table of AmigaDOS hunk type codes, relocating and copying hunk data in memory, calling the library's exec.AllocMem and exec.FreeMem routines when needed, then returns.
- `decrement_counter_to_zero`: Decrements a 16-bit counter in d1 until it reaches zero, looping back each time.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `shift_and_reload_value`: Reads a byte from the stack, and when non-zero decrements it and calls another routine to adjust a source pointer.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `rotate_and_reload_shifted_value`: Rotates the 32-bit value in d5 right by the low six bits of d4 and folds the carried bit into d0 across a loop that runs d1 plus one times, returning when the shift amount is zero.
- `write_packed_and_trap`: Writes a 16-bit value from register d4 to the structure in a2 at offset 176 using a movep instruction, then issues trap number 6, then sets bit 16 of register d0 by OR-ing it with register d2, and returns.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:cd

- `find_max_and_prepare_buffer_2`: Walks a linked list of table entries to find the largest value in a field, then stores that maximum plus a sequence of incrementing words into a buffer, sets up a stack frame with a saved return address, copies sixteen words between a temporary area and a global buffer, installs a function pointer, and returns success or failure depending on the conditions checked during the loop.
- `move_d1_to_d0`: Copies the value in register d1 into register d0 and then branches to the routine that decompresses a bit-packed data block.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:add21k

- `initialize_and_dispatch`: Clears a 60-word table in a global structure, stores the value 60 at offset 202 of that structure, then reads a flag bit at offset 297 and calls a handler if it is set.
- `open_system_libraries`: Opens the dos, graphics, and intuition libraries by name, showing an alert if either of the first two fails to open.
- `load_workbench_base`: Loads the address of the workbench base address global variable into register a4, then returns.
- `store_opened_handles_into_record`: Calls a routine to fetch a value from a global variable, stores it in another global variable, and returns if that value is zero.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `split_string_into_tokens`: Reads a 16-bit index from offset 172 of a structure, looks up a pointer in a table, and stores a value derived from it.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit`: Finds the end of the first string by advancing a pointer until it reaches the null terminator, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `handle_library_entry`: Reads a value from a library vector table and stores it in a global variable, then returns if that value is zero.
- `move_d0_to_d5`: Copies the 32-bit contents of register d0 into register d5, then returns.
- `copy_string_or_pad`: Copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `calculate_string_length`: Advances a pointer through a zero-terminated byte string until it finds the terminating zero byte, then returns the difference between the address of the terminating byte and the original pointer, which is the number of bytes before the terminator.
- `copy_string_simple`: Copies characters from a source string to a destination buffer until the null terminator is copied, then returns the pointer to the start of the destination buffer.
- `release_list_entries_and_call`: When a flag is set it walks a linked list using a global head pointer, calling the function pointer stored at offset 4 of each node and advancing to the next node until the pointer becomes zero.
- `release_locks_and_close_resources`: Checks a flag and, when set, loops through a table calling a routine for each index, then calls a cleanup routine.
- `restore_registers_and_return`: Restores registers d2 and d3 from the stack, then returns.
- `release_library_entry`: Looks up an entry in a table indexed by a parameter and checks whether the entry is in use.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 5 of 5).adf:makedir

- `make_directories_from_args`: Program entry for the makedir tool.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 5 of 5).adf:InstallHD

- `jump_to_program_entry`: Contains only a single jump to the program entry point and returns immediately without touching any data.
- `process_path_and_query_disk`: Copies a path string into a stack buffer, then loops over its characters and truncates the string at the colon character.
- `initialize_and_process_events`: Initializes a set of 16-bit fields in a structure in a4 with fixed values, then repeatedly calls a routine at offset 552 of a table in a4 and dispatches on the returned code, calling other routines for the codes 32, 64 and 512.
- `dispatch_event_by_type`: Dispatches on a type code read from a structure.
- `update_gadget_text_and_refresh`: Looks up a value and, when it is non-zero, copies a text into a buffer, stores its length plus one, and refreshes the gadget display.
- `check_status_and_call_routines`: Checks bit 3 of a status byte and calls the routine at offset -48 of a table in a4 if it is set, then checks bit 2 and calls the routine at offset -19e of a table in a4 if it is set, then calls the routine at offset -d2 of the first table, then checks bit 1 and calls the routine at offset -19e again if it is set, then returns.
- `store_value_and_call_indirect`: Stores the incoming parameter into a global variable, then performs an indirect call through the pointer at offset -14a of a table in a4, then performs another indirect call through the pointer at offset -15c of the same table, then returns.
- `read_vol_cnf_config`: Fills a 512-byte stack buffer through a called routine, then passes it to a second called routine with a fixed value.
- `call_and_return_result`: Performs an indirect call through the pointer at offset -78 of a table in a4, then returns the result if it is nonzero, otherwise calls the routine at offset -84 of the table and returns its result.
- `install_game_files_retry`: Retrieves the DOS error code and, if it indicates a missing or invalid source, normalizes the input path by trimming a trailing slash and repeatedly attempts to copy the game files from the source volumes to the target directory, retrying several times before giving up.
- `update_gadget_count_and_refresh`: Calls a routine with a buffer, reads the returned string length, stores that length plus one into a field of a structure, then calls a routine to refresh the window gadgets and returns.
- `update_gadget_length`: Calls a routine with a buffer, reads the returned string length, stores that length plus one into a global variable, then calls a routine to refresh the window gadgets and returns.
- `validate_stream_buffers_against_sources`: Copies two source strings into a stack buffer, then validates a destination against both by reading fixed-size blocks and comparing them, returning success, failure, or a partial-match code.
- `copy_long_from_table`: Copies a long word from the address at offset 6400 in a4 into the memory location pointed to by a2, then returns.
- `write_stream_buffer`: Copies bytes from the source buffer in param_1 into the destination in param_1, advancing the stream read position and stopping when the stream reports its 65535 end marker, then returns the number of bytes written.
- `adjust_result_from_validation`: Pushes a byte from offset 14 of the structure in a2 and a flag, calls the validation routine, then adjusts the returned value using offsets 0, 4 and 8 of the structure depending on a bit in offset 12, storing error code 6 at offset 28400 and returning -1 on failure.
- `write_stream_buffer_2`: Copies up to param_3 bytes from the source buffer in param_1 into the destination stream, advancing the stream read position and stopping when the stream reports its 65535 end marker, then returns the number of bytes written.
- `forward_call_with_base_offset`: Pushes a value taken from offset -29550 of a4 as a stack argument, calls another routine, then returns that routine's 16-bit result in d0.
- `clear_status_bits_and_reset`: Clears bit 3 of a 16-bit status field at offset 12 of the structure it receives, then calls another routine with that structure and zero, and returns.
- `seek_buffered_stream_position`: Checks a flag in a stream buffer structure and, when the request is valid, advances its read position by the requested count while staying within the buffer bounds.
- `load_stack_pointer`: Loads the address of the top of the stack into a4 and returns immediately.
- `init_standard_stream_record`: Calls a library routine to obtain a value, stores it at offset 1006 of a structure in a4, and returns immediately if the value is zero.
- `copy_d0_to_d5`: Copies the contents of d0 into d5, then returns.
- `tokenize_string_into_table`: Loads a text pointer from a structure, calls a library routine to obtain a buffer address, then tokenizes the referenced text by skipping whitespace and splitting it into quoted or plain words, storing each word's address in a table and null-terminating the result.
- `copy_d0_to_d0`: Copies the contents of d0 into d0, then returns.
- `copy_string_with_limit_2`: Finds the end of the first string by scanning forward until a null byte, then copies characters from the second string into the first while decrementing a counter, stopping when the counter reaches zero or the source string ends, and null-terminates the result before returning.
- `call_library_and_store_results`: Loads a pointer from a global variable and calls the routine stored at offset 78 of that structure, then calls the routine at offset 96 and the one at offset 1502, storing each result back into the global variable.
- `copy_d0_to_d5_2`: Copies the 32-bit value from register d0 into register d5 and then returns.
- `format_bounded_string_into_buffer`: Initializes a stack buffer with a starting word, then calls another routine to format a string into it and stores the returned length.
- `process_stream_entry`: Checks the flags in the structure in a2 and returns -1 when invalid, otherwise advances a pointer within bounds, clears a flag, calls the validation routine, sets a flag based on its result, and finally reads a byte from the new pointer position and returns it as an unsigned value.
- `call_with_computed_argument`: Calls another routine to obtain a value, then passes that value as a third argument to a second routine before returning.
- `parse_mode_and_create_file`: Parses a mode string starting at the pointer in a2, setting flags in d3 based on the first character and scanning subsequent characters, then calls another routine with a6 and stores the result and flags into offsets 12 and 14 of the structure in a3, returning that pointer.
- `find_and_clear_structure`: Walks through a sequence of structures by advancing a pointer until it finds one whose word at offset 12 is zero, then clears the word at offset 20 and the longs at offsets 0, 4 and 8 of that structure before returning its address, otherwise returns zero.
- `copy_string_until_null`: Scans a byte string starting at the first pointer until a zero byte is found, then decrements that pointer to point at the terminator and copies bytes from the second pointer into the destination until the null terminator is copied.
- `copy_string_or_pad_2`: Copies bytes from the source string to the destination until the source ends or a count reaches zero, padding the destination with null bytes when the count is zero.
- `forward_single_argument_call`: Pushes a word from the stack, a constant, and a pointer, calls another routine, discards the result, and returns.
- `find_free_slot_and_create_file`: Walks a table of 24-byte records in a global variable, skipping entries already in use, and for the first free slot calls dos.library to create a new empty file, then stores the slot index and a flag word in that record before returning.
- `validate_index_and_call_entry`: Computes an index into a table of structures from a signed argument, validates it against a stored count and checks the entry, storing error code 3 or 6 at offset 28400 of the structure in a4 and returning -1 when invalid, otherwise calling a routine through a vector and returning its result or -1 on failure.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `decode_percent_escaped_string`: Decodes a percent-sign escape sequence in a byte string, writing literal characters to a destination buffer and interpreting numeric and flag escapes, then returns the number of bytes written.
- `call_and_return_concatenated`: Pushes the value of register d1 onto the stack, calls another routine, then combines the original d1 value with the value left in d1 by the call into a 64-bit result returned in d0 and d1 before restoring d1 and returning.
- `recursive_call`: Pushes the value of register d1 onto the stack, calls another routine, restores d1, and returns, so it recurses without performing any other operation.
- `compute_32bit_division`: Computes the quotient and remainder of a 32-bit division using inputs in the first two registers.
- `calculate_string_length_2`: Walks a byte string starting at the pointer until a zero byte is found, then subtracts the original pointer from the terminator address.
- `walk_table_and_call_routine`: When the input pointer is zero it walks a table of structures advancing by 22 bytes, calling another routine for each entry whose word at offset 12 is non-zero and has none of bits 0, 3 and 8 set and whose result is negative, returning negative one on the first such match.
- `walk_file_slot_table`: Walks every entry of a table in order, starting at a global base and advancing by twenty-two bytes, passing each entry to another routine until it reaches a second global base.
- `process_stream_buffer_entry`: Processes a stream buffer entry by advancing its read position, writing decoded bytes to a destination buffer, and invoking a callback routine for each byte until the stream ends or an error occurs.
- `unlink_list_node_and_report`: When the input pointer is zero or its word at offset 12 is zero it returns negative one.
- `tiny_addq_addq`: Small routine of 2 instructions that adds 2 to a5, then adds 8 to d0.
- `set_up_stream_buffer`: Reads a byte from offset 14 of the structure in a2, converts it to a signed word, and calls another routine with it.
- `unlink_list_node`: Walks a linked list using a global head pointer looking for a node whose second field matches the input pointer.
- `branch_into_other_routine`: Pushes registers a2 and a3 onto the stack, loads a pointer from a global variable into a2, and branches to another routine without returning.
- `free_linked_list`: Walks a linked list freeing each node by calling a library routine with the node data until the head pointer becomes zero.
- `store_allocated_block`: Checks a pointer stored in a global variable and returns zero if it is empty, otherwise calls a routine from an indexed table to allocate a block and stores a handler address, the passed value, and the returned pointer into a structure.
- `copy_string_simple_2`: Copies bytes from the source string to the destination until the null terminator is copied, advancing both pointers.
- `validate_index_and_query`: Validates an index against bounds and checks that the table entry at that index is non-zero.
- `validate_index_and_call_vector`: Computes an index into a table of structures from a signed argument and validates it against a stored count, storing error code 3 at offset 28400 of the structure in a4 and returning -1 when invalid, otherwise calling a routine through a vector with the argument and returning its result or -1 on failure.
- `delete_file_and_report_error`: Calls a library routine to delete a file given by the input pointer.
- `validate_index_and_call_write`: Computes an index into a table of structures from a signed argument and validates it against a stored count, storing error code 3 or 6 at offset 28400 of the structure in a4 and returning -1 when invalid, otherwise conditionally calling one routine and then another with the argument and returning its result or -1 on failure.
- `check_flag_and_call_routine`: Calls a routine to fetch a value from a global variable, then returns that value if bit twelve is set and a flag at offset 28386 is clear.
- `write_buffer_to_outputs`: Writes four bytes from a global buffer to the standard output through a library call, then writes the same four bytes to another output handle.
- `run_pending_cleanup_callbacks`: Walks a linked list of pending entries, calling each entry's handler and moving to the next, then runs an optional final handler and calls another routine.
- `release_program_resources`: When a flag is set it loops from zero up to a stored count calling another routine for each index, then calls a routine from an indexed table.
- `restore_registers_and_return_2`: Restores the two data registers from the stack and returns immediately, performing no other work.
- `check_and_call_indexed_pointer`: Loads a word from a global variable at offset -29132 and uses it as an index into a table of 32-bit pointers, then checks the pointed-to long word.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 1 of 5).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 2 of 5).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 3 of 5).adf:bootblock

- `boot_entry_3`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 4 of 5).adf:bootblock

- `boot_entry_4`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Cruise for a Corpse (1991)(U. S. Gold)(Sp)[cr Sparrows](Disk 5 of 5).adf:bootblock

- `boot_entry_5`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
