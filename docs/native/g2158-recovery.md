# Game 2158 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02158). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905232 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 665 of 665 named (0 author symbols), 665 of 665 described (69 by rule, 461 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1001 of 1001 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 17550 of 17563 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 71290 |
| code | 56284 |
| data | 482598 |
| padding | 14996 |
| unused | 280064 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Keymap | 1 | 3 | 1 |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_BLIT | 11 | 252 | 6 |
| d | 1 | 3 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_CPU | 10 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon | 8 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon_install | 7 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1041 | 32 |

## Cases handled

- runtime segment an address (play-rs-m1500): 543 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Keymap

- `empty_routine`: This routine is empty and contains no instructions.

### Run

- `find_max_and_prepare_buffer_3`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: When its pointer argument is not null, copies sixteen long words from a fixed block into it, then calls the routine pointer held in the address register.
- `update_table_entry_and_copy_range_2`: Updates a structure or table entry with a new pointer and range-end value, and then copies a sequence of bytes from a source buffer to a destination buffer, terminating the copy with a specific byte value.
- `update_table_entry_and_copy_range`: Reads a byte from a table using an index stored in a structure, updates a pointer and a range-end value in that structure, and then enters a loop to copy a sequence of bytes from a source table to a destination table, calling another routine during the process.
- `update_structure_and_branch_offset_2`: Updates a structure or table by calculating a new range, then iterates through a loop to copy bytes from a source table to a destination table using an offset-based index, and finally updates a pointer or value at a specific offset in the structure.
- `update_structure_and_branch_offset`: Updates a table or structure using an offset-based index, copies certain longwords between structure members, and then enters a loop that checks a value against a counter, potentially calling another routine and updating further structure members or a global variable.
- `calculate_branch_offset`: Sets d1 to one, adds a value from a structure offset to d1, and branches to a relative offset.
- `test_structure_member_bits`: Tests if a longword at offset 48 of the structure in a1 is zero, otherwise it branches to a different routine or jumps to another routine.
- `mask_structure_bits_and_compare`: Checks if certain bits are set in a structure member at offset 48 of a1, and if not, sets the upper 24 bits of that member to all ones.
- `update_structure_pointer_and_clear_table`: Updates a pointer in a structure, calls another routine, and if the new pointer value is non-zero, proceeds to update a table entry, reset several fields in a structure, and conditionally decrement a value in a table or array based on a pointer check.
- `update_structure_pointers_2`: Updates a pointer in a structure, calls a routine, and conditionally clears or decrements a value in a table or array based on a pointer check.
- `update_structure_pointers`: Updates a pointer in a structure, calls a routine, and conditionally clears or decrements a value in a table or array based on a pointer check.

### Install

- `find_max_and_prepare_buffer`: Walks a linked list to find the largest value stored at an index field.
- `copy_words_then_call_2`: Copies a fixed number of words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.
- `load_and_check_pointer`: Loads an address into a1, calls a library routine, and if d0 is not zero, loads a pointer from a structure in a0 into a0 and then returns.
- `set_d0_and_branch`: Sets d0 to a constant and branches to a fixed address.
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

- `find_max_and_prepare_buffer_2`: Same logic as the install variant: walks a linked list for the largest value at an index field, fills a buffer and copies a sixteen-long block when a library call succeeds, and returns zero or minus one.
- `copy_sixteen_longs_then_call_2`: Copies sixteen 32-bit values from a fixed offset in a base record into a caller-supplied buffer when that pointer is non-null, then calls through a stored handler pointer.

### Delete

- `allocate_table_by_max_entry`: Walks a linked list to find the largest value stored in each entry's field, then asks a service routine for a buffer.
- `copy_buffer_and_call_2`: Copies a fixed block of words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.
- `update_table_index_and_traverse_2`: Calculates a new index or pointer for a table in a1, clears certain fields in a1 and a2, and calls another routine to check or update a secondary table or list, potentially entering a loop that traverses or updates entries based on values read from a table in a0.
- `update_list_index_from_table_2`: Reads a value from a table using an offset, calculates a new index, and stores two bytes or a word into a structure at offset 544 and 548, then branches to another routine.
- `execute_two_subroutines_with_offset`: Increments a value in a structure pointed to by a1, then calls another routine using a pointer from a4, and repeats this process using a different value from a1, before branching back to a previous instruction.
- `process_list_with_conditional_call`: Increments a value in a structure, then checks if a certain value in a list or a global variable matches a certain condition, calling another routine if certain values are encountered or if a list-traversal condition is met.
- `validate_and_increment_list_index_2`: Checks if a value at offset 544 of a structure in a1 matches certain constants, and if so, branches to other routines; otherwise, it increments a value at offset 544 of the structure in a1 and continues execution.
- `update_table_entry_and_branch`: Increments a byte at an offset of 536 in a1, compares a longword from a2 to a longword in a1, and if a certain condition is met, loads a byte from a table using a scaled index and jumps to a routine in a6.
- `validate_index_check`: Tests a longword at an offset of 640 from a2 and branches if it is not zero, otherwise clears d1 and jumps to a routine in a6.
- `update_pointer_and_validate_index`: Increments a counter in a structure, then enters a loop that reads an index from a table and checks if that index or a subsequent byte matches certain constant values, updating a global variable if a match is found or calling another routine if certain conditions are met; otherwise, it continues or returns via an indirect jump.
- `branch_to_next_instruction`: Performs a short branch to a relative offset, bypassing any preceding or following instructions.
- `update_pointer_and_branch_conditionally`: Calculates a new index by adding a value from offset 548 of a structure in a1 to a constant, uses this to fetch a byte from a table in a0, and stores it in offset 552 of a1.
- `validate_and_update_index_table`: Checks if a value loaded from a structure offset against a list of prohibited or permitted constants, and if a match is found, updates a value in a structure and calls another routine; otherwise, it enters a loop that iterates through a table, updating structure members and calling another routine if certain byte values are encountered.
- `update_pointer_value`: Sets d1 to negative one, adds a value from a structure at offset 544 of a1 to d1, and branches to another routine.
- `jump_to_offset_2`: Performs an unconditional jump to a different location.
- `update_global_pointer`: If a certain long-word is not zero, it sets a global variable at offset 676 of a structure in a2 to a constant value and then jumps to another routine.
- `search_and_update_list`: Copies a 32-bit value from a structure in a2 to offset 4 of a1, then loops to check if a value at a certain offset in a0 matches a value in a1, updating a pointer in a1 if a match is not found.
- `update_and_store_pointer`: Increments a 32-bit value at an offset of a structure in a2, calculates a new address using a value from a structure in a2 and a pointer from a1, stores a value at that address, and jumps to an indirect address in a6.
- `set_index_to_max`: Compares a value from a structure in a2 with another value from a structure in a2, and if the first is not greater, sets a value at offset 624 of a2 to negative 255 and branches to another routine, otherwise it calls another routine.
- `update_index_and_load_byte`: Increments a 32-bit value at an offset in a structure, then loads a byte from a table into another offset in that structure using a calculated index.
- `execute_conditional_calls`: Compares a value in a structure to a constant and calls a routine twice if they match, otherwise calls the routine once and jumps to an indirect address.
- `copy_and_call_routine`: Copies a long-word from a structure in a2 to a pointer in a1, copies another long-word from a structure in a2 to an offset of a1, modifies a pointer in a4, and calls another routine.
- `set_global_to_max`: Sets a 32-bit value at an offset in a structure to a constant value and jumps to an indirect address.
- `execute_two_routines`: Calls a routine twice with different pointer offsets in a4 and jumps to an indirect address.
- `update_pointer_and_set_value`: Calls a routine, updates a pointer, and if a certain value in a structure does not match a constant, sets a 32-bit value in a structure to a negative constant before calling another routine.
- `branch_on_index_value`: Checks a value from a structure in a1 against several constants, calling other routines or setting a value in a structure in a2 if certain conditions are met, and finally jumps to an indirect address.
- `update_structure_member_on_condition`: Clears a 4-byte value at an offset of 4 from a pointer in a1 and calls another routine, then checks if a value at offset 656 of a structure in a2 matches certain constants; if certain conditions are met, it calls another routine, updates a pointer in a1, and performs a memory write or further routine calls before jumping to a routine in a6.
- `modify_and_store_byte`: Calculates an offset using a value from a structure in a2 and a pointer in a1, stores a byte from a structure in a2 into a memory location, copies a long-word from a structure in a2 to a pointer in a1, modifies a pointer in a4, and calls another routine.
- `load_and_call_routine`: Loads a value from a structure in a2 into d2, loads a value from a structure in a1 into d1, sets a4 to an offset of a4, and calls another routine before branching to another routine.
- `do_nothing_and_branch_2`: Does nothing and then branches to another routine.
- `traverse_and_replace_list`: A loop that replaces a value in a list with a byte from a table and updates a pointer, continuing until a null pointer is encountered.
- `branch_to_target_address_3`: Stores a long-word from d1 into an offset of a pointer in a1, then if d1 is not zero, jumps to another routine; otherwise, it overwrites d1 with d2 and jumps to an address in a6.
- `branch_to_subroutine_middle`: Performs a short branch to a location inside another routine.
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

