# Game 2157 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02157). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 55 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 635 of 635 named (0 author symbols), 635 of 635 described (69 by rule, 212 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 240 of 240 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16522 of 16535 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 71290 |
| code | 52254 |
| data | 481405 |
| padding | 14683 |
| unused | 281600 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Keymap | 1 | 3 | 1 |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_BLIT | 11 | 252 | 6 |
| d | 1 | 3 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_CPU | 10 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon | 8 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon_install | 7 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `stub_unreached_gap_code` (More): The recovered body is only a short call sequence ending in an indirect call, and it was reached only as a gap in the decode. Kind: sabotage path. Pass path: the instruction following the call to the third (unaffected/unreachable) pointer.

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Keymap

- `empty_routine`: the routine performs no operations and contains no valid executable instructions.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_and_call`: If the destination pointer is nonnull, copies 16 longwords from a fixed offset in the structure into it.
- `update_table_entry_and_copy_range_2`: Updates a structure or table entry with a new pointer and range-end value, and then copies a sequence of bytes from a source buffer to a destination buffer, terminating the copy with a specific byte value.
- `update_table_entry_and_copy_range`: Reads a byte from a table using an index stored in a structure, updates a pointer and a range-end value in that structure, and then enters a loop to copy a sequence of bytes from a source table to a destination table, calling another routine during the process.
- `update_structure_and_branch_offset_2`: Updates a structure or table by calculating a new range, then iterates through a loop to copy bytes from a source table to a destination table using an offset-based index, and finally updates a pointer or value at a specific offset in the structure.
- `update_structure_and_branch_offset`: Updates a table or structure using an offset-based index, copies certain longwords between structure members, and then enters a loop that checks a value against a counter, potentially calling another routine and updating further structure members or a global variable.
- `increment_d1_with_offset`: Loads one into d1 and adds a longword from an offset 84 of a1 to d1, then branches to another routine.
- `test_structure_member`: Tests if a longword at offset 40 of a1 is zero, otherwise it branches to another routine.
- `mask_structure_bits_and_compare`: Checks if certain bits are set in a structure member at offset 48 of a1, and if not, sets the upper 24 bits of that member to all ones.
- `update_structure_pointer_and_clear_table`: Updates a pointer in a structure, calls another routine, and if the new pointer value is non-zero, proceeds to update a table entry, reset several fields in a structure, and conditionally decrement a value in a table or array based on a pointer check.
- `update_structure_pointers_2`: Updates a pointer in a structure, calls a routine, and conditionally clears or decrements a value in a table or array based on a pointer check.
- `update_structure_pointers`: Updates a pointer in a structure, calls a routine, and conditionally clears or decrements a value in a table or array based on a pointer check.

### Install

- `build_command_record_with_handler`: Scans a linked list for the largest stored value, then builds a record whose handler is the small routine at offset 310, with a 140-byte stack buffer and words generated from a counter.
- `copy_block_and_branch`: copies a fixed number of words from a memory location to a stack-allocated buffer before calling another routine.
- `load_and_check_pointer`: Loads an address into a1, calls a library routine, and if d0 is not zero, loads a pointer from a structure in a0 into a0 and then returns.
- `set_d0_and_branch`: Sets d0 to negative one and then branches to another routine.
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

- `find_max_and_prepare_buffer_5`: Iterates through a linked list to find the maximum value in a specific field and, if a certain condition is met, initializes a header or buffer.
- `copy_block_to_stack`: copies a fixed-size block of 16 or 17 words from a memory offset or buffer to the current stack pointer and then calls another routine.

### Delete

- `find_max_and_populate_buffer_3`: Traverses a linked list to find the maximum integer value in a specific field, then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_data_to_stack`: copies a fixed number of words from a memory offset into the stack.
- `update_table_index_and_traverse_2`: Calculates a new index or pointer for a table in a1, clears certain fields in a1 and a2, and calls another routine to check or update a secondary table or list, potentially entering a loop that traverses or updates entries based on values read from a table in a0.
- `update_list_index_from_table_2`: Reads a value from a table using an offset, calculates a new index, and stores two bytes or a word into a structure at offset 544 and 548, then branches to another routine.
- `execute_two_subroutines_with_offset`: Increments a value in a structure pointed to by a1, then calls another routine using a pointer from a4, and repeats this process using a different value from a1, before branching back to a previous instruction.
- `process_list_with_conditional_call`: Increments a value in a structure, then checks if a certain value in a list or a global variable matches a certain condition, calling another routine if certain values are encountered or if a list-traversal condition is met.
- `validate_and_increment_list_index_2`: Checks if a value at offset 544 of a structure in a1 matches certain constants, and if so, branches to other routines; otherwise, it increments a value at offset 544 of the structure in a1 and continues execution.
- `update_table_entry_and_branch`: Increments a byte at an offset of 536 in a1, compares a longword from a2 to a longword in a1, and if a certain condition is met, loads a byte from a table using a scaled index and jumps to a routine in a6.
- `branch_on_null_pointer`: Tests if a long-word at offset 640 of the structure in a2 is zero, otherwise it jumps to another location, otherwise it clears d1 and jumps to the address in a6.
- `update_pointer_and_validate_index`: Increments a counter in a structure, then enters a loop that reads an index from a table and checks if that index or a subsequent byte matches certain constant values, updating a global variable if a match is found or calling another routine if certain conditions are met; otherwise, it continues or returns via an indirect jump.
- `branch_to_next_instruction_2`: Performs a short branch to a relative offset, bypassing any instructions or data located at the current address.
- `update_pointer_and_branch_conditionally`: Calculates a new index by adding a value from offset 548 of a structure in a1 to a constant, uses this to fetch a byte from a table in a0, and stores it in offset 552 of a1.
- `validate_and_update_index_table`: Checks if a value loaded from a structure offset against a list of prohibited or permitted constants, and if a match is found, updates a value in a structure and calls another routine; otherwise, it enters a loop that iterates through a table, updating structure members and calling another routine if certain byte values are encountered.
- `add_to_register_and_branch`: Sets d1 to one, adds a long-word from offset 544 of the structure in a1 to d1, and then branches to another location.
- `branch_to_target_address_4`: Performs a short branch to a relative offset or a fixed address in the code.
- `update_global_pointer`: If a certain long-word is not zero, it sets a global variable at offset 676 of a structure in a2 to a constant value and then jumps to another routine.
- `search_and_update_list`: Copies a 32-bit value from a structure in a2 to offset 4 of a1, then loops to check if a value at a certain offset in a0 matches a value in a1, updating a pointer in a1 if a match is not found.
- `update_and_store_pointer`: Increments a 32-bit value at an offset of a structure in a2, calculates a new address using a value from a structure in a2 and a pointer from a1, stores a value at that address, and jumps to an indirect address in a6.
- `set_index_to_max`: Compares a value from a structure in a2 with another value from a structure in a2, and if the first is not greater, sets a value at offset 624 of a2 to negative 255 and branches to another routine, otherwise it calls another routine.
- `update_index_and_load_byte`: Increments a 32-bit value at an offset in a structure, then loads a byte from a table into another offset in that structure using a calculated index.
- `execute_conditional_calls`: Compares a value in a structure to a constant and calls a routine twice if they match, otherwise calls the routine once and jumps to an indirect address.
- `copy_and_call_routine`: Copies a long-word from a structure in a2 to a pointer in a1, copies another long-word from a structure in a2 to an offset of a1, modifies a pointer in a4, and calls another routine.
- `set_global_to_max_and_jump`: Sets d1 to negative one, stores this value as a long-word at offset 668 of the structure in a2, and then jumps to the address in a6.
- `execute_two_routines`: Calls a routine twice with different pointer offsets in a4 and jumps to an indirect address.
- `update_pointer_and_set_value`: Calls a routine, updates a pointer, and if a certain value in a structure does not match a constant, sets a 32-bit value in a structure to a negative constant before calling another routine.
- `branch_on_index_value`: Checks a value from a structure in a1 against several constants, calling other routines or setting a value in a structure in a2 if certain conditions are met, and finally jumps to an indirect address.
- `update_structure_member_on_condition`: Clears a 4-byte value at an offset of 4 from a pointer in a1 and calls another routine, then checks if a value at offset 656 of a structure in a2 matches certain constants; if certain conditions are met, it calls another routine, updates a pointer in a1, and performs a memory write or further routine calls before jumping to a routine in a6.
- `modify_and_store_byte`: Calculates an offset using a value from a structure in a2 and a pointer in a1, stores a byte from a structure in a2 into a memory location, copies a long-word from a structure in a2 to a pointer in a1, modifies a pointer in a4, and calls another routine.
- `load_and_call_routine`: Loads a value from a structure in a2 into d2, loads a value from a structure in a1 into d1, sets a4 to an offset of a4, and calls another routine before branching to another routine.
- `nop_and_branch_2`: Performs a no-operation and then branches to another location.
- `traverse_and_replace_list`: A loop that replaces a value in a list with a byte from a table and updates a pointer, continuing until a null pointer is encountered.
- `branch_to_target_address_3`: Stores a long-word from d1 into an offset of a pointer in a1, then if d1 is not zero, jumps to another routine; otherwise, it overwrites d1 with d2 and jumps to an address in a6.
- `branch_to_target_address_5`: Performs a short branch to a fixed memory address.
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

