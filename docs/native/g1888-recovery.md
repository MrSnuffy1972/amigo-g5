# Game 1888 (future wars - time travellers (1989)(u. s. gold)(it)[cr aggressors]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01888). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802960 top-level bytes in 35 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 556 of 556 named (0 author symbols), 556 of 556 described (70 by rule, 56 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5273 of 5273 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18443 of 18443 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1555169 |
| code | 62200 |
| data | 144981 |
| padding | 7842 |
| unused | 32768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:run | 2 | 117 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:-AGGRESSORS- | 9 | 319 | 18 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:setpatch | 24 | 833 | 45 |
| CSI | 2 | 83 | 3 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:endcli | 2 | 117 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:DELPHINE | 3 | 16385 | 445 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:add21k | 3 | 480 | 33 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:bootblock | 1 | 92 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- future wars - time travellers (1989)(u. s. gold)(it)[cr aggressors](disk 2 of 2)/Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_header_and_call_handler`: If the output pointer is not null, copies sixteen longwords from a fixed offset in the data block into it.

### Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:-AGGRESSORS-

- `parse_startup_arguments_and_run`: Checks whether the program was started from the command line or from Workbench.
- `invoke_two_function_pointers`: Calls two library function vectors through the base register at fixed negative offsets and returns, with no other logic.
- `open_library_or_fail`: Calls a library vector to open a library and returns its base when that succeeds.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_delay`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `wait_for_alloc_vector_release`: Hooks the memory allocation vectors and repeatedly delays while checking whether they are still in use.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_wait`: Library stub: loads its arguments and the exec library base, calls Wait and returns its result to the caller.
- `call_signal`: Library stub: loads its arguments and the exec library base, calls Signal and returns its result to the caller.
- `call_set_function`: Library stub: loads its arguments and the exec library base, calls SetFunction and returns its result to the caller.
- `call_stored_handler_pointer`: Calls the routine whose address is held in a global pointer variable, through an indirect call the decompiler could not resolve to a jump table, then returns.
- `call_second_stored_handler`: Calls the routine whose address is held in a second global pointer variable, then returns.

### Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:setpatch

- `run_setpatch_entry_point`: Program entry: allocates a control block and returns error code 20 if that fails.
- `release_control_block_and_return`: Stores the given value in the control block, closes the file handle and library when they are set, frees the block, and returns the stored value.
- `call_guarded_pass_through`: Calls a routine, then if a global flag is nonzero calls a second routine with the parameter and passes its result to a third.
- `apply_setpatch_patches`: Records a flag from its parameter, checks the command-line arguments for a usage message and the RAD option, and then patches the Exec, layers.library and trackdisk vectors as the code finds them.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_stored_library_routine`: Calls the routine at offset 46 of the library base held in a global, and returns the value left in the data register.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `allocate_stub_and_checksum`: If the memory checks pass and a suitable block can be allocated, copies a short code stub from a neighbouring routine into it and stores its address in the control structure.
- `store_checksum_and_call`: Stores the complement of a 16-bit sum over a short word array in a structure field, then calls the routine through a pointer.
- `disable_interrupts_around_patch_calls`: Opens the disk resource through a library vector and, when that succeeds and a size field is small, disables interrupts by writing the interrupt-enable register and incrementing a nesting byte.
- `copy_long_on_nonzero_result`: Calls the routine stored in a global pointer and, when it returns a nonzero value, copies a longword from a fixed offset of the global structure into a global.
- `call_handler_unless_value_matches`: Returns without action when the stored longword matches the value at a fixed offset in the global structure and is not minus one.
- `call_handler_if_input_minus_one`: Returns when the data register input is not minus one.
- `install_marker_and_vector_once`: Disables interrupts through the interrupt-enable register.
- `call_vector_at_offset_136`: Calls the routine at offset 136 of the pointer held in a global, then returns.
- `fill_vector_table_from_routine`: Stores a computed offset in a global, then fills a sixteen-entry longword table at a fixed low-memory location with addresses that step through a routine, two bytes apart.
- `call_helper_repeatedly_return`: Calls a helper repeatedly in a straight line and returns the result of its last call.
- `return_input_unchanged`: The routine returns the value held in a data register without modification.
- `pass_register_value`: This routine is empty and performs no operations, returning the input data register to the caller unchanged.
- `return_input_value`: Returns the data register input unchanged and performs no other operation.
- `set_function_if_index_small`: When the word at offset 20 is 35 or less, calls the SetFunction library vector through the exec base.
- `allocate_entry_list_or_free`: Allocates a block with the memory allocator, then fills its entries from the input structure, allocating a further block for each nonzero size word.
- `execute_loop_and_modify`: Repeatedly calls a function through a pointer in a loop and returns a value with a bitwise OR applied.
- `execute_and_modify_pointer`: The routine calls a function through a pointer stored at an offset from a global variable and returns the value from a register with the highest bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine performs no observable work and immediately returns to its caller.
- `call_check_and_notify`: Passes the longword at offset 68 to the routine held in a global pointer.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker_word`: Advances through 16-bit words from the input pointer until one has the value 97 in its high byte, then returns a pointer to that word.
- `advance_by_branch_displacement`: Decodes a relative displacement from the word at the pointer.
- `compute_offset_from_word`: Reads a 16-bit value two bytes after the input pointer, halves it and sign-extends the 15-bit result.
- `compute_three_global_offsets`: Walks a chain of helper calls that advance a position by two each step, storing three returned pointers into three globals in turn.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_with_stack_slot_pointer`: Calls another routine with its two parameters and a pointer to a fixed stack slot, then returns.
- `format_into_stack_buffer_and_send`: Formats its input through the exec formatting library call into a 140-byte stack buffer.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### CSI

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:endcli

- `size_buffer_from_list_max`: Finds the maximum of the values in a linked list and uses it to size and fill a buffer.
- `copy_memory_block`: The routine copies a fixed number of words from a memory location to a destination and then calls another routine.

### Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:DELPHINE

