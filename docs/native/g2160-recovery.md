# Game 2160 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a5]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02160). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (933424 top-level bytes in 51 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 27 raw-word lines |
| Functions named and described | pass, 491 of 491 named (0 author symbols), 491 of 491 described (87 by rule, 150 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2421 of 2421 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14585 of 14602 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 78585 |
| code | 47934 |
| data | 501481 |
| padding | 13072 |
| unused | 292352 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:Keymap | 1 | 13 | 1 |
| Run | 2 | 117 | 2 |
| Install | 3 | 117 | 2 |
| Else | 2 | 117 | 2 |
| Delete | 2 | 117 | 2 |
| Avail | 11 | 376 | 12 |
| Path | 2 | 117 | 2 |
| Resident | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Rename | 2 | 117 | 2 |
| Dir | 2 | 117 | 2 |
| NewCLI | 2 | 117 | 2 |
| NewShell | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:AM2_BLIT | 11 | 45 | 2 |
| d | 1 | 13 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:AM2_CPU | 10 | 45 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:Ambermoon | 8 | 253 | 7 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:Ambermoon_install | 7 | 253 | 7 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1129 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3137 | 124 |

## Cases handled

- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 517 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1406 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a5]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a5]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a5]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a5]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `finish_with_vector_call` (More): Calls two routines in turn, then jumps through the A3 vector. Kind: loader/protection stub. Pass path: the branch following the indirect call (*unaff_A3)().

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:Keymap

- `stub_bad_instruction_data`: The bytes at this entry are not valid code, so it is only a stub with no behaviour to describe.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the destination pointer passed on the stack is not null, copies sixteen longwords from a fixed offset in the structure at the address register into it, then calls a routine through a register pointer.

### Install

- `prepare_command_buffer`: Prepares a command buffer for an AmigaDOS command: it scans a linked list for its largest field value, reserves a longword array sized to match, fills it with a fixed pattern and a stack-frame pointer, then runs a 16-entry block copy.
- `copy_memory_block_3`: Transfers a fixed number of words from a source offset to a destination address.

### Else

- `update_linked_list_and_copy_data_5`: Traverses a linked list to find the maximum value in a specific field, then expands the list by allocating or calculating a new buffer size.
- `copy_memory_block_5`: Copies sixteen words from a source address to a destination address using a loop.

### Delete

- `update_linked_list_and_copy_data_6`: Traverses a linked list to find a maximum value and then updates a structure with this value and a sequence of incrementing integers.
- `copy_memory_block_and_call_8`: Copies sixteen bytes from a memory location offset from a register to a destination pointer, then calls another routine.

### Avail

- `initialize_command_process`: Program entry point for the Avail command: it allocates a record for the process, opens dos.library and raises an alert when that fails, parses the command line into argument strings that honour quotes and escape codes, then runs the command body and restores the saved state before returning.
- `finish_command_and_reply`: Stores a result in the command record, closes the dos library, frees the record's buffers, replies to the waiting message, and returns that result.
- `sum_matching_memory_regions`: Adds up a size difference for each memory region in a linked list whose attribute flags include the given mask, and returns the total.
- `print_memory_usage_table`: Prints the memory usage report for the Avail command, showing the available, in-use, maximum and largest figures for chip, fast or total memory.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_with_stack_slot`: Passes its two arguments to another routine together with a pointer to a stack slot that holds the third value.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Formats text into a 140-byte stack buffer through the exec formatting routine, computes an inverted count of the characters produced, and passes the buffer and count to another routine when the count is non-zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Path

- `update_linked_list_and_sync_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a structure with this value and a sequence of incrementing integers.
- `copy_memory_block_and_call_9`: Copies sixteen bytes from a memory location offset from a register to a destination address on the stack, then calls another routine.

### Resident

- `update_linked_list_buffer`: Traverses a linked list to find the maximum value in a specific field and then expands a data structure by writing a header and copying sixteen longwords.
- `copy_memory_block_and_call_10`: Copies sixteen bytes from a memory location offset from a pointer to a destination address, then calls another routine.

### Assign

- `find_max_and_update_list_2`: Iterates through a linked list to find the maximum value in a specific field, then updates a header and copies data blocks between a structure and a provided destination pointer.
- `copy_memory_block_and_call_11`: Copies a fixed sequence of sixteen bytes from a memory location offset from a register to a destination pointed to by the stack.

### EndCLI

- `find_max_and_update_list_3`: Traverses a linked list to find the maximum value in a specific field and then updates a structure with this value and a generated sequence of incrementing longwords.
- `copy_memory_block_6`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer.

### Rename

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field, then updates a structure with this value and a sequence of incrementing integers.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location offset from a pointer to a destination address on the stack, then calls another routine.

### Dir

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field and then updates a structure with this value and a generated sequence.
- `copy_memory_block_and_call_2`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then calls another routine.

### NewCLI

- `find_max_and_populate_buffer`: Traverses a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a buffer.
- `copy_memory_block_and_call_3`: Copies a fixed number of words from a memory location to a destination pointed to by the stack, then calls another routine.

### NewShell

- `update_linked_list_and_buffer`: Iterates through a linked list to find the maximum value in a specific field, then updates a header and populates a buffer with a sequence of incrementing values.
- `copy_sixteen_longs_to_argument`: Copies sixteen 32-bit words from a fixed block addressed through the A2 register into the buffer whose pointer is passed on the stack, skipping the copy when that pointer is null.

### Echo

- `install_header_copy_hook`: Scans an index-linked table of entries for the largest value in one field, then, if a library call returns a non-zero result, fills a run of sequentially increasing words and saves a sixteen-word image of the returned block.
- `copy_memory_block_and_call_4`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then calls another routine.

### Execute

- `update_linked_list_and_copy_structure`: Traverses a linked list to find the maximum value in a specific field and then updates a header structure.
- `copy_memory_block_and_call_5`: Copies sixteen bytes from a memory location offset from a register to a stack address, then calls another routine.

