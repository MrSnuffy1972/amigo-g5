# Game 2159 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02159). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 635 of 635 named (0 author symbols), 635 of 635 described (69 by rule, 217 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 240 of 240 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16522 of 16535 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 71290 |
| code | 52254 |
| data | 482627 |
| padding | 14997 |
| unused | 280064 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Keymap | 1 | 3 | 1 |
| Run | 2 | 458 | 12 |
| Install | 3 | 543 | 14 |
| Else | 2 | 117 | 2 |
| Delete | 2 | 643 | 42 |
| Avail | 11 | 376 | 12 |
| addbuffers | 1 | 101 | 1 |
| Path | 2 | 117 | 2 |
| Resident | 2 | 171 | 4 |
| Assign | 2 | 435 | 7 |
| EndCLI | 2 | 182 | 3 |
| Rename | 2 | 177 | 3 |
| Dir | 2 | 419 | 11 |
| NewCLI | 2 | 388 | 9 |
| NewShell | 2 | 379 | 9 |
| Echo | 2 | 241 | 10 |
| cls | 2 | 20 | 1 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 601 | 42 |
| Makedir | 2 | 211 | 4 |
| List | 2 | 1896 | 96 |
| CD | 2 | 282 | 5 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 413 | 15 |
| LoadWB | 12 | 536 | 30 |
| Version | 11 | 574 | 18 |
| SetMap | 13 | 674 | 30 |
| More | 19 | 2321 | 79 |
| Format | 30 | 2069 | 79 |
| CLI | 11 | 637 | 28 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_BLIT | 11 | 252 | 6 |
| d | 1 | 3 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_CPU | 10 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon | 8 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon_install | 7 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `call_two_routines_then_jump` (More): Calls one routine with a global value and another with no arguments, then makes an indirect call through a pointer. Kind: checksum or integrity trap. Pass path: the jump to the function pointer (*unaff_A3).

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Keymap

- `null_routine`: The routine is empty and contains no executable instructions.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_to_buffer`: If a destination pointer is given, copies sixteen longwords from a fixed offset in a structure into it, then calls another routine.
- `update_table_entry_and_copy_range_2`: Updates a structure or table entry with a new pointer and range-end value, and then copies a sequence of bytes from a source buffer to a destination buffer, terminating the copy with a specific byte value.
- `update_table_entry_and_copy_range`: Reads a byte from a table using an index stored in a structure, updates a pointer and a range-end value in that structure, and then enters a loop to copy a sequence of bytes from a source table to a destination table, calling another routine during the process.
- `update_structure_and_branch_offset_2`: Updates a structure or table by calculating a new range, then iterates through a loop to copy bytes from a source table to a destination table using an offset-based index, and finally updates a pointer or value at a specific offset in the structure.
- `update_structure_and_branch_offset`: Updates a table or structure using an offset-based index, copies certain longwords between structure members, and then enters a loop that checks a value against a counter, potentially calling another routine and updating further structure members or a global variable.
- `increment_d1_with_offset`: Sets register d1 to 1, adds a longword from offset 54 of the structure in a1 to d1, and then branches to another routine.
- `test_structure_longword`: Tests a longword at offset 40 of the structure in a1, branches to another routine if it is non-zero, otherwise it branches to a third routine.
- `mask_structure_bits_and_compare`: Checks if certain bits are set in a structure member at offset 48 of a1, and if not, sets the upper 24 bits of that member to all ones.
- `update_structure_pointer_and_clear_table`: Updates a pointer in a structure, calls another routine, and if the new pointer value is non-zero, proceeds to update a table entry, reset several fields in a structure, and conditionally decrement a value in a table or array based on a pointer check.
- `update_structure_pointers_2`: Updates a pointer in a structure, calls a routine, and conditionally clears or decrements a value in a table or array based on a pointer check.
- `update_structure_pointers`: Updates a pointer in a structure, calls a routine, and conditionally clears or decrements a value in a table or array based on a pointer check.

### Install

- `install_max_sized_buffer_handler`: Finds the largest value stored in a linked list of entries, then, if a library call succeeds, fills a new buffer header with a pattern and copies a sixteen-entry block into it.
- `copy_memory_block_5`: Transfers a fixed block of sixteen bytes from a source offset to a destination address.
- `load_and_check_pointer`: Loads an address into a1, calls a library routine, and if d0 is not zero, loads a pointer from a structure in a0 into a0 and then returns.
- `set_d0_and_branch`: Sets d0 to 255 and branches to an offset of 36 relative to itself.
- `initialize_installer_data_structures_2`: Sets certain flags and pointers in a structure, clears or initializes several memory ranges, and calls a routine multiple times to check or reset certain memory-based or pointer-based values.
- `initialize_installer_data_structures`: Updates several pointers and values in a structure, then calls a routine multiple times to clear or initialize various memory ranges, including a loop that copies a range of longwords from a source table and a loop that clears a memory range up to a certain limit.
- `initialize_buffer_and_verify_data_2`: Calculates a target index or pointer, decrements a value in a structure, and then enters a loop that copies a range of longwords from a source table to a destination buffer, or otherwise copies a single longword if the loop condition is not met.
- `initialize_buffer_and_verify_data`: Calculates a pointer or index into a table, clears a value in a structure, and then enters a loop that initializes a memory range up to a certain limit while checking for a specific constant.
- `verify_and_update_table_index`: Clears a long value at offset 572 of a structure in a1 and calls another routine to fetch data.
- `update_structure_index_and_validate_table`: Checks if a value at offset 528 of a structure in a1 is zero, and if so, calculates a new index or pointer using a bitwise NOT and a table lookup, then executes a series of subroutines that update or validate a list of pointers or values.
- `update_structure_value`: Checks if a 4-byte value at offset 544 of a structure in a1 is equal to all ones, and if not, calculates a new address based on a pointer in a3, calls another routine, and sets a 4-byte value at offset 508 of a structure in a1 to 5.
- `validate_list_contents`: Checks a value in a table for a specific constant, updates a structure in a1 with a new pointer and a counter, and uses a loop to check if certain values in a table match or if a certain constant is present, then calls another routine or updates a structure in a1 before jumping to another routine.
- `branch_on_test`: Tests a value at an offset in a structure and, if not zero, loads a pointer into d1 and calls a routine.
- `execute_multiple_subroutines`: Calls a subroutine with a pointer or a value derived from a pointer, updates a value at offset 508 of a structure in a1, and then calls several other subroutines with different parameters or pointers before jumping to a target.

### Else

- `find_max_and_populate_buffer_13`: Traverses a linked list to find the maximum value in a specific field, then uses that value to calculate a buffer size and populate a structure with a header and data.
- `copy_memory_block_9`: Copies sixteen bytes from a source address to a destination address using a loop.

### Delete

- `find_max_and_populate_buffer_14`: Traverses a linked list to identify the maximum value in a specific field, then uses that value to initialize and populate a data buffer.
- `copy_memory_block_and_call_5`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then executes a function via a pointer stored in a register.
- `update_table_index_and_traverse_2`: Calculates a new index or pointer for a table in a1, clears certain fields in a1 and a2, and calls another routine to check or update a secondary table or list, potentially entering a loop that traverses or updates entries based on values read from a table in a0.
- `update_list_index_from_table_2`: Reads a value from a table using an offset, calculates a new index, and stores two bytes or a word into a structure at offset 544 and 548, then branches to another routine.
- `execute_two_subroutines_with_offset`: Increments a value in a structure pointed to by a1, then calls another routine using a pointer from a4, and repeats this process using a different value from a1, before branching back to a previous instruction.
- `process_list_with_conditional_call`: Increments a value in a structure, then checks if a certain value in a list or a global variable matches a certain condition, calling another routine if certain values are encountered or if a list-traversal condition is met.
- `validate_and_increment_list_index_2`: Checks if a value at offset 544 of a structure in a1 matches certain constants, and if so, branches to other routines; otherwise, it increments a value at offset 544 of the structure in a1 and continues execution.
- `update_table_entry_and_branch`: Increments a byte at an offset of 536 in a1, compares a longword from a2 to a longword in a1, and if a certain condition is met, loads a byte from a table using a scaled index and jumps to a routine in a6.
- `test_and_jump_indirect`: Tests a longword at offset 640 of a structure in a2; if non-zero, it branches to another routine, otherwise it clears d1 and jumps to the address in a6.
- `update_pointer_and_validate_index`: Increments a counter in a structure, then enters a loop that reads an index from a table and checks if that index or a subsequent byte matches certain constant values, updating a global variable if a match is found or calling another routine if certain conditions are met; otherwise, it continues or returns via an indirect jump.
- `branch_to_target_routine_4`: Returns immediately by performing a short branch to another routine.
- `update_pointer_and_branch_conditionally`: Calculates a new index by adding a value from offset 548 of a structure in a1 to a constant, uses this to fetch a byte from a table in a0, and stores it in offset 552 of a1.
- `validate_and_update_index_table`: Checks if a value loaded from a structure offset against a list of prohibited or permitted constants, and if a match is found, updates a value in a structure and calls another routine; otherwise, it enters a loop that iterates through a table, updating structure members and calling another routine if certain byte values are encountered.
- `add_and_branch_3`: Sets d1 to 1, adds a longword from offset 544 of a structure in a1 to d1, and branches to another routine.
- `branch_to_another_routine`: Performs a short branch to a location within another routine.
- `update_global_pointer`: If a certain long-word is not zero, it sets a global variable at offset 676 of a structure in a2 to a constant value and then jumps to another routine.
- `search_and_update_list`: Copies a 32-bit value from a structure in a2 to offset 4 of a1, then loops to check if a value at a certain offset in a0 matches a value in a1, updating a pointer in a1 if a match is not found.
- `update_and_store_pointer`: Increments a 32-bit value at an offset of a structure in a2, calculates a new address using a value from a structure in a2 and a pointer from a1, stores a value at that address, and jumps to an indirect address in a6.
- `set_index_to_max`: Compares a value from a structure in a2 with another value from a structure in a2, and if the first is not greater, sets a value at offset 624 of a2 to negative 255 and branches to another routine, otherwise it calls another routine.
- `update_index_and_load_byte`: Increments a 32-bit value at an offset in a structure, then loads a byte from a table into another offset in that structure using a calculated index.
- `execute_conditional_calls`: Compares a value in a structure to a constant and calls a routine twice if they match, otherwise calls the routine once and jumps to an indirect address.
- `copy_and_call_routine`: Copies a long-word from a structure in a2 to a pointer in a1, copies another long-word from a structure in a2 to an offset of a1, modifies a pointer in a4, and calls another routine.
- `set_and_jump_indirect`: Sets d1 to negative one, stores it as a longword at offset 668 of a structure in a2, copies a longword from a1 to d1, and jumps to the address in a6.
- `execute_two_routines`: Calls a routine twice with different pointer offsets in a4 and jumps to an indirect address.
- `update_pointer_and_set_value`: Calls a routine, updates a pointer, and if a certain value in a structure does not match a constant, sets a 32-bit value in a structure to a negative constant before calling another routine.
- `branch_on_index_value`: Checks a value from a structure in a1 against several constants, calling other routines or setting a value in a structure in a2 if certain conditions are met, and finally jumps to an indirect address.
- `update_structure_member_on_condition`: Clears a 4-byte value at an offset of 4 from a pointer in a1 and calls another routine, then checks if a value at offset 656 of a structure in a2 matches certain constants; if certain conditions are met, it calls another routine, updates a pointer in a1, and performs a memory write or further routine calls before jumping to a routine in a6.
- `modify_and_store_byte`: Calculates an offset using a value from a structure in a2 and a pointer in a1, stores a byte from a structure in a2 into a memory location, copies a long-word from a structure in a2 to a pointer in a1, modifies a pointer in a4, and calls another routine.
- `load_and_call_routine`: Loads a value from a structure in a2 into d2, loads a value from a structure in a1 into d1, sets a4 to an offset of a4, and calls another routine before branching to another routine.
- `nop_and_branch_3`: Performs a no-operation and branches to another routine.
- `traverse_and_replace_list`: A loop that replaces a value in a list with a byte from a table and updates a pointer, continuing until a null pointer is encountered.
- `branch_to_target_address_3`: Stores a long-word from d1 into an offset of a pointer in a1, then if d1 is not zero, jumps to another routine; otherwise, it overwrites d1 with d2 and jumps to an address in a6.
- `branch_to_target_routine`: Performs a short branch to another routine.
- `search_and_replace_list_end`: A loop that replaces a value in a list with a byte from a table and checks if that byte is non-zero, then replaces a byte at a specific offset with a value from a table.
- `clear_and_populate_table`: Clears several offsets in a structure in a2, copies a value from a0 to a structure in a1, and loops to clear a range of bytes in a0 using a pointer from a1, then calls other routines and jumps to an indirect address.
- `execute_three_routines_and_invert`: Calls a routine three times with different pointer offsets and then inverts a 32-bit value in a structure.
- `copy_and_sum_bytes`: Copies bytes from a table in a0 into a structure in a1, then loops to copy bytes from a table in a0 into a structure in a1, and finally writes a byte representing the sum of two bytes into a table in a0.
- `store_sum_to_table`: Calculates the sum of two 32-bit values from a structure and stores the lower byte of that sum into a table at an offset determined by a third value in the structure.
- `copy_and_expand_list`: Copies a byte from a table, updates a pointer, and enters a loop that calls a routine and writes bytes to a table-based destination until a certain limit is reached.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `process_pointer_list`: Checks a bit in a stack value, copies a value from a structure in a0 to a structure in a1, and enters a loop that calls other routines while updating pointers and checking a byte in a table, eventually writing a byte to a table in a0.
- `update_pointer_loop`: Loads a value from a pointer, calls another routine, and repeats this process while updating a pointer-based value in a structure if the returned value is non-zero, then writes a byte to a table and calls another routine before jumping to a new address.

### Avail

- `run_program_with_task_setup`: Program entry point that allocates a task structure, opens the DOS library, and then calls the main routine with the command-line text.
- `finish_and_reply_message`: Stores a result in a task field, calls the exec CloseLibrary and FreeMem routines when their pointers are set, then replies to the message through the exec ReplyMsg routine.
- `sum_matching_region_sizes`: Walks a linked list of memory regions and adds up the sizes of those whose attribute word shares at least one bit with the given mask.
- `print_memory_usage_table`: Prints a memory usage report with columns for available, in-use, maximum and largest blocks, for chip, fast or total memory, using the referenced text template.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_with_stack_slot`: Forwards two arguments to another routine together with a pointer to a stack slot.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `format_stack_string_and_forward`: Calls the exec RawDoFmt routine through a stored base to fill a 140-byte stack buffer, then measures the resulting string.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `add_buffers_to_drive`: Looks up the drive named by the first argument in the DOS library and adds buffers to it, then prints how many buffers it now has.