- `allocate_control_record_and_wait`: Allocates a control record for the current task and takes the current directory lock and the task's message port.
- `release_task_resources_and_reply`: Finds the current task and stores the supplied value in its record.
- `sum_region_sizes_matching_mask`: Walks a linked list of memory regions and adds up the size of each region whose attribute bits match the given mask.
- `print_memory_usage_table`: Prints the memory usage table with available, in-use, maximum and largest figures for chip, fast and total memory, gathered through exec library calls.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_pair_with_stack_pointer`: Passes its two parameters on to another routine together with the address of its own stack slot, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `format_and_output_text`: Formats text into a 140-byte stack buffer with the system formatter, measures the resulting string, and passes any non-empty text with its length to a write routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `parse_and_add_volume_buffers`: Finds the device named by the DRIVE argument and adds the number of buffers given by BUFFERS. It prints the resulting buffer count, or reports an invalid device or volume name.

### Path

- `find_max_and_prepare_buffer_13`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen long words into a buffer before invoking a routine pointer.
- `copy_buffer_block_3`: Copies a fixed block of sixteen words from a source table into a destination buffer when the destination pointer is valid, then invokes another routine.

### Resident

- `find_max_and_prepare_buffer_14`: Walks a linked list of entries to find the maximum value in a field, then stores that maximum along with a small header block into a buffer.
- `copy_words_then_call_4`: Copies a fixed number of words from a source table into a destination buffer when the destination is not null, then calls another routine.
- `validate_and_update_structure_pointer`: Calculates a new pointer or index, updates a field in a structure, and calls several subroutines to check or update further structure members; if certain conditions are met, it checks a bitmask in a table and updates a field before performing an indirect jump or calling another routine.
- `set_zero_and_jump_2`: Sets register d1 to zero and jumps to an address in a6.

### Assign

- `find_max_and_prepare_buffer_15`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes a small header block and copies sixteen long words into a target buffer before invoking a routine pointer.
- `copy_buffer_and_call_3`: Copies a fixed block of sixteen words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.
- `update_linked_list_node_and_table_2_2`: Calculates an index-based pointer, updates a structure or table with new bounds or pointers, and traverses a linked list or table-based chain, updating node pointers and checking for end-of-list or invalid index conditions, calling another routine to perform certain updates or checks during the traversal.
- `update_linked_list_node_and_table_2`: Updates a linked list node by copying a pointer from one index to another, then enters a loop that traverses and updates subsequent nodes in.
- `update_linked_list_node`: Updates a pointer in a structure or list, checks a condition or flag, and if certain criteria are met, updates a value in a structure or list to a constant or a value from another structure, then calls another routine.
- `update_linked_list_node_and_table`: Updates a linked list node by copying a pointer from one index to another, then enters a loop that traverses and updates subsequent nodes in the list.
- `assign_table_values`: Updates several values in a table using offsets derived from a structure, calls another routine multiple times to check or clear certain flags, and finally jumps to an address in a6.

### EndCLI

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen long words into a target buffer before invoking a routine pointer.
- `copy_block_and_call`: Copies a fixed block of words from a source table into a destination buffer when the destination is not null, then invokes another routine.
- `modify_list_and_table`: Modifies a bitmask in a structure, updates a pointer in a structure, and conditionally decrements a value in a table if it is greater than zero.

### Rename

- `find_max_prepare_buffer`: Walks a linked list of entries to find the largest value in a field, then, when a condition is met, writes a small header block and copies sixteen long words into a target buffer before invoking a routine pointer.
- `copy_words_and_call`: Copies a fixed number of consecutive words from one memory location to another when the destination is not null, then invokes another routine.
- `update_pointer_and_call`: Updates a pointer in a structure, calls a routine, and conditionally updates a value in a structure or table.

### Dir

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value in a field, then writes a small header block containing that value and a sequence of incremented words.
- `copy_words_and_call_2`: Copies a fixed count of words from a source table into a destination buffer when the destination is not null, then invokes another routine.
- `update_entity_bounds_and_state`: Updates several pointer-based offsets in a structure in a1 to define a memory range, clears certain fields, and conditionally updates a structure in a2 with values from a1 or a table, then calls another routine or returns.
- `update_structure_field_from_registers`: If a longword at offset 332 of the structure in a1 is non-zero, it loads three values from offsets 88, 48, and 48 (the same offset) into d3, d2, and d1, then calls another routine using a pointer-adjusted a4, and finally writes the value from d1 into offset 320 of the structure in a1 before branching away.
- `clear_structure_member_if_set`: Tests if a longword at offset 320 of the structure in a1 is zero, otherwise it clears that longword to zero and jumps to another routine.
- `update_entity_property_and_loop`: Updates a field in a structure using a value from another structure, then calls a routine multiple times with different offsets to update or check further fields, and finally loops through a range of indices to clear or set values in a table, or otherwise returns.
- `update_entity_state_and_pointer_2`: Checks a value in a table, and if it equals two, calls a routine to update a field in a structure, then updates a byte in a table using a pointer.
- `update_entity_state_and_pointer`: Checks a value in a table, and if it equals two, calls a routine to update a field in a structure, then updates a byte in a table using a pointer, and subsequently calls several other routines to update or reset various fields in the structure, including updating a pointer-based index and a byte-based index, before finally branching to another routine.
- `update_structure_range_and_set_value`: Checks if certain pointers in a structure in a2 and a structure in a1 are non-zero, then if a certain value is not one, calls a routine, and if a certain flag in a2 is zero, initializes a range of values in a structure in a1 by looping and setting each entry to one, before calling another routine and updating a value in a2 from a structure in a1.
- `set_structure_value_with_offset`: Calculates a pointer or offset by adding a value from a structure in a1 to d1, shifts it right by two, and calls a routine using a pointer from a2.
- `set_structure_value`: Calls a routine, and if a certain value is zero, sets a value at an offset in a structure.

### NewCLI

- `find_max_and_prepare_buffer_6`: Walks a linked list of entries to find the maximum value in a field, then writes a small header block containing that value and a run of related data.
- `copy_buffer_and_call`: Copies a fixed 15-word block of data from a source table pointed to by a register into a destination buffer passed as an argument, repeating the copy only when that buffer is not null.
- `recalculate_bounds_and_copy_table_entries`: Updates a table of bounds or limits by recalculating or copying pointer-indexed values, and then performs a loop to copy a range of entries from one table or array to another, potentially incrementing a counter if a certain value is non-zero.
- `update_table_and_recalculate_bounds_2`: Updates a table entry by copying a value from one pointer-indexed location to another and increments a counter if it is non-zero, then calls another routine to perform further updates or resets on a structure, potentially involving a loop that searches for a zero-value entry in a table or updates a pointer-based bound or counter.
- `update_table_and_recalculate_bounds`: Iterates through a table, updating or resetting certain values, and if a certain condition is met, it updates a pointer or value in a structure and calls another routine multiple times.
- `branch_on_incremented_value`: Increments d1 by a value from a structure in a1 and branches to another routine.
- `update_table_and_decrement_logic_branch`: If a certain value in a structure is zero, it calls a routine three times with a constant value in d0 and a pointer from a table in a2; otherwise, it performs a series of pointer-based assignments and calls a routine multiple times, including a calculation that shifts a pointer-based address right by two bits before calling the routine again.
- `update_table_and_decrement_logic`: Checks if a value at an offset in a1 is below a certain threshold, and if so, performs several pointer-based assignments and calls a routine using a pointer from a2.
- `update_table_and_decrement`: Updates a value in a table, calls several routines, and conditionally clears or decrements a value in a table if a certain condition is met.

### NewShell

- `find_max_and_prepare_buffer_7`: Walks a linked list of entries to find the maximum value stored in a field, then, if a condition is met, writes a small header block and copies sixteen long words into a target buffer before invoking a routine pointer.
- `copy_buffer_block`: Copies a fixed block of sixteen words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.
- `update_structure_index_and_table`: Updates a structure's index and table by copying values from a source table to a destination table, and increments or decrements a counter in a global structure if a certain condition is met.
- `sync_structure_index_and_update_table`: Checks if a table entry or structure index is non-zero, copies a value from one table entry to another, and increments or decrements a counter in a structure.
- `update_structure_index_and_sync_values`: Checks if a table entry is zero and updates it with a calculated value, then enters a loop that updates a structure's index or clamps a value to 800, while calling another routine and performing several conditional updates to a structure and a table.
- `branch_on_incremented_value_alt`: Increments d1 by a value from astructure in a1 and branches to another routine.
- `sync_and_update_structure_value`: Checks if a value at offset 692 of a structure in a1 is zero; if not, it clamps a value at offset 276 of a1 to 800, synchronizes a secondary field at offset 268 with a value from offset 276, and calls a routine using a pointer from a2, then performs further updates to a table and structure using a pointer from a2 and a pointer from a4.
- `clamp_and_sync_structure_value`: Ensures a value in a structure does not exceed 800, synchronizes a secondary field with that value, and then executes a series of calls to update various structure members and table-based pointers or counters based on whether the primary value is zero or non-zero.
- `update_structure_and_table`: Updates a value in a structure, calls several routines, and conditionally clears or decrements a value in a table if a certain condition is met.

### Echo

- `find_max_and_prepare_buffer_8`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes a small header block containing that maximum and copies sixteen long words into a caller-provided buffer before invoking another routine.
- `copy_words_and_call_3`: Copies a fixed block of 15 words from a source table into a destination buffer when the destination is valid, then invokes another routine.
- `update_table_range_and_iterate_2`: Updates a range or pointer values in a structure by calling a routine to retrieve new indices, then enters a loop that iterates through a range of indices, updating a structure's range or pointer values.
- `update_table_range_and_iterate`: Updates a structure's range or pointer values by calling a routine to retrieve new indices, then enters a loop that iterates through a range of table entries, calling a routine at each step and updating a pointer or index in the structure.
- `update_list_index_from_table_3`: Loads a longword from a table at offset 336 of the structure in a1, writes it back to offset 328 of the same structure, and branches to a relative address stored in d4.
- `adjust_list_bounds_and_iterate`: Sets a longword at offset 328 in a1 to 1, checks certain values in a structure, and if certain conditions are met, calculates a new index and branches back to an earlier instruction or to another routine.
- `update_pointer_to_previous_value`: Copies a 32-bit value from an offset of 336 in a structure pointed to by a1 to an offset of 332 in the same structure, then branches to another routine.
- `update_list_bounds_and_iterate`: Updates two pointer-sized values in a structure, checks if a certain range is valid, and if so, iterates through a table to read a byte from an offset calculated from a value in the structure, then calls another routine and updates a value in the structure before branching.
- `check_structure_and_call`: Checks a value at an offset in a1, and if it is not zero, calls a routine using a pointer from a2.
- `conditional_routine_calls`: Checks a value in a structure and calls a routine or a sequence of routines if certain conditions are met.

### cls

- `write_clear_screen_sequence`: Opens dos.library and writes a fixed screen-clearing sequence to the output handle.

### Execute

- `find_max_prepare_buffer_2`: Walks a linked list of entries to locate the largest value in a field, then builds a small header block and copies a block of sixteen long words before invoking another routine.
- `copy_words_then_call`: Copies a fixed number of words from one table to another when the destination is not null, then invokes another routine.

### Copy

- `find_max_and_prepare_buffer_9`: Walks a linked list of entries to find the largest value in a field, then, when a condition is met, builds a small header block, fills sixteen long words from a fixed source, and stores a routine pointer before invoking it.
- `copy_buffer_block_2`: Copies a fixed block of sixteen words from a source table into a destination buffer when the destination is not null, then invokes another routine.
- `update_table_index_and_traverse`: Calculates a new index or size for a table entry in a structure, clears certain fields in a secondary structure, and then enters a loop that traverses a table of pointers, checking for specific byte-encoded command or jump values and calling another routine if certain conditions or specific values are met.
- `update_list_index_from_table`: Calculates a new index using a value from a table, retrieves two bytes from a data structure using this index, and stores these bytes into a structure at offset 544 and 548, then branches to another routine.
- `execute_conditional_table_updates`: Updates a byte at offset 536 in a1, then calls another routine using a pointer or offset or offset-based address from a4, and repeats this process with a different value or offset or offset-based address from a4 before branching back to the start of the routine.
- `update_list_index_and_traverse_table`: Increments a byte at offset 536 of a structure in a1, then calls another routine to check if a certain value exists in a table, and if a certain condition is met, calls another routine again before branching back to a previous instruction or exiting.
- `validate_and_increment_list_index`: Checks if a value at offset 544 of a structure in a1 matches certain constants, and if so, branches to other routines; otherwise, it increments a value at offset 544 of the structure in a1 and branches to a final instruction.
- `copy_range_to_structure`: Increments a value at an offset of 536 from a1, compares a value from a2 to that new value, and if it is not greater or equal, checks a byte from a table using a shifted offset from a1 and jumps to a routine in a6 if a certain condition is met.
- `check_long_word_and_jump`: Tests if a long word at offset 676 of a structure in a2 is zero, otherwise it jumps to a location that clears d1 and jumps to the address in a6.
- `update_index_table_and_validate_constants`: Increments a counter in a structure, then iterates through a list of offsets to check if the bytes loaded from a table match certain permitted or prohibited constants, updating a global value if a match is found or calling another routine if certain conditions are met; otherwise, it returns or jumps to another location.
- `branch_to_target_address`: Performs a short branch to a relative offset, effectively redirecting execution to a target address.
- `update_pointer_value_and_verify`: Updates a value in a structure at offset 552 using a byte read from a table, then compares this new value against a global variable at offset 700; if they match, it performs a secondary table lookup and calls another routine.
- `validate_and_process_id_list`: Checks if a value loaded from a structure offset against a list of allowed constants, and if a match is found, updates a value in the structure and calls another routine; otherwise, it enters a loop that iterates through a table, updating structure members and calling another routine if certain byte values are encountered.
- `add_offset_and_branch`: Sets d1 to one, adds a long word from offset 544 of a structure in a1 to d1, and then branches to a relative offset.
- `jump_to_offset`: Performs a relative jump to a different location in the code.
- `set_constant_d2`: Sets a 32-bit value in a structure at offset 424 of a2 to a constant and then jumps to another routine.
- `compare_and_update_pointer`: Compares a value from a structure in a1 with d1, and if certain conditions are met, calculates a new address using a value from a structure in a2 and calls a routine.
- `increment_and_copy_value`: Increments a value in a structure and copies a word from a source pointer to a destination pointer in a table.
- `compare_and_set_constant`: Compares a value from a structure in a2 with another value from a structure in a2, and if certain conditions are met, sets a value in a structure in a2 to a constant and branches.
- `increment_and_store_byte`: Increments a value in a structure and stores a byte from a source pointer into a destination pointer in a structure.
- `check_table_and_call`: Compares a value in a structure to a constant and calls a routine if they are equal.
- `copy_structure_data`: Copies a 32-bit value from a structure in a2 to a pointer in a1, copies another 32-bit value from a2 to an offset in a1, and calls another routine before jumping to another routine.
- `set_full_mask`: Sets a 32-bit value to all ones at an offset of 44 or 456 of a structure in a2, then jumps to an indirect address.
- `execute_two_subroutines`: Calls a routine using a pointer in a5, then calls it again with a different address, and finally jumps to an indirect address.
- `update_conditional_value`: Calls a routine, updates a pointer-based value, and if a certain value in a structure in a2 does not equal 41, sets a value at offset 704 of that structure to -1 before calling another routine and jumping to an indirect address.
- `branch_on_multiple_constants`: Compares a value from a structure in a1 against several constant values and branches to different locations or sets a value in a structure in a2 to a constant.
- `execute_subroutine_loop`: Clears a 4-byte value at an offset of 4 in a pointer-based structure, calls another routine, and then enters a loop that updates a 4-byte value at an offset of 8 in that structure, continuing or branching based on a comparison with a value in a structure in a2, before calling another routine and jumping to a location in a6.
- `update_pointer_and_store`: Calculates an offset using a value from a structure in a2 and a value from a structure in a1, stores a byte from a structure in a2 at an offset in a0, copies a 32-bit value from a2 to a1, and calls another routine before jumping to another routine.
- `load_parameters_and_call_subroutine`: Loads a longword from a global variable into d2 and a longword from a structure offset 8 into d1, sets d0 to a constant, updates a pointer in a4, and calls another routine before branching to a different code location.
- `do_nothing_and_branch`: Performs a no-operation and then branches to another routine.
- `copy_linked_list_bytes`: Loops through a list of bytes, copying a byte from a source table to a destination, and updates a pointer in the list, continuing until a null pointer is encountered, then jumps to an indirect address.
- `store_and_branch`: Stores a 32-bit value from d1 into a structure in a1, then jumps to a location in a6.
- `branch_to_address`: Branches to a fixed address.
- `copy_and_replace_bytes`: Loops through a list of bytes, copying a byte from a source table to a destination, and updates a pointer in the list, continuing until a zero byte is found, then copies a byte from a different offset and jumps to an indirect address.
- `clear_and_populate_list`: Clears several 32-bit values in a structure, loads a byte from a table using a pointer and an offset, and then loops to clear a range of bytes in a table.
- `execute_three_subroutines`: Calls a routine three times with different offsets of a4, then negates a value from a structure in a2 and jumps to an indirect address.
- `copy_and_sum_table_values`: Loads bytes from a table, stores them in a structure, and then loops to copy bytes from a table and add a byte to a table at an offset determined by a pointer.
- `add_and_store_byte`: Adds two values from a structure in a1 and stores the result as a byte into a table at an offset determined by a value in a1, then jumps to an indirect address.
- `copy_and_extend_list`: Copies a byte from a source table, then checks a value in a list, and if it is not greater than a certain value, calls a routine and copies further bytes from a source table to a destination list, continuing until a certain condition is met, then jumps to an indirect address.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `swap_and_store_pointer_2`: Calls a routine, sets a value in a structure, and if a certain value is zero, calls another routine twice and sets a pointer to a new address.
- `swap_and_store_pointer`: Copies a 32-bit value from a source table to a location in a structure in a2, then copies a value from a list into that source table, and finally updates a pointer in that structure, then jumps to an indirect address.

### Makedir

- `find_max_and_prepare_buffer_10`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen long words into a caller-provided buffer before invoking another routine.
- `copy_words_from_struct_buffer`: Copies sixteen words from a fixed offset in a global structure into a caller-provided buffer when that buffer is not null, then jumps through a stored pointer.
- `execute_conditional_logic_2`: Performs several routine calls and updates various offsets in a structure based on values retrieved from a source table.
- `execute_conditional_logic`: Performs several routine calls and updates various offsets in a structure in a1 based on values retrieved from a source table, including conditional logic that sets a value to 10 if a certain check fails, before jumping to an indirect address.

### List

- `find_max_and_fill_buffer`: Walks a linked list of entries to find the maximum value in a field, then writes that value and a small header into a buffer, fills a region with a repeating pattern, and copies sixteen long words before invoking another routine.
- `copy_words_then_call_3`: Copies a fixed block of words from a source table into a destination buffer when the destination is valid, then invokes another routine.
- `update_table_and_traverse_links`: Updates a structure's size or index field, then enters a loop that traverses a linked list or table of pointers, checking for specific byte-encoded command or jump values and calling another routine if certain conditions or specific command bytes are encountered.
- `update_list_index_and_cache_values`: Calculates a new index using a base offset and a value from a table, then reads two bytes from a structure at that calculated offset and stores them into a structure at offsets 544 and 548, before branching to another routine.
- `process_list_index_update`: Increments a byte at offset 536 in a1, then calls a routine to check a value at offset 540, and calls another routine to check a value at offset 548 before branching back to the start of the routine.
- `update_list_index_and_traverse_table_2`: Increments a byte at offset 536 of a structure in a1, then checks if a longword at offset 548 of that structure is non-zero; if so, it calls another routine and then checks if a longword at offset 548 is zero, calling another routine again before branching back to the start or exiting.
- `check_list_element_type`: Compares a value loaded from an offset in a structure to several constant values and branches to different subroutines if a match is found, otherwise it branches to a default target.
- `copy_range_to_structure_alt`: Increments a value at an offset of 536 from a1, compares a value from a2 to that new value, and if it is not greater or equal, checks a byte from a table using a shifted offset from a1 and jumps to a routine in a6 if a certain condition is met.
- `check_long_word_and_jump_alt`: Tests if a long word at offset 612 of a structure in a2 is zero, otherwise it jumps to a location that clears d1 and jumps to the address in a6.
- `validate_and_update_index_table_2`: Iterates through a list of indices, checking if certain bytes loaded from a table match a set of permitted or prohibited constants, and if a match is found, updates a value in a structure and calls another routine; otherwise, it continues or returns via an indirect jump.
- `branch_to_target_address_2`: Performs a short branch to a target address relative to the current instruction pointer.
- `update_pointer_and_branch_on_value`: Calculates a new index by adding a value from offset 548 of a structure in a1 to a base offset, retrieves a byte from a table in a0 using this index, and stores it in offset 552 of the structure in a1.
- `validate_entity_type_and_update_state`: Checks if a value at offset 552 of a structure in a1 matches certain prohibited or permitted constants, and if a match is found, updates a byte in a structure via a table lookup and calls another routine; otherwise, it enters a loop that iterates through a table, updating a global variable and calling another routine if certain conditions or certain byte values are met.
- `add_offset_and_branch_alt`: Sets d1 to one, adds a long word from offset 544 of a structure in a1 to d1, and then branches to a relative offset.
- `branch_to_target_address_5`: Performs a short branch to a relative offset or a fixed address in the code stream.
- `set_global_pointer`: Sets a global variable at offset 616 of a structure in a2 to a constant value, otherwise it returns.
- `update_and_store_pointer_2`: Sets a value in a structure, and then loops to compare a value from a table against a value in a structure, potentially updating a pointer and a table value.
- `increment_and_store_pointer`: Increments a value at offset 676 of a structure in a2, calculates an offset using a value from a structure in a2 and a pointer from a1, and stores a value from a1 at that calculated address before jumping to a target.
- `update_index_and_load_byte_2`: Compares a value from a structure and, if it is within a certain range, sets a value in a structure to a constant.
- `update_value_from_table`: Increments a value at offset 696 of a structure in a2, calculates an offset using a value from a structure in a2 and a pointer from a0, and stores a byte from that table into a value at offset 692 of a structure in a2 before jumping to a target.
- `conditional_subroutine_execution`: Compares a value at offset 692 of a structure in a2 with a constant, and if they are equal, calls a subroutine twice using a pointer or a value derived from a pointer before jumping to a target.
- `copy_structure_members`: Copies a 32-bit value from a structure in a2 to a pointer in a1, copies another 32-bit value from a structure in a2 to an offset of 4 in a1, then calls another routine and branches to a new location.
- `set_constant_value`: Sets a value at offset 704 of a structure in a2 to a constant and copies a value from a pointer in a1 into a register before jumping to a target.
- `execute_subroutines_with_pointers`: Calls a subroutine using a pointer or a value derived from a pointer, then calls another subroutine using a pointer or a value derived from a pointer, and jumps to a target.
- `update_structure_with_subroutine_result`: Calls a subroutine, copies the result back to a pointer in a1, and if a value at offset 628 of a structure in a2 is not 41, sets a value at offset 640 of a structure in a2 to -1, then calls another subroutine before jumping to a target.
- `branch_on_value_match`: Checks a value in a structure against several constants and, depending on the match, calls routines or sets a value in a structure to a constant.
- `execute_conditional_logic_3`: Clears a 32-bit value at an offset of 4 in a pointer, calls another routine, and then enters a loop that calls another routine while checking if a value in a structure in a2 matches certain constants, or otherwise calls another routine and jumps to a location in a6.
- `update_table_and_call`: Calculates an offset using a value from a structure in a2 and a pointer in a1, copies a byte from a table in a0 to a pointer in a1, copies a 32-bit value from a structure in a2 to a pointer in a1, then calls another routine and branches to a new location.
- `load_and_call_subroutine`: Loads a long-word from an offset of 632 in a2 into d2, loads a long-word from an offset of 8 in a1 into d1, loads the constant 24 into d0, loads a pointer or address or offset 128 in a4 into a4, calls a routine using the address in a5, and branches to another routine.
- `skip_nop_instruction`: Performs a no-operation and then jumps to a different location.
- `process_linked_list_and_swap`: Loops through a list where each element contains a pointer to the next, reading a byte from a table and writing it to a certain offset, and swaps a byte from a pointer in a1 into a table before checking if the pointer is null.
- `store_and_branch_2`: Stores a 32-bit value from d1 into a structure in a1, otherwise it branches to another routine or jumps to an address in a6.
- `jump_to_address`: Jumps to a different location.
- `traverse_and_replace_list_element`: Loops through a list, replacing a value in the list with a byte from a table, and once a certain byte is found, writes a byte from a pointer in a1 into a table before jumping to a target.
- `initialize_and_clear_table`: Clears several 32-bit or 16-bit values in a structure pointed to by a2, copies a byte from a table (indexed by d1 multiplied by 4) into a structure field, and then loops to clear a range of bytes in a table (indexed by d1 and a pointer from a2) until a certain value is reached.
- `execute_subroutines_and_invert_value`: Calls a subroutine three times with different parameters or pointers, and then calculates the bitwise NOT of a value at offset 640 of a structure in a2 before jumping to a target.
- `branch_on_nop`: Performs a no-operation and then branches to another routine.
- `branch_to_offset`: Branches to a nearby instruction.
- `set_negative_one_and_jump`: Sets d1 to negative one and jumps to the address in a6.
- `set_zero_and_jump`: Sets d1 to zero and jumps to the address in a6.
- `check_value_against_list`: Loads a value from a pointer in a1 and compares it against several constant values, otherwise it jumps to another routine.
- `check_pointer_and_branch`: Tests if a pointer in a1 is null and, if not null, branches to a different location; otherwise, it clears d1 and jumps to a location in a6.
- `process_list_and_call`: Copies a byte from a table in a0 to a pointer in a1, updates a value in a structure in a1, and if a certain condition is met, calculates a new offset, copies a byte from a table in a0 to a pointer in a1, and calls another routine.
- `update_and_branch`: Sets d1 to 1, adds a value from a structure offset 8 (pointed to by a1) to d1, and then jumps to a different location.
- `clear_d1_and_jump`: Clears register d1 and jumps to the address stored in a6.
- `reset_string_bounds_and_update_display`: Resets several string index bounds and pointer-based array limits in a structure pointed to by a1, calls a routine to initialize string bounds, and then updates certain display-related pointer or size fields in the structure before branching to another routine.
- `initialize_string_index_bounds`: Sets certain array elements in a structure pointed to by a1 to a default value and then uses a routine to check or copy data from a source string or block, potentially updating a global or structure-based index or terminator.
- `parse_string_and_update_index_2_2`: Reads a pointer from a structure, checks if the target string or data block is null, and if not, copies bytes from that source or a secondary source into a destination buffer while updating an index and a length or end-marker flag in the structure.
- `jump_to_target_address`: Performs a short branch to a target address located in the code segment.
- `parse_string_and_update_index_2`: Parses a string or a sequence of bytes from a structure, updating several offset-based pointers and flags in a structure, and checks for a termination character or a specific length limit, otherwise it loops through the string or a pointer-based sequence.
- `parse_string_and_update_index`: Checks if a certain flag in a structure is zero and if a pointer in a table is non-null, then calls another routine to process a string.
- `update_table_and_rebuild_index_2`: Updates a byte in a table using an index-based offset, updates a pointer or counter in a structure, and then enters a loop that copies bytes.
- `update_table_and_rebuild_index`: Updates a byte in a table using an index-based offset, updates a pointer or counter in a structure, and then enters a loop that copies bytes from a source table to a destination table while updating a secondary index or pointer.
- `modify_table_byte`: Decrements a longword loaded from an offset of 1988 in a1, shifts a longword loaded from an offset of 1448 in a1 left by two, and stores the resulting byte into a table at a calculated offset in a0, then branches to another routine.
- `update_pointer_logic_and_display_index`: Clears a byte in a table using an index, checks a flag in a structure, and if certain conditions are met, calculates and stores a new pointer or a bitwise-inverted address into a structure, then calls a routine to update a pointer or display logic.
- `calculate_scaled_address`: Calculates an address by adding a constant to a4, stores that address in d1, shifts d1 right by two, and branches to a different location.
- `update_pointer_logic_and_display`: Updates a pointer in a structure using an index, potentially updates a table entry with a calculated address, and executes a sequence of calls to a routine using a constant or a pointer-derived value, followed by conditional updates to a structure's field and a final check of a pointer-based value to update a certain field or set a constant value.
- `update_pointer_logic`: Calls another routine using a pointer from a structure, then checks a byte in a table; if the byte is zero, it calls another routine and updates a value in a structure using a pointer-based offset, otherwise it calls another routine and updates a pointer-based value in a structure.
- `update_display_bounds_and_offset_4`: Calculates a bit-shifted value from a pointer-based address, stores it in a structure, and calls another routine to check if the resulting value is zero; if zero, it resets a different structure field to zero using another routine call, otherwise it updates a pointer-based offset or boundary.
- `reset_display_bounds_and_offset_2`: Updates a display boundary or offset in a structure in a1 by calling another routine to retrieve a value, then checks if that value is zero to either reset a display parameter or calculate a new boundary or offset based on a pointer in a2 and calls several other routines to update display parameters or reset certain values.
- `reset_display_bounds_and_offset`: Checks if a display offset or boundary (stored at offset 584 of a structure in a1) is negative, and if so, updates a value in a structure (a), then resets certain display parameters or calculates a new display boundary or offset (via a call to another routine) and updates a structure (a).
- `update_display_bounds_and_offset_logic`: Checks if a display offset or boundary (stored at offset 584 of a structure in a1) is negative, and if so, updates a value in a structure (at offset 648 of a1) with the bitwise NOT of that offset.
- `update_scaled_display_offset`: Calculates a new scaled display offset by loading a value from a pointer in a1, shifting it left by two, and storing it into a structure offset in a0, then updates a pointer in a1 with a value from a2 and calls another routine.
- `calculate_scaled_display_offset`: Loads an address from a pointer in a4, copies it to d1, shifts the value right by two, and stores the result into a structure offset in a2 before branching to another routine.
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
- `execute_indirect_with_params`: Sets d1 to 45 and d0 to 24, loads a pointer from a2 plus 224 into a4, calls the routine at a5, and jumps to the address in a6.
- `update_display_index_and_parameters_2`: Updates a display index or offset in a structure based on a comparison between a pointer-based value and a target value, then updates a secondary index or pointer-based value in that structure.
- `test_and_mask_bit_one`: Tests if the least significant bit of a register is set and, if so, clears all but the first bit of that register before branching or moving a value into it.
- `execute_indirect_with_params_alt`: Sets d1 to 45 and d0 to 24, loads a pointer from a2 plus 224 into a4, calls the routine at a5, and jumps to the address in a6.
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

- `find_max_and_prepare_buffer_11`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes that value and a small header into a buffer.
- `copy_words_and_call_4`: Copies a fixed number of words from a source table into a destination buffer when the destination is not null, then invokes another routine.
- `update_structure_and_call_subroutines`: Updates a bit in a byte using a bit-shift, modifies several pointer-based offsets in a structure, and executes a sequence of conditional calls to other routines based on the contents of a pointer-indexed table or a structure-relative offset.
- `check_structure_and_call_2`: Calculates a bit-shifted address from a4, calls another routine, and if a certain value at an offset in a1, scaled by a pointer, is non-zero, calls another routine and updates a global variable with a value from a pointer in a2.
- `set_buffer_and_draw`: Loads a pointer from a2 plus 1544, calls a routine with a length of 720, and if the loaded value is zero, calls another routine and sets a value at a1 plus 704.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_prepare_buffer_12`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen long words into a target buffer before invoking another routine.
- `copy_words_from_table`: Copies a fixed number of words from a source table pointed to by a register into a destination buffer passed as a parameter, stopping after the count reaches zero.
- `update_object_state_and_indices`: Updates a pointer in a structure, clears several fields at certain offsets, and then checks a series of table-indexed values to conditionally update or reset other fields in the structure, including checking for a specific character '$' in a table.
- `check_table_index_and_set_value_2`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a structure in a0; if that byte is non-zero, it compares a value of 10 in d2 against a longword at offset 548 in a2 and, if less, sets d1 to 255 before branching to another routine.
- `check_table_index_and_set_value`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a structure in a0; if not zero, it compares a value in a2 to 20 and, if less than, sets d1 to a certain value before branching.
- `traverse_and_update_structure_pointers`: Traverses a linked-list-like structure or table-based hierarchy by updating pointers in a structure in a1 based on values found in a table in a0, and calls another routine if certain character or pointer conditions are met.
- `validate_and_update_structure_state`: Checks if certain characters in a table-driven structure match a specific character, and if so, calls another routine to update or reset structure members.
- `branch_to_target_address_4`: Performs a short branch to a relative offset, effectively redirecting execution flow if certain conditions in the calling routine are met.
- `test_structure_member_and_branch`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a0; if not zero, it compares a longword at offset 388 in a1 to 1 and, if not equal, sets d1 to a certain value and branches.
- `branch_to_next_routine`: Performs a short branch to a relative address, effectively acting as a jump to skip or redirect execution flow.
- `test_structure_member_bits_branch_logic`: Checks if a longword at offset 356 of the structure in a1 is equal to one, and if not, tests if that same longword is zero to determine whether to branch to another routine, branch to a different routine, or branch to a third routine with d1 set to a negative value.
- `update_object_index_and_call_subroutines`: Checks if a pointer-indexed array entry is non-zero, calls a routine, and updates a field in a structure with the returned pointer-index.
- `reset_and_initialize_object_state`: Resets a flag in a structure, checks a pointer-indexed array, and if certain conditions are met, clears a field, sets a counter to ten, and calls another routine.
- `update_object_bounds_and_visibility`: Updates a size or count in a structure, then checks or updates bitmasks or visibility flags in a structure, potentially looping to update a value in a global variable until a certain condition or limit is met, and finally calls another routine.
- `update_object_bounds_and_visibility_2`: Calls another routine to update a value in a structure, loops to check if that value is certain constants, and if certain flags in a1 are zero, calls another routine to update a value in a structure and calls another routine to update a bitmask in a structure.