- `parse_command_line_and_start`: Program entry for the Avail utility that opens the dos library and allocates a control block, then splits the command line into argument strings, honoring double quotes and asterisk escapes.
- `release_file_and_library_state`: Fetches a state pointer from a global library base, stores the given value in it, and conditionally closes a file, closes a library, frees memory and replies to a message based on flags in that state.
- `sum_memory_region_sizes_by_flag`: Walks a linked list of memory region records and adds up the size of each region whose flag bits overlap the given mask.
- `print_memory_availability_table`: Prints a memory table with Type, Available, In-Use, Maximum and Largest columns for chip, fast and total memory.
- `call_reschedule_3`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_with_stack_slot_pointer`: Passes its two arguments to another routine together with a pointer to a fixed slot 12 bytes into the caller's stack frame.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `measure_stack_string_and_forward`: Calls a function pointer from a global variable to fill a 140-byte stack buffer, then counts the characters up to the null terminator.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `add_disk_buffers_to_device`: Program entry for the addbuffers command: it reads the DRIVE and BUFFERS arguments, checks the named device, and reports either its current buffer count or an invalid device or volume error.

### Path

- `find_max_and_prepare_buffer_6`: Iterates through a linked list to find the maximum value in a specific field, then initializes a destination buffer with this value and a sequence of incrementing constants.
- `copy_list_element_header`: copies a fixed-size block of data from a structure or list entry to a stack-based or destination buffer.

### Resident

- `find_max_and_initialize_buffer_4`: Iterates through a linked list to find the maximum value in a specific field and, if a destination pointer is valid, writes a header and copies a sequence of incrementing values and a block of data into a target buffer.
- `copy_block_and_execute`: copies a fixed-size block of 16 dwords from a memory location certain enough to be a structure or buffer back to the stack and then calls another routine.
- `validate_and_update_structure_pointer`: Calculates a new pointer or index, updates a field in a structure, and calls several subroutines to check or update further structure members; if certain conditions are met, it checks a bitmask in a table and updates a field before performing an indirect jump or calling another routine.
- `clear_d1_and_jump`: Clears d1 and jumps to the address in a6.

### Assign

- `find_max_and_initialize_buffer_5`: Iterates through a linked list to find the maximum value in a specific field and, if a destination pointer is valid, initializes a buffer or structure header with this maximum value and a sequence of incrementing values.
- `copy_block_and_execute_2`: Copies a fixed number of words from a memory location or structure offset to a stack-based or temporary buffer and then calls another routine.
- `update_linked_list_node_and_table_2_2`: Calculates an index-based pointer, updates a structure or table with new bounds or pointers, and traverses a linked list or table-based chain, updating node pointers and checking for end-of-list or invalid index conditions, calling another routine to perform certain updates or checks during the traversal.
- `update_linked_list_node_and_table_2`: Updates a linked list node by copying a pointer from one index to another, then enters a loop that traverses and updates subsequent nodes in.
- `update_linked_list_node`: Updates a pointer in a structure or list, checks a condition or flag, and if certain criteria are met, updates a value in a structure or list to a constant or a value from another structure, then calls another routine.
- `update_linked_list_node_and_table`: Updates a linked list node by copying a pointer from one index to another, then enters a loop that traverses and updates subsequent nodes in the list.
- `assign_table_values`: Updates several values in a table using offsets derived from a structure, calls another routine multiple times to check or clear certain flags, and finally jumps to an address in a6.

### EndCLI

- `find_max_and_initialize_buffer`: Walks a linked list to find the maximum value in a specific field, then initializes a buffer or structure header with this value and a sequence of incrementing integers.
- `copy_memory_block`: copies a fixed number of words from a source pointer to a destination pointer.
- `modify_list_and_table`: Modifies a bitmask in a structure, updates a pointer in a structure, and conditionally decrements a value in a table if it is greater than zero.

### Rename

- `find_max_and_initialize_buffer_2`: Iterates through a linked list to find the maximum integer value in a specific field, then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_block_and_call_function`: copies a fixed number of words from a memory location or structure into a stack-based buffer and then executes another routine.
- `update_pointer_and_call`: Updates a pointer in a structure, calls a routine, and conditionally updates a value in a structure or table.

### Dir

- `update_max_header_and_copy_data`: Iterates through a linked list to find the maximum value in a field, writes this value and a sequence of incrementing integers into a header, and then performs a block copy of data between a source and destination if a pointer is valid.
- `copy_fixed_size_block`: copies a fixed number of words from a memory location or table, starting at a certain offset, to the stack.
- `update_entity_bounds_and_state`: Updates several pointer-based offsets in a structure in a1 to define a memory range, clears certain fields, and conditionally updates a structure in a2 with values from a1 or a table, then calls another routine or returns.
- `update_structure_field_from_registers`: If a longword at offset 332 of the structure in a1 is non-zero, it loads three values from offsets 88, 48, and 48 (the same offset) into d3, d2, and d1, then calls another routine using a pointer-adjusted a4, and finally writes the value from d1 into offset 320 of the structure in a1 before branching away.
- `clear_structure_field`: Tests if a longword at offset 352 of a1 is zero; if not, it clears that longword and then branches to another routine.
- `update_entity_property_and_loop`: Updates a field in a structure using a value from another structure, then calls a routine multiple times with different offsets to update or check further fields, and finally loops through a range of indices to clear or set values in a table, or otherwise returns.
- `update_entity_state_and_pointer_2`: Checks a value in a table, and if it equals two, calls a routine to update a field in a structure, then updates a byte in a table using a pointer.
- `update_entity_state_and_pointer`: Checks a value in a table, and if it equals two, calls a routine to update a field in a structure, then updates a byte in a table using a pointer, and subsequently calls several other routines to update or reset various fields in the structure, including updating a pointer-based index and a byte-based index, before finally branching to another routine.
- `update_structure_range_and_set_value`: Checks if certain pointers in a structure in a2 and a structure in a1 are non-zero, then if a certain value is not one, calls a routine, and if a certain flag in a2 is zero, initializes a range of values in a structure in a1 by looping and setting each entry to one, before calling another routine and updating a value in a2 from a structure in a1.
- `set_structure_value_with_offset`: Calculates a pointer or offset by adding a value from a structure in a1 to d1, shifts it right by two, and calls a routine using a pointer from a2.
- `set_structure_value`: Calls a routine, and if a certain value is zero, sets a value at an offset in a structure.

### NewCLI