### Path

- `find_max_and_populate_buffer_15`: Iterates through a linked list to find the maximum value in a specific field, then allocates or prepares a buffer based on that value.
- `copy_data_and_call_function_2`: Copies sixteen bytes of data from a memory location offset by a register to a destination pointed to by a stack pointer.

### Resident

- `find_max_and_populate_buffer_16`: Traverses a linked list to find the maximum value in a specific field, then initializes a buffer with a header and populates it by copying data from a source structure and a secondary source.
- `copy_memory_block_10`: Copies sixteen bytes of data from a memory location to a destination specified by a pointer on the stack.
- `validate_and_update_structure_pointer`: Calculates a new pointer or index, updates a field in a structure, and calls several subroutines to check or update further structure members; if certain conditions are met, it checks a bitmask in a table and updates a field before performing an indirect jump or calling another routine.
- `clear_and_jump_indirect`: Clears d1 and jumps to the address in a6.

### Assign

- `find_max_and_populate_buffer_17`: Traverses a linked list to find the maximum value in a specific field, then allocates or prepares a buffer by writing a header and a sequence of incrementing values.
- `copy_memory_block_and_call_6`: Copies sixteen bytes from a memory location offset from a register to a destination address, then executes a function pointer.
- `update_linked_list_node_and_table_2_2`: Calculates an index-based pointer, updates a structure or table with new bounds or pointers, and traverses a linked list or table-based chain, updating node pointers and checking for end-of-list or invalid index conditions, calling another routine to perform certain updates or checks during the traversal.
- `update_linked_list_node_and_table_2`: Updates a linked list node by copying a pointer from one index to another, then enters a loop that traverses and updates subsequent nodes in.
- `update_linked_list_node`: Updates a pointer in a structure or list, checks a condition or flag, and if certain criteria are met, updates a value in a structure or list to a constant or a value from another structure, then calls another routine.
- `update_linked_list_node_and_table`: Updates a linked list node by copying a pointer from one index to another, then enters a loop that traverses and updates subsequent nodes in the list.
- `assign_table_values`: Updates several values in a table using offsets derived from a structure, calls another routine multiple times to check or clear certain flags, and finally jumps to an address in a6.

### EndCLI

- `find_max_and_populate_buffer`: Iterates through a linked list to find the maximum value in a specific field, then uses that value to calculate a buffer size and populate a structure with a header and data.
- `copy_data_and_call_function`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then calls a function via a pointer held in another register.
- `modify_list_and_table`: Modifies a bitmask in a structure, updates a pointer in a structure, and conditionally decrements a value in a table if it is greater than zero.

### Rename

- `find_max_and_populate_buffer_2`: Traverses a linked list to find the maximum value in a specific field, then initializes a buffer with a header and a sequence of incrementing values.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.
- `update_pointer_and_call`: Updates a pointer in a structure, calls a routine, and conditionally updates a value in a structure or table.

### Dir

- `find_max_and_populate_buffer_3`: Iterates through a linked list to find the maximum value in a specific field, then allocates or prepares a buffer based on that value.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then calls another routine.
- `update_entity_bounds_and_state`: Updates several pointer-based offsets in a structure in a1 to define a memory range, clears certain fields, and conditionally updates a structure in a2 with values from a1 or a table, then calls another routine or returns.
- `update_structure_field_from_registers`: If a longword at offset 332 of the structure in a1 is non-zero, it loads three values from offsets 88, 48, and 48 (the same offset) into d3, d2, and d1, then calls another routine using a pointer-adjusted a4, and finally writes the value from d1 into offset 320 of the structure in a1 before branching away.
- `clear_structure_longword`: Tests a longword at offset 320 of the structure in a1; if it is zero, it branches to another routine, otherwise it clears that longword and branches to a third routine.
- `update_entity_property_and_loop`: Updates a field in a structure using a value from another structure, then calls a routine multiple times with different offsets to update or check further fields, and finally loops through a range of indices to clear or set values in a table, or otherwise returns.
- `update_entity_state_and_pointer_2`: Checks a value in a table, and if it equals two, calls a routine to update a field in a structure, then updates a byte in a table using a pointer.
- `update_entity_state_and_pointer`: Checks a value in a table, and if it equals two, calls a routine to update a field in a structure, then updates a byte in a table using a pointer, and subsequently calls several other routines to update or reset various fields in the structure, including updating a pointer-based index and a byte-based index, before finally branching to another routine.
- `update_structure_range_and_set_value`: Checks if certain pointers in a structure in a2 and a structure in a1 are non-zero, then if a certain value is not one, calls a routine, and if a certain flag in a2 is zero, initializes a range of values in a structure in a1 by looping and setting each entry to one, before calling another routine and updating a value in a2 from a structure in a1.
- `set_structure_value_with_offset`: Calculates a pointer or offset by adding a value from a structure in a1 to d1, shifts it right by two, and calls a routine using a pointer from a2.
- `set_structure_value`: Calls a routine, and if a certain value is zero, sets a value at an offset in a structure.

### NewCLI