### Copy

- `update_linked_list_and_copy_data_3`: Traverses a linked list to find the maximum value in a specific field, then updates a header and performs a series of memory copies between a structure and a destination buffer.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer.

### Makedir

- `find_max_and_initialize_buffer`: Traverses a linked list to determine the maximum value in a specific field and then initializes a buffer with a sequence of incrementing values.
- `copy_memory_block_and_call_6`: Copies sixteen bytes from a memory location offset by a register value into a destination address, then calls another routine.

### List

- `find_max_and_update_list`: Iterates through a linked list to find the maximum value in a specific field.
- `copy_memory_block_and_call_7`: Copies a fixed number of words from a memory location to a destination pointed to by a stack pointer, then calls another routine.

### CD

- `update_linked_list_and_copy_data_4`: Traverses a linked list to find the maximum value in a specific field and then updates a structure with this value and a sequence of incrementing integers.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_populate_buffer_2`: Traverses a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a buffer.
- `copy_memory_block_4`: Copies sixteen bytes from a source address to a destination address using a loop.

### LoadWB

- `parse_command_line_into_words`: Splits a command-line string into words at spaces and tabs, keeping double-quoted spans together and translating the asterisk escapes for newline and escape.
- `release_state_and_return_code`: Stores the supplied result code in a state record, then releases the two resources the record holds through library jump calls when their pointers are non-null.
- `run_loadwb_startup_sequence`: Opens a library through an indirect call and returns error code twenty when it fails, then allocates a buffer and XOR-processes a global name string twice with the same key, so it ends unchanged.
- `compare_strings_ordered`: Compares two zero-terminated strings byte by byte.
- `build_linked_node_chain`: Walks an index-linked table, allocating an eight-byte node for each entry and storing a library lookup result of the entry's value in the node.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Calls a library routine through a jump vector at a fixed negative offset from the global library base, then returns.
- `execute_library_offset`: Calls a function pointer located at a specific offset from a global variable.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `match_name_and_pad_field`: Compares the record's name with a global name string character by character.
- `write_dot_padding_field`: Writes a period followed by four spaces into five consecutive bytes of the record addressed by the A2 register.
- `compare_record_prefix_with_name`: Compares the record at the A2 pointer with the global name string byte by byte, stopping when the name ends or a byte differs.
- `stub_return_only`: Contains only a return instruction, so calling it performs no work.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_dispatch_pointer`: Calls a function pointer located at an offset from a global variable.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `forward_args_with_stack_pointer`: Forwards its two parameters to a three-parameter routine and passes a pointer into the caller's stack frame as the third argument.
- `write_local_text_when_nonempty`: Calls a library routine, then measures a 140-byte stack text buffer by scanning for its terminating zero.
- `store_low_byte_via_pointer`: Stores the low byte of data register D0 through the A3 pointer and returns.

### Version

- `parse_version_command_line`: Allocates a state record and returns error code twenty when that fails, then opens a library and either copies a stored path or sets up a default entry.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `print_library_version_line`: Handles the version command arguments: when the first argument is a question mark it writes the global text and returns.
- `forward_with_size_128`: Passes its first parameter together with the number 128 to one routine, then passes its second parameter to a second routine.
- `parse_signed_decimal_value`: Reads an optional plus or minus sign and then accumulates decimal digits into an integer, negating the result for a minus sign.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `measure_zero_terminated_length`: Returns the number of characters before the terminating zero of the string at the given pointer.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_indirect_function`: Fetches a function pointer from a table using a fixed offset and executes it.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `execute_indirect_function_2`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_offset_pointer`: Calls a function pointer located at a specific offset within a global structure.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_offset_pointer_2`: Executes a routine by dereferencing a pointer found at a specific offset from a global variable.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer`: Stores a single byte from a register into a location pointed to by another register.

### SetMap

- `parse_setmap_command_line`: Allocates a state record and returns error code twenty when that fails, then opens a library and either records a looked-up entry in several fields or splits the command-line text into words with quoted spans and asterisk escapes.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_offset_pointer_3`: Performs a call to a function address stored at a fixed offset from a global variable.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `call_offset_pointer_4`: Invokes a subroutine through a pointer located at a specific offset from a global base address.
- `free_table_entries_and_resources`: Releases each non-null entry of a counted pointer table, then releases the saved pointers that are set and finally the value passed in.
- `store_args_and_report_failure`: Saves its two parameters in global slots, makes one library call with a fixed set of constant arguments, and then reports failure code twenty through the error routine.
- `select_keymap_and_lookup_entries`: Chooses the name from the mode argument, lowercases its letters, and looks for matching entries, copying up to thirty-two characters into a local buffer and searching again when none are found.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_offset_pointer_5`: Calls a routine via a pointer stored at a specific offset within a global structure.
- `call_offset_pointer_6`: Dereferences and calls a function pointer at a specific offset from a global variable.
- `call_offset_pointer_7`: Executes a function pointer retrieved from a specific offset within a global structure.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_offset_pointer_8`: Calls a routine located at a specific offset from a global variable.
- `call_offset_pointer_9`: Invokes a function through a pointer found at a specific offset from a global structure.
- `execute_indirect_function_3`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_indirect_handler_two`: Executes a function pointer retrieved from a global variable at a fixed offset.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_handler_three`: Invokes a subroutine located at an offset from a global data pointer.
- `call_indirect_handler_four`: Performs an indirect call to a routine address found in a global table.
- `call_library_vector_96`: Calls a library routine through a jump vector at a fixed negative offset from the base held in a global slot, then returns.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_value_with_stack_pointer`: Passes a stored global value, its one parameter and a pointer into the caller's stack frame to a three-parameter routine.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer_2`: Stores a byte from a register into a memory location pointed to by another register.

### More