- `clear_block_and_dispatch_result`: Calls a routine, zeroes a large block of longwords starting at a fixed offset from the base register, and stores the base pointer.
- `set_global_or_mask_input`: Returns the input byte ORed with 185 when the overflow and sign flags disagree and the carry is set.
- `set_global_bit_one`: Sets bit 1 of the global and returns.
- `build_status_bits_from_global`: Builds a two-bit status value from the global: bit 0 is set when bit 6 of the global is clear, and bit 1 is set when bit 2 of the global is clear.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `set_global_constant_79`: Stores the constant 79 in a global and returns.
- `set_global_to_marker_value`: Stores the constant 32879 in a global and returns.
- `set_global_to_eight`: Stores the constant eight into one global variable and returns.
- `update_pseudo_random_state`: Doubles a 32-bit state word held behind a stack-relative pointer, applying a conditional XOR feedback constant when the shift overflows or yields zero.
- `read_inverted_keyboard_byte`: When bit 3 of a global status byte is set, returns that byte's complement rotated right by one bit, runs a fixed countdown delay loop, and clears bit 6 of the global.
- `map_keycode_through_table`: Calls one unnamed routine to read a byte, and if that byte is nonzero with its high bit clear, scans a 49-entry word table for it and returns the matching entry from the parallel word table, or zero if nothing matches or the high bit is set.
- `initialize_table_entries`: Copies a 180-word template into a buffer addressed through a global pointer, then overwrites position fields in it: from a first parameter it writes split high and low words at offsets stepping by 8000 up to 32000, and it writes a second parameter and a global value the same way.
- `write_structure_constant_fields`: Writes a fixed set of sixteen-bit constants at fixed offsets inside a structure reached through a global pointer, including several zero words and a few distinctive values at higher offsets.
- `store_pointer_into_control_block`: Stores a 32-bit parameter at one offset of a control block reached through a global pointer, clears a neighbouring word, and writes two constant words to the block's control field before returning.
- `read_stored_long_global`: Returns the 32-bit value stored at a fixed offset from the global base, which holds a previously saved pointer.
- `store_split_word_entries`: Optionally stores its first parameter in a global when that parameter is not the sentinel value; when the second parameter is not the sentinel, it calls a routine that waits for the display beam and then writes split high and low words of the value, with 8000 steps, into a table buffer.
- `copy_word_table`: Copies sixteen words from a source buffer passed as a parameter into every second word slot of a structure reached through a global pointer, starting at a fixed offset.
- `store_blitter_parameters`: Writes eleven incoming parameter values into fixed offsets of a structure reached through a global pointer, splitting some values into high and low words, then calls a routine that waits for the blitter to finish.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_without_effect`: Contains only a return and performs no work, so it acts as an empty entry point that seventeen unnamed routines call.
- `invoke_three_routines`: Calls three routine routines in sequence: one with an address computed from the global base, one with two constant words of all ones, and one with no arguments, then returns.
- `read_dos_info_after_lock`: Takes a DOS lock on a name pointer, asks DOS for information about it into a global buffer, releases the lock, and returns the second longword of that information block.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `read_dh0_info_after_lock`: Locks the DH0: name referenced in the code, asks DOS for its information into a global buffer, unlocks it and returns the low word of the longword at offset four of that block.
- `call_four_routines_with_negation`: No decompiled pseudo-code is available.
- `tiny_neg_move`: Small routine of 2 instructions that negates d6, then copies d0 to d0.
- `dispatch_to_one_routine`: No decompiled pseudo-code is available.
- `initialize_tables_and_buffers`: Reserves several blocks of fixed sizes through a routine call with a second argument and stores their pointers in globals, clears global tables and per-entry fields, then runs passes that call routine routines on each entry.
- `return_without_effect_2`: Contains only a return and performs no work, so it acts as an empty entry point that one unnamed routine calls.
- `swap_values_and_wait_raster`: Swaps two 32-bit global values, passes the new values to routine routines, and writes one of two constant words into a structure, chosen by whether the low word of the parameter is zero.
- `wait_raster_then_store_word`: Calls a routine that busy-waits until the display beam reaches a raster line, then stores a constant word at offset 256 of a structure reached through a global pointer.
- `process_and_clear_table_entries`: Walks 255 twenty-six-byte records in a table reached through a global base.
- `allocate_first_free_table_slot`: Finds the first record in a 255-entry table whose pointer is zero and returns minus one if none is free.
- `release_table_slot`: Releases the table record selected by its parameter: when the record holds a pointer it calls a routine with that pointer and a size product, clears the record's pointer and fields, sets two words to all ones, calls a routine on the record, and decrements a global count.
- `repeat_table_entry_processing`: The routine loops a number of times given by its short parameter and invokes another routine on each iteration, performing no other work.
- `clear_two_long_globals`: Stores zero into two consecutive 32-bit global variables and returns.
- `process_list_nodes_then_call`: Walks a singly linked list whose head is held in a global, calling a routine on each node with the size constant twenty, then calls a second routine once.
- `insert_node_into_sorted_list`: Walks a linked list in order of a sixteen-bit key that each entry looks up in a table, stops before the first node whose key is not smaller than the parameter's key, allocates a twenty-byte node through a routine, copies two parameter words into it and links it in at that point.
- `remove_matching_list_node`: Searches a list held in a global for a node whose two sixteen-bit fields equal the parameter's two words, unlinks it and passes it to a routine with size twenty, then returns zero.
- `clear_two_adjacent_longs`: Writes zero to two adjacent 32-bit global variables and returns.
- `process_second_list_nodes`: Walks a second linked list headed by a different global, calling a routine on each node with the size constant twenty, then calls another routine once.
- `append_node_with_table_fields`: Allocates a twenty-byte node through a routine and appends it to the tail of the list headed by a global.
- `prepend_node_with_table_fields`: Allocates a twenty-byte node through a routine and links it at the front of a list whose head pointer is passed on the stack.
- `clear_long_set_word_flag`: Writes zero to a 32-bit global and sets a 16-bit global to all ones, then returns.
- `process_third_list_nodes`: Walks a linked list headed by a global, calling a routine on each node with the size constant 214, then calls another routine once.
- `append_node_copying_table_block`: Walks to the tail of a list headed by a global, allocates a 214-byte node through a routine and links it at the end.
- `mark_matching_node_for_removal`: Searches the list held in a global for a node whose sixteen-bit index field equals the parameter, sets that field to all ones to mark it for later removal, and returns zero.
- `remove_marked_list_nodes`: Walks the list held in a global and unlinks every node whose sixteen-bit index field is all ones, passing each removed node to a routine with size 214.
- `clear_list_head_and_word`: Stores zero into a 32-bit global that serves as a list head and sets a 16-bit global to all ones, then returns.
- `process_fourth_list_nodes`: Walks a second 214-byte list headed by a different global, calling a routine on each node with size 214, then calls another routine once.
- `append_zeroed_node_with_index`: Allocates a 214-byte node through a routine and appends it to the tail of the list headed by a global.
- `mark_second_list_node_removal`: Searches the second 214-byte list, headed by a different global, for a node whose index field equals the parameter, sets that field to all ones to mark it for removal, and returns zero.
- `remove_marked_second_list_nodes`: Walks the second 214-byte list and unlinks every node whose index field is all ones, passing each removed node to a routine with size 214.
- `dispatch_record_with_row_fields`: Reads several sixteen-bit fields from a 32-byte table row selected by its parameter, and looks up a record pointer through a second table index.
- `dispatch_with_computed_args`: Calls a routine with a record pointer taken from a stack-selected table entry, stack-passed words and a global value, with one argument computed by masking and concatenating words, then calls a routine with zero.
- `store_parameters_to_globals`: Copies the low and high sixteen-bit halves of two parameters into four globals.
- `call_routine_with_two_globals`: Loads two 32-bit global values and passes them to one routine call, then returns.
- `call_routines_with_bounds_test`: Compares three stored globals against the fixed values zero, 160 and 200.
- `call_routine_twice_with_pair`: Calls one routine twice in succession, each time with the same global longword and a stored word and stack word, once with one stored longword and once with a second stored longword.
- `return_without_work`: Contains only a return and performs no work.
- `set_row_field_by_selector`: Dispatches on a selector word across six cases, each writing the low half of a value into a different sixteen-bit field of a 32-byte table row chosen by a row index.
- `store_row_primary_word`: Stores a sixteen-bit stack value into the first word field of a 32-byte table row, with the row chosen by another stack value, then returns.
- `store_row_secondary_word`: Stores a sixteen-bit stack value into the sixth word field of a 32-byte table row, with the row chosen by another stack value, then returns.
- `store_four_row_fields`: Writes four sixteen-bit values taken from three parameters into four consecutive fields of a 32-byte table row chosen by the first parameter's low half.
- `build_four_word_bit_mask`: Computes the address of four consecutive sixteen-bit words from a stride, a row value and an offset derived from the first parameter.
- `test_table_word_bit`: Computes the address of one sixteen-bit word from a stride, a row value and an offset derived from the first parameter, then returns whether the bit selected by the low nibble of the first parameter is set in that word.
- `initialize_and_run_game_loop`: Initializes several global variables and tables, makes a series of routine calls including a raster-wait call, then loops processing input and timing until an exit flag is set and runs cleanup calls.
- `process_input_flags_and_dialog`: Reads input flags through a routine, repeatedly accumulating bits one and two, and then branches on a mode global.
- `dispatch_lookup_result_cases`: Runs routine-driven setup and then switches on a value returned by a lookup.
- `find_matching_table_entry`: Scans up to 255 twelve-byte table entries for the first non-null entry whose sixteen-bit field matches the low half of the first parameter and whose next field matches the first word of the second parameter.
- `hit_test_list_entries`: Walks a linked list of entries and returns the index of the first entry whose tabled position range covers the parameter point and whose routine check passes.
- `dispatch_list_entries_by_type`: Walks a linked list headed by a global and dispatches each entry on its type field.
- `process_type_five_entries_buffer`: Walks a linked list passed as a parameter and, for each entry of type five, calls a routine with values taken from table rows indexed by the entry.
- `process_type_five_entries_masked`: Walks a passed linked list like its sibling routine, calling a routine for each type-five entry with table-derived values and a running index, and allocating a temporary buffer when needed.
- `call_routine_for_first_list`: Walks the linked list headed by one global and calls a routine once on each node, in list order, then returns.
- `call_routine_for_second_list`: Walks the linked list headed by a second global and calls a routine once on each node, in list order, then returns.
- `interpret_byte_command_stream`: Runs a byte-coded command stream stored in a structure until a stop flag is set.
- `enter_stream_selector_call`: Entry block of a byte-coded command stream: reads a slot byte, a selector byte and a sixteen-bit operand, calls a routine with the selector, advances four bytes and then joins the shared opcode loop.
- `enter_stream_three_byte_call`: Entry block of a byte-coded command stream: reads a slot byte and two further operand bytes, calls a routine with the second byte, advances three bytes and then joins the shared opcode loop.
- `enter_stream_pair_routine_call`: Entry block of a byte-coded command stream: reads a slot byte and four sixteen-bit operands, calls a routine with the first and second operands, advances nine bytes and then joins the shared opcode loop.
- `enter_stream_pair_routine_call_2`: Entry block of a byte-coded command stream with the same layout as its sibling: reads a slot byte and four sixteen-bit operands, calls a routine with the first and second operands, advances nine bytes and then joins the shared opcode loop.
- `enter_stream_slot_assign`: Entry block of a byte-coded command stream: reads a slot byte, a mode byte and an operand.
- `enter_stream_slot_add`: Entry block of a byte-coded command stream: reads a slot byte, a mode byte and an operand, then adds to the slot either the operand word (mode zero) or the value of another slot.
- `enter_stream_slot_subtract`: Entry block of a byte-coded command stream: reads a slot byte, a mode byte and an operand, then subtracts from the slot either the operand word (mode zero) or the value of another slot.
- `enter_stream_slot_multiply`: Entry block of a byte-coded command stream: reads a slot byte, a mode byte and an operand, then multiplies the slot by either the operand word (mode zero) or the value of another slot.
- `enter_stream_slot_divide`: Entry block of a byte-coded command stream: reads a slot byte, a mode byte and an operand, then divides the slot by either the operand word (mode zero) or the value of another slot, with no check for a zero divisor.
- `divide_slot_by_stream_word`: Continuation block of a byte-coded command stream: divides the slot selected by the caller by a sixteen-bit operand read from the stream, advances past it and then joins the shared opcode loop.
- `enter_stream_routine_result`: Entry block of a byte-coded command stream: reads a slot byte, a mode byte and an operand.
- `enter_stream_table_store`: Entry block of a byte-coded command stream: reads a slot byte, a mode byte and an operand.
- `enter_stream_routine_table_result`: Entry block of a byte-coded command stream: reads a slot byte, a mode byte and an operand.
- `enter_stream_call_zero`: Entry block of a byte-coded command stream: reads one byte, calls a routine with zero, advances one byte and then joins the shared opcode loop.
- `enter_stream_call_five`: Entry block of a byte-coded command stream: reads one byte, calls a routine with the constant five, advances one byte and then joins the shared opcode loop.
- `enter_stream_set_flag`: Entry block of a byte-coded command stream: reads one byte from the stream, stores it into a 16-bit global used as a flag, advances one byte and then joins the shared opcode loop.
- `enter_stream_call_zero_2`: Entry block of a byte-coded command stream with the same layout as its sibling: reads one byte, calls a routine with zero, advances one byte and then joins the shared opcode loop.
- `enter_stream_call_three_routines`: Entry block of a byte-coded command stream: reads one byte, calls three routines with no arguments, advances one byte and then joins the shared opcode loop.
- `enter_stream_call_one`: Entry block of a byte-coded command stream: reads one byte, calls a routine with the constant one, advances one byte and then joins the shared opcode loop.
- `record_stream_position_label`: Entry block of a byte-coded command stream: reads one byte as an index and stores the stream position just after it into a per-index table in the structure, then advances one byte and joins the shared opcode loop.
- `clear_stream_position_label`: Entry block of a byte-coded command stream: reads one byte as an index, sets that index's entry in the structure's position table to minus one, advances one byte and joins the shared opcode loop.
- `jump_to_stream_label`: Entry block of a byte-coded command stream: reads one byte as an index and moves the stream position to the offset recorded for that index in the structure's position table, then joins the shared opcode loop.
- `branch_on_flag_bit_one`: Entry block of a byte-coded command stream: reads one byte as an index.
- `branch_on_either_flag_bit`: Entry block of a byte-coded command stream: reads one byte as an index.
- `branch_on_flag_bit_two`: Entry block of a byte-coded command stream: reads one byte as an index.
- `branch_if_bit_zero_set`: Entry block of a byte-coded command stream: reads one byte as an index and steps past it.
- `branch_if_bit_zero_clear`: Entry block of a byte-coded command stream: reads one byte as an index and steps past it.
- `decrement_counter_and_branch`: Entry block of a byte-coded command stream: reads a counter index and a label index.
- `call_routine_with_second_byte`: Entry block of a byte-coded command stream: reads two bytes, calls a routine with the second byte, advances two bytes and then joins the shared opcode loop.
- `enter_stream_call_no_args`: Entry block of a byte-coded command stream: reads one byte, calls a routine with no arguments, advances one byte and then joins the shared opcode loop.
- `enter_stream_call_no_args_2`: Entry block of a byte-coded command stream with the same layout as its sibling: reads one byte, calls a routine with no arguments, advances one byte and then joins the shared opcode loop.
- `copy_stream_string_and_call`: Entry block of a byte-coded command stream: calls a routine, then copies a zero-terminated string of stream bytes into a local buffer, passes the buffer to another routine, calls a further routine and steps past the terminator before joining the shared opcode loop.
- `copy_string_and_report_failure`: Entry block of a byte-coded command stream: copies a zero-terminated string of stream bytes into a local buffer and passes it to a routine.
- `copy_string_into_slot_entry`: Entry block of a byte-coded command stream: reads a slot byte, copies a zero-terminated string of stream bytes into a local buffer, passes the buffer and a twenty-byte table entry chosen by the slot to a routine, then steps past the terminator and joins the shared opcode loop.
- `load_background_or_report_failure`: Entry block of a byte-coded command stream: calls a routine, copies a zero-terminated string from the stream and passes it to a routine.
- `copy_string_flag_on_failure`: Entry block of a byte-coded command stream: calls a routine, copies a zero-terminated string from the stream and passes it to a routine.
- `load_ct_or_report_failure`: Entry block of a byte-coded command stream that finishes a string copy: it terminates a local buffer and passes it to a routine.
- `call_three_routines_then_join`: Calls three routines in sequence without reading from the stream, then joins the shared opcode loop.
- `call_one_routine_then_join`: Calls one routine without reading from the stream, then joins the shared opcode loop.
- `set_global_flag_one`: Sets a 16-bit global flag to one at a fixed offset and then joins the shared opcode loop.
- `save_position_and_stop`: Saves the current stream position into a field of the structure, sets the stop flag so the interpreter loop ends, and then returns.
- `set_mode_flag_one`: Sets a 16-bit mode global to one at a fixed offset and then joins the shared opcode loop.
- `clear_mode_flag_zero`: Sets a 16-bit mode global to zero at a fixed offset and then joins the shared opcode loop.
- `branch_routine_and_stop`: Calls one of two routines depending on whether a word in the caller's frame is zero, then sets the stop flag so the interpreter loop ends.
- `set_pending_flag_one`: Sets a 16-bit pending flag global to one at a fixed offset and then joins the shared opcode loop.
- `read_five_operand_command`: Reads two bytes and three sixteen-bit operands from the stream and passes them to routines before joining the shared opcode loop.
- `index_stream_label_positions`: Pre-scans a byte-coded command stream, stepping over each opcode's operands by a fixed length for that opcode, and fills a 50-entry word table with all ones first.
- `read_slot_and_word_operands`: Reads a slot byte, a second byte and a sixteen-bit operand from the stream into locals, advances four bytes and then joins the shared opcode loop.
- `read_three_byte_operands`: Reads three bytes from the stream into locals, advances three bytes and then joins the shared opcode loop.
- `parse_operand_stream`: Iterates through a data stream, reading a type byte and extracting a variable number of subsequent bytes or words into a local buffer.
- `read_slot_and_four_words`: Reads a slot byte and four sixteen-bit operands from the stream into locals and then joins the shared opcode loop.
- `read_slot_and_optional_word`: Reads a slot byte and a second byte from the stream.
- `read_one_byte_operand`: Reads one byte from the stream into a local, advances one byte and then joins the shared opcode loop.
- `record_stream_position_alt`: Reads one byte as an index and stores the stream position just after it into a per-index table referenced from the frame, then advances one byte and joins the shared opcode loop.
- `read_one_byte_discard`: Reads one byte from the stream into a local, advances one byte and joins the shared opcode loop.
- `read_two_byte_operands`: Reads two bytes from the stream into locals, advances two bytes and joins the shared opcode loop.
- `parse_operand_stream_2`: Reads a stream of bytes and interprets them as various operand types based on a command byte.
- `read_slot_and_word_operand`: Reads a slot byte and a sixteen-bit operand from the stream into locals, advances three bytes and joins the shared opcode loop.
- `read_byte_then_continue`: Reads one byte from the stream into a local and records the following stream position.
- `join_shared_loop_only`: Does nothing beyond joining the shared opcode loop.
- `set_stop_flag`: Sets the stop flag so the interpreter loop ends, then joins the loop.
- `parse_operand_stream_3`: Iterates through a data stream, reading an opcode byte and then extracting a variable number of operands based on that opcode.
- `read_multi_operand_stream_block`: Reads a run of operand bytes and sixteen-bit words from the stream into a local in turn, then continues with code that is truncated in the decompiled output, so the rest of the block is not described.
- `compare_word_halves`: Compares the low and high sixteen-bit halves of its 32-bit parameter and returns a byte with bit zero set when they are equal, bit one set when the high half is smaller, and bit two set when the low half is smaller.
- `clip_and_blit_four_planes`: Clips a rectangle to a 160-pixel-wide, 200-row area and shifts it by the low nibble of its position parameter.
- `return_parameter_unchanged`: Returns its register input unchanged and makes no calls.
- `clip_blit_rect_variant_one`: Clips a source rectangle to a 160-pixel-wide area and applies the same position shift as its sibling clipping routines, then combines source words into the destination in its own inner loop.
- `return_parameter_unchanged_2`: The routine receives a single value in a register and returns it unchanged.
- `clip_blit_rect_variant_two`: Clips a source rectangle to a 160-pixel-wide area and applies a position shift like its sibling clipping routines, then writes combined source words to the destination in a further loop.
- `pass_register_value_2`: Returns the value passed in a register without modification.
- `call_routine_with_fixed_block`: Calls one routine with its parameter as the first argument, zeros for several others, all-ones and the value 800 as fixed arguments, then returns.
- `call_routine_four_offsets`: Calls one routine four times with the same fixed arguments, the value 200 among them, where the parameter is offset by 8000 for each successive call up to 24000.
- `return_without_work_2`: Contains only a return and performs no work.
- `call_routine_four_planes_masked`: Turns four flag bits in its third parameter into four all-ones or zero words, calls a routine with them, then calls the same routine three more times at offsets of 8000 from the first call's base.
- `compute_plane_span_masks`: Derives a span width, a bit mask from the low nibble of the first position and a row span from two positions, selects four plane-mask words from flag bits, and passes them to routines.
- `call_routine_span_at_base`: Derives a span width and a bit mask from two positions, computes a destination address from a global base offset by 32000 plus row and column terms, and calls a routine with the address, the width and the mask.
- `call_single_routine_undecoded`: The decompiler timed out, so no pseudo-code is available.
- `process_text_entry_table`: Looks up a pointer in a six-byte-stride table by a sixteen-bit parameter.
- `measure_longest_text_line`: Scans a zero-terminated byte string, splitting it at space and vertical-bar characters, sums a per-character width taken from a table for each segment, and returns the widest segment's total.
- `clear_shifted_mask_rows`: Clears eight rows of a shifted source pattern in a destination area: for each row it combines a source word shifted by the low nibble of the position and ANDs the inverted result into a destination longword, stepping forty bytes per row.
- `decode_unavailable_routine`: The decompiler timed out, so no pseudo-code is available.
- `append_type_two_node`: Allocates a twenty-byte node through a routine, appends it to the tail of the list headed by a global, sets its type field to two and copies two parameter words into it.
- `append_type_two_stack_node`: Stack-argument version of its sibling: allocates a twenty-byte node through a routine, links it into a list passed on the stack, sets its type field to two and copies stack-passed words into it.
- `append_type_three_node`: Allocates a twenty-byte node, appends it to the tail of the list headed by one global, sets its type field to three and also pushes it onto the front of a second list whose head is another global, storing a parameter word in the node.
- `process_text_characters_skip_spaces`: Makes three routine calls with positions derived from its short parameter, then loops over the bytes of a string, skipping space characters and calling routines for each other character up to the length returned by a routine.
- `process_numbered_text_entry`: Converts a signed sixteen-bit parameter into an index into a table of 65-byte entries, then calls routines that measure and process that entry's text, updating layout values through eight routine calls.
- `layout_text_lines_by_width`: Chooses a line width of 80, 20 or 140 from a selector parameter, then walks the bytes of a text string through several routine calls, including a raster-wait routine, to lay out lines within that width.
- `clip_and_draw_screen_box`: When a flag global is clear it clips a box to a 320-pixel-wide, 200-row screen, derives its position from two parameters and runs a sequence of routine calls, including a raster wait, before returning a short result.
- `clip_span_and_draw_sequence`: Clips a vertical span so its bottom stays within 199 rows, then runs a sequence of routine calls over table globals, including a raster wait.
- `clear_entries_and_collect_flagged`: Clears the first byte of twenty 20-byte entries in a table, then scans 255 32-byte records and, for each active record whose sixteen-bit field is minus two, calls a routine with an address inside that table.
- `lookup_slot_after_routine_check`: Returns minus two when a status routine fails.
- `poll_routine_until_status_changes`: Returns minus two when a status routine fails.
- `pack_nibble_fields_scaled`: Doubles two parameter words, splits the first parameter into separate nibble fields and combines them with masks and shifts into a 32-bit result.
- `update_table_rows_raster`: Repeats sixteen times: for each of sixteen words in a table it calls a routine with that word and an all-ones value and stores the result back.
- `update_table_rows_descending`: Repeats fifteen times, counting down to one: for each of sixteen words in a table it calls a routine using the counter as a negated offset and stores the result back.
- `fill_table_range_from_routine`: For every index from the first word of its first parameter up to the second word, stores into a table the result of a routine called with two parameter words.
- `rotate_table_segment`: When its mode parameter is one, it saves the word at the high index of a table range, moves each word in the range up by one place and writes the saved word at the low index, so the top word wraps to the bottom.
- `insert_table_word_shift`: Moves the words of a table up by one place from a top index down to a stack-given index, writes a register word into that index, then calls a routine with the table.
- `rotate_table_range_forward`: When its mode field equals two, saves the first word of a table range, shifts the remaining words down by one place and writes the saved word at the end of the range, then calls a routine.
- `remove_table_word_shift_down`: Moves table words down by one place from a start index up to a stack-given end index, writes a register word into the end index, then calls a routine with the table.
- `copy_table_range_from_buffer`: Copies the words of an index range from a second table buffer into the main table, then calls a routine with the main table.
- `copy_table_tail_from_buffer`: Copies words from a second table buffer into the main table from a register-held start index up to a stack-given end index, then calls a routine with the main table.
- `load_database_records`: Clears a table of thirty-byte records and then opens and reads a database file through a DOS read wrapper, reporting an open failure with the message 'Couldn't open DATABASE %s'. The decompiled code is partial, so the remaining steps are not described.
- `clear_database_records`: Clears a table of thirty-byte records by zeroing their fields in a loop, then calls two routines.
- `copy_word_rows_between_buffers`: Computes several row pointers into two word buffers from a quotient of one parameter and a stride, then copies words between the buffers in a loop.
- `read_counted_bytes_into_buffer`: Returns minus one when a state global is negative.
- `seek_and_read_database_record`: Seeks in an open file to an offset stored in a thirty-byte database record and reads from it through DOS seek and read wrappers, with a second stored value as the length.
- `copy_bytes_advancing_source`: Copies a counted run of bytes from the source pointer held in its first parameter into a destination buffer, then stores the advanced source pointer back into the parameter.
- `allocate_frame_buffers`: Looks up a record, sets up frame buffers through routine calls and a raster wait, and reports allocation failure with the text 'ECHEC allocframe' followed by two numbers.
- `transform_nibbles_odd_mapping`: Returns zero for a zero input.
- `update_record_if_valid_wait`: Looks up a record and, when it is valid, calls routines on a table entry, compares two stored longwords and waits on the raster.
- `update_secondary_record_if_valid`: Looks up a record and, when it is valid, calls routines on a second table entry and compares two stored longwords before its remaining steps.
- `update_entry_when_globals_match`: Calls a routine with a table entry and a frame word, then when two stored longwords are equal it calls routines, including a raster wait.
- `scan_name_for_dot_extension`: Stores a frame slot value, then scans a name string up to a length returned by a routine for a period character and tests the characters after it.
- `parse_dot_n_extension`: Checks whether a period in a name string is followed by the character N and, if so, advances a frame pointer by four and calls routines.
- `read_header_fields_return_true`: Reads a two-byte field and a thirty-two-byte field into frame storage through routine calls, sets a global pointer thirty-four bytes past another global, and returns one.
- `return_false_status`: Returns zero and does nothing else.
- `read_sized_block_from_file`: Opens a file through a routine using a fixed mode argument, reads two sixteen-bit values through DOS read wrappers, reads a block whose size is their product into the caller's buffer, and closes the file.
- `depack_bitstream_block`: Unpacks a packed block from a stream using a bit reader, whose refill routine folds each longword it fetches into a checksum.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bits_from_stream_words`: Shifts a longword right by a count held in a register, pulling in a set top bit from the next earlier longword in memory whenever the register runs out.
- `process_active_entries_raster`: Walks 106-byte table entries and, for each active entry, calls routines including a raster wait while updating frame state.
- `process_record_entry_with_raster`: Looks up a record and, when it is valid, reads a longword from a table entry and calls routines to process it, including a raster wait.
- `process_twelve_byte_entries`: Walks twelve-byte table entries and, for each non-null entry, calls routines that use the entry's fields, including a raster wait.
- `process_six_byte_entries`: Walks six-byte table entries and, for each non-null entry, calls routines that use the entry's fields, including a raster wait, and clears a status word.
- `process_flagged_slots_sequence`: Checks several byte flags in a row of table slots and, for each set flag, calls routines with the slot address and fixed arguments in sequence.
- `load_auto_script_and_process`: Makes a series of routine calls, then walks twelve-byte table entries and calls routines on each non-null entry.
- `read_lvdt_directory`: Fills a 120-byte and an 80-byte stack buffer through routine calls, loads a table entry name and calls routines including a directory read.
- `read_block_return_success`: Reads a block of two hundred bytes into a table entry through a routine, calls a second routine, and returns one.
- `report_missing_archive`: Builds an 80-byte buffer from a table entry, calls routines and passes the message text 'Questo archivio non esiste...' to a routine, which reports a missing archive.
- `read_file_and_process_entries`: Makes seven calls to other routines, then walks twelve-byte table entries and calls a routine on each non-null entry, with a file read wrapper among the calls.
- `read_counted_records_from_file`: Calls a read routine in a loop up to a count, then reads a two-byte value through a DOS read wrapper and repeats the read routine for a second loop up to that value.
- `read_counted_records_from_file_2`: Follows the same pattern as its sibling: reads a counted series of records through a read routine, a two-byte value through a DOS read wrapper, and a second counted series.
- `read_counted_records_from_file_3`: Follows the same pattern as its siblings: reads a counted series of records through a read routine, a two-byte value through a DOS read wrapper, and a second counted series.
- `run_guarded_helper_sequence`: Loops over a counted range calling an unnamed routine with a value taken from a record, then makes a fixed series of calls at offsets from a base value.
- `append_record_and_read_fields`: Walks a global linked list to its last node, allocates a cleared 214-byte record, and appends it there.
- `link_and_initialize_record`: The routine traverses a linked list to find the end, inserts a new node at the head, and initializes several fields within that node using parameters and a table lookup.
- `prepend_record_to_linked_list`: Traverses a linked list to find the last element and inserts a newly allocated record at the head of the chain.
- `read_and_link_buffer_chain`: Allocates a cleared 214-byte record and links it into a chain through two pointers held in locals.
- `fill_record_slot_from_table`: Walks a global linked list of records and, for each one, stores a pointer from a table into a slot of the record.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `append_record_with_table_lookup`: Walks a global linked list to its last node and appends a newly allocated 214-byte cleared record there.
- `read_link_buffer_chain_alt`: Allocates a cleared 214-byte record and links it into a chain through two pointers held in locals.
- `fill_slot_from_small_table`: Walks a global linked list of records and, for each one, stores a pointer from a 12-byte-entry table into a slot of the record.
- `stub_return_immediately`: The routine body is a single return with no operations, so it has no behaviour beyond returning to its caller.
- `read_record_into_list`: Allocates a cleared 20-byte record and reads 20 bytes into it from a handle.
- `read_and_insert_record`: Allocates a cleared 20-byte record and reads 20 bytes into it through a read wrapper.
- `insert_buffer_into_list`: Traverses a linked list to find the end, reads data into a new buffer, and inserts this buffer into the list.
- `read_and_link_buffer_chain_small_alt`: Reads data via a wrapper, links pointers in a buffer chain, performs a read into the buffer, and links the buffer into a global list using a different global base.
- `load_slot_table_entries`: Scans a table of 26-byte slot entries and, for each slot in use, looks up a 30-byte descriptor and fetches its data with unnamed routine calls.
- `load_slot_entries_then_wait`: Performs the same slot-table traversal and descriptor loading as its neighbours, then calls a busy-wait unnamed routine that polls the video beam position before returning.
- `resume_slot_load_after_match`: Re-enters the slot-loading loop after a descriptor has been found.
- `scan_slot_table_next_entry`: Scans the slot table forward for the next entry with the same key, loading and linking each matching entry's data.
- `step_slot_table_scan`: Advances through the 26-byte slot table and resumes the match scan, fetching and linking each entry that carries the current key.
- `finish_slot_table_scan`: Continues the slot table scan from the current index, skipping unused entries and entries whose key is the terminator value.
- `draw_record_list_entries`: Walks the global record list and, for each record, passes the slot's buffer and the record's position fields to a draw unnamed routine.
- `draw_slot_entries_with_widths`: Walks a list of slot records, reading each record's slot index and position fields.
- `draw_remaining_slot_entries`: Continues the slot record walk from the current node, drawing each in-use slot with an unnamed routine call.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `prompt_insert_disk_message`: Builds a disk-insertion prompt from referenced text and a volume number held in a global, drawing the message lines with unnamed text routines.
- `wait_raster_then_halt_loop`: Calls one unnamed routine repeatedly until a local word reads zero, then enters an endless loop calling two unnamed routines.
- `setup_blitter_and_wait`: Calls a busy-wait on the video beam, then configures blit parameters from two stored buffer pointers and performs one block operation of 200 rows between the buffers' second halves.
- `draw_wrapped_text_box`: Measures the longest line of a message split by vertical bars, then sets a box width clamped between 180 and 300 on a 320-wide screen and centres it.
- `draw_wrapped_text_lines`: Renders the lines of a message glyph by glyph inside a centred box, wrapping at spaces with per-character width and bitmap-offset tables.
- `finish_text_box_render`: Continues text box rendering once the line counts are known: it computes the box width from the widest line, centres the box, and draws its lines glyph by glyph.
- `wait_and_draw_text_box`: Waits for the video beam to reach a raster line, computes the box width from a line count and centres it, then draws the text lines glyph by glyph inside a frame.
- `draw_text_lines_with_wrap`: Draws a message's lines glyph by glyph from a given offset, wrapping at spaces and advancing one line at a time.
- `draw_text_rows_and_frame`: Draws successive rows of glyphs from a message, wrapping at spaces.
- `draw_text_rows_from_index`: Draws the remaining characters of a message in rows, advancing for each space and wrapping at the end of a row.
- `draw_text_box_contents`: Draws a complete message box: it renders wrapped glyph rows from the message, centres each row inside a fixed-width frame, and draws the box outline and separator lines.
- `read_text_input_line`: Waits for the raster line, then reads key codes through a library stub into a buffer until Enter or Escape.
- `process_text_input_key`: Handles one keystroke of a text input line: backspace removes the last character, while letters, digits and spaces are stored and drawn up to 19 characters.
- `continue_text_input_loop`: Repeats the text input loop after a key has been handled, drawing the cursor and accepting letters, digits and spaces until Enter or Escape ends the line.
- `sequence_two_helper_calls`: Makes a library stub call followed by two unnamed routine calls, then returns.
- `switch_active_frame_entry`: When the requested index differs from the active one, it adds the position change between the old and new 74-byte entries to two running offsets.
- `initialize_display_structure`: Copies two saved position words into their slots, then writes two fixed control words into a structure.
- `configure_interrupts_and_globals`: Stores the caller's frame pointer in a global variable and writes a set-mask to the interrupt enable register INTENA. The mask enables the master interrupt and returns the two input registers unchanged.
- `restore_interrupts_and_clear_dma`: Restores a saved global value, writes a set-mask to INTENA that enables the master interrupt, and then clears the low four audio DMA enable bits through DMACON. It returns the two input registers unchanged.
- `software_delay_loop`: Calls an unnamed routine first when a status bit in a global is set, then counts a 16-bit register down until it wraps.
- `delay_and_return`: The routine checks a bit in a global variable to decide whether to call another routine, then performs a decrementing loop before returning.
- `update_mouse_position_from_joy0dat`: Reads the mouse counter register and turns each axis change into a signed delta with 8-bit wraparound.
- `apply_sprite_position_delta`: Applies a passed-in position delta to the running X and Y positions, clamps them to fixed limits, and writes two sprite position words into the buffer.
- `set_sprite_position_from_input`: Stores the given raw counter values as the new reference, adds a vertical delta to the running position, and clamps both axes to fixed limits.
- `setup_blitter_line_draw`: Computes the octant and slope from two endpoints and loads the blitter control, size and modulo registers for a line draw.
- `copy_length_prefixed_string`: Copies a length-prefixed string from a table entry into the destination buffer one byte at a time, then terminates the destination with a zero byte.
- `search_string_table`: Walks a linked list of table entries, copying each unflagged entry's string and comparing it with the input string.
- `find_trackdisk_device_unit`: Looks up the trackdisk.device through the exec name-search call, then reads the unit entry at the given index.
- `call_helper_if_found`: Calls a lookup unnamed routine and, when it returns a non-zero handle, passes that handle to a second unnamed routine and returns zero.
- `poll_drive_until_ready`: Polls a drive status word in a loop and counts the attempts.
- `poll_drive_status_with_retry`: Repeatedly reads a drive status longword until its flag word is zero.
- `prompt_check_drive_for_disk`: Shows the referenced message asking the user to check the drive and insert a disk, then polls the drive status until it is ready.
- `wait_for_drive_status_clear`: Polls a drive status longword until its low two bits read clear, using an unnamed routine result to choose between one and three status reads.
- `poll_drive_status_three_stages`: Runs three polling stages in turn, each reading a drive status longword until its low two bits or its status word read clear.
- `poll_drive_status_checked`: Checks a status word from an unnamed routine first, then polls the drive status longword until its low two bits clear.
- `poll_drive_status_two_stages`: Polls a drive status longword until its low two bits clear, then polls once more for a status word and returns its value.
- `return_drive_status_word`: Reads drive status longwords in a loop and returns the low status bits once a read shows them clear.
- `open_library_and_read_file`: Waits for a pending flag to clear, then opens dos.library through the exec open call.
- `examine_and_lock_file`: Allocates a file info block and locks the named file on the dos library.
- `set_global_flag`: Stores the value one into a global flag variable and returns.
- `set_flag_and_wait`: Sets a global flag to one when it is zero, then busy-waits until the flag reaches a fixed value of 64.
- `initialize_cia_resource`: Opens ciaa.resource through the exec open-resource call and, when a setup flag is set, initialises global state and a 15-entry pointer table with running values.
- `shutdown_audio_channels`: When the audio setup flag is set, it calls a library function through the resource base and calls an unnamed routine that releases the audio channels.
- `free_buffer_if_allocated`: When an allocation flag and a buffer pointer are both set, it frees the buffer with the exec free-memory call and clears the allocation flag.
- `compute_max_indexed_value`: Scans a table of bytes located at a fixed offset from a global base, keeping the largest value it sees.
- `write_cia_timer_registers`: Writes a fixed control value and then a stored value to the CIA-A timer registers, passing each through shadow globals.
- `clear_pointer_table_entries`: Writes zero to the memory location that each of fifteen pointers in a global table refers to.
- `reset_audio_and_custom_state`: Writes zero to the four audio volume registers and to a custom register, sets one global to the minus-one value, and clears several audio state globals.
- `initialize_audio_dma_channels`: Clears the four audio volume registers and a custom register, then sets each channel's length to one word and points each channel's pointer at a shared global.
- `call_audio_helper_return_zero`: Calls one unnamed routine and returns zero to its caller, adding no logic of its own.
- `process_audio_tick`: Advances a tick counter, and on each tick other than the sixth it runs a per-channel update for all four audio register blocks.
- `call_helper_pass_through`: Calls one unnamed routine and returns the value that was passed to it on the stack, unchanged.
- `update_all_audio_channels`: Calls one unnamed routine for each of the four audio channels, passing a channel number counting down from three and that channel's register block.
- `run_channel_effect_tick`: Applies one tick of a per-channel effect: when a slide value is set it adds that value to the channel's running value, clamped to a limit.
- `apply_slide_with_limit`: Adds a step value to a running value and raises the result to a stored minimum when it falls below it.
- `dispatch_channel_effect_command`: Decodes the effect command nibble of a channel step and runs the matching handler.
- `set_status_bit_one`: Sets bit one in a global status variable and returns.
- `clear_status_bit_one`: Clears bit one in a global status variable and returns.
- `load_channel_slot_from_note`: Loads a channel slot from a note record: it stores the record's note word, sets the first slot field from the low nibble of the record's last byte, and stores a table-derived value in the third field.
- `load_channel_slot_with_sign`: Loads a channel slot from a note record like its neighbour, but takes the sign from an input register and negates the low nibble when the sign is set.
- `copy_word_with_signed_offset`: Computes a word offset from an input value, doubled and negated when the sign input is zero, and copies the word at that offset from a source address into a destination slot.
- `store_word_into_slot`: Stores the input word at offset four of a record in a fixed position.
- `dispatch_effect_jump_table`: Calls one of the handler routines selected by a counter minus one from a jump table kept in globals.
- `lookup_table_entry_by_key`: Searches a table of words for the key stored in a record, then copies the entry at an index offset from the matching position into a channel slot field.
- `lookup_entry_by_high_nibble`: Searches a word table for the key stored in a record, then copies the entry at an index taken from the high nibble of the record's last byte into a channel slot field.
- `lookup_entry_by_low_nibble`: Searches a word table for the key stored in a record, then copies the entry at an index taken from the low nibble of the record's last byte into a channel slot field.
- `copy_record_word_to_slot`: Copies the word at a fixed offset of a record into a channel slot field.
- `adjust_word_by_high_nibble`: Adjusts a record word by the high nibble of its last byte, adding it when non-zero.
- `decrement_word_by_low_nibble`: Subtracts the low nibble of a record's last byte from the record word when that nibble is non-zero, and stores the result in a slot field.
- `configure_audio_dma`: Clears a counter and calls an unnamed routine four times, then writes the set-mask that enables DMA to DMACON after a short delay.
- `step_channel_volume_envelope`: Updates one channel's playback state: when a note index is set it loads the next entry from a 30-byte table, then applies a volume slide capped at 64.
- `apply_channel_volume_slide`: Applies the volume slide for one channel step: it adds an offset to the channel's position and slides volume with a cap of 64 before writing it to the output.
- `apply_volume_step_and_load`: Adds the step from the input register to the channel volume, capped at 64, and stores the result less the current level in the output.
- `start_channel_from_record`: When a record's command is not zero, it clears the channel and loads the output fields from the record.
- `store_channel_fields_and_mask`: Copies a record's word fields into the channel slot's fields and ORs the record's channel mask into a global mask.
- `call_two_unnamed_helpers`: The pseudo-code could not be produced because the decompiler timed out.
- `clear_and_initialize_sound_twice`: Clears the first word of a block and one word at an index from it, then calls an unnamed routine with the values 65535 and 63 and again with 1 and 0.
- `decrement_clear_and_set_volume`: Decrements a position word by ten, clears the first word of a block and one word at an index, and clamps a volume word to 63.
- `call_one_unnamed_helper`: The pseudo-code could not be produced because the decompiler timed out.
- `clear_and_initialize_sound_twice_2`: The routine clears a word pointed to by a pointer at an offset, clears another word computed from a pointer and a word, and calls another routine twice with parameters taken from the pointer and constant values.
- `open_cia_resource_once`: Opens ciaa.resource through the exec open-resource call once, then calls the library through the resource base and sets a setup flag.
- `close_cia_resource_if_open`: When the CIA resource flag is set, it calls a library function through the resource base, calls an unnamed routine, and clears the flag.
- `write_cia_timer_registers_2`: The routine writes three values to CIAA control and timer registers.
- `clear_audio_pointers_and_counters`: Writes zero to the four audio volume registers and points each audio channel pointer at a shared global.
- `reset_audio_and_enable_dma`: Clears the four audio volume registers and points each channel pointer at a shared global.
- `call_helper_return_register`: Calls one unnamed routine and returns the second input register unchanged.
- `advance_tick_enable_dma`: Counts audio ticks, and on each tick other than the sixth it runs a four-channel update only.
- `reset_audio_channels_and_dma`: Clears two counters and runs a four-channel update, then sets the DMA enable bit and waits briefly.
- `initialize_sound_channel_state`: Computes the record offset for a channel index using 22-byte records, zeroes a counter word and stores the supplied words into the channel's fields.
- `advance_channel_envelope_step`: Counts down a channel's delay counter and returns while it is non-zero.
- `clamp_and_copy_channel_level`: Copies the reload word into the channel counter, then adds the step to the channel level.
- `copy_channel_record_to_output`: When a channel's pointer is non-zero, it copies the channel's record fields to the output block and clears the pointer if the status word is zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_block_and_update_scroll`: Copies 32-bit words from a source into a word buffer, writing each pair to two positions, when a flag is clear.
- `initialize_dos_startup_state`: Allocates a state block from a pointer table, sets its fields and writes a four-character marker into it.
- `read_process_startup_arguments`: Reads the current process structure and its command-line field.
- `create_dos_process_from_startup`: Reads the current process's command-line state, then opens a file and duplicates a lock through dos wrappers.
- `stub_empty_decompile_output`: The decompiler produced an empty body for this 113-instruction routine, so its behaviour cannot be read from the pseudo-code.
- `copy_string_to_buffer`: Copies a zero-terminated byte string from the source into the destination one byte at a time, including the terminator, and returns the destination pointer.
- `copy_string_to_buffer_2`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks a zero-terminated byte string to its terminator and returns the number of bytes before it.
- `convert_to_uppercase`: Converts a lowercase ASCII letter to uppercase by subtracting the case difference, and returns any other byte unchanged.
- `convert_to_lowercase`: Converts an uppercase ASCII letter to lowercase by adding the case difference, and returns any other byte unchanged.
- `compare_two_byte_strings`: Compares the first zero-terminated string with the second byte by byte, up to a fixed loop limit.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `open_dos_library_or_alert`: Clears a block of global state words, then opens dos.library through the exec open call.
- `return_from_call`: The routine is empty and performs no operations before returning.
- `build_and_scan_command_buffer`: Allocates a buffer sized from a stored prefix length and copies a template string into it, followed by an argument string.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `append_string_to_buffer`: Finds the end of the destination zero-terminated string and appends the source string to it, up to a fixed loop limit.
- `append_string_with_limit`: Finds the end of the destination string and appends up to a counted number of bytes from the source, stopping at the source terminator.
- `copy_string_pad_zeros`: Copies bytes from the source up to a counted limit or its terminator, then fills the rest of the counted length with zero bytes.
- `open_icon_library_for_window`: Opens the icon library by name, looks up an entry from the given structure, and stores the entry pointer and its index in the structure.
- `terminate_string_after_helper`: Stores the first argument in a global pointer, calls an unnamed routine with the second argument, and then writes a zero byte at the stored pointer.
- `store_param_and_dispatch`: The routine stores the first parameter into a global variable, calls another routine with itself and the second parameter, then clears a byte at the address stored in the global variable and returns the result from the call.
- `store_byte_and_advance_pointer`: Stores the low byte of its argument at the address held in a global pointer, advances that pointer by one byte, and returns the byte.
- `forward_format_with_stack_args`: Forwards its argument, together with a character-output callback and a pointer to its stack arguments, to a formatting routine.
- `call_two_unnamed_routines`: The pseudo-code could not be produced because the decompiler timed out.
- `format_text_with_specifiers`: Walks a format string and passes literal characters to an output callback.
- `apply_sign_to_helper_result`: Records whether the two input operands have different signs, calls an unnamed routine on them, and negates the unnamed routine's result when the signs differ.
- `call_helper_and_pair_result`: Calls one unnamed routine and returns a 64-bit value built from the original second register and the register the unnamed routine left behind.
- `call_single_helper_and_return`: Calls one unnamed routine with no arguments and returns.
- `perform_32bit_division`: Divides a 32-bit dividend by a 32-bit divisor, using a 16-step shift-and-subtract loop when the divisor's high half is set and 16-bit division otherwise.
- `call_other_routine`: The routine is 7 instructions long and is entered because its address is taken with pcrel-pea.
- `call_three_unnamed_routines`: The pseudo-code could not be produced because the decompiler timed out.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `call_two_unnamed_helpers_2`: The pseudo-code could not be produced because the decompiler timed out.
- `store_field_byte_and_advance`: Stores a byte from a fixed offset of a structure at the address held in a pointer register, advances that pointer by one byte, and returns the byte.
- `call_helper_for_each_record`: Calls one unnamed routine for each record in a table of 22-byte entries, walking from a base address up to a fixed end address.
- `release_structure_resources`: Returns minus one for a null structure pointer.
- `release_stream_buffer`: Releases a 22-byte stream buffer's resources according to the flag bits in the structure, then zeroes its pointer, count and flag fields.
- `write_byte_to_stream_buffer`: Writes one byte into a stream buffer, first flushing it when the write pointer reaches its end.
- `put_byte_with_flush_check`: Writes one byte into a stream buffer like its neighbour, with an end-of-file check that clears the buffer state.
- `put_byte_into_stream_buffer`: Writes one byte into a stream buffer, allocating a buffer when none is set.
- `write_byte_or_set_error`: Writes one byte into a stream buffer, or through an unnamed routine for unbuffered streams.
- `start_buffer_with_byte`: Starts a stream buffer by setting its write pointer to the base and its end to the base plus the buffer size, then marks it active and writes the byte.
- `allocate_free_stream_buffer`: Searches a table of 22-byte stream buffer records for one whose flag byte is clear, zeroes its pointer, count and flag fields, and returns it.
- `find_free_buffer`: The routine searches a table of 22-byte records for an entry with a clear flag byte, zeroes its fields, and returns a pointer to it.
- `zero_three_longwords`: Writes zero to three consecutive 32-bit fields of a record pointed to by an address register, then returns.
- `initialize_stream_buffer_record`: Initialises a stream record: when a buffer allocation returns nothing it sets a one-byte size and points at the record's inline area.
- `initialize_stream_buffer_with_pointer`: Sets a 1024-byte buffer size on a stream record, marks it as allocated and stores the given buffer pointer.
- `free_linked_allocations`: Walks a global linked list of allocations, releasing each node through an unnamed routine with its recorded size plus eight bytes.
- `free_list_nodes_via_exec`: Walks a linked list of allocations and calls the exec free-memory vector on each node with its recorded size plus eight bytes.
- `allocate_linked_allocation`: Allocates a block of the requested size plus an eight-byte header through an unnamed routine, links the block at the head of a global list, and returns a pointer past the header.
- `push_allocation_node_on_list`: Links a node with a size field into the head of a global list and returns a pointer just past its eight-byte header.
- `forward_low_word_to_helper`: Passes the low word of its argument to an unnamed routine and returns, with no other operations.
- `unlink_and_free_allocation`: Searches the global allocation list for the node holding the given block, unlinks it and releases it through an unnamed routine with its recorded size plus eight bytes.
- `unlink_allocation_node_from_list`: Walks a linked allocation list to find the node that matches the given block, then unlinks it and releases it by its recorded size plus eight bytes.
- `unlink_list_node`: Unlinks a node from the global allocation list, using the predecessor register when it is set and the list head otherwise.
- `pop_list_head_node`: Sets the global allocation list head to the first node's next pointer and calls an unnamed routine, removing the head node from the list.
- `test_indexed_stream_entry`: Checks that an index is in range and that its table entry is set, recording an error code of two when the entry is missing.
- `check_handle_is_interactive`: Looks up a handle in an indexed table and returns whether the dos library reports it as interactive, as a true or false value.
- `return_zero`: Returns zero with no other logic.
- `delete_file_and_record_error`: Calls a delete-file wrapper with its argument.
- `return_zero_2`: This routine is empty and simply returns a value of zero to the caller.
- `call_two_unnamed_helpers_3`: The pseudo-code could not be produced because the decompiler timed out.
- `call_helper_with_access_check`: Checks the access bits of an indexed record and fails with error code five when none are set.
- `write_and_record_error`: Calls an unnamed routine with the record's buffer and length.
- `return_from_error_handler`: This routine is an empty function that performs no operations before returning to its caller.
- `check_ctrl_c_break_signal`: Reads the exec signal state with a zero mask and tests the Ctrl-C break bit.
- `call_helper_then_return_zero`: Calls one unnamed routine and returns zero, with no other logic of its own.
- `chain_result_to_output`: Passes the address of a global value with a length of four to an unnamed routine, then passes the returned value to the dos output-write wrapper and calls one further routine.
- `dispatch_on_flag_then_call`: Calls the function pointer stored in a global when that pointer is set, then calls one unnamed routine and returns.
- `release_program_resources`: Releases the program's resources in sequence, calling dos and library cleanup routines for each global handle that is set and clearing the pointer globals.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_indexed_stream_entry`: Checks an index against the table size and that its entry is set, recording an error code of two and returning minus one when it is not.
- `close_dos_handle`: Calls the dos library's close routine through the dispatch table at the stored dos base, passing its argument through.
- `call_close_2`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir_2`: The routine calls a function from a library to retrieve the current directory.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_3`: The routine calls a function from a library to open a file.
- `call_open_2`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_3`: The routine calls a function from a library to write data.
- `call_write_2`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_indirect_function_8`: Calls a routine by dereferencing a pointer located at an offset from a base address.
- `call_indirect_via_table`: The routine loads a function pointer from a global variable table and calls another routine through it.
- `execute_nested_function_pointer`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_exec_vector_294`: Calls the exec library function at minus 294 bytes from the stored base and returns its result.
- `execute_structure_offset_pointer`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_indirect_function`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `call_library_function_via_base`: Loads a library base pointer from the global data block and calls the library function at a fixed negative offset from it, with no arguments, then returns.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:add21k