- `find_max_and_populate_buffer_4`: Traverses a linked list to find the maximum value in a specific field, then uses that value to initialize a buffer with a sequence of incrementing longwords.
- `copy_memory_block_and_call_2`: Copies sixteen bytes from a memory location offset from a pointer to a destination address, then calls another routine.
- `recalculate_bounds_and_copy_table_entries`: Updates a table of bounds or limits by recalculating or copying pointer-indexed values, and then performs a loop to copy a range of entries from one table or array to another, potentially incrementing a counter if a certain value is non-zero.
- `update_table_and_recalculate_bounds_2`: Updates a table entry by copying a value from one pointer-indexed location to another and increments a counter if it is non-zero, then calls another routine to perform further updates or resets on a structure, potentially involving a loop that searches for a zero-value entry in a table or updates a pointer-based bound or counter.
- `update_table_and_recalculate_bounds`: Iterates through a table, updating or resetting certain values, and if a certain condition is met, it updates a pointer or value in a structure and calls another routine multiple times.
- `increment_d1_with_offset_alt`: Sets register d1 to 1, adds a longword from offset 300 of the structure in a1 to d1, and then branches to another routine.
- `update_table_and_decrement_logic_branch`: If a certain value in a structure is zero, it calls a routine three times with a constant value in d0 and a pointer from a table in a2; otherwise, it performs a series of pointer-based assignments and calls a routine multiple times, including a calculation that shifts a pointer-based address right by two bits before calling the routine again.
- `update_table_and_decrement_logic`: Checks if a value at an offset in a1 is below a certain threshold, and if so, performs several pointer-based assignments and calls a routine using a pointer from a2.
- `update_table_and_decrement`: Updates a value in a table, calls several routines, and conditionally clears or decrements a value in a table if a certain condition is met.

### NewShell

- `find_max_and_populate_buffer_5`: Traverses a linked list to determine the maximum value in a specific field and then initializes a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.
- `update_structure_index_and_table`: Updates a structure's index and table by copying values from a source table to a destination table, and increments or decrements a counter in a global structure if a certain condition is met.
- `sync_structure_index_and_update_table`: Checks if a table entry or structure index is non-zero, copies a value from one table entry to another, and increments or decrements a counter in a structure.
- `update_structure_index_and_sync_values`: Checks if a table entry is zero and updates it with a calculated value, then enters a loop that updates a structure's index or clamps a value to 800, while calling another routine and performing several conditional updates to a structure and a table.
- `increment_d1_with_offset_shell`: Sets register d1 to 1, adds a longword from offset 304 of the structure in a1 to d1, and then branches to another routine.
- `sync_and_update_structure_value`: Checks if a value at offset 692 of a structure in a1 is zero; if not, it clamps a value at offset 276 of a1 to 800, synchronizes a secondary field at offset 268 with a value from offset 276, and calls a routine using a pointer from a2, then performs further updates to a table and structure using a pointer from a2 and a pointer from a4.
- `clamp_and_sync_structure_value`: Ensures a value in a structure does not exceed 800, synchronizes a secondary field with that value, and then executes a series of calls to update various structure members and table-based pointers or counters based on whether the primary value is zero or non-zero.
- `update_structure_and_table`: Updates a value in a structure, calls several routines, and conditionally clears or decrements a value in a table if a certain condition is met.

### Echo

- `find_max_and_populate_buffer_6`: Traverses a linked list to determine the maximum value in a specific field and then initializes a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_3`: Copies sixteen bytes from a memory location offset from a pointer to a destination address, then executes another routine.
- `update_table_range_and_iterate_2`: Updates a range or pointer values in a structure by calling a routine to retrieve new indices, then enters a loop that iterates through a range of indices, updating a structure's range or pointer values.
- `update_table_range_and_iterate`: Updates a structure's range or pointer values by calling a routine to retrieve new indices, then enters a loop that iterates through a range of table entries, calling a routine at each step and updating a pointer or index in the structure.
- `copy_longword_to_offset_148`: Copies a longword from offset 336 of the structure in a1 into register d1, stores d1 at offset 328 of the structure in a1, and then branches to another routine.
- `adjust_list_bounds_and_iterate`: Sets a longword at offset 328 in a1 to 1, checks certain values in a structure, and if certain conditions are met, calculates a new index and branches back to an earlier instruction or to another routine.
- `copy_longword_to_offset_14c`: Copies a longword from offset 336 of the structure in a1 into register d1, stores d1 at offset 332 of the structure in a1, and then branches to another routine.
- `update_list_bounds_and_iterate`: Updates two pointer-sized values in a structure, checks if a certain range is valid, and if so, iterates through a table to read a byte from an offset calculated from a value in the structure, then calls another routine and updates a value in the structure before branching.
- `check_structure_and_call`: Checks a value at an offset in a1, and if it is not zero, calls a routine using a pointer from a2.
- `conditional_routine_calls`: Checks a value in a structure and calls a routine or a sequence of routines if certain conditions are met.

### cls

- `open_dos_library`: Opens the DOS library through the exec base, makes two calls on it, and returns zero, or the result of a further call when the second call reports a negative value.

### Execute

- `find_max_and_populate_buffer_7`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_4`: Copies a sequence of sixteen bytes from a memory location offset by a register to a destination address on the stack, then executes a function pointer.

### Copy