### LoadWB

- `allocate_control_record_and_wait_2`: Allocates a control record for the current task, takes the current directory lock and the task's message port, and opens dos.library.
- `release_task_resources_and_reply_2`: Finds the current task and stores the supplied value in its record.
- `start_workbench_task_from_tag`: Opens the icon and intuition libraries, allocates a record, and starts the Workbench task from its resident tag.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte.
- `build_linked_list_from_table`: Walks a table of pointers, allocating a zeroed node for each entry, copying a looked-up value into it, and linking the nodes into a list.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_function_from_global`: Invokes a function pointer taken from a fixed offset within a global variable structure, then returns.
- `call_function_from_global_offset`: Invokes a function pointer taken from a different fixed offset within a global variable structure, then returns.
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
- `dispatch_task_stub`: Calls the Dispatch entry of the stored exec library base and returns, handing control to the task scheduler.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_other_routine_with_stack`: Calls another routine, passing the two incoming parameters together with a pointer to a local stack slot, then returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_output_byte`: Stores the low byte of its input register at the address held in the pointer register and returns.
- `compare_string_prefixes`: Checks if a2 is null or certain, then checks if a byte at a2 plus 20 is '2'. If so, it compares a string at a2 with a global string, potentially padding with dots if a null terminator is found.
- `write_dotted_padding_bytes`: Writes a dot followed by four spaces into a text buffer at a fixed position.
- `compare_stored_prefix_to_buffer`: Compares a stored prefix string against a text buffer, advancing through both while the bytes match and stopping at the end of the prefix or the first mismatch.
- `empty_return_stub`: Contains only a return instruction, so it does no work when called.

### Version

- `parse_command_line_tokens`: Parses a command-line string into individual tokens, splitting on spaces and tabs while handling quoted sections and escape sequences, then stores the resulting tokens in a table.
- `store_param_and_invoke_callbacks`: Fetches a structure pointer from a global variable, stores the incoming parameter into a slot at a fixed offset within that structure, and then invokes a series of callbacks from a table based on flag checks before returning the previously stored value.
- `report_library_or_device_version`: Parses the library name, version and revision arguments, opens the named library or device, and prints its version.
- `forward_with_fixed_flag_value`: Passes the first parameter to one routine together with a fixed flag value of 128, then passes the second parameter to another routine.
- `parse_signed_decimal_string`: Parses an optional plus or minus sign followed by decimal digits into an integer.
- `compare_strings_byte_by_byte`: Walks two null-terminated strings in lockstep, comparing each pair of bytes, and returns a negative value when the first string sorts before the second, a positive value when it sorts after, and zero when they match.
- `calculate_string_length`: Returns the length of a null-terminated string by counting bytes up to the terminator.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_callback_from_structure`: Dereferences a function pointer stored at a fixed offset within a global structure and invokes it, then returns.
- `recurse_with_stack_slot`: Calls another routine passing its two parameters along with the address of a slot on the current stack frame.
- `invoke_system_library_call`: It invokes the routine whose entry point is stored in the system library's function table, passing no arguments.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_indirect_global`: It invokes the function whose address is stored in a global variable and then returns.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_routine_via_table`: It invokes the function whose address is stored at a fixed offset within a global routine table, then returns.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer held in a global variable, then walks a stack buffer computing a rolling checksum until it reaches a null terminator, and forwards the buffer along with the inverted checksum to another routine when that checksum is non-zero.
- `store_byte_from_register`: Copies a single byte held in a data register into the memory location pointed to by an address register, then returns.