- `find_max_and_prepare_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or structure header with this value and a sequence of incrementing or fixed data.
- `copy_sixteen_longwords_and_call`: Copies sixteen 32-bit values from a fixed offset in a base register into a caller-supplied buffer when that buffer pointer is non-null.
- `recalculate_bounds_and_copy_table_entries`: Updates a table of bounds or limits by recalculating or copying pointer-indexed values, and then performs a loop to copy a range of entries from one table or array to another, potentially incrementing a counter if a certain value is non-zero.
- `update_table_and_recalculate_bounds_2`: Updates a table entry by copying a value from one pointer-indexed location to another and increments a counter if it is non-zero, then calls another routine to perform further updates or resets on a structure, potentially involving a loop that searches for a zero-value entry in a table or updates a pointer-based bound or counter.
- `update_table_and_recalculate_bounds`: Iterates through a table, updating or resetting certain values, and if a certain condition is met, it updates a pointer or value in a structure and calls another routine multiple times.
- `add_offset_to_d1`: Loads one into d1 and adds a longword from an offset 300 of a1 to d1, then branches to another routine.
- `update_table_and_decrement_logic_branch`: If a certain value in a structure is zero, it calls a routine three times with a constant value in d0 and a pointer from a table in a2; otherwise, it performs a series of pointer-based assignments and calls a routine multiple times, including a calculation that shifts a pointer-based address right by two bits before calling the routine again.
- `update_table_and_decrement_logic`: Checks if a value at an offset in a1 is below a certain threshold, and if so, performs several pointer-based assignments and calls a routine using a pointer from a2.
- `update_table_and_decrement`: Updates a value in a table, calls several routines, and conditionally clears or decrements a value in a table if a certain condition is met.

### NewShell

- `fill_pattern_and_install_handler`: Walks a linked list to find the largest value in a field, then fills a buffer with an incrementing word pattern and records that maximum in its header.
- `copy_fixed_size_block_2`: copies a fixed number of dwords from a memory location certain offset from a pointer into a stack or destination buffer and then calls another routine.
- `update_structure_index_and_table`: Updates a structure's index and table by copying values from a source table to a destination table, and increments or decrements a counter in a global structure if a certain condition is met.
- `sync_structure_index_and_update_table`: Checks if a table entry or structure index is non-zero, copies a value from one table entry to another, and increments or decrements a counter in a structure.
- `update_structure_index_and_sync_values`: Checks if a table entry is zero and updates it with a calculated value, then enters a loop that updates a structure's index or clamps a value to 800, while calling another routine and performing several conditional updates to a structure and a table.
- `increment_and_branch`: Loads a value from a table or structure at offset 304 into a register, decrements it by one, and branches to another routine.
- `sync_and_update_structure_value`: Checks if a value at offset 692 of a structure in a1 is zero; if not, it clamps a value at offset 276 of a1 to 800, synchronizes a secondary field at offset 268 with a value from offset 276, and calls a routine using a pointer from a2, then performs further updates to a table and structure using a pointer from a2 and a pointer from a4.
- `clamp_and_sync_structure_value`: Ensures a value in a structure does not exceed 800, synchronizes a secondary field with that value, and then executes a series of calls to update various structure members and table-based pointers or counters based on whether the primary value is zero or non-zero.
- `update_structure_and_table`: Updates a value in a structure, calls several routines, and conditionally clears or decrements a value in a table if a certain condition is met.

### Echo

- `find_max_and_update_buffer`: Iterates through a linked list to find the maximum value in a field, then writes a header and copies a sequence of longwords into a destination buffer if a condition is met.
- `copy_fixed_block_and_execute`: copies a fixed number of words from a memory location or table to the stack and then calls another routine.
- `update_table_range_and_iterate_2`: Updates a range or pointer values in a structure by calling a routine to retrieve new indices, then enters a loop that iterates through a range of indices, updating a structure's range or pointer values.
- `update_table_range_and_iterate`: Updates a structure's range or pointer values by calling a routine to retrieve new indices, then enters a loop that iterates through a range of table entries, calling a routine at each step and updating a pointer or index in the structure.
- `copy_longword_to_offset`: Copies a longword from offset 336 of a1 to offset 328 of a1, then branches to another routine.
- `adjust_list_bounds_and_iterate`: Sets a longword at offset 328 in a1 to 1, checks certain values in a structure, and if certain conditions are met, calculates a new index and branches back to an earlier instruction or to another routine.
- `copy_longword_to_offset_alt`: Copies a longword from offset 336 of a1 to offset 332 of a1, then branches to another routine.
- `update_list_bounds_and_iterate`: Updates two pointer-sized values in a structure, checks if a certain range is valid, and if so, iterates through a table to read a byte from an offset calculated from a value in the structure, then calls another routine and updates a value in the structure before branching.
- `check_structure_and_call`: Checks a value at an offset in a1, and if it is not zero, calls a routine using a pointer from a2.
- `conditional_routine_calls`: Checks a value in a structure and calls a routine or a sequence of routines if certain conditions are met.

### cls

- `open_dos_library`: Opens a library by name through the exec library table and calls two more functions on the returned base.

### Execute

- `find_max_and_prepare_buffer_3`: Iterates through a linked list to find the maximum value in a specific field, then initializes a destination buffer with a header and copies or writes a block of data if a pointer is non-null.
- `copy_fixed_size_block_3`: copies a fixed number of words from a memory location or structure offset to a stack or memory destination.

### Copy

- `find_max_and_copy_buffer`: Traverses a linked list to find the maximum value in a specific field, then writes a header and copies a block of data or a fixed number of longwords if a destination pointer is valid.
- `copy_memory_block_2`: copies a fixed block of sixteen bytes from a memory location offset from a register or pointer to a destination in the stack.
- `update_table_index_and_traverse`: Calculates a new index or size for a table entry in a structure, clears certain fields in a secondary structure, and then enters a loop that traverses a table of pointers, checking for specific byte-encoded command or jump values and calling another routine if certain conditions or specific values are met.
- `update_list_index_from_table`: Calculates a new index using a value from a table, retrieves two bytes from a data structure using this index, and stores these bytes into a structure at offset 544 and 548, then branches to another routine.
- `execute_two_subroutines_with_offset_2`: Increments a value in a structure pointed to by a1, then calls another routine using a pointer from a4, and repeats this process using a different value from a1, before branching back to a previous instruction.
- `process_list_with_conditional_call_2`: Increments a value in a structure, then checks if a certain value in a list or a global variable matches a certain condition, calling another routine if certain values are encountered or if a list-traversal condition is met.
- `validate_list_index_range`: Checks if a value loaded from an offset in a structure matches any of several hardcoded constants, and branches to different subroutines if a match is found or otherwise continues execution.
- `copy_range_to_structure`: Increments a value at an offset of 536 from a1, compares a value from a2 to that new value, and if it is not greater or equal, checks a byte from a table using a shifted offset from a1 and jumps to a routine in a6 if a certain condition is met.
- `check_long_value`: Tests a long-word in a structure and otherwise jumps to a new address or sets d1 to zero and jumps to a pointer in a6.
- `update_index_table_and_validate_constants`: Increments a counter in a structure, then iterates through a list of offsets to check if the bytes loaded from a table match certain permitted or prohibited constants, updating a global value if a match is found or calling another routine if certain conditions are met; otherwise, it returns or jumps to another location.
- `branch_to_next_instruction`: Performs a relative branch to a nearby instruction, effectively acting as a jump or a single-instruction stub that redirects execution flow.
- `update_pointer_value_and_verify`: Updates a value in a structure at offset 552 using a byte read from a table, then compares this new value against a global variable at offset 700; if they match, it performs a secondary table lookup and calls another routine.
- `validate_index_range`: Compares a value loaded from offset 552 of a structure in a1 against several hardcoded constants and branches to different locations if a match is found, otherwise it branches to a final instruction.
- `add_to_d1_and_branch`: Sets d1 to one, adds a value from a structure at an offset of 544 to d1, and then branches to another routine.
- `branch_to_offset`: Performs a relative branch to a nearby address.
- `set_constant_d2`: Sets a 32-bit value in a structure at offset 424 of a2 to a constant and then jumps to another routine.
- `compare_and_update_pointer`: Compares a value from a structure in a1 with d1, and if certain conditions are met, calculates a new address using a value from a structure in a2 and calls a routine.
- `increment_and_copy_value`: Increments a value in a structure and copies a word from a source pointer to a destination pointer in a table.
- `compare_and_set_constant`: Compares a value from a structure in a2 with another value from a structure in a2, and if certain conditions are met, sets a value in a structure in a2 to a constant and branches.
- `increment_and_store_byte`: Increments a value in a structure and stores a byte from a source pointer into a destination pointer in a structure.
- `check_table_and_call`: Compares a value in a structure to a constant and calls a routine if they are equal.
- `copy_structure_data`: Copies a 32-bit value from a structure in a2 to a pointer in a1, copies another 32-bit value from a2 to an offset in a1, and calls another routine before jumping to another routine.
- `set_value_and_jump`: Sets d1 to negative one, stores it as a long-word into a structure at an offset of 712, loads a long-word from a pointer in a1 into d1, and jumps to a pointer in a6.
- `execute_two_routines_2`: Calls a routine twice with different pointer offsets in a4 and jumps to an indirect address.
- `update_conditional_value`: Calls a routine, updates a pointer-based value, and if a certain value in a structure in a2 does not equal 41, sets a value at offset 704 of that structure to -1 before calling another routine and jumping to an indirect address.
- `validate_and_branch_on_type_id`: Checks a value loaded from a pointer in a1 against several constants; if the value matches certain constants, it calls another routine, potentially updates a long-word in a structure, and then jumps to an indirect address, otherwise it sets a long-word in a structure to negative one and jumps to an indirect address.
- `execute_subroutine_loop`: Clears a 4-byte value at an offset of 4 in a pointer-based structure, calls another routine, and then enters a loop that updates a 4-byte value at an offset of 8 in that structure, continuing or branching based on a comparison with a value in a structure in a2, before calling another routine and jumping to a location in a6.
- `update_pointer_and_store`: Calculates an offset using a value from a structure in a2 and a value from a structure in a1, stores a byte from a structure in a2 at an offset in a0, copies a 32-bit value from a2 to a1, and calls another routine before jumping to another routine.
- `load_parameters_and_call_subroutine`: Loads a longword from a global variable into d2 and a longword from a structure offset 8 into d1, sets d0 to a constant, updates a pointer in a4, and calls another routine before branching to a different code location.
- `do_nothing_and_branch`: Performs a no-operation and then branches to another routine.
- `copy_linked_list_bytes`: Loops through a list of bytes, copying a byte from a source table to a destination, and updates a pointer in the list, continuing until a null pointer is encountered, then jumps to an indirect address.
- `branch_to_target_address_3_2`: Stores a long-word from d1 into an offset of a pointer in a1, then if d1 is not zero, jumps to another routine; otherwise, it overwrites d1 with d2 and jumps to an address in a6.
- `branch_to_offset_2`: Performs a relative branch to a nearby address.
- `copy_and_replace_bytes`: Loops through a list of bytes, copying a byte from a source table to a destination, and updates a pointer in the list, continuing until a zero byte is found, then copies a byte from a different offset and jumps to an indirect address.
- `clear_and_populate_list`: Clears several 32-bit values in a structure, loads a byte from a table using a pointer and an offset, and then loops to clear a range of bytes in a table.
- `execute_three_subroutines`: Calls a routine three times with different offsets of a4, then negates a value from a structure in a2 and jumps to an indirect address.
- `copy_table_values`: Reads bytes from a table using an offset, stores them into a structure, and enters a loop that copies bytes from a table to another table location while updating a counter.
- `store_sum_to_table_2`: Calculates the sum of two 32-bit values from a structure and stores the lower byte of that sum into a table at an offset determined by a third value in the structure.
- `update_buffer_with_looping_call`: Copies a byte from a table using an index, then enters a loop that calls another routine and writes a byte from a table into a buffer, incrementing a counter or pointer until a certain value is reached, otherwise it returns.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `swap_and_store_pointer_2`: Calls a routine, sets a value in a structure, and if a certain value is zero, calls another routine twice and sets a pointer to a new address.
- `swap_and_store_pointer`: Copies a 32-bit value from a source table to a location in a structure in a2, then copies a value from a list into that source table, and finally updates a pointer in that structure, then jumps to an indirect address.

### Makedir

- `find_max_and_populate_buffer`: Iterates through a linked list to find the maximum value in a specific field, then writes a header and copies a block of data or a pattern of incrementing values into a destination buffer.
- `copy_memory_block_3`: copies a fixed number of words from a memory location or a pointer-offset to a destination buffer.
- `execute_conditional_logic_2`: Performs several routine calls and updates various offsets in a structure based on values retrieved from a source table.
- `execute_conditional_logic`: Performs several routine calls and updates various offsets in a structure in a1 based on values retrieved from a source table, including conditional logic that sets a value to 10 if a certain check fails, before jumping to an indirect address.

### List

- `find_max_and_initialize_buffer_3`: Iterates through a linked list to find the maximum value in a specific field, then writes this value to a header followed by an incrementing sequence of integers.
- `copy_fixed_size_block_4`: copies a fixed number of longwords from a memory location offset from a register pointer to a stack or memory location, then calls another routine.
- `update_table_and_traverse_links`: Updates a structure's size or index field, then enters a loop that traverses a linked list or table of pointers, checking for specific byte-encoded command or jump values and calling another routine if certain conditions or specific command bytes are encountered.
- `update_list_index_and_cache_values`: Calculates a new index using a base offset and a value from a table, then reads two bytes from a structure at that calculated offset and stores them into a structure at offsets 544 and 548, before branching to another routine.
- `update_list_index_and_branch`: Increments a byte at offset 536 of a structure in a1, then calls another routine with a new pointer or offset, and subsequently calls another routine again using a value loaded from offset 548 of the same structure before branching back to the start of the routine.
- `update_list_index_and_traverse_table`: Increments a byte at offset 534 in a structure pointed to by a1, then checks a longword at offset 548 in a1 against a longword in a2; if the longword at offset 548 is non-zero, it calls another routine and then performs a second check of the longword at offset 548 before calling another routine again and branching back to the start of the routine.
- `validate_list_index_range_2`: Checks if a value loaded from an offset in a structure matches certain predefined constant values, branching to different subroutines if a match is found or otherwise continuing execution.
- `copy_range_to_structure_alt`: Increments a value at an offset of 536 from a1, compares a value from a2 to that new value, and if it is not greater or equal, checks a byte from a table using a shifted offset from a1 and jumps to a routine in a6 if a certain condition is met.
- `test_and_jump_to_subroutine`: Tests if a long-word at an offset of 612 in a2 is non-zero, otherwise it sets d1 to zero and jumps to the address in a6.
- `validate_and_update_index_table_2`: Iterates through a list of indices, checking if certain bytes loaded from a table match a set of permitted or prohibited constants, and if a match is found, updates a value in a structure and calls another routine; otherwise, it continues or returns via an indirect jump.
- `branch_to_target_address_6`: Performs a short branch to a relative offset, effectively redirecting execution flow to a new location.
- `update_pointer_and_branch_on_value`: Calculates a new index by adding a value from offset 548 of a structure in a1 to a base offset, retrieves a byte from a table in a0 using this index, and stores it in offset 552 of the structure in a1.
- `validate_index_against_set_of_constants`: Checks if a value loaded from a structure offset against a list of specific constant integers and branches to different targets or another routine depending on whether a match is found.
- `add_and_branch_to_subroutine`: Sets d1 to one, adds a long-word from a1 to d1, and then branches to another routine.
- `branch_to_target_address`: Performs a short branch to a fixed offset in another routine.
- `set_global_pointer`: Sets a global variable at offset 616 of a structure in a2 to a constant value, otherwise it returns.
- `update_and_store_pointer_2`: Sets a value in a structure, and then loops to compare a value from a table against a value in a structure, potentially updating a pointer and a table value.
- `increment_and_store_pointer`: Increments a value at offset 676 of a structure in a2, calculates an offset using a value from a structure in a2 and a pointer from a1, and stores a value from a1 at that calculated address before jumping to a target.
- `update_index_and_load_byte_2`: Compares a value from a structure and, if it is within a certain range, sets a value in a structure to a constant.
- `update_value_from_table`: Increments a value at offset 696 of a structure in a2, calculates an offset using a value from a structure in a2 and a pointer from a0, and stores a byte from that table into a value at offset 692 of a structure in a2 before jumping to a target.
- `conditional_subroutine_execution`: Compares a value at offset 692 of a structure in a2 with a constant, and if they are equal, calls a subroutine twice using a pointer or a value derived from a pointer before jumping to a target.
- `copy_structure_members`: Copies a 32-bit value from a structure in a2 to a pointer in a1, copies another 32-bit value from a structure in a2 to an offset of 4 in a1, then calls another routine and branches to a new location.
- `set_and_jump_to_subroutine`: Sets d1 to negative one, stores this value as a long-word at an offset of 640 in a2, loads a long-word from a1 into d1, and jumps to the address in a6.
- `execute_two_subroutines_and_jump`: Sets d0 to 32, loads a pointer into a4, calls a routine, sets d0 to 20, loads a pointer with an offset of 164 into a4, calls a routine, loads a long-word from a1 into d1, and jumps to the address in a6.
- `update_structure_with_subroutine_result`: Calls a subroutine, copies the result back to a pointer in a1, and if a value at offset 628 of a structure in a2 is not 41, sets a value at offset 640 of a structure in a2 to -1, then calls another subroutine before jumping to a target.
- `check_value_and_branch`: Loads a long-word from a pointer in a1, compares it against several constants, and branches to another routine if a match is found or if the value is certain other constants.
- `execute_conditional_logic_3`: Clears a 32-bit value at an offset of 4 in a pointer, calls another routine, and then enters a loop that calls another routine while checking if a value in a structure in a2 matches certain constants, or otherwise calls another routine and jumps to a location in a6.
- `update_table_and_call`: Calculates an offset using a value from a structure in a2 and a pointer in a1, copies a byte from a table in a0 to a pointer in a1, copies a 32-bit value from a structure in a2 to a pointer in a1, then calls another routine and branches to a new location.
- `load_and_call_subroutine`: Loads a long-word from an offset of 632 in a2 into d2, loads a long-word from an offset of 8 in a1 into d1, loads the constant 24 into d0, loads a pointer or address or offset 128 in a4 into a4, calls a routine using the address in a5, and branches to another routine.
- `nop_and_branch`: Performs a no-operation and then branches to another routine.
- `process_linked_list_and_swap`: Loops through a list where each element contains a pointer to the next, reading a byte from a table and writing it to a certain offset, and swaps a byte from a pointer in a1 into a table before checking if the pointer is null.
- `update_memory_and_jump`: Stores the contents of d1 into a memory location at an offset of 8 from a1, then checks if d1 is zero and either branches or jumps to an address in a6.
- `branch_to_target_address_2`: Performs a short branch to a fixed address in another routine.
- `traverse_and_replace_list_element`: Loops through a list, replacing a value in the list with a byte from a table, and once a certain byte is found, writes a byte from a pointer in a1 into a table before jumping to a target.
- `initialize_and_clear_table`: Clears several 32-bit or 16-bit values in a structure pointed to by a2, copies a byte from a table (indexed by d1 multiplied by 4) into a structure field, and then loops to clear a range of bytes in a table (indexed by d1 and a pointer from a2) until a certain value is reached.
- `execute_subroutines_and_invert_value`: Calls a subroutine three times with different parameters or pointers, and then calculates the bitwise NOT of a value at offset 640 of a structure in a2 before jumping to a target.
- `branch_to_offset_4`: Performs a short branch to a relative offset.
- `check_value_against_list_2`: Loads a byte from a pointer in a1 and compares it against a list of constant values, otherwise it calls another routine.
- `set_d1_and_jump`: Sets d1 to negative one and jumps to the address in a6.
- `set_d1_and_jump_2`: Sets d1 to zero and jumps to the address in a6.
- `check_value_against_list`: Loads a value from a pointer in a1 and compares it against several constant values, otherwise it jumps to another routine.
- `check_pointer_and_jump`: Checks if a pointer in a1 is null, otherwise it clears d1 and jumps to an address in a6.
- `process_list_and_call`: Copies a byte from a table in a0 to a pointer in a1, updates a value in a structure in a1, and if a certain condition is met, calculates a new offset, copies a byte from a table in a0 to a pointer in a1, and calls another routine.
- `add_structure_offset_to_d1`: Sets d1 to one, adds a value from an offset 8 of a1-pointed structure to d1, and branches to another routine.
- `jump_to_address`: Clears d1 and jumps to an address in a6.
- `reset_string_bounds_and_update_display`: Resets several string index bounds and pointer-based array limits in a structure pointed to by a1, calls a routine to initialize string bounds, and then updates certain display-related pointer or size fields in the structure before branching to another routine.
- `initialize_string_index_bounds`: Sets certain array elements in a structure pointed to by a1 to a default value and then uses a routine to check or copy data from a source string or block, potentially updating a global or structure-based index or terminator.
- `parse_string_and_update_index_2_2`: Reads a pointer from a structure, checks if the target string or data block is null, and if not, copies bytes from that source or a secondary source into a destination buffer while updating an index and a length or end-marker flag in the structure.
- `branch_to_offset_3`: Performs a relative branch to another routine.
- `parse_string_and_update_index_2`: Parses a string or a sequence of bytes from a structure, updating several offset-based pointers and flags in a structure, and checks for a termination character or a specific length limit, otherwise it loops through the string or a pointer-based sequence.
- `parse_string_and_update_index`: Checks if a certain flag in a structure is zero and if a pointer in a table is non-null, then calls another routine to process a string.
- `update_table_and_rebuild_index_2`: Updates a byte in a table using an index-based offset, updates a pointer or counter in a structure, and then enters a loop that copies bytes.
- `update_table_and_rebuild_index`: Updates a byte in a table using an index-based offset, updates a pointer or counter in a structure, and then enters a loop that copies bytes from a source table to a destination table while updating a secondary index or pointer.
- `modify_table_byte`: Decrements a longword loaded from an offset of 1988 in a1, shifts a longword loaded from an offset of 1448 in a1 left by two, and stores the resulting byte into a table at a calculated offset in a0, then branches to another routine.
- `update_pointer_logic_and_display_index`: Clears a byte in a table using an index, checks a flag in a structure, and if certain conditions are met, calculates and stores a new pointer or a bitwise-inverted address into a structure, then calls a routine to update a pointer or display logic.
- `calculate_shifted_address`: Loads an address from an offset 4904 of a4-pointed structure into a3 and d1, shifts d1 right by two, and branches to another routine.
- `update_pointer_logic_and_display`: Updates a pointer in a structure using an index, potentially updates a table entry with a calculated address, and executes a sequence of calls to a routine using a constant or a pointer-derived value, followed by conditional updates to a structure's field and a final check of a pointer-based value to update a certain field or set a constant value.
- `update_pointer_logic`: Calls another routine using a pointer from a structure, then checks a byte in a table; if the byte is zero, it calls another routine and updates a value in a structure using a pointer-based offset, otherwise it calls another routine and updates a pointer-based value in a structure.
- `update_display_bounds_and_offset_4`: Calculates a bit-shifted value from a pointer-based address, stores it in a structure, and calls another routine to check if the resulting value is zero; if zero, it resets a different structure field to zero using another routine call, otherwise it updates a pointer-based offset or boundary.
- `reset_display_bounds_and_offset_2`: Updates a display boundary or offset in a structure in a1 by calling another routine to retrieve a value, then checks if that value is zero to either reset a display parameter or calculate a new boundary or offset based on a pointer in a2 and calls several other routines to update display parameters or reset certain values.
- `reset_display_bounds_and_offset`: Checks if a display offset or boundary (stored at offset 584 of a structure in a1) is negative, and if so, updates a value in a structure (a), then resets certain display parameters or calculates a new display boundary or offset (via a call to another routine) and updates a structure (a).
- `update_display_bounds_and_offset_logic`: Checks if a display offset or boundary (stored at offset 584 of a structure in a1) is negative, and if so, updates a value in a structure (at offset 648 of a1) with the bitwise NOT of that offset.
- `update_scaled_display_offset`: Calculates a new scaled display offset by loading a value from a pointer in a1, shifting it left by two, and storing it into a structure offset in a0, then updates a pointer in a1 with a value from a2 and calls another routine.
- `update_structure_with_shifted_address`: Loads an address from an offset 4788 of a4-pointed structure into a3 and d1, shifts d1 right by two, stores d1 into a structure at offset 604 of a2-pointed structure, and branches to another routine.
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
- `call_subroutine_with_constants`: Sets d1 to 45 and d0 to 24, loads an address from an offset 224 of a2-pointed structure into a4, calls a routine at the address in a5, and jumps to an address in a6.
- `update_display_index_parameters`: Updates a display index or offset or a pointer-based table by checking a value against a range and calculating a new index or offset, then calls another routine with varying parameters.
- `test_and_mask_bit_one`: Tests if the least significant bit of a register is set and, if so, clears all but the first bit of that register before branching or moving a value into it.
- `execute_indirect_calls`: Loads an address from a pointer in a2, calls the address, and then jumps to another address held in a6.
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

- `find_max_and_populate_buffer_2`: Walks a linked list to find the maximum value in a specific field, then writes a header and copies a block of data into a destination buffer if a pointer is valid.
- `copy_data_block`: copies a fixed number of words from a memory location to the stack.
- `update_structure_and_call_subroutines`: Updates a bit in a byte using a bit-shift, modifies several pointer-based offsets in a structure, and executes a sequence of conditional calls to other routines based on the contents of a pointer-indexed table or a structure-relative offset.
- `check_structure_and_call_2`: Calculates a bit-shifted address from a4, calls another routine, and if a certain value at an offset in a1, scaled by a pointer, is non-zero, calls another routine and updates a global variable with a value from a pointer in a2.
- `set_buffer_and_draw`: Loads a pointer from a2 plus 1544, calls a routine with a length of 720, and if the loaded value is zero, calls another routine and sets a value at a1 plus 704.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_prepare_buffer_4`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or structure header with this value and a sequence of incrementing or decrementing integers.
- `copy_memory_block_4`: Copies a fixed number of words or dwords from a source pointer to a destination pointer.
- `update_object_state_and_indices`: Updates a pointer in a structure, clears several fields at certain offsets, and then checks a series of table-indexed values to conditionally update or reset other fields in the structure, including checking for a specific character '$' in a table.
- `check_table_index_and_set_value_2`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a structure in a0; if that byte is non-zero, it compares a value of 10 in d2 against a longword at offset 548 in a2 and, if less, sets d1 to 255 before branching to another routine.
- `check_table_index_and_set_value`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a structure in a0; if not zero, it compares a value in a2 to 20 and, if less than, sets d1 to a certain value before branching.
- `traverse_and_update_structure_pointers`: Traverses a linked-list-like structure or table-based hierarchy by updating pointers in a structure in a1 based on values found in a table in a0, and calls another routine if certain character or pointer conditions are met.
- `validate_and_update_structure_state`: Checks if certain characters in a table-driven structure match a specific character, and if so, calls another routine to update or reset structure members.
- `branch_if_member_non_zero`: Performs a conditional branch to a distant instruction if a longword at offset 40 of the structure pointed to by a1 is non-zero; otherwise, it returns immediately.
- `test_structure_member_and_branch`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a0; if not zero, it compares a longword at offset 388 in a1 to 1 and, if not equal, sets d1 to a certain value and branches.
- `branch_if_member_non_zero_2`: Performs a conditional branch to a target address if a longword at offset 40 of the structure pointed to by a1 is non-zero; otherwise, it returns immediately.
- `test_structure_member_bits_branch_logic`: Checks if a longword at offset 356 of the structure in a1 is equal to one, and if not, tests if that same longword is zero to determine whether to branch to another routine, branch to a different routine, or branch to a third routine with d1 set to a negative value.
- `update_object_index_and_call_subroutines`: Checks if a pointer-indexed array entry is non-zero, calls a routine, and updates a field in a structure with the returned pointer-index.
- `reset_and_initialize_object_state`: Resets a flag in a structure, checks a pointer-indexed array, and if certain conditions are met, clears a field, sets a counter to ten, and calls another routine.
- `update_object_bounds_and_visibility`: Updates a size or count in a structure, then checks or updates bitmasks or visibility flags in a structure, potentially looping to update a value in a global variable until a certain condition or limit is met, and finally calls another routine.
- `update_object_bounds_and_visibility_2`: Calls another routine to update a value in a structure, loops to check if that value is certain constants, and if certain flags in a1 are zero, calls another routine to update a value in a structure and calls another routine to update a bitmask in a structure.

