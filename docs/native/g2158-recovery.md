# Game 2158 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02158). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905232 top-level bytes in 58 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 41 of 41 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 6071 of 6071 named (0 author symbols), 6071 of 6071 described (194 by rule, 2719 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1001 of 1001 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 117429 of 117433 instructions identical in start and length, 4 mismatches |

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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_BLIT#unpacked | 7 | 52210 | 2715 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_CPU#unpacked | 6 | 47660 | 2691 |

## Cases handled

- runtime segment an address (play-rs-m1500): 543 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_BLIT: crunched executable decrunched by XFDMaster (Imploder (xfd), 161876 -> 359116 bytes), 85 of 167 windows confirmed in reference RAM: recovered as a hunk executable
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_CPU: crunched executable decrunched by XFDMaster (Imploder (xfd), 160772 -> 347516 bytes), 84 of 165 windows confirmed in reference RAM: recovered as a hunk executable
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon: Imploder (xfd) stream found by ancient scan (5784 -> 10488 bytes) but only 2 of 6 sampled windows are in reference RAM: not used
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon_install: Imploder (xfd) stream found by ancient scan (10908 -> 22940 bytes) but only 0 of 11 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 operand
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a3]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_and_copy_data` (AM2_BLIT#unpacked): Sets DMA, interrupt, and custom register values, calls another routine, copies 12 bytes from a calculated offset in a global table to another table, updates three global words, and calls another routine. Kind: not judged: HTTP Error 503: Service Unavailable. Pass path: not determined.
- `configure_hardware_and_copy_data_2` (AM2_CPU#unpacked): Sets DMA, interrupt, and custom register values, calls update_pra_registers_2, copies 12 bytes from a calculated offset in a global table to another table, updates three global words, and calls another routine. Kind: not judged: HTTP Error 503: Service Unavailable. Pass path: not determined.

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

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a3].adf:AM2_BLIT#unpacked

- `jump_to_callback_handler`: Jumps to another routine that validates a magic constant at the pointer in a0, extracts a word and a byte from the structure, and executes a callback.
- `validate_and_execute_callback`: Compares a longword at the pointer in a0 to a constant, and if it matches, copies a word and a byte from the pointer to global variables, updates two global pointers, and calls a routine via a pointer.
- `initialize_data_tables`: Sets the stack pointer to a specific address, calls eleven different routines, loads a pointer from a global table into a0, and calls several more routines before branching.
- `clear_d0_and_execute_handlers`: Clears register d0 and calls two consecutive subroutines.
- `execute_callback_via_stack_setup`: Calls a routine to free a list of memory blocks, then restores the stack pointer from a global variable, loads a value from another global variable into a register, and performs an indirect jump to a third global variable.
- `initialize_and_update_data_tables`: Clears a block of memory, performs several conditional checks and calls to other routines, populates a table with incrementing values in a loop, copies a block of data from a source to a destination, and updates a series of values in a table by reading bytes from a structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_word_blocks`: Copies three 16-bit words from a source pointed to by a0 to a destination pointed to by a1 in a loop, increments a counter in a global variable, and clears the destination word if the counter reaches 512.
- `iterate_and_process_table`: Iterates through a table pointed to by a global variable, comparing each word to a value in d7, and if they match, copies a byte from offset 2, a byte from offset 3, and a word from offset 4 of the current entry into registers before calling another routine.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `compare_and_update_structure`: Loads a pointer from a global variable, compares several bytes and a word at offsets 0, 1, 3, and 4 of the pointed-to structure against d0, d1, and d2, and if all match and the word at offset 4 equals d2, it writes d2 to offset 4; otherwise, if the structure is empty, it populates it with values from registers and calls another routine.
- `advance_pointer_and_jump`: Increments the address in a0 by 6 and performs a short branch to another routine.
- `update_structure_data`: Increments a global 16-bit value and, if it is less than 513, writes a 16-bit word, two bytes, and a 16-bit word from registers to a structure pointed to by a0.
- `check_conditional_byte`: Saves registers to the stack, compares a byte at a specific address to one, and if it matches, loads a pointer from a table and compares a byte at offset 2 of that pointer with d7.
- `increment_pointer_and_decrement_loop`: Increments a 16-bit pointer in a0 by 4, decrements a loop counter in d6, and branches to another routine if the counter is not zero; otherwise, it branches to a different routine.
- `process_conditional_data_3`: Compares d7 with a value in a table, then enters a loop that conditionally calls two different routines to either write a word to an offset of 332 plus a calculated index in a structure or write a byte to an offset of 348 plus a calculated index.
- `execute_dual_callbacks`: Saves d0 to the stack, calls a routine using a byte from d6, uses the returned pointer to write a byte from d2 to an offset of 332 plus the pointer, then repeats the process by calling another routine and restoring registers.
- `process_conditional_data_6`: Compares d0 to 11, stores d0 in a global word if it is less than 11, calls another routine if a global byte is greater than 2, and if a global byte is non-zero, retrieves a byte from a table using d0 as an index, stores it in a global word, and calls another routine.
- `call_conditional_routine_6`: Checks if the high byte of d0 is non-zero and not equal to a value in a global word, then calls another routine if these conditions are met.
- `process_conditional_updates_2`: Uses a byte from a table to call a routine, performs several conditional checks against global values and structure members, and updates a global pointer to a calculated offset in a table.
- `load_parameter_and_jump`: Loads the value 256 into register d0 and performs an indirect jump to another routine.
- `set_d0_constant_and_jump`: Loads the value 512 into register d0 and performs a long jump to another routine.
- `load_parameter_and_jump_2`: Loads the value 768 into register d0 and performs an indirect jump to another routine.
- `set_d0_and_jump`: Loads the value 1024 into register d0 and performs an indirect jump to another routine.
- `load_parameter_and_jump_3`: Loads the value 1280 into register d0 and performs a long jump to another routine.
- `load_parameter_and_jump_4`: Loads the value 1536 into register d0 and performs a long jump to another routine.
- `handle_event_data_2`: Checks if the high byte of d0 is non-zero, retrieves a byte from a table using d0 as an index, calls a routine, and if a specific byte in the resulting structure is non-zero, stores the retrieved byte into a global address and calls another routine.
- `store_and_call_handler`: Stores a byte from d2 into a global address and a word from d1 into another global address, calls a routine, and then calls another routine if a bitmask check on d3 succeeds.
- `call_event_handler`: Loads a pointer from a memory location into a0 and calls another routine, then restores the registers and returns.
- `set_event_index_and_jump`: Sets the high byte of register d0 to 256 and jumps to another routine.
- `set_event_index_and_jump_2`: Loads the value 512 into register d0 and performs a long jump to another routine.
- `set_event_index_and_jump_3`: Loads the value 768 into register d0 and performs a long jump to another routine.
- `set_event_index_and_jump_4`: Loads the value 1024 into register d0 and jumps to another routine.
- `set_event_index_and_jump_5`: Loads the value 1280 into register d0 and performs a long jump to another routine.
- `load_event_id_and_jump`: Loads the constant value 1536 into register d0 and jumps to another routine.
- `set_event_index_and_jump_6`: Sets the d0 register to 1 and jumps to another routine.
- `return_constant_two`: Loads the value two into a register and calls another routine.
- `return_constant_three`: Loads the value 3 into register d0 and jumps to another routine.
- `return_constant_four`: Loads the value 4 into a register and jumps to another routine.
- `set_event_index_and_jump_7`: Loads the value 5 into register d0 and jumps to another routine.
- `set_event_parameter_and_jump`: Loads the value 6 into register d0 and jumps to another routine.
- `load_event_parameter_and_jump`: Loads the value 7 into register d0 and jumps to another routine.
- `set_d0_and_jump_2`: Sets the d0 register to 8 and jumps to another routine.
- `load_event_index_and_jump`: Loads the value 9 into register d0 and jumps to another routine.
- `iterate_and_call_handler`: Iterates through a series of bytes starting at an address in a0, calling a routine if a byte is non-zero, and continues for up to seven iterations.
- `process_conditional_structure_update`: Calls a routine to retrieve a pointer, checks bit 5 of a 16-bit word at offset 46 of the pointed-to structure, and if that bit is not set, checks a global byte and calls another routine; it then calls a second routine using a byte from d6 and restores registers.
- `process_event_table_entries`: Iterates through a table of up to six entries, calling a routine to resolve a pointer from a byte and another routine to retrieve a word from offset 46 of the resulting structure.
- `set_event_parameters_4`: Stores a word into a global variable, stores a long into another global variable, and calls two different routines using a pointer in a0.
- `dispatch_event_handler_2`: Calls a routine to process an event list, loads a pointer into a register, and jumps to another routine.
- `execute_complex_sequence_4`: Performs a series of routine calls with various constant parameters, copies 32 words from one memory location to another in a loop, updates several global values, and calls multiple routines including one that takes specific values in d0, d1, d2, and d3.
- `execute_blit_and_pop_pointer`: Calls execute_complex_blit and then calls pop_pointer_from_stack, which decrements a global pointer by 4 and updates two global words with the values found at the new pointer location.
- `execute_conditional_calls_11`: Loads two words and a byte into registers, calls a routine, and if the zero flag is not set, calls another routine before returning.
- `execute_conditional_handler_10`: Loads values into d0, d1, and d3, calls a routine, and if the result is not zero, calls another routine before returning.
- `process_callback_table`: Retrieves a byte from a table using an index in d0, calls a routine with that byte, retrieves a pointer from the resulting structure, calls another routine, and if a specific bitwise condition is met on a value from that structure, stores a word in a global variable and calls a third routine.
- `process_callback_and_update_index`: Retrieves a byte from a table using an index in d0, and if non-zero, calls a routine with that byte as an argument.
- `dispatch_indexed_callback_update`: Reads a flag byte from a global table at the index in d0, and if it is non-zero, calls a routine to obtain a pointer, reads a word at offset 30 of that pointer, and calls two further routines.
- `process_event_data_3`: Retrieves a byte from a table using d0 as an index, calls another routine, and if the byte is non-zero, calls a second routine and then a third routine that stores d7 into a location at a global address.
- `update_global_word_if_mask_cleared`: Performs a bitwise AND on a register with a constant, and if the result is zero, stores a word from a register into a global variable and calls another routine; otherwise it returns.
- `execute_conditional_callback_5`: Checks a byte at an offset from a pointer in a0 using d0 as an index, and if that byte is non-zero, stores d0 in a global variable and calls another routine.
- `call_pointer_handler`: Pushes d1 to the stack, loads a byte from offset 30 of a global variable into d1, calls a routine using d1 as a pointer, and restores d1 from the stack before returning.
- `execute_conditional_callback_4`: Saves registers, retrieves a byte from a global variable, calls a routine to get a pointer, retrieves a word from offset 30 of that pointer, calls another routine, and if a bitwise check on the result passes, calls a third routine using a byte from a global variable.
- `call_pointer_handler_2`: Pushes d1 to the stack, loads a byte from an offset of 72 into d1, calls a routine using d1 as a pointer, and then restores d1 from the stack before returning.
- `call_pointer_handler_3`: Pushes the contents of d1 to the stack, loads a byte from an offset of 70 into d1, calls a routine using d1 as a pointer, restores d1 from the stack, and returns.
- `call_pointer_handler_4`: Pushes d1 to the stack, loads a byte from offset 71 of a global variable into d1, calls a routine using d1 as a pointer, and then restores d1 from the stack before returning.
- `copy_string_and_update`: Calls a routine to get a pointer, uses that pointer to call a routine that copies a null-terminated string, decrements a value in a0, calls another routine using a byte from d1, and restores registers.
- `execute_dual_handlers`: Calls two different routines using pointers retrieved from a table and a structure, then loads a new pointer into a1 if a byte comparison is non-zero.
- `copy_string_from_address`: Loads a pointer from a global variable into a1, calls copy_null_terminated_string_3, decrements a0, and restores registers from the stack before returning.
- `execute_dual_handlers_alt`: Calls two different routines using pointers retrieved from a table and a structure, then loads a pointer from a different global variable into a1 if a byte comparison is non-zero.
- `copy_string_from_address_2`: Loads a pointer from a global variable into a1, calls copy_null_terminated_string_3, decrements the destination pointer in a0 by one, and returns.
- `call_with_constant_100`: Saves register d1 to the stack, loads the constant 100 into d1, calls another routine, and then restores d1 before returning.
- `check_constant_and_clear_d7`: Compares a 16-bit word at a specific memory location to the value 6; if the value is not 6, it branches to another routine, otherwise it sets register d7 to 0 and branches to another routine.
- `test_signed_word_and_set_flag`: Tests a 16-bit signed word in register d0; if the value is greater than zero, it branches to another routine, otherwise it sets register d7 to negative one and branches to another routine.
- `compare_signed_word_and_set_flag`: Compares two 16-bit signed words and, if the first is less than the second, branches to another routine; otherwise, it clears register d7 and branches to another routine.
- `compare_multiplied_result`: Copies d0 to d2, calls another routine, multiplies d1 by d0, swaps the high and low words of d0, compares the result to d2, sets d7 based on the comparison result, and restores registers before returning.
- `update_data_table`: Performs a loop 20 times, writing a byte to a table, calling another routine, and then writing three more bytes to the table based on values retrieved from a table and bitwise operations.
- `process_conditional_structure_updates`: Compares a word in a global variable to six, retrieves a byte from a table using a value in a7 as an index, and if the byte is non-zero, calls one routine, then checks if a word at offset 202 of a structure in a0 minus a value from the stack is greater than zero to decide whether to call another routine and then call a third routine.
- `update_structure_values_5`: Writes a word to an offset of 170 in a structure, calls a routine, and then writes a byte and a constant to a structure at an offset determined by d7.
- `process_table_entry_and_set_bit`: Retrieves a byte from a table using d7 as an index, calls two routines, sets bit 13 of a word at offset 30 of a structure, clears a word at offset 202 of that structure, and calls three more routines if certain conditions are met.
- `manage_blit_sequence`: Calculates a value based on d7, stores it in a structure, and then executes a complex sequence of calls including a loop that copies bytes and several routines to manage memory and blitting.
- `process_multiple_table_elements`: Calls a routine, retrieves a byte from a table using d7 as an index, calls several routines, clears a byte and two words at specific offsets in tables using d7 and d0, decrements a global word, and calls two more routines.
- `process_blit_sequence_2`: Calculates a value based on d7, stores it in a local stack area and a global table, calls three routines, copies four bytes from a table to a destination in a loop, and performs several subsequent routine calls including a loop that runs up to three times.
- `update_list_pointers`: Checks bit 6 of a 16-bit word at offset 30 of a structure; if set, it clears that bit and performs two loops to copy eight 16-bit words from two different locations within the structure to new offsets.
- `clamp_structure_value_202`: Retrieves a structure via a table, calls two routines, and updates a 16-bit value at offset 202 of the structure if it is less than the sum of values at offsets 204 and 206.
- `clamp_structure_value_208`: Retrieves a structure via a table, calls two routines, and updates a 16-bit value at offset 208 of the structure if it is less than the sum of values at offsets 210 and 212.
- `add_table_offset_low`: Calculates an offset into a table using a 2-bit mask of d2, retrieves a signed byte from that table, and adds it to d0.
- `add_table_offset_high`: Calculates an offset into a table using a 3-bit mask of d2, retrieves a signed byte from that table, and adds it to d0.
- `clear_byte_array`: Clears 128 bytes starting at a global variable address using a loop.
- `modify_bit_in_table`: Checks if d1 is negative, then uses d0 and d1 to calculate an offset into a table, compares a value at that offset, and if successful, modifies a bit in a byte at a calculated address based on the value in d2.
- `set_bit_if_byte_is_one`: Compares a byte in register d2 to 1; if it is equal, it sets bit 1 of a byte at the address in a0, otherwise it returns.
- `toggle_bit_if_byte_is_two`: Compares a byte in register d2 to 2; if it is not equal, it returns, otherwise it toggles bit 1 of a byte at the address in a0 and branches to another routine.
- `check_bit_conditional`: Compares a byte in register d2 to 3; if not equal, it otherwise checks if bit 0 of a longword at the address in a0 is set and returns.
- `process_table_loop`: Iterates through a table up to six times, calling two routines per entry and updating a value at offset 24 and a value at offset 270 of a structure based on bitwise checks and arithmetic.
- `process_entity_timer_updates`: Iterates through a table of structures to calculate a new timer value based on offsets 42, 46, and 270, then stores the result in a second table.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_and_update_structure_list`: Iterates through a list of structures, performing calculations based on values at offsets 2, 26, 30, 42, 46, and 270, and stores the results in a new table.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_pointer_table`: Retrieves a pointer from a location specified by a byte in d0, then iterates through a table of 23 elements starting at offset 344 of the retrieved structure, performing bitwise OR operations on values at 6-byte intervals.
- `process_table_elements`: Retrieves a pointer from a location specified by d0, then iterates through a table of 23 elements starting at offset 4 of the retrieved structure, performing bitwise OR operations on values at 6-byte intervals, and calls another routine before returning.
- `iterate_structure_list`: Retrieves a pointer from a location specified by d0, then iterates through a table of 23 elements starting at offset 4 of the retrieved structure, performing bitwise OR operations on values at 6-byte intervals if the value at offset 4 is non-zero, and calls another routine before returning.
- `scan_data_table`: Iterates through a table of 6 elements starting at a global address, calls another routine for each non-zero byte, and if certain bitwise conditions are met in the returned structure, increments a counter and calls another routine.
- `iterate_event_data`: Saves registers, initializes several pointers, and enters a loop that calls a routine, performs bitwise checks, and calculates values using multiplication and table lookups to potentially increment a global byte and continue the loop.
- `process_event_with_stack`: Pushes d0 to the stack, reads a byte from a0, calls a routine, restores d0 from the stack, increments a0 and a4, and then either calls another routine or returns.
- `update_structure_coordinates_and_counters`: Calculates a memory offset from a structure, calls another routine to retrieve a pointer, and enters a loop that increments a byte at offset 5 and updates a value at offset 17 based on a division of that byte by a 16-bit value at offset 226.
- `update_tile_coordinates_and_draw_ui`: Iterates through a loop to update coordinate fields in a structure by calculating offsets and scaling values, then performs a series of calls to draw and format various UI elements or graphical components using specific constants and table lookups.
- `calculate_structure_offset_and_update`: Retrieves dimensions from a structure, calculates a memory offset using multiplication, and updates several coordinate fields in a structure by adding values derived from a series of calls.
- `clear_bit_and_call`: Pushes d0 to the stack, loads a word from a global address into d0, clears bit 0 of d0, stores the result back to the global address, stores a byte from a global address, sets d0 to 255, calls a routine, restores d0 from the stack, and returns.
- `set_global_flag`: Sets bit 0 of a word in a global variable, clears a longword at a global address, and calls another routine.
- `save_and_call_routine`: Saves registers to the stack, loads a byte from d0 into d4, calls a routine, and then uses the returned pointer in a1 to perform arithmetic and a branch.
- `count_valid_structure_entries`: Iterates through a list of structures, checking if a bit at offset 32 plus an offset calculated from a word at offset 4 is set, increments a counter in d6 for each valid entry, and calls another routine using a value from d4.
- `process_structure_list_9`: Calculates an offset from a value in d6, calls another routine, and uses the result to store a pointer and a byte into a structure at a2, then enters a loop that performs similar operations using values from the stack and a table.
- `store_calculated_addresses`: Calculates an offset based on a 16-bit value at offset 4 of a structure in a1, checks a bit in a structure in a0, and stores either a calculated address or a constant value into a destination pointed to by a2.
- `store_address_and_branch`: Stores a longword constant into the memory location pointed to by a2 and increments a2, then performs a short branch into another routine.
- `process_structure_list_5`: Increments a pointer in a1 by 6 and enters a loop that checks a bit in a structure at a0 and compares a value at a1 plus 10 to zero, eventually storing a pointer and a global address into a structure pointed to by a2.
- `increment_global_byte_5`: Checks a global byte; if it is zero, it calls a routine to retrieve a value, stores that value in the global byte, and then increments the global byte.
- `decrement_global_byte_7`: Decrements a byte in a global variable if it is non-zero and not equal to one, otherwise it returns immediately.
- `check_status_and_branch`: Compares a longword at a global address to 3; if equal, it branches to another location, otherwise it compares a byte from a table to d0 and branches based on the result.
- `call_indexed_function`: If the byte at the address in a0 is zero, it calls a function from a table using an index in a global variable, increments that index, and stores a pointer in a global variable.
- `jump_to_table_initializer`: Clears the d0 register and jumps to initialize_table_structure.
- `set_parameter_and_jump`: Loads the value 293 into a data register and jumps to another routine.
- `update_structure_flags_3`: Calls a routine with d0 set to 294, then loops six times through a global structure to retrieve pointers, set bit 5 of a word at offset 30 of those pointers, and call another routine.
- `execute_table_callback_4`: Performs several routine calls to process data, then uses a value from the stack to calculate an offset into a table.
- `call_blit_routine_one`: Loads several 16-bit constants into d0, d1, d2, and d3, sets d4 to 28, and calls another routine.
- `call_blit_routine_two`: Loads several 16-bit constants into d0, d1, d2, and d3, sets d4 to 28, and calls another routine.
- `call_blit_routine_twice`: Sets d4 to 28 and calls another routine twice using two different sets of 16-bit values in d0, d1, d2, and d3.
- `dispatch_clipping_calculation`: Jumps to the routine that calculates a clipping area.
- `call_blit_clipping_rectangle_2`: Loads registers d0 through d4 with the values 16, 138, 303, 192 respectively and calls blit_clipping_rectangle.
- `blit_two_rectangles`: Calls blit_clipping_rectangle twice with specific coordinate sets, first using 15, 43, 191, and 122, and then using 15, 43, 303, and 122.
- `call_blit_clipping_rectangle_single_2`: Loads registers d0 through d4 with the values 16, 50, 191, and 193 respectively and calls blit_clipping_rectangle.
- `blit_two_clipping_rectangles`: Calls blit_clipping_rectangle twice with specific rectangular coordinates defined by registers d0 through d3, using d4 to pass a constant value of 28.
- `fill_array_with_white`: Fills a series of 32-bit words with the value negative one starting from a global variable, using an index in d7 to determine the offset.
- `process_graphics_list`: Saves registers, calls a routine with a pointer to a structure, calls another routine in a loop six times with varying offsets, calls a third routine with a modified pointer, and calls a fourth routine with a value in d7.
- `sync_sprite_data`: Compares two words from global variables, updates a table with white values if they differ, and enters a loop that compares and updates bytes between two structures while calling two routines if certain bit conditions are met.
- `process_graphics_structure`: Saves registers to the stack, performs arithmetic to calculate a value for d7, calls several routines to handle data at various offsets, copies a string of bytes from a source to a destination, and performs a loop to copy 5 bytes from a structure to a destination.
- `update_sprite_scaling_and_status`: Calculates scaling factors for a structure based on width and height dimensions, updates scaling bytes in a secondary structure, and performs several blitter operations to redraw or adjust the sprite's appearance.
- `update_element_arrays`: Calls a routine, then enters a loop that checks a byte at an offset of a structure, calls a routine, decrements that byte, and continues until the byte is zero; it then performs a second loop that calls two routines for up to seven iterations.
- `save_registers_and_branch`: Saves longwords from registers d7 and a0 onto the stack and branches to another routine.
- `execute_blitter_setup_2`: Calls two routines, then enters a loop that checks bytes in a table at a specific address and calls a routine if the byte is non-zero, repeating this up to six times before calling a final routine.
- `execute_scaled_blit_sequence_2`: Calculates a value using multiplication and addition, calls a routine, and if d2 is non-zero, updates a global variable and calls two more routines.
- `update_byte_array_elements`: Calculates a value based on a byte from a table, performs a scaled blit, and enters a loop to update elements in a structure while decrementing a counter.
- `process_byte_array`: Decrements a byte at an offset of a1, and if it reaches zero, it decrements a word in d4 and calls another routine; it then enters a loop that calls another routine and writes bytes to a table based on d4 and d7.
- `process_bit_flag`: Checks a global word against a threshold, clears it if negative, and if a specific byte is non-zero, searches a table to find a matching bit index to store in a destination buffer.
- `search_and_set_bit`: Retrieves a pointer from a table using a byte from a0, then enters a loop to search a table for a byte that satisfies a bit test against a value from a structure, finally storing the result in a1 and a pointer in a3.
- `increment_and_store_values`: Stores d2 at the address in a3, increments d2, stores the result in a1, and then enters a loop that increments several registers and pointers until d7 reaches 7.
- `increment_global_word`: Increments a 16-bit word located at a specific global memory address and returns.
- `execute_blitter_loop`: Loads a pointer from a global variable into a1 and another into a3, then enters a loop that runs five times, calling another routine if the value at the pointer in a1 is zero.
- `clamp_d7_value`: Compares the value in d7 to 180; if the value is greater than 180, it sets d7 to 12 and otherwise continues to another routine.
- `execute_blitter_sequence_4`: Performs arithmetic on d7, calls a routine with a pointer in a3, and executes a loop that performs two blitter-related routine calls while incrementing d0 and d1 and decrementing d3.
- `set_status_word`: Sets a global word to 31 and compares a global word at offset 3734 to 6; if it is 6, it branches to another routine, otherwise it loads a value from offset 3736 into d3.
- `toggle_status_bits`: Performs bitwise operations on two global words, and if specific bits are set, calls another routine with specific parameters.
- `setup_blit_parameters_8`: Loads several constant values into registers, loads a pointer from a table into a0, calls another routine, and then branches based on the values of bits 2 and 0 of d2 and d3.
- `execute_conditional_blit_3`: If bit 2 of d3 is set, it branches; otherwise it saves d2 and d3, sets several registers, calls another routine, and branches to another location.
- `calculate_offset_and_call`: Loads a word from a table using an index in d0, multiplies it by 384, adds the result to a0, and calls another routine before branching to another routine.
- `dispatch_conditional_blit_parameters`: Tests bit 0 of register d2 and bit 0 of register d3, branching to different locations based on these bits, otherwise it returns via a branch to another routine.
- `setup_blit_parameters_7`: Tests bit 0 of d3, and if it is not set, saves registers and calls another routine with specific constant values in d0, d1, d2, and d3.
- `setup_blit_parameters_6`: Loads several constant values into registers and calls another routine.
- `calculate_blit_parameters_4`: Calls a routine with specific values in d0-d3, then checks a byte in a global variable and performs arithmetic on a word from another global variable before branching.
- `calculate_blit_parameters_3`: Calculates a value based on a word from a global variable, performs arithmetic involving multiplication and division, and calls three different routines to update various memory locations and structures.
- `initialize_blit_sequence`: Tests bit 1 of d3 and branches if set, otherwise saves d2 and d3 to the stack, loads several constant values into d0, d1, d2, d3, and d4, calls another routine, and branches.
- `check_blit_status_and_load_pointer`: Tests a byte in a global variable; if the byte is non-zero, it branches to another location, otherwise it loads a longword from a memory address into a0 and branches to another routine.
- `initialize_blit_parameters`: Loads a longword from a global variable into a0, sets d0, d1, d4, d5, d6, and d7 to specific constant values, and calls another routine.
- `call_handler_with_constant`: Saves registers, sets d1 to a constant, calls another routine, and stores the resulting byte from d0 into a global memory location.
- `execute_handler_calls`: Saves registers, calls a routine using a byte from a global variable, calls another routine with specific constant values in d1, d5, d6, and d7, and then calls a third routine using a byte from a global variable.
- `process_structure_data_6`: Saves registers, calls a routine to retrieve a pointer, fetches a byte from a table using a register as an index, and performs several calls to update a structure and its fields.
- `update_structure_pointer_2`: Saves d0, d7, and a0 through a1 to the stack, loads two pointers into a1 and a0, calls a routine, stores the result in a structure, loads a byte from a table using d7, calls another routine, updates a pointer in a structure, clears a long value, and calls a third routine before finally calling a fourth routine and returning.
- `update_structure_pointer`: Fetches a pointer from a table, adds an offset of 274 to it, and stores the resulting address into the location pointed to by a0, then calls another routine.
- `update_table_and_call_2`: Saves d0 and a0 through a2 to the stack, loads a pointer from a table into a1, adds d7 to a1, reads a byte from a1 into d0, calls a routine, updates a pointer in a structure, calls another routine, calls a third routine, and returns.
- `set_flags_and_dispatch_pointer`: Sets two 16-bit values in global memory to all ones, loads a 32-bit pointer from a third global location into a register, calls a subroutine via that pointer, and then jumps to another subroutine via a pointer.
- `execute_blit_pipeline`: Calls swap_and_invert_pointers, calculate_clipping_area, and update_conditional_fields in sequence.
- `process_indexed_callback`: Checks if d0 is zero or negative one, calls a routine, calculates an index using a word from a global variable and a multiplication, calls two routines using values retrieved from a table, and if a value is non-zero, stores a word in a global variable and calls a third routine.
- `fetch_and_store_table_value`: If d0 is non-zero, it calls another routine, calculates an index by multiplying the masked value of d0 by 6, retrieves a word from a table at offset 290 of a pointer returned by another routine, and if that word is non-zero, stores the original d1 value into a global variable and calls another routine.
- `fetch_and_store_offset_value`: If d0 is non-zero and not 255, it calls another routine, calculates an index by adding 8 and a value from a global variable to the masked d0, retrieves a word from a table at offset 290 of a pointer returned by another routine, and if that word is non-zero, stores the original d1 value into a global variable and calls another routine.
- `process_clamped_index`: Tests d0 for zero or 255, masks and decrements d0, adds a word from a global pointer to d0, and calls another routine if the result equals a constant.
- `process_scaled_data_2`: Multiplies d0 by 6, calls another routine using a byte from a global variable, adds a global address to a pointer, retrieves a word from a table, calls another routine, calls a third routine, and if the result is non-zero, stores d1 into a global variable and calls a fourth routine.
- `dispatch_conditional_callback`: Calls a routine via a pointer and then returns immediately.
- `test_and_branch_data`: Tests a byte in d0 and branches if it is zero, otherwise it masks the lower byte, decrements d0, and compares it to a value in a global variable.
- `process_offset_scaled_data`: Multiplies d0 by 6, calls another routine using a byte from a global variable, adds 290 to a pointer, retrieves a word from a table, calls another routine, calls a third routine, and if the result is non-zero, stores d1 into a global variable and calls a fourth routine.
- `dispatch_conditional_callback_2`: Calls another routine and then returns.
- `update_global_constant`: Decrements d0, multiplies it by 60, adds the result to a base address in a0, writes the constant 2 to a global variable, and calls another routine.
- `execute_callback_loop_2`: Loads two words from a table, performs a loop of eleven calls to another routine using those words and two other constants, then performs one final call with different constants and returns.
- `loop_graphics_updates`: Executes a routine in a loop for eleven iterations, incrementing d0 on each pass.
- `update_graphics_structure`: Saves registers, performs multiple calculations involving multiplication and table lookups, calls several routines to update data, and modifies a global word at a specific offset.
- `modify_graphics_data`: Saves registers, calculates an offset into a table using d0, calls a routine, performs arithmetic on a value from a table, and calls several routines to update data based on bit tests.
- `update_index_and_byte`: Decrements a 16-bit value in a global variable; if it reaches zero, it increments another global variable, resets the first to four, and then reads a byte from a table at an offset determined by the second global variable.
- `decrement_counter_advance_index`: Decrements a 16-bit global counter, and when it reaches zero, increments a second global index, reloads the counter with 4, and reads a byte from a global table at that index.
- `execute_blit_logic`: Checks a global byte, and if it is zero, calculates an offset into a table, calls another routine, and then performs a multiplication and addition to update a pointer in a1.
- `push_and_call_pointer`: If d0 is not equal to 1, it pushes d0 to the stack, retrieves a pointer from a location at offset 40 of a6, calls another routine, moves the result to a1, restores d0 from the stack, and branches to another routine.
- `execute_blit_command_2`: Compares d0 to two, branches if not equal, pushes d0 to the stack, loads a byte from a pointer in d0, calls a routine, sets a1 to the result, loads d0 from the stack, adds a constant to a1, sets a2 to a constant, and branches.
- `execute_blit_subroutine_and_decrement`: Decrements the byte at offset 19 of a structure pointed to by a register if a global variable is less than 200 and the byte is non-zero, after calling another routine and updating a register with a value from a global variable.
- `dispatch_blit_callback_chain`: Calls a routine via a pointer and then jumps to another routine.
- `update_tile_data_offset_20`: Calls a routine, stores a byte from d0 at an offset of 32 from a global pointer, stores a word from d7 at an offset of 332 from a global pointer, and updates a structure at offset 16 with d0 and offset 4 with d6 before calling two more routines.
- `update_tile_data_offset_21`: Calls a routine, stores a byte from d0 at an offset of 33 from a global pointer, stores a word from d7 at an offset of 334 from a global pointer, and updates a structure at offset 16 with d0 and offset 4 with d6 before calling two more routines.
- `update_conditional_fields`: Checks a word in a global variable, calls two routines with different pointers if the word is not all ones, and finally writes a constant to a global variable.
- `calculate_dual_values`: Calls another routine using values from two different global locations, then checks a byte at a global address and calls the same routine again with a different set of parameters.
- `calculate_clamped_value`: Stores a value in a table, performs a multiplication and several arithmetic operations on d7, and then stores the result in d0, branching to another location if the result is negative.
- `clamp_and_store_value`: Calculates a value by adding a 16-bit value from a register to a 16-bit value from another register and subtracting one; if the result is greater than or equal to 384, it sets the value to 384 minus the second register's value, then stores this result in a memory location pointed to by a register.
- `loop_and_copy_word`: Saves registers to the stack, enters a loop that runs 30 times, where it increments a0, fetches a word from the new a0, calls another routine, and stores the result back to a0, then restores registers and returns.
- `check_parameter_bounds`: Saves d0 through d7 and a0 through a2 to the stack, compares d0 to nine, and if d0 is not positive, sets d4 to d0 plus one and branches.
- `calculate_scaled_parameters`: Calculates a value from a global word, uses it to index into a table to retrieve two words, calls a routine, and then enters a loop that performs several calculations and calls a routine multiple times based on a counter.
- `copy_data_and_dispatch`: Writes a word to a global variable, stores a pointer, calls a routine, stores a byte from d0 into a global variable, calls another routine, enters a loop that copies words from a0 to a structure and increments counters, calls a third routine, stores a word in a global variable, and if a counter is not zero, calls a fourth routine before branching.
- `dispatch_callback_chain`: Calls two separate routines in sequence and then returns.
- `execute_indirect_callback_3`: Loads a byte from a memory location into a register, calls a routine using that byte as a pointer, and then jumps to another routine using a pointer from a different memory location.
- `perform_multiple_blit_calls`: Calls several routines with specific word parameters, updates a pointer from a global variable, and performs a final jump to another routine.
- `process_conditional_list_2`: Checks if a word in a global variable is non-zero, and if so, calls two different routines before returning; otherwise it returns.
- `calculate_scaled_value_13`: Initializes several registers, calls another routine, and then performs a loop that calculates a value based on a global word and a global longword, updating a global word with either 31 or 16 during the process.
- `modify_table_entry`: Loads a pointer from a table, fetches a word from a location pointed to by a1, calls another routine, and if the result is zero, writes sixteen to a specific global address before branching to another routine.
- `update_and_call_routine`: Writes 31 to a global word, calls a routine, sets d0 to 52, copies d3 to d1, calls another routine, increments d3, and then pushes d0 to the stack, loads a byte from a global address into d0, calls a routine, and restores d0 before returning.
- `execute_conditional_blit_operation`: Loads two 16-bit values and a 30-count constant into registers, calls check_blit_bounds, and if the zero flag is not set, calls dispatch_callback_and_update_pointer before returning.
- `execute_multiple_tasks_2`: Writes 2 to a global variable, performs four separate calls with various values in d0, d1, d2, and d3, and if certain conditions are met involving a global byte and a byte at a pointer in a0, it calls another routine, stores a word in a global variable, and performs three conditional calls to another routine before setting a byte in a global variable and calling another routine.
- `process_scaled_index`: Checks if a value in d0 is within a specific range, calls two routines to fetch and process data, updates a global word, and performs arithmetic to set several global values.
- `dispatch_blitter_command`: Validates a command index against a global limit, retrieves a pointer and a secondary value through two function calls, and then calls another routine with specific parameters.
- `execute_blitter_sequence_3`: Sets two global words to zero and 65535, calls another routine, performs arithmetic on d3 to calculate new values for d1 and d2, calls another routine, and then calls a third routine before clearing one of the global words.
- `execute_multiple_callbacks_3`: Saves registers to the stack, pushes d0 to the stack, loads a byte from a global address into d0, calls a routine, updates a6 with the result, restores d0, loads a word from a global address into d7, loads two addresses into a0 and a1, calls a routine, pushes d0 to the stack, loads a byte from a global address into d0, calls another routine, restores d0, and restores the saved registers.
- `compare_string_buffers`: Saves registers to the stack, fetches two pointers from a table using values in d5 and d6, calls another routine for each pointer, and then compares bytes from the two resulting locations in a loop until a null byte is found or a mismatch occurs.
- `swap_table_entries`: Swaps two 16-bit values located at offsets determined by two registers within a table pointed to by a register.
- `fetch_and_compare_pointer`: Saves registers to the stack, fetches a pointer from a global address, calls another routine, and then compares a value from the stack with a value from the newly fetched pointer.
- `iterate_and_clear_byte`: Decrements d0 in a loop while incrementing a1 by a value fetched from a1, then calls another routine and clears the byte at the address in a0.
- `measure_and_compare_strings`: Calls a routine to get a length, then uses a byte from a global address to fetch a pointer and compares bytes from two sources in a loop.
- `load_byte_and_setup_pointers`: Loads a byte from the address in a1 into d6, moves it to d5, sets a2 to the address in a0, sets a3 to the incremented address in a1, and then branches to another location.
- `compare_and_increment_buffer`: Compares bytes from two different sources in a loop, and if a match is found, increments d0 and adjusts a1 based on a value from a table before calling another routine.
- `process_event_with_byte_update`: If the high byte of d0 is non-zero and not equal to a global word, it retrieves a pointer from a table, calls a routine, and if a byte at offset 9 of the resulting structure is non-zero, stores d2 into a global byte and calls another routine.
- `update_global_state_2`: Stores d2 into a global byte, stores d1 into a global word, and calls three consecutive routines before returning.
- `load_event_parameter_and_branch`: Loads the constant value 256 into register d0 and branches to another routine.
- `load_event_parameter_and_branch_2`: Loads the value 512 into register d0 and branches to another routine.
- `load_event_id_into_d0`: Loads the constant value 768 into register d0 and branches to another routine.
- `load_event_parameter_and_branch_3`: Loads the constant 1024 into register d0 and branches to another routine.
- `load_event_parameter`: Loads the value 1280 into register d0 and branches to another routine.
- `load_event_parameter_constant`: Loads the constant 1536 into register d0 and branches to another routine.
- `process_event_with_word_check`: If the high byte of d0 is non-zero and not equal to a global word, it retrieves a pointer from a table, calls a routine, and if a byte at offset 9 of the resulting structure is non-zero, stores d2 into a global byte and calls another routine after checking a word at offset 30.
- `update_conditional_state`: Performs a bitwise AND on d3 with a constant, and if the result is non-zero, calls three consecutive routines after storing d2 and d1 into global locations.
- `load_event_parameter_2`: Loads the value 256 into register d0 and branches to another routine.
- `load_event_parameter_3`: Loads the value 512 into register d0 and branches to another routine.
- `load_event_id_and_branch`: Loads the value 768 into register d0 and branches to another routine.
- `load_event_parameter_4`: Loads the value 1024 into register d0 and branches to another routine.
- `load_event_id_into_d0_2`: Loads the value 1280 into register d0 and branches to another routine.
- `load_event_constant`: Loads the constant 1536 into register d0 and branches to another routine.
- `execute_callback_chain_6`: Loads 6 into d0, calls two routines, loads a pointer from a global address into a0, calls a third routine, and jumps to another routine.
- `execute_complex_blit_and_update_fields`: Calls execute_complex_blit and then calls update_conditional_fields before jumping to another routine.
- `execute_sprite_update_sequence`: Calls sync_sprite_data, update_element_arrays, update_and_blit_mask, and draw_complex_scaled_elements in sequence, then updates a global variable with a value based on comparisons of two other global variables before jumping to another routine.
- `set_status_bit_11`: Fetches a pointer from a table, checks specific bits in a word at offset 30 of the structure, and if certain conditions are met, sets bit 7 of the byte at the address in a0.
- `dispatch_callback_via_pointer`: Calls process_list_and_callbacks and then calls call_offset_1e_handler before jumping to another routine.
- `dispatch_event_list_handler`: Calls process_event_list_2 and process_list_and_callbacks, updates a global pointer, and loads a new address into a0 before jumping to another routine.
- `test_and_branch_data_2`: Tests a byte in d0 and branches if it is zero, otherwise it masks the lower byte, decrements d0, and compares it to a value in a global variable.
- `process_conditional_data_5`: Calls two different routines, stores a word from d0 into a global variable, and if a specific bit is set in a word from d7, it stores d0 into a global variable and calls another routine.
- `process_data_table_2`: Loads a pointer into a0, moves d0 to d2, and loads a byte from the address in a0 into d1.
- `update_global_word_5`: Stores a word from d0 into a global variable, moves a byte from d1 into d0, and tests a bit in d0 using a constant in d1.
- `dispatch_tile_update`: Moves a word into a global variable, loads a pointer into a0, extends d0, and calls two different routines before branching to another location.
- `execute_indirect_callback`: Calls another routine via a pointer and then returns.
- `render_ui_text_elements_2`: Calls several routines to fetch data from tables and performs a series of blitter operations to draw text characters or symbols, including loops that iterate through character sets and write specific byte values to memory.
- `configure_blitter_parameters_2`: Sets up three address registers, enters a loop that checks a byte from a table, and if a bit is set, calculates an offset and calls another routine.
- `call_decrement_routine`: Pushes d0 to the stack, fetches a byte from a global address into d0, and calls another routine.
- `process_graphics_parameters`: Calls several routines to retrieve data from a table, updates a global variable, and performs a conditional branch based on a byte at an offset of a structure in a1.
- `render_complex_elements`: Performs several routine calls using various offsets from a structure in a1, including a loop that checks a byte in a table and performs calculations to determine a value for subsequent routine calls.
- `draw_complex_scaled_elements`: Performs a series of scaled blitter operations using various offsets from a structure and updates a global variable based on a comparison.
- `calculate_scaled_dimension_and_update`: Calculates a scaled value by summing two words from a structure in a1, multiplying it by a value from a global variable plus 100, and dividing the result by 100.
- `perform_scaled_calculations`: Writes two to a global word, performs a series of multiplications and divisions using values from a structure in a1, calls three different routines to update parameters and perform drawing, and finally restores registers and returns.
- `execute_callback_and_jump`: Calls a routine via a pointer and then jumps to another routine.
- `validate_sprite_attribute_type_3`: Checks if a byte at offset 4 of a structure equals 3, then verifies that the sum of two 16-bit words at offsets 218 and 220 of a structure pointed to by a retrieved pointer is non-zero; if the check fails, it clears a byte at offset 4 and sets a 16-bit value at offset 20 of the structure to 65535.
- `validate_sprite_metadata`: Checks if a byte in d0 equals 4, then validates a sprite index from a structure in a1 against a table at a fixed address by comparing specific bytes.
- `validate_sprite_attribute_data`: Checks if a byte in d0 equals 7, then uses values from the structure in a0 to index into a table via a1 to validate specific attribute bytes against a secondary data table.
- `process_inventory_data`: Loads 2 into d0, calls a routine, sets d0, d1, and d2 to specific constants, calls three more routines, and then branches based on a byte at a global address.
- `load_and_execute_subroutine`: Loads a pointer from a memory location into a register, calls a routine via that pointer, and then jumps to another routine.
- `execute_composite_blit_sequence`: Calls execute_complex_blit, decrement_and_update_pointer, and update_conditional_fields in sequence.
- `validate_element_position_and_update`: Calls several routines to retrieve a structure, then compares the sum of two short values at offsets 42 and 46 of that structure against a value at offset 270 divided by 1000; if the sum is less than the quotient, it updates a global variable with a specific constant, otherwise it sets the global variable to 2.
- `dispatch_event_list_and_update_state`: Calls process_event_list_2 and process_list_and_callbacks, then calls fetch_and_store_word to load a word from a global address and store it at a table offset determined by d1, before jumping to a new address loaded into a0.
- `execute_callback_at_offset_30`: Calls a routine, then calls another routine, and finally jumps to a third routine located at offset 30 of a structure pointed to by a global variable, provided that the global variable and the pointer at that offset are both non-zero.
- `process_entity_event_logic`: Calculates an offset into a table based on a global index and a retrieved pointer, then performs complex conditional checks on entity properties and bitmasks to trigger specific subroutines or update global state variables.
- `update_entity_state_and_trigger_event`: Checks two global status bytes and, if both are zero, calls two routines to validate a condition before incrementing a global counter and updating a pointer.
- `test_bit_and_call_12`: Loads a byte from offset 3 of a structure in a0 and a word from offset 34 of a structure in a2, tests a bit in the word using the byte as an index, and if the bit is set, loads 243 into d0, calls another routine, and branches to another routine; otherwise it branches to another routine.
- `test_bit_and_call_11`: Calculates an offset using bytes from a structure and a multiplier, retrieves a byte from that offset, and calls a routine if a specific bit in a global word is not set.
- `branch_on_bit_test_3`: Adds five to a word loaded from a global pointer into d4, tests a bit in d0 based on d4, and if the bit is set, sets d0 to 245 and calls another routine before branching to another location.
- `update_global_pointers_5`: Stores bytes from offsets 23 and 24 of the structure in a2 into two global words, then if a byte at a global address is zero, increments a global word and copies a word from one global location to another before calling another routine.
- `update_table_entry_4`: Calls another routine, calculates an offset based on a value from a global variable, updates a global pointer with the address of a structure element, and copies several bytes from that structure to other global locations before calling another routine and setting a byte in a global variable.
- `compare_and_call_routine_13`: Compares a byte at offset 1 of a structure in a2 to 19, and if it is not equal, branches to another routine; otherwise it loads a long word from a global address into d0, loads a byte from offset 21 of the structure in a2 into d1, tests a bit in d1 using d0 as an index, and if the bit is set, loads 48 into d0, calls another routine, and branches to another routine; otherwise it branches to another routine.
- `set_and_call_routines`: Sets bit d0 of d1, stores d0 in a global word, increments a value from a global word into d0, calls another routine, sets d0 to 43, calls another routine, and branches to another location.
- `process_structure_and_call_routines`: Checks if a byte at offset 1 of a structure is 8, and if so, performs a sequence of calls using bytes from offsets 21 and 22 of the structure and a value from a global pointer.
- `check_byte_and_call`: Compares a byte at offset 1 of a structure in a2 to 20, and if equal, tests a longword at a global address; if that longword is non-zero, sets d0 to 63 and calls another routine before branching to another location.
- `process_conditional_calls`: Checks if a global byte is less than 2 and if another global byte is non-zero, then if specific bits are set in a global word, increments a global word, calls three different routines, and branches to another location.
- `execute_nested_call`: Loads the value 442 into d0, calls another routine, pushes d0 to the stack, loads a byte from a pointer in d0, calls another routine, restores d0 from the stack, and returns.
- `process_event_data_4`: Updates a global word, calls a routine, and if a value is non-negative, calculates an offset into a table to retrieve a pointer, then performs conditional logic involving several global variables and a routine call.
- `update_event_parameter`: Checks a bit in a structure and a byte in a passed pointer, then if both conditions are met, updates several global variables from the pointer, calls a routine, and if a specific global is non-zero, calls another routine to increment a global counter and calls a third routine.
- `update_structure_fields_13`: Calls several routines to retrieve a pointer, updates a global structure with a value from a structure in a0, and performs arithmetic on a field at offset 270 of that structure.
- `update_structure_fields_alt`: Calls several routines to retrieve a pointer, updates a global structure with a value from a structure in a0, and performs arithmetic on a field at offset 270 of that structure using a different multiplier.
- `fetch_structure_pointer`: Calls two routines, checks if a global word is non-negative, and if so, calculates an address based on a value from a structure and copies a pointer and a word to a global location.
- `adjust_structure_values_sub_2`: Retrieves a word from offset 24 of a structure, stores it in a global variable, sets a global byte to 1, clears a longword in a global variable, and performs arithmetic on fields at offset 18 and offset 270 of a structure if a global pointer is non-zero.
- `adjust_structure_values_sub`: Calls several routines, and if a global word is non-zero, performs arithmetic on a field at offset 24 and a field at offset 270 of a structure, then repeats the process with an addition and a different multiplier.
- `adjust_structure_values_add_2`: Retrieves a word from offset 26 of a structure, stores it in a global variable, sets a global byte to 2, clears a longword in a global variable, and performs arithmetic on fields at offset 26 and offset 270 of a structure if a global pointer is non-zero.
- `adjust_structure_values_add`: Calls several routines, and if a global word is non-zero, performs arithmetic on a field at offset 26 and a field at offset 270 of a structure, then repeats the process with an addition and a different multiplier.
- `handle_structured_event`: Updates a global word, calls a routine, and if a value is non-negative, calculates an offset into a table to retrieve a pointer, then updates several global variables and calls a routine if certain conditions are met.
- `process_conditional_call_2`: Increments a word from a global variable, calls another routine, and if d2 is not zero, calls another routine with d0 set to 92 before calling another routine and returning.
- `update_structure_values_16_2`: Retrieves a word from offset 24 of a structure, retrieves a word from offset 144 of another structure, and if the difference is not negative, updates a global longword, sets a global word to 86, sets a global byte to 1, clears a longword, and performs arithmetic on fields at offset 18 and offset 90 of a structure.
- `update_structure_values_16`: Stores a longword from d0 into a global variable, sets a word at offset 22 of a global structure to 86, sets a byte at offset 237 of a global structure to 1, clears a longword at offset 3520, calls another routine, and if a global longword is non-zero, performs arithmetic on a structure in a0 using values from d0 and a global longword before calling two more routines.
- `update_structure_values_alt_2_2`: Retrieves a word from offset 26 of a structure, retrieves a word from offset 146 of another structure, and if the difference is not negative, updates a global longword, sets a global word to 87, sets a global byte to 2, clears a longword, and performs arithmetic on fields at offset 26 and offset 92 of a structure.
- `update_structure_values_alt_2`: Stores a longword from d0 into a global variable, sets a word at offset 22 of a global structure to 87, sets a byte at offset 237 of a global structure to 2, clears a longword at offset 3520, calls another routine, and if a global longword is non-zero, performs arithmetic on a structure in a0 using values from d0 and a global longword before calling two more routines.
- `modify_graphics_data_2`: Calculates an offset into a structure using d2, checks a bit in a table, and if conditions are met, sets a bit in a structure and calls several routines.
- `execute_sub_routine_sequence`: Subtracts two values from d0, calls two routines, and then calls a third routine using a byte from a global pointer.
- `initialize_structure_pointer`: Saves registers, calls another routine to retrieve a pointer, stores the result plus an offset into a global variable, sets a global pointer to a structure, calls another routine, and calls a third routine before returning.
- `loop_structure_updates_2_2`: Calculates an offset into a structure, copies two words to global variables, and if a value from a structure is non-zero, performs arithmetic on a table value and updates several global variables and a structure.
- `loop_structure_updates_2`: Calls another routine, performs arithmetic on d0 using a value from a global variable, calls two more routines, writes two words from global variables to a structure in a0, calls a third routine, and enters a loop that repeats these operations using d7 as a counter.
- `loop_structure_updates`: Executes a loop where each iteration calls a routine with a byte from a global pointer, performs subtractions, and calls two more routines.
- `update_structure_and_loop_data`: Calculates an offset into a table to retrieve two words, then calls a routine to process a pointer.
- `loop_structure_updates_with_scale`: Calls another routine, performs arithmetic on d0 using a value from a global variable, calls two more routines, writes two words from global variables to a structure in a0, calls a fourth routine with specific parameters, and enters a loop that repeats these operations using d7 as a counter.
- `loop_structure_updates_3`: Executes a loop where each iteration calls a routine with a byte from a global pointer, performs subtractions, and calls two more routines.
- `fetch_and_store_word`: Loads a word from a global address, calls another routine, and stores the returned word at an offset in a table determined by d1.
- `process_structure_data_2`: Calls several routines to manipulate data within a structure pointed to by a0, including performing arithmetic on a value at offset 270 and conditionally updating bytes at specific offsets using values from a table.
- `update_entity_status_flags`: Retrieves an entity structure via a call, then performs multiple conditional checks on its fields to set specific bits in a status structure passed in a0.
- `process_graphics_structures_2`: Calls two routines, sets a longword in a global variable, sets a byte in a global variable, sets a longword in a global variable, and copies a word from a table into a structure at offset 18.
- `process_graphics_structures`: Executes a loop to call a routine, then performs multiple conditional checks involving table lookups and structure offsets to call various routines that modify data or perform blitting operations.
- `clear_and_call_routines`: Clears a longword at a global address, sets d0 to 44, and calls swap_and_invert_pointers and call_subroutine_via_pointer before returning.
- `execute_sequence_calls`: Clears a byte in a global variable and calls swap_and_invert_pointers, process_bit_flag, and calculate_clipping_area in sequence before returning.
- `check_bit_and_store`: Calls another routine, checks if a specific bit is set in a word from a global variable, and if so, stores the high byte of d0 into a global variable and calls another routine.
- `process_bitmask_table`: Iterates through a table, retrieves a pointer, and checks several conditions in a structure in a2 including bit tests and arithmetic comparisons, then stores a bitmask in a global variable.
- `process_bitmask_table_scaled`: Iterates through a table, retrieves a pointer, and performs arithmetic checks on a structure in a2 using a multiplier of 250, then stores a bitmask in a global variable.
- `write_conditional_byte`: Checks a bit in d0 using a value from a global variable and, if the bit is not set, writes the byte 8 to the address in a0 and increments it.
- `write_byte_and_loop`: Writes the byte 9 to the address in a0 and increments a0, then enters a loop that increments d7 and branches back to the start if d7 is less than 7.
- `update_complex_graphics_data`: Performs a series of calculations and table lookups to call multiple routines that update global pointers, modify bytes in a structure, and execute various scaled blitting and callback operations.
- `check_and_process_blit`: Checks a bit in a global structure, compares a global word to 6, and if both conditions are met, calls another routine with specific parameters and then calls a third routine.
- `process_scaled_tile_data_3`: Calculates an offset using a word from a global variable, updates a global variable, and performs a series of blit operations using bytes from a structure at a calculated address.
- `calculate_string_offset_2`: Fetches a byte from a structure, calls a routine, performs arithmetic on a value multiplied by 30, and calls several routines to update a global word and a structure field.
- `update_global_pointer_6`: If a byte at a global address is non-zero, it calls another routine, updates a global variable with a value from a table, and then calls another routine.
- `negate_if_nonzero`: Checks a byte in a global variable; if it is not zero, it negates the value in register d0 and returns, otherwise it returns.
- `process_indexed_event_handler`: Calls a routine to retrieve an index, uses that index to calculate an offset into a table to fetch a pointer, and if a word at offset 4 of the resulting structure is non-zero, calls another routine after checking a bit in a secondary indexed table.
- `call_decrement_handler_3`: Tests a byte at a global address and, if bit 3 of a byte at offset 32 of a structure in a1 is set, calls a routine with d0 set to 56.
- `copy_and_update_structure`: Copies a longword and a word from a structure in a2 to a structure at offset 1250, calls another routine, calls three more routines, and then updates three global variables from specific sources before calling a final routine.
- `call_decrement_handler`: Loads a byte from a memory location into d0 and calls another routine.
- `store_byte_and_branch`: Stores a byte from a memory location into a destination and then branches to another routine.
- `copy_and_update_structure_extended_2`: Checks a global byte, updates a global word, clears a longword, retrieves a word from a structure, and if the word is not 1, updates a global longword with a value from a structure.
- `copy_and_update_structure_extended`: Stores a longword from d0 into a global variable, copies a word from a2 to a global variable, sets a word at offset 22 of a global structure to 54, clears a global byte, calls another routine, and if a global longword is non-zero, copies a structure from a2 to offset 1250 and performs multiple routine calls and global updates.
- `call_decrement_handler_2`: Loads a byte from a memory location into d0 and calls another routine.
- `load_and_jump_to_handler`: Loads two 16-bit words from memory into a destination address specified by a pointer and then jumps to a new address.
- `execute_triple_calls`: Calls swap_and_invert_pointers, sets d0 to 55, and then calls call_subroutine_via_pointer and update_graphics_state before returning.
- `execute_blit_pipeline_2`: Pushes a pointer from a global variable onto the stack and calls swap_and_invert_pointers, process_bit_flag, calculate_clipping_area, and update_conditional_fields in sequence before jumping to a final destination.
- `calculate_structure_offset_and_dispatch`: Calculates an offset into a table by multiplying a value from a structure by 60, retrieves a pointer from that location, and calls another routine.
- `dispatch_byte_value`: Loads a byte from offset 2 of a structure in a1 into d0, and if it is not zero, loads 34 into d0, calls another routine, and branches to another routine; otherwise it branches to another routine.
- `update_structure_index_and_dispatch`: Updates a global word with a value from d0, calculates an offset into a structure based on d0, and checks a pointer at that offset.
- `test_bit_and_call_9`: Tests a byte at a global address, and if it is non-zero, tests bit 3 of a word at offset 32 of a structure in a4; if that bit is set, sets d0 to 56 and calls another routine before branching to another location.
- `copy_structure_data_5`: If a byte at a2 is 1, copies a pointer and a word from a3 to a new location, calls another routine, stores a global pointer, and if bit 1 of a2 offset 3 is set, calls another routine.
- `test_bit_and_call_7`: Loads a byte from offset 3 of a structure in a0 and a word from offset 22 of a structure in a1, tests a bit in d0 against d1, and either branches to another routine or calls another routine and then branches to another location.
- `test_and_branch_logic`: Loads a byte from offset 1 of a structure in a0 and a byte from offset 4 of a structure in a1, tests a bit in d0 against d1, and either calls a routine or branches.
- `compare_and_branch_11`: Loads a byte from offset 5 of a structure in a1, and if it is not zero, compares it with the value of 2 minus a byte at offset 6 of a structure in a0; if the result is negative, it calls another routine and branches to another location.
- `compare_and_call_routine_8`: Loads a byte from an offset of 6 in a structure into d0, and if that byte is not zero, compares d0 with the result of subtracting a byte at offset 7 of another structure from 2; if the comparison results in a negative value, it loads 38 into d0, calls another routine, and branches to a different location.
- `update_structure_from_table_5`: Checks if a global byte is non-zero, calls another routine, copies a structure from a2 to offset 1250, calls five more routines, and updates three global variables from specific sources.
- `process_structure_data_9`: Loads a pointer into a0, a byte into d0, and a word into d2, then calls three routines and, if a word at a specific offset is non-zero, calls a fourth routine before branching.
- `process_structure_byte`: Calls two routines, pushes a word from d0 to the stack, loads a byte from a global address into d0, calls another routine, and restores d0 from the stack before returning.
- `branch_on_byte_test_3`: Tests a byte at a global address, and if it is zero, branches to another location; otherwise, it loads a byte from a global pointer into d0, loads a word from a global pointer into d2, loads a pointer into a0, calls another routine, and branches to another location.
- `update_entity_attribute_and_offset`: Calculates an index into a table based on a value from another routine, then accesses a structure to compare and update a byte value.
- `apply_scaled_offset_and_reset_flag`: Adds a byte from a register to a memory location, calculates a scaled value by multiplying a 16-bit word from a structure by the register value and adds the result to a 32-bit integer at an offset in a structure, then clears a 16-bit word at offset 4 of a structure before returning.
- `copy_and_scale_structure`: Copies a longword and a word from a structure in a3 to a structure in a2, calculates a value by multiplying a byte from a3 by a word from a1 and adding it to a longword at offset 270 of a0, then clears two bytes in a3 and branches to another routine.
- `update_graphics_and_globals`: Copies a structure from a2 to a4, copies a word from a4 to a4, copies a byte from a global variable to d0, copies a word from a global variable to d2, calls several routines including one that uses a3 and a2, updates several global variables from d0, d2, and other globals, and branches to another routine.
- `update_graphics_list`: Calls two routines, checks a word at an offset of a structure in a3, and if non-zero calls two more routines, then loads a byte from a global variable and calls a final routine.
- `update_entity_attribute_and_offset_3`: Compares a value from a global structure against a threshold, and if it exceeds a specific limit, calculates an index to access a table.
- `execute_conditional_callbacks_5`: Calls a routine, then checks a bit in a word from a global variable; if the bit is set, it retrieves a byte from a table using an offset in d0, calls another routine, and then performs a series of conditional calls based on a global word and a comparison with a value in d7.
- `update_structure_elements_2`: Calls a routine to retrieve a pointer, then enters two loops to iterate through a structure, performing conditional arithmetic and updates to bytes and a long value at offset 270 based on table values.
- `update_structure_values_7`: Copies a structure from one address to another, decrements a byte at the source, and adds the original byte value to the destination while also adding a multiplied value to a global variable.
- `loop_through_structure`: Copies an address to another register and enters a loop that runs up to twenty-three times, checking a word at an offset and incrementing the pointer if it is non-zero.
- `update_structure_and_decrement`: Copies a structure from a2 to a4, decrements a byte in a2, adds a value from a1 to a global variable at offset 270 of a0, and calls another routine if a byte in a2 is zero.
- `subtract_structure_values_2`: Calls a routine to retrieve a pointer, calculates an offset into a structure, and if a comparison with d2 is met, subtracts values from the structure and updates a long value at offset 270.
- `subtract_table_elements`: Subtracts bytes and words from a structure using values from a table, performs conditional additions to indexed locations in another structure, and calls another routine if a bit is set.
- `subtract_and_clear_offset`: Calls another routine, subtracts a word from offset 38 of a1 from a long value at offset 270 of a0, clears a word at offset 4 of a2, and calls another routine.
- `update_and_add_table_values`: Calls a routine to get a pointer, updates a linked structure, and then adds values from a table to a structure while conditionally subtracting values from indexed locations.
- `add_and_decrement_counter`: Calls another routine, adds a word from offset 38 of a1 to a long value at offset 270 of a0, decrements a byte at a4, and calls another routine if the byte is zero.
- `add_table_values`: Adds bytes from a source table to specific offsets in a destination structure, including conditional additions to two indexed locations within that structure based on values in the source table.
- `subtract_table_values`: Subtracts bytes from a source table from specific offsets in a destination structure, including conditional subtractions to two indexed locations within that structure based on values in the source table.
- `update_graphics_state`: Calls a routine to retrieve a byte, stores it in a global variable, and then writes two words and a byte to a structure at offset 73e before calling another routine.
- `update_structure_pointers_5`: Calls a routine with a specific address, calls another routine, and then calls a third routine to process a byte from a global variable.
- `update_structure_byte_2`: Loads a byte from a global variable, calls a routine, then loads a byte from a global variable, calls another routine, and finally stores the result into a global variable at offset d7f.
- `manage_complex_graphics_state`: Saves registers, calls several routines to retrieve values, updates a global pointer and two global words, and performs a series of conditional routine calls and data updates.
- `validate_and_loop_elements`: Retrieves a pointer from a table using a value from a structure, calls another routine, and then enters a loop that checks specific bitfields and numeric bounds within a structure to conditionally set a bit in a global variable.
- `set_bit_and_loop`: Sets a bit in d6, loads a byte from a1, calls another routine, and loops if d7 is less than 7, otherwise it stores d6 in a global variable.
- `fetch_and_test_structure_flags`: Retrieves a pointer from a routine, calculates an offset into a table using a value from that pointer, and calls another routine if specific bits in a bitfield at the calculated offset are not set.
- `modify_parameter_value`: Loads a byte from a global address, calls two routines, and if d0 is not equal to 5, performs a comparison and potentially calls another routine.
- `perform_triple_call`: Subtracts 40 and 9 from d0, calls another routine, calls another routine, calls another routine using d0, and restores registers before returning.
- `copy_table_elements_2`: Compares a word in d0 to ten, and if it is not less than ten, subtracts a word from a global variable, calculates an offset into a table, copies two words from that table to a new location, and calls three other routines.
- `update_entity_status_flags_3`: Retrieves two pointers from a table, checks specific bits in a structure, and conditionally sets bits at offsets 6, 8, 10, and 12 of a structure in a0 based on global flags and the results of other routines.
- `set_status_bits_3`: Calls a routine to check a value, then conditionally sets various bits at offsets 6, 8, 10, and 12 of a structure in a0 based on register and global variable states.
- `update_pointer_and_call_3`: Compares a byte at offset 84 of a global structure to 2, and if it is negative, loads a long value into a global pointer; otherwise it loads the address of a table into a0, calls another routine, and branches to another routine.
- `load_and_compare_words`: Loads two words from memory, calls another routine, and if the result of a comparison with a global byte is not positive, calls another routine and then calls the first routine again before returning.
- `update_pointer_and_call_3_2`: Compares a byte at offset 84 of a global structure to 2, and if it is negative, loads a long value into a global pointer; otherwise it loads the address of a table into a0, calls another routine, and branches to another routine.
- `load_and_compare_words_2`: Loads two words from memory, calls another routine, and if the result of a comparison with a global byte is not positive, calls another routine and then calls the first routine again before returning.
- `check_mouse_and_dispatch_event`: Compares a byte at a hardware register to 2 and, if the value is 2 or greater, updates a global variable with a constant and calls another routine using a pointer from a table; otherwise, it returns.
- `execute_conditional_calls_17`: Loads two words from memory into d0 and d1, calls another routine, and then performs a sequence of conditional calls based on the results of the first call and subsequent comparisons.
- `process_event_pointer_2`: Pushes d0 to the stack, retrieves a pointer from a global variable, calls another routine, and then uses the returned pointer to access a byte at offset 4.
- `decrement_loop_counter_and_branch`: Decrements a loop counter in register d7 and branches to a target address if the counter is not zero; otherwise, it performs a short branch to another location.
- `execute_event_logic_2`: Loads a pointer into a0 and calls another routine, then if a global word is non-zero, sets a global byte and calls another routine.
- `dispatch_conditional_subroutine_3`: Tests a byte in a longword at a specific memory location; if the byte is non-zero, it loads the value 39 into a register and calls another routine, otherwise it continues to another routine.
- `execute_indirect_calls`: Calls a routine via a pointer at a specific address, loads a new address into a0, calls another routine, and then returns.
- `select_event_type_by_index`: Compares a word at a global address to six; if it is not six, it returns with zero in register d0, otherwise it sets d0 to fifteen and continues to another routine.
- `handle_event_logic`: Pushes d0 to the stack, calls another routine with a byte from a global address, uses the returned pointer to load a word into d1, pushes d0 to the stack, calls another routine with a byte from a global address, and if bit 3 of d1 is not set, calls another routine with d0 set to 289.
- `set_conditional_byte_4`: Checks if a word at a global address is non-zero, otherwise it sets a byte in a global variable to fifteen, calls a routine, loads an address into a0, calls another routine, and returns.
- `dispatch_event_handler_and_update_pointer`: Calls process_event_list_2, loads a pointer into a0, and then calls call_offset_22_handler before returning.
- `dispatch_event_list_handler_2`: Loads a pointer from a memory location into a0 and jumps to another routine.
- `populate_structure_table`: If a global byte is zero, it calls two routines, calculates two offsets based on global words, stores these offsets and a byte into a structure at a global address, then enters a loop that performs 24 iterations of storing a longword and calling a routine.
- `load_table_values`: Loads two words from memory, compares a byte at a global variable to 2, and if the byte is not positive, loads a third word into d2 and calls another routine.
- `calculate_shifted_offsets_2`: Loads two words from a table using an index from a global variable, performs multiplication and bit shifting on these values, modifies two global longwords by subtracting and adding the results, and calls another routine.
- `update_event_list_and_dispatch`: Calls a routine, updates a global pointer with the constant an address, loads a new address into a0, and jumps to another routine.
- `compare_and_call_handler`: Performs arithmetic on two values from global variables, compares the results against constants, and if the conditions are met, loads a pointer into a0 and calls another routine.
- `load_pointer_and_call`: Loads a longword from a memory location into a0 and calls another routine.
- `calculate_and_dispatch`: Performs arithmetic on two sets of values from global variables, calls two routines, and jumps to an address contained in a global pointer.
- `validate_range_and_call`: Calls another routine, then checks if the result in d0 satisfies a range condition relative to a global word at offset 3704, and if the value in d1 satisfies a range condition relative to a global word at offset 3706, before calling another routine.
- `calculate_structure_offsets`: Calculates and stores three 16-bit values into a structure at offsets zero, two, and four, using values derived from two global variables and applying bounds checks.
- `calculate_structure_offsets_alt`: Calculates and stores three 16-bit values into a structure at offsets zero, two, and four, using different arithmetic constants and bounds checks compared to another routine.
- `process_table_entries_3`: Compares a byte in a global variable to one, iterates through a table using a0, performs several conditional checks on table entries, and if conditions are met, calls one routine, then calls another routine using a byte from a global pointer, then calls a third routine using a byte from another global pointer, and finally calls a fourth routine.
- `increment_and_call_2`: Increments d7 and compares it to 32, calls another routine if it is less than 32, sets the sign bit of d5 based on the result, and restores registers from the stack before returning.
- `execute_conditional_logic_10`: Saves registers to the stack, calls another routine, and if the result is not negative, calls another routine; if that routine returns a non-zero value, it checks if a byte from a global pointer is 13 and if a byte at offset 1 of that pointer is 20, otherwise it calls another routine, stores a longword from a global pointer into a global variable, calls another routine, and restores the global variable.
- `execute_and_restore`: Calls lookup_and_call, sets the high bit of d7, and restores d0, d1, and d7 from the stack before returning.
- `lookup_and_call`: Saves d0 and a0, loads a byte from a table using a0 and d0, and if that byte is not zero, calls another routine, masks d0, calls another routine, and sets d0 to 255 before returning.
- `check_attribute_conditions`: Calls two routines and, if the results meet specific criteria involving a byte from a global structure and specific bit tests, it sets a global byte to 255, calls another routine, and then sets the global byte to zero.
- `perform_conditional_sequence`: Calls another routine and, if the zero flag is set, calls a third routine before returning.
- `check_structure_conditions`: Compares a byte at a global address to one, then if not equal, loads a pointer from a global address into a0 and performs several conditional checks against values at offsets 0, 2, 10, and 12 of the structure in a0, eventually calling another routine with d0 set to 169 if certain conditions are met.
- `process_structure_callbacks`: Checks a bit in a byte at offset 2 of a structure in a0, then performs a series of conditional calls using pointers retrieved from global tables and bytes from the structure.
- `update_and_call_structure`: Copies d7 to a global word, sets a byte based on the sign flag, calls another routine, loads a pointer from a global address into a0, calls another routine, sets d5 to 255, and branches to another location.
- `execute_nested_callbacks`: Calls two different routines using pointers retrieved from a table and a byte from a structure, then sets d5 to negative one and branches to another location.
- `update_d7_and_call`: Increments d7, compares it to thirty-two, and if it is less than thirty-two, calls another routine and sets d5 based on the result.
- `validate_global_state`: Compares several global words and bytes, and if they match specific criteria, calls a routine and potentially another routine with d0 set to 120.
- `update_global_state_3`: Calls two routines, writes a word and a pointer to two global variables, calls a third routine using an address from a structure, and if a word from a global variable is one, calls a fourth routine before branching.
- `fetch_and_process_index`: Copies two bytes from a table into global variables, stores a word in a global variable, adds d0 to itself, loads a word from a table into d0, calls a routine, copies a byte into a global variable, stores a longword in a global variable, calls two more routines, and restores registers before returning.
- `process_conditional_file_data`: Saves registers, calls two routines, branches if the first returns a negative value, checks a byte from a global variable against specific values, calls a routine, loads an address into a0, calls another routine, calls a third routine, and if a global byte is zero, copies a word from one global variable to another before returning.
- `initialize_structure_field`: Copies a byte from a global address to a new location, clears a global long value, sets a global pointer to a specific address, and writes 24 to a structure at offset 36 of a0 before calling another routine.
- `initialize_and_fill_table`: Clears eight bytes starting at a global address, then iterates through a loop eight times, using a byte from a second global table as an index to write the current loop counter into the cleared memory, and finally calls another routine.
- `loop_and_write_table`: Loads pointers into a0 and a1, sets d7 to 11, and enters a loop that reads a byte from the pointer in a0, extends it to a word, writes the value 11 to a location calculated from the byte and a1, and decrements d7 until it reaches -1 before jumping to another routine.
- `execute_sequence_of_calls_10`: Sets d0 to 10, calls one routine, calls a second routine, loads a pointer from a global address into a0, calls a third routine, calls a fourth routine, sets d0 to 18, and jumps to another routine.
- `jump_to_complex_blit`: Performs a long jump to another routine.
- `process_scaled_tile_loop`: Iterates through a table using a loop, performing a series of calls to update structure fields, modify bytes based on table lookups, and execute three consecutive callbacks for each entry.
- `dispatch_callback_and_jump`: Calls process_list_and_callbacks and then jumps to call_offset_1e_handler.
- `update_scaled_value_2`: Calls a routine, checks if d0 is non-zero, if so it extends d0 to a word, stores it in a global variable, adds it to a pointer in a0, checks a byte at that pointer, and if non-zero clears it, calls two routines, extends d0, stores it in a global variable, loads an address into a0, calls another routine, and branches if the global word is non-zero.
- `set_global_pointers`: Writes two words to a global structure, fetches a byte from a table, calls a routine, and then swaps two global words before jumping to another routine.
- `update_function_pointers_and_jump`: Loads a pointer from a global variable into a register, writes two 16-bit words from a different global memory location into the memory pointed to by that register, and then performs a long jump to another routine.
- `swap_byte_at_offset`: Calls a routine, and if d0 is non-zero, extends d0 to a word, stores it in a global variable, calculates an address by adding d0 to a global pointer, and if the byte at that address is zero, stores a byte from a global variable there and clears that global variable; otherwise, it swaps the byte at that address with a global variable.
- `swap_and_store_byte`: Moves a byte from a source to a destination, extends it to a word, stores it in a global variable, and calls three different routines.
- `fetch_offset_and_branch`: Loads a 16-bit word from a specific memory location into a register and then branches to a preceding instruction.
- `process_conditional_pointers`: Sets a global variable to zero, calls a routine, and if a global byte is zero, calls two other routines using pointers retrieved from a table and then calls a fourth routine based on a bit test.
- `load_constant_and_branch`: Loads the decimal value 19 into a data register and branches to another routine.
- `execute_handler_sequence_2`: Loads the constant 18 into a register, calls two separate routines, and then jumps to another routine.
- `execute_blit_sequence_with_flags`: Loads a byte from a pointer into a register, then calls decrement_global_byte_7, update_structure_flags_6, execute_complex_blit, and finally jumps to another routine.
- `initialize_handler_state`: Stores the value 3 in a global word, then calls three routines and jumps to another routine, provided that a global pointer and a value at an offset of 38 from that pointer are both non-zero.
- `modify_status_flags`: Calls a routine to retrieve a pointer, then conditionally sets bit 7 at offsets 0, 6, or 12 of a structure in a0 based on the value of a byte at a global address and a byte in a1.
- `set_status_bit_and_call`: Calls another routine, and if the zero flag is set, sets bit 7 of a byte at offset 6 of a structure in a0, then calls a second routine.
- `dispatch_conditional_routines`: Calls two routines, checks a byte at a global address, and if that byte is non-zero, calls a third routine before branching to another location.
- `load_and_dispatch_words`: Sets the sign bit of a byte at a global address, loads two words from a structure into global variables, and if the third word is not zero, writes it to a global variable and calls two other routines.
- `program`: Calls an initialization routine, a main routine, and an exit routine before returning.
- `check_attribute_and_call_2`: Tests a byte at a global address, branches if non-zero, compares a word at a global address to 8, and if it is not greater than or equal to 8, sets d0 to 176 and calls a routine before returning.
- `process_attribute_list_and_calculate`: Iterates through a list of six bytes, calling two different routines for each non-zero byte to retrieve and process data.
- `calculate_scaled_value_19`: Performs arithmetic on d7 using a subtraction, a multiplication, and a division, then branches to another instruction.
- `execute_fixed_call_and_reset_global`: Loads the value 24 into a data register, calls a routine at a fixed address, loads 95 into another data register, clears a longword in a global variable, and branches to another location.
- `process_table_loop_4`: Clears a word in a global variable, then loops through a table of pointers.
- `process_table_and_clear`: Loads a word from a global variable and a byte from a pointer, and if the byte is non-zero, calls a routine, checks a bitmask on a word at an offset of 46 from the resulting pointer, and if certain conditions are met, decrements a word at offset 26 and a longword at offset 238 of a structure; if a loop finishes, it clears six longwords starting at a global variable.
- `decrement_and_clear_table`: Decrements two values in a structure, calls two routines, and then enters a loop that clears six consecutive longwords starting at a global address.
- `process_event_table_entry_2`: Initializes a structure and calls several routines to process data, then uses a value from a global variable to index into a table.
- `test_bit_and_branch_4`: Loads a byte from offset 4 of a structure in a0 and a byte from offset 23 of a structure in a1, tests bit d0 of the value in d1, and if the bit is set, branches to another location; otherwise, it loads 21 into d0, calls another routine, and branches to another location.
- `test_bit_and_call_10`: Calculates an offset into a table using a value from a register, tests a bit in a longword at that offset, and either calls another routine or branches to a different location.
- `calculate_sprite_offset_and_update`: Calculates an offset into a table using a value from a byte at offset 23 of a structure, adds a value from a register, and multiplies the result by 5 to find a target address.
- `update_sprite_attribute_and_check_bounds`: Writes a 16-bit value to offset 20 of a structure in a0, then calls another routine to check if the sum of two 16-bit values at offsets 186 and 190 is negative.
- `update_bitmask_and_process_table`: Sets a bit in a longword pointed to by a register using a bit index from a byte at offset 24 of a structure, then iterates through a loop of 23 elements by calling another routine.
- `copy_scaled_table_data`: Subtracts a word from a global variable from d0, increments the result, calls two routines, calculates an offset into a table using a global pointer, copies two words from that table to a new location, and calls three other routines.
- `update_graphics_parameters_3`: Subtracts a word from a global address from d0, calls two routines, calculates offsets into tables using d0 and d1, and calls two more routines to update data.
- `clamp_and_update_structure`: Calculates clamped values based on sums and differences of fields in a structure, calls several routines to update a table and a global pointer, and if a certain condition is met, writes two addresses into a structure and calls another routine.
- `update_structure_pointers_4`: Calls another routine, then writes two specific addresses into a structure at offset 8 and offset 16, and finally calls another routine.
- `initialize_parameters_and_branch`: Tests a byte at a global address, and if it is zero, calls another routine, otherwise it sets d0, d1, d2, and d3 to specific values and branches to another routine.
- `initialize_loop_parameters`: Compares a byte at a memory location to 2, and if it is not positive, loads 16 into d0, 49 into d1, 11 into d2, and 9 into d3 before performing a branch.
- `perform_nested_blit_loops`: Executes nested loops that call a routine 10 times in the inner loop and 9 times in the outer loop, performs arithmetic on d1 and d2, calls a routine with a calculated offset, and finally updates a global byte and calls two more routines.
- `test_byte_and_execute_sequence`: Tests a byte in a global variable and, if non-zero, calls another routine, loads a pointer into a0, calls another routine, stores a byte into a global variable, and branches to another routine; otherwise it returns.
- `execute_graphics_sequence_callback`: Calls a routine and then jumps to another routine.
- `set_status_bit`: Checks a byte in a global variable; if it is not zero, it sets bit 7 of a byte at offset 12 of the structure pointed to by a6, then returns.
- `process_status_flags_3`: Loads a word from a memory location into d7, calls a routine, and then performs several conditional checks involving bit testing and bitwise inversion on values at various memory locations, calling other routines based on the results.
- `process_status_flags_2`: Calls another routine with a byte from a pointer in d6, checks bit 1 of a byte at a pointer in d6, and if bit 4 of d7 is set, toggles bit 1 of a byte at a pointer in d6 and conditionally calls two other routines.
- `check_and_call_routines`: Checks bit 0 of a byte in a global variable; if it is not set, it calls another routine using a byte from a global variable, otherwise it returns.
- `execute_handler_calls_2`: Loads 60 into d0, calls another routine, and if d0 is zero, calls two more routines before performing an indirect jump.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `execute_conditional_logic_8`: Loads 34 into d0, calls a routine, stores the result in a byte at a memory location, writes 96 into a word at a memory location, loads an address into a0, calls another routine, and then performs conditional branches based on a word at a memory location and the byte stored earlier.
- `execute_conditional_calls_4`: Tests a byte in a global variable and, if non-zero, branches to another routine; otherwise, it loads a value into d0 from a pointer in a global variable, calls another routine, and branches to another routine.
- `execute_table_and_data_routines`: Checks a global byte, performs a loop to copy data from a table, calculates an offset, calls several routines, and updates a value in a structure via a pointer.
- `process_pointer_and_clear`: Calls two routines, stores a byte from a register into a memory location, and enters a loop that checks a series of bytes in a structure.
- `update_global_pointer_via_validation`: Validates a value from a global variable and, if non-zero and a subsequent check passes, stores that value into another global variable and calls a final routine via a jump.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `execute_blitter_setup`: Clears a 16-bit word in a global variable, performs two routine calls with specific register parameters, writes 31 to a global variable, performs a third routine call with different register values, and jumps to another routine.
- `execute_blitter_sequence_2`: Calls a routine with specific parameters, stores a value in a global variable, and then enters a loop that performs multiple calls to other routines while incrementing a counter and modifying a pointer.
- `process_scaled_tile_data_2`: Shifts d0 right by 8 bits, and if the result is non-zero, calls a routine, calculates an offset into a table using d0, checks a byte at that offset, writes d0 to a global variable, sets two global variables, performs a routine call with d0, calls another routine, and finally calls three more routines.
- `calculate_and_blit_area`: Checks a value in d0, calls another routine to retrieve a structure, calculates an offset using multiplication, and then executes a loop of Blitter transfers via another routine before calling a final routine.
- `process_scaled_data_and_callbacks`: Shifts d0 right by eight, and if the result is not zero, stores it in a global variable, calls a routine, calculates an offset into a table, performs arithmetic, calls three routines, and then calls another routine after checking a global byte and a byte at a pointer.
- `execute_callback_function`: Calls a function via a pointer and returns.
- `process_nested_tile_data`: Checks if the high byte of a parameter is non-zero, then calls another routine to retrieve a pointer, performs arithmetic involving multiplications by 39 and 7 to calculate offsets into a table, calls a routine to process a range, and finally calls another routine before resetting two global 16-bit values.
- `modify_control_flags`: Sets d0 to 255, compares a byte at a global address to 2, and if it is not less than 2, checks a byte at a global address; if that byte is non-zero, it clears bits 4 and 5 in d0 before branching.
- `modify_global_flags`: Checks bits in a global variable to clear specific bits in register d0, then calls several routines to update a global word and perform scaled blitting operations.
- `call_conditional_routine_3`: Loads 224 into d0, 125 into d1, and 30 into d3, calls a routine, and if the result is not zero, calls another routine before returning.
- `toggle_global_bit_field`: Checks if the high byte of d0 is non-zero and if a bit is set in a global byte, then retrieves a byte from a table, performs a bitwise change on a global word, and calls two other routines.
- `validate_and_process_tile_index`: Checks if a high byte in a register is non-zero and if a corresponding bit is set in a global variable, then performs arithmetic on a value, updates two global words, and calls three different routines.
- `check_bit_and_load_pointer`: Tests a specific bit in a register; if the bit is set, it loads a pointer from a table into a register and branches to another routine, otherwise it returns.
- `reset_global_value`: Loads a pointer from a table into a0, calls another routine with the constant 232 in d0, and then sets a global variable to zero before returning.
- `loop_and_set_global`: Saves registers, calls another routine, and then enters a loop that checks bits in a byte from a global variable to set a global word to either 28 or 31.
- `loop_and_test_bits`: Writes 31 to a global word, loads a1 into a0, sets d0 to 65, calls a routine, updates a1 with a0, loads a byte from a2 into d0, clears bit 7 of d0, and performs conditional branches based on d0 and bit 2 of d2.
- `test_bit_and_load_pointer`: Tests a bit in a longword using a register and, if the bit is set, loads a pointer into a0 from a global variable and branches to another routine; otherwise, it branches to a different location.
- `call_and_loop_counter`: Loads a pointer into a0, calls another routine, and then enters a loop that increments d1 and d7, returning when d7 reaches 6.
- `update_data_from_buffer`: Checks a byte at offset 2 of a buffer; if the lower two bits are not 2 or 3, it stores a byte from the buffer into a global word, then optionally calls another routine twice to store results into global bytes at offsets 34 and 35.
- `store_dual_values`: Copies the value from d2 into a global variable via a routine call, then repeats the process with a different constant in d1 to store a second value in another global variable.
- `call_two_routines_2`: Pushes a longword from the stack, calls two different routines using bytes from global variables, and then restores the stack.
- `process_and_clear_buffer`: Retrieves a pointer via a routine, then enters a loop that clears 70 consecutive bytes starting at that pointer.
- `execute_multiple_subroutines_5`: Sets three global words to zero, calls two routines, and if a byte in a global variable is zero, performs a sequence of calls and pointer retrievals.
- `set_parameter_and_call_fixed_routine`: Loads the value 119 into register d0 and calls a routine at a fixed address before returning.
- `execute_conditional_blits_2`: Loads a byte from a global variable, calls two routines, and if two specific global bytes are both zero, calls three more routines using bytes from three different global locations before jumping to another routine.
- `dispatch_subroutine_and_jump`: Calls a subroutine via a jump to subroutine instruction and then performs a long jump to another address.
- `execute_conditional_sequence_4`: Calls two routines and then jumps to a third routine if a global pointer and a value at offset 38 of its structure are both non-zero.
- `update_sprite_and_sync_state`: Retrieves a structure via a pointer, updates two global words with values from offsets 24 and 26 of that structure, and performs a series of calculations and calls to update sprite or object properties.
- `test_and_dispatch_list`: Tests a byte at a global address, and if it is not zero, sets the sign bit of that byte, calls three other routines, and jumps to another routine.
- `update_structure_flags_4`: Calls two different routines, sets various bits in a structure based on the results, and then performs several conditional bit sets in the structure based on values retrieved from a table and global variables.
- `process_event_callback_chain`: Loads a byte from a global structure, calls a routine with a constant in d7, and if the result is zero, checks a byte in another global structure to decide whether to call a sequence of three routines or a different sequence of three routines based on the status of two additional bytes in the global structure.
- `dispatch_conditional_callbacks`: Calls four routines in sequence, tests a byte at a memory location, and if it is zero, compares a word from a memory location to negative 1, calling another routine if the word is not negative 1.
- `dispatch_conditional_call_2`: Calls a routine and, if the result is not negative, sets d0 to 107 and calls another routine.
- `initialize_and_dispatch_event`: Writes several values to a structure pointed to by d6, calls two routines, and then conditionally calls a third routine if a value in d0 is not negative, followed by a check of a byte in a global variable to decide whether to call a final routine.
- `execute_multiple_subroutines_3`: Loads the word 114 into d0 and calls five different subroutines in sequence before returning.
- `initialize_event_structure_3`: Sets several global values, clears a word at offset 24 of a structure in a0, calls two routines, and performs conditional logic based on the result of a routine call and a global byte.
- `load_constant_and_call_subroutine`: Loads the word value 114 into register d0, calls a subroutine at a fixed address, and then branches to another location.
- `call_subroutines_conditionally`: Tests a byte at a global address and, if it is zero, calls four subroutines in sequence; otherwise, it calls four subroutines after branching.
- `dispatch_event_data_2`: Pushes a value to the stack, retrieves a byte from a global variable to use as an index for a routine call, and then uses the resulting pointer to check a bitmask.
- `test_bit_and_call_handler`: Tests bit 1 of a register; if set, it loads the value 99 into a register, calls another routine, and then branches to a subsequent instruction; otherwise it returns.
- `dispatch_event_by_bit_test`: Tests bit 10 of a register; if set, it loads the value 100 into a register, calls another routine, and then branches to a subsequent instruction; otherwise, it branches to a subsequent instruction.
- `dispatch_conditional_event`: Tests a word at a specific memory location and, if non-zero, loads the value 97 into register d0 and calls a routine at a fixed address before branching to another location; otherwise, it branches to another location.
- `dispatch_event_sequence`: Calls a routine, and if the result is positive, calls another routine with a constant in d0.
- `compare_and_call_on_threshold`: Compares a word at a specific memory location with 6; if the value is 6 or greater, it loads 121 into a register and calls another routine, otherwise it branches to another location.
- `fetch_and_test_pointer_byte`: Calls a routine to retrieve a pointer, fetches a byte from that pointer, and then calls another routine to check that byte; if the byte is non-zero, it calls a third routine with the value 115 in d0.
- `test_and_store_byte`: Loads the value 5 into d0, calls a routine, and if the result is positive, stores a byte from a pointer in d6 into a global variable before calling another routine.
- `update_event_state`: Calls a routine, stores a pointer from the result into a global variable, writes 110 into a memory location, sets a byte at a memory location to 1, clears a longword at a memory location, calls a routine, and performs several conditional checks and calls based on the state of a memory location.
- `update_event_state_variant`: Calls a routine, stores a pointer from the result into a global variable, writes 111 into a memory location, sets a byte at a memory location to 2, clears a longword at a memory location, calls a routine, and performs several conditional checks and calls based on the state of a memory location.
- `fetch_table_entry`: Loads a byte from a table using a value from a register shifted left by two, then returns that value as a longword.
- `scale_and_copy_data`: Saves registers, calculates an offset into a table using d2, copies two words from that table into global variables, calls two routines, calculates a new offset using multiplication, copies a byte from a memory location into a structure, and calls three more routines.
- `copy_and_process_data`: Copies two words from a source address to a destination address, calls three subroutines, and uses d0 and d1 to pass values to a subroutine using a global address in a0.
- `process_scaled_coordinates`: Saves registers, loads an address into a0, loads two words from memory into d0 and d1, sets d2 to 10, and calls three routines before returning.
- `loop_and_test_byte`: Performs a loop that runs up to six times, testing a byte at a pointer in a0 and incrementing d6 if the byte is not zero, then branches to another routine.
- `update_graphics_and_table`: Stores a byte at a negative offset of a pointer in a0, calculates an offset into a table using a global word and a value in d0, calls a routine, and then enters a loop that sets bits in a bitmask based on bytes read from a table.
- `update_graphics_structure_3`: Calculates a value based on d7, stores it in a local stack area and a global word, calls a routine, and then enters a loop that copies bytes from a table to a global location, performing various routine calls and updates to a global word and a structure byte.
- `conditionally_call_callback`: Tests a byte in a global variable; if it is non-zero, it calls another routine, otherwise it returns.
- `call_initialize_structure_field`: Calls initialize_structure_field and then returns.
- `setup_structure_and_call`: Pushes a0 to the stack, copies a long value from a global address to another global address, clears a global long, writes a constant to a global long, loads a pointer into a0, writes a word to the address in a0, clears a word at offset 18 of the structure in a0, calls another routine, restores a0 from the stack, and returns.
- `execute_callback_sequence`: Calls three separate routines in sequence, the second of which executes a loop of eleven calls to a third routine before the final routine is called.
- `initialize_and_update_tile_metadata`: Sets four 16-bit values in a table, calls several routines to retrieve a structure, and updates two global words with values from offsets 24 and 26 of that structure.
- `calculate_and_update_tile_data`: Calculates a value based on a structure offset and a multiplication by 6, then updates two global words with values from offsets 24 and 26 of a structure.
- `parse_blit_metadata_stream`: Calculates and stores several word-sized offsets and counts in global variables based on values read from a data stream, then iterates through the stream to perform table lookups and conditional calls to update structure values.
- `validate_and_lookup_blit_metadata`: Checks if a value in d2 is within bounds, then uses it to index into a table to retrieve a new value in d2 via a call.
- `process_blit_data_stream_metadata`: Iterates through a data stream to extract and validate metadata, performing table lookups and conditional calls to modify structure values based on bitmask checks and specific byte values.
- `execute_blit_metadata_subroutines`: Calls three consecutive subroutines and then branches to another location in the code.
- `check_bits_and_set_d2`: Loads a byte from offset 2 of a structure in a0, masks it with 3, and compares it to 2.
- `process_blit_data_stream`: Checks a bit in a global variable and, if set, executes a loop that iterates up to 32 times, calling another routine to process data and updating a pointer in a0 by an offset.
- `process_loop_with_callbacks`: Compares a global word to 6, otherwise enters a loop that calls another routine and checks its result, performs a second routine call with specific values in d0, d1, and d2, and then clears two global words before jumping to another routine.
- `perform_sequential_updates`: Loads three bytes from consecutive global addresses into d0, calls a routine for each byte, calls another routine, and then jumps to another routine.
- `execute_callback_sequence_3`: Loads the value 11 into a register and calls three consecutive subroutines before jumping to a target address.
- `process_table_and_jump`: Calls two routines, checks if a global word is greater than or equal to 250, clears a global word if so, increments a global word by 4, and if a specific table entry is zero, resets that global word to 0 before calling two more routines and jumping to another routine.
- `increment_global_word_2`: Increments a 16-bit word in a global variable by one and returns.
- `calculate_table_indices`: Performs arithmetic on two global words to calculate new indices, checks if these indices fall within a specific range defined by two other global words, and if so, stores them in a global table and calls another routine.
- `call_conditional_routine_7`: Calls a routine, moves d0 to d7, tests a byte at a global address, and if it is zero, sets d0 to 51 and calls another routine before branching to another routine.
- `frag_tst_beq_move_jsr_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then copies 137 to d0, then calls execute_indirect_subroutine, then jumps to another point in the code.
- `update_table_values_7`: Calls a routine, and if the result is non-zero, retrieves three bytes from a table at an offset calculated from a pointer in d6, stores them in a global structure, calls another routine, and then performs arithmetic on the difference of the stored values to either call another routine or enter a loop that calls a routine multiple times.
- `divide_counter_by_four`: Extends a 16-bit value in a register to a 32-bit value, divides it by four using an unsigned division, and branches to another routine.
- `execute_looping_calls`: Decrements a counter in a register in a loop, calling two different routines during each iteration before returning.
- `execute_conditional_callback_6`: Calls a subroutine, checks if the returned 16-bit value is negative, and if so, calls another subroutine before branching to a fixed location; otherwise, it calls a second subroutine and branches to a fixed location.
- `conditionally_call_subroutine`: Compares a 16-bit value in a register with a value from a global variable and, if they are not equal, calls a subroutine and then branches to another routine; otherwise, it branches to another routine.
- `update_tile_offset`: Calls a routine, stores the result in a global variable, pushes d0 to the stack, loads a byte from a global address, calls another routine, calculates an offset by multiplying d0 by 20 and adding it to a global pointer, stores the constant 27 in a global variable, and calls a final routine.
- `set_callback_index_and_branch`: Sets register d0 to 1 and branches to another routine.
- `set_callback_index_and_branch_2`: Sets register d0 to 3 and branches to another routine.
- `prepare_callback_index_dispatch`: Loads the value 5 into register d0 and branches to dispatch_indexed_callbacks.
- `set_callback_count_and_branch`: Sets register d0 to 7 and branches to the next instruction.
- `dispatch_indexed_callbacks`: If d0 is non-zero, it masks d0, decrements it, and uses the result to index into a table of pointers to call a routine; if bit 0 of d0 is set, it uses a second table to call another routine, and finally jumps to another routine.
- `decrement_global_word_conditionally`: Decrements a 16-bit global variable by one if a byte in a global variable is non-zero and the 16-bit variable itself is non-zero.
- `increment_global_word_range`: Increments a 16-bit global variable by two if a byte in a global variable is non-zero and the variable is less than the difference between another 16-bit global variable and nineteen.
- `increment_global_word_limit`: Increments a 16-bit global variable by one if a byte in a global variable is non-zero and the variable is less than the difference between another 16-bit global variable and thirteen.
- `decrement_global_word_two`: Decrements a 16-bit global variable by two if a byte in a global variable is non-zero and the 16-bit variable is non-zero.
- `execute_callback_chain`: Calls a routine to check global bits, calls a routine to process a list and callbacks, and then calls a routine that executes a function pointer located at offset 30 of a structure pointed to by a global variable.
- `search_table_for_match_2`: Checks if a global word is non-zero, calculates two values based on global words and offsets, and if those values fall within a specific range, enters a loop to search a table for a match against those values.
- `modify_table_entry_2`: Compares bytes at offsets 0 and 1 of a structure in a0 with d4 and d5, and if they match and a global variable is not 6, calls two routines to modify data.
- `process_table_entry_3`: Increments a value in d6, performs a loop based on d7, pushes d0 to the stack, calls a routine using a pointer from a global variable, restores d0, sets d6 to 255, and restores registers before returning.
- `traverse_linked_list_5`: If d2 is 9, it calls a routine using a pointer from a global variable, then enters a loop that retrieves a word from a table using an offset derived from d3 and a global pointer, calculates a new address by multiplying a value by 12, and continues the loop if a byte at that address is not 2 or if a word is not 65535.
- `call_modify_routine`: Calls a routine with specific values in d1 and d2, and if the result is non-zero, branches to another location; otherwise it branches to another location.
- `call_decrement_routine_short`: Pushes a word from d0 onto the stack, retrieves a byte from a global address, and calls another routine.
- `traverse_offset_table`: If d2 is 13, it calls another routine to retrieve a byte from a global address, calculates an offset using a value from a table and a multiplication, and enters a loop to check structure members and sum values.
- `validate_and_sum_offsets`: Checks a byte at offset 2 of a structure in a1 and a global byte; if both conditions are met, it retrieves a pointer, sums two 16-bit words at offsets 178 and 182 from that pointer, and calls another routine with the result.
- `modify_table_bit`: Loads a byte from a structure in a1, calls another routine, and if the byte is not zero, calls a second routine to modify a bit in a table.
- `call_decrement_routine_long`: Pushes a word from d0 onto the stack, retrieves a byte from a global address, and calls another routine.
- `prepare_scaled_blit_parameters_3`: Calculates dimensions and offsets for scaled graphics transfers by manipulating values from a table and calls a routine to prepare specific memory regions.
- `process_blit_loops`: Calls a routine, increments a global word, and enters a loop that decrements d2 and calls a routine if a byte in a buffer is non-zero, then performs further calculations and loops using values from a global variable and a pointer in a1.
- `perform_blit_operations`: Calls a subroutine, increments a global word, and enters a loop that decrements d2 and calls a subroutine if a byte in a buffer is non-zero; it then performs further operations involving a pointer in a1 and a global word.
- `execute_blit_loop`: Increments a global word, performs a loop that calls a routine, and then enters a nested loop that calls a routine if a byte in a buffer is non-zero.
- `execute_scaled_blit_sequence_3`: Enters a loop that increments a global word and calls a routine, then performs a series of scaled blit operations using various global pointers and tables, including a nested loop that checks a global word against a table value before calling a routine.
- `perform_scaled_blit_loop`: Calculates a destination address using division and multiplication, executes a scaled blit, and then enters a nested loop to increment global counters and call routines.
- `calculate_blit_offset_5`: Retrieves two bytes from a table, performs a conditional check, and if certain bits are set, calculates a destination address and calls a scaled blit routine.
- `calculate_blit_offset`: Calculates an offset by masking d3, multiplying it by 192, and adding it to a0 before calling another routine.
- `execute_complex_blit_sequence_4`: Saves registers, retrieves a byte from a global address, calls several subroutines with various parameters to perform scaled operations, and conditionally executes further subroutines based on bits in a global word.
- `execute_blit_and_modify`: Calculates a destination address through division and multiplication, calls a scaled blit routine, and then calls another routine to modify a value from a global pointer.
- `check_blit_parameters_2`: Saves registers, calls a routine to retrieve a pointer, sets up several registers with constants, and then checks a byte from a structure to decide whether to branch to different locations.
- `validate_blit_scaling_flags`: Compares a value in register d0 to 8 and, if not equal, checks bit 1 of a byte at a memory location pointed to by a register; if that bit is set, it branches to another routine, otherwise it branches to process_scaled_blit_parameters.
- `process_scaled_blit_parameters`: Checks bits in d0 and a bit in a table, then calculates a destination address using a multiplication, calls a routine, and performs a loop that calls another routine and a third routine while updating d0 and d1.
- `call_decrement_routine_2`: Pushes d0 to the stack, fetches a byte from a global address into d0, and calls another routine.
- `perform_complex_bitmasking`: Checks d2 against several bounds, and if valid, executes a loop that performs multiple routine calls, bitwise AND operations between a global byte and a table value, and writes results to a calculated address.
- `fetch_and_store_byte_2`: Calls another routine using a value from a global pointer and a constant in d1, stores the resulting byte in a global variable, and sets the sign bit of a global variable.
- `execute_two_routines_4`: Pushes d0 to the stack, calls a routine, loads a byte from a global pointer into d0, calls another routine, and returns.
- `process_structure_data_8`: Checks a global byte, and if non-zero, calls two routines using values from a structure, then updates the global byte.
- `set_buffer_bits`: Checks a global byte, calls another routine to retrieve a pointer, and then executes a loop to add values from two tables to d0 and d1 before calling two more routines.
- `set_bits_in_buffer`: Performs two nested loops to call a routine for bit manipulation and then calls another routine to clear a byte from a global address.
- `check_blit_metadata_type`: Compares a 16-bit value at a specific memory location to the constant 6; if they match, it sets a register to 255 and branches elsewhere, otherwise it returns.
- `calculate_and_test_bit`: Calculates an index using a multiplication, retrieves a pointer, calls another routine, and then tests a bit in a byte retrieved from the pointer's offset.
- `set_bit_in_buffer`: Calculates a bit position and index within a buffer based on two input values and two global width/height values, then sets that bit in the buffer and updates a global variable if the bit was not already set.
- `calculate_and_store_data`: Clears a global word, calls another routine to get a pointer, loads a word from a structure at an offset, calculates a pointer into a table using a global word and multiplication, and enters a loop that tests bits and writes a four-byte sequence to a destination based on several conditional checks.
- `test_bit_and_branch_2`: Loads a word from a global address into d4, adds 5 to it, tests bit 4 of d0, and if bit 4 is not set, sets d0 to 2 and branches to another location; otherwise it returns.
- `compare_and_branch_byte`: Loads a byte from offset 1 of the structure in a1 into d0, and if d0 is zero, branches to another location; otherwise, it compares d0 with d6 and branches to another location if d0 is negative, or sets d0 to 3 and branches to another location.
- `calculate_and_fill_data`: Divides d6 by d0, compares the result to 100, and if it is not less than 100, enters a loop that writes four bytes to a destination pointed to by a0, increments pointers, and conditionally calls two routines if a loop counter reaches 31.
- `call_update_scaled_parameters`: Calls another routine and then returns immediately.
- `update_scaled_parameters`: Sets register values and calls two different routines, retrieves a word from a table using a value from a global variable and an offset, stores that word at offset 18 of a structure in a0, calls another routine, and jumps to another routine.
- `process_list_element`: If a global word is non-zero, it calls another routine, loads a word from a global variable into d1, adds d1 to itself, calls another routine, stores the result from d0 into a memory location at offset -2 from a pointer in a0 plus d1, and calls a final routine before returning.
- `process_blit_sequence`: Loads several constants into d0 through d4 and calls another routine, then jumps to another routine.
- `process_data_sequence`: Loads several constants into registers, calls another routine, calculates a difference between two global values, and if the difference is greater than 16, sets a register to 16, loads two addresses into a1 and a2, shifts d0 left by 2, adds the result to a1, and branches to another routine.
- `set_global_flag_conditionally`: Checks if the byte pointed to by a1 is zero; if it is not zero, it writes the value 31 to a global word and otherwise returns.
- `write_scaled_data_sequence`: Writes a constant to a global word, performs a series of calls to process d0, and enters a loop that writes a sequence of bytes and a zero byte to a buffer at a0 based on a counter in d2.
- `check_and_store_tile_byte`: Shifts d0 right by 8 bits, and if non-zero, checks if a calculated address is within bounds and points to a zero byte, then stores a byte from that address into a global variable, performs a routine call, sets two global variables, performs a routine call with a calculated value, and calls three more routines.
- `setup_clipping_parameters`: Checks if the high byte of d0 is non-zero and within a specific range, then calculates an address in a table, and if the byte at that address is non-zero, sets several registers and calls another routine.
- `check_byte_and_load_constant`: Compares a byte at the address in a0 to 1 and branches if not equal; otherwise, it loads 27 into register d3 and branches to another routine.
- `load_constant_and_call_routine`: Loads the constant 241 into register d3, calls another routine, and then performs a short branch.
- `setup_blit_parameters_2`: Subtracts one from a byte at offset 3 of a structure in a0, calls another routine, sets two words at a global address to 0 and 65535, calculates new values based on d7 and d1, calls another routine, and finally sets a global word to 0.
- `calculate_index_and_call_2`: Saves registers, calculates an index into a table using two global words, retrieves a byte from that table, and if bit 7 is clear and bit 6 is set, performs a series of operations involving setting global words, calling a routine, and potentially calling another routine with a shifted d0 value.
- `update_status_and_call_2`: Shifts d1 left by 8 bits and stores it in d0, then calls a routine, stores a byte from a pointer in a1 into a global variable, sets a global word to a constant, loads a pointer from a global address into a0, calls another routine, and if d0 is not negative, stores d0 in a global variable and stores two words from a global address into two global variables before branching to another location.
- `test_and_set_global`: Tests bit 0 of d0 and, if set, checks a byte at a global address; if that byte is non-zero, it writes the word 284 to a global address and branches.
- `set_event_parameters`: Stores two values into global memory, calls a routine, and then calculates a value using a bit shift and stores it in a global word.
- `execute_indirect_calls_2`: Stores a value in a global variable, loads a pointer from a table using an offset in d0, and if the pointer is non-zero, calls it; then tests a global byte and calls another routine if non-zero.
- `fetch_byte_from_table_conditionally`: Checks two global bytes for zero; if both are non-zero, it loads a pointer from a global address, retrieves a byte from an offset of 27 from that pointer into a register, and branches to another routine; otherwise, it branches to one of two other locations.
- `execute_conditional_parameter_call`: Loads a word from a global variable into register d0, calls another routine, sets the carry flag based on the result of a byte check in a global variable, and branches to another location.
- `calculate_and_update_structure_3`: Calculates an offset into a global table using values from two global words, retrieves a byte from that table, and calls another routine.
- `call_with_parameter`: Loads the word 248 into d0, calls another routine, restores registers from the stack, and checks a byte in a global variable before returning.
- `execute_callback_loop`: Iterates through a list, checking a bitmask against a global value, and if a bit is set, calls a routine, executes a function pointer from a table, and calls another routine.
- `validate_graphics_and_decrement`: Saves registers, calls a routine to retrieve a pointer, calculates an offset into a structure, checks a bit in a table, and if a byte at offset 1 of a structure is not 255, decrements it before calling two more routines and performing several comparisons.
- `validate_and_execute_conditional_logic`: Fetches a byte from a global pointer, calls two routines, and checks if the byte equals five and if two values in a table are equal.
- `update_graphics_parameters_2`: Subtracts 40 from d0, subtracts 9 from d0, calls two routines, moves d2 into d0, calls a third routine, and restores registers from the stack before returning.
- `call_handler_and_execute_callback`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0, calls another routine, and then calls a third routine before returning.
- `check_structure_condition`: Checks if bit 14 and 15 are set in a word at offset 30 of a structure in a0, and if so, retrieves a pointer from a global variable, fetches a byte from a table, and if that byte is non-negative, performs a division and calls two other routines.
- `call_handler_and_execute_callback_2`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0, calls another routine, and then calls a third routine before returning.
- `clear_bit_in_word`: Checks if specific bits are set in a word at offset 30 of a structure; if not, it retrieves a byte from a global pointer and clears the corresponding bit in the word.
- `dispatch_conditional_callback_7`: Calls a routine via a pointer, then branches based on the result, otherwise loads a pointer into a0 and performs an unconditional branch.
- `call_two_routines_6`: Loads a fixed address into a0, then calls execute_callback_loop and modify_status_bit in sequence before returning.
- `test_bits_and_call`: Loads a word from offset 30 of the structure in a0 into d0, tests bit 15 of d0, and if bit 15 is set, branches to another location; otherwise it tests bit 14 and if bit 14 is clear, sets d0 to 250 and calls a routine before branching to another location.
- `test_bit_and_call_subroutine`: Tests bit 13 of a word loaded from a structure offset, branches to another routine if the bit is set, otherwise loads the value 249 into d0, calls another routine, and branches to a final destination.
- `set_structure_flags_3`: Clears bit 13 of d0, stores d0 at offset 30 of the structure in a0, stores the word 1 at offset 202 of the structure in a0, calls a routine, loads 57 into d0, calls another routine, and then calls a third routine using d7 in d0.
- `modify_status_bit`: Loads a word from offset 30 of the structure in a0, checks bit 13, and if it is set, clears bit 13, sets bit 14, stores the result back to offset 30, and calls a routine.
- `dispatch_conditional_callback_4`: Calls a routine, then conditionally branches based on the result, otherwise loads a pointer into a0 and performs an unconditional branch.
- `call_callback_function`: Loads a pointer from a table into a0 and calls the routine pointed to by that address.
- `modify_word_and_call`: Loads a word from offset 30 of the structure in a0 into d0, tests bit 14 of d0, and if it is set, branches to another location; otherwise, it sets d0 to a constant, calls a routine, and branches to another location.
- `set_status_bits_5`: Clears bit 14 of d0, sets bit 13 of d0, stores the word in d0 at offset 30 of the structure in a0, and calls a routine.
- `modify_status_bit_2`: Checks bit 14 of a word at offset 30 of a structure in a0; if set, it clears bit 14 and sets bit 15 of that word before calling another routine and returning.
- `call_handler_and_execute_callback_alt`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0 and calls another routine before returning.
- `modify_word_and_call_alt`: Loads a word from offset 30 of the structure in a0 into d0, tests bit 15 of d0, and if it is set, branches to another location; otherwise, it sets d0 to a constant, calls a routine, and branches to another location.
- `clear_and_set_bits`: Clears bit 15 and sets bit 14 of a word at offset 30 of a structure in a0, then calls another routine and returns.
- `dispatch_conditional_callbacks_3`: Calls a routine, then conditionally branches to another routine based on the previous result, otherwise loads a pointer into a0 and branches to a third routine.
- `call_two_routines_7`: Loads a fixed address into a0, then calls save_registers_and_branch and execute_callback_loop in sequence before returning.
- `check_bits_and_call_3`: Loads a word from offset 30 of the structure in a0 into d0, performs a bitwise AND with a constant in d1, and if the result is zero, branches to another location; otherwise, it tests bit 15 of d0, and if set, branches to another location; then it tests bit 14 of d0, and if not set, it sets d0 to a constant, calls a routine, and branches to another location.
- `update_structure_values_17`: Clears bit 13 of a word at offset 30 of a structure in a0, adds two words from offsets 204 and 206 of the structure to a third word at offset 202, then calls three different routines.
- `modify_status_bit_3`: Loads a word from offset 30 of the structure in a0, checks bit 13, and if it is set, clears bit 13, sets bit 14, stores the result back to offset 30, and calls a routine.
- `call_handler_and_execute_callback_4`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0, calls fetch_byte_from_table_conditionally, and then calls execute_callback_loop before returning.
- `update_structure_on_bit_set`: Checks if a word at offset 30 of a structure has specific bits cleared and bit 6 set; if so, it calls another routine twice and then calls a third routine.
- `dispatch_conditional_table_processing`: Calls a routine to conditionally fetch a byte from a table, then branches based on the result to either call another routine or jump to a different handler.
- `process_pointer_table_data`: Retrieves a byte from a table using a pointer and index, calls another routine with that byte, and then calculates an offset into a structure to perform arithmetic and call two more routines.
- `compare_and_call_8`: Subtracts a byte from offset 1 of a structure in a0 from a byte at offset 28 of a structure in a1; if the result is positive, it pushes d0 to the stack, calls another routine, restores d0, sets d0 to 257, and calls another routine.
- `calculate_and_update_byte`: Calls two subroutines, compares a byte from a structure in a2 to d0, and if d0 is not positive, performs a multiplication, swaps the result, and adds it to a byte at offset 1 of a structure in a0.
- `call_handler_and_update_table`: Calls a routine, and if the zero flag is not set, subtracts two from a word loaded from a global variable, uses the result to index into a table to load two bytes into d1 and d0, and then calls two routines before returning.
- `frag_jsr_bne_lea_rts`: Short code fragment that calls execute_callback_loop, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then calls fetch_byte_from_table_conditionally, then calls increment_structure_values, then returns.
- `increment_structure_values`: Checks if a word at offset 30 of a structure masked with 58496 is zero and if a word at offset 26 is not 32767; if both conditions are met, it increments the word at offset 26, adds 250 to a longword at offset 270, and calls another routine.
- `call_handler_and_execute_callback_alt_2`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0 and calls another routine before returning.
- `process_graphics_list_2`: Checks bits 13 and 14 of a word at offset 30 of a structure in a0, and if they are clear, performs a sequence of routine calls, manipulates a global byte, and enters a loop that iterates up to 10 times to process structure data and call routines.
- `process_attribute_updates`: Calls a routine and, if the zero flag is set, performs several operations including loading values from a structure, calling two different routines, and conditionally updating two words in a structure based on bit tests.
- `trigger_attribute_update_event`: Loads the value 66 into register d0 and calls an indirect subroutine, then returns.
- `update_structure_pointers_9`: Calls a routine and, if the zero flag is set, retrieves a pointer from a table, uses it to write three words into a structure at offset 36, and then calls two more routines.
- `sync_structure_data`: Calls a routine and, if the zero flag is set, retrieves a pointer from a table, copies three words from a structure into registers, calls another routine, and then conditionally updates three words in a structure if a specific register value matches a global word.
- `check_and_store_results`: Loads 260 into d0 and calls another routine; if the resulting value in d3 matches a longword at a global address, it stores d1 and d2 into two consecutive global words and calls another routine.
- `store_subroutine_results`: Calls a subroutine and stores the resulting values from d1, d2, and d3 into three consecutive global words.
- `update_table_value_two`: Calls a routine and, if the zero flag is set, calculates an index from a global word, retrieves two bytes from a table using that index, and calls another routine with the second byte in d7 set to two.
- `update_table_value_three`: Calls a routine and, if the zero flag is set, calculates an index from a global word, retrieves two bytes from a table using that index, and calls another routine with the second byte in d7 set to three.
- `execute_conditional_loop`: Calls another routine, and if the zero flag is not set, performs a series of operations including calling another routine, loading a byte from a global address, and executing a loop that updates a global word and calls three different routines.
- `update_global_status_2`: Compares two global words against zero, checks if a byte at a global address matches specific values, and if so, stores two global words and calls another routine.
- `execute_conditional_calls_15`: Sets d0 to 64, calls a routine, compares a byte at a global address to 2, and if it is not less than 2, calls another routine, moves a word from a global address to d0, sets d1 to 2, and branches to another routine.
- `execute_looping_calls_6`: Stores a word from d0 into a global variable, calls three different routines in a loop, decrements d1 and d7, and then calls another routine before returning.
- `update_table_value_four`: Calls a routine and, if the zero flag is set, calculates an index from a global word, retrieves two bytes from a table using that index, and calls another routine with the second byte in d7 set to four.
- `execute_multiple_updates`: Calls a routine and, if the zero flag is set, performs four separate calls to another routine with varying values in d0 and d7, then calls a final routine before returning.
- `dispatch_conditional_table_logic`: Calls a routine to conditionally fetch a byte from a table, then either branches or calls another routine before jumping to a final destination.
- `process_table_and_bit`: Retrieves a byte from a table using an index in d7 and a pointer in a0, calls another routine, calculates an offset by multiplying a word from a global address by 6 and adding it to a new pointer, then checks bit 1 of a byte at that offset and calls another routine if the bit is clear.
- `clear_bit_and_call_subroutines`: Clears bit 1 of a byte at offset 3 of the structure in a0, then calls two subroutines using a global address and a value from a global address.
- `execute_conditional_subroutine_chain`: Calls a subroutine, then conditionally calls another subroutine based on a zero flag, before finally calling a third subroutine and branching to a subsequent location.
- `update_structure_and_call_3`: Retrieves a byte from a table using an index from a global word, calls a routine, calculates an offset into a structure, checks a bit in a table, and if bit 6 is clear, calls another routine and then calls a routine with the value 269 in d0.
- `update_structure_pointers_7`: Copies a pointer and a word from a structure to another location, calls two routines, sets a byte to one, and then calls three more routines based on a conditional check.
- `set_parameter_and_call_subroutine`: Loads the value 432 into a register and calls another routine before returning.
- `update_structure_and_call_7`: If a condition is met, it retrieves a byte from a table using a value from a global structure, calls a routine with that byte, calculates an offset into a structure using a value from a global word multiplied by six, sets bit 0 of a byte at that offset, copies a pointer and a word to a new location, calls two more routines, and finally calls a routine with a pointer to a global address.
- `fetch_table_byte_and_call`: If a condition is met, it retrieves a byte from a table at an offset derived from a global word and then calls a routine with d7 set to five.
- `process_conditional_structure_data`: If a condition is met, it retrieves a byte from a global structure, calls a routine with that byte, then calls another routine with the value from a structure, and if bit 3 of that value is zero, calls a routine with the constant 440.
- `process_conditional_call`: Calls a routine, compares the result to five, and if equal, calls another routine before calling a third routine and returning.
- `call_handler_and_execute_callback_alt_3`: Calls process_event_list_2, and if the zero flag is not set, loads a pointer from a table into a0 and calls fetch_byte_from_table_conditionally before returning.
- `initialize_scaled_blit_parameters`: Sets d0 to 48, d1 to 64, d2 to 4, d3 to 4, and clears a0, then calls another routine; subsequently sets d0 to 64, d1 to 80, d4 to 24, d5 to 3, d6 to 2, and d7 to 32, loads a table address into a0, and calls another routine; if a byte at a global address is not 2, it loads a word from a global address into d3, increments it, multiplies it by 48, subtracts 13, and branches to another location.
- `calculate_scaled_offset_8`: Performs arithmetic on a global word in d3, calls a routine, sets two global variables, performs a routine call with a value derived from d3, clears a global variable, and calls another routine before jumping to another routine.
- `check_hardware_status_and_call_routine`: Compares the value of a hardware register to two and, if the result is not negative, loads the value two hundred fifty-eight into d0 and calls another routine; otherwise, it branches to another routine.
- `load_table_and_call_routines`: If a condition is met, it retrieves a byte from a table using an index derived from a global word, calls a routine, and then calls another routine with a global address loaded into a0.
- `fetch_table_byte_and_call_six`: If a condition is met, it retrieves a byte from a table using an index derived from a global word minus thirteen, and then calls a routine with d7 set to six.
- `call_two_routines_with_constants`: If a condition is met, it calls a routine with d1 set to 180 and d7 set to five, then calls another routine with d7 set to six.
- `test_flag_and_call_handler`: Tests a word at a specific memory location; if it is not zero, it loads the value 283 into a register and calls another routine, otherwise it branches to a different location.
- `check_mouse_button_and_call_handler`: Checks if the first byte of a long value at a specific address is equal to one; if not, it loads the value 244 into a register and calls another routine, otherwise it returns.
- `execute_conditional_sequence_3`: If a condition is met, it calls two routines with d0 set to 46, and if the result is two, it sets a global word to one, calls a routine, and calls another routine with d0 set to five.
- `frag_jsr_bne_lea_rts_2`: Short code fragment that calls calculate_and_verify_offset, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then calls execute_callback_loop, then calls fetch_byte_from_table_conditionally, then returns.
- `calculate_and_verify_offset`: Checks if bits 14, 15, and 16 of a word at offset 30 of a structure in a0 are zero; if so, it calls another routine, calculates a new value for a word at offset 102 of the structure based on a multiplication and subtraction, calls another routine, and if a global byte and a global word match specific values, calls two more routines.
- `call_handler_and_execute_callback_alt_4`: Calls process_event_list_2, and if the zero flag is not set, loads a pointer from a table into a0 and calls fetch_byte_from_table_conditionally before returning.
- `execute_multiple_blits`: Clears a 16-bit value in a global variable, then performs three separate calls with specific register parameters and finally jumps to another routine.
- `initialize_callback_parameters`: Writes a word to a global variable, sets several registers to constant values, calls another routine, and then conditionally sets d7 and d1 based on a value retrieved from a global pointer.
- `loop_and_draw`: Loads a pointer into a0, calls another routine, sets d0 to 36 and calls another routine, then enters a loop that increments d1 and d2 and decrements d7 until d7 reaches negative 1.
- `dispatch_list_processing_task`: Calls pop_and_process_list and then jumps to another routine.
- `test_and_call_routine_2`: Sets d0 and d1 to specific values, calls a routine twice, and if the result is non-zero, calls another routine before returning.
- `perform_scaled_operations`: Shifts d0 right by 8, checks if d0 is zero, adds a value from a global pointer to d0, stores d0 in a global word, performs several routine calls with various parameters, and finally calls another routine using d0 as an index.
- `execute_conditional_blit`: Checks if the high byte of d0 is non-zero and if adding a value from a global pointer to d0 results in a value less than 32, then performs a series of calculations, calls three routines, and sets two global words.
- `call_handler_and_execute_callback_5`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0, calls fetch_byte_from_table_conditionally, and then calls execute_callback_loop before returning.
- `process_structure_value`: Checks if bit 14 and 15 are set in a word at offset 30 of a structure in a0, retrieves a byte from a table using a value from a global pointer, and if a word at offset 210 of the structure is non-zero, performs a division and calls two other routines.
- `frag_jsr_bne_lea_rts_3`: Short code fragment that calls execute_callback_loop, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then calls fetch_byte_from_table_conditionally, then calls reset_and_calculate_sum, then returns.
- `reset_and_calculate_sum`: Clears a word at offset 30 of a structure, calculates the sum of two words at offsets 204 and 206 of that structure, stores the result at offset 202, and jumps to another routine.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `frag_jsr_bne_lea_rts_4`: Short code fragment that calls execute_callback_loop, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then calls fetch_byte_from_table_conditionally, then calls save_registers_and_branch, then returns.
- `update_table_entry`: Checks if a word at offset 30 of a structure masked with 58496 is zero; if so, it calls another routine, calculates a value based on the result, compares it to a word at offset 2 of a table, and updates the table entry if the value is smaller, then calls another routine.
- `validate_and_update_attribute_table`: Validates two 16-bit values against a table, checks if a byte at a specific offset matches two constant values, and if so, calls another routine with a constant value in d0.
- `process_table_and_loop`: Calls another routine, checks a byte in a global variable, and if it is at least 2, calls another routine, performs several stack operations, calls three different routines in a loop, and finally calls another routine and stores 255 in a global variable.
- `update_conditional_bytes`: Calls a routine, stores a byte at offset 13c of a global variable, and then calls another routine if the status flag is set.
- `frag_jsr_bne_lea_rts_5`: Short code fragment that calls execute_callback_loop, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then calls fetch_byte_from_table_conditionally, then calls save_registers_and_branch, then returns.
- `set_structure_bit_2`: Checks if a word at offset 30 of a structure masked with 58496 is zero; if so, it sets bit 5 of that word and calls another routine.
- `update_table_with_offsets`: If a global byte is zero, it calls two routines, calculates two offsets using global words and bit shifts, stores these and a byte into a structure, and then enters a loop of 24 iterations that increments d0 and d1 while storing a longword and calling a routine.
- `update_graphics_and_coordinates`: Retrieves a byte from a table, calls several routines to perform arithmetic and table lookups, and then calculates a pointer to a structure to copy two words into it before calling another routine.
- `update_table_values`: Compares two 8-bit values against a 16-bit value in a table at an index derived from a register, and if the comparison passes, writes the two 8-bit values into the table.
- `initialize_and_process_scaled_dimensions`: Sets two 16-bit values and clears four 16-bit values in global memory, then branches based on a byte value to either scale two 16-bit values by doubling them or copy them directly into a destination, before calling several routines and performing bitwise operations on global variables.
- `update_scaled_values_4`: Doubles two words from a global table, stores them in another location, calls a routine, and then performs four calls to another routine while passing bytes from a structure in a6.
- `calculate_scaled_offsets_3`: Stores two words from global memory into another location, calls three routines, and then calculates and stores two different values based on comparisons of the original words.
- `execute_callback_chain_2`: Loads a byte from a pointer in a longword, calls update_structure_flags_6, calls check_global_bits, and then jumps to the address stored in a third longword.
- `setup_pointer_swap`: Loads specific values into d0, d1, d2, and d3, then calls two routines and jumps to another routine.
- `update_clipping_and_counters`: Calls a routine with a pointer from a table, performs several calculations involving global values and bit tests, updates a counter in a global variable, and calls two more routines before returning.
- `dispatch_function_table`: Extracts an index from the high byte of a register, calls a routine from a table at that index, and if the returned value has bit zero set, calls a second routine from a different table at the same index.
- `decrement_global_word_two_conditionally`: Decrements a 16-bit global variable by two if a byte in a global variable is non-zero and the 16-bit variable is at least two.
- `decrement_global_word_two_conditionally_2`: Decrements a 16-bit global variable by two if a byte in a global variable is non-zero and the 16-bit variable is at least two.
- `increment_global_word_with_limit`: If a global byte is non-zero, it increments a 16-bit value in a global variable by 2, provided the new value is less than a value in another global variable minus 71.
- `increment_global_word_with_limit_alt`: If a global byte is non-zero, it increments a 16-bit value in a global variable by 2, provided the new value is less than a value in another global variable minus 55.
- `process_nested_tables_2`: Performs nested loops using values from two global pointers to fetch bytes from a structure, calculates offsets, and calls two different routines based on the byte values.
- `process_indexed_table_data`: Uses nested loops to fetch bytes from a structure, calculates an index using a global table, and calls another routine if the byte value is at least 101.
- `apply_rotated_mask`: Fetches a byte from a table using a value from a5, calculates a destination address using a global pointer and d6, calls another routine to get a pointer, and then enters a loop that performs bitwise OR operations on words at calculated offsets using a rotated byte value.
- `apply_mask_and_loop`: Performs a bitwise AND on a word at an offset in a structure using d1 and a word at an offset in a structure using d6, then enters a loop that decrements d7, calls another routine, and restores registers before returning.
- `apply_rotated_mask_with_rotation`: Fetches a byte from a table, calculates a destination address using a global pointer, calls another routine, and then enters a loop that performs bitwise OR operations on words at calculated offsets while rotating the source word.
- `rotate_and_mask_data`: Performs a bitwise AND on a word at an offset in a structure using d1, rotates d1, performs another bitwise AND on a word at an offset in a structure using the rotated d1, then enters a loop that decrements d7, calls another routine, and restores registers before returning.
- `calculate_and_store_result_2`: Pushes d0 to the stack, calculates a value using a word from a pointer at offset 596, performs several multiplications and additions, calls two different routines, stores a byte from d0 into a location at offset 193, and returns.
- `process_blit_sequence_3`: Copies eight words from one location to another, calls two routines with specific parameters, stores a byte from a register into a global variable, and then enters a loop to compare a value against a table before updating a global variable and jumping to another routine.
- `process_list_and_table`: Iterates through a list of bytes calling a routine for each, then retrieves a value from a table based on a global word and calls several other routines before jumping to a new address.
- `run_callback_sequence_2`: Performs two blit-related calls with specific parameters, then enters a loop that calls a routine multiple times while incrementing values from a table, and finally calls another routine and jumps to a third.
- `execute_callback_sequence_2`: Loads a pointer from a table, calls a routine, then calls another routine using values from the stack, performs a triple callback with specific constants, and finally calls a routine with a modified pointer.
- `clear_global_word_2`: Calls a routine, and if the return value is not zero and a word in a global variable is at least four, clears that word.
- `set_global_word_2`: Calls a routine, and if the return value is zero and a word in a global variable is less than four, sets that word to four.
- `divide_global_word`: Calls a routine, and if the return value is zero, divides a word in a global variable by four and updates it using a calculation involving the quotient and remainder.
- `increment_global_word_3`: Calls a routine, and if the return value is zero, divides a word in a global variable by four and updates it with a new value based on the quotient and a modulo-based offset.
- `process_table_callback`: Calls a routine, and if the return value is non-zero, calls another routine if a second routine returns a non-zero value.
- `execute_nested_calls_2`: Pushes a constant to the stack, calls a routine to get a pointer, uses that pointer to call another routine with an offset, and then calls a third routine after pushing and popping a byte from a global variable.
- `fetch_table_data`: Calculates an offset into a table by multiplying d0 by six, retrieves two words from that table, adds constants to them, and jumps to another routine.
- `sync_blit_parameters_3`: Clears the high bit of eight 16-bit words in a table and stores them in a global variable, then calls a routine via a pointer at offset 19 of the structure.
- `dispatch_subroutine_and_jump_2`: Calls a subroutine and then performs a long jump to another address.
- `dispatch_callback_and_update_pointer`: Calls check_global_bits and then process_list_and_callbacks, which updates a global pointer by subtracting 54 from its current value and conditionally decrements a global variable if bit 0 is set.
- `execute_event_callback_at_offset_30`: Calls a routine located at offset 30 of a structure pointed to by a global variable, then calls another routine and jumps to a third routine.
- `process_event_data`: Retrieves a word from a global variable, calls a routine, and stores the result in another global variable.
- `conditional_callback_execution`: Tests a byte at a global address, and if it is non-zero, calls another routine, stores a byte at a global address, calls another routine, and then jumps to another routine.
- `store_byte_and_jump`: Stores a byte from a register into a memory location and then jumps to another routine.
- `execute_multiple_calls_7`: Saves registers to the stack, calls four different routines in succession, and then loads a word from a global variable into d0 before branching.
- `loop_and_call_routines_3`: Stores a word from d0 into a global variable, enters a loop that calls another routine and decrements d0 until d0 reaches negative one, calls another routine, restores registers from the stack, and returns.
- `process_table_and_decrement`: Calls another routine, uses the result to calculate various offsets and values from a table, performs a loop 32 times to traverse data, updates several global longwords and words, and finally calls another routine or jumps to another routine.
- `copy_words_conditionally`: Copies two words from offsets 13572 and 13574 to offsets 2114 and 2116, then checks a byte at offset 1330; if that byte is zero, it copies two more words from offsets 1374 and 1376 to offsets 1372 and 1374.
- `copy_global_words`: Compares a byte at a global address to 1 and, if not equal, copies three words from one set of global variables to another set of global variables before branching.
- `copy_three_longwords`: Copies three consecutive longwords from a source location to a destination location in global memory and returns.
- `copy_words_conditionally_2`: Copies two words from offsets 13572 and 13574 to offsets 2114 and 2116, then checks a byte at offset 1330; if that byte is zero, it copies two more words from offsets 1374 and 1376 to offsets 1372 and 1374.
- `copy_global_words_2`: Compares a byte at a global address to 1 and, if not equal, copies three words from one set of global variables to another set of global variables before branching.
- `copy_three_longwords_2`: Copies three consecutive longwords from one global memory location to another global memory location and returns.
- `update_status_flag_3`: Calls another routine, checks a byte at a global address, and if it is non-zero, checks bit 6 of a word at a global address and sets d7 accordingly.
- `dispatch_entity_status_handlers`: Calls a routine to store a value in a stack and update global bytes and words, then calls another routine to iterate through a list of structures to find an entry matching two specific bytes.
- `process_loop_with_calls`: Pushes d0 to the stack, stores a byte at a global address, clears a 16-bit word at a global address, and enters a loop that calls a routine and conditionally updates d0 from a global word based on a byte check.
- `set_global_word_and_pop`: Sets a 16-bit global variable to 4 and pops a 32-bit value from the stack into register d0 before returning.
- `search_and_update_entity_list`: Calls another routine to retrieve a pointer, then iterates through a list of structures to find an entry matching two specific bytes.
- `validate_and_set_attribute`: Compares two bytes at the address in a0 with d0 and d1, and if they match, loads a byte from offset 3 of the structure into d0, then calls another routine twice with different parameters and calls a third routine to write the value 144 into d0.
- `update_entity_attributes_and_flags`: Iterates through a list of structures, performing attribute comparisons and bitwise updates based on global status flags and table lookups.
- `process_complex_table_logic`: Performs multiple conditional checks against global words and bytes, iterates through a table to perform calculations involving multiplication and division, and calls several routines to update global state and handle bitwise operations.
- `check_attribute_and_call`: Calls two routines, compares a global byte to one, and if it matches, retrieves a byte from a table and calls another routine if the byte is non-zero.
- `process_nested_structures`: Compares a global word to 6, then iterates through a list of structures, calling another routine for each non-zero byte, performing comparisons, and updating a global byte based on results from two different loops.
- `execute_nested_calls`: Pushes a word to the stack, retrieves a byte from the structure at offset -1 of a0 into d0, calls another routine, restores d0 from the stack, and then performs a loop using d7 before restoring registers and returning.
- `branch_on_status`: Moves d0 and d1 to d4 and d5, then checks a byte at a global address; if the byte is not 2 and not zero, it moves a byte from a global address into d7 and branches.
- `update_status_word_3`: Loads a word from a global variable into d2, compares d0 to 1, clears bit 0 of d2 if d0 is not positive, adds a word from a global variable to d4, and branches to another location.
- `update_status_bits_2`: Compares d0 to a global word and sets or clears bit 0 or bit 1 of d2 based on the comparison results and the sign of d1.
- `search_and_update_table_2`: Compares d1 to a global word, retrieves a byte from a table, calls another routine, calculates an offset into a structure using a global word, and then performs several lookups and calls to update values.
- `fetch_and_transform_byte`: Retrieves a byte from a structure via two nested table lookups, and if the byte is within a specific range, performs bitwise operations and shifts to transform d0.
- `compare_and_branch`: Subtracts 101 from d0, compares d0 to a value at a global address, and branches if d0 is less than that value; otherwise it clears d0 and branches to another location.
- `search_structure_table`: Calculates an offset into a table using d0, retrieves a pointer, and calls another routine twice with different bytes from a global pointer.
- `modify_pointer_and_test_bit`: Increments a value in d7, and if it reaches 32, checks if bit 5 of d2 is set; if it is, it moves d2 to d0 and otherwise branches.
- `check_bit_and_restore_registers`: If a long value in d1 is non-zero and its fifth bit is not set, it moves d1 to d0 and restores d1 through d7 and a0 from the stack.
- `copy_table_entries`: Saves registers, calls another routine to retrieve a pointer, retrieves a byte from offset 10 of that pointer, calls another routine, and stores the resulting byte into a global word.
- `check_and_load_graphics_pointer`: Checks if a byte at a specific memory location is non-zero; if so, it loads the address of a table into a register and branches to another routine, otherwise it returns.
- `load_graphics_block_pointer`: Loads a pointer from a memory location into a register and jumps to the address stored at that location.
- `swap_pointers_and_invert_flag`: Calls swap_and_invert_pointers, inverts a byte in a global variable, calls check_global_bits, and then calls check_and_load_graphics_pointer.
- `set_structure_bits`: Checks a byte at a global address and compares another global byte to 2; if the global byte is less than 2, it sets bit 7 of several bytes at offsets 0, 2, 4, 6, 10, 12, 14, and 16 from a0.
- `set_bits_in_structure`: Sets the seventh bit of four different bytes located at offsets 2, 6, 10, and 14 from a pointer in a0 and returns.
- `dispatch_conditional_function`: Calls a routine at a pointer in a0, loads a 16-bit word from a global variable into d0, and if d0 is not equal to 4, calls another routine and branches to a different location; otherwise it calls a routine at a different pointer.
- `check_mouse_and_call_event`: Checks if the value at a hardware register is less than 2; if so, it loads a pointer from a memory location and calls another routine, otherwise it returns.
- `call_table_function`: Shifts a 16-bit value in d0 left by 2, uses it as an index into a table pointed to by a global variable to fetch a pointer, and calls that pointer if it is not zero.
- `set_and_jump_to_handler`: Stores the value 255 into a global variable and then jumps to a long address stored at that same location.
- `clear_status_byte`: Sets a byte in a global variable to zero and returns.
- `dispatch_callback_via_pointer_2`: Calls process_list_and_callbacks and then calls call_offset_22_handler.
- `execute_table_pointer`: Calls another routine, loads a word from a global address, uses it as an index into a table to retrieve a pointer, and calls that pointer if it is not zero.
- `calculate_and_add_offset`: Subtracts 49 and 47 from a word loaded from a global address, negates the result, calls another routine, multiplies the result by a word from a global address, shifts it left by 4, then shifts it left by 13 and adds the result to a global long value before jumping to another routine.
- `subtract_and_update_pointers`: Loads a word from a global address, subtracts 49 and 47 from it, negates the result, calls two other routines, and then updates two global longwords by subtracting the result from the first and adding d1 to the second.
- `subtract_and_update_pointers_offset`: Loads a word from a global address, subtracts 49, 47, and 16 from it, negates the result, calls two other routines, and then updates two global longwords by subtracting the result from the first and adding d1 to the second.
- `subtract_and_update_pointer`: Subtracts 128 from a word loaded from a global address, calls another routine, multiplies the result by a word from a global address, shifts it left by 4, then shifts it left by 13 and subtracts the result from a global long value before jumping to another routine.
- `scale_and_subtract_pointers`: Loads a word from a global address, subtracts 32 and 47 from it, negates the result, shifts it right by two, calls two other routines, and then updates two global longwords by subtracting d1 from the first and subtracting the result from the second.
- `scale_and_add_pointers`: Loads a word from a global address, subtracts 128 from it, shifts it right by two, calls two other routines, and then updates two global longwords by adding d1 to the first and adding the result to the second.
- `scale_and_adjust_pointers`: Loads a word from a global address, subtracts 145 from it, shifts it right by one, calls two other routines, and then updates two global longwords by adding the result to the first and subtracting d1 from the second.
- `scale_and_add_pointers_arithmetic`: Loads a word from a global address, subtracts 161 from it, shifts it right arithmetically by one, calls two other routines, and then updates two global longwords by adding the result to the first and subtracting d1 from the second.
- `scale_and_subtract_pointer`: Subtracts 128 from a word loaded from a global address, performs an arithmetic right shift by 1, calls another routine, multiplies the result by a word from a global address, shifts it left by 4, then shifts it left by 13 and subtracts the result from a global long value before jumping to another routine.
- `scale_and_add_pointer`: Subtracts 49 and 47 from a word loaded from a global address, negates the result, performs an arithmetic right shift by 1, calls another routine, multiplies the result by a word from a global address, shifts it left by 4, then shifts it left by 13 and adds the result to a global long value before jumping to another routine.
- `clamp_and_scale_value`: Checks if the 16-bit value in register d0 is negative; if so, it sets d0 to zero and branches away, otherwise it calls another routine to perform a scaling operation.
- `calculate_scaled_value`: If the input in d0 is less than 48, it sets d0 to 47, otherwise it performs a logical shift right by 1 and adds 1 before returning.
- `compute_product_pair`: Multiplies d0 by a longword from a global variable and shifts the result right by 4, then uses this to scale two values fetched from a table using an index from d1, returning the results as two 32-bit values shifted left by 2.
- `calculate_index_and_store`: Loads two words from a global address, calls another routine, and if a specific bitmask of the result matches a constant, performs arithmetic on d0 and d1 using values from a global table and stores the result in a global variable.
- `calculate_table_index_3`: Performs arithmetic on d0 and d1 to calculate a value, stores the result in a global word, shifts it left by two, and uses it as an offset into a table pointed to by a global pointer to call another routine.
- `calculate_and_call_indexed_routine`: Calls another routine, checks a bitmask of the result, and if valid, performs division and addition on two values to calculate an index into a table before calling the resulting pointer.
- `execute_multiple_subroutines_7`: Loads a pointer into a0, calls three different routines, then performs a series of conditional calls based on values loaded from two global words.
- `validate_and_update_pointer`: Loads two words from global addresses into d0 and d1, calls another routine, and if the result in d2 masked with a constant is not equal to a specific value, calls another routine.
- `fetch_and_store_byte_alt`: Calls another routine using a value from a global pointer and a constant in d1, then stores the resulting byte in a global variable.
- `copy_data_block_7`: Saves multiple registers, performs several routine calls to retrieve data, and then enters a loop to copy 32 words from a source structure to a destination table if a byte in a global variable is less than two.
- `copy_and_call_subroutines`: Copies 16 words from a source in a0 to two different destination tables using a loop, then calls two different routines and returns.
- `set_status_constant`: Calls another routine to get a pointer, retrieves a word from that pointer, calls another routine again, and then tests bit 5 of that word to set a global longword to either 0, 1, or 2.
- `test_bit_and_set_d0`: Tests bit 6 of a register; if set, it branches to another routine, otherwise it loads the value 1 into that register and branches to the next instruction.
- `set_constant_word`: Writes the value 2 to a longword at a specific memory location and restores the stack.
- `process_structure_data_7`: Saves registers, checks a global byte, and if it is non-zero, performs a series of table lookups and calls to two different routines before updating a global word and calling a third routine.

The function list continues in: g2158-recovery-functions-2.md, g2158-recovery-functions-3.md, g2158-recovery-functions-4.md, g2158-recovery-functions-5.md.