- `parse_more_command_line`: Allocates a state record and returns error code twenty when that fails, then opens a library and either copies a stored path or sets up a default entry.
- `update_and_cleanup_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `run_more_pager_loop`: Allocates a 4924-byte working record and reports failure through the error routine when that fails, then records whether a file argument was given and opens the named input through library lookups.
- `send_four_values_to_routine`: Passes each of four global slots, in order, together with a field of the record to a two-argument routine.
- `scan_for_control_bytes`: Calls a library routine to fill up to eighty bytes of the record's text field, then checks each byte.
- `read_pager_key_command`: Waits for input on the record's port with a timeout, and when the break signal is set it marks the record as aborted and returns zero.
- `finish_with_vector_call`: Calls two routines in turn, then jumps through the A3 vector.
- `refresh_pager_display_until_done`: Resets a record field, reads the first page through a library call and then calls the refresh routine repeatedly while a counter is positive, the window's line position is still below its limit, and the stop flag is clear.
- `process_pager_line_text`: Runs a text-processing call on the record's text buffer and stores its result in the record.
- `read_next_pager_line`: Reads up to 128 bytes of the input into the buffer, terminates the string at the end of the first line or form feed, and returns the number of characters kept.
- `draw_page_until_quit_key`: Chooses how many lines a page holds from two record flags, then repeatedly draws lines from the global table and reads a key.
- `read_and_display_until_end`: Prepares the record's text buffer and an input call, then reads and processes input blocks until the remaining count is below one or the end test succeeds.
- `print_message_then_run_routine`: Prints a message through the record's output call, prints a second message when a flag is set, runs a routine on the second argument, finishes the output and returns that routine's result.
- `call_routine_with_conditional_followup`: Calls a routine with its two arguments, then calls the first argument's routine again only when a flag in the record is clear.
- `seek_and_report_line_count`: Calls a routine with the record's position and a fixed minus-one argument, then passes the window's line-count field to a second routine.
- `seek_and_report_last_line`: Calls a routine with the record's position, a zero offset and a mode of one, then passes the window's line count minus one to a second routine.
- `scroll_input_back_by_lines`: Clears the record's stop flag, then moves the input back by the requested number of lines.
- `print_text_after_record_setup`: Calls a routine on the record, then passes the record's output handle, a global text and the first argument to a formatting routine.
- `print_page_prompt_message`: Sets the window's line position to one when the record's first flag is zero, then writes one of two global text messages through the output call, chosen by that flag.
- `print_line_position_message`: Writes a global text message through the output call.
- `print_prompt_for_position`: Writes a global text through the output call.
- `query_size_clamped_nonnegative`: Calls a routine with a zero offset and mode one, then returns the result of a second call with a mode of minus one, clamped to zero when that result is negative.
- `copy_buffered_bytes_to_caller`: Copies up to the requested number of buffered bytes into the caller's buffer, refilling the buffer through a read routine first when more bytes are wanted than are buffered.
- `refill_input_buffer`: Refills the record's 4096-byte input buffer by moving any unread bytes to the start and reading into the remainder, then updates the counters.
- `seek_input_position`: Seeks in the input to an offset relative to the start, the current position or the end, as selected by the mode argument, and returns the previous position or minus one for an invalid mode.
- `read_line_into_buffer`: Reads single bytes from the input, storing each into the buffer, until a newline is read or after at most 127 characters.
- `copy_zero_terminated_string`: Copies a zero-terminated string into the destination buffer, including its terminating zero.
- `uppercase_string_in_place`: Converts each lowercase letter a to z in a zero-terminated string to uppercase, leaving all other bytes unchanged, up to the terminating zero.
- `parse_signed_decimal_value_2`: Reads an optional plus or minus sign and then accumulates decimal digits into an integer, negating the result for a minus sign.
- `conditionally_run_handle_operation`: Calls a routine on the stored handle with the first argument and a minus-one argument, but only when the first argument is nonzero and the stored handle is set.
- `release_pager_record`: Tears down the pager state.
- `send_pager_message_code`: Passes its argument, placed in a one-entry argument list, together with the number 994 to a routine on the stored library base, and returns that routine's result.
- `query_port_info_into_record`: Allocates a 36-byte record and returns zero when that fails.
- `run_with_copied_argument_block`: Returns zero when the first argument is zero or the lookup fails.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule_handler`: Calls a routine via a pointer located at a specific offset within a global structure.
- `call_switch_context`: Calls a function pointer stored at an offset within a global table.
- `call_dispatch_function`: Executes a routine located via a pointer in a global table.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem_handler`: Invokes a function pointer retrieved from a global table.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_function_a`: Calls a function pointer retrieved from a global table.
- `call_library_function_b`: Executes a routine through a pointer found in a global table.
- `call_library_function_c`: Performs a call to a function pointer stored in a global table.
- `call_library_function_d`: Calls a routine via a pointer located in a global table.
- `execute_indirect_function_4`: Fetches a function pointer from a location within a global table and executes it.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_function_f`: Executes a function pointer stored in a global table.
- `call_library_function_g`: Calls a routine through a pointer offset from a global table.
- `execute_indirect_call`: Calls a function pointer retrieved from a specific offset within a global structure.
- `execute_indirect_call_2`: Invokes a routine located at a fixed offset from a pointer stored in a global variable.
- `execute_indirect_call_3`: Performs an indirect call to a function address found within a global data structure.
- `execute_indirect_call_4`: Retrieves a function pointer from a global table and executes it.
- `call_with_stack_slot_2`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `read_byte_or_minus_one`: Reads one byte through a routine into a local buffer.
- `invoke_and_checksum_stack_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack-allocated buffer until a null terminator is found.
- `store_byte_to_pointer_alt`: Writes a single byte to an address held in a register.
- `divide_unsigned_longword`: Divides an unsigned 32-bit dividend by an unsigned 32-bit divisor and returns the quotient in the low longword and the remainder in the high longword of a 64-bit result.
- `multiply_longwords_partial_products`: Builds a 64-bit product of two 32-bit inputs from 16-bit partial products.
- `call_adjacent_routine_and_return`: Calls one other routine with no arguments and returns.
- `negate_call_result_when_flagged`: Calls another routine, then returns its result negated when the A0 register left by that call is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_pointer_alt2`: Transfers a byte from a register to a destination specified by a pointer.
- `execute_indirect_call_5`: Executes a routine by dereferencing a pointer stored at a constant offset from a global variable.
- `create_linked_request_record`: Allocates a 34-byte record when a library lookup succeeds, stores the first parameter, the flag byte and a second looked-up value in it, then calls one of two library routines depending on whether the first parameter is zero.
- `release_request_record`: Releases the request record.
- `find_name_in_text`: Searches forward through the text for the zero-terminated name in its second argument, returning one when the name is found and zero when the text ends.
- `initialize_empty_list_header`: Initialises an empty list header in place: the first longword points just past itself, the second is zero, and the third points back at the header.

### Format

- `initialize_filesystem_environment`: Initializes internal structures by querying the current directory and opening necessary files.
- `reset_resource_state`: Retrieves a pointer from a global structure and updates a field within a secondary structure with a provided parameter.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `execute_indirect_call_6`: Calls a routine through a pointer located at a specific offset from a global variable.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `execute_indirect_call_7`: Executes a function pointer stored at a constant offset from a global variable.
- `call_forbid_handler`: Executes a function pointer stored at an offset from a global variable.
- `call_dispatch_handler`: Calls a routine located via a pointer offset from a global variable.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_schedule_handler`: Performs an indirect call to a routine retrieved from a global variable.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_allocabs_handler`: Invokes a function pointer found at a specific offset within a global variable.
- `call_reschedule_handler_2`: Calls a routine through a pointer stored in a global variable.
- `setup_format_working_resources`: Allocates working resources, including a 36-byte record and a 260-byte buffer, and returns error code twenty when any of them is unavailable.
- `release_format_working_resources`: Releases the working resources obtained by the setup routine, freeing the 260-byte buffer, the 36-byte record and the other allocated items when they are set, then releases the value passed in.
- `read_console_key_with_mapping`: Reads one character through a library call, repeating the read while it is the sequence-start code.
- `emit_key_response_and_return`: Writes one of two responses through the output call, chosen by a register value, then returns the character it was given as a signed value.
- `store_matched_table_code`: Searches an eight-byte table for the given byte value and stores the matching value, or zero when there is no match, in a field of the working record.
- `find_matching_list_entry`: Walks a linked list of entries and, when an entry matches the given value or name, runs two library calls on it and returns one.
- `calculate_string_length`: Returns the number of characters in a null-terminated string before its terminator.
- `copy_cstring_to_buffer`: Library code that copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers and writing each byte until a zero byte is encountered.
- `uppercase_character`: Returns the uppercase form of a byte when it is a lowercase letter a to z, and the byte unchanged otherwise.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings after converting each character to uppercase.
- `terminate_string_at_colon`: Shortens the string in place at its first colon, or at its end when there is no colon, by writing a zero there.
- `send_single_word_message`: Places its first argument in a one-entry argument list and passes it with a fixed code to a library routine.
- `run_with_word_argument_block`: Returns zero when the lookup fails.
- `call_library_offset_handler`: Executes a routine by dereferencing a pointer at an offset from a global variable.
- `call_library_offset_handler_alt`: Performs an indirect call to a routine using an offset from a global variable.
- `call_indirect_function`: Fetches a function pointer from a table and executes it.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_offset_handler_gamma`: Invokes a routine through a pointer offset from a global variable.
- `call_library_offset_handler_delta`: Executes a routine stored at an offset from a global variable.
- `call_indirect_handler`: Executes a routine by dereferencing a pointer stored at a specific offset within a global table.
- `call_indirect_handler_2`: Calls a function located at an offset from a global variable.
- `call_indirect_handler_3`: Performs an indirect call to a routine found via an offset in a global table.
- `call_indirect_handler_4`: Dereferences a pointer from a global table and jumps to that address.
- `call_indirect_handler_5`: Invokes a subroutine by accessing a pointer at a fixed offset from a global base.
- `call_indirect_handler_6`: Calls a subroutine through a pointer stored in a global data structure.
- `call_indirect_handler_7`: Executes a function pointer retrieved from a global table using a specific offset.
- `call_indirect_handler_8`: Jumps to a routine address stored at an offset within a global variable.
- `call_indirect_handler_9`: Performs an indirect jump to a routine address located in a global table.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_indirect_handler_10`: Calls a routine by following a pointer at a defined offset from a global base address.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `execute_indirect_call_8`: Calls a function through a pointer retrieved from a global table.
- `forward_argument_to_library_stub`: Passes its one incoming argument, a library base pointer read from a global variable, and the address of its own stack argument to the library call stub at offset minus 132.
- `execute_indirect_function_8`: Fetches a function pointer from a global table and executes it.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `write_dos_sector_tracks`: Opens the disk device, then for each track in the requested range fills a buffer with sector images carrying the DOS disk signature and writes it through the device by library calls.
- `run_device_control_sequence`: Makes three device calls on the disk device at its stored offset: two control calls with fixed argument values and a third that passes along a value obtained from another call.
- `invoke_and_checksum_stack`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over the stack buffer until a null terminator is found.
- `store_byte_via_pointer`: Stores a single byte from a data register into a memory location pointed to by an address register.
- `divide_32_bit_integers`: Performs division on a 32-bit value using a shift-and-subtract loop or 16-bit partial division.
- `divide_shift_subtract_loop`: Runs the sixteen-step shift-and-subtract division loop on the dividend in the D1 register against the divisor, leaving the partial quotient and remainder in registers.
- `multiply_32_bit_integers`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry_2`: Retrieves a function pointer from an offset within a global data structure and executes it.
- `store_byte_via_pointer_2`: Stores a single byte from a data register into the memory location pointed to by an address register.
- `allocate_and_initialize_block`: Calls a library function to allocate a memory block, fills in fields including a stored parameter and flag bytes, and calls further library functions, returning the block address or zero.
- `reset_object_and_recurse`: Recursively processes a linked structure when a pointer field is non-zero, sets a byte field to all ones and a four-byte field to all ones, then calls another routine on a byte field value and again with a constant second argument.
- `forward_with_fixed_code_48`: Passes its argument to a routine together with the number 48.
- `pass_single_argument_through`: Passes its one argument straight through to a routine and returns.
- `initialise_drive_parameters`: Saves a field of the working record, sets it to a sentinel while looking up the drive record, then fills several global geometry fields from that record or from fixed defaults, and restores the field.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `execute_indirect_function_5`: Fetches a function pointer from a location within a table and executes it.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores a pointer into a memory location, increments that pointer by four, clears the next word, and stores the original pointer in the following word.
- `allocate_sized_record_with_link`: Returns zero when the first argument is zero or the allocation of the requested size fails.
- `reset_sized_record_state`: Returns zero for a null record.
- `execute_indirect_function_6`: Fetches a function pointer from a location calculated by adding an offset to a global variable and executes it.
- `execute_indirect_function_7`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_and_start`: Opens the DOS library and allocates a control block to process a command line string.
- `call_supervisor_function`: Calls a function pointer retrieved from a specific offset within a global data structure.
- `call_exit_interrupt_handler`: Executes a routine located at a fixed offset from a global pointer.
- `call_switch_function`: Performs an indirect call to a function pointer stored in a global table.
- `call_dispatch_function_2`: Invokes a subroutine through a pointer found at a specific offset in a global variable.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions_handler`: Calls a function pointer stored at a calculated offset from a global base address.
- `call_find_resident_handler`: Executes a function pointer retrieved from a global data structure.
- `call_enable_handler`: Calls a routine via a pointer located at a specific offset from a global variable.
- `call_alloc_entry_handler`: Invokes a function pointer stored at a large offset within a global data structure.
- `setup_cli_process_session`: Looks up the current process record and, when the first argument is zero, allocates a 64-byte block with two fixed marker values and records a looked-up path and flag in it.
- `build_linked_node_chain_shifted`: Walks an index-linked table, allocating an eight-byte node for each entry and storing a library lookup result of its value in the node, then chains each node by storing its shifted address in the previous node's first word.
- `walk_node_chain_with_callbacks`: Follows a chain of linked records, where the first word of each record holds the link to the next.
- `parse_signed_decimal`: Reads an optional plus or minus sign and then decimal digits from a null-terminated string, accumulating the digits into an integer.
- `call_system_library_allocate`: Calls the allocation entry of the system library through its jump vector, with no arguments set here, and returns nothing.
- `call_system_library_free`: Calls the release entry of the system library through its jump vector, with no arguments set here, and returns nothing.
- `call_system_library_find_task`: Calls the task lookup entry of the system library through its jump vector, with no arguments set here, and returns nothing.
- `call_system_library_close`: Calls the library close entry of the system library through its jump vector, with no arguments set here, and returns nothing.
- `call_system_library_open`: Calls the library open entry of the system library through its jump vector, with no arguments set here, and returns nothing.
- `call_dos_rename_entry`: Calls the rename entry of the second library, whose base is held in a global, through its jump vector, with no arguments set here, and returns nothing.
- `call_dos_unlock_entry`: Calls the unlock entry of the second library, whose base is held in a global, through its jump vector, with no arguments set here, and returns nothing.
- `call_dos_duplicate_lock_entry`: Calls the duplicate-lock entry of the second library, whose base is held in a global, through its jump vector, with no arguments set here, and returns nothing.
- `divide_32bit_with_remainder`: Divides a 32-bit dividend by a 32-bit divisor.
- `multiply_32bit_low_result`: Multiplies two 32-bit values from the data registers using 16-bit partial products and keeps the low 32 bits.
- `call_and_negate_on_flag`: Calls a routine and returns its first result, negated when a second value returned by that routine is nonzero.