- `find_max_and_populate_buffer_8`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_3`: Copies sixteen bytes from a memory location offset from a register to a destination address.
- `update_table_index_and_traverse`: Calculates a new index or size for a table entry in a structure, clears certain fields in a secondary structure, and then enters a loop that traverses a table of pointers, checking for specific byte-encoded command or jump values and calling another routine if certain conditions or specific values are met.
- `update_list_index_from_table`: Calculates a new index using a value from a table, retrieves two bytes from a data structure using this index, and stores these bytes into a structure at offset 544 and 548, then branches to another routine.
- `execute_two_subroutines_with_offset_2`: Increments a value in a structure pointed to by a1, then calls another routine using a pointer from a4, and repeats this process using a different value from a1, before branching back to a previous instruction.
- `process_list_with_conditional_call_2`: Increments a value in a structure, then checks if a certain value in a list or a global variable matches a certain condition, calling another routine if certain values are encountered or if a list-traversal condition is met.
- `dispatch_list_index_by_type`: Compares a value at offset 544 of a structure in a1 against several constants and branches to different subroutines if the value matches 35, 37, 40, or 124; otherwise, it branches to a default location.
- `copy_range_to_structure`: Increments a value at an offset of 536 from a1, compares a value from a2 to that new value, and if it is not greater or equal, checks a byte from a table using a shifted offset from a1 and jumps to a routine in a6 if a certain condition is met.
- `branch_on_long_value`: Tests a long value at offset 716 of the structure in a2 and branches to another routine if non-zero, otherwise it clears d1 and jumps to the address in a6.
- `update_index_table_and_validate_constants`: Increments a counter in a structure, then iterates through a list of offsets to check if the bytes loaded from a table match certain permitted or prohibited constants, updating a global value if a match is found or calling another routine if certain conditions are met; otherwise, it returns or jumps to another location.
- `branch_to_target_routine_2`: Performs a short branch to another routine.
- `update_pointer_value_and_verify`: Updates a value in a structure at offset 552 using a byte read from a table, then compares this new value against a global variable at offset 700; if they match, it performs a secondary table lookup and calls another routine.
- `validate_index_against_exception_list`: Compares a value from offset 552 of a structure in a1 against a list of specific constant values, branching to different routines or returning based on whether a match is found.
- `add_and_branch`: Sets d1 to 1, adds a long value from offset 544 of the structure in a1 to d1, and branches to another routine.
- `branch_to_offset`: Branches to an offset of 82 relative to itself.
- `set_constant_d2`: Sets a 32-bit value in a structure at offset 424 of a2 to a constant and then jumps to another routine.
- `compare_and_update_pointer`: Compares a value from a structure in a1 with d1, and if certain conditions are met, calculates a new address using a value from a structure in a2 and calls a routine.
- `increment_and_copy_value`: Increments a value in a structure and copies a word from a source pointer to a destination pointer in a table.
- `compare_and_set_constant`: Compares a value from a structure in a2 with another value from a structure in a2, and if certain conditions are met, sets a value in a structure in a2 to a constant and branches.
- `increment_and_store_byte`: Increments a value in a structure and stores a byte from a source pointer into a destination pointer in a structure.
- `check_table_and_call`: Compares a value in a structure to a constant and calls a routine if they are equal.
- `copy_structure_data`: Copies a 32-bit value from a structure in a2 to a pointer in a1, copies another 32-bit value from a2 to an offset in a1, and calls another routine before jumping to another routine.
- `set_and_jump`: Sets d1 to 255, stores it as a long value at offset 712 of the structure in a2, loads a long value from the address in a1 into d1, and jumps to the address in a6.
- `execute_two_routines_2`: Calls a routine twice with different pointer offsets in a4 and jumps to an indirect address.
- `update_conditional_value`: Calls a routine, updates a pointer-based value, and if a certain value in a structure in a2 does not equal 41, sets a value at offset 704 of that structure to -1 before calling another routine and jumping to an indirect address.
- `dispatch_structure_type_actions`: Reads a value from an offset of 4 in a structure and branches based on its value.
- `execute_subroutine_loop`: Clears a 4-byte value at an offset of 4 in a pointer-based structure, calls another routine, and then enters a loop that updates a 4-byte value at an offset of 8 in that structure, continuing or branching based on a comparison with a value in a structure in a2, before calling another routine and jumping to a location in a6.
- `update_pointer_and_store`: Calculates an offset using a value from a structure in a2 and a value from a structure in a1, stores a byte from a structure in a2 at an offset in a0, copies a 32-bit value from a2 to a1, and calls another routine before jumping to another routine.
- `load_parameters_and_call_subroutine`: Loads a longword from a global variable into d2 and a longword from a structure offset 8 into d1, sets d0 to a constant, updates a pointer in a4, and calls another routine before branching to a different code location.
- `nop_and_branch`: Performs a no-operation and then branches to another routine.
- `copy_linked_list_bytes`: Loops through a list of bytes, copying a byte from a source table to a destination, and updates a pointer in the list, continuing until a null pointer is encountered, then jumps to an indirect address.
- `branch_to_target_address_3_2`: Stores a long-word from d1 into an offset of a pointer in a1, then if d1 is not zero, jumps to another routine; otherwise, it overwrites d1 with d2 and jumps to an address in a6.
- `branch_to_offset_2`: Branches to an offset of 1038 relative to itself.
- `copy_and_replace_bytes`: Loops through a list of bytes, copying a byte from a source table to a destination, and updates a pointer in the list, continuing until a zero byte is found, then copies a byte from a different offset and jumps to an indirect address.
- `clear_and_populate_list`: Clears several 32-bit values in a structure, loads a byte from a table using a pointer and an offset, and then loops to clear a range of bytes in a table.
- `execute_three_subroutines`: Calls a routine three times with different offsets of a4, then negates a value from a structure in a2 and jumps to an indirect address.
- `update_table_values`: Loads two bytes from a table using scaled offsets from d1 and d2, stores them into a structure in a1, and enters a loop that copies bytes from the table to another location in the table based on values stored in the structure.
- `store_sum_to_table_2`: Calculates the sum of two 32-bit values from a structure and stores the lower byte of that sum into a table at an offset determined by a third value in the structure.
- `process_and_call_routine`: Copies a byte from a table to another location in the table, updates a structure in a1 with values from the table, and enters a loop that calls a routine via a pointer in a5 and writes the returned byte back to the table.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `swap_and_store_pointer_2`: Calls a routine, sets a value in a structure, and if a certain value is zero, calls another routine twice and sets a pointer to a new address.
- `swap_and_store_pointer`: Copies a 32-bit value from a source table to a location in a structure in a2, then copies a value from a list into that source table, and finally updates a pointer in that structure, then jumps to an indirect address.

### Makedir

- `find_max_and_populate_buffer_9`: Traverses a linked list to find the maximum value in a specific field, then allocates or prepares a buffer by writing a header and filling it with incrementing values.
- `copy_memory_block_4`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer.
- `execute_conditional_logic_2`: Performs several routine calls and updates various offsets in a structure based on values retrieved from a source table.
- `execute_conditional_logic`: Performs several routine calls and updates various offsets in a structure in a1 based on values retrieved from a source table, including conditional logic that sets a value to 10 if a certain check fails, before jumping to an indirect address.

### List

- `find_max_and_populate_buffer_10`: Traverses a linked list to find the maximum value in a specific field, then initializes a buffer with a header and populates it by copying data from a source structure.
- `copy_memory_block_6`: Copies a sequence of sixteen bytes from a memory location offset by a register value to a destination pointer.
- `update_table_and_traverse_links`: Updates a structure's size or index field, then enters a loop that traverses a linked list or table of pointers, checking for specific byte-encoded command or jump values and calling another routine if certain conditions or specific command bytes are encountered.
- `update_list_index_and_cache_values`: Calculates a new index using a base offset and a value from a table, then reads two bytes from a structure at that calculated offset and stores them into a structure at offsets 544 and 548, before branching to another routine.
- `fetch_and_update_list_metadata`: Loads a value from an offset of 540 in a structure into a register, adds 1 to it, and calls another routine.
- `initialize_list_iterator`: Sets an index in a structure to one and adds a value from a table offset to it, then calls another routine.
- `dispatch_list_item_type`: Compares a value at offset 544 of a structure in a1 against several constant values and branches to different routines if the value matches 35, 37, 40, or 124.
- `copy_range_to_structure_alt`: Increments a value at an offset of 536 from a1, compares a value from a2 to that new value, and if it is not greater or equal, checks a byte from a table using a shifted offset from a1 and jumps to a routine in a6 if a certain condition is met.
- `branch_on_long_value_2`: Tests a long value at offset 548 of a structure in a2 and branches to another routine if it is non-zero, otherwise it clears d1 and jumps to the address in a6.
- `validate_and_update_index_table_2`: Iterates through a list of indices, checking if certain bytes loaded from a table match a set of permitted or prohibited constants, and if a match is found, updates a value in a structure and calls another routine; otherwise, it continues or returns via an indirect jump.
- `branch_to_target_routine_3`: Returns immediately by performing a short branch to another routine.
- `update_pointer_and_branch_on_value`: Calculates a new index by adding a value from offset 548 of a structure in a1 to a base offset, retrieves a byte from a table in a0 using this index, and stores it in offset 552 of the structure in a1.
- `validate_index_against_constants`: Compares a value loaded from offset 552 of a structure in a1 against several constant values, branching to different routines or returning based on the match.
- `add_and_branch_2`: Sets d1 to 1, adds a long value from offset 544 of a structure in a1 to d1, and branches to another routine.
- `branch_to_offset_3`: Branches to another routine at a relative offset.
- `set_global_pointer`: Sets a global variable at offset 616 of a structure in a2 to a constant value, otherwise it returns.
- `update_and_store_pointer_2`: Sets a value in a structure, and then loops to compare a value from a table against a value in a structure, potentially updating a pointer and a table value.
- `increment_and_store_pointer`: Increments a value at offset 676 of a structure in a2, calculates an offset using a value from a structure in a2 and a pointer from a1, and stores a value from a1 at that calculated address before jumping to a target.
- `update_index_and_load_byte_2`: Compares a value from a structure and, if it is within a certain range, sets a value in a structure to a constant.
- `update_value_from_table`: Increments a value at offset 696 of a structure in a2, calculates an offset using a value from a structure in a2 and a pointer from a0, and stores a byte from that table into a value at offset 692 of a structure in a2 before jumping to a target.
- `conditional_subroutine_execution`: Compares a value at offset 692 of a structure in a2 with a constant, and if they are equal, calls a subroutine twice using a pointer or a value derived from a pointer before jumping to a target.
- `copy_structure_members`: Copies a 32-bit value from a structure in a2 to a pointer in a1, copies another 32-bit value from a structure in a2 to an offset of 4 in a1, then calls another routine and branches to a new location.
- `set_and_jump_2`: Sets d1 to negative one, stores it as a long value at offset 640 of a structure in a2, loads a long value from a1 into d1, and jumps to the address in a6.
- `call_twice_and_jump`: Sets d0 to 32 and calls a routine using a pointer in a5, then sets d0 to 20 and calls a routine using a pointer in a4 plus an offset of 260, and finally loads a long value from a1 into d1 before jumping to the address in a6.
- `update_structure_with_subroutine_result`: Calls a subroutine, copies the result back to a pointer in a1, and if a value at offset 628 of a structure in a2 is not 41, sets a value at offset 640 of a structure in a2 to -1, then calls another subroutine before jumping to a target.
- `compare_and_branch_on_value`: Loads a longword from offset 4 of a structure in a1 and compares it against several constants, branching to different locations if the value matches -1, 35, 40, 41, or 124, otherwise it performs an unconditional branch.
- `execute_conditional_logic_3`: Clears a 32-bit value at an offset of 4 in a pointer, calls another routine, and then enters a loop that calls another routine while checking if a value in a structure in a2 matches certain constants, or otherwise calls another routine and jumps to a location in a6.
- `update_table_and_call`: Calculates an offset using a value from a structure in a2 and a pointer in a1, copies a byte from a table in a0 to a pointer in a1, copies a 32-bit value from a structure in a2 to a pointer in a1, then calls another routine and branches to a new location.
- `load_and_call_subroutine`: Loads a long-word from an offset of 632 in a2 into d2, loads a long-word from an offset of 8 in a1 into d1, loads the constant 24 into d0, loads a pointer or address or offset 128 in a4 into a4, calls a routine using the address in a5, and branches to another routine.
- `nop_and_branch_2`: Performs a no-operation and branches to another routine.
- `process_linked_list_and_swap`: Loops through a list where each element contains a pointer to the next, reading a byte from a table and writing it to a certain offset, and swaps a byte from a pointer in a1 into a table before checking if the pointer is null.
- `store_and_jump_indirect`: Stores the value in d1 to offset 8 of a structure in a1, then checks if d1 is non-zero to branch to another location; otherwise it moves d2 to d1 and performs an indirect jump using a6.
- `branch_to_offset_4`: Branches to another routine at a relative offset.
- `traverse_and_replace_list_element`: Loops through a list, replacing a value in the list with a byte from a table, and once a certain byte is found, writes a byte from a pointer in a1 into a table before jumping to a target.
- `initialize_and_clear_table`: Clears several 32-bit or 16-bit values in a structure pointed to by a2, copies a byte from a table (indexed by d1 multiplied by 4) into a structure field, and then loops to clear a range of bytes in a table (indexed by d1 and a pointer from a2) until a certain value is reached.
- `execute_subroutines_and_invert_value`: Calls a subroutine three times with different parameters or pointers, and then calculates the bitwise NOT of a value at offset 640 of a structure in a2 before jumping to a target.
- `branch_to_offset_6`: Performs an unconditional branch to an instruction at a relative offset.
- `validate_value_against_constants`: Compares a value pointed to by a1 against a set of constant values and calls another routine regardless of the result.
- `set_d1_and_jump`: Sets d1 to negative one and jumps to the address in a6.
- `set_d1_and_jump_2`: Sets d1 to zero and jumps to the address in a6.
- `check_value_against_list`: Loads a value from a pointer in a1 and compares it against several constant values, otherwise it jumps to another routine.
- `branch_on_pointer_nullity`: Tests if the pointer in a1 is null, otherwise it branches to another location; if null, it sets d1 to zero and performs an indirect jump using the address in a6.
- `process_list_and_call`: Copies a byte from a table in a0 to a pointer in a1, updates a value in a structure in a1, and if a certain condition is met, calculates a new offset, copies a byte from a table in a0 to a pointer in a1, and calls another routine.
- `add_offset_to_constant`: Sets d1 to one, adds the longword at offset 8 of the structure in a1 to d1, and then branches to another location.
- `jump_via_register`: Sets d1 to zero and performs an indirect jump using the address in a6.
- `reset_string_bounds_and_update_display`: Resets several string index bounds and pointer-based array limits in a structure pointed to by a1, calls a routine to initialize string bounds, and then updates certain display-related pointer or size fields in the structure before branching to another routine.
- `initialize_string_index_bounds`: Sets certain array elements in a structure pointed to by a1 to a default value and then uses a routine to check or copy data from a source string or block, potentially updating a global or structure-based index or terminator.
- `parse_string_and_update_index_2_2`: Reads a pointer from a structure, checks if the target string or data block is null, and if not, copies bytes from that source or a secondary source into a destination buffer while updating an index and a length or end-marker flag in the structure.
- `branch_to_offset_5`: Performs a short branch to another location.
- `parse_string_and_update_index_2`: Parses a string or a sequence of bytes from a structure, updating several offset-based pointers and flags in a structure, and checks for a termination character or a specific length limit, otherwise it loops through the string or a pointer-based sequence.
- `parse_string_and_update_index`: Checks if a certain flag in a structure is zero and if a pointer in a table is non-null, then calls another routine to process a string.
- `update_table_and_rebuild_index_2`: Updates a byte in a table using an index-based offset, updates a pointer or counter in a structure, and then enters a loop that copies bytes.
- `update_table_and_rebuild_index`: Updates a byte in a table using an index-based offset, updates a pointer or counter in a structure, and then enters a loop that copies bytes from a source table to a destination table while updating a secondary index or pointer.
- `modify_table_byte`: Decrements a longword loaded from an offset of 1988 in a1, shifts a longword loaded from an offset of 1448 in a1 left by two, and stores the resulting byte into a table at a calculated offset in a0, then branches to another routine.
- `update_pointer_logic_and_display_index`: Clears a byte in a table using an index, checks a flag in a structure, and if certain conditions are met, calculates and stores a new pointer or a bitwise-inverted address into a structure, then calls a routine to update a pointer or display logic.
- `shift_structure_address`: Loads the address of a structure at offset 4648 of a4 into a3 and d1, shifts d1 right by two bits, and then branches to another location.
- `update_pointer_logic_and_display`: Updates a pointer in a structure using an index, potentially updates a table entry with a calculated address, and executes a sequence of calls to a routine using a constant or a pointer-derived value, followed by conditional updates to a structure's field and a final check of a pointer-based value to update a certain field or set a constant value.
- `update_pointer_logic`: Calls another routine using a pointer from a structure, then checks a byte in a table; if the byte is zero, it calls another routine and updates a value in a structure using a pointer-based offset, otherwise it calls another routine and updates a pointer-based value in a structure.
- `update_display_bounds_and_offset_4`: Calculates a bit-shifted value from a pointer-based address, stores it in a structure, and calls another routine to check if the resulting value is zero; if zero, it resets a different structure field to zero using another routine call, otherwise it updates a pointer-based offset or boundary.
- `reset_display_bounds_and_offset_2`: Updates a display boundary or offset in a structure in a1 by calling another routine to retrieve a value, then checks if that value is zero to either reset a display parameter or calculate a new boundary or offset based on a pointer in a2 and calls several other routines to update display parameters or reset certain values.
- `reset_display_bounds_and_offset`: Checks if a display offset or boundary (stored at offset 584 of a structure in a1) is negative, and if so, updates a value in a structure (a), then resets certain display parameters or calculates a new display boundary or offset (via a call to another routine) and updates a structure (a).
- `update_display_bounds_and_offset_logic`: Checks if a display offset or boundary (stored at offset 584 of a structure in a1) is negative, and if so, updates a value in a structure (at offset 648 of a1) with the bitwise NOT of that offset.
- `update_scaled_display_offset`: Calculates a new scaled display offset by loading a value from a pointer in a1, shifting it left by two, and storing it into a structure offset in a0, then updates a pointer in a1 with a value from a2 and calls another routine.
- `store_shifted_address`: Loads the address of a structure at offset 4788 of a4 into a3 and d1, shifts d1 right by two bits, stores the result at offset 604 of the structure in a2, and then branches to another location.
- `update_display_bounds_and_offset_3`: Calculates a new display boundary, updates a value in a structure, and calls a routine to update display parameters.
- `update_display_bounds_and_offset_2`: Calculates a new display boundary, updates a value in a structure, and calls a routine to update display parameters.
- `update_display_bounds_and_offset`: Clears a longword at offset 1994 of a structure in a1, loads a pointer from a table into a4, and calls another routine.
- `validate_display_bounds`: Checks if a value at offset 992 of a structure in a1 is zero, then performs a bounds check by calling another routine with a pointer-based range and a constant, followed by a second bounds check using a different range and constant, otherwise it returns.
- `update_display_offset_logic_4_2`: Calculates a new display offset or limit by adding a value from a structure offset to a register, then calls another routine to check a condition and potentially updates a global variable or pointer-based value.
- `evaluate_display_offset_condition`: Adds a value from a structure offset to a register, loads a pointer or value into another register, and calls another routine; then checks if the previously calculated value is non-positive and branches or otherwise returns.
- `update_display_offset_logic_4`: Checks several fields in a structure for specific value ranges and non-zero status, then calculates a new display offset or limit by adding a constant to a value in a structure and calls another routine.
- `update_display_offset_logic_3`: Checks a value in a structure, then calls another routine to update a display offset or limit.
- `update_display_offset_logic_2_2`: Calculates an index-based offset, updates a limit or a new offset in a structure, and calls another routine.
- `update_display_offset_logic_2_3`: Calculates a new display offset or limit by adding a constant to a value in a structure, updates a field in that structure with a value from a table, and calls another routine to perform further offset or limit updates.
- `update_display_offset_logic_2`: Calculates a new display offset or limit by adding a constant to a value in a structure, updates a field in that structure with a value from a table, and calls another routine.
- `update_display_offset_logic`: Calculates a display offset or limit based on a value in a structure, updates a field in that structure with a value from a table, and calls another routine.
- `update_display_offset_pointer_2_2_2`: Sets a value in a structure to 24, then calculates an index into a table using a value from a structure, retrieves a byte from that table, and updates a display offset pointer in the structure.
- `update_display_offset_pointer_2_2`: Checks if a certain value in a structure is non-zero, then calls another routine to update a display offset pointer using a table, and if certain conditions in a structure are met, increments a counter in a structure and updates a pointer-based value or a global.
- `update_display_offset_pointer_2`: Updates a display offset pointer in a structure by reading a value from a table, increments a counter in a structure, and then updates a pointer or value based on a conditional check of a longword in a structure, otherwise it returns.
- `update_display_offset_pointer`: Copies a longword from an offset of 2000 in a1 to an offset of 2000 in a1, then checks a longword at an offset of 568 in a2 and branches or calculates a new pointer value in a3.
- `update_display_offset_and_index`: Calculates a new display offset or index by checking a condition in a structure, then updates a pointer-based value in a structure or a global variable.
- `test_and_set_bit_one`: Checks if the least significant bit of a register is set after a right shift, and if not, copies a register value to that same register before jumping to another routine.
- `call_and_jump_indirect`: Sets d1 to 45 and d0 to 24, loads the address from offset 224 of the structure in a2 into a4, calls the routine at the address in a5, and then performs an indirect jump using the address in a6.
- `update_display_index_and_parameters_2`: Calculates a new display index or offset based on a series of conditional checks against values in a structure, updates a value at offset 3160 of a structure, and calls another routine multiple times with varying parameters.
- `test_and_mask_bit_one`: Tests if the least significant bit of a register is set and, if so, clears all but the first bit of that register before branching or moving a value into it.
- `execute_indirect_calls`: Loads constants into d1 and d0, moves a pointer from an offset of a2 into a4, calls a routine at the address in a5, and jumps to the address in a6.
- `update_display_index_and_parameters`: Executes a series of calls to a routine with varying register parameters, then calculates a new index or offset based on a pointer-based table lookup and updates a field in a structure, before potentially calling another routine with a new address or offset.
- `check_pointer_equality_and_set_address`: Compares a longword at offset 392 of a structure in a1 to a longword at offset 584 of the same structure; if they are not equal, it branches to another routine, otherwise it loads a pointer to a location relative to a4 into a3 and shifts the value in d1 right by two before branching to another routine.
- `compare_and_shift_pointer_result`: Loads a longword from an offset of 616 in a1, compares it to a longword at offset 384 in a1, and if not equal, loads a new address into a3 and shifts it right by 2 before branching to another routine.
- `compare_and_shift_pointer`: Decrements a longword loaded from an offset of 616 in a1, compares it to a longword at offset 384 in a1, and if not equal, loads a new address into a3, shifts it right by 2, and branches to another routine.
- `calculate_address_offset`: Decrements a longword loaded from an offset of 616 in a1, compares it to a longword at offset 384 in a1, and if the result is not negative, loads a new address from a certain offset in a4 into a3, shifts that address right by two, and branches to another routine.
- `compare_and_update_pointer_offset`: Compares a value from a pointer at offset 616 in a1 with a value from a pointer at offset 384 in a1, then either sets a register to a constant and adds a value from offset 268 in a1 or sets a register to a constant and adds a value from offset 268 in a1, otherwise it returns.
- `update_display_parameters_and_offset`: Updates a global variable with a value from a pointer in a1, then checks a condition to potentially update it again.
- `update_display_parameters_2_2`: Calculates a bit-shifted address from a pointer in a4, calls another routine, and if a certain value in a1 is non-zero, updates a global variable with a bit-shifted address and copies a value from a structure to another location.
- `update_display_parameters_2`: Calculates a bit-shifted address or offset from a pointer in a4, calls another routine, and if a certain value in a1 is non-zero, updates a display-related field in a1 with a bit-shifted value derived from a4, then calls another routine again.
- `update_display_parameters`: Loads a pointer from a4, calculates a value, and calls a routine using a pointer from a2.

### CD

- `find_max_and_populate_buffer_11`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_7`: Copies sixteen bytes from a memory location offset from a register to a destination address.
- `update_structure_and_call_subroutines`: Updates a bit in a byte using a bit-shift, modifies several pointer-based offsets in a structure, and executes a sequence of conditional calls to other routines based on the contents of a pointer-indexed table or a structure-relative offset.
- `check_structure_and_call_2`: Calculates a bit-shifted address from a4, calls another routine, and if a certain value at an offset in a1, scaled by a pointer, is non-zero, calls another routine and updates a global variable with a value from a pointer in a2.
- `set_buffer_and_draw`: Loads a pointer from a2 plus 1544, calls a routine with a length of 720, and if the loaded value is zero, calls another routine and sets a value at a1 plus 704.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_populate_buffer_12`: Iterates through a linked list to find the maximum value in a specific field, then uses that value to initialize a buffer with a sequence of incrementing longwords.
- `copy_memory_block_8`: Copies sixteen bytes from a memory location offset from a register to a destination address.
- `update_object_state_and_indices`: Updates a pointer in a structure, clears several fields at certain offsets, and then checks a series of table-indexed values to conditionally update or reset other fields in the structure, including checking for a specific character '$' in a table.
- `check_table_index_and_set_value_2`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a structure in a0; if that byte is non-zero, it compares a value of 10 in d2 against a longword at offset 548 in a2 and, if less, sets d1 to 255 before branching to another routine.
- `check_table_index_and_set_value`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a structure in a0; if not zero, it compares a value in a2 to 20 and, if less than, sets d1 to a certain value before branching.
- `traverse_and_update_structure_pointers`: Traverses a linked-list-like structure or table-based hierarchy by updating pointers in a structure in a1 based on values found in a table in a0, and calls another routine if certain character or pointer conditions are met.
- `validate_and_update_structure_state`: Checks if certain characters in a table-driven structure match a specific character, and if so, calls another routine to update or reset structure members.
- `branch_to_conditional_target`: Performs a short branch to a target address.
- `test_structure_member_and_branch`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a0; if not zero, it compares a longword at offset 388 in a1 to 1 and, if not equal, sets d1 to a certain value and branches.
- `branch_to_secondary_routine`: Performs a short conditional branch to another routine based on the result of a structure member test performed by the caller.
- `test_structure_member_bits_branch_logic`: Checks if a longword at offset 356 of the structure in a1 is equal to one, and if not, tests if that same longword is zero to determine whether to branch to another routine, branch to a different routine, or branch to a third routine with d1 set to a negative value.
- `update_object_index_and_call_subroutines`: Checks if a pointer-indexed array entry is non-zero, calls a routine, and updates a field in a structure with the returned pointer-index.
- `reset_and_initialize_object_state`: Resets a flag in a structure, checks a pointer-indexed array, and if certain conditions are met, clears a field, sets a counter to ten, and calls another routine.
- `update_object_bounds_and_visibility`: Updates a size or count in a structure, then checks or updates bitmasks or visibility flags in a structure, potentially looping to update a value in a global variable until a certain condition or limit is met, and finally calls another routine.
- `update_object_bounds_and_visibility_2`: Calls another routine to update a value in a structure, loops to check if that value is certain constants, and if certain flags in a1 are zero, calls another routine to update a value in a structure and calls another routine to update a bitmask in a structure.

### LoadWB

- `parse_command_line_arguments`: Parses a command line string into individual arguments, handling quoted strings and converting escape sequences like newline and escape characters.
- `finish_and_reply_message_2`: Stores a result in a task field, calls the library cleanup routines when their pointers are set (including closing a file through the DOS Close routine), then replies to the message.
- `start_workbench_tool`: Starts a Workbench tool from a named resident module by checking that the required library is available, allocating a block, processing a name string, and starting the tool.
- `compare_strings`: Compares two null-terminated strings byte by byte.
- `build_resident_lookup_list`: Builds a singly linked list from a source list, allocating an 8-byte node for each entry and storing the result of a resident lookup for it.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_no_args`: Calls a library function through the stored base at a fixed negative offset, passing no arguments.
- `call_library_function`: Invokes a function located at a specific offset within a library base retrieved from a global variable.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_task_dispatch_function`: Calls the exec Dispatch function through the stored library base, passing no arguments.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_through_pointer`: Writes the low byte of the first data register to the memory location held in the A3 pointer register.
- `compare_string_prefixes`: Checks if a2 is null or certain, then checks if a byte at a2 plus 20 is '2'. If so, it compares a string at a2 with a global string, potentially padding with dots if a null terminator is found.
- `write_dotted_blank_field`: Writes a dot followed by four spaces into a text buffer at a fixed offset from its start.
- `match_prefix_against_release_text`: Advances through a text buffer while it matches the global release text byte by byte, returning when the release text ends or a byte differs.
- `empty_routine`: The routine is empty and performs no observable operation before returning.

