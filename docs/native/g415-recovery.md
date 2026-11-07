# Game 415 (Future Wars - Time Travellers): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00415). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802896 top-level bytes in 30 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 527 of 527 named (0 author symbols), 527 of 527 described (59 by rule, 466 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5282 of 5282 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 17766 of 17766 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1551730 |
| code | 59996 |
| data | 137493 |
| padding | 7597 |
| unused | 46080 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 1 of 2).adf:setpatch | 24 | 833 | 45 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 1 of 2).adf:endcli | 2 | 117 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 1 of 2).adf:delphine | 3 | 16318 | 442 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 1 of 2).adf:add21k | 3 | 480 | 33 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |

## Cases handled

- 6 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- future wars - time travellers (1989)(u. s. gold)[cr](disk 2 of 2)/Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 1 of 2).adf:setpatch

- `initialize_setpatch`: The routine obtains a pointer from a global variable, calls another routine through it and stores the returned pointer in a local structure.
- `restore_patch_state`: The routine saves a pointer from a global variable into a saved slot and then invokes a series of cleanup callbacks stored in a global table.
- `self_calling_guard`: The routine calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `install_setpatch_handler`: The routine installs a SetPatch handler and performs a series of conditional patches and library initialisations.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_via_table`: The routine calls another routine through a pointer stored in a global variable.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `patch_memory_block`: The routine checks fields of a global structure and, if conditions are met, obtains a memory block and a code pointer via library calls, copies a small code stub from another routine into the target, updates a pointer in the global structure, computes a checksum over a short array in the structure and stores its complement, then returns.
- `sum_words_and_jump`: The routine sums a series of words from a structure pointed to by a register, stores the bitwise complement of the sum into a field of the same structure, and then performs an indirect jump through a table.
- `disable_interrupts_and_patch`: The routine checks a global variable and calls another routine through a table.
- `check_flag_and_copy_word`: The routine calls another routine and tests its return value.
- `conditional_indirect_call_2`: The routine checks a global variable against a value from memory and a constant, returns early if the condition is met, otherwise performs an indirect call through a pointer stored in a global variable.
- `check_and_dispatch_indirect`: Checks if a register value is not equal to negative one, returning early if true.
- `enable_interrupts_and_dispatch`: The routine stores a constant into a global variable, initializes two memory locations from registers and a global pointer, and then performs an indirect call through a pointer derived from a global variable with a fixed offset.
- `call_indirect_routine`: The routine performs an indirect call through a pointer stored in a global variable, adding a fixed offset to the base address before invoking the target code.
- `initialize_patch_table`: The routine writes a global variable with a computed offset and then fills a 16-entry table with successive addresses of another routine, incrementing the pointer each iteration.
- `recursive_self_call`: The routine repeatedly calls itself many times and returns the value produced by the final self call.
- `return_input_value_2`: The routine contains a long sequence of self calls and finally returns the value obtained from a call to another routine, effectively passing through the input value unchanged.
- `chain_calls_and_return`: The routine repeatedly calls another routine a fixed number of times and returns the result of the last call.
- `return_input_value`: The routine receives a value in D0 and returns it unchanged.
- `set_patch_handler`: The routine checks a global variable and returns early if a condition is met, otherwise it calls another routine via a function pointer stored in a global table.
- `copy_resource_list_entries`: The routine reads a word from the argument block, obtains a resource handle via an indirect library call, and if the handle is valid copies a fixed number of words from the argument block into the resource structure using an indirect library call in a loop.
- `indirect_call_and_mask_return`: The routine performs an indirect call through a pointer computed from a register with a fixed offset, then returns the value from a register with a mask applied.
- `call_indirect_and_mask_return`: The routine performs an indirect call through a pointer computed from a register with a fixed offset, then returns the value from a register pointer with the high bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `conditional_cleanup_call`: The routine calls another routine with a value taken from a structure, and if the result is non-zero it calls two more routines using a value from the same structure and returns true.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: The routine receives a pointer to a word array and advances it one element at a time until the high byte of the current word equals the marker value, then returns the pointer to that element.
- `compute_word_offset_2`: The routine reads a 16-bit value from two bytes after the input pointer, sign-extends it if the high bit is set, and returns the input pointer advanced by the computed offset plus one.
- `compute_word_offset`: The routine reads a 16-bit value from two bytes after the input pointer, sign-extends it if the high bit is set, and returns the input pointer plus twice that signed value plus two.
- `patch_global_pointers`: The routine sequentially calls a routine and stores the returned pointers into a series of global variables.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot`: The routine calls another routine passing two parameters and a pointer to a certain location on the stack.
- `compute_checksum_and_call`: The routine calls a function pointer taken from a global variable, then builds a 32-bit rolling checksum over a 140-byte stack buffer until a null byte is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 1 of 2).adf:endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_patch_table`: The routine is two instructions long and is address taken.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 1 of 2).adf:delphine

- `clear_memory_and_initialize_state`: The routine clears a block of memory pointed to by a global variable, stores a constant marker in a global structure, copies a global pointer into a global variable and then conditionally calls another routine via a function pointer stored in memory.
- `write_interrupt_control_enable_2`: The routine writes the value 32879 to a global variable overlapping INTENA, which is used to control interrupts.
- `set_global_flag`: The routine sets bit 1 of a global variable by OR-ing the value 2 into it.
- `read_input_flags`: The routine returns a byte derived from a global variable, testing bits 64 and 4 and combining the results with a logical OR. It references the CIAA_PRA and POTGOR hardware registers.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `write_interrupt_control`: The routine writes the value 79 to a global variable that overlaps the INTENA register.
- `write_interrupt_control_enable`: The routine writes the value 32879 to a global variable overlapping INTENA, which is used to control interrupts.
- `write_interrupt_control_disable`: The routine writes the value 8 to a global variable overlapping INTENA, which controls interrupt enable/disable.
- `update_pseudo_random_state`: The routine updates a 32-bit value stored through a pointer derived from the stack frame, applying a multiply-by-two with a conditional XOR, and returns a mixed result based on the parameter and the updated value.
- `handle_keyboard_serial`: The routine returns a byte computed from the bitwise complement of a global variable when a flag bit is set, performs a decrementing delay loop, and clears a flag bit in the global variable.
- `lookup_table_search`: The routine obtains a value from another routine and, if the value is non-zero and its high bit is clear, searches a table derived from a register for a matching entry and returns an associated short value, otherwise returns zero.
- `initialize_table_entries`: The routine copies 180 bytes from a table located relative to the stack frame into a buffer pointed to by a global variable, then writes several derived 16-bit high/low parts of parameters and global values into fixed offsets of that buffer in a loop.
- `write_memory_constants`: The routine writes a series of 16-bit constants to multiple offsets of a structure pointed to by a global variable, initializing fields at various positions.
- `store_blitter_setup`: The routine writes several fields through a pointer derived from a global variable, storing a constant word, a parameter word, a zero word and another constant word into consecutive offsets of the pointed-to structure, then returns.
- `read_global_pointer`: The routine returns the 32-bit value stored at an offset from a global base, effectively exposing a global pointer.
- `set_raster_wait_values`: The routine optionally stores a parameter into a global variable and, when a second parameter is supplied, writes derived raster timing words into a structure pointed to by another global variable, then calls a raster wait helper.
- `copy_word_table`: The routine copies sixteen words from a source buffer passed as a parameter into a fixed offset area of a structure pointed to by a global variable.
- `configure_blitter_parameters`: The routine stores multiple parameters into various offsets of a structure referenced via a global variable and then invokes a helper routine, with no explicit blitter wait in the shown code.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_routine`: The routine contains three instructions and performs no observable work; it ends immediately without accessing memory, registers, or calling another routine.
- `invoke_helper_routines`: The routine calls three other routines with specific arguments, including one with a computed address and one with constant parameters, and then returns.
- `lock_and_read_info`: The routine obtains a lock via a DOS wrapper, then uses info and unlock wrappers to retrieve a value from a structure referenced by a global variable, returning that value or zero on failure.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `lock_and_read_volume`: The routine attempts to lock a volume named DH0 via DOS wrappers, reads a short value from a structure referenced by a global variable, unlocks the volume, and returns the value or zero on failure.
- `volume_lock_and_string_search`: The routine is 43 instructions long and is called by three unnamed routines.
- `tiny_neg_move`: Small routine of 2 instructions that negates d6, then copies d0 to d0.
- `read_input_flags_2`: The routine returns a byte derived from a global variable, testing bits 64 and 4 and combining the results with a logical OR. It is called by ten unnamed routines and is referenced via a handler pointer in data.
- `initialize_game_state_3_2`: The routine clears several global variables and tables, sets up raster timing values, and then enters a loop that repeatedly calls other routines.
- `empty_routine_2`: The routine contains three instructions and performs no observable work; it is effectively empty.
- `swap_and_raster_wait`: The routine swaps two values stored in a global area, calls set_raster_wait_values, wait_for_raster_line, copy_word_table, and process_word_table_reverse, then writes a word to a structure pointed to by a global variable based on a parameter and conditionally calls another routine.
- `set_raster_constant`: The routine calls a helper routine and then stores a constant word into a fixed offset of a structure referenced via a global variable.
- `clear_table_entries_loop`: The routine iterates over 255 entries in a table referenced via a global variable.
- `find_free_table_slot_and_store`: The routine scans a global table for the first zero entry and, if found, allocates a new entry via an indirect dispatch, writes several fields derived from the parameter into the slot, increments a global counter, and returns an index built from the slot offset.
- `process_table_entry_by_index`: The routine processes a single table entry indexed by a parameter.
- `repeat_table_entry_processing`: The routine loops a number of times given by its short parameter and invokes another routine on each iteration, performing no other work.
- `clear_two_global_words`: The routine stores zero into two 32-bit locations referenced through a global base pointer.
- `walk_linked_list_and_clear`: The routine walks a linked list whose head is stored in a global variable.
- `insert_sorted_list_node`: The routine walks a linked list stored in a global variable to find the insertion point based on a short value from a table, allocates a new node via an indirect dispatch, links it into the list and copies fields from the parameter into the node.
- `remove_list_node_by_keys`: The routine searches a linked list referenced via a global variable for a node matching two short keys.
- `clear_two_globals`: The routine stores zero into two consecutive global variables.
- `walk_list_and_clear_globals`: The routine walks a linked list whose head is stored in a global variable.
- `prepend_node_to_list_2`: The routine walks to the tail of a list referenced on the stack, allocates a new node via an indirect dispatch, links it at the head of the list, and copies several words from a table indexed by a parameter into the node before updating a global list head.
- `prepend_node_to_list`: The routine allocates a new node via an indirect dispatch, swaps pointers to prepend it to a list referenced on the stack, copies several words from a table indexed by a short on the stack into the node, and updates a global list head pointer.
- `clear_global_word_and_flag`: The routine stores zero into a 32-bit location and stores a 16-bit all-ones value into another location referenced through a global base pointer, then returns.
- `walk_list_and_clear_flag`: The routine walks a linked list whose head is stored in a global variable.
- `append_node_to_tail`: The routine finds the tail of a linked list in a global variable, allocates a new node via an indirect dispatch, links it at the end, copies a block of words from a table indexed by the parameter into the node, clears another block of words, and stores the parameter index in the node.
- `find_and_mark_list_entry`: The routine walks a linked list whose head is in a global variable, comparing a short field at a fixed offset to a parameter.
- `remove_dead_nodes_from_list`: The routine walks a linked list referenced via a global variable and removes nodes whose 16-bit field equals -1 by updating the predecessor pointer and calling another routine indirectly on the removed node.
- `clear_two_globals_2`: The routine writes zero to a global variable and writes a word with all bits set to another global variable.
- `walk_list_and_clear_two_globals`: The routine walks a linked list whose head is stored in a global variable.
- `parse_tile_descriptor_2_2`: The routine walks to the tail of a list referenced on the stack, allocates a new node via an indirect dispatch, links it at the head of the list, clears two 50-word tables inside the node, stores an index and a parameter, and then calls another routine to parse a descriptor stream.
- `remove_entry_from_linked_list`: The routine walks a linked list stored in a global variable and searches for a node whose short field at offset 53 matches a value from the argument.
- `self_referential_table_chase`: The routine walks a singly linked list whose head pointer is taken from a global variable offset.
- `process_message_chain`: The routine reads several fields from a per-message structure indexed by a parameter and from a global table indexed by a short field, then conditionally calls another routine with arguments built from those fields, including a masked and duplicated word, and finally calls the same routine with a zero argument.
- `dispatch_with_computed_args`: The routine calls another routine with several arguments computed from global variables and stack offsets, including masked and concatenated values, then calls another routine with a constant argument and returns.
- `store_parameters_to_globals`: The routine copies several 16-bit parts of two parameters into distinct global locations and computes two additional 16-bit values from the parameter parts, storing the results in globals, then returns.
- `configure_blitter_once`: The routine loads two pointers from global variables and calls a blitter configuration routine once with those arguments, then returns.
- `conditional_blit_pair`: The routine reads three shorts from global variables and, depending on their values, either calls a blitter configuration routine twice with fixed arguments or calls it twice with arguments built from the loaded shorts and global pointers.
- `call_other_routine_twice`: The routine calls another routine twice with arguments loaded from a global variable and a register.
- `empty_routine_3`: The routine contains three instructions and performs no observable work; it is empty.
- `store_word_by_index`: The routine selects one of several offsets from a global base using a field of the first parameter and stores a 16-bit value from the second parameter at the indexed location.
- `copy_word_to_indexed_slot`: The routine copies a 16-bit value from a register-derived address to a location computed from a global base and a 16-bit index taken from a register, scaled by a constant.
- `copy_word_via_index`: The routine reads a 16-bit index from a register, scales it, adds it to a global base, and copies a 16-bit value from a source address to the computed destination, then returns.
- `store_table_entries_and_cleanup`: The routine stores four short values from three parameters into a table indexed by a short key, then calls another routine to obtain a short value and conditionally calls another routine once.
- `compute_bitmask_from_table`: The routine computes an address from its parameters, reads four consecutive 16-bit words together with a high word from a register, tests four bit positions derived from the first parameter, builds a 16-bit mask from the results, and returns it combined with an input register.
- `test_bit_in_table`: The routine computes an index from the low word of the first parameter, a second parameter and the high word of the third parameter, then reads a 16-bit value from a table at that index.
- `initialize_game_state_3`: The routine clears several global variables and tables, sets up raster timing values, and then enters a loop that repeatedly calls other routines, waits for a raster line condition, and processes input flags until a termination flag is set.
- `process_input_flags_and_dialog`: The routine reads input flags from a global variable and accumulates button bits, then conditionally calls helper routines to set a raster constant, draw a text block, and manage a linked list of dialog entries.
- `handle_input_and_dialog`: The routine checks a global flag and, if clear, runs initialization steps, reads input flags, sets a raster constant, draws a text block, and conditionally resets audio.
- `find_entry_in_table`: The routine scans a table stored via a global variable for up to 255 entries, each 12 bytes.
- `search_linked_list_entries`: The routine walks a linked list pointed to by a global variable and examines entries that meet size and flag conditions, loading several indexed words from a global table.
- `process_display_list`: The routine walks a linked list referenced via a global variable and dispatches each entry based on a type field.
- `process_list_and_rotate_buffer`: The routine iterates a linked list, and for each entry with a specific short value it allocates a temporary buffer once via an indirect dispatch, calls a word buffer rotation routine with parameters derived from tables and the list entry, then either copies and updates scroll state using the temporary buffer or directly, and finally frees the temporary buffer.
- `process_list_and_mask_buffer`: The routine iterates a linked list, and for each entry with a specific short value it allocates a temporary buffer once via an indirect dispatch, calls a word block masking routine with parameters derived from tables and the list entry, then either copies and updates scroll state using the temporary buffer with masked control words or directly, and finally frees the temporary buffer.
- `traverse_linked_list`: The routine loads a pointer from a global variable and walks a singly linked list by repeatedly reading the next pointer from the current node, calling another routine with the current node pointer, and advancing to the next node until the list ends.
- `traverse_linked_list_2`: The routine loads a pointer from a different global variable and walks a singly linked list by repeatedly reading the next pointer from the current node, calling another routine with the current node pointer, and advancing to the next node until the list ends.
- `parse_script_bytecode_2`: The routine reads a byte stream from a buffer pointed to by a register and dispatches each opcode to a handler, updating a set of temporary words on the stack and a global variable.
- `parse_script_bytecode_3`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_4`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_37`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_38`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_5`: The routine reads bytes from a source buffer and writes 16-bit values into temporary words on the stack.
- `parse_script_bytecode_6`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_7`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_8`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `decode_script_bytecode_2`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_9`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_10`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_11`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_12`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_13`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_14`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_15`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_16`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_39`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_17`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_18`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `decode_script_bytecode`: The routine reads a byte from a source buffer and writes a 16-bit value into a table, then loops reading an opcode byte and, based on its value, loads immediate bytes from the source buffer into temporary words on the stack and dispatches to another routine or helper functions, advancing the source pointer accordingly.
- `parse_script_bytecode_19`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_20`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_21`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_22`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_23`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_24`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_25`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `process_script_bytecode`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_26`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_27`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_43`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack, then dispatches each opcode to a handler via indirect calls through global tables.
- `parse_animation_script`: The routine reads a byte stream from a source buffer and writes 16-bit values into a set of temporary words on the stack, then dispatches each opcode to a handler that prepares blitter parameters and raster wait values or stores a table entry.
- `parse_script_bytecode_28`: The routine reads bytes from a source buffer and writes 16-bit values into temporary words on the stack, then dispatches based on an opcode byte.
- `parse_script_bytecode_41`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_42`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_40`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_29`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_30`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_31`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_32`: The routine reads bytes from a source buffer and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_33`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_34`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_35`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode_36`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_script_bytecode`: The routine reads bytes from a source buffer pointed to by a register and writes 16-bit values into a set of temporary words on the stack.
- `parse_tile_descriptor_2`: The routine clears a 50-word table pointed to by a pointer and then walks a byte descriptor stream pointed to by another pointer, advancing an index according to the descriptor byte value and stopping when a terminator is seen or a limit is reached.
- `parse_tile_descriptor_3`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_4`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_5`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_6`: Reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_7`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_8`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_9`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_10`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_11`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_12`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_13`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_14`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_15`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_16`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor_17`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer.
- `parse_tile_descriptor`: The routine reads a byte descriptor from a byte stream and writes a variable number of 16-bit words into a structure pointed to by a pointer, using the descriptor value to select how many words to copy and in which byte order.
- `compare_word_fields`: The routine compares the low and high bytes of a 32-bit parameter and builds a byte result with bits set for equality and for less-than / greater-than relations between those bytes.
- `rotate_and_mask_word_block`: The routine processes a block of 16-bit words pointed to by the first argument, using the second argument as a word count and the third argument as a base offset.
- `return_argument`: The routine is referenced by address and returns its 64-bit argument unchanged.
- `rotate_word_buffer`: The routine receives a pointer, a word count, a byte offset and a control word and rotates the contents of a 16-bit word buffer by a variable amount while updating a global flag and two global words.
- `return_parameter_unchanged`: The routine receives a single value in a register and returns it unchanged.
- `apply_bitmask_to_planes`: The routine receives a pointer to a word buffer and several parameters and uses them to compute offsets and a bit mask derived from the buffer words.
- `return_parameter_unchanged_2`: The routine receives a single value in a register and returns it unchanged without performing any operations on it.
- `configure_blit_single`: The routine calls configure_blitter_parameters once with fixed arguments derived from its parameters.
- `configure_blit_quad`: The routine calls configure_blitter_parameters four times with the same base parameters and offsets added to the second argument.
- `empty_routine_4`: The routine contains no instructions and performs no operations; it immediately returns to its caller.
- `setup_bitplane_blits`: The routine obtains a global pointer, derives four flag words from a parameter, and calls setup_bitplane_blit four times with offsets added to the base pointer.
- `configure_blitter_76_2`: The routine computes a width and height from two 32-bit parameters, obtains a base offset via a global pointer helper, and builds four blitter parameter blocks with different source offsets.
- `configure_blit_region`: The routine computes a size and a base address from its parameters using a global variable and then calls configure_blitter_parameters once with derived values.
- `configure_blitter_76`: The routine has 76 instructions, is entered from outside static flow, is called by several unnamed routines, and calls configure_blitter_parameters.
- `render_text_line`: The routine processes a text line referenced via a global variable, computes a layout based on a width parameter and a global table, and invokes another routine to draw each character using a global font table and blitter setup calls.
- `mask_table_words`: The routine iterates eight times over two pointers derived from the parameters, reads 16-bit words from one pointer, rotates them according to a nibble from the third parameter, combines them with the inverse and stores the result masked with a 32-bit word from the other pointer.
- `unknown_routine`: The routine has 81 instructions, is called and executed in a reference run, and is entered from outside static flow.
- `insert_linked_list_node`: The routine walks a singly linked list stored through a global variable until it reaches the last node, obtains a new node via another routine and links it after the last node, then writes several word fields from three parameters into the new node and updates a global pointer to the new node.
- `swap_linked_list_node`: The routine obtains a new node via another routine and stores it through a pointer derived from a stack variable.
- `insert_node_at_list_tail`: The routine walks a linked list starting from a global variable until it reaches the last node, obtains a new node via another routine, links the new node after the last node, copies a word from the parameter into the new node and sets another word field to a constant, then updates a global variable pointer with the new node address.
- `process_input_string`: The routine calls several helper routines with values derived from parameters and a global base, then iterates over a byte string pointed to by a parameter, looks up each byte in a global table and conditionally calls a buffer rotation routine with computed addresses and constants, and finally makes two more helper calls before returning.
- `render_text_line_2`: The routine computes a line width from a string length, centers the line vertically, and then walks the characters of a text string, skipping leading spaces, and for each printable character looks up a glyph table entry via a global table and issues two blit operations for the glyph using global pointers and a base offset.
- `render_text_block`: The routine measures the width of a text string using a character width table and a global variable, computes a height and vertical offset, then repeatedly calls other routines to set up blitter parameters and draw glyphs line by line, advancing a horizontal cursor and wrapping lines until a terminator is reached.
- `draw_text_block`: The routine renders a block of text by iterating over rows and characters, using a global variable to decide whether to draw, and calls other routines to set up blitter parameters and raster waits while positioning each glyph based on a lookup table.
- `render_text_block_2`: The routine prepares blitter parameters for a block of text and then issues a series of blitter configuration calls that set up copies and masks for each character in the block.
- `build_string_index_table`: The routine clears a fixed-size area in a global structure, scans a table of strings for entries with a specific marker, copies matching strings into the cleared area using copy_string_to_buffer_2, and stores indices into another global area.
- `lookup_indexed_word_pair`: The routine calls another routine to obtain a short value and returns a sentinel if the result is zero.
- `iterate_stack_until_nonzero`: The routine calls another routine and returns a sentinel if the result is zero.
- `scale_and_clamp_fields`: The routine doubles several fields of the two 32-bit parameters and then computes three derived values from the first parameter using nibble extraction and clamping logic with the second parameter, updating intermediate variables accordingly.
- `process_word_table_loop`: The routine loops sixteen times, each iteration processing sixteen words from a global area through scale_and_clamp_fields, then calls copy_word_table and two empty helpers, and finally calls two routines with values from a global structure.
- `process_word_table_reverse`: The routine loops sixteen times in reverse order, each iteration processing sixteen words from a global area through scale_and_clamp_fields with varying arguments, then calls copy_word_table and two helpers.
- `fill_word_table_range`: The routine iterates over a range defined by its first parameter and writes the result of scale_and_clamp_fields into a global word table at computed offsets.
- `shift_words_in_table`: The routine examines a flag in the second parameter and, for one flag value, saves a word from a table in a global area, shifts words down within that area using a loop bounded by values from the first parameter, and stores the saved word at a position derived from the first parameter.
- `shift_words_down_and_store`: The routine shifts words down within a global area using a loop bound by a value from a second global structure, then stores a word from a register into the table and calls another routine with the table base.
- `shift_and_copy_table_entries`: The routine checks a flag in a global structure and either shifts words up within a global area using a loop bound by values from the structure and stores a saved word, or copies a range of words from one global area to another using bounds from the structure.
- `shift_words_up_and_store`: The routine shifts words up within a global area using a loop bound by a value from a second global structure, then stores a word from a register into the table and calls another routine with the table base.
- `copy_word_range_between_tables`: The routine copies a range of words from one global area to another using bounds from a second global structure and then calls another routine with the destination base.
- `copy_word_table_range`: The routine copies a variable number of words from one global area to another using a loop bound by a value from a second global structure, then calls copy_word_table.
- `reset_disk_prompt_state`: The routine clears a 255-entry table of 30-byte records by zeroing selected bytes and words, resets two short counters, and then conditionally invokes other routines based on a global flag.
- `clear_struct_array_entries`: The routine clears several fields in each of 255 entries of a table referenced through a global variable and then, if a global pointer is non-negative, calls another routine twice with arguments derived from that pointer and a global variable and finally writes a constant to the global pointer.
- `copy_words_to_four_pointers`: The routine copies a block of words from a source pointer to four destination pointers that are spaced apart in the destination buffer.
- `find_table_entry_by_string`: The routine returns -1 when a global pointer is negative.
- `apply_table_offsets_to_buffer`: The routine calls another routine twice to set up a table lookup.
- `copy_bytes_and_advance_source`: The routine copies a specified number of bytes from a source buffer whose address is stored in a global variable to a destination buffer, then updates the stored source pointer to the next byte after the copied data.
- `process_table_entry`: The routine looks up a table entry by an index returned from another routine and, if found, prepares blitter parameters and raster wait values, swaps a few global variables, and then iterates over a list of slots to allocate them, writing offsets and indices into a table.
- `expand_nibble_mask`: The routine returns zero when the input is zero, otherwise it expands each nibble of the input into a three-bit pattern and packs the results back into a 16-bit value.
- `process_disk_prompt_2`: The routine calls another routine with the input parameter and, if the result is not -1, it performs a series of indirect calls using stack buffers and constants, repeats a small loop that reads words from another routine into a buffer, and returns a flag.
- `parse_dot_name_and_update_state`: The routine looks up a table entry by string, and if found it may swap three values in a global variable and update related fields when a condition on two of those values holds, then calls another routine with derived parameters.
- `swap_and_update_table_entry`: The routine swaps three values stored in a global variable and updates related fields when a condition on two of those values holds, then copies values from a global variable into another structure and conditionally calls another routine based on a comparison of two table entries.
- `parse_dot_n_and_update_offsets`: The routine copies a value from a global location, obtains a length from another routine, scans a string for a dot and checks the following byte for N. If found it updates a global offset, calls a routine to copy data, and updates another global pointer; otherwise it returns zero.
- `check_dot_n_and_copy`: The routine checks a byte at an offset for a dot and the next byte for N, then conditionally copies bytes from a source referenced by a global variable to destinations and updates global offsets, returning a flag.
- `copy_buffers_and_update_offset`: The routine copies a small number of bytes from a source buffer referenced by a global variable to a stack location and copies a larger block to another stack location, then adds a constant to a global integer and returns a constant.
- `return_zero_immediately`: The routine is empty except for returning a constant zero to its caller.
- `read_data_via_indirect_calls`: The routine obtains a handle via an indirect call, reads two short values from it using indirect calls, computes a size from them, performs an indirect read into a buffer using that size, and then performs a final indirect call on the handle.
- `decompress_bitstream_to_buffer`: The routine reads a packed bitstream using a bit reader and writes decoded bytes into a destination buffer pointed to by a base address plus an offset.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bits_across_words`: The routine shifts a 32-bit value right by one bit repeatedly for a given count, refilling from the next word in a pointer when the value becomes zero.
- `process_pending_tile_requests`: The routine clears pending tile request entries, then processes a tile request parameter by looking up a table entry via a global pointer and invoking several helper routines to configure blitting, raster timing and sprite offsets.
- `process_table_entry_2`: The routine looks up a table entry by a string parameter and, if found, prepares blitter parameters and raster timing values, then updates global offsets and dispatches indirect calls to configure hardware registers.
- `process_table_entries`: The routine clears a fixed-size table of pending entries, looks up an entry by an index parameter in a table referenced via a global variable, and if found updates related fields using several helper routines and a bitstream decompressor.
- `clear_pending_requests_and_process_entry`: The routine clears a pending request table and then looks up an entry by a parameter in a table referenced via a global variable.
- `process_multiple_string_entries`: The routine checks four character locations derived from a register, and for each non-zero entry it calls several other routines with offsets computed from the register, passing different constants.
- `clear_game_state_tables`: The routine performs a series of clean-up operations on data referenced through a global base pointer.
- `initialize_level_directory`: The routine copies data into stack buffers using several helper calls, references the text LVDT.DIR, and then either loops ten times calling a routine with an indexed offset or processes a single entry based on a returned count.
- `initialize_game_state_2`: The routine calls another routine with arguments loaded from a global variable and a constant, then calls it again with a value from the global variable, then calls it with no arguments, and returns a constant.
- `clear_game_state_2`: The routine performs a series of clean-up operations on several tables and linked structures referenced via a global variable, zeroing entries and invoking another routine for each non-zero element.
- `clear_game_state`: The routine performs a series of clean-up operations on several tables and linked structures referenced via a global variable, zeroing entries and invoking another routine for each non-zero item.
- `reset_and_process_disk_prompt_table_2`: The routine repeatedly invokes another routine with a value taken from a global variable and a counter stored in a global variable, resetting the counter after each pass.
- `reset_and_process_disk_prompt_table`: The routine repeatedly invokes another routine with a value taken from a global variable and a counter stored in a global variable, resetting the counter between passes, then calls another routine with the same arguments.
- `clear_disk_prompt_table`: The routine clears a 255-entry table of 30-byte records by zeroing selected bytes and words, resets two short counters, and then conditionally processes a small buffer chain using another routine.
- `process_disk_prompt_state`: The routine clears a 255-entry table of 30-byte records by zeroing selected bytes and words, resets two short counters, and then conditionally calls helpers to process disk prompts, parse names, handle tile requests, update node tables, manage buffers, process dialogue entries and update sprite offsets.
- `initialize_node_and_fill_fields`: The routine walks a linked list held in a global variable to its last node, allocates a new node via another routine, links it, and fills several fields of the new node by calling another routine multiple times and copying a value from a table.
- `follow_global_linked_list`: The routine walks a global linked list of pointers until the last element, then obtains a buffer from another routine, links the buffer into the list, and copies several fixed offsets from the buffer into the list node using another routine.
- `link_and_fill_buffer_chain`: The routine walks a linked list of buffers stored in a global variable to its tail, allocates a new buffer via a wrapper, links it into the chain, performs several reads into fixed offsets of the buffer, and stores a pointer from a global table indexed by a short value from the buffer.
- `read_and_link_buffer_chain`: The routine reads data via a wrapper, links pointers in a buffer chain, performs several reads into offsets of the buffer, and stores a pointer from a global table indexed by a short into the buffer.
- `update_node_table_field`: The routine walks a linked list held in a global variable and writes a value from a table into a field of each node based on an index stored in the node.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `link_and_fill_buffer_chain_alt`: The routine walks a linked list of buffers stored in a different global variable to its tail, allocates a new buffer via a wrapper, links it into the chain, performs several reads into fixed offsets of the buffer, and stores a pointer from a different global table indexed by a short value from the buffer.
- `read_and_link_buffer_chain_alt`: The routine reads data via a wrapper, links pointers in a buffer chain, performs several reads into offsets of the buffer, and stores a pointer from a different global table indexed by a short into the buffer.
- `update_linked_list_values`: Iterates through a linked list of structures pointed to by a global variable.
- `empty_routine_5`: The routine contains no instructions and returns immediately.
- `link_small_buffer_chain`: The routine walks a linked list of buffers stored in a global variable to its tail, allocates a new small buffer via a wrapper, reads data into it, links it into the chain, and updates a global pointer with the new buffer address.
- `read_and_link_buffer_chain_small`: The routine reads data via a wrapper, links pointers in a buffer chain, performs a read into the buffer, and links the buffer into a global list using a global variable as a base.
- `link_small_buffer_chain_alt`: The routine walks a linked list of buffers stored in a different global variable to its tail, allocates a new small buffer via a wrapper, reads data into it, links it into the chain, and updates a different global pointer with the new buffer address.
- `read_and_link_buffer_chain_small_alt`: The routine reads data via a wrapper, links pointers in a buffer chain, performs a read into the buffer, and links the buffer into a global list using a different global base.
- `process_dialogue_entries`: The routine walks a 255-entry table of 30-byte records referenced through a global variable, skips entries whose first word is zero or whose short field is -1, and for each remaining entry it loads related data from another table via an index, updates counters and fields, performs conditional swaps of two global pointers and calls other routines to handle dialogue display and buffer copying.
- `process_dialogue_entry_2_2`: The routine prepares a dialogue table lookup by copying a source buffer into a temporary area and applying table offsets, then walks a table.
- `process_dialogue_entry_2`: The routine prepares a dialogue table lookup by copying a source buffer into a temporary area and applying table offsets, then walks a table of dialogue entries indexed by a counter, compares a stored short with a value from a stack frame and updates a pointer.
- `process_dialogue_entry`: The routine walks a table of dialogue entries indexed by a counter, compares a stored short with a value from a stack frame, and updates a pointer in the table with the result of another routine.
- `process_dialogue_table_entry`: The routine walks a table of dialogue entries indexed by a counter, compares a stored short with a value from a stack frame, and updates a pointer in the stack frame.
- `reset_disk_prompt_state_2`: The routine clears a 255-entry table of 30-byte records by zeroing selected bytes and words, resets two short counters, and then conditionally processes each record in the table.
- `process_linked_list_entries_3`: The routine walks a linked list pointed to by a global variable, reads fields from each node and from a table indexed by a node field, and calls another routine with those values.
- `process_linked_list_entries_2`: The routine walks a linked list pointed to by a stack variable, copying fields from each node into local stack slots and using a table index to load values from a global table.
- `process_linked_list_entries`: The routine walks a linked list pointed to by a stack variable, copying fields from each node into local stack slots and using a table indexed by a node field to load values and decide whether to call another routine with those values.
- `return_immediately`: The routine is empty.
- `process_disk_prompt`: The routine stores a parameter into a global variable and then performs a series of indirect calls using stack buffers and constants, repeatedly invoking another routine until a returned short becomes zero, then returns.
- `loop_other_routine`: The routine repeatedly calls another routine until a stack word becomes zero, then calls the same routine twice in an infinite loop.
- `setup_blitter_and_wait`: The routine calls several helpers to wait for a raster line, set raster wait values, configure blitter parameters and perform a single blit, using values read from a global structure.
- `render_centered_dialogue_box`: The routine measures the longest line in a null-terminated string that uses pipe as a line separator, clamps the resulting width to a minimum and maximum, computes a vertically centered box height and stores layout fields in a state structure.
- `draw_text_line`: The routine processes a string pointed to by a global variable and renders it using another routine.
- `center_dialogue_box`: The routine clamps a width field in a state structure, computes a centered box height from that width, stores layout fields in the structure, and then iterates over a text buffer to advance a pointer past leading spaces, measure the string, and render each character using a font table while updating cursor positions and line wrapping.
- `draw_dialogue_box`: The routine computes a centered box size from a width parameter, clamps it, stores layout fields in a state structure, and then iterates over a text buffer pointed to by the state.
- `process_dialogue_text_2`: The routine walks through a byte stream pointed to by a state field, updating counters and a length field in the state structure.
- `process_dialogue_text`: The routine initializes two fields in a state structure to constant values and then repeatedly calls another routine with values derived from the state and a global variable.
- `iterate_dialogue_table`: The routine walks an indexed byte table and writes each byte into a word field of a state structure.
- `process_dialogue_state`: The routine iterates over a table of bytes using an index, writes the byte to a word field in a state structure and, when non-zero, invokes another routine four times with parameters derived from two global pointers and the table entry, then updates a counter.
- `handle_text_input`: The routine reads characters from input, stores them into a buffer pointed to by a global variable while maintaining a position counter, updates on-screen sprite offsets for each character, and handles backspace and termination keys.
- `process_text_input`: The routine reads bytes from an input source and stores them into a buffer pointed to by a global variable, updating a position counter and a terminator byte.
- `process_input_buffer`: The routine repeatedly reads a byte from input and writes it into a buffer pointed to by a global variable, updating a position counter and a display offset each iteration.
- `update_sprite_and_call`: The routine calls another routine to update sprite offsets and then calls a library vector stub, returning immediately.
- `update_sprite_offsets`: The routine updates two short offsets and copies a block of words from a table indexed by a parameter into a buffer pointed to by a global variable, then clears two words in that buffer and updates a short flag.
- `initialize_display_structure`: The routine copies values from a global area into a structure pointed to by a global variable, writes constant words into the structure, fills a repeated pair of words from a global table, and clears two words and a global variable.
- `configure_interrupts_and_globals`: The routine stores a frame pointer into a global variable, writes several constants to global variables and returns the concatenation of two input registers, while touching the INTENA hardware register.
- `set_global_constants`: Stores a constant value into a global variable and writes to DMACON and INTENA hardware registers.
- `return_parameter`: Conditionally calls another routine based on a global variable flag and then loops a fixed number of times.
- `wait_interrupt_ack`: The routine conditionally calls another routine based on a global variable flag and then decrements a register until it reaches a sentinel value before returning the second argument.
- `update_mouse_position`: Reads the mouse port hardware register and updates several offsets stored via a frame pointer.
- `setup_bitplane_blit`: Computes blitter parameters from two points and writes them to multiple custom chip registers.
- `copy_cstring`: Copies a null-terminated byte string from a source pointer to a destination pointer, byte by byte, and terminates the destination with a zero byte.
- `search_string_table`: The routine walks a pointer table obtained from a global variable, copies a candidate string via a copy routine and compares it with the input string via a compare routine, setting a flag when a match is found and returning a combined result.
- `find_trackdisk_device`: Calls the exec library to locate the trackdisk.device and extracts a value from a device structure, returning it combined with an input register.
- `conditional_indirect_call`: The routine calls another routine and returns a constant if the result is zero, otherwise it performs two indirect calls via global tables and returns zero.
- `poll_device_and_update_state_2`: The routine repeatedly calls another routine, stores its result through a pointer, updates a short field based on a flag byte and a counter, and conditionally invokes other routines after a count threshold.
- `poll_device_and_update_state`: The routine repeatedly calls another routine, stores its result through a pointer, updates a short field based on a flag byte and a counter, and triggers two further calls when a counter reaches a threshold.
- `wait_for_disk_insert`: The routine displays the text Please insert a formatted diskette and then calls helpers to wait for raster lines, render a text block, loop another routine, set up the blitter and decode a stream word.
- `decode_stream_word_2`: The routine repeatedly calls another routine to obtain a value, stores it in a stack slot and updates a short on the stack from the high word.
- `decode_stream_word_loop_2_2`: The routine repeatedly calls another routine to obtain a value, stores it in a stack slot and updates a short on the stack from the high word of the value when the low two bits are zero, otherwise sets the short to one.
- `decode_stream_word_loop_2`: The routine repeatedly calls another routine to obtain a value, stores it in a stack slot and updates a short on the stack from the high word of that value.
- `decode_stream_word_loop`: The routine extracts a word from a frame pointer, repeatedly calls another routine to decode a stream word and updates a stack slot until a condition on the low bits is met, then returns a derived value.
- `decode_stream_word`: The routine extracts a word from a value referenced through a frame pointer, stores it to a stack slot and repeatedly calls another routine while updating the value and a short counter until a termination condition is met.
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
- `increment_counter_and_dispatch_audio_2`: The routine increments a global counter and, unless it reaches a specific value, loops four times calling another routine with a decreasing argument.
- `increment_counter_and_dispatch_audio`: The routine increments a global counter and, unless it reaches a specific value, loops four times calling another routine with a decreasing short and an address that steps by sixteen.
- `call_audio_dma_config`: The routine is three instructions long, is referenced by data and called by one unnamed routine.
- `write_audio_channel_pointers`: The routine loops over four iterations, calling another routine with a decreasing short and an address that steps by sixteen each time.
- `process_input_command`: The routine reads a nibble from a byte pointed to by a frame pointer and dispatches on its value.
- `clamp_and_store_value`: The routine adds an input word to a short stored via a frame pointer, clamps the result to a maximum short also stored via the frame pointer, stores the clamped value to another frame pointer location and updates the original short.
- `dispatch_input_command`: The routine reads a nibble from a byte pointed to by a frame pointer and branches on its value.
- `set_ciaa_pra_bit`: The routine reads a global variable, sets bit one in it, and writes the result back to the global variable that aliases CIAA_PRA.
- `clear_ciaa_pra_bit`: The routine reads a global variable, clears bit one in it, and writes the result back to the global variable that aliases CIAA_PRA.
- `lookup_table_index`: The routine initializes two words via a frame pointer, searches a global word table for a match with a value from another pointer, and stores a word from a computed offset into a stack slot, returning early if a sentinel is found.
- `copy_word_with_signed_offset`: The routine computes a signed word offset from an input value, selects the sign based on a flag, and copies a word from a source address offset by that amount to a destination address derived from another register.
- `store_word_to_stack_slot`: The routine stores the incoming word from D0 into a fixed offset from a register, effectively writing to a stack slot.
- `lookup_table_index_2`: The routine initializes words via a frame pointer, searches a global word table for a match with a value from another pointer and stores a result word from the table.
- `dispatch_via_jump_table`: The routine indexes a table of code pointers using a global variable adjusted by a constant, then calls the selected routine via an indirect jump.
- `fetch_table_entry`: The routine scans a global word table for a match with a word from an argument, then copies a word from the table at an index given by an input value to a stack slot.
- `fetch_table_entry_high_nibble`: The routine scans a global word table for a match with a word from an argument and copies a word from the table at an index derived from the high nibble of a byte.
- `fetch_table_entry_low_nibble`: The routine scans a global word table for a match with a word from an argument and copies a word from the table at an index derived from the low nibble of a byte.
- `copy_word_from_table`: Copies a word from a table pointed to by an argument to an offset in another argument.
- `adjust_short_by_nibble`: The routine reads the fourth byte of a pointer, extracts the high nibble and, if non-zero, adds it to the pointed short and stores the result to a word at an offset in another pointer.
- `decrement_short_by_nibble`: Masks the fourth byte of a short pointer with 15 and, if non-zero, subtracts that nibble from the pointed short and stores the result to an offset in another argument.
- `configure_audio_dma`: The routine clears a global variable, calls another routine four times, writes a constant to a global variable, delays with a counter, then copies four words from a global structure to the custom chip registers AUD0LCH, AUD3LCH and DMACON. It also updates a counter and wraps it against a limit.
- `update_table_entry_2`: The routine updates a table entry pointed to by a pointer in memory.
- `update_table_entry`: The routine computes a bounded offset from a byte and a register, stores it, and conditionally copies words from a table pointed to by an argument.
- `copy_table_to_structure_2`: The routine computes a bounded offset from a byte and a register, stores it, and conditionally copies words from a table pointed to by an argument into a structure and updates a global flag with an OR operation.
- `copy_table_to_structure`: The routine checks a word pointed to by a pointer and, if non-zero, clears a word, copies a word from the pointer to an array element, stores a word from the pointer to a global variable, performs a delay loop, conditionally writes a word, otherwise copies several words from the pointer to a structure, and ORs a word from the pointer into a global variable.
- `store_fields_and_or_flag`: Copies several words from a table pointed to by an argument into a structure pointed to by another argument and ORs a word from the table into a global variable.
- `handle_sound_interrupt`: The routine is nine instructions long and is referenced as a handler pointer in data and as a pointer in hunk zero data.
- `clear_sound_state_and_init`: The routine clears a word through a pointer and clears another word computed from a pointer and an offset, then calls another routine twice with parameters derived from the pointer.
- `decrement_and_initialize_sound`: The routine decrements a word at an offset in a pointer, clears a word pointed to by a pointer at an offset, clears another word computed from a pointer and a word, clamps a word at an offset to a maximum, and calls another routine with parameters taken from the pointer.
- `invoke_clear_and_init_sound`: The routine has seven instructions and is referenced by data and a hunk pointer.
- `clear_and_initialize_sound_twice`: The routine clears a word pointed to by a pointer at an offset, clears another word computed from a pointer and a word, and calls another routine twice with parameters taken from the pointer and constant values.
- `open_cia_resource_and_init_audio`: The routine references the text ciaa.resource and calls exec.OpenResource and a library at offset minus six.
- `conditional_library_reset_audio`: The routine checks a global word and, if non-zero, calls an indirect library function via a global pointer, calls another routine, and clears the global word.
- `write_cia_timer_registers_2`: Writes values to CIAA control and timer registers.
- `clear_audio_pointers_and_counters`: Clears four audio channel length and volume registers and four pointer variables and zeroes four RAM counters.
- `reset_audio_and_enable_dma`: Clears audio channel registers and pointers, zeroes RAM counters, writes a value to a global variable and touches DMACON to enable DMA channels.
- `reset_audio_channels`: The routine is five instructions long and is referenced as a handler pointer in data and as a pointer in hunk one data.
- `reset_audio_channels_and_delay`: The routine touches the AUD0LCH hardware register and manipulates a global counter.
- `reset_audio_channels_and_dma`: The routine clears two global words, calls another routine three times, writes a value to a global variable, performs a delay loop, and then iterates over audio channel registers, conditionally writing pointers and values to them.
- `initialize_sound_channel_state`: Computes an offset from a parameter, zeroes a word in RAM, stores several words from parameters into RAM at that offset and writes a parameter to RAM.
- `decrement_counter_and_clamp_value`: The routine checks a word at an offset in a pointer and, if non-zero, decrements it and returns.
- `clamp_and_copy_short`: Copies a word within a structure, adds two words, clamps the sum to 0..63 and stores the clamped value back into the structure and another argument.
- `conditional_copy_and_clear_flag`: If a word in a table is non-zero, copies several words from the table to a structure, conditionally clears a word in the table and ORs a word from the table into a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_update_scroll_state`: The routine copies a block of 32-bit words from a source pointer into a destination word buffer when a flag is clear, then updates several global variables that hold scroll-related state, including a base offset, a pointer to a word table, a pointer to a word buffer and a signed index.
- `initialize_window_state`: The routine initializes fields in a structure pointed to by a global variable, stores a constant marker, and conditionally calls another routine with parameters derived from a global variable.
- `compare_and_build_table`: The routine compares a string from a structure with a global string and, on a match, builds a table of pointers from a linked list and stores its length.
- `compare_strings_and_build_table`: The routine compares a string from a table entry with a string from a global variable and returns a difference value.
- `empty_routine_8`: The routine contains 113 instructions but the decompiler produced an empty stub, so no observable behavior can be described from the available pseudo-code.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `copy_string_to_buffer_2`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator, and returns the original destination pointer.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `convert_to_uppercase`: The routine checks if the input byte is a lowercase ASCII letter and subtracts a constant to convert it to uppercase if true.
- `convert_to_lowercase`: The routine checks if the input character is an uppercase ASCII letter and adds a constant to convert it to lowercase.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte and returns a non-zero value when a difference is found, zero when they match up to the terminator, with a limited comparison length.
- `copy_bytes_with_count`: The routine copies a number of bytes from one memory area to another.
- `initialize_window_state_2`: The routine clears a block of memory pointed to by a global variable, stores a constant marker in a global structure, copies a global pointer into the structure, and conditionally calls another routine based on a flag in a global variable.
- `empty_routine_6`: The routine contains no instructions that perform work and returns immediately.
- `parse_script_string`: The routine parses a script string stored in memory, copies it into a newly allocated buffer, tokenizes the buffer into words handling quoted strings and whitespace, and stores the resulting token pointers in another allocated table.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit_3`: The routine finds the end of the destination string and copies bytes from the source pointer into the destination, stopping at a null terminator or after a fixed count, and writes a terminating null if the copy was truncated.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `copy_bytes_with_terminator`: The routine copies bytes from a source pointer to a destination pointer, stopping early if a zero byte is encountered or a byte count expires, then writes zero bytes to fill any remaining space in the count.
- `open_window_with_icon_library`: The routine references the text WINDOW and icon.library and calls several library vector stubs and wrappers for opening and indirect dispatch.
- `store_param_and_call`: Stores the first parameter into a global variable and then calls another routine with the first parameter as the first argument, the second parameter as the second argument and a stack address as the third argument.
- `store_param_and_dispatch`: The routine stores the first parameter into a global variable, calls another routine with the routine itself and the second parameter, then clears a byte at the address stored in the global variable and returns the result from the call.
- `increment_and_store_byte`: Loads a pointer from a global variable, increments a counter stored in a global variable and stores the low byte of the parameter through that pointer.
- `self_calling_stub`: The routine is 8 instructions long and is entered because it is called and its address is stored as a handler pointer in data.
- `call_and_return_concatenated_2`: The routine has forty-eight instructions and is called by one unnamed routine.
- `parse_format_string`: The routine walks a format string and for each conversion specifier it parses flags, width, precision and length modifiers, then reads the corresponding argument from a pointer and writes a formatted representation into a stack buffer using another routine.
- `sign_combine_and_negate`: The routine reads two signed inputs, flips a sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: Computes the quotient and remainder of a 32-bit division using the values in registers D0 and D1.
- `call_other_routine`: The routine is 7 instructions long and is entered because its address is taken with pcrel-pea.
- `callers_and_callees`: The routine is 21 instructions long and is called by empty_routine_9 and one unnamed routine.
- `empty_routine_9`: The routine is empty and returns immediately without performing any operation.
- `store_byte_from_offset_2`: The routine is 15 instructions long and is called by store_byte_from_offset and by one unnamed routine.
- `store_byte_from_offset`: The routine reads a byte from a fixed offset in a register and stores it through a pointer taken from a global variable, then returns that byte.
- `iterate_range_and_process`: The routine computes a start offset from a register and repeatedly calls another routine with successive offsets spaced by a fixed amount until a limit is reached.
- `process_message_flags_2`: The routine takes a pointer argument, checks a flag byte in the pointed structure and conditionally calls another routine, combines results into a word, clears fields of the structure, and returns a word.
- `process_message_flags`: The routine checks a flag byte in a structure, conditionally calls other routines, combines results into a word, clears fields of the structure, and returns the combined word.
- `process_message_flags_3`: The routine examines a structure pointed to by a parameter and a flag word, conditionally calls another routine with derived offsets, updates fields of the structure and a byte stored through a pointer, and returns a status byte or a sentinel value.
- `reset_structure_flags_and_return_byte`: The routine checks fields of a structure pointed to by a parameter, conditionally calls another routine and updates fields, and stores a byte from a global variable through a pointer.
- `update_structure_and_return_byte_2`: The routine checks fields of a structure pointed to by a parameter, conditionally calls another routine and updates fields, stores a byte from a register through a pointer, and returns a word derived from the structure.
- `update_structure_and_return_byte`: The routine checks fields of a structure pointed to by a parameter, conditionally calls another routine and updates fields, stores a byte from a fixed offset through a pointer, and returns a value derived from that byte.
- `update_pointer_and_store_byte`: The routine updates fields in a structure pointed to by a global variable, stores a byte from a fixed offset in a register through a pointer, and returns a masked combination of the updated value and the stored byte.
- `scan_list_and_clear_words`: The routine is unreached and consists of fourteen instructions.
- `scan_list_and_clear`: The routine scans a list using a pointer in A2, clearing three words when a null terminator is found, and returns the pointer or null if the scan ends.
- `clear_three_words`: The routine clears three consecutive words at the address in A2 and returns.
- `set_flags_and_validate_2`: The routine calls another routine and, depending on its result, writes constants to fields of a structure pointed to by the parameter, sets bits in a flag byte, copies a value from another register, and conditionally sets an additional flag.
- `set_flags_and_validate`: The routine writes constants to fields of a structure referenced by a register, sets bits in a flag byte, copies a value from another register, calls a validation routine and sets an additional flag bit if the call returns non-zero.
- `traverse_list_and_clear_2`: The routine walks a linked list using a pointer in A4, repeatedly calling a library vector routine with the current node and the next node offset, until the list ends, then clears the head pointer and returns zero.
- `traverse_list_and_clear`: Walks a linked list using a pointer in A2, repeatedly calling a library vector routine with the current node and the next node offset, until the list ends.
- `store_pointer_and_traverse_list`: The routine stores a pointer into a global slot and calls a routine that traverses a linked list and clears nodes.
- `store_pointer_and_return`: The routine copies a value from a global variable into a pointer, stores a second value from another global variable into the next word of that pointer, writes the pointer back to a global variable and returns the pointer advanced by two words.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_node_from_list`: The routine walks a linked list obtained from a global variable to find a node matching a computed address, unlinks the node by updating the previous pointer, calls another routine with the node and a derived value, and returns a constant.
- `traverse_list_and_store`: The routine walks a linked list using a pointer from a global variable and stores the current node value into a global slot or another pointer.
- `store_value_and_call`: The routine has four instructions and its address is taken.
- `store_and_call_library`: The routine stores a value from a register into a global variable, calls another routine and then returns zero.
- `validate_input_and_check`: The routine checks a parameter against limits from a global variable and a table.
- `check_input_validity`: The routine loads a value from a global variable and a table, passes it to another routine, and returns true if the result is non-zero.
- `return_zero`: The routine contains no instructions that modify data and returns zero immediately.
- `check_result_and_store_flag`: The routine calls another routine with the input argument and tests its return value.
- `empty_stub_routine_2`: The routine contains no instructions and performs no operations.
- `dispatch_conditional_store`: The routine is nineteen instructions long, is address taken and called by conditional_dispatch_and_store, empty_routine_10 and three unnamed routines.
- `conditional_dispatch_and_store`: The routine tests a flag in a structure and either writes a constant to a global variable and returns a negative value or calls another routine with several arguments.
- `check_file_status_and_error`: Checks a condition in a global structure and, if met, calls a routine to retrieve an I/O error code, storing the result in a global variable before returning an error status.
- `empty_routine_10`: The routine contains no instructions that modify data and returns without performing any operation.
- `check_library_vector`: The routine calls another routine with fixed arguments, tests a bit in the result, returns the result if a global short is zero, otherwise calls another routine and returns zero.
- `invoke_and_return_zero`: The routine calls another routine with no arguments and then returns a constant zero.
- `call_chain_with_result`: The routine calls another routine with a global variable and a constant, stores the returned value, then calls another routine with that value, calls another routine with no arguments, and returns.
- `dispatch_on_flag_then_call`: The routine checks a global variable for non-zero, and if set calls the function pointer stored there.
- `conditional_global_dispatch`: The routine checks several global variables for non-zero values and, when set, calls other routines with parameters taken from those globals or performs indirect calls through pointers derived from globals.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_handle_conditional`: Indexes into a table using a global base and the parameter, checks bounds and a flag, and if valid and a flag is clear it calls another routine with the handle and clears the entry.
- `call_indirect_via_table_2`: The routine loads a pointer from a global variable table and calls another routine through it.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `unknown_indirect_jumptable_call`: The routine loads an address from an offset in a global variable, subtracts a constant from it, and calls through a double indirection to another routine.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_indirect_via_table_3`: The routine loads a pointer from a global variable table and calls the routine it points to.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_indirect_via_table_4`: The routine loads a pointer from a global variable, adds an offset, and calls the routine found there via an indirect jump.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `indirect_library_dispatch`: The routine performs an indirect call through a pointer derived from a global variable with a fixed offset, using an Amiga OS library offset, and then returns.
- `indirect_library_call`: Performs an indirect call through a pointer derived from a global variable with an Amiga OS library offset.
- `indirect_table_dispatch`: The routine loads a pointer from a global variable, adds a constant offset, and calls the routine found there indirectly, then returns.
- `indirect_call_via_table`: The routine loads a pointer from a global variable, adds a constant offset, and calls the resulting address indirectly.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_indirect_via_table_5`: The routine loads a pointer from a global variable, adds a constant offset, and calls the routine found at that address.
- `call_indirect_via_global_table_5`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the routine found there.
- `call_indirect_via_global_table`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the routine found there.
- `call_indirect_table_entry`: The routine loads a function pointer from a table referenced via a global variable and calls it indirectly.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_indirect_through_table`: The routine loads a pointer from a global variable, adds an offset, and calls the routine found there indirectly.
- `call_indirect_via_table_6`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the routine pointed to by the resulting address.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 1 of 2).adf:add21k