### Disk-Validator

- `validate_disk_bitmap_words`: Writes a two-digit count into a status buffer, then walks a range of bitmap words and compares each bit against a stored copy.

### Ram-Handler

- `copy_memory_then_dispatch`: Calls the system memory copy routine through the library vector, then transfers control through a target taken from a computed table.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:AM2_BLIT

- `clear_blitter_task_word`: Runs a sequence of library calls and a base-address routine, then reads a pointer from the task structure.
- `decrement_until_minus_one`: Decrements a 16-bit counter one step at a time until it wraps to minus one, then returns.

### d

- `null_routine`: The routine is empty and contains no executable instructions.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:AM2_CPU

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_until_minus_one_2`: Decrements a 16-bit counter one step at a time until it wraps to minus one, then returns.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:Ambermoon

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_register_to_negative_one`: Decrements a 16-bit register until its value reaches negative one and then returns.
- `copy_words_then_flush_cache`: Runs a loop four times that calls a bit-stream routine and copies words from one buffer into another, then flushes the processor caches through the system library when a global counter exceeds thirty-six.
- `unpack_backward_bitstream`: Unpacks a compressed block by reading control bits from the end of the input and copying literal bytes backward into the output, with run lengths decoded from the bit stream.
- `decode_backward_packed_block`: Decodes a packed block by reading bits from the end of the input and copying bytes backward into the output, using a table of length codes to size each copy run.
- `unpack_backward_literal_runs`: Copies literal bytes backward from the end of the input, four bytes per pass, with run lengths decoded from the bit stream deciding how many bytes each run copies before the loop ends.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:Ambermoon_install