### Version

- `parse_command_line_string`: Parses a string into individual tokens, handling quoted substrings and converting specific character sequences into newline or escape characters.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `report_library_version_and_usage`: Parses the command arguments and prints the version of a named library or device, together with the Kickstart and Workbench versions, or a usage message.
- `forward_two_arguments_in_sequence`: Passes its first argument with a fixed numeric code to one routine, then passes its second argument to another routine.
- `parse_signed_decimal`: Reads an optional plus or minus sign followed by decimal digits and returns the signed integer value.
- `compare_strings_2`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Counts the bytes of a null-terminated string up to, but not including, its terminating zero, and returns the count.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatch_function`: Calls a function through a pointer stored at an offset within a global table.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_function_offset_c6`: Calls a function using an offset from a pointer stored in a global table.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `execute_indirect_function`: Fetches a function pointer from an offset within a global table and executes it.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_indirect_function`: Fetches a function pointer from a location offset from a global variable and executes it.
- `invoke_and_checksum_stack`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a null-terminated buffer on the stack.
- `store_byte_to_pointer`: Stores a byte from a register into a location pointed to by another register.

### SetMap

- `parse_command_line_arguments_2`: Parses a command line string into individual tokens, handling quoted substrings and converting specific escape sequences into newline or escape characters.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_indirect_function_2`: Calls a function pointer stored at an offset from a global variable.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `reschedule_tasks_via_exec`: Calls the exec Reschedule routine through the stored library base, passing no arguments.
- `walk_saved_table_backwards`: Walks a table of saved pointers from the last entry backwards and passes each non-null entry to another routine.
- `save_arguments_and_run_setup`: Saves its two arguments into global fields, then calls another routine with a fixed set of numeric values, and finally calls it again with a fixed code.
- `load_keymap_from_arguments`: Reads a keymap identifier from the arguments or the icon tool types, opens the keymap resource, and loads the named keymap file from the keymaps directory.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_indirect_function_3`: Executes a routine located via a pointer within a global table.
- `call_indirect_function_4`: Performs an indirect call to a function address retrieved from a global variable.
- `call_indirect_function_5`: Jumps to a subroutine address found by offsetting a global pointer.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_indirect_function_6`: Invokes a function by dereferencing a pointer stored in a global variable.
- `call_indirect_function_7`: Redirects execution to a routine address held in a global data structure.
- `call_indirect_function_8`: Calls a subroutine through a pointer located at a specific offset in a global variable.
- `call_indirect_function_9`: Uses a pointer from a global variable to call another routine.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_function_10`: Executes a function through an indirect pointer stored in a global variable.
- `call_indirect_function_11`: Calls a routine by accessing a pointer within a global data structure.
- `call_library_vector_no_args_2`: Calls a library function through a global base at a fixed negative offset, passing no arguments.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_global_with_stack_slot`: Forwards a global value and its argument to another routine together with a pointer to a stack slot.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `invoke_and_checksum_stack_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over the local stack buffer until a null terminator is reached.
- `store_byte_indirect`: Stores a byte from a register into a memory location pointed to by another register.

### More

- `parse_command_string`: Parses a command string by splitting it into tokens and handling quoted substrings.
- `update_and_cleanup_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `page_through_file_in_window`: Displays a file one page at a time in a new window, showing how far through it the reader is and a quit prompt.
- `iterate_table_with_function`: Calls another routine four times, passing a value from a structure field together with each of four entries in a fixed global array.
- `validate_input_buffer`: Checks up to eighty bytes of a buffer against a set of accepted byte ranges.
- `wait_and_read_key_code`: Waits for an input signal with a timeout and returns zero when a termination flag is set.
- `execute_page_routines`: Checks if a value at a2 plus 60 is positive, then calls a routine using a2, calls another routine using a2, and jumps back to the start.
- `call_two_routines_then_jump`: Calls one routine with a global value and another with no arguments, then makes an indirect call through a pointer.
- `initialize_and_run_page_loop`: Sets two state fields to their starting values, calls a routine to set up a result, then repeatedly calls another routine while a counter is positive, the position is before the end and a stop flag is clear.
- `update_entity_and_increment_counter`: Calls two routines on a structure's buffer, stores the second result in a field, and when a flag field is zero it increments a short counter reached through a pointer in the structure.
- `scan_buffer_for_line_end`: Scans a buffer up to its stored size, stopping at a newline or form-feed character, and records how many bytes were scanned.
- `run_pager_display_loop`: Repeatedly fills and draws up to nine to fourteen lines of text, reads a key, and stops when the key is q or the control-C code.
- `run_search_with_prompt`: Chooses one of two global strings from a flag, then runs a search step and loops while the result is zero, updating a counter.
- `run_handler_sequence_and_return`: Runs a fixed sequence of calls to another routine, passing a global and values from a parameter block, and returns the result of its main call.
- `call_then_conditional_follow_up`: Calls another routine with two arguments, then calls it again with one argument only when a short field in the parameter block is zero.
- `call_two_routines_with_bounds`: Calls one routine with a stored value and a fixed negative argument, then calls another routine with a field from a nested structure.
- `call_routines_with_range_end`: Calls one routine with a stored value, a zero and a one, then calls another with a field from a nested structure reduced by one.
- `iterate_text_buffer_lines`: Iterates over a text buffer line by line, counting lines and stopping at a line limit, a stop flag or a special character.
- `call_handler_then_forward_pointer`: Calls another routine with its second argument, then calls a further routine with a pointer taken from the structure together with a global value.
- `set_flag_and_forward`: When a field in the parameter block is zero, sets a short value in a linked structure to one.
- `select_and_forward_by_flag`: Chooses between two values depending on a flag in the parameter block, then calls another routine with a field from the block and the chosen values.
- `dispatch_handler_by_flag`: Calls another routine with a field from the block and either three copies of a global value or a field from a nested structure, depending on a flag.
- `clamp_negative_result`: Calls another routine twice with the same first argument, once with a flag of one and once with minus one, and returns the second result clamped to zero if it is negative.
- `consume_from_buffer`: Takes up to a requested number of bytes from a buffer whose size is recorded in a structure.
- `refill_buffer_to_full_size`: Moves any buffered bytes to the start of the buffer when it is full, resets the offset fields, then fills the remaining space of a 4096-byte buffer by calling another routine.
- `update_scroll_window_position`: Computes a target position from a selector and an offset, using the window's start, end and length fields.
- `read_line_from_buffer`: Reads characters one at a time from a source into a buffer, stopping after a newline or a limit of 127 characters, and adds a terminating zero.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination, including the terminating zero, and stops after copying the zero.
- `convert_string_to_uppercase`: Converts each lowercase ASCII letter in a null-terminated string to uppercase in place, leaving other bytes unchanged.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated string, accepting an optional leading sign and then decimal digits.
- `call_library_function_when_set`: Calls a library function with a fixed argument of minus one, but only when the given pointer and a stored base pointer are both non-null.
- `close_pager_window_and_cleanup`: Cleans up the pager window state by calling routines that release saved pointers and reset fields, reads characters until a newline code, and then re-enables and reschedules through library calls.
- `send_single_value_message`: Stores its argument in a one-element stack buffer and passes it, with a fixed numeric code, to a routine that uses a handle taken from a stored structure.
- `copy_block_fields_and_release`: Allocates a 36-byte block and passes it to a routine with a fixed code, then copies two fields from the block into the caller's structure and frees the block.
- `initialize_object_from_template`: Obtains a handle from a routine, allocates a 68-byte object, links its fields to the handle and a source block, and copies a count of longwords from the source into it.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule_offset`: Calls a routine located at a specific offset from a global pointer.
- `invoke_task_switch_hook`: Calls the exec Switch function through the stored library base, passing no arguments.
- `execute_indirect_function_2`: Fetches a function pointer from a memory location and executes it.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `allocate_memory_via_exec`: Calls the exec AllocMem function through the stored library base, passing no arguments from this code.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_indirect_function_12`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_3`: Fetches a function pointer from a global memory location and executes it.
- `execute_indirect_function_4`: Fetches a function pointer from a global table at a specific offset and executes it.
- `execute_indirect_function_5`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_indirect_pointer`: Executes a function pointer retrieved from a global variable with a specific offset.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_indirect_pointer_2`: Calls a subroutine located at an offset from a global variable.
- `call_indirect_pointer_3`: Performs an indirect call to a function address stored in a global table.
- `call_indirect_pointer_4`: Invokes a routine by dereferencing a pointer found at a global offset.
- `call_indirect_pointer_5`: Triggers a function call using an address stored within a global data structure.
- `call_indirect_pointer_6`: Calls a subroutine via a pointer located at a specific offset from a global variable.
- `call_indirect_pointer_7`: Executes a function through a pointer stored in a global variable.
- `forward_with_stack_slot_2`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `read_signed_byte_or_fail`: Reads one byte through a routine that takes a handle, a buffer and a count of one.
- `call_hook_then_measure_string`: Calls a function pointer loaded from a global table, then counts the characters of a stack buffer up to its terminating zero byte.
- `store_byte_through_pointer_2`: Writes the low byte of the data register to the memory location held in the address register, then returns.
- `divide_32bit_with_remainder`: Divides a 32-bit dividend by a 32-bit divisor and returns the quotient and remainder together.
- `multiply_32bit_integers`: Multiplies two 32-bit values using 16-bit partial products and returns the 32-bit product.
- `stub_self_call_gap`: Calls its own entry point and then returns, with no condition and no other work.
- `call_helper_return_register`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag_2`: Handles the signs of two values, calls a helper, and negates the result when the flag the helper leaves in an address register is nonzero before returning.
- `negate_result_on_flag`: Calls another routine and returns its result unchanged, or negated when a second value it leaves in an address register is non-zero.
- `call_global_table_entry`: Calls the function whose address is stored at a fixed offset inside a global data block, then returns.
- `store_byte_via_pointer`: Stores the contents of a data register into a memory location pointed to by an address register.
- `call_library_vector_276`: Loads a library base pointer from a global variable and calls the library function at a fixed negative offset from that base, returning its result.
- `create_message_port`: Allocates a signal bit, then allocates and initialises a message port structure with its type, signal bit and task fields set.
- `delete_message_port`: Removes a message port from its list when a linked field is set, marks its type and signal fields as deleted, frees the signal bit and releases the port's memory block.
- `find_pattern_in_text`: Searches a null-terminated text for a non-empty pattern string by comparing the pattern at each starting position.
- `initialise_list_header`: Sets up an empty list header in the given block: the first pointer refers to the field after it, the middle field is cleared, and the last pointer refers back to the header itself.