### LoadWB

- `parse_args_and_launch_program`: Allocates a task record and opens the dos library and standard handles, then either opens a named file from the startup message or splits the command string into arguments, honouring quoted strings and star sequences for newline and escape characters.
- `release_process_resources_and_reply`: Stores its parameter as the return value, then closes a file handle, closes a library base and frees a memory block, each step guarded by a stored pointer.
- `start_resident_tool_task`: Opens the icon and intuition libraries and decodes a fixed string in place with a one-byte key.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte.
- `build_lookup_node_list`: Walks a chain of table entries, allocating a node for each and storing in it the result of a library lookup on the entry's value.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_indirect_function`: Calls a function pointer retrieved from a memory location relative to a global pointer.
- `call_indirect_function_offset`: Calls a function pointer retrieved from a memory location with a different offset relative to a global pointer.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_exec_dispatch`: Calls the Dispatch routine of the exec library through a stored library base and returns its result to the caller.
- `call_find_resident_2`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_with_local_stack_slot`: Forwards its two arguments to a second routine together with the address of a slot on its own stack, then returns.
- `format_message_and_forward`: Calls the RawDoFmt routine of the exec library through a stored base to format text into a 140-byte stack buffer.
- `store_byte_at_address`: Writes the low byte of a data register to the address held in an address register.
- `compare_string_prefixes`: Checks if a2 is null or certain, then checks if a byte at a2 plus 20 is '2'. If so, it compares a string at a2 with a global string, potentially padding with dots if a null terminator is found.
- `write_dot_and_four_spaces`: Writes a dot followed by four space characters into a buffer at a fixed offset, filling five bytes.
- `advance_while_matching_global_prefix`: Advances a buffer pointer for as long as its characters match a global string, stopping at the first mismatch or at the end of the global string.
- `empty_routine_2`: The routine performs no observable operation and ends with a return.

