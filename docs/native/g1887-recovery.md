# Game 1887 (future wars - time travellers (1989)(u. s. gold)(de)[cr vf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01887). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802896 top-level bytes in 34 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 541 of 541 named (0 author symbols), 541 of 541 described (59 by rule, 349 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5286 of 5286 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18204 of 18204 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1556627 |
| code | 61492 |
| data | 145055 |
| padding | 7978 |
| unused | 31744 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:type | 2 | 117 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:setpatch | 24 | 833 | 45 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:endcli | 2 | 117 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:delphine | 3 | 16389 | 443 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:add21k | 3 | 480 | 33 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:bootblock | 1 | 126 | 6 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 2 of 2).adf:bootblock | 1 | 126 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |

## Cases handled

- 6 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- future wars - time travellers (1989)(u. s. gold)(de)[cr vf](disk 1 of 2)/Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf: blocks 931-934 (bytes 476672-478720) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.48)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_then_call`: If its output pointer is non-null, copies sixteen 32-bit words from a fixed offset in a base structure into that output, then calls through a function pointer held in a register.

### Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:setpatch

- `open_libraries_and_parse_args`: Program entry for a command-line utility: it allocates a task record, opens the required libraries and a file, then splits the argument text into strings, honouring double quotes and star escapes for newline and escape characters.
- `release_patch_resources_and_reply`: Cleanup routine that reads the saved record from the current task, closes any file and libraries it holds, frees its memory, and replies to the pending message before returning a stored status value.
- `dispatch_when_flag_set`: Calls a dispatch stub, then if a global flag is set it calls a routine with the argument and passes the result to a second routine.
- `install_system_patches`: Main body of the SetPatch utility: it checks the argument count and the first argument, then installs the named patches for alerts, memory-entry allocation, layers, floating-point exceptions and RAD memory.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_handler_through_pointer`: Calls a function pointer read from a fixed offset in a data pointer, then returns the D1 register value unchanged.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `allocate_patch_memory_block`: Checks the system version and that total memory exceeds 512 KB, then allocates a memory block of computed size, trying an absolute allocation and falling back to a general allocation while multitasking is disabled.
- `compute_checksum_then_jump`: Sums 23 words starting at a structure offset into a 16-bit total, stores its complement in a fixed field, then calls the routine at a pointer.
- `patch_disk_resource_functions`: Opens the disk resource and, on an OS version at or above the threshold, replaces two of its functions through the set-function call while saving the originals in globals and setting an interrupt-enable value.
- `save_state_when_call_succeeds`: Calls a saved function pointer and, if it returns nonzero, copies a 32-bit value from a fixed memory location into a global variable.
- `call_conditional_handler`: Calls a saved handler through a pointer only if a specific register holds -1 or matches a value from a memory location.
- `call_handler_if_minus_one`: Calls a saved handler through a pointer only when the D1 register holds -1; otherwise it returns.
- `write_low_memory_tag_once`: Sets a global to a fixed value, and if a four-letter tag is not yet stored at the bottom of memory, writes the tag with a saved value and a flag into low memory and conditionally calls a handler.
- `call_handler_through_pointer_2`: Calls a function pointer read from a fixed offset in a global table, with no arguments, and returns.
- `install_fpu_exception_vectors`: Stores the address of a small entry stub in each of sixteen consecutive low-memory exception vector slots for the floating-point coprocessor, stepping through the entries two bytes at a time, and records the address of the base entry.
- `chain_repeated_routine_calls`: Calls a routine in a long series of consecutive calls, with no arguments, and returns the value from the final call.
- `return_d0_value`: Returns the value already held in the D0 register without changing it.
- `return_d0_value_extended`: Performs a series of operations that ultimately returns the value held in the D0 register.
- `stub_return_input_value`: Returns the value already held in the D0 register without changing it; it has no other work.
- `set_function_if_old_exec`: On systems with an OS version at or below the threshold, calls the exec set-function routine through its library vector; otherwise it returns.
- `duplicate_memory_entry_list`: Allocates a new memory-entry block and copies the entry count and the address-and-size pairs from a source list into it.
- `repeat_library_call_then_flag`: Calls a library function once per count value, counting down to zero, then calls it once more and returns the word at the A2 address with its top bit set.
- `call_library_then_return_flag`: Calls a library function once, then returns the word at the A2 address with its top bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `test_and_run_patch_callbacks`: Passes a field of its argument structure to a test routine; if the test succeeds it calls two stored callback pointers, the second with the same field.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `find_next_branch_instruction`: Scans forward through 16-bit words until it reaches a branch-to-subroutine instruction and returns a pointer to that word.
- `resolve_branch_target`: Computes the address a short branch instruction leads to, using its 8-bit displacement or, when that is zero, the following 16-bit displacement, and returns it.
- `resolve_word_branch_target`: Returns the address a word-displacement branch leads to, computed from the sign-extended 16-bit displacement in the instruction's second word.
- `record_three_branch_addresses`: Walks forward through a code sequence using the branch-target helpers and stores the addresses it reaches in three globals.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_args_with_stack_pointer`: Forwards its two arguments to another routine together with a fixed stack-frame address, then returns.
- `format_message_and_output`: Formats a message into a 140-byte stack buffer with the exec formatting function, measures the text length, and passes the text and its length to an output routine when the text is not empty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:endcli

- `find_max_entry_setup_buffer`: Program entry that walks a linked list of entries to find the largest stored value, then, if allocation succeeds, fills a buffer with a sequence of words and calls routines to set up the result.
- `copy_memory_block`: Copies a block of sixteen bytes from a source address to a destination address.

### Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:delphine

- `clear_global_block_and_init`: Program entry that clears a large block of longs in the global data area, saves a register value into a global, and calls initialization through the library base.
- `mask_byte_on_condition`: Tests a signed-comparison condition from the processor flags.
- `set_global_status_bit`: Sets bit value 2 in a global status word and returns.
- `read_two_input_state_bits`: Derives a small code from two bits of a global input-state word: a clear bit 6 contributes 1 and a clear bit 2 adds 2.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `store_constant_in_status`: Stores the constant 79 into a global status word and returns.
- `store_constant_in_status_2`: Stores the constant 32879 into a global status word and returns.
- `store_constant_in_status_3`: Stores the constant 8 into a global status word and returns.
- `advance_random_state_modulo`: Advances a 32-bit shift-register random state, applying a fixed feedback mask when the shift overflows, then returns the state reduced modulo the argument.
- `read_serial_byte_and_handshake`: If a global status bit is set, returns a byte taken from the global serial-state word and rotated by one bit, waits briefly, and clears a handshake bit.
- `map_key_code_to_value`: Reads a code through an input routine and, when it is nonzero and below the high range, searches a 49-entry table for a matching code and returns the value stored alongside it.
- `copy_template_write_address_pairs`: Copies 180 words from a template table into the buffer at a stored pointer, then writes eight address pairs into it, each a masked high part and a low 16-bit part, derived from the arguments and fixed offsets.
- `write_constant_words_to_structure`: Writes a fixed set of sixteen-bit constants into fixed offsets of a structure, with no computation.
- `store_argument_and_reset_fields`: Stores a 32-bit argument at a fixed offset of a structure, zeroes a 16-bit field, and sets two 16-bit control fields to constants before returning.
- `return_global_long_value`: Returns the 32-bit value stored in a fixed global variable.
- `write_split_pairs_and_wait`: Stores the first argument in a global when it is not -1.
- `copy_sixteen_words_interleaved`: Copies sixteen 16-bit words from an input array into a structure, placing them in every second 16-bit slot starting at a fixed offset.
- `set_blitter_registers_and_wait`: Stores eleven arguments into fixed fields of a blitter parameter structure, then calls a routine that waits for the blitter to finish.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_from_subroutine`: performs a standard return from a subroutine.
- `call_routine_with_sentinel_args`: Calls a routine with a computed address and two arguments of minus one, then calls another routine and returns.
- `query_file_info`: Uses system functions to check a file and retrieves a value from a specific offset within the file information structure.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `retrieve_file_attribute`: Checks a file status and returns a short integer retrieved from a pointer within the file information structure.
- `call_helper_routines_sequence`: Calls four helper routines in sequence, including a tiny routine that negates a register.
- `tiny_neg_move`: Small routine of 2 instructions that negates d6, then copies d0 to d0.
- `call_single_helper_routine`: Calls one helper routine in sequence.
- `load_script_and_text_files`: Refers to the file names AUTO00.PRC and texte.dat and loops over a table of records, calling a raster-wait routine as part of each iteration.
- `return_from_subroutine_2`: Performs a subroutine return to the caller.
- `swap_buffers_and_set_control`: Swaps two global buffer pointers, waits for a display raster line, then writes one of two fixed control words into a structure depending on whether its argument is zero.
- `wait_raster_then_set_control`: Waits for a display raster line, then writes a fixed control word into a structure.
- `release_all_table_entries`: Loops over 255 records of a 26-byte table; for each record holding a nonzero handle it calls a routine with the handle and the product of two stored 16-bit values, then clears the record.
- `allocate_table_entry`: Finds the first empty record in a 255-entry table of 26-byte records; if none is free it returns -1.
- `free_table_entry_by_index`: Frees the record at a given index in a 26-byte table: when the record holds a handle it calls a routine with that handle and the product of two stored sizes, then clears the record fields.
- `repeat_routine_count_times`: Calls one routine once per iteration for as many iterations as its 16-bit argument, then returns.
- `reset_two_list_heads`: Sets two 32-bit global list heads to zero.
- `free_all_list_nodes`: Walks a singly linked list from a global head, calling a routine with each node and the size 20, then calls one more routine and returns.
- `insert_node_sorted_by_key`: Allocates a 20-byte chip-memory node and inserts it into a linked list in order of a 16-bit key looked up from a table for each node.
- `remove_node_matching_pair`: Searches a linked list for a node whose two 16-bit fields match the two halves of its argument, unlinks it, frees it with size 20, and returns zero.
- `clear_two_longs`: The routine stores zero into two consecutive longwords in memory.
- `walk_list_and_clear_globals`: The routine walks a linked list whose head is stored in a global variable.
- `append_node_to_list_tail`: Allocates a 20-byte chip-memory node, stores two 16-bit values from its arguments in it, and appends it to the end of a list headed by a global.
- `allocate_and_link_node`: Allocates a 20-byte chip-memory node, copies two 16-bit fields from the caller's area into it, and links it into a list through the head pointer pair.
- `reset_list_head_and_marker`: Sets a global list head to zero and a 16-bit global marker to minus one.
- `free_all_list_nodes_2`: Walks a singly linked list from a global head, calling a routine with each node and the size 214, then calls one more routine and returns.
- `allocate_record_from_template`: Allocates a 214-byte record and copies fifty 16-bit words from a template table, selected by its argument, into it.
- `mark_matching_record_inactive`: Searches a list of 214-byte records for one whose 16-bit identifier equals its argument.
- `remove_inactive_records`: Walks the 214-byte record list, unlinks every record whose identifier is minus one, and frees each unlinked record.
- `clear_long_and_word`: The routine writes zero to a longword and a word with all bits set to another global variable.
- `walk_list_and_clear_two_globals`: The routine walks a linked list whose head is stored in a global variable.
- `allocate_zeroed_record_list`: Allocates a 214-byte chip-memory record, zeroes its sixteen 16-bit words, and links it into a list headed by a global.
- `search_and_modify_list`: The routine traverses a linked list to find an entry matching a specific value, then modifies a word within that entry if a match is found.
- `self_referential_table_chase`: The routine walks a singly linked list whose head pointer is taken from a global variable offset.
- `dispatch_table_entry_calls`: Reads several fields from a table entry selected by an index, then calls a routine with those values, choosing between two call forms by a field.
- `call_with_frame_arguments`: Calls a routine with arguments taken from a table entry and stack-frame fields, then calls another routine with zero and returns.
- `store_position_and_derived_offsets`: Stores the halves of its two 32-bit arguments into global position fields, then stores two derived offsets: eight times the low word of the first argument plus 160 times its high word, and 160 minus eight times the low word of the second argument.
- `call_routine_with_stored_pair`: Calls a routine with two 32-bit values held in globals, then returns.
- `call_pair_by_stored_state`: If three stored 16-bit values equal zero, 160 and 200, it calls a routine with two stored values; otherwise it calls the routine with a different pair of stored values.
- `call_two_entries_with_flags`: Calls one routine twice, each time with a stored 32-bit value, a shared stored 16-bit value and a 16-bit value taken from the caller's frame.
- `return_from_subroutine_3`: performs a return from a subroutine.
- `set_table_field_by_selector`: Selects one of several 16-bit fields in a 32-byte-per-entry table by a selector argument and stores the value there.
- `store_table_entry_word`: Stores a 16-bit value from the caller's frame into one field of a 32-byte table entry, with the entry chosen by another frame value.
- `store_table_entry_word_2`: Stores a 16-bit value from the caller's frame into one field of a 32-byte table entry, with the entry chosen by another frame value at a different field offset.
- `store_entry_and_notify`: Stores four 16-bit values from its arguments into fields of a 32-byte table entry, then calls a routine and, if it returns zero, calls another routine with zero.
- `test_word_aligned_bitmap_bit`: Computes the address of a 16-bit word in a bitmap from a column and row argument, then tests one bit in it and returns whether that bit is set.
- `test_byte_aligned_bitmap_bit`: Computes the address of a byte in a bitmap from a column and row argument, then tests one bit in it and returns whether that bit is set.
- `run_setup_then_print_ok`: Runs initialization calls, busy-waits on the display beam, and polls a routine until it reports done.
- `process_entries_with_helper_calls`: Makes a long sequence of calls to thirteen helper routines and uses a local buffer of 80 bytes.
- `setup_local_buffers_if_clear`: When a stored 16-bit flag is zero, calls routines with pointers to several local buffers of 80 and 20 bytes and a stack-computed offset.
- `find_table_entry_index`: Searches a 12-byte-per-entry table for an entry whose stored fields match its arguments, with a selector value adding a further match test.
- `find_active_list_entry`: Walks a linked list of entries and returns -1 when it runs out.
- `draw_list_entries_after_raster`: Waits for a raster line, makes a set of calls with constants including 32000, 200 and 65535, then walks a linked list and calls routines for each entry.
- `process_list_region_with_count`: Walks a linked list of entries, computing a size from two 16-bit arguments multiplied by eight, and calls four routines to process matching entries.
- `process_list_entries_by_region`: Walks a linked list of entries using six arguments, computing a size from two 16-bit arguments multiplied by eight, and calls four routines for matching entries.
- `release_each_list_record`: Walks a linked list of records from a global head, calling a routine with each record, then returns.
- `release_each_list_entry`: Walks a second linked list of entries from a global head, calling a routine with each entry, then returns.
- `process_entry_when_slot_set`: When an entry's 16-bit slot field is not minus one, processes the entry using a large stack buffer and calls a routine.
- `read_script_bytes_into_fields`: Reads consecutive bytes from a script buffer at the current position into 16-bit fields, then calls two routines.
- `execute_script_bytecode`: Iterates through a sequence of bytecode instructions, parsing operands from the data stream to update table fields or trigger specific subroutines.
- `read_script_words_and_call`: Reads bytes from a script buffer into 16-bit fields, forming some values from two bytes as high byte times 256 plus low byte, then calls two routines.
- `execute_script_command_sequence`: Iterates through a byte stream to parse command opcodes and their associated 16-bit operands.
- `decode_script_opcode_branch`: Reads bytes from a script buffer, branching on whether a byte is zero, stores values into fields, and calls three routines accordingly.
- `decode_script_opcode_branch_2`: Reads bytes from a script buffer, branching on whether a byte is zero, stores values into fields, and calls one routine accordingly.
- `execute_script_opcode`: Parses a sequence of bytes from a script buffer to extract operands and executes specific subroutines based on the opcode value.
- `execute_script_command`: Parses a sequence of script bytes to extract operands and executes specific subroutines based on a command opcode.
- `parse_script_bytecode`: Iterates through a byte stream to extract multi-byte values and command opcodes.
- `store_script_word_in_table`: Forms a 16-bit value from two script bytes as high byte times 256 plus low byte, stores it into a table at an offset derived from a stored value, and calls one routine.
- `decode_script_opcode_branch_3`: Reads bytes from a script buffer, branching on whether a byte is zero, stores values into fields, and calls two routines accordingly.
- `parse_script_commands`: Reads a sequence of command opcodes and operands from a data stream to update game state or object properties.
- `execute_script_bytecode_5`: Parses a stream of bytecode to perform a sequence of operations.
- `read_script_byte_and_call`: Reads one script byte into a field, calls a routine with zero, advances the script position, and continues decoding bytes in a loop.
- `read_script_byte_and_call_2`: Reads one script byte into a field, calls a routine with the value five, advances the script position, and continues decoding bytes in a loop.
- `store_script_byte_global`: Reads one script byte into a 16-bit global variable, advances the script position, and continues decoding bytes in a loop.
- `execute_script_commands`: Iterates through a byte stream to parse command opcodes and their associated 16-bit operands.
- `read_script_byte_three_calls`: Reads one script byte into a field, makes three calls to routines, advances the script position, and continues decoding bytes in a loop.
- `read_script_byte_and_call_3`: Reads one script byte into a field, calls a routine with the value one, advances the script position, and continues decoding.
- `store_script_position_in_table`: Reads a script byte as an index, stores the following script position in the table slot chosen by that index, and advances.
- `clear_script_table_slot`: Reads a script byte as a table index, stores minus one in that slot, and advances the script position.
- `jump_to_table_script_position`: Reads a script byte as a table index and continues decoding from the script position stored in that slot.
- `branch_on_object_flags`: Reads a script byte and skips or jumps depending on flag bits in an object's status byte, calling one routine when the test passes.
- `branch_on_object_flags_2`: Reads a script byte and either advances or jumps depending on flag bits in an object's status byte, calling one routine.
- `parse_script_commands_2`: Iterates through a byte stream to interpret command opcodes and their associated operands.
- `execute_script_bytecode_2`: Parses a stream of bytecode instructions by reading an opcode and subsequent operand bytes.
- `parse_script_bytecode_2`: Iterates through a byte stream to interpret command opcodes and their associated operands.
- `store_script_pair_in_slot`: Reads two script bytes, uses them to address an entry of a structure, stores a value there, and calls one routine.
- `call_routine_with_script_byte`: Reads two script bytes, calls a routine with the second byte value, advances the script position, and continues decoding.
- `read_script_byte_two_calls`: Reads one script byte, calls two routines without arguments, advances the script position, and continues decoding.
- `execute_script_bytecode_3`: Iterates through a buffer of script bytes, decoding an opcode and its associated operands to perform various actions.
- `read_script_string_and_call`: Calls a routine, copies a zero-terminated string from the script into a local buffer, and then calls further routines with it.
- `load_animation_named_in_script`: Copies a zero-terminated name from the script into a local buffer and calls routines to load it as an animation.
- `read_byte_and_script_string`: Reads a script byte into a field, then copies a zero-terminated string from the script into a local buffer and calls two routines.
- `load_background_named_in_script`: Copies a zero-terminated name from the script into a local buffer and calls routines to load it as a background.
- `parse_script_command_stream`: Iterates through a data buffer to extract string identifiers and command opcodes.
- `load_colour_table_from_script`: Copies a name from the script into a local buffer and calls routines to load it as a colour table.
- `dispatch_script_opcode_switch`: Makes three initial calls, then reads script bytes and dispatches through a switch on the byte value.
- `call_then_dispatch_opcode`: Calls two routines and then continues into the script opcode switch shared by the decoder.
- `set_flag_then_dispatch_opcode`: Sets a 16-bit field to one and continues into the shared script opcode switch.
- `store_position_then_dispatch_opcode`: Stores the current script position in an object field, sets a flag field to one, and continues into the shared script opcode switch.
- `set_flag_then_dispatch_opcode_2`: Sets a different 16-bit field to one and continues into the shared script opcode switch.
- `clear_flag_then_dispatch_opcode`: Clears a 16-bit field to zero and continues into the shared script opcode switch.
- `branch_call_then_dispatch_opcode`: Calls one of two routines depending on whether a field is zero, sets a flag field to one, and continues into the shared script opcode switch.
- `execute_script_bytecode_4`: Iterates through a sequence of bytes, interpreting each byte as an opcode to perform specific actions.
- `read_script_operands_and_call`: Reads three script bytes into 16-bit fields, forming some values from pairs of bytes, and calls two routines.
- `index_script_opcode_positions`: Sets fifty 16-bit table slots to minus one, then scans a byte buffer of script opcodes, advancing by a length that depends on each opcode.
- `read_long_operand_and_call`: Reads four script bytes into 16-bit fields, forming some values from pairs of bytes, and calls one routine.
- `read_three_bytes_then_dispatch`: Reads three script bytes into 16-bit fields, advances the position by three, and continues into the shared script opcode switch.
- `parse_script_operands`: Iterates through a buffer of script opcodes to extract multi-byte operands into a destination table.
- `read_script_words_and_call_2`: Reads several script bytes, forming 16-bit values from pairs of bytes, and calls one routine.
- `read_conditional_script_operands`: Reads two script bytes into fields, and when the second is zero reads further operand bytes before calling a routine.
- `read_byte_then_dispatch_opcode`: Reads one script byte, advances the position, and continues into the shared script opcode switch.
- `store_script_position_in_table_2`: Reads one script byte as an index and stores the following script position in a table at a stored pointer, then advances.
- `parse_script_opcodes_2`: Iterates through a buffer of script opcodes and extracts their associated operands into a structured table.
- `read_two_bytes_then_dispatch`: Reads two script bytes into fields, advances by two, and continues into the shared script opcode switch.
- `parse_script_operands_2`: Iterates through a buffer of script opcodes and extracts associated operands into a structured table.
- `read_words_then_dispatch`: Reads three script bytes, forming a 16-bit value from pairs of them, advances the position, and continues into the shared script opcode switch.
- `parse_script_bytecode_3`: Iterates through a buffer of opcodes and extracts multi-byte operands into a structured table.
- `parse_script_operands_3`: Iterates through a buffer of opcodes and extracts multi-byte operands into a temporary table.
- `parse_script_operands_4`: Iterates through a buffer of script opcodes to extract multi-byte operands into a structured table.
- `parse_script_operands_5`: Iterates through a buffer of opcodes and extracts multi-byte operands into a structured table.
- `parse_script_opcodes`: Iterates through a buffer of script opcodes and extracts associated operands into a table.
- `compare_word_fields`: The routine compares the low and high bytes of a 32-bit parameter and builds a byte result with bits set for equality and for less-than / greater-than relations between those bytes.
- `update_object_list_attributes`: Iterates through a list of objects to update their properties based on bitwise masks and shifts.
- `return_parameter_unchanged`: The routine receives a single value in a register and returns it unchanged.
- `update_bitmask_buffer`: Performs bitwise operations on a series of 32-bit words within a memory region, applying a bitwise NOT and a circular bit shift.
- `return_passed_register`: The routine receives a value in a register and returns it unchanged.
- `apply_bitmask_to_buffer`: Performs bitwise AND operations on a series of memory buffers using a calculated bitmask derived from a source pointer.
- `return_parameter_unchanged_2`: The routine receives a single value in a register and returns it unchanged.
- `configure_blit_single`: The routine calls configure_blitter_parameters once with fixed arguments derived from its parameters.
- `configure_blit_quad`: The routine calls configure_blitter_parameters four times with the same base parameters and offsets added to the second argument.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `setup_bitplane_blits`: The routine obtains a global pointer, derives four flag words from a parameter, and calls setup_bitplane_blit four times with offsets added to the base pointer.
- `configure_blitter_76`: The routine computes a width and height from two 32-bit parameters, obtains a base offset via a global pointer helper, and builds four blitter parameter blocks with different source offsets.
- `configure_blit_region`: The routine computes a size and a base address from its parameters using a global variable and then calls configure_blitter_parameters once with derived values.
- `configure_blitter`: This routine consists of 76 instructions and calls a routine that waits for the blitter to finish.
- `parse_and_render_text_string`: Parses a space-delimited string to calculate text dimensions and render characters using blitter operations.
- `calculate_max_sum`: The routine iterates through a sequence of bytes until a zero byte is encountered, summing values retrieved from a table based on non-delimiter bytes and returning the largest sum found.
- `mask_table_words`: The routine iterates eight times over two pointers derived from the parameters, reads 16-bit words from one pointer, rotates them according to a nibble from the third parameter, combines them with the inverse and stores the result masked with a 32-bit word from the other pointer.
- `unknown_routine`: An 81-instruction routine for which no functional details are available.
- `prepend_linked_list_node`: Traverses a linked list to find the end and inserts a new node at the tail.
- `swap_linked_list_node`: The routine obtains a new node via another routine and stores it through a pointer derived from a stack variable.
- `insert_node_at_list_tail`: The routine walks a linked list starting from a global variable until it reaches the last node, obtains a new node via another routine, links the new node after the last node, copies a word from the parameter into the new node and sets another word field to a constant, then updates a global variable pointer with the new node address.
- `process_input_string`: The routine calls several helper routines with values derived from parameters and a global base, then iterates over a byte string pointed to by a parameter, looks up each byte in a global table and conditionally calls a buffer rotation routine with computed addresses and constants, and finally makes two more helper calls before returning.
- `render_text_line`: Computes a line width from a string length, centers the line vertically, and then walks the characters of a text string, skipping leading spaces, and for each printable character looks up a glyph table entry via a global table and issues two blit operations for the glyph using global pointers and a base offset.
- `render_formatted_text_string`: Parses a custom encoded string to calculate text dimensions and then performs blitter operations to draw characters.
- `render_text_glyph_buffer`: Iterates through a character buffer to draw text by looking up glyph dimensions in a table and performing blitter operations to copy pixel data to a screen area.
- `render_text_block`: Prepares blitter parameters for a block of text and issues a series of blitter configuration calls that set up copies and masks for each character in the block.
- `build_string_index_table`: The routine clears a fixed-size area in a global structure, scans a table of strings for entries with a specific marker, copies matching strings into the cleared area using copy_string_to_buffer_2, and stores indices into another global area.
- `lookup_string_index`: Calls another routine to retrieve a value and then performs a calculation to return a combined 32-bit result based on a table lookup.
- `process_conditional_loops`: Executes a routine and then enters two separate loops that continue based on the state of a value returned by another routine.
- `scale_and_clamp_fields`: The routine doubles several fields of the two 32-bit parameters and then computes three derived values from the first parameter using nibble extraction and clamping logic with the second parameter, updating intermediate variables accordingly.
- `process_word_table_loop`: The routine loops sixteen times, each iteration processing sixteen words from a global area through scale_and_clamp_fields, then calls copy_word_table and two empty helpers, and finally calls two routines with values from a global structure.
- `process_word_table_reverse`: The routine loops sixteen times in reverse order, each iteration processing sixteen words from a global area through scale_and_clamp_fields with varying arguments, then calls copy_word_table and two helpers.
- `fill_word_table_range`: The routine iterates over a range defined by its first parameter and writes the result of scale_and_clamp_fields into a global word table at computed offsets.
- `shift_word_table_elements`: Shifts a range of words within a table either upwards or downwards, or copies a range from one table to another, depending on a mode parameter.
- `shift_word_table_down`: The routine copies a variable number of words from a global area to another, shifting them downwards based on a value from a second global structure.
- `shift_word_table`: Depending on a flag, this routine either shifts a range of words upwards or copies a range of words from one table to another.
- `shift_word_table_up`: The routine copies a variable number of words from a global area to another, shifting them upwards based on a value from a second global structure.
- `copy_word_table_range_2`: The routine copies a range of words from one global area to another using a loop bound by values from a second global structure.
- `copy_word_table_range`: The routine copies a variable number of words from one global area to another using a loop bound by a value from a second global structure, then calls copy_word_table.
- `reset_disk_prompt_state`: The routine clears a 255-entry table of 30-byte records by zeroing selected bytes and words, resets two short counters, and then conditionally invokes other routines based on a global flag.
- `clear_struct_array_entries`: The routine clears several fields in each of 255 entries of a table referenced through a global variable and then, if a global pointer is non-negative, calls another routine twice with arguments derived from that pointer and a global variable and finally writes a constant to the global pointer.
- `copy_words_to_four_pointers`: The routine copies a block of words from a source pointer to four destination pointers that are spaced apart in the destination buffer.
- `process_string_data`: The routine calculates the length of a string, converts its characters to uppercase, and copies it to a new location.
- `apply_table_offsets_to_buffer`: This routine calls another routine twice to set up a table lookup.
- `copy_bytes_and_advance_source`: The routine copies a specified number of bytes from a source buffer whose address is stored in a global variable to a destination buffer, then updates the stored source pointer to the next byte after the copied data.
- `update_animation_frame_data`: Retrieves animation metadata from a table and updates a buffer of frame records.
- `expand_nibble_mask`: The routine returns zero when the input is zero, otherwise it expands each nibble of the input into a three-bit pattern and packs the results back into a 16-bit value.
- `render_background_layer_from_script`: Processes script-driven background loading by checking coordinate equality, performing blitter-based area fills, and managing buffer offsets.
- `parse_script_string_markers`: Scans a string for a period character and checks the subsequent character to determine if it is an 'N' or another value.
- `parse_string_and_update_offsets`: Scans a string for a period character and checks if the following character is an 'N' to determine how to update global offsets and values.
- `parse_string_marker`: Scans a string for a period character and checks the subsequent character for an 'N' to determine how to update global offsets and values.
- `parse_string_markers`: This routine checks a buffer for specific character sequences and, if found, copies data to different destinations and updates global values.
- `copy_and_update_buffer`: The routine copies two separate blocks of bytes to different destinations and updates a global variable with a calculated value.
- `return_zero`: This routine is empty and simply returns zero.
- `read_data_via_indirect_calls`: The routine obtains a handle via an indirect call, reads two short values from it using indirect calls, computes a size from them, performs an indirect read into a buffer using that size, and then performs a final indirect call on the handle.
- `decompress_bitstream_data`: Reads a compressed bitstream using a bit-by-bit extraction method to reconstruct bytes.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bits_across_words`: The routine shifts a 32-bit value right by one bit repeatedly for a given count, refilling from the next word in a pointer when the value becomes zero.
- `process_script_event_queue`: Iterates through a table of pending script commands, executing each function pointer and clearing the entry upon completion.
- `process_script_command_data`: Processes a command from a table by retrieving parameters and executing a sequence of operations including blitter transfers and sprite updates.
- `process_script_command_queue`: Iterates through a table of command structures to execute pending script instructions and update associated data buffers.
- `process_script_command_stack`: Iterates through a table of command pointers and arguments to execute a sequence of scripted instructions.
- `process_multiple_string_entries`: Checks four specific character locations relative to a base address and, if a character is non-zero, executes a sequence of several other routines with varying offsets and constants.
- `clear_game_state_tables`: Performs a series of clean-up operations on data referenced through a global base pointer.
- `load_data_structures`: Reads data into local buffers and uses the results to either loop through and copy data from a table or copy a block of words from a specific location.
- `read_and_update_offsets`: This routine performs a file read, updates offsets in a buffer using another routine, and closes a file.
- `clear_resource_tables_2`: Iterates through several global tables to check for non-zero handles, calls a routine to release them, and then zeros out the entries.
- `clear_resource_tables`: Iterates through multiple tables of varying sizes to check for non-zero handles, calls a routine to release them, and then zeros out the entries.
- `iterate_and_process_linked_list`: Iterates through a linked list structure multiple times, incrementing a counter and performing operations on elements until a limit is reached.
- `read_and_link_data`: Performs a read operation and then calls another routine to append a new node to a linked list.
- `process_buffer_loop`: Increments a counter while calling another routine using a pointer from a table, performs a read operation, and then enters a second loop to repeat the calls.
- `execute_nested_logic`: Iterates through a list of pointers to call another routine repeatedly, then performs a series of conditional calls to other routines based on the results of previous calls.
- `traverse_linked_list`: The routine traverses a linked list to find the end, inserts a new node, and performs several subsequent operations involving other routines and memory access.
- `traverse_linked_list_and_insert_node`: Traverses a linked list to find the terminal null pointer and inserts a new node at that position.
- `link_buffer_chain_with_data`: Appends a new node to a linked list and populates various offsets within that node using data from a provided parameter and a table lookup.
- `read_and_link_buffer_chain`: The routine reads data via a wrapper, links pointers in a buffer chain, performs several reads into offsets of the buffer, and stores a pointer from a global table indexed by a short into the buffer.
- `update_linked_list`: Traverses a linked list and updates values at a specific offset within each node using data from a table.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `link_buffer_chain_alt`: Appends a new node to a linked list and populates various offsets within that node using data from a provided parameter and a different table lookup.
- `read_and_link_buffer_chain_alt`: The routine reads data via a wrapper, links pointers in a buffer chain, performs several reads into offsets of the buffer, and stores a pointer from a different global table indexed by a short into the buffer.
- `traverse_linked_list_2`: Iterates through a linked list of pointers and updates a value at a fixed offset within each node using data from another memory location.
- `return_from_subroutine_4`: This routine is empty and performs no operations before returning.
- `append_buffer_to_list`: Traverses a linked list to find the last element and inserts a newly read buffer into the chain.
- `read_and_link_buffer_chain_small`: The routine reads data via a wrapper, links pointers in a buffer chain, performs a read into the buffer, and links the buffer into a global list using a global variable as a base.
- `insert_buffer_into_list`: Finds the end of a linked list by following pointers and then links a newly allocated buffer into the list.
- `read_and_link_buffer_chain_small_alt`: The routine reads data via a wrapper, links pointers in a buffer chain, performs a read into the buffer, and links the buffer into a global list using a different global base.
- `update_entity_state_and_blitter_buffer_2`: Iterates through an entity table to check for active or valid entries, updating their internal properties and state values.
- `update_entity_state_and_blitter_buffer`: Iterates through an entity table to compare state values and update internal properties.
- `process_entity_state_updates`: Iterates through a structured entity table to match specific properties and update their internal state values.
- `process_data_table_entries`: Iterates through a table of structures to compare and update values based on specific conditions.
- `update_entity_state_table`: Iterates through a structured table to compare entity properties against a set of conditions.
- `process_entity_table`: Iterates through a table of records to update entity states and properties.
- `process_linked_list_object_states`: Recursively traverses a linked list of data structures to synchronize their properties with values from a global lookup table.
- `sync_object_state_from_list`: Iterates through a linked list of data structures to synchronize their internal properties with values stored in a global lookup table.
- `traverse_linked_list_nodes`: Recursively traverses a linked list of data structures by following pointers stored within each node.
- `nop_placeholder`: The routine is empty and performs no operations.
- `dispatch_script_command`: Executes a sequence of commands by recursively calling itself with various parameters and constants.
- `loop_other_routine`: Repeatedly calls another routine until a stack word becomes zero, then enters an infinite loop calling that routine twice.
- `execute_blitter_sequence`: Performs a series of blitter operations by waiting for specific raster lines and configuring blitter parameters using values from a global structure.
- `render_formatted_text_string_2`: Calculates the maximum word length in a string delimited by a specific character and determines text centering parameters.
- `parse_text_string`: Iterates through a null-terminated string to calculate word lengths and handle whitespace.
- `parse_sprite_command_stream_3`: Iterates through a byte stream to interpret sprite control commands, updating sprite positions and performing tile index lookups from a table.
- `parse_sprite_command_stream_2`: Iterates through a byte stream to interpret sprite control commands, updating sprite positions and performing tile index lookups from a table.
- `process_sprite_command_stream`: Iterates through a byte stream to interpret sprite control commands, updating sprite positions and performing tile index lookups from a table.
- `parse_sprite_command_stream`: Iterates through a byte stream to interpret sprite control commands, including position updates and tile index lookups.
- `process_tiled_sprite_hierarchy`: Recursively traverses a data table to fetch sprite or tile indices and performs drawing operations.
- `render_tiled_sprite_layers`: Iterates through a data table to fetch sprite or tile indices and performs recursive drawing operations.
- `process_text_input_buffer`: Reads keyboard input and updates a character buffer while managing a cursor position.
- `parse_and_render_text_string_2`: Parses a string of characters to determine their visual representation and updates a buffer.
- `process_sprite_data_table`: Iterates through a data table to update byte values and perform blitter operations based on character or sprite properties.
- `call_library_function_2`: Loads a library base from a global variable and calls a function at a specific offset from that base.
- `update_sprite_offsets`: The routine updates two short offsets and copies a block of words from a table indexed by a parameter into a buffer pointed to by a global variable, then clears two words in that buffer and updates a short flag.
- `initialize_display_structure`: Copies values from a global area into a structure pointed to by a global variable, writes constant words into the structure, fills a repeated pair of words from a global table, and clears two words and a global variable.
- `configure_interrupts_and_globals`: Stores a frame pointer into a global variable, writes several constants to global variables, and returns the concatenation of two input registers while modifying the INTENA register.
- `set_global_constants`: Stores a constant value into a global variable and writes to DMACON and INTENA hardware registers.
- `check_global_flag`: Checks a bit in a global variable and performs a loop based on a local value before returning.
- `check_interrupt_flag`: The routine checks a bit in a global variable and, if set, performs an indirect call and a loop.
- `update_mouse_position`: Reads the mouse port hardware register and updates several offsets stored via a frame pointer.
- `setup_bitplane_blit`: Computes blitter parameters from two points and writes them to multiple custom chip registers.
- `copy_cstring`: Copies a null-terminated byte string from a source pointer to a destination pointer, byte by byte, and terminates the destination with a zero byte.
- `search_string_table`: The routine walks a pointer table obtained from a global variable, copies a candidate string via a copy routine and compares it with the input string via a compare routine, setting a flag when a match is found and returning a combined result.
- `find_trackdisk_device`: Calls the exec library to locate the trackdisk.device and extracts a value from a device structure, returning it combined with an input register.
- `execute_indirect_call_8`: The routine executes a function pointer retrieved from a structure and returns a status value based on the result.
- `process_nested_data_structure`: Recursively traverses a data structure while updating a counter and performing bitwise operations based on flag bits.
- `process_nested_data_2`: Recursively processes a data structure, updating a counter and performing bitwise shifts and conditional assignments based on flag bits.
- `process_nested_data`: Recursively extracts values from a data structure and updates a memory location while performing bitwise shifts and conditional checks on a counter.
- `parse_nested_data`: Recursively extracts values from a data structure and updates a memory location.
- `parse_nested_data_structure`: Recursively extracts values from a data structure and updates a memory location.
- `process_nested_values_3`: Recursively processes values by extracting high words and updating a memory location based on a bitwise check of the result.
- `process_nested_values_2`: Extracts a high word from a value, stores it in a local variable, and enters a loop that calls another routine to update a value.
- `process_nested_values`: The routine extracts a high word from a value, stores it in a local variable, and enters a loop that calls another routine to update a value and check a bitmask.
- `manage_dos_library_resources`: The routine checks global flags, conditionally calls helpers to examine and lock a file, wait for a flag, shut down audio and free memory, and performs indirect dos.library calls for opening and closing libraries.
- `examine_and_lock_file`: Uses dos library calls to examine a file, lock it, read an attribute from the file info, unlock it, and free allocated memory.
- `set_flag`: Stores the constant value one into a global variable and returns.
- `wait_flag`: Sets a global flag to one if it is zero and then busy-waits until the flag reaches a specific value.
- `initialize_cia_resource`: The routine opens the ciaa.resource via exec.OpenResource, initializes several global variables, writes a series of values to a pointer table, performs indirect calls through a library base, and calls routines that compute a max indexed value, write CIA timer registers, clear pointer tables, reset audio state and initialize audio DMA channels.
- `shutdown_audio`: The routine checks a global flag and, if set, performs an indirect call through a library base, calls a routine that initializes audio DMA channels, and clears the flag.
- `conditional_free_memory`: If two global variables are non-zero, it calls the exec library to free memory and clears one of the globals.
- `compute_max_indexed_value`: The routine scans a block of bytes starting from a table located via a global variable and a fixed offset, keeping the largest byte value seen.
- `write_cia_timer_registers`: The routine writes three values to CIAA control and timer registers.
- `clear_pointer_table_entries`: The routine iterates a fixed number of times over an array of pointers referenced through a global variable and writes zero to the memory location each pointer refers to.
- `reset_audio_and_custom_state`: The routine writes zero to four audio volume registers and a custom register, and clears several global variables used for audio state.
- `initialize_audio_dma_channels`: The routine clears audio volume registers and a custom register, sets length registers to one, points four channel pointer registers to the same global variable, and writes a value to a global variable.
- `return_zero_constant`: The routine performs a single call to another routine and then returns zero.
- `manage_audio_state`: Increments a global counter and either loops to call a routine with decreasing values and incrementing hardware addresses or performs conditional recursive calls and memory management.
- `perform_recursive_call`: The routine performs a single recursive call to itself before returning.
- `write_audio_channel_pointers`: The routine loops over four iterations, calling another routine with a decreasing short and an address that steps by sixteen each time.
- `process_command_and_clamp`: Interprets a command byte to either perform arithmetic operations, modify a global variable, or execute an indirect function call.
- `clamp_value_to_range`: The routine adds a value to a short integer in a structure and clamps the result to a maximum value stored in that same structure.
- `execute_command_data`: Interprets a command byte to perform various actions including indirect function calls, modifying a global variable, or adjusting a value using a nibble from a pointer.
- `set_ciaa_bit`: Sets the second bit of a global variable using a bitwise OR operation.
- `clear_ciaa_bit`: Clears the second bit of a global variable using a bitwise AND operation.
- `find_word_in_table`: The routine searches for a word within a table, extracts a signed offset from a source, and stores a word from the calculated address into a destination buffer.
- `copy_word_with_signed_offset`: The routine computes a signed word offset from an input value, selects the sign based on a flag, and copies a word from a source address offset by that amount to a destination address derived from another register.
- `store_word_to_offset`: Stores a word from a register into a destination address offset by four bytes from a provided pointer.
- `find_word_in_table_2`: Searches for a word within a table, extracts a signed offset from a source, and stores a word from the calculated address into a destination.
- `execute_indirect_call_2`: Calculates an offset into a table of pointers and calls the function located at the resulting address.
- `copy_word_from_table`: The routine iterates through a table until a match is found and then copies a word from an offset of the matching entry to a destination.
- `copy_word_from_table_high`: Iterates through a table until a match is found and then copies a word from an offset determined by the high nibble of a byte to a destination.
- `copy_word_from_table_low`: Iterates through a table until a match is found and then copies a word from an offset determined by the low nibble of a byte to a destination.
- `copy_word_from_offset`: Copies a word from an offset of sixteen bytes from one pointer to an offset of six bytes from another pointer.
- `decrement_short_by_nibble_2`: Decrements a short value pointed to by a pointer using a nibble extracted from the fourth byte of that pointer, or adds that nibble if it is non-zero.
- `decrement_short_by_nibble`: Masks the fourth byte of a short pointer with 15 and, if non-zero, subtracts that nibble from the pointed short and stores the result to an offset in another argument.
- `configure_audio_dma`: The routine clears a global variable, calls another routine four times, writes a constant to a global variable, delays with a counter, then copies four words from a global structure to the custom chip registers AUD0LCH, AUD3LCH and DMACON. It also updates a counter and wraps it against a limit.
- `update_entity_state_from_table`: Fetches data from a lookup table based on a value extracted from a pointer, performs arithmetic adjustments including clamping and offset subtractions, and updates several memory locations and a global variable.
- `update_table_state`: Calculates a value based on table data and a constant, clamps it within a range, and updates several memory locations and a global variable.
- `normalize_and_copy_table_data`: Clamps a value derived from a table offset to a specific range and subtracts a constant before storing it.
- `copy_and_or_table_fields`: Copies several words from a table into a structure, updates a global variable with a value from the table, and performs a bitwise OR on a global variable.
- `store_fields_and_or_flag`: Copies several words from a table pointed to by an argument into a structure pointed to by another argument and ORs a word from the table into a global variable.
- `initialize_sound_state`: A short routine that calls other routines to zero out memory locations and decrement or clamp values.
- `initialize_sound_channel_state_2`: Zeroes a word at a calculated offset, zeroes a word at a pointer offset, and calls another routine twice with specific parameters.
- `decrement_and_initialize_sound`: Decrements a word at an offset in a pointer, zeroes a word at a calculated offset, clamps a word to a maximum value, and calls another routine with parameters from the pointer.
- `clear_and_initialize_sound_twice_2`: A seven instruction routine that calls another routine twice using parameters from a pointer and constants.
- `clear_and_initialize_sound_twice`: Clears a word at a calculated offset, zeroes another word, and calls another routine twice with parameters taken from the pointer and constant values.
- `open_cia_resource_and_init_audio`: Opens the ciaa resource and performs an indirect call through a library base at a specific offset.
- `conditional_library_reset_audio`: Checks a global word and, if non-zero, calls an indirect library function, calls another routine, and clears the global word.
- `update_cia_timer_registers`: Stores a constant into a global variable, copies a value from RAM into that global variable, and then writes that value to the CIAA control and timer registers.
- `clear_audio_pointers_and_counters`: Clears four audio channel length and volume registers and four pointer variables and zeroes four RAM counters.
- `reset_audio_and_enable_dma`: Clears audio channel registers and pointers, zeroes RAM counters, writes a value to a global variable and touches DMACON to enable DMA channels.
- `reset_audio_channels`: Calls a routine that touches the AUD0LCH hardware register and manipulates a global counter, then returns the value passed in D1.
- `update_audio_hardware_registers`: Increments a global counter and performs a loop to write values to audio hardware registers.
- `reset_audio_channels_and_dma`: Clears two global words, calls another routine three times, writes a value to a global variable, performs a delay loop, and iterates over audio channel registers to conditionally write pointers and values.
- `initialize_sound_channel_state`: Computes an offset from a parameter, zeroes a word in RAM, stores several words from parameters into RAM at that offset and writes a parameter to RAM.
- `clamp_and_copy_short_2`: Decrements a word at an offset if it is non-zero, otherwise copies a word, calculates a sum, clamps that sum between zero and sixty-three, and stores the result in two locations.
- `clamp_and_copy_short`: Copies a word within a structure, adds two words, clamps the sum to 0..63 and stores the clamped value back into the structure and another argument.
- `update_pointer_data`: If a pointer in a structure is non-zero, it copies several values from that structure to the location pointed to by the pointer and performs a bitwise OR on a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_update_scroll_state`: Copies a block of 32-bit words from a source pointer into a destination word buffer based on a flag, then updates several global variables that hold scroll-related state, including a base offset, a pointer to a word table, a pointer to a word buffer, and a signed index.
- `initialize_window_state`: The routine initializes fields in a structure pointed to by a global variable, stores a constant marker, and conditionally calls another routine with parameters derived from a global variable.
- `sync_sprite_data_structures`: Compares a string from a data structure against a global string and, if they match, traverses a linked list of pointers to populate a new table with the addresses of those elements.
- `parse_and_store_file_metadata`: Compares a string from a structure against a global buffer and, if they match, traverses a linked list to count elements.
- `empty_routine`: This routine is empty and performs no observable actions or data manipulations.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `copy_string_including_terminator`: Copies bytes from a source pointer to a destination pointer until a null byte is encountered, including the null byte in the destination.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `convert_to_uppercase`: The routine checks if an input byte represents a lowercase ASCII letter; if so, it subtracts a constant to convert it to uppercase, otherwise it returns the byte unchanged.
- `convert_to_lowercase`: The routine checks if an input byte represents an uppercase ASCII letter; if so, it adds a constant to convert it to lowercase, otherwise it returns the byte unchanged.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `initialize_window_state_2`: The routine clears a block of memory pointed to by a global variable, stores a constant marker in a global structure, copies a global pointer into the structure, and conditionally calls another routine based on a flag in a global variable.
- `return_void`: This routine performs no operations and immediately returns to its caller.
- `parse_command_string`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit_3`: Copies bytes from a source pointer to a destination pointer until a null terminator is reached or a maximum length is exceeded.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `copy_string_pad_zeros`: The routine copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `open_window_and_get_icon`: Performs a series of library calls to open a resource, retrieve a nested pointer, and extract values from a structure into a parameter list.
- `store_and_restore_pointer`: Stores a parameter through a pointer at a global location, calls another routine, and then clears that global location.
- `store_param_and_dispatch`: The routine stores the first parameter into a global variable, calls another routine with the routine itself and the second parameter, then clears a byte at the address stored in the global variable and returns the result from the call.
- `increment_and_store_byte`: Loads a pointer from a global variable, increments a counter stored in a global variable and stores the low byte of the parameter through that pointer.
- `parse_format_string_and_copy`: It parses a format string and copies a byte from a table into a destination pointed to by a global pointer.
- `call_and_return_concatenated_2`: A routine consisting of 48 instructions that calls a routine returning a concatenated value and a routine that recurses.
- `parse_format_string`: Parses a format string containing specifiers for integers, floating-point numbers, and strings.
- `perform_signed_division`: Reads two signed inputs, determines the sign of the result, calls another routine to perform division, and combines the result with the second input into a 32-bit value.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original contents of a register with the value left in that register by the call.
- `recursive_call`: Calls itself repeatedly without performing any other operation.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `call_other_routine`: This routine is 7 instructions long and is entered because its address is taken with pcrel-pea.
- `modify_linked_list_nodes`: This routine modifies nodes within a linked list by updating pointers and status flags.
- `return_to_caller`: This routine is empty and performs no operations before returning to its caller.
- `update_linked_list_nodes`: Modifies nodes within a linked list by updating pointers and status flags.
- `increment_and_store_byte_2`: Increments a pointer stored in a global variable and stores a byte from a table at the original pointer location.
- `walk_record_table_entries`: Iterates through a table of 22-byte records from a starting address up to a calculated end address, calling another routine on each entry.
- `clear_resource_structure`: Recursively traverses a structure based on bit flags to call other routines and then zeroes out the structure's fields.
- `clear_resource_entry`: The routine checks flags in a structure to conditionally call other routines for removing nodes, deleting files, or closing entries, then zeroes out the structure's fields.
- `manipulate_linked_list_nodes`: Updates, inserts, or removes nodes within a linked list structure by modifying pointers and status flags.
- `manage_linked_list_node`: Updates or removes nodes within a linked list structure by modifying pointers, clearing status bits, or copying byte values.
- `update_list_node_fields`: Modifies fields within a structure, including clearing specific bits and resetting pointers, or updates a node by copying a byte from a source and adjusting pointer offsets.
- `update_list_node_2`: Modifies a structure's fields and pointers based on conditional checks, potentially updating a list head and copying a byte from a source location.
- `update_list_node`: Updates a global variable using values from a table, modifies a byte in a structure, and returns a 16-bit value derived from a byte and a calculated sum.
- `search_and_clear_list_2`: Iterates through a list of structures and clears three consecutive longwords to zero if a specific byte is found to be null.
- `search_and_clear_list`: Iterates through a list of structures, clearing three consecutive longwords to zero if a specific byte is null, or returning zero if the end of the range is reached.
- `clear_three_longwords`: Sets three consecutive longwords at a location pointed to by a global variable to zero.
- `update_status_fields`: The routine modifies fields within a structure based on the return value of another routine, including setting short words and bitwise ORing bytes.
- `update_status_flags`: The routine writes a value to a structure offset, modifies a byte in a structure using a bitwise OR, and updates a longword from another location before conditionally setting another bit in a structure.
- `traverse_list_and_clear_2`: Walks a linked list using a pointer, calling another routine for each node, and finally sets the list head to zero.
- `traverse_list_and_clear`: Walks a linked list using a pointer in A2, repeatedly calling a library vector routine with the current node and the next node offset, until the list ends.
- `insert_into_linked_list`: Updates a pointer table and inserts a new node into a linked list by setting the next pointer of a new structure to a global address.
- `update_pointer_table`: Stores two longwords from a memory location into a table and updates a pointer with the new table address.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_linked_list_node`: The routine traverses a linked list to find a matching pointer, removes the node by updating the previous pointer to skip it, and then calls another routine.
- `traverse_and_store_pointer`: The routine traverses a linked list to find a matching pointer, stores a value into a global variable or a provided location, and then calls another routine.
- `store_pointer_and_call_library`: Stores a longword from a pointer into either a global variable or another location before performing an indirect call.
- `store_and_call_library`: Stores a longword from a pointer into a global variable and then calls another routine.
- `check_table_status`: The routine validates an index against a table boundary and checks a boolean status from a nested table structure.
- `check_interactive_status`: Retrieves a value from a table using an offset and calls a library function to check a boolean status.
- `return_zero_2`: This routine is empty and simply returns a value of zero to the caller.
- `delete_file_with_error_check`: Calls a library function to delete a file and, if it fails, calls another library function to retrieve an error code.
- `return_zero_lea`: Returns zero after performing a memory address calculation.
- `execute_conditional_logic`: A routine consisting of nineteen instructions that is called by several routines to perform conditional checks and data writing operations.
- `write_data_with_error_handling`: The routine attempts to write data and, if the operation fails, retrieves an error code and stores it in a global variable.
- `write_data_with_error_check`: Attempts to write data using a library call and, if the result is negative one, calls a library function to retrieve an error code.
- `return_from_subroutine_5`: This routine performs a single return instruction to exit the current execution flow.
- `check_conditional_flag`: The routine calls another routine and checks a specific bit in the result, then conditionally executes another routine if a global short value is non-zero.
- `evaluate_conditional_result`: Compares a global variable against a constant value and executes a conditional routine based on the outcome.
- `chain_result_to_output`: Checks a global variable against a constant, executes a conditional function using the result, and passes the returned value to a write wrapper followed by an output wrapper.
- `dispatch_on_flag_then_call`: Checks a global variable for a non-zero value and calls the function pointer stored there if it is set.
- `conditional_global_dispatch`: Checks several global variables for non-zero values and, when set, calls other routines with parameters taken from those globals or performs indirect calls through pointers derived from globals.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_entry`: Checks if an index is within the bounds of a table and if the entry is valid, then calls a library function to close the resource and clears the table entry.
- `call_dos_close`: Calls a library function via an offset from a base address stored in a global variable.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested offset within a structure pointed to by a value in a register and executes it.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested structure within a global data block and executes it.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_callback`: Retrieves a function pointer from a nested structure within a data table and executes it.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `execute_indirect_pointer`: Performs an indirect call using a function pointer retrieved from an offset within a structure.
- `execute_indirect_function_pointer_3`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `execute_library_offset_call`: Performs an indirect call using a function pointer retrieved from a library-related table offset.
- `call_library_function_offset_202`: Calls a function from a library using a pointer retrieved from a table at a specific offset.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_4`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_nested_pointer`: Performs an indirect call using a function pointer retrieved from a nested offset within a structure pointed to by a global variable.
- `execute_indirect_function_pointer_5`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_indirect_function_pointer_6`: Retrieves a function pointer from a nested structure within a memory block and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_7`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `execute_indirect_function_pointer_8`: Retrieves a function pointer from a nested structure within a data block and executes it.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:add21k