### Format

- `parse_command_line_and_run`: Program entry point.
- `cleanup_program_and_reply`: Stores a result code in the control block, then closes the open file handle when present, frees the memory block, closes the library and replies to the owning message.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `allocate_memory_block`: Calls the exec memory allocation function through a stored library base pointer, passing its arguments through and returning the address of the block.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `invoke_task_switch`: Calls the exec task-switch function through the stored library base, which lets other ready tasks run.
- `forbid_multitasking`: Calls the exec Forbid function through the stored library base, which stops task switching until multitasking is permitted again.
- `dispatch_task_queue`: Calls the exec dispatch function through the stored library base, which selects the next task from the ready queue to run.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `schedule_next_task`: Calls the exec scheduler through the stored library base to choose the next ready task to run.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `allocate_absolute_memory`: Calls the exec absolute memory allocation function through a stored library base pointer, passing its arguments through and returning the result.
- `reschedule_tasks`: Calls the exec reschedule function through the stored library base, which lets the scheduler reconsider which task should run next.
- `run_format_disk_command`: Main routine of the disk format command.
- `free_format_buffers`: Frees each of several format buffers that are still allocated, using their recorded sizes and clearing their pointers, then calls a final routine with the given argument.
- `translate_disk_error_code`: Runs a disk operation, retrying while the device reports one specific busy code, then classifies the returned error code and displays the matching message text.
- `match_byte_in_table`: Scans an eight-entry byte table for the given value and records the matching entry in a field of a structure returned by a routine call.
- `find_drive_entry`: Walks a linked list of drive entries, matching either by a name string or by a numeric identifier.
- `measure_cstring_length`: Counts the bytes that come before the terminating zero byte of a string and returns that count.
- `copy_cstring_to_buffer`: Copies a zero-terminated string byte by byte from the source into the destination buffer, including the terminating zero.
- `convert_char_to_uppercase`: Returns the character in upper case when it is a lowercase letter from a to z, and returns any other character unchanged.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings one character at a time after passing each character through a case conversion routine.
- `terminate_string_at_colon`: Advances through a string until it reaches a colon or the end, then writes a zero byte at that position so the string is cut at its first colon.
- `check_single_argument_result`: Stores one argument value in a small local array, passes it to a lookup routine with a fixed code, and returns a failure code when the lookup fails or a success code otherwise.
- `build_request_block`: Allocates a request block, sets up its self-linked list header, stores a pointer and a value, and copies up to seven words of arguments into it.
- `call_indirect_pointer_8`: Performs an indirect jump to a routine address stored in a global variable.
- `call_indirect_pointer_9`: Calls a function by dereferencing a pointer at a calculated offset from a global variable.
- `call_indirect_pointer_10`: Executes a routine through a pointer retrieved from a global variable.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `execute_indirect_call`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_call_2`: Calls a subroutine located at an offset within a global data structure.
- `execute_indirect_call_3`: Performs an indirect jump to a routine stored in a global table.
- `execute_indirect_call_4`: Loads a pointer from a global variable and branches to that address.
- `execute_indirect_call_5`: Executes a function by dereferencing a pointer found in a global table.
- `execute_indirect_call_6`: Branches to a subroutine address retrieved from a global data structure.
- `execute_indirect_call_7`: Calls a routine via a pointer stored at a specific offset in a global variable.
- `execute_indirect_call_8`: Retrieves a function address from a global table and transfers control to it.
- `execute_indirect_call_9`: Executes a subroutine through a pointer located in a global data structure.
- `execute_indirect_call_10`: Performs an indirect call to a routine address stored in a global table.
- `call_indirect_pointer_11`: Calls a function pointer stored at an offset within a global table.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_indirect_pointer_12`: Executes a routine located at a specific offset from a global variable.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_indirect_pointer_13`: Performs an indirect call to a function address retrieved from a global table.
- `forward_call_with_stack_args`: Forwards its argument, together with a stored value and the address of a stack slot, to another routine and then returns.
- `call_indirect_pointer_15`: Executes a routine via a pointer found at an offset in a global table.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_and_verify_tracks`: Main disk formatting loop.
- `run_device_operation_sequence`: Issues a short sequence of device calls: two calls with fixed constant arguments, then passes its argument through a routine and forwards the routine's result to a final device call.
- `invoke_and_checksum_stack_3`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over the local stack buffer until a null terminator is reached.
- `store_byte_indirect_2`: Stores a single byte into a memory location pointed to by a register.
- `divide_32_bit_integers`: Performs 32-bit division using a shift-and-subtract algorithm or a 16-bit remainder method.
- `divide_32bit_restoring_loop`: Performs a sixteen-step shift-and-subtract division, subtracting the divisor from the high half whenever it fits and leaving the quotient bits in the low half.
- `multiply_32_bit_integers`: Computes the 32-bit product of two 32-bit values using 16-bit partial products.
- `call_adjacent_routine_and_return`: Calls one unnamed routine and returns, passing no arguments.
- `call_helper_return_register_2`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `perform_signed_division`: Normalizes the signs of two 32-bit integers, calls divide_32_bit_integers, and negates the quotient if the result in the address register is non-zero before returning.
- `negate_result_on_flag_3`: Calls another routine and returns its first result, negated when the second result register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_indirect_3`: Writes a byte from a register to an address held in another register.
- `allocate_and_initialize_block`: Calls a library function to allocate a memory block, fills in fields including a stored parameter and flag bytes, and calls further library functions, returning the block address or zero.
- `reset_object_and_recurse`: Recursively processes a linked structure when a pointer field is non-zero, sets a byte field to all ones and a four-byte field to all ones, then calls another routine on a byte field value and again with a constant second argument.
- `call_with_fixed_code_argument`: Forwards its argument to another routine together with a fixed constant code, then returns.
- `forward_argument_unchanged`: Passes its argument unchanged to another routine and then returns.
- `setup_trashcan_device`: Initialises the trash-can device for the command environment by storing default values in global fields, taking a copy of a process field around a library call, and calling a routine with the trash-can path text.
- `read_one_byte_or_fail`: Calls a routine to read one character into a one-byte buffer and returns that character as a signed value when the call succeeds, or minus one otherwise.
- `call_indirect_pointer_14`: Calls a subroutine using a pointer retrieved from a global data structure.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `allocate_node_with_fields`: When the first pointer is non-null, allocates a node through a routine and, if that succeeds, stores a byte value, a word argument and the pointer into fixed offsets of the node.
- `reset_node_and_release`: If the pointer is non-null, sets the node's status byte to all ones and clears two longword fields, then passes the node and a stored word to another routine and returns that result.
- `execute_indirect_function_6`: Fetches a function pointer from a memory location and executes it.
- `initialise_structure_from_table`: Calls the exec structure initialisation routine through the stored library base, which fills a memory structure from an initialisation table.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_string_2`: Parses a command line string into individual arguments, handling quoted strings and expanding specific character sequences.
- `call_supervisor_routine`: Calls the exec Supervisor function through the stored library base, which runs a routine in supervisor mode.
- `call_exit_interrupt`: Calls the exec ExitIntr function through the stored library base, which returns control from an interrupt handler.
- `call_switch_2`: Calls a function pointer located at a specific offset within a global structure.
- `execute_indirect_function_7`: Fetches a function pointer from a global table and executes it.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `make_library_functions`: Calls the exec MakeFunctions routine through the stored library base, which installs the function vector table of a library, and returns its result.
- `find_resident_module`: Calls the exec FindResident routine through the stored library base, which searches the resident module list for a module by name.
- `enable_multitasking`: Calls the exec Enable function through the stored library base, which re-enables interrupts after a matching disable call.
- `allocate_entry_table`: Calls the exec AllocEntry function through the stored library base, which allocates a table of memory entries and returns it.
- `start_cli_session`: Starts a command-line shell session.
- `build_node_chain`: Walks a linked list of descriptors, allocating a small node for each one, copying a value from each descriptor into its node, and linking the nodes together.
- `free_node_chain`: Walks a chain of nodes, releasing the resource stored in each node and then the node itself, until the chain ends.
- `parse_signed_decimal_3`: Reads a signed decimal number from a string by skipping an optional leading plus or minus sign and accumulating digits until a non-digit character, then applies the sign to the result.
- `execute_indirect_function_8`: Fetches a function pointer from a table using a fixed offset and executes it.
- `execute_indirect_function_9`: Fetches a function pointer from a memory location using an offset and executes it.
- `execute_indirect_function_10`: Fetches a function pointer from a specific offset within a global data structure and executes it.
- `execute_indirect_function_11`: Fetches a function pointer from an offset within a global structure and executes it.
- `execute_indirect_function_12`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_offset_function`: Calls a subroutine located at a specific offset from a global pointer.
- `invoke_pointer_offset`: Executes a routine by dereferencing a pointer found at a fixed offset from a global address.
- `run_offset_routine`: Performs an indirect jump to a function address stored at a specific offset within a global structure.
- `modular_inverse_32bit`: Performs a 32-bit modular reduction or a 16-bit modulo operation using repeated doubling and subtraction, returning a combined 32-bit result.
- `multiply_32_bit_integers_2`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `call_adjacent_routine_and_return_2`: Calls one unnamed routine and returns, passing no arguments.
- `call_helper_return_register_3`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_modular_inverse_result`: Normalizes the signs of two input values, calls modular_inverse_32bit, and negates the returned value if the address register contains a non-zero result before returning.
- `negate_on_flag`: Calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.