- `init_resources_and_configure_screen`: Program start code that requests an item through a shared global, falling back to a null request when the first attempt returns zero and releasing earlier results on failure.
- `set_up_graphics_and_intuition`: Requests the same items through a shared global as the entry code does, releasing the earlier item and retrying when a lookup fails.
- `clear_long_block_store_buffer`: Clears a block of long words starting at an offset computed from a stack argument, then stores the address of a stack buffer and a stack value into the following slots.
- `call_hook_if_flag_set`: If a global function-pointer slot is nonzero, calls the function it holds.
- `split_text_into_token_array`: Looks up a named item, and if found reads its text, splits the text into tokens at separator characters found in a character-class table, and stores pointers to the tokens in an allocated array.
- `shutdown_after_ten_iterations`: Runs a fixed loop of ten calls to a routine, then calls through a global function pointer if one is set and releases two global items if they are nonzero.
- `initialize_game_state`: Runs a short loop ten times calling another routine, then checks several global flags to conditionally call other routines with various parameters.
- `append_string_limited`: Appends a zero-terminated source string to the end of a destination string, copying at most a fixed limit of about 32 kilobytes, and always ends the result with a zero byte.
- `append_string_with_count`: Appends a zero-terminated source string to the end of a destination string, copying no more than the count given as the third argument and always writing a terminating zero byte.
- `copy_zero_terminated_string`: Copies bytes from a source string to a destination buffer up to and including the zero terminator, then returns the original destination pointer.
- `measure_string_length`: Walks a zero-terminated byte string to its terminator and returns the number of bytes before it.
- `copy_string_with_zero_padding`: Copies up to a counted number of bytes from a source string, stopping early at a zero byte, then writes zero bytes to fill the rest of the counted area.
- `free_table_slot_by_index`: Checks that an index is within 0 to 19 and that its six-byte table entry is in use.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_function_via_base_2`: Loads a library base pointer from the global data block and calls the library function at a fixed negative offset from it, with no arguments, then returns.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `dereference_nested_pointer`: Loads a pointer from a memory offset, uses that value to find another pointer, and executes the function located at the resulting address.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_indirect_via_nested_pointer`: Loads a function pointer from a location within a table and calls it.
- `call_indirect_table_pointer`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `execute_indirect_function_pointer_4`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `dereference_nested_pointer_2`: Accesses a memory location via a double-indirection through a pointer stored at a calculated offset from a register.
- `dereference_nested_pointer_3`: Retrieves a function pointer by traversing two levels of indirection through offsets from a base address stored in a register.
- `call_indirect_function`: Calls a function through a pointer retrieved from a table using an offset from a base address.
- `call_indirect_function_2`: Executes a routine by dereferencing a pointer located at a specific offset from a base address.
- `call_indirect_function_3`: Performs an indirect call to a routine using a pointer found within a table.
- `execute_indirect_callback`: Performs a double-dereference to fetch a function pointer from a table and executes it.
- `call_indirect_function_4`: Loads a function pointer from a table and jumps to that address.
- `call_indirect_function_5`: Uses a base address and an offset to call a function through a pointer.
- `call_indirect_function_6`: Retrieves a function pointer from a table and executes it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_function_7`: Calls a routine via an indirect pointer stored at an offset from a base address.

### Future Wars - Time Travellers (1989)(U. S. Gold)(It)[cr Aggressors](Disk 1 of 2).adf:bootblock

- `decompress_bit_packed_boot_payload`: Boot block decompressor that reads control bits from a packed bit buffer.
- `shift_bit_buffer_with_sentinel`: Reads the 32-bit word stored four bytes before the input pointer, shifts it right by one bit and sets the top bit as a sentinel.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_loop`: Decrements a value in a register within a loop until it reaches a specific state.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_endless_trap_loop`: Repeatedly executes a trap instruction in an endless loop and never returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_immediate_return`: Does nothing and returns immediately; it has no body beyond the return.
- `call_pointer_at_fixed_address`: Calls through a function pointer stored at a fixed memory location, with no arguments, and returns.