### SetMap

- `initialize_map_structure`: Initializes a global structure by calling its setup routines and storing pointers and values into its fields, then returns a status code.
- `restore_patch_state`: Reads a pointer from a global structure, saves the incoming parameter into a slot within the pointed-to structure, conditionally runs several cleanup callbacks from a table while recording specific values before each, and finally returns the saved parameter.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_indirect_from_d1`: Invokes the function whose entry point is stored at a fixed offset from a data register, then returns without using the result.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `reschedule_task_stub`: Calls the Reschedule entry of the stored exec library base and returns, letting the scheduler run other tasks.
- `unlock_stored_file_locks`: Walks a table of stored file locks and unlocks each non-zero entry, then releases a few remaining locks and the supplied lock.
- `save_params_and_run_setup`: Saves its two parameters in global variables, calls a setup routine with fixed numeric arguments, then calls a finishing routine with the value twenty.
- `load_keymap_into_console`: Opens the keymap resource, console device and icon library, reads the KEYMAP tool type from the icon, and loads the named keymap file from the keymaps directory.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_indirect_from_d1_2`: Fetches a routine pointer from a fixed offset in a data register and jumps to it, discarding any returned value.
- `call_indirect_from_d1_3`: Loads a function address from a fixed offset in a data register and calls it, returning immediately afterward.
- `call_indirect_from_d1_4`: Dereferences a pointer stored at a fixed offset from a data register and invokes the pointed-to routine.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_indirect_from_d1_5`: Calls the routine whose address is held at a fixed offset in a data register, then returns without using the outcome.
- `call_indirect_from_d1_6`: Fetches a function pointer from a fixed offset in a data register and jumps to it, discarding any returned value.
- `call_indirect_from_d1_7`: Loads a routine address from a fixed offset in a data register and calls it, returning immediately afterward.
- `call_indirect_ptr`: Dereferences a pointer stored in a global variable and jumps to the routine it holds, then returns.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_d1_offset`: It invokes the routine whose pointer is stored at a global variable located 22 bytes before the address given by register d1, then returns.
- `invoke_indirect_pointer`: Dereferences a pointer stored at an offset from a global variable and invokes the routine it points to.
- `call_global_function_pointer`: Dereferences a function pointer stored in a global structure and jumps to it without passing any argument.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_argument_to_library_call`: Passes its single argument to a library routine together with a stored base pointer and the address of a stack slot, then returns.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_3`: Loads and runs a function pointer stored in a global variable, then scans a stack buffer byte by byte computing a rolling checksum until it reaches a null terminator.
- `store_byte_from_register_2`: Stores the low byte held in a register into the memory location addressed by a pointer register, then returns.