- `program_entry_initialize`: The routine is the program entry point.
- `set_viewport_and_refresh`: The routine stores two integer values obtained from another routine into a global variable area, then conditionally calls other routines based on those values.
- `clear_table_and_recurse`: The routine clears a block of forty-one long words starting at an offset from a base pointer taken from the stack, then stores a pointer to a small stack buffer and a global variable into two fields of the same structure and calls another routine.
- `call_indirect_if_flag_then_recurse`: The routine checks a global variable for non-zero and, if set, calls the function pointer stored there.
- `parse_dialogue_text`: The routine obtains two identifiers from a global table and stores them in global variables, then branches based on their values.
- `call_handlers_and_loop`: The routine loops ten times calling another routine, then conditionally calls handlers stored in global variables and calls another routine with values read from global variables.
- `initialize_game_state`: The routine runs a short loop ten times calling another routine.
- `find_end_copy_string_limited`: The routine finds the end of the destination string and copies bytes from the source pointer into the destination buffer one by one, stopping after a limited number of bytes or on a null terminator, then writes a terminating zero.
- `copy_string_with_limit_2`: The routine finds the end of the destination string, then copies bytes from the source pointer into the destination buffer one by one, stopping at the first null byte or when a counter in the third argument reaches -1, and finally ensures the destination is null terminated.
- `copy_string_until_zero`: It copies bytes from the second pointer to the first pointer, advancing both pointers until a zero byte is read and stored, including the terminator.
- `compute_string_length_2`: Iterates through a zero-terminated byte string starting from the input pointer until the null terminator is found.
- `copy_bytes_with_padding`: The routine copies bytes from a source pointer to a destination pointer for a limited count derived from a 32-bit parameter, stopping early if a zero byte is encountered, then writes zero bytes to the remainder of the destination area for the same count.
- `validate_and_clear_entry`: The routine indexes a table using the input value and checks bounds and a zero entry, setting a global word to a constant and returning a negative value on failure.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_via_base`: The routine loads a function pointer from a global variable and calls an Amiga OS library function at a fixed offset from that base.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_indirect_via_table_7`: The routine calls another routine indirectly via a table.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_indirect_via_table_8`: The routine loads a function pointer from a global variable table and calls another routine through it.
- `indirect_call_via_table_2`: The routine performs an indirect call through a pointer obtained from a global variable table.
- `call_indirect_via_table_9`: The routine calls another routine indirectly through a pointer obtained from a global variable table.
- `call_indirect_via_global_table_2`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the routine found there via an indirect jump.
- `call_indirect_via_global_table_3`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the routine found there indirectly.
- `call_indirect_via_table_10`: The routine loads a pointer from a global variable, adds an offset, and calls the routine found there via an indirect jump.
- `call_indirect_via_table_11`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the routine found there via an indirect call.
- `indirectly_call_via_table`: The routine loads a pointer from a global variable via an offset and calls another routine through an indirect jump.
- `indirect_call_via_table_3`: The routine performs an indirect call through a pointer obtained from a table.
- `call_indirect_via_table_12`: The routine calls another routine indirectly through a table.
- `call_indirect_via_table_13`: The routine loads a pointer from a global variable, adds a constant offset, and calls the routine found at the resulting address.
- `invoke_indirect_through_table`: The routine loads a pointer from a global variable, adds a constant offset, and calls the routine found at the resulting address via an indirect call.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_via_global_table_4`: The routine loads a function pointer from a global table via an offset from a4 and calls it indirectly.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_register_loop_3`: The routine loads an undefined short from D2, repeatedly decrements it until it reaches -1, then returns the value passed in the second argument unchanged.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_7`: The routine contains no instructions and returns immediately.
- `indirectly_call_via_table_2`: The routine performs an indirect call to another routine via a pointer.