- `reset_install_state_at_entry`: Program entry for Ambermoon_install that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_word_until_negative_one`: Decrements a 16-bit value stored in a register until it reaches negative one.
- `copy_words_then_flush_cache_2`: Runs a loop three times that calls a bit-stream routine and copies words from one buffer into another, then flushes the processor caches through the system library when a global counter exceeds thirty-six.
- `verify_checksum_buffer`: Performs a checksum validation on a memory buffer by iterating through data and applying bitwise operations.
- `copy_memory_block_optimized`: Copies a block of data from a source address to a destination address using a loop.
- `copy_memory_block_7`: Copies a block of data from a source address to a destination address using a loop.
- `null_routine_2`: This routine is empty and contains no executable instructions.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a5].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `invoke_library_routine`: Calls one library routine through its vector, with no arguments set here, and returns nothing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_task_guarded_setup`: Brackets a sequence of library calls with a task forbid and permit pair, looking up the current task and freeing or allocating memory, then stores a status value in a global and returns it.
- `allocate_and_open_checked_handle`: Gets a value from a routine, allocates a block through the system library and calls the open entry of the second library, comparing returned values.
- `examine_locked_object_field`: Locks an object through the second library and examines it.
- `dispatch_table_entry_by_index`: Looks up an index through a library call.
- `open_then_close_file`: Opens a file through the second library and, if the open succeeds, stores the handle in a global, calls three routines and closes the handle.
- `open_show_error_then_close`: Opens a file through the second library and, if the open succeeds, calls a sequence of routines.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `write_while_data_pending`: Calls the write entry of the second library in a loop that repeats while a check routine reports more data, and calls a routine after each write.
- `write_if_check_passes`: Calls the write entry of the second library once, but only when a check routine returns a nonzero value.
- `find_word_in_terminated_table`: Scans a table of 16-bit entries that ends with a minus-one marker, stepping forward by the length of each entry, until it finds an entry equal to the value in the data register.
- `measure_string_length`: Counts the characters of a null-terminated string addressed by the address register and returns the count in the data register.
- `wait_blitter_then_spin`: Clears a state word and, when a flag bit is set, waits until the blitter is idle, installs a routine as the interrupt handler pointer and then loops forever.
- `store_constant_status_value`: Stores the constant sixty-four into a global status cell and returns.
- `probe_floppy_drive_entry`: When a state cell is zero, it attempts a lock through the second library and records whether a returned status equals a fixed code.
- `parse_unit_number_after_prefix`: Compares the input string against a fixed prefix stored in the program.
- `set_flag_if_field_zero`: Sets a global flag to minus one when a field of the structure it reads is zero, and to zero otherwise.
- `wait_for_port_message`: Waits on a message port, takes the message from it through the system library, and calls the enable routine of the library when a field of that message is nonzero.
- `release_forbid_and_reply`: When a message pointer is set, forbids task switching, replies to the message through the system library and clears the pointer.
- `open_library_and_read_version`: Opens libraries through the system open entry, storing each returned base in a global, and finally reads the 16-bit version field of the last base into the global.
- `close_held_libraries`: Closes the library whose base is held in a global through the system close entry, once for each of three checks that find the global nonzero.
- `call_entry_and_store_flag`: Calls a library entry through its vector and stores minus one in a global when the call returns nonzero, and zero otherwise.
- `step_to_parent_if_flagged`: When a flag in a global is set, calls the parent-directory entry of the second library through its vector, with no arguments set here, and returns nothing.
- `setup_shared_record_state`: When a flag is clear, allocates memory and runs several system library calls, exchanges a field of a shared record with a saved value, and when a lookup succeeds, does the same for a second record.
- `clear_shared_record_entry`: When a flag is set, stores a value in a shared record, looks up and closes an entry through the second library, calls two more library entries and then clears the flag.
- `store_current_task_pointer`: Calls the task lookup entry of the system library and stores the returned task pointer in a global.
- `store_adjusted_char_then_call`: When a flag is clear, stores the character at the input pointer plus sixty-four into a fixed byte cell and calls a routine.
- `copy_null_terminated_string`: Copies characters from the source pointer to the destination pointer, one at a time, until a zero byte has been copied.
- `set_flag_if_field_below`: Sets a global flag to minus one when a 16-bit field of a record, reduced by thirty-six, is negative, and to zero otherwise.
- `setup_device_io_request`: When a flag is clear, allocates a signal through the system library and, if that succeeds, opens a device and sends a prepared I/O request whose fields are set beforehand.
- `clear_flag_after_library_call`: When a flag is set, calls a library entry and then clears the flag.
- `try_lock_with_fallback`: When a flag is clear, temporarily sets a field of a shared record to minus one, attempts a lock through the second library and restores the field.
- `invoke_fallback_library_entry`: Calls one library entry through its vector, with no arguments set here, and returns nothing.
- `configure_port_when_ready`: When a 16-bit field minus thirty-nine is nonnegative, calls a library entry.
- `clear_flag_after_library_calls`: When a flag is set, calls a library entry and, if it returns zero, calls two more library entries and clears the flag.
- `set_flag_and_enter_supervisor`: Sets a global to minus one when bit one of a status byte is set, and to zero otherwise, then enters supervisor state through the system library.
- `read_vector_base_register`: Copies the processor's vector base register into a global.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_status_gated_library_call`: Calls a routine, calls a library entry only when a status bit of a record is set, and then runs one of two alternative paths depending on the result of a second library call.
- `run_install_menu`: Shows the installation menu with options to install the game on the hard disk, create a save-state disk, view the troubleshooting document or quit.
- `dispatch_ram_command_lines`: Loops over command dispatch steps that refer to the game's command files in RAM, such as Copy, Format, Install, More and Run, and branches on a character code read from a line buffer.
- `prompt_and_install_to_path`: Prompts for a destination path, checks it with lock and examine calls, and copies the game files from the install disk, with German status messages shown during the copy.
- `read_single_input_into_buffer`: Reserves a ten-byte stack buffer and passes it with a count of one to a routine that takes the global handle, returning nothing.
- `copy_string_up_to_limit`: Formats a string into a 100-byte stack buffer, then copies it to the destination.
- `store_table_entry_and_dispatch`: Copies one word from a table, indexed by a register, into a fixed field.
- `run_copy_command`: Formats a copy command line from two text arguments into a 200-byte buffer and runs it through the system execute entry.
- `prompt_until_yes_or_no`: Repeats a prompt and reads a character until it is a J or an N, then returns the character read.
- `create_directory_then_unlock`: Calls the directory creation step with its argument and, when that returns a lock handle, releases the lock through the second library.
- `generate_save_directories`: Shows a German status message that save directories are being generated, then creates eleven directories with names built from a two-digit suffix, releasing each lock after creation.
- `generate_save_file`: Shows a German status message that a save file is being generated, opens a file, copies two names into a buffer, and then writes or removes data with a fixed block size depending on a check.
- `read_file_size_by_lock`: Allocates a 260-byte info block, locks the given path in shared mode, examines it into the block and returns the 32-bit size field of the block.
- `copy_file_with_error_messages`: Checks that a source file exists and can be read and that the destination can be written, then copies the file.
- `check_install_disk_drive`: Checks the install folder and the install program on the disk.
- `set_flag_if_field_above`: Sets a global flag to one when a 16-bit field of a record is greater than thirty-five, and to zero otherwise.
- `test_lock_and_release`: Attempts to lock a named path through the second library, releases the lock if one was obtained, and returns whether the lock succeeded.
- `build_full_path_upwards`: Walks upward through the parent directories of the current directory, reading each directory name into a path buffer with slash separators.
- `copy_string_to_buffer`: Copies bytes from the source string to the destination, including the terminating zero, and returns the destination pointer.
- `compute_string_length`: Walks a zero-terminated string and returns the number of characters before the terminator.
- `convert_char_to_uppercase`: Returns the input character converted to uppercase when it is a lowercase letter, and unchanged otherwise.
- `convert_char_to_lowercase`: Returns the input character converted to lowercase when it is an uppercase letter, and unchanged otherwise.
- `run_formatted_print`: Clears a status global and passes its argument and the caller's stack argument list to the formatted output routine.
- `select_stream_handle`: When the argument is zero, stores either the value a routine returns for a fixed stream, or minus one when a status bit is set.
- `format_printf_style_text`: Walks a format string, copying literal characters through a character output routine and expanding each percent specifier.
- `skip_whitespace_then_peek`: Reads characters through the stream routine until one is not in the space class, then pushes it back.
- `scan_signed_integer_in_base`: Reads an optional sign and then digits of a given base through the stream routine, accumulating the value by digit position and storing the signed result.
- `read_nonzero_char_with_eot`: Reads characters until a nonzero result, returning minus one at end of input.
- `read_buffered_byte`: Returns the next byte of a buffer when the read position is before the end, advancing the position.
- `refill_buffer_and_read_byte`: Refills a read buffer through a block read routine when it is empty, closes flagged entries when a flag is set, sets an end or error flag from the result, and returns the next byte or minus one.
- `write_char_to_buffer`: Stores one character at the write position of a buffer and advances it, returning the character.
- `find_char_in_string`: Scans a zero-terminated string for a given character and returns a pointer to its first occurrence, or null when the terminator is reached first.
- `read_file_slot_with_error`: Checks that a file slot index is valid and that the slot is open for reading, setting an error code and returning minus one otherwise.
- `return_immediately`: Returns at once without doing any work.
- `create_tagged_context_block`: Allocates a context block sized as six times a global count, initializes its header fields, stamps a four-letter tag into it and links it to the current process record.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `build_command_line_arguments`: Allocates a buffer for a command line, copies the command name and the given text into it, skips whitespace and splits the remaining text into arguments, each terminated with a zero byte.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `append_string_bounded`: Appends a source string to the end of a destination string, copying at most 32766 characters, and terminates the result.
- `append_counted_string`: Appends up to a given number of characters from a source string to the end of a destination, stopping at the source terminator.
- `copy_string_with_zero_padding`: Copies characters from a source string to a destination for up to a given count, stopping at the source terminator, then fills the remaining positions with zero bytes.
- `setup_icon_window_record`: Opens the icon library and looks up entries by name, storing the found pointer and two values in the window record it receives, then releases what it locked.
- `call_and_clear_first_byte`: Stores the buffer pointer in a global, calls a routine with that value and a fixed stack argument, then clears the first byte of the buffer and returns the routine's result.
- `format_string_output`: Sets a global pointer to a destination, processes a formatted string, and terminates the output with a null byte.
- `store_char_and_advance`: Writes one byte to the output pointer held in a global, advances that pointer and returns the byte.
- `call_with_stack_arguments`: Passes its argument and the caller's stack argument list to a routine, with no return value.
- `write_digits_backward`: Writes the digits of a number backward into a buffer from the end, using a digit table, and prepends a minus sign when the value is negative.
- `format_conversion_with_padding`: Processes a format string, copying literal characters through a character output routine and expanding each conversion specification with its sign, zero fill and width.
- `perform_signed_division`: Calls a division routine on its operands and negates the quotient when exactly one operand is negative.
- `call_and_return_register_pair`: Calls a routine and returns a pair built from the data register it was given and the register value the call left behind.
- `perform_recursive_call`: The routine calls another routine and then returns.
- `unsigned_divide_with_remainder`: Divides a 32-bit dividend by a 32-bit divisor.
- `call_with_fixed_buffer`: Calls a routine with its argument and a fixed buffer address, and returns nothing.
- `output_char_flushing_newline`: Passes its character and stream to an output routine.
- `store_char_in_buffer`: Stores one character into a buffer while there is room, advances the position and returns the character.
- `process_each_22byte_record`: Walks a table of 22-byte records up to a fixed end address and calls a routine on each record, returning nothing.
- `release_record_resources`: Releases the resources a record holds according to its flag bits, returns the accumulated status, or minus one for a null record, and zeroes the record's fields.
- `release_record_flagged_resources`: Releases the resources a record holds according to its flag bits, returns the accumulated status, and zeroes the record's fields.
- `write_char_to_stream`: Returns minus one when the stream is in an error state.
- `allocate_free_record_slot`: Scans a table of 22-byte records for one whose in-use flag is clear, clears its fields and returns it, or null when every record is in use.
- `find_free_record_from`: Scans forward from the given record for one whose in-use flag is clear, clears it and returns it, or null at the end of the table.
- `clear_record_fields`: Zeroes the three words at the start of the record addressed by the address register.
- `allocate_record_buffer`: Allocates a 1024-byte buffer for a record.
- `free_linked_node_list`: Walks a singly linked list from the global head, releasing each node through a memory routine with a size equal to the node's stored size plus eight, then clears the head.
- `free_node_chain_from_argument`: Walks a linked chain starting at the address register, releasing each node through a memory routine with a size equal to its stored size plus eight, and clears the global head when done.
- `push_node_onto_list`: Allocates a node sized from the argument plus eight, links it at the front of the global list with the given owner, and returns a pointer past the link words.
- `forward_argument_to_routine`: Passes its argument to a routine and returns nothing.
- `remove_node_from_list`: Searches the global list for the node whose data pointer matches the argument, unlinks it and releases it through a memory routine.
- `remove_node_matching_owner`: Searches a chain for the node that belongs to the owner structure passed in, unlinks it and releases it.
- `unlink_node_and_release`: Unlinks the node in the address registers from the chain, updating the head when the node is first, and releases it through the memory routine.
- `set_list_head_from_pointer`: Stores the value read from the address register into the global list head, then calls a routine and returns zero.
- `check_file_slot_interactive`: Checks that a file slot index is valid and occupied, setting an error code and returning minus one otherwise.
- `delete_file_with_error`: Deletes a file through the second library and returns zero when it succeeds.
- `return_zero`: Returns zero without doing any work.
- `write_file_slot_with_error`: Checks that a file slot index is valid and open for writing, setting an error code and returning minus one otherwise.
- `check_break_signal`: Checks the task's signal bits for the break signal.
- `write_global_word_block`: Passes a four-byte global cell to a write routine along with the output handle, then calls a routine with the value one, returning nothing.
- `call_optional_hook_then_routine`: Calls an optional hook routine when one is installed, then passes its argument to another routine and returns nothing.
- `release_global_record_resources`: Releases a group of resources held in a global record: it frees each sized entry in a loop, calls the optional hook, unlocks and drops the lock entries, then frees the record itself.
- `nop_padding_sequence`: This routine is empty and consists only of padding instructions.
- `close_file_slot`: Checks that a file slot index is valid and in use, closing its handle through the second library unless the slot is marked as preserved, then clears the slot and returns zero.
- `multiply_32bit_partial_sum`: Forms the cross products of the 16-bit halves of two 32-bit data registers and returns their sum shifted into the high word, together with the second operand register.
- `close_file_handle`: Calls the close entry of the second library through its vector, with no arguments set here, and returns nothing.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_dir`: Wrapper that calls the dos library function CreateDir and returns its result to the caller.
- `call_current_directory_entry`: Calls the current-directory entry of the second library through its vector, with no arguments set here, and returns nothing.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `delete_file_entry`: Calls the delete-file entry of the second library through its vector, with no arguments set here, and returns nothing.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_execute`: Wrapper that calls the dos library function Execute and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `fetch_io_error_code`: Calls the I/O error entry of the second library through its vector, with no arguments set here, and returns nothing.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `open_file_entry`: Calls the open entry of the second library through its vector, with no arguments set here, and returns nothing.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_parent_dir`: Wrapper that calls the dos library function ParentDir and returns its result to the caller.
- `read_file_entry`: Calls the read entry of the second library through its vector, with no arguments set here, and returns nothing.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `write_file_entry`: Calls the write entry of the second library through its vector, with no arguments set here, and returns nothing.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `close_system_library_entry`: Calls the library close entry of the system library through its vector, with no arguments set here, and returns nothing.
- `call_indirect_function_2`: The routine calls a function pointer stored at an offset within a global variable.
- `allocate_memory_entry`: Calls the memory allocation entry of the system library through its vector, with no arguments set here, and returns nothing.
- `call_indirect_function_3`: The routine executes a function through a pointer located at a specific offset from a global variable.
- `find_task_entry`: Calls the task lookup entry of the system library through its vector, with no arguments set here, and returns nothing.
- `call_indirect_function_4`: The routine performs an indirect call to a function address retrieved from a global variable.
- `forbid_task_switching`: Calls the forbid entry of the system library, which stops task switching, and returns nothing.
- `free_memory_entry`: Calls the memory release entry of the system library through its vector, with no arguments set here, and returns nothing.
- `call_indirect_function_5`: The routine jumps to a function address found by applying an offset to a global variable.
- `get_message_from_port`: Calls the get-message entry of the system library through its vector to take a message from a port, and returns nothing.
- `open_system_library_entry`: Calls the library open entry of the system library through its vector, with no arguments set here, and returns nothing.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `invoke_system_library_routine`: Calls a routine of the system library through its vector, with no arguments set here, and returns nothing.
- `wait_on_message_port`: Calls the wait entry of the system library through its vector, which waits for a message on a port, and returns nothing.
- `duplicate_lock_entry`: Calls the duplicate-lock entry of the second library through its vector, with no arguments set here, and returns nothing.
- `unlock_entry`: Calls the unlock entry of the second library through its vector, with no arguments set here, and returns nothing.
- `empty_routine`: The routine consists of two instructions and performs no visible library calls or register operations in the provided data.