### Disk-Validator

- `validate_disk_bitmap`: Entry point of the disk validation tool.

### Ram-Handler

- `execute_indirect_jump`: Performs an indirect call through a pointer stored at an offset from a global variable and then executes an unrecovered jump.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_BLIT

- `reset_blitter_state_entry`: Runs a fixed sequence of routine calls and a library call in a loop, then clears a memory word.
- `busy_wait_countdown`: Busy-waits by decrementing a 16-bit counter from its starting value until it reaches minus one, then returns.
- `decompress_bitstream_passes`: Decodes a bit-packed compressed stream over seven passes: it reads variable-length codes, copies runs of earlier output, and adds values in place.
- `stub_forward_or_zero`: Two-instruction wrapper that calls a one-instruction routine merging zero into a register and returns its result.
- `decode_packed_stream_copy`: Second decoding stage of the same packed format: it reads codes from a bit buffer, copies or repeats bytes from earlier output into the destination, and returns when the stream is exhausted.
- `push_and_branch`: Modifies a byte in d0, then pushes d0, d1, and d2 onto the stack before restoring all used registers and branching to another routine.

### d

- `stub_bad_instruction_data`: The bytes do not decode to a valid routine.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_CPU

- `initialize_cpu_state_entry`: Runs a sequence of routine calls and a library call in a loop, then clears a memory word.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `update_array_with_loop_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 6, calling a routine if a certain condition is met.
- `update_array_with_loop`: Checks if a value in d0 is negative, otherwise it calls another routine and, if d0 is not zero, enters a loop that adds a value from a pointer in a3 to a table and decrements a counter until it reaches zero.
- `branch_and_call`: If a zero flag is set, it branches to a target; otherwise, it moves a3 to a1, calls a routine, and branches to a target.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `expand_bitstream_sequence_2`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting exp.
- `expand_bitstream_loop`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `set_constants_and_branch`: Sets d0 to 5 and d7 to 3, then branches to another location.
- `set_constants_and_branch_2`: Sets d0 to 4 and d7 to 2, then branches to another location.
- `set_constants_and_branch_3`: Sets d0 to 3 and d7 to 1, then branches to another location.
- `expand_bitstream_sequence`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or or compressed data to a destination.
- `modify_d1_and_branch`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `expand_and_copy_bitstream_2`: Reads a bitstream-encoded sequence where certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or decompressed byte sequence to a destination buffer.
- `expand_and_copy_bitstream`: Reads a bitstream or compressed-style bit-count-based sequence, expanding or skipping data based on a bit-mask, and copies the resulting bytes from a source pointer to a destination pointer while decrementing a counter in d0.
- `push_data_and_branch`: Pushes bytes from d0, d1, and a word from d2 onto the stack at the location pointed to by a4, restores several registers from the stack, and branches to another location.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_d6_to_negative_one`: Decrements the value in register D6 until it reaches negative one and then returns.
- `update_a1_from_a3_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 4, calling a routine if a certain condition is met.
- `update_array_elements_via_pointer`: Calculates a count from a register, calls a function via a pointer, and then enters a loop that iterates through a table starting at a pointer in a3, adding a value from a1 to each element in the table.
- `branch_and_call_2`: If a zero flag is set, it branches to a target; otherwise, it moves a3 to a1, calls a routine, and branches to a target.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop_2`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `expand_bitstream_and_copy_data`: Reads a variable-length integer from a bitstream in a0 and uses its value to determine how many bytes to copy from the bitstream to a destination buffer in a3.
- `expand_bitstream_loop_2`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `initialize_registers_and_branch`: Loads the value 5 into register d0, loads the value 3 into register d7, and performs a short branch to another location.
- `initialize_registers_and_branch_2`: Loads the value 4 into register d0, loads the value 2 into register d7, and performs a short branch to another location.
- `initialize_registers_and_branch_3`: Loads the value 3 into register d0, loads the value 1 into register d7, and performs a short branch to another location.
- `decode_variable_length_integer`: Reads a variable-length integer from a buffer in a0 by repeatedly shifting and adding bits, using a carry-based expansion method to determine a value.
- `modify_d1_and_branch_2`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `decompress_variable_length_data`: Reads a variable-length integer from a buffer in a0 to determine a count, then enters a loop to decompress data by reading bit-streamed values and copying bytes from a source buffer to a destination buffer pointed to by a3.
- `decode_variable_length_integer_and_copy`: Reads a variable-length integer from a buffer in a0 by processing a control byte and its subsequent bits, then uses the resulting value to copy a block of data from a source address to a destination address in a3.
- `push_data_and_branch_2`: Pushes bytes from d0, d1, and a word from d2 onto the stack at the location pointed to by a4, restores several registers from the stack, and branches to another location.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon_install