### More

- `init_and_tokenize_string`: Initializes a small structure by storing a parameter and a stack pointer into it, then branches based on the result of a check.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `display_file_pages_in_window`: Opens intuition.library and a window, then reads the named file and shows it page by page.
- `apply_helper_to_four_globals`: Calls one routine four times, passing a field of its argument together with each of four global values in turn.
- `check_buffer_for_binary_bytes`: Fills an 80-byte buffer through two routine calls, then scans it for control bytes.
- `read_key_with_break_check`: Waits for input with a timeout and checks for a break signal, returning zero when a break arrives.
- `execute_page_routines`: Checks if a value at a2 plus 60 is positive, then calls a routine using a2, calls another routine using a2, and jumps back to the start.
- `run_external_editor_command`: Passes a global command string to a routine, then calls a handler pointer taken from a register.
- `run_page_display_loop`: Sets the first two state fields, calls a setup routine, then repeats a page routine while more lines remain to show.
- `process_page_buffer_and_count`: Runs a text-processing routine on the page buffer, stores its result in a state field, and increments a counter in a state record when a flag is clear.
- `find_next_line_break`: Reads a chunk into a buffer, records its size, and scans for a newline or form-feed character to find the end of the next line.
- `run_pager_key_loop`: Chooses how many lines to show from state flags, draws them in a loop, and reads a key after each pass.
- `search_file_for_pattern`: Chooses a search string based on a flag, runs a find routine, and repeats the search while a counter stays positive.
- `run_action_between_setup_calls`: Runs a fixed sequence of setup and reschedule calls around a main action, then returns that action's result.
- `call_action_with_optional_follow`: Calls a routine with two arguments, then calls another routine with the first argument alone when a flag in the state record is clear.
- `draw_then_update_line_count`: Calls a drawing routine with a stored handle and value, then calls a second routine with the line-count field from the state record.
- `draw_then_decrement_line_count`: Calls a drawing routine with a zero argument and a value of one, then calls another routine with the line-count field minus one.
- `draw_text_lines_until_limit`: Iterates over the text buffer, drawing lines through a handler pointer and tracking the line count.
- `call_pair_with_state_field`: Calls one routine with a state record, then calls another with a field of that record, a global buffer and the first argument.
- `reset_position_and_call_routine`: Sets a position counter to one when a flag in the state record is clear, then calls a routine with a field of the record and a global.
- `call_routine_with_selected_offsets`: Calls a routine with a field of the state record and two arguments chosen by a flag.
- `print_formatted_pair_or_count`: Calls a routine that prints a formatted line, using two global string arguments or a counted value when a flag is set.
- `query_and_clamp_result`: Calls a routine once with a fixed argument, then calls it again with an argument of minus one and returns that result, clamped to zero when negative.
- `copy_available_bytes_from_buffer`: Copies up to a requested number of bytes from a buffer region into the output, limited to the bytes remaining.
- `refill_read_buffer`: Flushes buffered bytes when a full block is pending, then reads up to a fixed block size of 4096 bytes into the buffer and records the byte count.
- `scroll_window_to_position`: Works out a target position from an offset and a mode argument, checks it against the window bounds, and moves the window there.
- `read_line_into_buffer`: Reads bytes one at a time through a read routine into a buffer until a newline or a fixed length limit is reached, then terminates the string.
- `copy_null_terminated_string`: Copies bytes from a source string into a destination buffer, including the terminating zero byte.
- `convert_string_to_uppercase`: Converts each lowercase ASCII letter in a null-terminated string to uppercase in place.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `release_handle_if_present`: Passes the argument and a fixed value of minus one to a library routine using the stored handle, when both the argument and the stored handle are non-null.
- `print_text_and_free_record`: Releases the sub-records of a display record, then either outputs the given text or only releases the record.
- `call_with_single_value_argument`: Stores its argument in a one-element array, then calls a library routine with that array, a fixed code and a count of one.
- `read_record_fields_into_struct`: Allocates a small cleared record, passes its pointer to a library routine, copies two of its fields into the caller's structure, and then frees the record.
- `allocate_linked_record_from_template`: Allocates a record, links its list fields to itself and to a template, and copies a given number of values from the source block into it.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `reschedule_task_stub_2`: Calls the Reschedule entry of the stored exec library base and returns.
- `switch_task_stub`: Calls the Switch entry of the stored exec library base and returns, handing the processor to another task.
- `call_indirect_via_d1`: It invokes the routine whose address is stored at the global variable located 60 bytes below the address currently in register d1, then returns.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `allocate_memory_stub`: Calls the AllocMem entry of the stored exec library base and returns its result.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `invoke_stored_function`: This routine calls the function whose address is stored at the beginning of a block of memory, passing no arguments.
- `call_indirect_via_table`: Invokes a function pointer obtained from a table indexed by a value in register d1, then returns.
- `call_indirect_d1`: This routine invokes the function whose address is stored at the location pointed to by the value in register d1, then returns without using the result.
- `call_indirect_global_2`: It invokes the routine whose address is stored in a global variable and then returns.
- `call_indirect_via_d1_2`: It invokes the routine whose address is stored at the offset pointed to by the value in register d1, then returns.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_indirect_global_3`: It invokes the routine whose address is stored in a global variable and then returns.
- `invoke_indirect_handler`: It invokes the routine whose address is stored at the offset located 372 bytes below a global base pointer, then returns.
- `invoke_indirect_caller`: This routine invokes the function whose address is stored at the pointer located 384 bytes before a base value in register d1, then returns.
- `call_indirect_d1_offset_2`: It invokes the routine whose address is stored at the offset located 414 bytes below a global variable pointer, then returns without using any library calls.
- `call_routine_table_entry`: It invokes the function whose address is stored at a fixed offset within a global routine table, then returns without using any return value.
- `call_exec_entry_stub`: Calls one exec library entry through the stored base and returns, with no processing of its own.
- `recurse_with_stack_slot_2`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `read_one_byte_or_fail`: Reads one byte through a read routine into a stack buffer and returns it.
- `invoke_and_checksum_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack-allocated buffer until a null terminator is found.
- `store_byte_via_a3`: Stores a byte from data register D0 through the pointer held in address register A3.
- `unsigned_divide_with_remainder`: Performs an unsigned 32-bit division by shift and subtract when the divisor is large, or a 16-bit modulo when it fits in 16 bits.
- `multiply_longs_via_partials`: Multiplies two 32-bit values from 16-bit partial products and returns the result together with a byte-swapped copy of the second input.
- `call_then_return_stub`: Calls one routine with no arguments and returns, doing nothing else.
- `call_helper_return_register`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag`: Handles the signs of two values, calls a helper, and negates the result when the flag the helper leaves in an address register is nonzero before returning.
- `call_and_negate_on_flag`: Calls a routine and returns its first result, or the negation of that result when a second register is non-zero.
- `raw_format_string_stub`: Calls the RawDoFmt entry of the stored exec library base and returns, formatting text through the supplied callbacks.
- `store_byte_a3`: Stores the low byte of register D0 into the byte pointed to by the A3 register.
- `call_indirect_global_4`: This routine invokes the function whose address is stored at an offset within a global variable, passing no arguments.
- `allocate_signal_and_message`: Allocates a signal bit and a message structure, then fills in the reply port, the signal number and the message type.
- `release_signal_and_message`: Removes the message from its port list when it is linked, marks its status fields as released, then frees its signal bit and its memory.
- `search_string_for_pattern`: Checks whether the second string appears in the first, stopping at a newline.
- `initialize_empty_list_head`: Points the first field at itself with an offset of four, clears the second field, and points the third field at itself, forming an empty list head.

### Format

- `initialize_task_dispatch`: Initializes a task control block by filling in its fields from the incoming registers, then dispatches based on a command code.
- `restore_stored_parameter`: Fetches a pointer from a global structure and stores the input parameter into a slot within the pointed-to structure.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `allocate_memory_block`: Allocates a block of memory through a system routine and returns no value.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `switch_active_task`: Switches the active task through a system routine and returns no value.
- `forbid_multitasking_stub`: Calls the Forbid entry of the stored exec library base and returns, which stops task switching until a matching permit call.
- `dispatch_library_message`: Dispatches a message through a system routine and returns no value.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `schedule_task_stub`: Calls the Schedule entry of the stored exec library base and returns.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `allocate_absolute_memory_stub`: Calls the AllocAbs entry of the stored exec library base, which allocates memory at a given address, and returns its result.
- `reschedule_task_queue`: Reschedules the task queue through a system routine and returns no value.
- `run_format_disk_command`: Parses the DRIVE, NAME, FFS, NOICONS and QUICK options, checks that the drive exists, and prompts for a disk before initializing it.
- `free_format_buffers`: Frees the buffers the format command allocated, passing each stored pointer with its size to the free routine, and releases the supplied parameter last.
- `report_disk_io_error`: Retries a disk operation while it reports a recoverable condition, then maps the error code to a message such as bad sector, seek error, disk changed or cannot open device.
- `store_matching_drive_code`: Looks up the given code in an eight-entry table and stores the matching value in a field of a record returned by a routine.
- `find_drive_by_name`: Walks the list of device nodes looking for a volume whose name matches the given drive text.
- `calculate_string_length_2`: Walks through a null-terminated string starting at the input pointer, advancing one byte at a time until reaching the terminator.
- `copy_cstring_to_buffer`: Copies a null-terminated byte string from the source pointer to the destination pointer, advancing both pointers and writing each byte until a zero byte is written.
- `convert_char_to_uppercase`: Converts one lowercase ASCII letter to uppercase and returns other characters unchanged.
- `compare_strings_ignoring_case`: Compares two null-terminated strings one byte at a time after converting each byte to uppercase.
- `truncate_at_colon`: Cuts a string at its first colon by writing a zero byte there, or at its end when no colon is present.
- `check_status_with_error_code`: Passes its first argument in a one-element array to a routine with a fixed code, then returns an error code of one hundred twenty-six when that call fails and zero otherwise.
- `allocate_linked_record_from_template_2`: Allocates a record, links its list fields to itself and to a template, and copies up to seven values from the source block into it.
- `invoke_library_routine`: Invokes a routine from a specific system library and returns no value.
- `call_second_library`: Invokes a routine from another system library and returns no value.
- `call_third_library`: Invokes a routine from a third system library and returns no value.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_table_routine`: Fetches a function pointer from a global table and invokes it, returning whatever that routine produces.
- `invoke_dispatched_call`: Loads a routine address from a global dispatch table and jumps to it, then returns.
- `call_indirect_table`: Reads a code pointer from a global table entry and calls the referenced routine before returning.
- `call_table_entry`: Retrieves a pointer from a global table and branches to the routine it names, then returns.
- `invoke_table_pointer`: Fetches a function pointer from a global table slot and executes the pointed-to routine.
- `call_dispatched_routine`: Loads a routine address from a global dispatch table and calls it, then returns.
- `call_indirect_entry`: Reads a code pointer from a global table and branches to the referenced routine before returning.
- `invoke_table_routine`: Retrieves a pointer from a global table slot and invokes the routine it addresses, then returns.
- `call_table_dispatch`: Fetches a function pointer from a global table and calls the pointed-to routine before returning.
- `call_dispatch_pointer`: Loads a routine address from a global dispatch table and branches to it, then returns.
- `call_indirect_handler`: Fetches a function pointer from a global table and jumps to it, acting as an indirect call trampoline.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `invoke_dispatched_routine`: Loads a routine address from a global dispatch table and transfers control to it via an indirect jump.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_stored_pointer`: Reads a stored code pointer from a global table and calls the referenced routine through it.
- `invoke_stored_function_with_stack`: Takes the address of the current stack location and forwards it along with a parameter to a stored function pointer at the start of a memory block, passing no arguments.
- `invoke_table_routine_2`: Fetches a function pointer from a global table and executes the routine it points to via an indirect jump.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_cylinder_by_cylinder`: Runs the format loop cylinder by cylinder through the disk port, reporting progress with the cylinder count to go.
- `configure_disk_port_timing`: Sets timing values on the disk port using two fixed values and an enable value, then passes the parameter through the port routine.
- `invoke_function_and_checksum_stack`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over the stack buffer until a null terminator is found.
- `store_byte_via_pointer`: Stores the low byte of a register into the memory location addressed by a preserved pointer.
- `divide_32bit_16bit`: Divides a 32-bit value by a 16-bit value, using a bit-by-bit restoring loop for large divisors and hardware division otherwise, leaving the quotient and remainder in place.
- `restoring_division_core`: Runs the restoring-division inner loop for sixteen steps on the dividend and divisor registers, leaving the quotient and remainder in place.
- `multiply_32_bit_integers`: Multiplies two 32-bit inputs using 16-bit partial products, returning the low 32 bits of the product.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register_2`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `compute_signed_division_result`: Normalizes the signs of two 32-bit or 16-bit operands, calls a 32-bit by 16-bit division routine, and negates the resulting quotient if the quotient is non-zero and the sign-handling logic requires it.
- `call_and_negate_on_flag_2`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an unable address register is nonzero.
- `call_routine_from_global_table`: Calls the routine whose address is stored at a fixed offset inside a global data block.
- `store_byte_through_pointer`: Writes the low byte of the first data register to the memory location held in the A3 pointer register.
- `allocate_and_initialize_block`: Allocates a fixed-size memory block with a library call and fills in a stored parameter, a flag byte and a value from a second call.
- `reset_object_and_recurse`: When an object's link field is set, it first processes the object again through the same routine.
- `forward_argument_with_fixed_code`: Passes its argument to another routine together with a fixed numeric code.
- `pass_argument_through`: Passes its single argument straight to another routine and returns nothing.
- `setup_trashcan_drawer_state`: Saves a field of an existing object, fills a block of global variables from that object or from defaults, and calls library routines to build the related functions.
- `read_signed_byte_or_fail`: Reads one byte through a routine that takes a handle, a buffer and a count of one.
- `call_table_handler`: Retrieves a handler address from a global table and jumps to it, serving as an indirect call shim.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Points the first longword of a block to the field four bytes past its start, clears the next longword, and stores the block's own address in the third longword.
- `store_fields_in_new_node`: If the first argument is non-null, allocates a node with a library call and stores a byte value, a word from the second argument and the pointer into fixed fields of it.
- `reset_node_and_call_routine`: If given a non-null node, sets its byte field and two longword fields to all ones, then passes the node and a stored word to another routine and returns that result.
- `call_library_routine_via_global`: Calls a library function through a global library base at a fixed negative offset, passing no arguments.
- `call_library_vector_from_global`: Calls a second library function through the same global library base at a different fixed negative offset, passing no arguments.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_and_run`: Program entry point that allocates a task structure, splits the command-line text into words (handling quoted text and star escapes), and calls the main routine.
- `call_exec_supervisor_function`: Calls the exec Supervisor function through a stored library base, passing no arguments.
- `exit_interrupt_handler`: Ends an interrupt handler by calling the exec ExitIntr function through the stored library base.
- `invoke_task_switch_hook`: Calls the exec Switch function through a stored library base, passing no arguments.
- `invoke_task_dispatch_function`: Calls the exec Dispatch function through a stored library base, passing no arguments.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_exec_make_functions`: Calls the exec MakeFunctions routine through a stored library base, passing no arguments from this code.
- `call_exec_find_resident`: Calls the exec FindResident routine through a stored library base, passing no arguments from this code.
- `enable_interrupts_through_exec`: Calls the exec Enable routine through a stored library base to re-allow interrupts.
- `call_exec_alloc_entry`: Calls the exec AllocEntry routine through a stored library base, passing no arguments from this code.
- `open_shell_window_and_start`: Creates a new command shell by allocating a state block and opening a console window that runs the CLI or Shell startup script.
- `build_linked_node_list`: Builds a singly linked list from a source list, allocating an 8-byte node for each entry and storing a value derived from the entry's second field.
- `free_linked_node_list`: Walks a linked list of nodes, releasing the value stored in each node's second field and then the node itself.
- `parse_signed_decimal`: Reads a null-terminated string of decimal digits with an optional leading plus or minus sign and returns the signed integer value.
- `call_library_vector_no_args`: Calls a library function through a vector in a global library base, passing no arguments.
- `call_library_vector_no_args_2`: Calls another library function through a vector in a global library base, passing no arguments.
- `call_dispatched_pointer`: Loads a routine address from a global dispatch table and transfers control to it through an indirect jump.
- `invoke_indirect_handler_2`: Reads a stored function pointer from a global table and jumps to the referenced routine.
- `call_table_pointer`: Fetches a code pointer from a global table and calls the routine it addresses via an indirect jump.
- `call_indirect_from_data`: Reads a function pointer from a fixed data location and jumps to it, returning immediately after the call.
- `call_library_entry`: It invokes the routine whose address is stored at the start of the library's data block and then returns, performing no work itself.
- `call_library_entry_2`: It invokes the routine whose address is stored at the beginning of the AmigaOS library, performing a simple indirect call.
- `divide_32_bit_with_remainder`: Divides a 32-bit dividend by a divisor and returns the quotient and remainder together.
- `multiply_32_bit_integers_2`: Multiplies two 32-bit inputs using 16-bit partial products, returning the low 32 bits of the product with the second input's halves swapped.
- `forward_call_to_helper_2`: Calls one helper and returns, with no other work.
- `call_helper_return_register_3`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_quotient_on_remainder`: Normalizes the signs of two 32-bit operands, calls a routine to perform a 32-bit division, and negates the resulting quotient if a non-zero remainder is detected in a register.
- `negate_result_if_flagged`: Calls another routine and returns its first result value, negated when the second result register is non-zero.

