# Game 1889 (future wars - time travellers (1989)(u. s. gold)[cr avenger]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01889). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1804848 top-level bytes in 35 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 323 raw-word lines |
| Functions named and described | pass, 540 of 540 named (0 author symbols), 540 of 540 described (59 by rule, 154 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5332 of 5332 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18434 of 18434 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1552233 |
| code | 62468 |
| data | 143201 |
| padding | 8546 |
| unused | 38400 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:type | 2 | 117 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:setpatch | 24 | 833 | 45 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:endcli | 2 | 117 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:delphine | 3 | 16318 | 444 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:add21k | 3 | 480 | 33 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:AVENGER | 2 | 83 | 3 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:bootblock | 1 | 79 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 381 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 2 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 58 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_jump`: Copies sixteen 32-bit values from a fixed offset in a structure into a caller-supplied buffer when that buffer pointer is non-null, then jumps through a stored code pointer.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:setpatch

- `setup_setpatch_entry_point`: Program entry of the SetPatch executable: it opens the dos and exec libraries, checks the current directory, allocates memory and waits on a message port.
- `swap_and_release_task_slot`: Stores a new value into a per-task slot and, if the previous value's fields are set, closes the file and library it held, frees memory and replies to a message.
- `dispatch_twice_if_flag_set`: Calls a dispatch routine, then calls it again with the parameter only when a global flag is non-zero, and finally calls the dispatch routine once more.
- `install_setpatch_handler`: Installs a SetPatch handler and performs a series of conditional patches and library initializations.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_and_return_d1`: Calls a routine whose address is read from a global pointer at a fixed offset and returns the value left in data register D1.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `allocate_memory_under_conditions`: Checks size and flag fields of a global structure and, when they qualify, uses TypeOfMem with AllocMem or AllocAbs under Forbid and Permit to obtain a memory block, then calls a library function with the block.
- `compute_complemented_word_checksum`: Sums a block of 16-bit words starting at a fixed offset in a structure, stores the one's complement of the sum in a following field, and then transfers control through a jump table.
- `install_disk_resource_hook`: Opens disk.resource and, when a size field is small, sets a global flag, installs replacement functions through SetFunction and adjusts a global counter.
- `copy_word_on_successful_call`: Calls a stored routine pointer and, if it returns non-zero, copies a 32-bit field from a fixed offset in a system structure into a global variable.
- `dispatch_when_status_matches`: Returns at once unless a stored status word equals the expected value or is minus one; otherwise it calls through a stored routine pointer.
- `call_handler_on_minus_one`: If the input value in D1 is minus one, it calls a stored routine pointer; for any other value it returns without action.
- `stamp_marker_and_dispatch`: Sets a global to a fixed value and, if a marker at the bottom of memory does not match a constant, writes the marker and two words there.
- `call_global_slot_routine`: Calls a routine whose address is read from a global structure at a fixed offset, with no arguments.
- `fill_routine_address_table`: Stores a computed offset in a global variable and fills a sixteen-entry table with successive addresses of this routine, each two bytes apart.
- `call_routine_repeatedly_return_last`: Calls the same routine several times in sequence and returns the value produced by the final call.
- `pass_register_value`: Returns the value currently held in a data register without modification.
- `pass_register_value_2`: returns the value held in a data register without modification.
- `return_input_value`: Returns the value received in the data register unchanged, with no other operation.
- `install_function_if_index_low`: If a 16-bit field in a global structure does not exceed a fixed limit, it calls a library function through a pointer stored in a global, which installs a function with exec SetFunction.
- `copy_entry_fields_into_block`: Reads a 16-bit size field from a structure and calls a library function to obtain a block; if that returns a non-zero pointer, it stores the size there and copies the following fields across in a loop before returning a status value.
- `call_library_loop_return_flagged`: Calls a library routine once for each step of a 16-bit counter until it goes negative, then returns the 32-bit word at a pointer with its top bit set.
- `call_library_return_flagged_word`: Calls a library routine once and then returns the 32-bit word at a pointer with its top bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `invoke_pair_if_check_passes`: Calls a stored check routine with a field from a structure and, if the check succeeds, calls two further stored routines, passing the field to the last one.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `find_bsr_instruction_word`: Walks forward through 68000 instruction words and returns the first word whose top byte is the BSR branch opcode.
- `compute_branch_target_address`: Computes the target address of a 68000 branch instruction, sign-extending either the 8-bit displacement in the opcode or the following 16-bit displacement word.
- `compute_word_displacement_target`: Reads the signed 16-bit displacement word after a branch opcode and returns the address it reaches, relative to that word.
- `store_three_decoded_addresses`: Steps through a run of instruction words using the branch-target routines and stores three resulting addresses in global variables.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_with_stack_pointer`: Passes its two parameters on to another routine together with the address of a stack slot.
- `format_and_output_message`: Formats a message through exec RawDoFmt into a 140-byte local buffer, measures the resulting string length and passes the text on to an output routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:endcli

- `run_endcli_entry_max_scan`: Program entry of the EndCli executable.
- `copy_stack_words`: If a pointer is non-null, copies a fixed number of words from a source table to the stack.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:delphine

- `clear_startup_memory_block`: Zeroes a large block of memory by writing zero longwords in a loop, then stores the register value passed at entry and calls a library routine.
- `mask_byte_on_flag_condition`: When the carry flag is set and the overflow and negative flags differ, it returns the input byte ORed with a fixed mask; otherwise it stores a fixed value in a global and returns the byte unchanged.
- `set_global_status_bit`: Sets bit one of a global status byte and returns.
- `read_button_status_bits`: Builds a small result from the inverted state of bit six and bit two of a global status byte and returns it.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `store_fixed_status_value`: Stores a fixed status value in a global variable and returns; its facts list the interrupt-enable register as touched.
- `store_fixed_flag_word`: Stores a fixed value with its top bit set in a global variable and returns; its facts list the interrupt-enable register as touched.
- `set_global_bit_three`: Sets a global variable to the value with only bit three set and returns; its facts list the interrupt-enable register as touched.
- `next_pseudorandom_scaled_value`: Advances a stored pseudo-random state by shifting it and XOR-ing with a fixed feedback constant when the carry is set, then returns the state divided by the 16-bit argument.
- `acknowledge_keyboard_serial_byte`: When a request bit is set in a global status byte, it decodes the pending keyboard byte by inverting and rotating it, waits in a short delay loop and clears the request bit.
- `lookup_keycode_in_table`: Searches a table of 49 raw key codes for the input code and returns the companion value stored for the first match; codes with the top bit set or zero are handled separately.
- `copy_word_table_into_buffer`: Copies 180 16-bit words from a fixed table into a buffer reached through a global pointer, then stores a masked parameter into a header word and writes further fields from the second argument.
- `write_fixed_word_values`: Writes a fixed sequence of 16-bit constants into offsets of a structure reached through a global pointer.
- `configure_slot_fields`: Writes a fixed value, a 32-bit parameter and a zero word into fields of a structure reached through a global pointer, then writes a second fixed value to a control word.
- `read_stored_long_value`: Returns the 32-bit value stored at a fixed offset in a global table and is called from three routines.
- `store_split_address_fields`: Stores its first argument in a global unless it is minus one.
- `copy_sixteen_words_into_structure`: Copies sixteen 16-bit words from an input array into alternate word slots of a structure reached through a global pointer.
- `store_arguments_then_wait_blitter`: Stores its eleven arguments into fields of a structure reached through a global pointer, then waits until the blitter has finished.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_routine`: The routine is empty and performs no operations.
- `call_setup_with_sentinel_args`: Computes a pointer from a global base and calls a routine with it, then calls another routine with two all-ones 16-bit arguments and finally a routine with no arguments.
- `process_library_result`: Checks a returned value and, if non-zero, calls another routine with a pointer and a value, then retrieves a value from a memory offset.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `read_dh0_volume_field`: Locks the volume named DH0: through DOS, reads a 16-bit field from its information block and unlocks it, returning the value or zero if the lock cannot be obtained.
- `negate_then_call_routines`: The decompiler timed out, so only the listed facts are known: the routine has 43 instructions, negates a register, makes four calls to other routines and is called from three places.
- `tiny_neg_move`: Small routine of 2 instructions that negates d6, then copies d0 to d0.
- `forward_to_single_routine`: The decompiler timed out, so only the listed facts are known: a 22-instruction routine that makes a single call to another routine and is called from ten places.
- `setup_display_and_load_data`: Sets up display-related structures over many steps, waits for raster lines and stores several fields.
- `return_nothing`: the routine is empty and performs no operations before returning.
- `swap_two_stored_pointers`: Exchanges two 32-bit values held in globals, passes the low halves of the swapped values to another routine and calls a raster-line wait routine.
- `wait_raster_then_set_word`: Waits for a raster line to pass, then stores a fixed 16-bit word in a field of a structure reached through a global pointer.
- `process_and_clear_record_table`: Walks a table of fixed-size records and, for each non-zero entry, calls a routine with the stored value and a product of two 16-bit fields, then clears the entry.
- `allocate_free_table_slot`: Scans a table of up to 255 fixed-size records for an empty slot.
- `call_routine_for_table_slot`: Looks up a record in a fixed-size table by index; if the record is empty it returns minus one, otherwise it calls another routine with two stored fields multiplied together.
- `repeat_routine_by_count`: Calls a routine once for each step of a 16-bit counter passed in as the argument.
- `clear_two_list_heads`: Sets two 32-bit list head pointers in a global structure to null.
- `walk_list_calling_routine`: Walks a singly linked list from a global head, passing each node with a fixed size argument to another routine, then calls a finishing routine.
- `insert_node_in_sorted_list`: Walks a linked list ordered by a 16-bit key and links the given node in at the position where its key fits.
- `find_node_by_word_pair`: Walks the linked list from a global head and returns the first node whose two stored 16-bit fields match the two halves of the argument.
- `clear_list_head_pair`: Sets two 32-bit pointers in a second global list structure to null.
- `walk_list_and_clear_globals`: Walks a linked list whose head is stored in a global variable, calls another routine for each node, and clears two consecutive global variables.
- `append_new_node_to_list`: Walks to the tail of a list kept in a global head, obtains a new fixed-size node from another routine and links it at the tail.
- `create_and_link_node`: Obtains a new fixed-size node from another routine, links it into the list structures referenced by stack-based pointers and sets a 16-bit field in it.
- `reset_list_head_and_marker`: Sets a 32-bit global list head to null and a 16-bit marker field to minus one.
- `release_nodes_of_fixed_size`: Walks a linked list from a global head, passes each node with a fixed size argument to another routine, then calls a finishing routine.
- `append_node_to_list_tail`: Walks to the end of a list kept in a global head and appends an entry built from a parameter, calling another routine to create it.
- `find_list_node_by_id`: Searches a list from a global head for a node whose 16-bit identifier field matches the argument and returns a value from it, or minus one if absent.
- `remove_marked_list_nodes`: Walks a list from a global head, unlinking each node whose 16-bit field is minus one and passing each removed node to another routine.
- `reset_two_globals`: The routine writes zero to a global variable and writes a word with all bits set to another global variable.
- `walk_list_and_clear_two_globals`: Walks a linked list whose head is stored in a global variable, calls another routine for each node, and writes zero to one global and a certain bit pattern to another.
- `parse_tile_descriptor_2`: Walks to the tail of a list, allocates a new node, links it at the head, clears two tables, and calls another routine to parse a descriptor stream.
- `find_and_update_node`: The routine traverses a linked list looking for a node containing a specific short value.
- `self_referential_table_chase`: Walks a singly linked list whose head pointer is in a global variable and removes nodes from the list if a certain short field equals -1, calling another routine during the process.
- `copy_record_fields_by_index`: Reads several 16-bit fields from a fixed-size record chosen by its argument, looks up a second record using one of them and passes the combined values to two other routines.
- `dispatch_record_values`: Reads fields from a record kept in a table and passes them, together with a computed mask, to two other routines.
- `store_argument_words_to_block`: Stores the two halves of each of its two arguments into a block of 16-bit fields and derives a combined field from them.
- `call_routine_with_globals`: Passes two stored global values to another routine.
- `run_if_fields_match`: Checks three 16-bit fields in globals against fixed values and, if they all match, calls two other routines with stored values.
- `call_routines_with_table_fields`: Passes stored globals and a table field to two other routines in turn.
- `do_nothing`: the routine is empty and performs no operations.
- `store_value_by_selector`: Stores its second argument in one of several 16-bit fields of a fixed-size table record, chosen by the selector in its first argument.
- `store_record_field_word`: Writes a 16-bit value taken from a record into a field of a table entry, selected by an index stored in that record.
- `store_record_secondary_word`: Writes a 16-bit value taken from a record into a second field of a table entry, selected by an index stored in that record.
- `store_record_three_parameters`: Writes its three arguments into fields of a fixed-size table record selected by the first argument.
- `read_word_and_bit_flag`: Computes an address from two arguments and a 16-bit stride, reads the 16-bit word there and returns it together with the result of testing a bit of that word.
- `test_bit_at_computed_address`: Computes an address from its arguments and a shift amount, then returns whether the selected bit of the 16-bit word there is set.
- `run_exit_sequence_with_text`: Runs a long sequence of setup steps with raster-line waits and references the message text 'Tout est Ok . Bye' near its end.
- `orchestrate_thirteen_routine_calls`: A 432-instruction routine that calls thirteen other routines in sequence and reads many 16-bit results.
- `fill_local_buffers_call_routines`: Sets up several local stack buffers, copies table fields into them and calls five other routines.
- `find_index_matching_record`: Scans a table of 12-byte records for an active entry whose two 16-bit fields match the argument and returns its index, or minus one if none matches.
- `find_node_with_small_flag`: Walks a linked list from a global head looking for a node whose small 16-bit field is below two and whose table value matches; it returns a value from that node, or minus one when the list is exhausted.
- `refresh_active_entry_display`: Waits for a raster line, then walks a linked list of entries and, for each active entry, looks up its table record and calls other routines to update it.
- `dispatch_four_parameter_action`: Takes a pointer and four 32-bit arguments, initialises local values and calls four other routines.
- `dispatch_six_parameter_action`: Takes a pointer and five 32-bit arguments, initialises local values and calls four other routines.
- `walk_first_list_applying_routine`: Walks the linked list held in the first global list head and calls another routine with each node.
- `walk_second_list_applying_routine`: Walks the linked list held in the second global list head and calls another routine with each node.
- `decode_packed_stream_into_scratch`: Decodes a packed byte stream into a 32 KB scratch buffer on the stack, using position and count fields from its structure argument, and then calls another routine.
- `decode_stream_bytes_into_field`: Reads bytes from a data stream at the current position, stores them in a structure field and calls other routines with the decoded value; only the opening part of the decompiled fragment was recovered.
- `decode_instruction_stream`: Iterates through a byte stream to interpret specialized opcodes that modify a target index and a value or a secondary target.
- `parse_opcode_stream`: Iterates through a byte stream to decode specialized opcodes, extracting or calculating parameters from subsequent bytes and using them to update a fixed-size table or call other subroutines.
- `parse_opcode_stream_2`: Iterates through a byte stream to decode specialized opcodes that extract or modify data, such as updating a global list or calling other subroutines with parameters parsed from the stream.
- `parse_bytecode_stream`: Iterates through a byte stream to interpret specialized opcodes that load, copy, or modify values in a destination table.
- `decode_stream_byte_into_field`: Reads one byte from a data stream into a structure field, advances the stream position and calls other routines with the value; only the opening part of the decompiled fragment was recovered.
- `parse_instruction_stream`: Iterates through a byte stream to decode specialized opcodes that modify or load data into a target memory table.
- `process_opcode_stream`: Iterates through a byte stream to execute or apply instructions that modify a table or call other routines.
- `parse_encoded_data_stream`: Reads or calculates a 16-bit value from a data stream and uses it to index a table for a division or a recursive call.
- `decode_stream_word_into_field`: Reads a big-endian 16-bit value from a data stream into a structure field and calls other routines with it; only the opening part of the decompiled fragment was recovered.
- `parse_bytecode_stream_2`: Iterates through a byte stream to interpret specialized opcodes that load or modify data in memory.
- `parse_instruction_stream_2`: Iterates through a byte stream to decode specialized opcodes that modify or load data into a destination table.
- `parse_bytecode_stream_3`: Parses a stream of custom opcodes to perform indirect or direct calls to other functions and update a data table.
- `decode_instruction_stream_2`: Iterates through a byte stream to parse custom opcodes and extract associated parameters or target addresses.
- `execute_instruction_stream`: Iterates through a byte stream to interpret specialized opcodes, extracting or calculating arguments or keys and then calling a corresponding routine or updating a linked list or a table.
- `parse_opcode_stream_3`: Iterates through a byte stream to interpret specialized opcodes, extracting or calculating parameters to pass to specialized subroutines or to update a global state or table.
- `execute_opcode_stream`: Iterates through a byte stream to interpret specialized opcodes, updating a temporary buffer with or loading values from the stream.
- `parse_bytecode_stream_8`: Iterates through a byte stream to decode specialized opcodes, which update temporary memory buffers and call or pass parameters to other subroutines.
- `parse_instruction_stream_3`: Iterates through a byte stream to interpret specialized opcodes, extracting or calculating parameters to pass to other routines or to update a table.
- `parse_instruction_stream_opcodes`: Iterates through a byte stream to interpret specialized opcodes, extracting or calculating parameters to update a data table or trigger secondary subroutines.
- `decode_instruction_stream_3`: Parses a byte stream to extract or calculate pointer and value offsets, then dispatches or executes specialized subroutines based on the opcode type.
- `parse_instruction_stream_4`: Iterates through a byte stream, reading opcodes and associated or secondary data or parameters.
- `parse_bytecode_stream_4`: Iterates through a stream of opcodes to extract or modify data, such as updating a table or calling other specialized routines with parameters decoded from the stream.
- `parse_instruction_stream_5`: Iterates through a byte stream to interpret specialized opcodes, extracting or calculating target addresses and parameters to invoke other subroutines or update a global data table.
- `parse_bytecode_stream_5`: Iterates through a byte stream to extract or calculate pointer and or value offsets, then dispatches or executes specialized instructions that update a global state or call subroutines with those parameters.
- `parse_opcode_stream_4`: Iterates through a byte stream to decode specialized opcodes that update a pointer or call other subroutines with arguments extracted from the stream.
- `parse_instruction_stream_6`: iterates through a byte stream to decode and execute specialized opcodes that modify a global state or call other subroutines.
- `decode_instruction_stream_4`: Parses a byte stream to execute or apply specialized opcodes that modify or load data into a target memory table.
- `parse_instruction_stream_7`: Iterates through a byte stream and dispatches specialized opcodes to perform data-loading or pointer-based updates.
- `parse_opcode_bytecode_stream`: Iterates through a byte stream, decoding specific opcodes to extract or calculate certain values, then calls specialized routines or appends new nodes to a linked list based on the decoded instructions.
- `decode_instruction_stream_5`: Parses a byte stream to execute specialized opcodes that extract or modify data or call other routines.
- `read_null_terminated_stream_string`: Copies bytes from a stream into a local buffer until a zero byte, then passes the buffer to other routines.
- `load_animation_by_stream_name`: Reads a zero-terminated name from the stream into a local buffer and passes it to another routine to load; its error path uses the referenced format text 'ECHEC LoadAni %s'.
- `parse_bytecode_instruction_stream`: Iterates through a byte stream to extract and decode specialized opcodes and their associated parameters.
- `load_background_by_stream_name`: Reads a zero-terminated name from the stream into a local buffer and passes it to another routine to load; its error path uses the referenced format text 'ECHEC LoadBg %s'.
- `parse_resource_command`: reads a null-terminated string from a data stream and processes a command-based instruction set or table-driven dispatch.
- `load_colour_table_by_name`: Reads a zero-terminated name from the stream and passes it to another routine to load; its error path uses the referenced format text 'ECHEC LoadCt %s' and then runs a loop that reads further stream data.
- `run_stream_opcode_loop`: Reads opcode bytes from a stream and branches on each one through a switch, performing the matching field stores and routine calls; the decompiled fragment is only partly recovered.
- `parse_bytecode_stream_6`: Iterates through a byte stream to decode specialized opcodes, extracting or calculating parameter values and passing them to a secondary routine or updating a data table.
- `parse_instruction_stream_8`: Iterates through a byte stream to decode specialized opcodes, extracting or calculating target pointers and associated data or offsets.
- `store_stream_value_in_struct`: Reads a stream position and stores its value in a 16-bit field of a structure reached through a global pointer, then calls another routine.
- `execute_opcode_stream_2`: Parses a stream of custom instructions to modify memory or trigger external subroutines.
- `parse_instruction_stream_9`: Parses a custom bytecode stream to update internal state or trigger secondary functions.
- `parse_opcode_stream_5`: Iterates through a byte stream to decode specialized opcodes, updating or accessing linked list nodes with data or pointers extracted from the instruction stream.
- `parse_bytecode_stream_7`: Iterates through a byte stream to decode specialized opcodes, extracting or calculating destination addresses and data values to call secondary routines or update a target memory table.
- `parse_stream_opcode_instructions`: Iterates through a byte stream to interpret or execute specialized opcodes that load or modify data in memory.
- `fill_array_with_minus_one`: Sets a run of 16-bit entries in an array to minus one in a loop, then calls another routine.
- `decompress_byte_stream`: Implements a custom decompression or expansion routine that reads a control byte from a source buffer and writes a variable number of 16-bit words to a destination buffer.
- `expand_byte_stream_2`: Decompresses a custom-encoded byte stream into a sequence of 16-bit words.
- `decompress_variable_length_stream`: Decodes a custom variable-length encoded byte stream into a sequence of 16-bit words.
- `expand_byte_stream_3`: Decodes a compressed or encoded byte stream into a buffer of 16-bit words.
- `decompress_byte_stream_2`: Decodes a custom variable-length-encoded byte stream into a destination buffer of 16-bit words.
- `decompress_byte_stream_3`: Decompresses a custom-encoded byte stream into a destination buffer of 16-bit words.
- `decompress_byte_stream_4`: Reads a compressed or encoded byte stream and expands it into a sequence of 16-bit words.
- `decompress_byte_stream_5`: Expands a compressed byte stream into a destination buffer of 16-bit words using a variable-length instruction-based decoding scheme.
- `decompress_byte_stream_6`: Expands a custom-encoded or compressed byte stream into a destination buffer of 16-bit words.
- `expand_byte_stream_4`: Decompresses a custom-encoded byte stream into a destination buffer of 16-bit words using a command-based lookup.
- `expand_byte_stream_variant`: Decodes a compressed or variable-length byte stream into a sequence of 16-bit words.
- `decompress_byte_stream_7`: Decodes a variable-length, custom-encoded byte stream into a sequence of 16-bit words.
- `decompress_byte_stream_8`: Expands a compressed byte stream into a destination buffer of 16-bit words using a custom variable-length instructionset.
- `decompress_byte_stream_9`: expands a compressed or encoded byte stream into a destination buffer of 16-bit words using a lookup-based or instruction-based decoding scheme.
- `decompress_byte_stream_10`: Decompresses a variable-length, command-based byte stream into a destination buffer of 16-bit words.
- `expand_byte_stream`: Reads a stream of bytes from a source buffer and expands them into a destination buffer of 16-bit words.
- `compare_words_return_flags`: Compares two 16-bit values from its argument and returns a flag byte: bit one when they are equal, bit two when the second is smaller, and bit four when the first is smaller.
- `blit_shifted_bitmap_rows`: Computes a pixel shift from the low four bits of a coordinate, then copies rows from a source buffer into a destination, calling three other routines.
- `return_input_quad_value`: Returns its eight-byte argument unchanged.
- `blit_clipped_bitmap_rows`: Blits bitmap rows with coordinate clipping, shifting by the low four bits of a coordinate and calling other routines for each row.
- `return_parameter_unchanged`: The routine receives a single value in a register and returns it unchanged.
- `blit_masked_bitmap_rows`: Blits bitmap rows through a source mask, shifting by the low four bits of a coordinate and calling other routines for each row.
- `pass_parameter_through`: The routine receives a single value in a register and returns it unchanged.
- `call_blit_with_full_mask`: Passes its two arguments to another routine together with fixed clipping values: a full 16-bit mask and a size of 800.
- `call_blit_for_four_bitplanes`: Passes one address to another routine four times, advancing it by 8000 bytes each time (the size of one 320 by 200 bitplane), with a full mask and 200 rows.
- `null_operation_routine`: This routine is empty and performs no operations before returning.
- `prepare_blit_arguments_and_call`: Initialises five local values to zero, calls one routine and then calls another routine with the arguments it was given.
- `blit_row_span_with_masks`: Computes start and end word masks for a horizontal span from x coordinates, fills four local mask words and calls three other routines to draw the span.
- `blit_span_to_screen_address`: Computes the screen address for a rectangle from its coordinates and calls another routine with a full-width mask for the span.
- `dispatch_single_call_undecompiled`: The decompiler timed out, so only the listed facts are known: a small routine that makes a single call and is called from six places.
- `render_table_entry_bytes`: Reads a byte pointer from a six-byte table record selected by its argument, processes the bytes it points to and calls seven other routines.
- `compute_screen_address_and_mask`: Computes a screen address and a bit mask from its coordinate arguments and returns them.
- `handle_undecompiled_entry`: The decompiler timed out, so only the listed facts are known: the routine is called from four places and has no recovered pseudo-code.
- `walk_first_list_for_match`: Walks the first global list and passes the node matching its argument to another routine.
- `swap_linked_list_node`: The routine obtains a new node via another routine and stores it through a pointer derived from a stack variable.
- `insert_node_at_list_tail`: The routine walks a linked list starting from a global variable until it reaches the last node, obtains a new node via another routine, links the new node after the last node, copies a word from a parameter into the new node and sets another word field to a constant, then updates a global variable pointer with the new node address.
- `process_input_string`: Iterates over a byte string and, if a character matches a certain value and a table lookup returns a non-zero value, calls a routine with a computed address and a fixed offset.
- `decode_stream_record_fields`: Reads bytes from a stream into local stack fields and passes them to eight other routines.
- `render_stream_rows_by_mode`: Waits for a raster line, then reads a byte stream into local fields; a mode argument selects a row width of 80, 20 or 140, and other routines are called with the results.
- `wait_raster_and_measure_span`: Waits for a raster line, then computes and returns a clipped length from its arguments and several local coordinate fields.
- `draw_clamped_block_rows`: Clamps a vertical extent so that it does not pass line 199 of the screen, waits for a raster line and then calls eleven other routines to draw the block's rows.
- `clear_slot_table`: Clears the first byte of each of twenty fixed-size slots, then runs a second loop that calls another routine.
- `lookup_slot_word_value`: Finds a slot index in a table and returns the value stored for it; it returns minus two when the first lookup fails and minus one when no slot matches.
- `iterate_slot_entries_after_lookup`: Looks up a slot and returns minus two if the lookup fails; otherwise it calls other routines in a loop over the entries.
- `compute_clipped_rectangle_mask`: Computes a clipped rectangle mask from coordinate arguments, shifting values by the low nibbles and returning the combined word.
- `update_grid_cell_values`: Waits for a raster line, then loops over a grid of stored values, calling another routine for each cell and writing the result back.
- `update_grid_cell_values_negated`: Waits for a raster line, then loops over a grid of stored values using negated step values, calling another routine for each cell and storing the result.
- `fill_word_table_range`: Loops over an inclusive index range and stores into each slot of a word table the value returned by a routine call.
- `rotate_word_table_range`: Shifts a range of words in a table by one slot, moving the last word to the front when the mode is one and the first word to the back when the mode is two.
- `shift_words_up_insert`: Moves words in the table one slot toward higher indexes until the lower bound is reached, then writes a saved word into the bound slot.
- `rotate_left_or_copy_range`: When the mode argument is two, rotates a word range left by one slot so the first word moves to the end; otherwise it copies the range from a second table.
- `shift_word_range_left`: Moves each word in a range one slot toward lower indexes and stores the input word in the top slot of the range.
- `copy_word_range_from_buffer`: Copies words from a second buffer into the table over an index range taken from two global fields.
- `copy_word_table_range`: Copies words from a second buffer into the table from a starting index through an upper bound inclusive.
- `open_database_and_load_table`: Clears 255 thirty-byte records and resets two counters, then opens the named database file through the dos read path.
- `clear_records_and_release_handle`: Clears the same 255 thirty-byte records, then if a stored file handle is positive it passes that handle to a routine and marks the handle as unused.
- `split_words_into_four_planes`: Reads four consecutive words at a time from a source buffer and writes each one to a different destination area, with the areas spaced evenly apart.
- `read_name_find_record`: Reads a run of bytes from a source into a buffer, then scans up to 255 records of the table and returns the index of the first one for which the comparison routine returns zero, or minus one if none matches.
- `seek_and_read_record_data`: Uses a record selected by its first argument to seek in the open file to an offset stored in that record, then reads a block whose length is also stored in the record into the buffer given by the second argument.
- `copy_bytes_and_advance_source`: Copies a given number of bytes from the buffer whose pointer is held in the first argument into the destination, then stores the advanced source pointer back.
- `allocate_animation_frames`: Looks up a record and, when found, swaps two screen buffers that sit 32000 bytes apart before loading its frames.
- `remap_three_nibbles`: Returns zero for a zero input; otherwise it transforms each of the three low nibbles of the word by computing twice the nibble plus one and keeping the low four bits, then repacks the three results into one word.
- `read_record_color_table`: Looks up a record and, when found, swaps two screen buffers 32000 bytes apart, copies a 32-byte block, and reads sixteen words into the 16-word table, returning one.
- `load_record_name_by_extension`: Looks up a record for a file name and, when the name contains a dot extension, copies a 32-byte name block into the record at an offset chosen by whether the extension starts with N. It returns one when an extension is present and zero otherwise.
- `copy_name_by_extension`: Scans a file name for a dot and, if a valid extension follows, copies a 32-byte name block at an offset chosen by whether the extension starts with N, otherwise copying a two-byte value first.
- `copy_name_block_by_extension`: Checks the name passed in for a dot extension and copies the name block into the data area, with the target offset depending on whether the extension starts with N. It returns one when the extension is present.
- `check_dot_extension_and_copy`: Checks whether the character at a given index is a dot followed by a character still inside the name length, and if so copies an extension block, choosing the offset by whether the next character is N. It returns one for a match and zero otherwise.
- `copy_word_and_name_block`: Copies a two-byte value and then a 32-byte name block into the record buffers, sets the data pointer past the fixed header, and returns one.
- `return_zero_stub`: Takes no action and returns zero, so callers always see the failure result.
- `read_sized_block_from_file`: Opens the named file, reads two short values from it, and reads a block whose length is their product into the buffer given as the second argument before releasing the handle.
- `decompress_backwards_bit_stream`: Decodes a packed stream by filling the output buffer from its end, alternating between copying a run from earlier output and writing literal bytes read through a bit reader, until the start of the buffer is reached.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_buffer_with_refill`: Shifts a 32-bit bit buffer right one bit at a time for a count taken from a register, loading the next word from memory whenever the buffer runs empty.
- `load_slot_table_from_file`: Releases any non-zero slots in a 50-entry table, then opens the named file and reads an entry count and entry list into the slots.
- `read_entries_skipping_marked`: Looks up a file record and reads a count-prefixed list of entries.
- `load_twelve_byte_entry_table`: Clears a 255-entry table of 12-byte records, then looks up the named record and reads a count followed by four words per entry into the table.
- `load_six_byte_index_table`: Clears a 255-entry table of six-byte records, then looks up the named record and reads a count followed by word values into the table.
- `process_four_flagged_entries`: Checks four flag bytes in the global area and, for each set flag, calls a sequence of routines with offsets computed from the same base and a different constant for each entry.
- `reset_all_game_state_tables`: Clears the entry tables, the 32-byte record table, the 16-word table and several scalar variables, then calls routines that reset named records.
- `load_directory_table_from_file`: Opens the LVDT.DIR file and, if it can be read, reads up to 200 bytes of directory data into a table, closes it, and returns one.
- `read_directory_block`: Reads a 200-byte block from the open file handle into the directory table, closes the handle, and returns one.
- `load_backup_game_state`: Opens the named backup file and, if it cannot be found, reports that the backup does not exist and returns minus one.
- `read_game_state_from_file`: Clears the entry tables, the word tables and several scalar variables, then reads a long sequence of game state fields from the open file into fixed buffers, each with its own byte count.
- `read_three_pass_entry_block`: Reads a counted block of entries from the file in three passes, each pass reading a count-driven number of records, then calls a routine for each non-zero flag in the global area and returns zero.
- `read_counted_entries_and_flags`: Reads a counted block of entries from the file in two passes, then calls routines for each non-zero flag in the global area and returns zero.
- `read_entries_with_flag_calls`: Reads a counted block of entries from the file and then calls a routine for each non-zero flag in the global area, returning zero.
- `read_single_pass_entry_block`: Reads one counted block of entries from the file and runs the same flag-gated routine calls on the result, returning zero.
- `allocate_and_link_record_node`: Walks a linked list to its tail, allocates a new node of fixed size and links it there, then reads five fields into the node, including two 100-byte blocks and a slot index whose table value it stores in the node.
- `replace_node_in_linked_list`: This routine traverses a linked list to find a node, replaces a node in the list with a new node, and then updates a value in a table using an offset calculated from a table entry.
- `allocate_linked_record_at_tail`: Walks a list to its tail, allocates a fixed-size node and links it, then reads five fields into it, including two 100-byte blocks, and stores the table value selected by a slot index in the node.
- `read_and_link_buffer_chain`: Allocates a fixed-size buffer, links it into a list, reads two 100-byte blocks and three 2-byte values into it through the read wrapper, and stores the table entry selected by the last value.
- `refresh_record_table_links`: Walks the record list and, for each node, stores into it the table entry selected by the index field already held in that node.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `allocate_record_node_second_list`: Allocates a fixed-size node, links it at the tail of a second list, reads five fields into it, and stores the 12-byte table entry selected by the last field.
- `read_link_buffer_chain_alt`: Allocates a fixed-size buffer, links it into a list, reads two 100-byte blocks and three 2-byte values through the read wrapper, and stores the 12-byte table entry selected by the last value.
- `refresh_second_list_entries`: Walks the second record list and, for each node, stores into it the 12-byte table entry selected by the index field already held in that node.
- `empty_return_routine`: Contains only a return, so calling it has no effect.
- `allocate_and_read_small_node`: Allocates a 20-byte node, reads 20 bytes from the open file into it, and links it at the tail of the list.
- `read_link_small_buffer`: Allocates a 20-byte buffer, reads 20 bytes into it through the read wrapper, then links the buffer into a global list using a global variable as the base.
- `allocate_small_node_second_list`: Allocates a 20-byte node, fills it from the input, and links it at the tail of a second list.
- `read_link_buffer_second`: Allocates a 20-byte buffer, reads 20 bytes into it through the read wrapper, and links it into a second global list using that list's base.
- `read_entry_blocks_for_records`: Walks a 255-entry table of 26-byte entries and, for each active entry that refers to a record, reads a 22-byte header and then the record's data block from the open file into the entry's slots.
- `read_entry_blocks_with_offsets`: Loops over active 26-byte entries, reading each entry's record data block from the open file using the record's stored size and placing the data at the entry's offsets.
- `read_entry_blocks_continued`: Continues the entry-block read loop over the 26-byte table, reading record blocks for each entry that refers to a valid record, until the index passes 254.
- `read_entry_blocks_second_pass`: Runs a second pass over the 26-byte entry table, reading record blocks for entries that share the current record index.
- `read_entry_blocks_final_pass`: Runs a final pass over the 26-byte entry table, reading record blocks for each matching entry and then calling a routine to finish the entry set.
- `scan_entry_table_for_record`: Scans the 26-byte entry table from its current index for the next active entry, and for each entry that refers to a record calls a routine on it, stopping when the index passes 254.
- `process_small_node_list_entries`: Walks the 20-byte node list and, for each node, calls a routine with the matching 26-byte entry's handle and fields, using a second call form when the node's word is non-zero.
- `process_active_node_entries`: Walks a node list and calls a routine for each node whose table entry is active, with arguments taken from the node's fields and the entry's size, and stops when the list ends.
- `process_flagged_node_entries`: Walks a node list and applies the same entry-table routine calls to each node, with the call form depending on whether the node's second field is zero.
- `no_op_return_stub`: Returns immediately without doing any work.
- `prompt_insert_disk_volume`: Builds a prompt that asks the user to insert a named disk and volume, shows it through the text routines, and waits in a loop until the response is confirmed.
- `busy_wait_until_flag_clear`: Calls a routine repeatedly until a local flag word becomes zero, then enters an endless loop calling two routines, using the raster-line wait for timing.
- `setup_blitter_and_wait`: Waits for the raster line, then sets raster and blitter parameters using two screen buffers offset by 32000 bytes and performs a single blit between them.
- `draw_text_lines_centered`: Measures a bar-separated text block to find its longest line, sizes and centers a box for it, then draws each character by copying glyph rows from a font table into both screen buffers.
- `draw_text_block_words`: Measures and lays out a bar-separated text block like its neighbours, then draws each word's characters from the glyph table, advancing across the line with spacing values.
- `draw_text_block_frame`: Continues a text block draw by sizing and centering the box, drawing each character row, and then drawing a final frame line.
- `draw_text_block_timed`: Waits for the raster line, sizes and centers a text box from the longest bar-separated line, draws each character in the block, and closes the box with frame lines.
- `draw_text_lines_until_end`: Draws each bar-separated line of text in a centered box, advancing down the box until the text ends, then draws the closing frame lines.
- `draw_text_box_with_input`: Sizes a box for a bar-separated text block, draws each line and its characters from the glyph table, then loops reading keyboard input and redrawing until a confirm key is pressed.
- `read_text_input_loop_alt`: Reads key codes in a loop, echoing accepted letters, digits and spaces into a text buffer, removing the last character on backspace, and ending when the enter or escape key is read.
- `draw_text_input_box_full`: Sizes and draws a full text input box, placing each typed character from the glyph table and handling backspace, enter and escape, with a loop that repeats until input ends.
- `read_text_input_line`: Waits for the raster line, then reads key codes one at a time into a buffer of up to nineteen characters, accepting letters, digits and spaces, handling backspace, and ending on enter or escape.
- `read_text_input_chars`: Reads key codes into a text buffer, echoing accepted letters, digits and spaces, removing characters on backspace, and redrawing the entry until the enter or escape key ends input.
- `poll_text_input_keys`: Polls key codes and updates the text entry buffer: backspace removes the last character, accepted characters are drawn from the glyph table, and enter or escape ends the poll.
- `invoke_library_pair`: Calls two routines in sequence, one of which is a library call through a fixed vector, then returns.
- `update_sprite_offsets`: Compares the stored index with its parameter and, when they differ, adjusts two running offsets by the change in stored positions, copies sixteen words from the table entry into the buffer, clears two trailing words, and records the new index.
- `initialize_display_structure`: Copies two stored values into place, writes two constant words and a sixteen-word block from a table into a structure, clears two trailing words, and resets the index variable to zero.
- `enable_interrupts_and_globals`: Stores a frame pointer and constants into global variables and writes to the interrupt enable register, which enables interrupts, then returns a pair of input registers.
- `set_dma_interrupt_control_words`: Stores a saved value and two constants into the global control word, then returns the input register pair.
- `delay_loop_with_flag_check`: Checks bit 5 of a global control word and calls a routine when it is set, then counts a 16-bit register down to zero as a delay before returning the input pair.
- `delay_loop_on_interrupt_request`: Checks bit 5 of a global control word and calls a routine when it is set, then counts a 16-bit register down to zero, using the interrupt request register on this path.
- `read_mouse_port_movement`: Reads the mouse port counter, computes the horizontal and vertical change since the last read with wrap-around handling, clamps the accumulated position to a fixed range, and writes the position to two output words.
- `update_mouse_position_from_registers`: Applies a wrap-around movement delta from its input registers to the stored mouse position, clamps the result to the fixed screen range, and writes the position words.
- `clamp_position_into_copper_list`: Applies a movement delta to the stored position, clamps it to the screen range, writes the position words into the copper list, and stores a global value.
- `setup_line_blit_from_points`: Derives blitter line parameters from two points, including the octant and shift values, writes them to the blitter registers, and waits until the blitter flag clears.
- `copy_length_prefixed_string`: Copies a length-prefixed string from the source into the destination byte by byte, then writes a terminating zero after the copied bytes.
- `search_string_table`: Walks a linked table of entries from a global index, copies each candidate name into a local buffer, and compares it with the input name, setting a flag and stopping when a match is found.
- `find_trackdisk_device`: Looks up trackdisk.device through the exec library name lookup.
- `call_with_nonzero_check`: Calls a routine and, when it returns a non-zero value, passes that value to a second routine and returns zero; otherwise it returns minus one.
- `poll_with_periodic_message`: Polls a status routine until it reports ready, and every hundred polls shows a message through a routine, clearing the message state afterwards.
- `poll_status_with_message_repeat`: Repeats a status read until it signals completion, showing a message every hundred failed polls and clearing the displayed message when the loop exits.
- `prompt_insert_formatted_diskette`: Waits on a disk status read while showing the insert-a-formatted-diskette message every hundred polls, and clears the message when the disk is ready.
- `wait_until_status_bits_clear`: Repeats a status read until its low two bits are clear, twice, then reads a combined value and returns its high word.
- `poll_status_three_stages`: Polls a status read until its low bits are clear, repeats the poll twice more, and returns the high word of the final read.
- `poll_status_nested_stages`: Polls a status word with nested waits, first for a clear low-bit state and then for a final value, and returns the high word once the state is clear.
- `poll_status_final_value`: Waits until a status read reports a clear low-bit state and returns the high word of that read.
- `read_status_until_clear`: Returns the high word of the status read when its low bits are clear, polling the status routine while they are set.
- `manage_dos_library_resources`: Waits for a busy flag, opens dos.library through the exec library when needed, runs calls on the opened library base, and records that the resource is held.
- `examine_and_lock_file`: Locks a named file through the dos library, reads an attribute value from its file information block, unlocks it, and frees the memory it allocated, returning the attribute.
- `set_global_flag_one`: Stores the value one into a global flag variable and returns.
- `wait_flag_until_value`: Sets a global flag to one if it is zero, then busy-waits until the flag equals sixty-four.
- `initialize_cia_resource`: Opens the ciaa resource through exec when the setup flag is set, copies a block of configuration bytes into the global area with running sums, obtains a value through a library call, and marks the setup as done.
- `shutdown_audio`: When the audio-active flag is set, calls a library routine, runs a helper that stops the audio channels, and clears the flag.
- `conditional_free_memory`: When two global handles are both set, calls the exec library to free memory and clears one of the flags.
- `compute_max_indexed_value`: Scans a table of bytes for its largest value, then returns that maximum plus one, scaled by 1024 and combined with the input register.
- `write_cia_timer_registers`: Writes a constant to a global, copies a value from RAM into a second global, and copies that into a third; the stored values correspond to the CIA-A control and timer registers.
- `clear_pointer_table_entries`: Writes zero to the memory location referenced by each of fifteen pointers in a table.
- `reset_audio_and_custom_state`: Writes zero to four audio volume registers and a custom register, sets one global to all ones, and clears several audio state globals.
- `initialize_audio_dma_channels`: Clears audio volume registers, points the four channel location registers at a shared global, sets their length registers to one, and writes a value into a global.
- `call_then_return_zero`: Calls one routine and returns zero.
- `advance_audio_channel_state`: Advances a frame counter and, on every sixth call, steps all four audio channels using channel-specific register addresses; otherwise it updates a playback state variable and caps it at sixty-four.
- `call_routine_and_pass_stack`: Calls one routine and returns the value found on the stack.
- `write_audio_channel_pointers`: Loops over four audio channels, calling a routine with a decreasing channel index and a register address that steps by sixteen bytes each time.
- `process_audio_channel_command`: Reads a command word for an audio channel and either applies a slide to its period, sets the value directly, or runs a jump-table command, updating the channel's stored period and volume.
- `step_channel_period`: Adds an input step to a stored value, clamps the result to a lower bound, and stores it as the channel value.
- `process_audio_channel_effect`: Interprets the effect nibble of an audio channel command, handling slides, flag set and clear, and table-based pitch lookups, and updates the channel's stored period and value.
- `set_disk_status_bit`: Sets bit one of a global status byte; the register note relates to the fire button and disk status register.
- `clear_disk_status_bit`: Clears bit one of a global status byte; the register note relates to the fire button and disk status register.
- `decode_sound_command_entry`: Stores a zero header in a record, copies a value from the source and a nibble-derived field, then searches a table for a matching value and stores it as the final field.
- `copy_word_with_signed_offset`: Computes a signed word offset from its input, negating it when the sign argument is zero, and copies the word found at that offset from the source into the destination structure.
- `store_input_word_into_field`: Stores the word passed in the input register into a field of the structure in the current frame.
- `decode_command_with_sign`: Decodes a command word by building a sign-adjusted nibble value, zeroing the header, then looking up the matching table entry and storing it with a signed offset.
- `dispatch_jump_table_command`: Calls a routine selected from a jump table by a channel command index, using a table of routine addresses.
- `lookup_table_value_by_key`: Searches a word table for the entry equal to a key and stores the element at the same position in a second table into the output field.
- `lookup_high_nibble_indexed_value`: Searches the word table for a key and stores the entry selected by the high nibble of a source byte.
- `lookup_low_nibble_indexed_value`: Searches the word table for a key and stores the entry selected by the low nibble of a source byte.
- `copy_word_to_field_direct`: Copies the input word from the offset field into the output field without changing anything else.
- `adjust_value_by_nibble`: Adds the high nibble of a source byte to a stored value when it is non-zero, otherwise subtracts the low nibble, and stores the result as the output.
- `decrement_short_by_nibble`: Subtracts the low nibble of a source byte from a stored short and stores the result to the output offset.
- `configure_audio_dma`: Clears a global, calls a routine four times, sets a DMA-related global, waits on a delay counter, copies four structure words into the audio location and DMA control registers, and advances a wrapping index.
- `update_channel_sequence_state`: Loads a channel step record and, when its command is valid, computes the channel's next volume from the table, stores the channel fields, waits on a delay counter, and merges the channel bit into the global status.
- `update_channel_volume_envelope`: Updates a channel's running value and volume from a record, clamping it to sixty-four and applying a sustain or decay adjustment, then stores the result into the channel structure.
- `clamp_channel_volume_to_limit`: Adds a step to a channel volume, clamps it to sixty-four, subtracts the current global threshold, and stores the result; when the record command is non-zero it also writes the channel fields after a delay.
- `apply_channel_command_record`: When the record command is non-zero, zeroes two channel words, copies the record's fields into the channel, waits briefly, and merges the command bit into the global status.
- `store_fields_and_or_flag`: Copies a record's words into a channel structure and ORs a word from the record into a global status flag.
- `handle_sound_record_pair`: The decompiler produced no pseudo-code, so this routine is described only by its calls: it is referenced from the global data and calls two other routines.
- `clear_channel_slot_pair`: Clears the word at the record pointer and the word at the slot offset computed from the stored index, then calls a routine twice to set the entry values with the values sixty-three and minus one and the values one and zero.
- `decrement_and_initialize_sound`: Subtracts ten from a stored counter, clears a pointed word and a second word at a computed offset, clamps a value to sixty-three, and calls a routine with the result.
- `forward_to_one_routine`: The decompiler timed out, so only the listed facts are available: the routine is referenced from the global data and calls one other routine, and it has seven instructions.
- `clear_and_initialize_sound_twice`: Clears a word pointed to by the record pointer and a second word at an offset from it, then calls a routine twice, once with the mask value and once with a fixed one-and-zero pair.
- `open_cia_resource_init_audio`: Opens the ciaa resource through exec once, calls a library vector on the result, and sets a flag to record that the audio setup is done.
- `conditional_library_reset_audio`: When a setup flag is set, calls a library function through a stored base, runs a helper, and then clears the flag.
- `write_cia_control_values`: Writes a constant and a RAM value into global control variables, copying the value through a third global; the stored values correspond to the CIA-A control and timer registers.
- `clear_audio_pointers_and_counters`: Clears four audio channel length and volume registers and points four channel pointers at a shared global, then zeroes four RAM counters.
- `reset_audio_and_enable_dma`: Clears audio channel registers and pointers, sets a global to fifteen, and zeroes four RAM counters before returning.
- `reset_audio_channels`: Calls a routine that resets the audio channel state and returns the input register unchanged.
- `cycle_audio_channel_pointers`: Counts calls up to six, calling a routine for each of four channels on every count; on the sixth call it resets the count, writes a status bit, waits, and points each channel at the shared global when its value is zero.
- `reset_audio_channels_and_dma`: Clears two global words, calls a routine three times, writes a status value with the top bit set, waits on a delay counter, and points each audio channel at the shared global when its value is zero.
- `initialize_sound_channel_state`: Computes a 22-byte-stride offset from a channel index, zeroes a word in RAM, and stores the parameters into the channel record at that offset.
- `advance_channel_envelope_counter`: Counts down a channel's delay counter and, when it reaches zero, reloads it and adds a step to the channel value, clamping the result to sixty-three and storing it in the output.
- `clamp_and_copy_short`: Copies a word within the channel record, adds two words, clamps the sum to the range zero to sixty-three, and stores the clamped value back into the record and the output.
- `apply_channel_record_if_set`: When the record's pointer field is non-zero, copies the record's fields into the output structure, clears the record value when its flag is zero, and merges the channel bit into the global status.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_update_scroll_state`: Copies a block of 32-bit words from the source into the destination when a flag is clear, then updates scroll state variables such as a base offset, table pointers and a signed index.
- `initialize_window_state`: Initializes a record pointed to by a global, stores a constant marker, opens its input and output handles through dos library wrappers, and runs several library calls to finish the setup.
- `find_directory_entry_by_name`: Either updates shared file-state fields and returns zero, or compares a requested name with a stored name and, on a match, builds a linked list of entries and returns the count.
- `open_file_entry_list`: Either updates shared file-state fields through dos library calls and returns zero, or compares a requested name with a stored name and, on a match, builds a linked list of entries and returns the count.
- `empty_routine_2`: The routine is empty, containing only a return instruction.
- `copy_null_terminated_string`: Copies a zero-terminated string from the source into the destination byte by byte, including the terminator, and returns the destination pointer.
- `copy_string_to_buffer`: Copies bytes from the source to the destination until and including a zero byte, then returns the original destination pointer.
- `compute_string_length`: Walks a zero-terminated string and returns its length, counting the bytes before the terminator without changing memory.
- `convert_to_uppercase`: Converts a lowercase ASCII letter in the input byte to uppercase by subtracting thirty-two, and returns any other byte unchanged.
- `convert_to_lowercase`: Adds thirty-two to an uppercase ASCII letter to make it lowercase, and returns any other byte unchanged.
- `compare_two_byte_strings`: Compares two zero-terminated byte strings up to a fixed limit, returning minus one when the first is smaller at the first difference, one when it is larger, and zero when they match.
- `safe_memory_copy`: Copies a counted block of bytes from source to destination, copying backwards when the regions overlap and the destination lies above the source.
- `clear_globals_and_dispatch`: Clears a large block of global longwords, stores the stack pointer into a global, checks a library flag and optionally calls a routine, then calls a library function and runs one of two follow-up routines depending on the result.
- `do_nothing_routine`: this routine is empty and performs no operations before returning.
- `split_quoted_fields_into_array`: Copies a string into a buffer while splitting it into fields at spaces and tabs, with double quotes protecting spaces, then builds a null-terminated array of pointers to the fields.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `append_string_to_buffer`: Finds the end of the destination string and appends the source string to it, stopping at a fixed limit and terminating the result.
- `append_string_with_limit`: Appends a source string to the end of the destination, copying at most a counted number of bytes and writing a terminator when the copy is cut short.
- `copy_string_pad_zeros`: Copies bytes from the source up to a count or until a zero byte, then fills any remaining bytes of the count with zeros.
- `open_window_from_icon_library`: Opens a window using the WINDOW and icon.library references and dos library wrappers, stores the resulting handle in the window structure, and closes the temporary handle.
- `call_and_terminate_buffer`: Stores the first argument as a buffer pointer, calls a formatting routine with the remaining arguments, then writes a zero byte at the buffer and returns the call result.
- `store_param_and_dispatch`: Stores the first argument in a global buffer pointer, calls a routine with the second argument, then writes a zero byte at the stored pointer and returns the 16-bit result.
- `increment_and_store_byte`: Stores the low byte of its argument at the current position of a global pointer, advances that pointer by one, and returns the byte.
- `forward_to_format_routine`: Passes its argument, together with the address of the stacked arguments, to a formatting routine.
- `call_two_helper_routines`: The decompiler timed out, so only the listed facts are available: the routine calls two other routines and is called by two routines.
- `format_string_into_buffer`: Walks a printf-style format string, emitting literal characters through a callback and expanding each conversion, including width and precision, for character, decimal, hexadecimal, string and padded fields.
- `perform_signed_division`: Determines the sign of the result from two signed inputs, calls a division routine, negates its result when needed, and returns the combined value with the second input.
- `call_and_return_concatenated`: Calls another routine and returns a value combining the original second register with the value that call leaves behind.
- `recursive_call`: Calls one routine and returns; the callee is the same routine, so no other work is done.
- `perform_32bit_division`: Divides a 32-bit value by a 32-bit divisor, using a bit-by-bit shift loop when the divisor is large and 16-bit division otherwise, and returns the quotient and remainder together.
- `call_other_routine`: Calls one other routine and returns.
- `dispatch_three_helper_calls`: The decompiler timed out, so only the listed facts are available: the routine calls three other routines and is called by two routines.
- `execute_three_subroutines`: This routine calls three other subroutines in sequence without performing any other operations or modifying any registers.
- `dispatch_two_helper_calls`: The decompiler timed out, so only the listed facts are available: the routine calls two other routines and is referenced from one routine by its address.
- `store_byte_and_advance`: Stores a byte from the argument structure at the current position of a pointer, advances that pointer by one, and returns the byte.
- `walk_record_table_entries`: Walks a table of 22-byte records from a base address up to a fixed end address, calling a routine on each record.
- `release_record_and_flush`: Returns minus one for a null record; otherwise, when the record's active flag is set, calls routines for its pending data and combines their results, then clears the record's fields and returns the combined value.
- `flush_record_pointer_state`: Flushes a record like its neighbour: when the record is active it calls routines for pending data and combines their results, then clears the record and returns the combined value.
- `write_record_byte_flagged`: Stores a byte into a record's buffer when the record allows it, updating the buffer pointer and state flags, and returns the byte, or minus one when the record is in an error state.
- `write_record_byte_register`: Stores a byte into a record's buffer as its neighbour does, updating the buffer pointer and state flags, and returns the byte, or minus one when the record is in an error state.
- `write_record_byte_first`: Writes one byte into a record buffer, setting the buffer pointer from the record's base when none is set, and returns the byte written or zero when the buffer is closed.
- `read_or_write_record_byte`: Initializes the record pointer when needed, then either stores a byte from the argument into the record buffer or reads through a routine when the record's mode is one, and returns the value.
- `store_record_byte_direct`: Stores the byte from the argument into the record buffer, updates the buffer pointer and active flag, and returns the byte.
- `find_free_record_slot`: Scans the record table from its start for an unused slot, clears that slot's fields, and returns its address, or zero when every slot is in use.
- `claim_free_record_slot`: Scans the record table for an unused slot, clears the first three longwords of that slot, and returns it, or zero when none is free.
- `clear_record_fields`: Clears the first three longwords of a record.
- `init_record_mode_fields`: Sets a record's mode field to one and its buffer pointer to the record's own data area when the source value is zero; otherwise it sets the mode to 1024, marks the record active, stores the value as the buffer pointer, and sets a flag if a routine reports success.
- `update_record_mode`: The routine modifies certain bytes in a structure, copies a value from a register to a pointer, and conditionally sets a bit in a structure field based on a return value.
- `clear_linked_list`: Traverses a linked list by following pointers and calls another routine to process or clear each node, finally setting the head pointer to zero.
- `traverse_list_and_clear`: Walks a linked list using a pointer in a register, repeatedly calling a library routine with the current node and the next node offset, until the list ends.
- `prepend_to_linked_list`: The routine checks if a pointer is null, and if not, sets the next pointer of a new node to a global head pointer, copies a value into the new node, and updates the global head pointer to the new node.
- `update_list_head`: Sets a global pointer or list head to point to a new location and sets a following or adjacent memory word to a value derived from a global variable.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `replace_list_node`: The routine searches a linked list for a specific node and replaces it with the next node in the list, or updates a global head pointer if the target is the first node.
- `replace_list_head_3`: The routine traverses a linked list and, if a certain node is found, updates a pointer or a global variable with a value from the list before calling another routine.
- `replace_list_head_2`: The routine checks if a pointer is null, and if so, stores a value from a pointer into a global variable, otherwise it stores the value into a pointer.
- `replace_list_head`: Stores a value from a pointer into a global variable and then calls another routine.
- `validate_table_index`: The routine checks if an index is within the bounds of a table and if the table entry contains a non-zero pointer, then returns a boolean-like result based on the success of a recursive call.
- `access_table_with_scaled_index`: Loads a value from a table using an index multiplied by a constant, then passes that value as an argument to another routine.
- `empty_routine_3`: The routine is empty, returning zero.
- `check_result_and_set_error`: The routine calls another routine and checks if the return value is zero.
- `return_zero`: the routine returns a zero value without performing any operations.
- `handle_conditional_write_logic`: A routine containing nineteen instructions that calls another routine to check a bitmask in a structure and potentially update a global variable.
- `handle_conditional_write`: The routine checks a bitmask in a structure and, if a certain condition is met, calls another routine with three arguments and updates a global variable with a returned value.
- `write_and_handle_error`: The routine calls another routine with a pointer and two or more arguments.
- `empty_routine_4`: The routine is empty, performing no operations.
- `check_library_return_status`: Calls another routine with certain arguments and returns a value if a bit in the result is set and a global memory location contains a non-zero value.
- `execute_empty_routine`: The routine calls another routine and returns zero.
- `chain_result_to_output`: Checks a global variable against a constant, executes a conditional function using the result, and passes the returned value to a write wrapper followed by an output wrapper.
- `dispatch_on_flag_then_call`: The routine checks a global variable for non-zero, and if set calls the function pointer stored there.
- `conditional_global_dispatch`: Checks several global variables for non-zero values and, when set, calls other routines with parameters taken from those globals or performs indirect calls through pointers derived from globals.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `invalidate_list_entry`: Checks if an index or pointer is valid within a certain range and if the target entry is non-zero; if valid, it calls another routine and sets the target entry to zero, otherwise it sets a global variable to a specific error value.
- `execute_indirect_call_2`: Loads a function pointer from a global table and executes a call to that address.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `dereference_indirect_function_pointer`: Loads a function pointer from a table or structure using an offset and executes it.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_wrapper`: Calls a function pointer located at an offset from a pointer retrieved from a global variable, specifically wrapping a file open routine.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_wrapper`: Calls a function pointer located at an offset from a pointer retrieved from a global variable, specifically wrapping a file write routine.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `execute_indirect_call_7`: Loads a function pointer from a global table using an offset and executes that pointer.
- `execute_indirect_call_3`: Executes an indirect call to a function pointer located at a fixed offset from a pointer retrieved from a global variable.
- `execute_indirect_pointer`: Loads a function pointer from a nested table structure and executes it.
- `execute_indirect_function_pointer`: loads a function pointer from a nested table structure and executes it.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `dereference_nested_pointer_and_call`: Loads a function pointer from a table or structure using a base address and two offsets, then executes that function.
- `execute_nested_pointer`: Loads a function pointer from a nested offset within a table and calls it.
- `execute_indirect_function`: loads a function pointer from a nested offset within a table and calls it.
- `call_library_vector`: Loads a library base from a global variable and calls a library function at a specific offset.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: fetches a function pointer from a nested table structure and executes it.
- `execute_indirect_function_pointer_3`: Retrieves a function pointer from a nested pointer or table and executes it.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:add21k

- `clear_and_recurse_table`: Clears a block of forty-one long words starting from a pointer and then calls another routine recursively.
- `set_viewport_and_refresh`: The routine stores two integer values obtained from another routine into a global variable area, then conditionally calls other routines based on those values.
- `clear_table_and_recurse`: The routine clears a block of forty-one long words starting at an offset from a base pointer taken from the stack, then stores a pointer to a small stack buffer and a global variable into two fields of the same structure and calls another routine.
- `call_indirect_if_flag_then_recurse`: The routine checks a global variable for non-zero and, if set, calls the function pointer stored there.
- `parse_dialogue_text`: Obtains two identifiers from a global table and stores them in global variables, then branches based on their values.
- `run_loop_and_calls`: Runs a loop ten times calling a routine, then conditionally calls several other routines based on values stored in a global variable or passed as arguments.
- `initialize_game_state`: runs a short loop ten times calling another routine, then checks several global flags to conditionally execute indirect calls or calls with parameters or return values taken from global pointers or values.
- `append_string_to_buffer_2`: Finds the end of a destination string and copies a source string into it until a null terminator or a maximum length is reached.
- `copy_string_with_limit`: Finds the end of a destination string, then copies bytes from a source pointer into that destination until a null byte or a counter reaching -1 is encountered, ensuring a null terminator is written.
- `copy_string_to_buffer_2`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `calculate_string_length`: iterates through a null-terminated string to calculate the number of bytes preceding the null terminator and returns that count.
- `copy_bytes_with_padding`: Copies bytes from a source to a destination for a specific count or until a zero byte is found, then pads the remaining destination area with zero bytes.
- `validate_and_dereference_index`: Checks if an index is within a valid range and if a pointer in a table is non-null, then calls another routine using the pointer or sets an error flag.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_indirect_from_library`: Loads a function pointer from an offset within a library base and calls it.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_indirect_from_library_offset`: Loads a function pointer from a different offset within a library base and calls it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_indirect_via_table`: Loads a function pointer from a global variable table and calls another routine through it.
- `execute_indirect_table_call`: Loads a function pointer from a table at a specific offset and executes it.
- `execute_indirect_function_2`: loads a function pointer from a table and executes it.
- `execute_indirect_function_3`: Loads a function pointer from a table located via a global offset and executes it.
- `execute_indirect_call_4`: Performs an indirect call to a function pointer located at an offset from a pointer stored in a global or register-relative address.
- `execute_indirect_call_5`: Performs an indirect call to a function pointer located at a different offset from a pointer stored in a global or register-relative address.
- `execute_indirect_function_4`: Loads a function pointer from a table located at an offset from a global pointer and calls it.
- `execute_indirect_function_5`: loads a function pointer from a nested offset within a table and executes it.
- `execute_offset_table_call`: Loads a function pointer from a table using a base address and an offset and jumps to that address.
- `execute_indirect_function_6`: loads a function pointer from a table using a base address and an offset, then jumps to that address.
- `dereference_nested_pointer`: Loads a function pointer from a nested pointer or table-based lookup and calls it.
- `execute_indirect_function_7`: loads a function pointer from a table located via an offset from a global pointer and calls it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `execute_indirect_call_6`: Performs an indirect call to a function or pointer located at a different offset from a pointer stored in a global or register-relative address.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:AVENGER

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `decrement_and_branch_call`: Decrements a global variable if it meets certain criteria, then calls a function if the result of a previous library call is non-zero and updates a global variable with a constant.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger](Disk 2 of 2).adf:bootblock

- `evaluate_library_result`: Calls a function via a pointer and returns a specific constant value depending on whether the returned value is zero or non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_loop`: decrements a 16-bit register value until it reaches a certain state and returns a passed parameter.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_hardware_and_memory`: Modifies hardware registers, copies a memory block, and fills a memory block with incrementing values.
- `fill_memory_with_incrementing_values`: Writes incrementing integer values to a memory block or table and wraps the pointer if a certain limit is exceeded.
- `copy_memory_block`: Copies a block of memory from a global source to a fixed destination address using a loop.
- `copy_memory_block_2`: copies a block of memory from a source address to a destination address, ending when a certain address is reached, and then updates a global pointer to the end of the copied range.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_max_value`: Loads a specific constant value into a register and returns.
- `execute_indirect_call`: Loads a function pointer from a memory location and executes it.