### Version

- `setup_task_and_parse_arguments`: Allocates a task record and opens the dos library, then either fetches a stored handle or parses the command string into arguments, honouring quoted strings and star escapes for newline and escape characters.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses command-line arguments to open a named library or device, then compares its version and revision with the values supplied.
- `forward_args_to_two_helpers`: Passes its first argument together with the constant 128 to one helper, then passes its second argument to a second helper and returns.
- `parse_integer_string`: Converts a decimal string to an integer, accepting an optional leading plus or minus sign.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Returns the number of characters before the terminator of a null-terminated string, by advancing to the terminator and subtracting the start address.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: loads a function pointer from a memory location and jumps to it via an indirect call.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `execute_indirect_handler`: Loads a function pointer from an offset of a global variable and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: loads a function pointer from a memory-offset or table and executes it.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: loads a function pointer from a memory-offset and executes it.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer`: Stores a single byte from a register into a location pointed to by a register.

### SetMap

- `initialize_execution_environment`: Initializes the game's internal state by resolving library pointers, checking for valid memory allocation, and parsing command-line arguments or tool-type strings into a buffer while handling quoted strings and escape-like character substitutions.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `execute_indirect_function`: Loads a function pointer from a global offset and executes it.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `call_reschedule`: Calls the Reschedule routine of the exec library through the stored base and returns its result to the caller.
- `release_table_entries_and_finish`: Walks a table from the last entry back to the first, calling a routine on each non-zero entry with its value shifted right by two.
- `store_pair_and_run_setup`: Stores its two arguments into global variables, then calls a routine with a fixed set of constants and global addresses.
- `load_keymap_for_console`: Opens the console device, icon and keymap resource libraries, then takes the keymap name from the command arguments or the icon's tool types.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `execute_indirect_function_2`: loads a function pointer from a global table and executes it.
- `call_library_vector_450`: Calls a library routine at a fixed offset through the stored library base and returns its result.
- `execute_indirect_function_pointer_4`: loads a function pointer from a global table and calls it.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `execute_indirect_function_3`: dereferences a pointer located at a fixed offset from a global base address and executes the routine found there.
- `execute_indirect_function_4`: loads a function pointer from a de-referenced global table and calls it.
- `execute_indirect_function_5`: loads a function pointer from a location relative to a global base address and jumps to it.
- `call_indirect_pointer`: Calls a function pointer retrieved from a global structure.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_366`: Calls a library routine at a fixed offset through the stored library base, with no arguments set up here, and returns its result.
- `execute_indirect_function_6`: loads a function pointer from a location offset from a global pointer and executes it.
- `execute_indirect_function_7`: fetches a function pointer from a global table and executes it.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_stored_value_with_slot`: Forwards a value held in a global variable together with its own argument and the address of a slot on its stack to a second routine, then returns.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `invoke_and_checksum_stack_buffer_2`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack-allocated buffer until a null terminator is found.
- `store_register_to_memory`: Stores a byte from a data register into a memory location pointed to by an address register.

### More

- `initialize_filesystem_context`: Obtains the current directory and opens a file or directory, then parses its contents or path string into a structured buffer.
- `update_and_cleanup_context`: Updates a state-tracking structure with a new parameter and conditionally executes a sequence of cleanup callback functions stored in a table, passing a specific identifier to each before the call.
- `run_file_pager_loop`: Opens a window and runs an interactive loop over a file, showing a progress percentage and a quit prompt.
- `iterate_table_with_function`: Loops four times over a global table of values, calling a routine with each value and with a fixed field of the structure it is given.
- `validate_input_buffer`: Calls a routine twice to fill or read an eighty-byte buffer, then scans its bytes.
- `dispatch_pager_key_command`: Waits for an input signal with a timeout, then reads a key code and maps it to a pager action such as help, search or an escape-sequence key.
- `execute_page_routines`: Checks if a value at a2 plus 60 is positive, then calls a routine using a2, calls another routine using a2, and jumps back to the start.
- `stub_unreached_gap_code`: The recovered body is only a short call sequence ending in an indirect call, and it was reached only as a gap in the decode.
- `initialize_state_and_loop`: Sets a counter to one and clears a flag in the structure, then calls a routine and stores its result.
- `update_result_and_increment_counter`: Calls a routine twice with arguments derived from the structure pointer and stores the second result in a field.
- `scan_line_mark_form_feed`: Fills a buffer through a routine, then scans it up to its length for a newline or form-feed character.
- `run_display_loop_until_quit`: Chooses a loop count from flags in the structure, and each pass calls a routine and forwards up to that many global values to another routine.
- `handle_search_loop`: Selects a global string according to a flag in the argument block and runs a search over it, repeatedly calling a routine and counting down until the search reports a match.
- `run_wrapped_call_with_cleanup`: Runs a setup call, a call with the parameter block, an optional call that depends on a flag, then the main call, keeping its result.
- `check_flag_and_call`: Calls a routine with its first two arguments, then calls it again with the first argument only when a flag field in the structure is zero.
- `call_handler_pair_from_struct`: Calls a routine with a stored value and a minus-one argument, then calls it again with a short field read through a pointer held in the structure.
- `call_handler_with_decremented_field`: Calls a routine with a zero argument and the constant one, then calls it again with a short field read through a pointer held in the structure, minus one.
- `emit_text_lines_until_break`: Clears a flag and counts the lines it outputs, limited by a field in the structure capped at 128 characters.
- `call_inner_then_dispatch_handler`: Calls a routine with the structure pointer, then calls a handler stored in the structure with a global address and the first argument.
- `set_flag_and_forward`: When a field in the structure is zero, it sets a short field in a linked structure to one.
- `dispatch_by_flag`: Chooses between a global address and a value read from a linked structure, depending on a flag in the block.
- `dispatch_handler_by_flag`: Calls a routine with a stored handle and a global address, followed by either two more global addresses or a short value read through a linked structure, depending on a flag in the block.
- `clamp_negative_result`: Calls a routine twice with the same first argument and two different trailing values, then returns the second result with negative values replaced by zero.
- `consume_from_buffer`: Takes up to the requested number of bytes from a buffer tracked inside the structure, first calling a refill routine when the request exceeds what is available.
- `reset_buffer_and_refill`: Resets the buffer bookkeeping and calls a routine to fill the remainder of a 4096-byte block, adding the returned count to the remaining size when it is positive.
- `update_scroll_window`: Computes a target offset from a mode argument and compares it with the current window bounds.
- `read_line_from_buffer`: Reads bytes one at a time through a routine into a destination buffer, stopping after a newline or a fixed length limit, then appends a null terminator.
- `copy_null_terminated_string`: Copies bytes from a source to a destination until a null terminator has been copied.
- `convert_string_to_uppercase`: Converts each lowercase ASCII letter in a null-terminated string to uppercase in place, leaving other bytes unchanged.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated string, accepting an optional leading plus or minus sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a unable to be determined and a provided mask.
- `process_text_and_restore_state`: Processes a text string by calling a routine with it, or with the structure when the string is empty, then restores several saved fields and, when a flag is set and the string is non-empty, reads characters until a line feed.
- `build_message_buffer`: Looks up a handle through a routine, places the argument in a one-word stack buffer, then passes that buffer with a constant tag and a length of one to a second routine.
- `copy_fields_from_allocated_block`: Allocates a 36-byte block and, if successful, passes it to a routine together with a constant.
- `allocate_object_and_copy_words`: Looks up a handle and allocates a 68-byte object, linking it to that handle and to a stored value.
- `call_supervisor_2`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr_2`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule_2`: Calls the Reschedule routine of the exec library through a stored base and returns its result to the caller.
- `call_exec_switch`: Calls the Switch routine of the exec library through the stored base, with no arguments set up here.
- `call_dispatched_function`: Calls a function pointer located at an offset from a global variable.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable_2`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Calls the AllocMem routine of the exec library through the stored library base and returns its result to the caller.
- `call_alloc_abs_2`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `execute_indirect_function_8`: Loads a function pointer from an offset within a global table and executes it.
- `execute_indirect_function_pointer_5`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_6`: Fetches a function pointer from a location relative to a global pointer and executes it.
- `execute_indirect_function_9`: Loads a function pointer from a global memory location and jumps to it.
- `call_indirect_function_2`: loads a function pointer from an offset within a global table and executes it.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `execute_indirect_function_10`: Loads a function pointer from a table and executes it.
- `call_library_function_offset_174`: Calls a functionget pointer located at an offset from a global variable.
- `call_library_function_offset_180`: Calls a function pointer located at an offset from a global unable to any or other global variable.
- `execute_indirect_handler_2`: Executes a function pointer retrieved from a global structure offset.
- `execute_indirect_function_pointer_7`: loads a function pointer from a memory location and executes it.
- `execute_indirect_function_11`: loads a function pointer from a global table and executes it.
- `recurse_with_stack_slot_2`: Calls another routine passing two parameters and a pointer to a fixed slot 12 bytes into the caller's stack frame.
- `extract_byte_from_call`: Calls a routine to read one byte into a four-byte stack buffer.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer_2`: copies a byte from a data register to a memory location pointed to by a register.
- `divide_32bit_with_remainder_2`: Divides a 32-bit value by a divisor, using a bit-by-bit shift-and-subtract loop for large divisors and two 16-bit divisions for small ones, and returns the remainder and quotient together as a pair of longwords.
- `multiply_32bit_partial_products`: Computes the low 32 bits of the product of two 32-bit values from 16-bit partial products, returning that product in the high longword.
- `stub_unreached_gap_code_2`: The body is one call followed by a return, and it was reached only as a gap in the decode.
- `call_helper_return_register`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag_2`: Handles the signs of two values, calls a helper, and negates the result when the flag the helper leaves in an address register is nonzero before returning.
- `negate_on_flag`: Calls a routine and keeps its first result, which is negated when the second result is non-zero.
- `call_rawdofmt_through_base`: Calls the RawDoFmt routine of the exec library through the stored base and returns.
- `store_register_to_pointer`: copies a byte from a data register into a memory location pointed to by an address register.
- `call_library_vector_276`: Calls a library routine at a fixed offset through a stored base pointer taken from a global variable, with no arguments set up here, and returns.
- `allocate_signal_and_message`: Allocates a signal bit and, if that succeeds, a 34-byte message block whose fields are filled from the arguments, the signal number and a value from a routine.
- `release_message_port_resources`: If a link field is set, removes the block from its list, then marks its status fields as deleted.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Initialises a block as an empty linked list head, pointing its first word at itself plus four, clearing the following word and storing the block's own address in the third word.

### Format

- `initialize_filesystem_and_parse_path`: Initializes a data structure with pointers to libraries and memory, then parses a directory path string while handling quoted strings and escaped or special character replacements.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_indirect_function_via_pointer`: Loads a function pointer from a memory location and executes it.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `execute_indirect_function_12`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_8`: loads a function pointer from a memory-offset relative to a global pointer and executes it.
- `execute_indirect_function_13`: loads a function pointer from a global table and jumps to it via an indirect call.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_exec_schedule`: Calls the Schedule routine of the exec library through the stored base, with no arguments set up here, and returns.
- `call_make_functions_2`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_alloc_abs`: Calls the AllocAbs routine of the exec library through the stored base and returns its result to the caller.
- `execute_indirect_handler_3`: Executes a function pointer retrieved from a global structure unable to be identified via registers.
- `format_disk_from_arguments`: Parses the DRIVE, NAME and option arguments, then checks that the drive exists and is empty and asks the user to insert a disk and press return.
- `release_format_buffers`: Frees each stored buffer that is set.
- `report_disk_error_and_retry`: Retries a disk operation while it returns a specific retry code, then maps the final error code to one of several messages such as bad sector, seek error, disk changed or cannot open device.
- `match_value_in_table`: Scans an eight-byte table in the global data for the given value and records the matching entry, or zero, in a field of a block obtained from a routine.
- `find_drive_entry_in_list`: Walks a linked list of drive entries, matching each either by name text or by a comparison routine.
- `calculate_string_length_2`: Returns the number of bytes before the null terminator of a string.
- `copy_cstring_to_buffer`: Copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers and writing each byte up to and including the zero byte.
- `convert_character_to_uppercase`: Returns the uppercase form of a single byte when it is a lowercase letter, otherwise returns the byte unchanged.
- `compare_strings_ignoring_case`: Compares two null-terminated strings a character at a time, converting each character through a routine before comparing.
- `terminate_string_at_colon`: Advances through a string to the first colon or terminator and overwrites the colon with a zero byte, truncating the string there.
- `store_arg_and_check_result`: Stores its first argument in a one-word stack buffer and passes it with a constant and a length of one to a routine.
- `allocate_object_copy_capped_words`: Looks up a handle and allocates a 68-byte object, linking it to that handle and to a stored value.
- `invoke_indirect_function`: Loads a function pointer from a memory offset and executes it.
- `execute_indirect_function_14`: loads a function pointer from a global offset and calls it.
- `execute_indirect_function_15`: dereferences a pointer stored at a global offset and executes the target routine.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `execute_indirect_function_16`: performs an indirect call to a function pointer located at a global offset.
- `execute_indirect_function_17`: executes a routine via a pointer retrieved from a global memory location.
- `execute_indirect_function_18`: calls a routine using a pointer found at a certain offset in a global table.
- `execute_indirect_function_19`: resolves a function pointer from a global offset and jumps to it.
- `execute_indirect_function_20`: executes a routine located at a pointer stored in a global offset.
- `execute_indirect_function_21`: performs an indirect jump to a routine pointer stored in a global offset.
- `execute_indirect_function_22`: loads a routine address from a global offset and transfers control to it.
- `execute_indirect_function_23`: dereferences a pointer from a global offset and executes the resulting address.
- `execute_indirect_handler_4`: Executes a function pointer loaded from a global variable offset.
- `execute_indirect_function_pointer_9`: Loads a function pointer from a calculated offset within a global table and executes it.
- `call_indirect_function_3`: loads a function pointer from a global table with a fixed offset and calls it.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_10`: loads a function pointer from a fixed offset within a global table and executes it.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_11`: fetches a function pointer from an offset within a global table and executes it.
- `execute_indirect_function_27`: Loads a function pointer from an offset within a global table and executes it.
- `execute_indirect_function_26`: fetches a function pointer from a global table and executes it.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_tracks_with_progress`: Allocates an interrupt server and track buffers, then formats the disk track by track, filling each buffer with DOS-style block headers, writing it out and verifying each track while printing a cylinder progress message.
- `run_device_command_sequence`: Calls a device routine through a stored request block with the constants twenty and thirty, then with one, then forwards its argument together with a value from another routine.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer retrieved from a global structure and then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_address`: Stores a single byte from a register into an address stored in another register.
- `divide_32bit_with_remainder_3`: Divides a 32-bit value by a divisor using the same shift-and-subtract and 16-bit division paths as the other division routine, returning the remainder and quotient as a pair of longwords.
- `shift_subtract_divide_loop`: Runs sixteen shift-and-subtract steps that shift a 32-bit dividend left one bit at a time and subtract the divisor from its high half whenever that half reaches it, setting a quotient bit.
- `multiply_32_bit_integers`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products and combines the results.
- `call_adjacent_routine_and_return`: Calls one unnamed routine and returns, passing no arguments.
- `call_helper_return_register_2`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_signed_division_result`: Normalizes the signs of two 32-bit registers, calls a 32-bit division routine, and negates the resulting quotient if the remainder is non-zero.
- `negate_result_on_flag_3`: Calls another routine and returns its first result, negated when the second result register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_address_2`: Stores a single byte from a register into a memory location pointed to by a register.
- `allocate_and_initialize_block`: Allocates a signal bit and, if that succeeds, a 34-byte block whose fields are filled from the arguments, the signal number and a value from a routine.
- `reset_object_and_recurse`: Recursively processes a linked structure when a pointer field is non-zero, sets a byte field and a four-byte field to all ones, then calls another routine on a byte field value and again with a constant second argument.
- `forward_with_constant_forty_eight`: Passes its argument to a routine together with the constant forty-eight, then returns.
- `forward_argument_unchanged`: Passes its single argument straight through to a second routine and returns.
- `init_trashcan_library_state`: Stores a sentinel value in a field of a task-level structure while it looks up a value, then restores it and sets several global fields from fixed constants and the looked-up block.
- `execute_and_get_result`: Calls a routine with certain arguments and returns a value or a modified byte based on the return status.
- `execute_indirect_function_24`: Loads a function pointer from a global table and executes it.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores a pointer into a memory location, increments that pointer, clears a subsequent word, and stores the original pointer address into a third location.
- `store_size_and_pointer`: If the pointer argument is non-null, allocates a block of the size given and, when that succeeds, stores the constant five in a byte, the size word and the pointer argument into fixed offsets.
- `reset_node_and_free`: When the pointer is non-null, marks a byte and two longword fields as released with all-ones values, then passes the pointer and its stored size word to a routine and returns that result.
- `execute_indirect_function_25`: loads a function pointer from a global table and jumps to that address or calls the routine pointed to by that address.
- `invoke_indirect_function_2`: Loads a function pointer from a location relative to a global pointer and executes it.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_string`: Parses a string from a command line, handling quoted arguments and substituting certain characters.
- `call_supervisor`: Calls the Supervisor routine of the exec library through the stored base, with no arguments set up here.
- `call_exit_intr`: Calls the ExitIntr routine of the exec library through the stored base, with no arguments set up here.
- `call_exec_switch_2`: Invokes the exec library switch routine via an offset from a global pointer.
- `call_exec_dispatch_2`: Invokes the exec library dispatch routine via an offset from a global pointer.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Calls the MakeFunctions routine of the exec library through a stored base, with no arguments set up here.
- `call_find_resident`: Calls the FindResident routine of the exec library through a stored base pointer and returns its result.
- `call_enable`: Calls the Enable routine of the exec library through the stored base, re-enabling interrupts, with no arguments set up here.
- `call_exec_alloc_entry`: Calls the exec library's AllocEntry function through a library base held in a global variable, leaving the result for the caller.
- `start_cli_session`: Starts a command-line interpreter session by opening a new console window and running the CLI and Shell startup scripts named in its text.
- `build_node_chain_from_list`: Walks a list of value records, allocating an eight-byte node for each one, storing the record's value in it and linking the nodes in order.
- `free_node_chain`: Walks a linked list of nodes, releasing each node's payload and then the eight-byte node itself, until the list ends at a zero link.
- `parse_signed_decimal`: Reads a signed decimal integer from a null-terminated string, accepting an optional leading plus or minus sign and stopping at the first non-digit.
- `call_library_entry_198`: Loads a library base from a global variable and calls the library function 198 bytes below that base, passing on the caller's arguments.
- `call_library_entry_210`: Loads a library base from a global variable and calls the library function 210 bytes below that base, passing on the caller's arguments.
- `call_library_entry_294`: Loads a library base from a global variable and calls the library function 294 bytes below that base, passing on the caller's arguments.
- `call_library_entry_414`: Loads a library base from a global variable and calls the library function 414 bytes below that base, passing on the caller's arguments.
- `call_library_entry_552`: Loads a library base from a global variable and calls the library function 552 bytes below that base, passing on the caller's arguments.
- `call_library_entry_78`: Loads a library base from a global variable and calls the library function 78 bytes below that base, passing on the caller's arguments.
- `call_library_entry_90`: Loads a library base from a global variable and calls the library function 90 bytes below that base, passing on the caller's arguments.
- `call_library_entry_96`: Loads a library base from a global variable and calls the library function 96 bytes below that base, passing on the caller's arguments.
- `divide_32bit_with_remainder`: Divides one 32-bit unsigned value by another and returns the quotient and remainder as a pair.
- `multiply_32bit_integers`: Multiplies two 32-bit values using four 16-bit partial products and keeps the low 32 bits of the product.
- `call_adjacent_routine_and_return_2`: Calls one unnamed routine and returns, passing no arguments.
- `call_helper_return_register_3`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag_4`: Handles the signs of two values, calls a helper, and negates the result when the flag the helper leaves in an or address register is nonzero before returning.
- `negate_result_on_flag`: Calls a routine and returns its first result, negated when its second result is non-zero.

### Disk-Validator

- `scan_disk_bitmap_flags`: Main routine of the Disk-Validator program.

### Ram-Handler

- `copy_memory_then_call_target`: Program entry that copies a block of memory through the exec library's copy routine, reached via a library base in a global variable, then makes a second indirect call.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_BLIT

- `release_blit_state_slot`: Runs a fixed sequence of library calls, including a memory free, repeated over three passes.
- `decrement_until_minus_one`: Decrements a 16-bit counter register until it reaches minus one, then returns.
- `decode_bitstream_passes`: Runs seven passes of a bit-stream decoder that reads variable-length codes and uses them to copy or add values within a buffer.
- `increment_pointer_address`: Increments a pointer or a memory address by a certain value.
- `decode_bitstream_byte_loop`: Bit-stream decoder working on byte-sized values: it reads variable-length codes from the input and copies or adds values within the output buffer in a loop.
- `push_and_branch`: Modifies a byte in d0, then pushes d0, d1, and d2 onto the stack before restoring all used registers and branching to another routine.

### d

- `stub_bad_instruction_data`: The entry of a keymap data file that the decompiler decodes as instructions.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_CPU

- `release_state_slot_after_calls`: Runs a fixed sequence of library calls, including a memory free, repeated over three passes.
- `decrement_until_minus_one_2`: Decrements a 16-bit counter register until it reaches negative one and then returns.
- `update_array_with_loop_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 6, calling a routine if a certain condition is met.
- `update_array_with_loop`: Checks if a value in d0 is negative, otherwise it calls another routine and, if d0 is not zero, enters a loop that adds a value from a pointer in a3 to a table and decrements a counter until it reaches zero.
- `update_pointer_and_call_2`: If a zero flag is set, it copies a pointer from a3 to a1, calls another routine, and then branches back to a previous instruction.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `expand_bitstream_sequence_2`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting exp.
- `expand_bitstream_loop`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `set_loop_counters_five`: Sets d0 to 5, sets d7 to 3, and branches to a previous instruction.
- `set_loop_counters_four`: Sets d0 to 4, sets d7 to 2, and branches to a previous instruction.
- `set_loop_counters_three`: Sets d0 to 3, sets d7 to 1, and branches to a previous instruction.
- `expand_bitstream_sequence`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or or compressed data to a destination.
- `modify_d1_and_branch`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `expand_and_copy_bitstream_2`: Reads a bitstream-encoded sequence where certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or decompressed byte sequence to a destination buffer.
- `expand_and_copy_bitstream`: Reads a bitstream or compressed-style bit-count-based sequence, expanding or skipping data based on a bit-mask, and copies the resulting bytes from a source pointer to a destination pointer while decrementing a counter in d0.
- `push_data_and_restore_registers`: Stores a byte from d0, a byte from d1, and a word from d2 onto the stack using a pointer in a4, restores several registers from the stack, and then branches to a previous instruction.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_until_minus_one_3`: Decrements a local variable until it reaches negative one and then returns.
- `update_a1_from_a3_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 4, calling a routine if a certain condition is met.
- `iterate_and_accumulate_pointer_values`: Checks if a value in d0 is negative, otherwise it calls another routine, then enters a loop that calls another routine, increments a counter, and adds a value from a table (indexed by d0 and a pointer in a3) to a pointer in a1, decrementing a loop counter in d1 until it reaches zero.
- `update_pointer_and_call_alt`: If a zero flag is set, it copies a pointer from a3 to a1, calls another routine, and then branches back to a previous instruction.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop_2`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `expand_bitstream_data`: Expands a bitstream-encoded sequence by checking or shifting bits and copying or duplicating bytes from a source pointer in a0 to a destination pointer in a3, or copies a fixed number of bytes if a certain counter or bit-count is reached.
- `expand_bitstream_loop_2`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `set_d0_d7_constants`: Sets d0 to negative 5 and d7 to negative 3, then branches to another routine.
- `set_d0_d7_constants_2`: Sets d0 to negative 4 and d7 to negative 2, then branches to another routine.
- `set_d0_d7_constants_3`: Sets d0 to negative 3 and d7 to negative 1, then branches to another routine.
- `expand_bitstream_bit_count`: Checks the least significant bit of a value in d1 and, if zero, loads a new byte from the source pointer in a0 and updates d1 with the new bit, or otherwise branches to a lookup table or a different branch or a loop that continues bitstream expansion or bit-count calculation.
- `modify_d1_and_branch_2`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `expand_bitstream_to_buffer`: Reads a bitstream from a source pointer in a0, expanding or copying bits or bytes into a destination buffer in a3, while using a table or a global variable to determine expansion or copy lengths or offsets, and updates a counter or pointer in a4.
- `expand_and_copy_bitstream_3`: Reads a bitstream or compressed-style sequence where certain bytes determine a bit-count or expansion factor, then copies or expands these values into a destination buffer pointed to by a register, while decrementing a counter and restoring registers from the stack via a helper routine.
- `push_data_and_restore_registers_alt`: Stores a byte from d0, a byte from d1, and a word from d2 onto the stack using a pointer in a4, restores several registers from the stack, and then branches to a previous instruction.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon_install

- `initialize_game_state`: The routine calls another routine several times and then runs a small loop that invokes a function pointer taken from a global variable.
- `decrement_until_minus_one_4`: Decrements a local word-sized variable until it reaches negative one and then returns.
- `modify_table_elements_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 3, calling a routine if a certain condition is met.
- `update_table_with_offset`: Checks a condition and branches, otherwise it calls a routine, copies d0 to d1, and if d0 is not zero, calls another routine, sets d6, calls a routine via a pointer in a4, copies a pointer from a1 to d3, calls another routine, and enters a loop that adds d0 to d2 and adds a value from a pointer in a3 plus d2 to a destination, decrementing d1 until it reaches zero.
- `branch_and_call_pointer`: If a condition is not met, it copies a pointer from a3 to a1, calls a routine, and then branches back to the start of the routine.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_bytes_from_stack`: Checks if a pointer in a2 equals a6, otherwise it loops to copy or move bytes from a source pointer in a2 to a destination pointer in a1, decrementing a counter in d0, and finally moves a value from a pointer in a1 to d0 before returning.
- `expand_bitstream_variable_length_2`: Decodes a bitstream or bit-shifted sequence from a source pointer in a0, using a bit-shifting and expansion logic, and writes the result to a destination pointer in a3.
- `process_bitstream_loop`: Sets a loop counter and checks a condition; if not zero, it loads a byte from a pointer in a0, adds it to d1, and adds d0 to d1, repeating or continuing until the counter reaches zero, then updates d0 and jumps to another routine.
- `initialize_bitstream_parameters`: Sets d0 to 5, sets d7 to 3, and jumps to another routine.
- `initialize_bitstream_parameters_alt`: Sets d0 to 4, sets d7 to 2, and jumps to another routine.
- `initialize_bitstream_parameters_min`: Sets d0 to 3, sets d7 to 1, and jumps to another routine.
- `expand_bitstream_variable_length`: Decodes a variable-length bitstream or bit-shifted sequence from a source pointer in a0, using a bit-shifting and expansion logic, and writes the result to a buffer.
- `update_registers_and_branch`: Shifts d1 left by one and branches if the result is non-zero; otherwise, it decrements the pointer in a0 and loads a byte from that location into d1, then shifts d1 and d2 left by one and branches to another routine.
- `expand_bitstream_to_buffer_2`: Decodes a variable-length bitstream or bit-shifted sequence from a source pointer in a0, using a bit-shifting and expansion logic, and writes the resulting or expanded data into a destination buffer pointed to by a5.
- `unpacks_variable_length_encoded_data`: Decodes a bitstream or variable-length encoded sequence where certain byte values trigger a bit-shifting or expansion logic, then copies or moves bytes from a source buffer to a destination buffer.
- `pushes_registers_to_stack`: Pushes a byte from d0, a byte from d1, and a word from d2 onto the stack using a4 as a pointer, restores a large set of registers from the stack, and branches to another routine.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `compare_and_call_routine`: Compares a longword at the address in a0 with a constant value; if they match, it branches to another routine, otherwise it loads a word and a byte from a0 into registers, updates two global pointers with the current stack pointer, sets a0 to point to a global variable, calls another routine, and then branches or jumps based on the resulting sign bit.