### Disk-Validator

- `validate_disk_block_bitmap`: Walks the disk block bitmap block by block, comparing each bit with its expected allocation state and counting mismatches.

### Ram-Handler

- `copy_memory_via_exec`: Calls the exec CopyMem routine through a stored library base, then makes an indirect call whose target could not be determined.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_BLIT

- `free_blocks_and_reset_slot`: Calls the exec FreeMem routine several times in a loop, then looks up a value in a global structure and, when the slot matches a fixed constant, swaps it with a global and clears that global.
- `decrement_until_minus_one`: Spins a 16-bit counter down from its starting value until it reaches minus one, a busy wait with no other effect.
- `decode_bitstream_then_clear_cache`: Reads a bit-packed stream and processes the decoded values in seven passes.
- `continue_bitstream_decode_loop`: Continues a bit-packed decoding loop that shares its bit buffer and state with the routine before it.
- `decode_bitstream_into_buffer`: Decodes a bit-packed stream by shifting a bit buffer, then adds values backwards through a destination block.
- `push_and_branch`: Modifies a byte in d0, then pushes d0, d1, and d2 onto the stack before restoring all used registers and branching to another routine.

### d

- `empty_routine_2`: This routine contains no executable instructions; it is empty and performs no operations.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_CPU

- `initialize_game_state`: Runs a sequence of function calls, executes a small loop that invokes a function pointer taken from a global variable, clears a word at a register-derived address, then reads a value from a structure and swaps a table entry with a RAM variable when a specific constant matches.
- `decrement_until_minus_one_2`: Spins a 16-bit counter down from its starting value until it reaches negative one, performing a busy wait with no other effect.
- `update_array_with_loop_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 6, calling a routine if a certain condition is met.
- `update_array_with_loop`: Checks if a value in d0 is negative, otherwise it calls another routine and, if d0 is not zero, enters a loop that adds a value from a pointer in a3 to a table and decrements a counter until it reaches zero.
- `transfer_pointer_and_call`: If a condition is met, it copies the pointer from a3 to a1 and calls another routine, otherwise it jumps to a different location.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `expand_bitstream_sequence_2`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting exp.
- `expand_bitstream_loop`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `initialize_expansion_parameters_5`: Sets d0 to 5, sets d7 to 3, and branches to another routine.
- `initialize_expansion_parameters_4`: Sets d0 to 4, sets d7 to 2, and branches to another routine.
- `initialize_expansion_parameters_3`: Sets d0 to 3, sets d7 to 1, and branches to another routine.
- `expand_bitstream_sequence`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or or compressed data to a destination.
- `modify_d1_and_branch`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `expand_and_copy_bitstream_2`: Reads a bitstream-encoded sequence where certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or decompressed byte sequence to a destination buffer.
- `expand_and_copy_bitstream`: Reads a bitstream or compressed-style bit-count-based sequence, expanding or skipping data based on a bit-mask, and copies the resulting bytes from a source pointer to a destination pointer while decrementing a counter in d0.
- `push_registers_and_jump`: Pushes a byte from d0, a byte from d1, and a word from d2 onto the stack using a4 as a pointer, restores a large set of registers from the stack, and jumps to another routine.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon

- `init_system_memory`: Waits out two short busy-wait loops, then runs a small set of setup routines and a four-iteration loop that invokes a handler from a table.
- `decrement_until_minus_one_3`: Repeatedly subtracts one from a signed 16-bit value until it equals negative one, then exits.
- `update_a1_from_a3_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 4, calling a routine if a certain condition is met.
- `accumulate_pointer_values`: Performs a conditional check on d0, calls another routine, and if d0 is non-zero, executes a loop that adds a value from a pointer in a3 to a memory location and decrements d1 until it reaches zero.
- `update_a1_from_a3`: If a condition is met, it copies the pointer from a3 to a1 and calls another routine, otherwise it jumps to a different location.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop_2`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `expand_rle_data_stream`: Expands a run-length encoded data stream by reading a control byte and a sequence of bytes to determine a repeat count, then either copies or repeats bytes from a source buffer to a destination buffer using a loop.
- `expand_run_length_encoded_stream_loop`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `initialize_expansion_parameters_5_alt`: Sets d0 to 5, sets d7 to 3, and branches to another routine.
- `initialize_expansion_parameters_4_alt`: Sets d0 to 4, sets d7 to 2, and branches to another routine.
- `initialize_expansion_parameters`: Sets d0 to 3 and d7 to 1, then jumps to another routine.
- `expand_run_length_encoded_stream`: Expands a compressed data stream by reading a control byte or a sequence of bytes to determine a repeat count, then copies or repeats bytes from a source buffer pointed to by a0 into a destination buffer.
- `update_d1_and_jump`: Multiplies d1 by two and branches if the result is non-zero, otherwise it loads a byte from the pointer in a0 into d1, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `expand_compressed_data_stream`: Reads a control byte from a table to determine a repeat count, then enters a loop that expands or copies bytes from a source buffer to a destination buffer, using bit-shifting and conditional logic to handle certain bit-flags or repeat-count-based expansions.
- `copy_and_expand_data_stream`: Reads a control byte from a table to determine a repeat count, then enters a loop that expands or copies bytes from a source buffer to a destination buffer, potentially updating a stack-based or memory-based register backup before returning to another routine.
- `push_registers_to_stack`: Pushes two bytes from d0 and d1, a word from d2, and a large set of registers onto the stack, then branches to another routine.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon_install

- `initialize_game_state_2`: Calls another routine several times, runs a small loop that invokes a function pointer taken from a global variable, clears a word at a register-derived address, then reads a value from a structure and swaps a table entry with a RAM variable when a table entry matches a constant.
- `decrement_to_minus_one`: Repeatedly subtracts one from a local signed value until that value equals negative one, then exits.
- `modify_table_elements_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 3, calling a routine if a certain condition is met.
- `modify_table_elements`: Executes a sequence that calls another routine, and if d0 is not zero, runs a loop that adds a value from a pointer in a3 to a table and decrements a counter until it reaches zero.
- `test_and_copy_pointer_to_register`: Checks if a zero flag is set, otherwise it copies a pointer from a register to another register and calls a routine that copies four bytes from a source to a destination, then jumps to a target address.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_bytes_from_source_to_dest`: Checks if a source pointer equals a destination pointer, otherwise it copies four bytes from a decrementing source pointer to an incrementing destination pointer in a loop, and otherwise it loads a value from a pointer in a1 into a register before returning.
- `decompress_variable_length_stream_2`: Decodes a bit-streamed, variable-length encoded sequence to calculate a block size and a destination pointer, then copies a block of data from a source buffer.
- `process_stream_loop`: Sets d7 to 2, and if d1 is not zero, increments d1 and loops back to a previous instruction; otherwise, it copies a byte from the pointer in a0 to d1, updates d1 and d0, and loops d7 times before setting d0 to 6, d7 to 3, and jumping to another routine.
- `initialize_expansion_parameters_2`: Sets d0 to 5 and d7 to 3, then jumps to another routine.
- `initialize_expansion_parameters_6`: Sets d0 to 4 and d7 to 2, then jumps to another routine.
- `initialize_expansion_parameters_7`: Sets d0 to 3 and d7 to 1, then jumps to another routine.
- `decode_variable_length_stream`: Decodes a bit-streamed, variable-length encoded sequence to calculate a block size and a destination pointer, then copies a block of data from a source buffer.
- `branch_on_d1_doubled`: Doubles d1 and branches if non-zero, otherwise it moves a byte from the pointer in a0 into d1, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `decompress_variable_length_stream`: Reads a bit-streamed, variable-length encoded sequence to calculate a block size and a destination pointer, then copies a block of data from a source buffer to a destination, potentially updating a counter or a pointer or copying extra bytes if a certain condition is met, and finally writes a length or a value to a memory location.
- `decode_variable_length_data`: Decodes a stream of variable-length, bit-shifted or compressed data, updating a destination pointer and a counter, then copies a block of bytes from a source range to a destination range before restoring registers from the stack and returning to another routine.
- `pushes_registers_to_stack`: Pushes a byte from d0, a byte from d1, and a word from d2 onto the stack using a4 as a pointer, restores a large set of registers from the stack, and branches to another routine.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_device_io_and_libraries`: Opens required system libraries, verifies their versions, and sets up device I/O requests.
- `examine_locked_object_field_2`: Allocates memory, locks an object through a library, and examines its fields, performing various cleanup or conditional jumps based on the results.
- `examine_locked_object_field`: Locks an object through the second library and examines it.
- `dispatch_table_entry_by_index`: Looks up an index through a library call; if the index is zero, it releases a lock and returns minus one, otherwise it calls a routine from a table and releases the lock.
- `open_then_close_file`: Opens a file through a library, and if successful, stores the handle in a global variable, calls three routines, and then closes the handle.
- `open_show_error_then_close`: Opens a file through a library and, if successful, executes a sequence of routines, including an error-handling routine if a status value is two or three, before closing the file.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `write_while_data_pending`: Calls a library function if a check routine returns non-zero, then enters a loop that repeatedly calls a library function and another routine as long as a condition remains met.
- `write_if_check_passes`: Calls a library function through a pointer in a global variable only if a preceding routine returns a non-zero value.
- `find_word_in_terminated_table`: Scans a table of 16-bit entries ending with a minus-one marker to find an entry matching the value in the data register, leaving the resulting position in the address register.
- `measure_string_length`: Counts the characters of a null-terminated string addressed by the address register and returns the count in the data register.
- `wait_for_hardware_status`: Sets a global variable to zero and enters an infinite loop if a specific bit in a memory location is set, while polling a hardware register.
- `write_interrupt_request`: Writes a value to the interrupt request register.
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
- `store_adjusted_char_then_call`: If a global variable is zero, it adds sixty-four to a character from a pointer and stores it in a fixed byte cell before calling another routine; otherwise, it simply calls that routine.
- `copy_null_terminated_string_2`: Copies characters from the source pointer to the destination pointer, one at a time, until a zero byte has been copied.
- `set_flag_if_field_below`: Sets a global flag to minus one when a 16-bit field of a record, reduced by thirty-six, is negative, and to zero otherwise.
- `setup_device_io_request`: Allocates a signal and, if successful, opens a device and performs an I/O operation using a prepared request, with cleanup paths for failures.
- `clear_flag_after_library_call`: When a flag is set, calls a library entry and then clears the flag.
- `try_lock_with_fallback`: When a flag is clear, temporarily sets a field of a shared record to minus one, attempts a lock through the second library and restores the field.
- `undefined_routine`: The routine contains eleven instructions and is called by one unnamed routine, but the specific operations performed are not provided.