- `initialize_game_state`: Runs a sequence of routine calls and a library call in a loop, then runs a small loop of function-pointer calls before clearing a memory word and swapping a table entry with a RAM word when it matches.
- `decrement_register_until_negative_one`: Decrements a 16-bit register value until it reaches negative one and then returns.
- `modify_table_elements_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 3, calling a routine if a certain condition is met.
- `update_table_with_offset`: Doubles d0 and branches if the result is negative; otherwise, it calls another routine, sets d1 to d0, and if d1 is not zero, calls another routine, calls a function at a4, calls another routine, and enters a loop that adds a value from a1 to a table at a3 using an offset calculated from d0 and d2, decrementing d1 until it reaches zero.
- `transfer_pointer_and_call`: If a condition is met, it moves the value from a3 to a1, calls another routine, and then branches back to the initial condition check.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes`: Compares a6 and a2, then if they are not equal, copies four bytes from the source pointed to by a2 to the destination pointed to by a1 using pre-decrement on the source and post-increment on the destination.
- `decompress_bitstream_data_2`: Reads a control byte from a source pointer to determine a repetition count and a bit-length, then performs bit-level shifts and additions to reconstruct values from a bitstream.
- `process_bitstream_loop`: Decrements a counter in d7 and loops back to the start of the routine if not zero, otherwise it performs arithmetic on d0 and d1 using a byte from a pointer in a0 and then increments d0 by 6 before branching to another routine.
- `initialize_bitstream_parameters`: Sets d0 to 5 and d7 to 3, then branches to another routine.
- `initialize_bitstream_parameters_2`: Sets d0 to 4 and d7 to 2, then branches to another routine.
- `initialize_bitstream_parameters_3`: Sets d0 to 3 and d7 to 1, then branches to another routine.
- `expand_bitstream_data`: Reads a bitstream from a source pointer in a0, performing bit-level shifts and additions to reconstruct values, and uses a table to determine repetition counts and bit lengths for decompression.
- `process_bitstream_data`: Adds the byte in d1 to itself and branches if the result is not zero; otherwise, it loads a byte from the address in a0 into d1, adds d1 to itself with carry, adds d2 to itself with carry, and performs a short branch.
- `decompress_bitstream_data`: Performs bit-stream decompression by reading a control byte to determine bit-length and repetition counts, then uses a table to perform LZ-style dictionary copies or literal byte transfers into a destination buffer.
- `decompress_and_copy_data`: Reads a control byte from a source pointer to determine a repetition count and a bit-length, then performs a bit-stream decompression to expand data into a destination buffer.
- `push_registers_to_stack`: Stores a byte from d0, a byte from d1, and a word from d2 onto the stack using a4 as a pointer, restores several registers from the stack, and branches to another routine.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `compare_and_call_routine`: Compares a longword at the address in a0 with a constant value; if they match, it branches to another routine, otherwise it loads a value into d0 and returns.