- `execute_initialization_sequence`: The routine performs several calls to other routines, updates global variables with return values, and modifies a byte in a nested structure before calling another routine with specific parameters.
- `set_viewport_and_refresh`: The routine stores two integer values obtained from another routine into a global variable area, then conditionally calls other routines based on those values.
- `clear_table_and_recurse`: The routine clears a block of forty-one long words starting at an offset from a base pointer taken from the stack, then stores a pointer to a small stack buffer and a global variable into two fields of the same structure and calls another routine.
- `call_indirect_if_flag_then_recurse`: Checks a global variable for a non-zero value and calls the function pointer stored there if it is set.
- `parse_dialogue_text`: Obtains two identifiers from a global table and stores them in global variables, then branches based on their values.
- `execute_callback_chain`: Executes a series of conditional function calls stored in various memory locations and performs a loop.
- `initialize_game_state`: The routine runs a short loop ten times calling another routine.
- `copy_null_terminated_string`: Finds the end of a destination buffer by searching for a null byte, then copies bytes from a source pointer into that buffer until a null byte is encountered or a limit is reached.
- `copy_string_with_limit_2`: Finds the end of a destination string, then copies bytes from a source pointer into that buffer until a null byte is found or a counter reaches zero, ensuring the result is null terminated.
- `copy_string_to_buffer_2`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `compute_string_length_2`: Calculates the number of bytes in a zero-terminated string by walking through it until the terminator is found.
- `copy_bytes_with_padding`: Copies bytes from a source pointer to a destination pointer for a limited count derived from a 32-bit parameter, stopping early if a zero byte is encountered, then writes zero bytes to the remainder of the destination area for the same count.
- `process_table_entry`: Validates an index against a range and checks a table entry, then either calls another routine recursively or clears the entry and returns a status.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_function`: Calls a function from a library using a pointer retrieved from a global variable.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_9`: Fetches a function pointer from a nested memory structure and executes it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_indirect_via_table`: The routine loads a function pointer from a global variable table and calls another routine through it.
- `execute_indirect_function_5`: Fetches a function pointer from a nested table structure using offsets from a base address and executes it.
- `execute_indirect_function`: Fetches a function pointer from a nested table structure using offsets from a base address and executes it.
- `execute_indirect_function_2`: Loads a function pointer from a nested offset within a structure and executes it.
- `execute_indirect_function_pointer_10`: Performs a double pointer dereference to call a function.
- `execute_indirect_function_3`: Fetches a function pointer from a nested table structure and executes it.
- `execute_indirect_function_4`: Fetches a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_indirect_call_3`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `execute_table_offset_call`: Calls a function located at a specific offset within a table accessed via a pointer.
- `execute_indirect_call_4`: Calls a function located at an offset within a table accessed via a pointer.
- `execute_indirect_call_5`: Retrieves a function address from a table and executes it.
- `execute_indirect_call_6`: Executes a routine by following a pointer through a multi-level table structure.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `execute_indirect_call_7`: Performs an indirect jump to a routine address found in a table.

### Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `initialize_graphics_context`: Checks for existing system resources and, if necessary, allocates memory and initializes a new graphics structure.
- `clear_copy_and_enable_dma`: Zeroes 8000 bytes of memory, copies a 51-byte block into another buffer and opens graphics.library.
- `call_init_then_library_jumps`: Calls one unnamed setup routine, then makes two library calls through the library base register at offsets of minus 240 and minus 60 bytes.
- `advance_past_string_terminator`: Moves a pointer forward through a zero-terminated string until it passes the terminating zero byte, leaving it just past the end.
- `initialize_sixteen_table_entries`: Fills a table of sixteen 8-byte entries.

### Future Wars - Time Travellers (1989)(U. S. Gold)(De)[cr VF](Disk 2 of 2).adf:bootblock

- `check_library_function`: The routine calls a function through a pointer and returns zero if the result is non-zero, otherwise it returns one.
- `initialize_graphics_environment`: Checks for existing system resources and, if necessary, closes libraries to prepare for a new graphics setup.
- `clear_copy_and_enable_dma_2`: Zeroes a block of memory, copies a fifty-one byte block from one buffer to another, and opens the graphics library.
- `execute_setup_and_library_calls`: Calls an unnamed routine and then performs two function calls through offsets in a library base register.
- `advance_past_string_terminator_2`: Increments a pointer through a sequence of bytes until a null byte is encountered.
- `initialize_sixteen_table_entries_2`: Populates a table with sixteen entries containing specific stepping and accumulating values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `null_routine`: This routine is empty and contains no valid instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_routine_2`: The routine is empty and performs no operations before returning.
- `execute_indirect_call`: Calls a routine through a pointer stored in memory.
