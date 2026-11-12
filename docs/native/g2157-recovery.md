# Game 2157 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02157). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 57 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 41 of 41 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 6041 of 6041 named (0 author symbols), 6041 of 6041 described (149 by rule, 5536 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 240 of 240 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 116388 of 116405 instructions identical in start and length, 4 mismatches |

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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_BLIT#unpacked | 7 | 52210 | 2715 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_CPU#unpacked | 6 | 47660 | 2691 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_BLIT: crunched executable decrunched by XFDMaster (Imploder (xfd), 161876 -> 359116 bytes), 123 of 167 windows confirmed in reference RAM: recovered as a hunk executable
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_CPU: crunched executable decrunched by XFDMaster (Imploder (xfd), 160772 -> 347516 bytes), 121 of 165 windows confirmed in reference RAM: recovered as a hunk executable
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon: Imploder (xfd) stream found by ancient scan (5784 -> 10488 bytes) but only 0 of 6 sampled windows are in reference RAM: not used
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon_install: Imploder (xfd) stream found by ancient scan (10908 -> 22940 bytes) but only 0 of 11 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 operand
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a2]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `stub_unreached_gap_code` (More): The recovered body is only a short call sequence ending in an indirect call, and it was reached only as a gap in the decode. Kind: sabotage path. Pass path: the instruction following the call to the third (unaffected/unreachable) pointer.
- `configure_hardware_and_copy_data` (AM2_BLIT#unpacked): Sets DMA, interrupt, and custom register values, calls another routine, copies 12 bytes from a calculated offset in a global table to another table, updates three global words, and calls another routine. Kind: checksum/data integrity trap. Pass path: the loop continues and the routine proceeds to update pointers/counters.
- `configure_hardware_and_copy_data_2` (AM2_CPU#unpacked): Sets custom hardware registers for DMA and interrupts, calls another routine, copies twelve bytes from a table to a destination using a loop, updates three global variables, and calls another routine after setting a byte to sixty-three in a global variable. Kind: disk hardware/track check. Pass path: the loop following the call (data copy loop).

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

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a2].adf:AM2_BLIT#unpacked

- `jump_to_address_6`: Performs an indirect jump to the address stored in a longword at the location specified by a0.
- `validate_and_execute_callback`: Compares a longword at the pointer in a0 to a constant, and if it matches, copies a word and a byte from the pointer to global variables, updates two global pointers, and calls a routine via a pointer.
- `initialize_data_tables`: Sets the stack pointer to a specific address, calls eleven different routines, loads a pointer from a global table into a0, and calls several more routines before branching.
- `execute_handler_sequence_4`: Sets d0 to zero and calls a routine, then calls another routine.
- `execute_callback_via_pointer`: Calls a routine, loads a pointer from a global variable into a7, loads a value from a global variable into d0, loads a pointer from a global variable into a0, and jumps to the address in a0.
- `initialize_and_update_data_tables`: Clears a block of memory, performs several conditional checks and calls to other routines, populates a table with incrementing values in a loop, copies a block of data from a source to a destination, and updates a series of values in a table by reading bytes from a structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_word_blocks`: Copies three 16-bit words from a source pointed to by a0 to a destination pointed to by a1 in a loop, increments a counter in a global variable, and clears the destination word if the counter reaches 512.
- `iterate_and_process_table`: Iterates through a table pointed to by a global variable, comparing each word to a value in d7, and if they match, copies a byte from offset 2, a byte from offset 3, and a word from offset 4 of the current entry into registers before calling another routine.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `compare_and_update_structure`: Loads a pointer from a global variable, compares several bytes and a word at offsets 0, 1, 3, and 4 of the pointed-to structure against d0, d1, and d2, and if all match and the word at offset 4 equals d2, it writes d2 to offset 4; otherwise, if the structure is empty, it populates it with values from registers and calls another routine.
- `branch_to_offset_5`: Adds 6 to the address in a0 and branches to a relative offset.
- `update_structure_data`: Increments a global 16-bit value and, if it is less than 513, writes a 16-bit word, two bytes, and a 16-bit word from registers to a structure pointed to by a0.
- `check_conditional_byte`: Saves registers to the stack, compares a byte at a specific address to one, and if it matches, loads a pointer from a table and compares a byte at offset 2 of that pointer with d7.
- `increment_and_loop`: Increments a word in a0 by 4, performs a loop using d6 as a counter, and then branches to another routine.
- `process_conditional_data_3`: Compares d7 with a value in a table, then enters a loop that conditionally calls two different routines to either write a word to an offset of 332 plus a calculated index in a structure or write a byte to an offset of 348 plus a calculated index.
- `execute_dual_callbacks`: Saves d0 to the stack, calls a routine using a byte from d6, uses the returned pointer to write a byte from d2 to an offset of 332 plus the pointer, then repeats the process by calling another routine and restoring registers.
- `process_conditional_data_6`: Compares d0 to 11, stores d0 in a global word if it is less than 11, calls another routine if a global byte is greater than 2, and if a global byte is non-zero, retrieves a byte from a table using d0 as an index, stores it in a global word, and calls another routine.
- `call_conditional_routine_6`: Checks if the high byte of d0 is non-zero and not equal to a value in a global word, then calls another routine if these conditions are met.
- `process_conditional_updates_2`: Uses a byte from a table to call a routine, performs several conditional checks against global values and structure members, and updates a global pointer to a calculated offset in a table.
- `set_d0_to_one_hundred`: Loads the constant 256 into d0 and jumps to another routine.
- `set_d0_to_five_hundred_twelve`: Loads the constant 512 into d0 and jumps to another routine.
- `set_d0_to_seven_hundred_sixty_eight`: Loads the constant 768 into d0 and jumps to another routine.
- `set_d0_to_one_thousand_twenty_four`: Loads the constant 1024 into d0 and jumps to another routine.
- `set_d0_to_one_thousand_two_hundred_eighty`: Loads the constant 1280 into d0 and jumps to another routine.
- `set_d0_to_one_thousand_five_hundred`: Loads the constant 1536 into d0 and jumps to another routine.
- `handle_event_data_2`: Checks if the high byte of d0 is non-zero, retrieves a byte from a table using d0 as an index, calls a routine, and if a specific byte in the resulting structure is non-zero, stores the retrieved byte into a global address and calls another routine.
- `store_and_call_handler`: Stores a byte from d2 into a global address and a word from d1 into another global address, calls a routine, and then calls another routine if a bitmask check on d3 succeeds.
- `call_event_handler`: Loads a pointer into a0 and calls another routine.
- `jump_with_constant_100`: Loads the word 256 into d0 and jumps to another routine.
- `jump_with_constant_200`: Loads the word 512 into d0 and jumps to another routine.
- `jump_with_constant_300`: Loads the word 768 into d0 and jumps to another routine.
- `jump_with_constant_400`: Loads the word 1024 into d0 and jumps to another routine.
- `jump_with_constant_500`: Loads the word 1280 into d0 and jumps to another routine.
- `jump_with_constant_600`: Loads the word 1536 into d0 and jumps to another routine.
- `dispatch_event_one`: Sets d0 to 1 and jumps to another routine.
- `dispatch_event_two`: Sets d0 to 2 and jumps to another routine.
- `dispatch_event_three`: Sets d0 to 3 and jumps to another routine.
- `dispatch_event_four`: Sets d0 to 4 and jumps to another routine.
- `dispatch_event_five`: Sets d0 to 5 and jumps to another routine.
- `dispatch_event_six`: Loads the value 6 into d0 and jumps to another routine.
- `dispatch_event_seven`: Loads the value 7 into d0 and jumps to another routine.
- `dispatch_event_eight`: Loads the value 8 into d0 and jumps to another routine.
- `dispatch_event_nine`: Loads the value 9 into d0 and jumps to another routine.
- `iterate_and_call_handler`: Iterates through a series of bytes starting at an address in a0, calling a routine if a byte is non-zero, and continues for up to seven iterations.
- `process_conditional_structure_update`: Calls a routine to retrieve a pointer, checks bit 5 of a 16-bit word at offset 46 of the pointed-to structure, and if that bit is not set, checks a global byte and calls another routine; it then calls a second routine using a byte from d6 and restores registers.
- `process_event_table_entries`: Iterates through a table of up to six entries, calling a routine to resolve a pointer from a byte and another routine to retrieve a word from offset 46 of the resulting structure.
- `set_event_parameters_4`: Stores a word into a global variable, stores a long into another global variable, and calls two different routines using a pointer in a0.
- `call_event_handler_2`: Calls a routine, loads a pointer into a0, and then jumps to another routine.
- `execute_complex_sequence_4`: Performs a series of routine calls with various constant parameters, copies 32 words from one memory location to another in a loop, updates several global values, and calls multiple routines including one that takes specific values in d0, d1, d2, and d3.
- `execute_blit_sequence_2`: Calls a routine to perform a complex blit and then jumps to another routine.
- `execute_conditional_calls_11`: Loads two words and a byte into registers, calls a routine, and if the zero flag is not set, calls another routine before returning.
- `execute_conditional_handler_10`: Loads values into d0, d1, and d3, calls a routine, and if the result is not zero, calls another routine before returning.
- `process_callback_table`: Retrieves a byte from a table using an index in d0, calls a routine with that byte, retrieves a pointer from the resulting structure, calls another routine, and if a specific bitwise condition is met on a value from that structure, stores a word in a global variable and calls a third routine.
- `process_callback_and_update_index`: Retrieves a byte from a table using an index in d0, and if non-zero, calls a routine with that byte as an argument.
- `execute_callback_and_update_index`: Checks a byte in a table using an index in d0; if non-zero, it calls another routine with that byte, retrieves a pointer from the result, calls a third routine using a word from offset 30 of that pointer's structure, and if a bitmask check on that word passes, stores the original index into a global variable and calls a fourth routine.
- `process_event_data_3`: Retrieves a byte from a table using d0 as an index, calls another routine, and if the byte is non-zero, calls a second routine and then a third routine that stores d7 into a location at a global address.
- `update_global_word_4`: Performs a bitwise AND on d1 with a constant, and if the result is non-zero, stores d0 into a global variable and calls another routine.
- `execute_conditional_callback_5`: Checks a byte at an offset from a pointer in a0 using d0 as an index, and if that byte is non-zero, stores d0 in a global variable and calls another routine.
- `call_pointer_handler_2`: Pushes d1 to the stack, loads a byte from an offset of 30 in a global variable into d1, calls a routine using d1 as a pointer, and then restores d1 from the stack before returning.
- `execute_conditional_callback_4`: Saves registers, retrieves a byte from a global variable, calls a routine to get a pointer, retrieves a word from offset 30 of that pointer, calls another routine, and if a bitwise check on the result passes, calls a third routine using a byte from a global variable.
- `call_handler_48`: Pushes d1 to the stack, loads a byte from an offset of 72 in a global variable into d1, calls a routine using d1 as a pointer, and then restores d1 from the stack before returning.
- `call_handler_46`: Pushes d1 to the stack, loads a byte from an offset of 70 in a global variable into d1, calls a routine using d1 as a pointer, and then restores d1 from the stack before returning.
- `call_handler_47`: Pushes d1 to the stack, loads a byte from an offset of 71 in a global variable into d1, calls a routine using d1 as a pointer, and then restores d1 from the stack before returning.
- `copy_string_and_update`: Calls a routine to get a pointer, uses that pointer to call a routine that copies a null-terminated string, decrements a value in a0, calls another routine using a byte from d1, and restores registers.
- `execute_dual_handlers`: Calls two different routines using pointers retrieved from a table and a structure, then loads a new pointer into a1 if a byte comparison is non-zero.
- `copy_string_from_address`: Loads a pointer from a global variable into a1, calls another routine, decrements a0, and restores registers from the stack before returning.
- `execute_dual_handlers_alt`: Calls two different routines using pointers retrieved from a table and a structure, then loads a pointer from a different global variable into a1 if a byte comparison is non-zero.
- `copy_string_from_address_alt`: Loads a different pointer from a global variable into a1, calls another routine, decrements a0, and restores registers from the stack before returning.
- `call_with_constant`: Saves d1 to the stack, loads the constant 100 into d1, calls a routine, and then restores d1 before returning.
- `test_constant_value`: Saves registers to the stack, compares a word at a specific address to six, and if they are not equal, sets d7 to zero and returns; otherwise it returns immediately.
- `check_word_sign`: Tests if d0 is greater than zero; if not, it sets d7 to 255 and branches to another location, otherwise it branches to a different location.
- `compare_word_values`: Compares two words and, if the first is not less than the second, clears d7 and branches.
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
- `set_bit_conditionally_2`: Compares d2 to 1 and, if not equal, branches; otherwise it sets a bit in a byte at the address in a0.
- `modify_bit_conditionally`: Compares a byte in d2 to 2; if it is not equal, it changes a bit in the byte at the address in a0 and branches to another location, otherwise it continues.
- `check_bit_conditional`: Compares d2 to 3; if not equal, it otherwise checks if bit 0 of a longword at the address in a0 is set.
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
- `store_address_and_branch`: Stores the constant value 34417 at the address in a2 and then branches to another location.
- `process_structure_list_5`: Increments a pointer in a1 by 6 and enters a loop that checks a bit in a structure at a0 and compares a value at a1 plus 10 to zero, eventually storing a pointer and a global address into a structure pointed to by a2.
- `increment_global_byte_5`: Checks a global byte; if it is zero, it calls a routine to retrieve a value, stores that value in the global byte, and then increments the global byte.
- `decrement_global_byte_7`: Decrements a byte in a global variable if it is non-zero and not equal to one, otherwise it returns immediately.
- `check_status_and_branch`: Compares a longword at a global address to 3; if equal, it branches to another location, otherwise it compares a byte from a table to d0 and branches based on the result.
- `call_indexed_function`: If the byte at the address in a0 is zero, it calls a function from a table using an index in a global variable, increments that index, and stores a pointer in a global variable.
- `jump_with_zero`: Sets d0 to zero and jumps to another location.
- `jump_with_constant`: Sets d0 to 293 and jumps to another location.
- `update_structure_flags_3`: Calls a routine with d0 set to 294, then loops six times through a global structure to retrieve pointers, set bit 5 of a word at offset 30 of those pointers, and call another routine.
- `execute_table_callback_4`: Performs several routine calls to process data, then uses a value from the stack to calculate an offset into a table.
- `call_blit_routine_one`: Loads several 16-bit constants into d0, d1, d2, and d3, sets d4 to 28, and calls another routine.
- `call_blit_routine_two`: Loads several 16-bit constants into d0, d1, d2, and d3, sets d4 to 28, and calls another routine.
- `call_blit_routine_twice`: Sets d4 to 28 and calls another routine twice using two different sets of 16-bit values in d0, d1, d2, and d3.
- `jump_to_address_2`: Jumps to the address stored in a longword at a specific location.
- `call_blit_clipping_rectangle`: Loads registers d0 through d4 with the values 16, 138, 303, and 192 respectively and calls blit_clipping_rectangle.
- `call_blit_clipping_rectangle_twice`: Calls blit_clipping_rectangle twice, first with d0 through d4 set to 15, 43, 191, and 122, and then with d0 through d4 set to 15, 43, 303, and 122.
- `call_blit_clipping_rectangle_single`: Loads registers d0 through d4 with the values 16, 50, 191, and 193 respectively and calls blit_clipping_rectangle.
- `call_blit_clipping_rectangle_double`: Calls blit_clipping_rectangle twice, first with d0 through d4 set to 16, 43, 303, and 122, and then with d0 through d4 set to 16, 144, 193, and 193.
- `fill_array_with_white`: Fills a series of 32-bit words with the value negative one starting from a global variable, using an index in d7 to determine the offset.
- `process_graphics_list`: Saves registers, calls a routine with a pointer to a structure, calls another routine in a loop six times with varying offsets, calls a third routine with a modified pointer, and calls a fourth routine with a value in d7.
- `sync_sprite_data`: Compares two words from global variables, updates a table with white values if they differ, and enters a loop that compares and updates bytes between two structures while calling two routines if certain bit conditions are met.
- `process_graphics_structure`: Saves registers to the stack, performs arithmetic to calculate a value for d7, calls several routines to handle data at various offsets, copies a string of bytes from a source to a destination, and performs a loop to copy 5 bytes from a structure to a destination.
- `update_sprite_scaling_and_status`: Calculates scaling factors for a structure based on width and height dimensions, updates scaling bytes in a secondary structure, and performs several blitter operations to redraw or adjust the sprite's appearance.
- `update_element_arrays`: Calls a routine, then enters a loop that checks a byte at an offset of a structure, calls a routine, decrements that byte, and continues until the byte is zero; it then performs a second loop that calls two routines for up to seven iterations.
- `branch_to_handler`: Saves d7 and a0 to the stack and then performs a short branch.
- `execute_blitter_setup_2`: Calls two routines, then enters a loop that checks bytes in a table at a specific address and calls a routine if the byte is non-zero, repeating this up to six times before calling a final routine.
- `execute_scaled_blit_sequence_2`: Calculates a value using multiplication and addition, calls a routine, and if d2 is non-zero, updates a global variable and calls two more routines.
- `update_byte_array_elements`: Calculates a value based on a byte from a table, performs a scaled blit, and enters a loop to update elements in a structure while decrementing a counter.
- `process_byte_array`: Decrements a byte at an offset of a1, and if it reaches zero, it decrements a word in d4 and calls another routine; it then enters a loop that calls another routine and writes bytes to a table based on d4 and d7.
- `process_bit_flag`: Checks a global word against a threshold, clears it if negative, and if a specific byte is non-zero, searches a table to find a matching bit index to store in a destination buffer.
- `search_and_set_bit`: Retrieves a pointer from a table using a byte from a0, then enters a loop to search a table for a byte that satisfies a bit test against a value from a structure, finally storing the result in a1 and a pointer in a3.
- `increment_and_store_values`: Stores d2 at the address in a3, increments d2, stores the result in a1, and then enters a loop that increments several registers and pointers until d7 reaches 7.
- `increment_global_counter`: Increments a 16-bit value at a global variable address.
- `execute_blitter_loop`: Loads a pointer from a global variable into a1 and another into a3, then enters a loop that runs five times, calling another routine if the value at the pointer in a1 is zero.
- `clamp_d7_value_3`: Compares d7 with a constant, and if d7 is not less than that constant, sets d7 to twelve and branches.
- `execute_blitter_sequence_4`: Performs arithmetic on d7, calls a routine with a pointer in a3, and executes a loop that performs two blitter-related routine calls while incrementing d0 and d1 and decrementing d3.
- `set_status_word`: Sets a global word to 31 and compares a global word at offset 3734 to 6; if it is 6, it branches to another routine, otherwise it loads a value from offset 3736 into d3.
- `toggle_status_bits`: Performs bitwise operations on two global words, and if specific bits are set, calls another routine with specific parameters.
- `setup_blit_parameters_8`: Loads several constant values into registers, loads a pointer from a table into a0, calls another routine, and then branches based on the values of bits 2 and 0 of d2 and d3.
- `execute_conditional_blit_3`: If bit 2 of d3 is set, it branches; otherwise it saves d2 and d3, sets several registers, calls another routine, and branches to another location.
- `calculate_offset_and_call`: Loads a word from a table using an index in d0, multiplies it by 384, adds the result to a0, and calls another routine before branching to another routine.
- `test_status_bits`: Tests bit 0 of d2 and bit 0 of d3, then branches based on the results or performs an unconditional branch.
- `setup_blit_parameters_7`: Tests bit 0 of d3, and if it is not set, saves registers and calls another routine with specific constant values in d0, d1, d2, and d3.
- `setup_blit_parameters_6`: Loads several constant values into registers and calls another routine.
- `calculate_blit_parameters_4`: Calls a routine with specific values in d0-d3, then checks a byte in a global variable and performs arithmetic on a word from another global variable before branching.
- `calculate_blit_parameters_3`: Calculates a value based on a word from a global variable, performs arithmetic involving multiplication and division, and calls three different routines to update various memory locations and structures.
- `initialize_blit_sequence`: Tests bit 1 of d3 and branches if set, otherwise saves d2 and d3 to the stack, loads several constant values into d0, d1, d2, d3, and d4, calls another routine, and branches.
- `branch_on_byte_3`: Tests a byte at a global address, and if it is non-zero, branches to another location; otherwise, it loads a pointer into a0 and branches.
- `initialize_blit_parameters`: Loads a longword from a global variable into a0, sets d0, d1, d4, d5, d6, and d7 to specific constant values, and calls another routine.
- `call_handler_with_constant`: Saves registers, sets d1 to a constant, calls another routine, and stores the resulting byte from d0 into a global memory location.
- `execute_handler_calls`: Saves registers, calls a routine using a byte from a global variable, calls another routine with specific constant values in d1, d5, d6, and d7, and then calls a third routine using a byte from a global variable.
- `process_structure_data_6`: Saves registers, calls a routine to retrieve a pointer, fetches a byte from a table using a register as an index, and performs several calls to update a structure and its fields.
- `update_structure_pointer_2`: Saves d0, d7, and a0 through a1 to the stack, loads two pointers into a1 and a0, calls a routine, stores the result in a structure, loads a byte from a table using d7, calls another routine, updates a pointer in a structure, clears a long value, and calls a third routine before finally calling a fourth routine and returning.
- `update_structure_pointer`: Fetches a pointer from a table, adds an offset of 274 to it, and stores the resulting address into the location pointed to by a0, then calls another routine.
- `update_table_and_call_2`: Saves d0 and a0 through a2 to the stack, loads a pointer from a table into a1, adds d7 to a1, reads a byte from a1 into d0, calls a routine, updates a pointer in a structure, calls another routine, calls a third routine, and returns.
- `initialize_and_jump_to_handler`: Writes the value 65535 to two global words and then calls another routine before jumping to a third routine.
- `swap_and_invert_pointers_2`: Calls two routines and then jumps to a third routine.
- `process_indexed_callback`: Checks if d0 is zero or negative one, calls a routine, calculates an index using a word from a global variable and a multiplication, calls two routines using values retrieved from a table, and if a value is non-zero, stores a word in a global variable and calls a third routine.
- `fetch_and_store_table_value`: If d0 is non-zero, it calls another routine, calculates an index by multiplying the masked value of d0 by 6, retrieves a word from a table at offset 290 of a pointer returned by another routine, and if that word is non-zero, stores the original d1 value into a global variable and calls another routine.
- `fetch_and_store_offset_value`: If d0 is non-zero and not 255, it calls another routine, calculates an index by adding 8 and a value from a global variable to the masked d0, retrieves a word from a table at offset 290 of a pointer returned by another routine, and if that word is non-zero, stores the original d1 value into a global variable and calls another routine.
- `process_clamped_index`: Tests d0 for zero or 255, masks and decrements d0, adds a word from a global pointer to d0, and calls another routine if the result equals a constant.
- `process_scaled_data_2`: Multiplies d0 by 6, calls another routine using a byte from a global variable, adds a global address to a pointer, retrieves a word from a table, calls another routine, calls a third routine, and if the result is non-zero, stores d1 into a global variable and calls a fourth routine.
- `call_update_routine_3`: Calls a routine and then returns.
- `test_and_branch_data`: Tests a byte in d0 and branches if it is zero, otherwise it masks the lower byte, decrements d0, and compares it to a value in a global variable.
- `process_offset_scaled_data`: Multiplies d0 by 6, calls another routine using a byte from a global variable, adds 290 to a pointer, retrieves a word from a table, calls another routine, calls a third routine, and if the result is non-zero, stores d1 into a global variable and calls a fourth routine.
- `call_update_routine_alt`: Calls a routine and then returns.
- `update_global_constant`: Decrements d0, multiplies it by 60, adds the result to a base address in a0, writes the constant 2 to a global variable, and calls another routine.
- `execute_callback_loop_2`: Loads two words from a table, performs a loop of eleven calls to another routine using those words and two other constants, then performs one final call with different constants and returns.
- `loop_graphics_updates`: Executes a routine in a loop for eleven iterations, incrementing d0 on each pass.
- `update_graphics_structure`: Saves registers, performs multiple calculations involving multiplication and table lookups, calls several routines to update data, and modifies a global word at a specific offset.
- `modify_graphics_data`: Saves registers, calculates an offset into a table using d0, calls a routine, performs arithmetic on a value from a table, and calls several routines to update data based on bit tests.
- `update_index_and_byte`: Decrements a 16-bit value in a global variable; if it reaches zero, it increments another global variable, resets the first to four, and then reads a byte from a table at an offset determined by the second global variable.
- `increment_table_index_and_fetch_byte`: Decrements a counter in a global variable and, if it reaches zero, increments a table index and resets the counter to four.
- `execute_blit_logic`: Checks a global byte, and if it is zero, calculates an offset into a table, calls another routine, and then performs a multiplication and addition to update a pointer in a1.
- `push_and_call_pointer`: If d0 is not equal to 1, it pushes d0 to the stack, retrieves a pointer from a location at offset 40 of a6, calls another routine, moves the result to a1, restores d0 from the stack, and branches to another routine.
- `execute_blit_command_2`: Compares d0 to two, branches if not equal, pushes d0 to the stack, loads a byte from a pointer in d0, calls a routine, sets a1 to the result, loads d0 from the stack, adds a constant to a1, sets a2 to a constant, and branches.
- `execute_blit_command`: Clears a pointer in a1, calls a routine, loads a byte from a global variable into d0, and jumps to another routine.
- `execute_callback_chain_5`: Calls a routine via a pointer, then jumps to another routine via a pointer.
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
- `call_subroutines_3`: Calls two different routines and then returns.
- `execute_handler_jump`: Loads a byte from a global variable into d0, calls another routine, and then jumps to a new address.
- `perform_multiple_blit_calls`: Calls several routines with specific word parameters, updates a pointer from a global variable, and performs a final jump to another routine.
- `process_conditional_list_4`: Checks a word in a global variable and, if it is not zero, calls two routines before returning.
- `calculate_scaled_value_13`: Initializes several registers, calls another routine, and then performs a loop that calculates a value based on a global word and a global longword, updating a global word with either 31 or 16 during the process.
- `modify_table_entry`: Loads a pointer from a table, fetches a word from a location pointed to by a1, calls another routine, and if the result is zero, writes sixteen to a specific global address before branching to another routine.
- `update_and_call_routine`: Writes 31 to a global word, calls a routine, sets d0 to 52, copies d3 to d1, calls another routine, increments d3, and then pushes d0 to the stack, loads a byte from a global address into d0, calls a routine, and restores d0 before returning.
- `execute_conditional_blit_operation`: Loads two 16-bit values and a 30-count constant into registers, calls check_blit_bounds, and if the zero flag is not set, calls check_bits_and_jump before returning.
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
- `set_event_one_hundred`: Loads the constant 256 into register d0 and branches to another routine.
- `set_event_two_hundred`: Loads the constant 512 into register d0 and branches to another routine.
- `set_event_three_hundred`: Loads the constant 768 into register d0 and branches to another routine.
- `set_event_four_hundred`: Loads the constant 1024 into register d0 and branches to another routine.
- `set_event_five_hundred`: Loads the constant 1280 into register d0 and branches to another routine.
- `set_event_six_hundred`: Loads the constant 1536 into register d0 and branches to another routine.
- `process_event_with_word_check`: If the high byte of d0 is non-zero and not equal to a global word, it retrieves a pointer from a table, calls a routine, and if a byte at offset 9 of the resulting structure is non-zero, stores d2 into a global byte and calls another routine after checking a word at offset 30.
- `update_conditional_state`: Performs a bitwise AND on d3 with a constant, and if the result is non-zero, calls three consecutive routines after storing d2 and d1 into global locations.
- `load_event_parameter`: Loads the constant value 256 into register d0 and branches to another routine.
- `set_event_two_hundred_word`: Loads the constant 512 into register d0 and branches to another routine.
- `set_event_three_hundred_word`: Loads the constant 768 into register d0 and branches to another routine.
- `set_event_four_hundred_word`: Loads the constant 1024 into register d0 and branches to another routine.
- `set_event_parameter`: Loads the value 500 into d0 and branches to another routine.
- `set_event_parameter_alt`: Loads the value 600 into d0 and branches to another routine.
- `execute_callback_chain_6`: Loads 6 into d0, calls two routines, loads a pointer from a global address into a0, calls a third routine, and jumps to another routine.
- `execute_blit_and_update_fields`: Calls execute_complex_blit to perform two blitter operations and update a global variable, then calls update_conditional_fields to conditionally execute two routines based on a global word value before writing a constant to a global variable.
- `execute_sequential_calls_5`: Calls three different routines via pointers and then jumps to a fourth routine via a pointer.
- `set_status_bit_11`: Fetches a pointer from a table, checks specific bits in a word at offset 30 of the structure, and if certain conditions are met, sets bit 7 of the byte at the address in a0.
- `call_and_jump`: Calls another routine and then jumps to another routine.
- `copy_list_data`: Calls a routine, loads a pointer into a0, and jumps to another routine after performing a loop that copies data from one list to another.
- `compare_and_call_5`: Tests d0, and if it is not zero, masks it, decrements it, and compares it to a value at a global address; if not equal, it calls another routine and branches.
- `process_conditional_data_5`: Calls two different routines, stores a word from d0 into a global variable, and if a specific bit is set in a word from d7, it stores d0 into a global variable and calls another routine.
- `process_data_table_2`: Loads a pointer into a0, moves d0 to d2, and loads a byte from the address in a0 into d1.
- `update_global_word_5`: Stores a word from d0 into a global variable, moves a byte from d1 into d0, and tests a bit in d0 using a constant in d1.
- `dispatch_tile_update`: Moves a word into a global variable, loads a pointer into a0, extends d0, and calls two different routines before branching to another location.
- `call_subroutine_and_return`: Calls a subroutine and then returns.
- `render_ui_text_elements_2`: Calls several routines to fetch data from tables and performs a series of blitter operations to draw text characters or symbols, including loops that iterate through character sets and write specific byte values to memory.
- `configure_blitter_parameters_2`: Sets up three address registers, enters a loop that checks a byte from a table, and if a bit is set, calculates an offset and calls another routine.
- `call_decrement_routine`: Pushes d0 to the stack, fetches a byte from a global address into d0, and calls another routine.
- `process_graphics_parameters`: Calls several routines to retrieve data from a table, updates a global variable, and performs a conditional branch based on a byte at an offset of a structure in a1.
- `render_complex_elements`: Performs several routine calls using various offsets from a structure in a1, including a loop that checks a byte in a table and performs calculations to determine a value for subsequent routine calls.
- `draw_complex_scaled_elements`: Performs a series of scaled blitter operations using various offsets from a structure and updates a global variable based on a comparison.
- `calculate_scaled_tile_dimensions`: Sums two 16-bit values from a structure, scales the result by a value from a global variable, and calls perform_scaled_calculations and draw_scaled_blt_tiles.
- `perform_scaled_calculations`: Writes two to a global word, performs a series of multiplications and divisions using values from a structure in a1, calls three different routines to update parameters and perform drawing, and finally restores registers and returns.
- `execute_jump_routine`: Calls a routine via a pointer and then jumps to another address.
- `validate_sprite_attribute_type_3`: Checks if a byte at offset 4 of a structure equals 3, then verifies that the sum of two 16-bit words at offsets 218 and 220 of a structure pointed to by a retrieved pointer is non-zero; if the check fails, it clears a byte at offset 4 and sets a 16-bit value at offset 20 of the structure to 65535.
- `validate_sprite_metadata`: Checks if a byte in d0 equals 4, then validates a sprite index from a structure in a1 against a table at a fixed address by comparing specific bytes.
- `validate_sprite_attribute_data`: Checks if a byte in d0 equals 7, then uses values from the structure in a0 to index into a table via a1 to validate specific attribute bytes against a secondary data table.
- `process_inventory_data`: Loads 2 into d0, calls a routine, sets d0, d1, and d2 to specific constants, calls three more routines, and then branches based on a byte at a global address.
- `dispatch_graphics_block`: Loads a pointer into a0, calls a routine, and jumps to another routine.
- `execute_complex_blit_sequence`: Calls two routines and then jumps to a third routine.
- `update_structure_values_15`: Calls four routines via pointers and then jumps to a fifth routine via a pointer.
- `load_and_jump`: Calls two routines via pointers, loads a constant address into a0, and then jumps to another routine.
- `call_two_and_jump`: Calls another routine, calls another routine, and then jumps to another routine.
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
- `update_structure_metadata_and_loop`: Calculates an index into a table to retrieve two words, then calls a routine to fetch a value used to index into a secondary table.
- `loop_structure_updates_with_scale`: Calls another routine, performs arithmetic on d0 using a value from a global variable, calls two more routines, writes two words from global variables to a structure in a0, calls a fourth routine with specific parameters, and enters a loop that repeats these operations using d7 as a counter.
- `execute_looping_calls_2`: Loads a byte from a global variable, calls another routine, performs arithmetic on a value from a global variable, and then calls two more routines within a loop controlled by a decrementing register.
- `fetch_and_store_word`: Loads a word from a global address, calls another routine, and stores the returned word at an offset in a table determined by d1.
- `process_structure_data_2`: Calls several routines to manipulate data within a structure pointed to by a0, including performing arithmetic on a value at offset 270 and conditionally updating bytes at specific offsets using values from a table.
- `update_entity_status_flags`: Retrieves an entity structure via a call, then performs multiple conditional checks on its fields to set specific bits in a status structure passed in a0.
- `process_graphics_structures_2`: Calls two routines, sets a longword in a global variable, sets a byte in a global variable, sets a longword in a global variable, and copies a word from a table into a structure at offset 18.
- `process_graphics_structures`: Executes a loop to call a routine, then performs multiple conditional checks involving table lookups and structure offsets to call various routines that modify data or perform blitting operations.
- `clear_and_call_routines_2`: Clears a longword at a global address, sets d0 to 44, and calls two different routines before returning.
- `execute_sequence_calls`: Sets a byte at a global address to zero and calls three different routines in sequence.
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
- `branch_to_handler_2`: Stores a byte at a global address and branches to another routine.
- `copy_and_update_structure_extended_2`: Checks a global byte, updates a global word, clears a longword, retrieves a word from a structure, and if the word is not 1, updates a global longword with a value from a structure.
- `copy_and_update_structure_extended`: Stores a longword from d0 into a global variable, copies a word from a2 to a global variable, sets a word at offset 22 of a global structure to 54, clears a global byte, calls another routine, and if a global longword is non-zero, copies a structure from a2 to offset 1250 and performs multiple routine calls and global updates.
- `call_decrement_handler_duplicate`: Loads a byte from a memory location into d0 and calls another routine.
- `copy_and_jump_values`: Copies two 16-bit words from a source location to a destination pointed to by a global variable, then jumps to a third location via a pointer.
- `execute_triple_calls`: Calls one routine, sets d0 to 55, and then calls two more routines before returning.
- `execute_handler_sequence_2`: Sets the sign flag based on a byte at a global address, then calls three different routines in sequence before jumping to another routine.
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
- `increment_structure_by_product`: Adds a byte from a2 to a memory location, calculates a product using a word from a structure in a1 and d3, adds the result to a longword at offset 270 of a structure in a0, clears a word at offset 4 of a structure in a3, and branches to another routine.
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
- `branch_on_byte_comparison_2`: Compares a byte at a global address to two, branches if it is positive, otherwise it stores a word in a global variable, loads an address into a0, calls another routine, and branches to another routine.
- `load_and_test_words`: Loads two words from memory, calls a routine, and if the result is not zero, calls another routine and then calls a third routine.
- `check_mouse_and_call`: Compares a byte at a specific address with two and, if it is not greater than or equal to two, stores a long value into a global variable, loads an address into a0, calls another routine, and then branches to another location.
- `execute_conditional_calls_17`: Loads two words from memory into d0 and d1, calls another routine, and then performs a sequence of conditional calls based on the results of the first call and subsequent comparisons.
- `process_event_pointer_2`: Pushes d0 to the stack, retrieves a pointer from a global variable, calls another routine, and then uses the returned pointer to access a byte at offset 4.
- `loop_and_branch`: Decrements d7 in a loop and branches to another location if the loop is not finished, otherwise it branches to a third location.
- `execute_event_logic_2`: Loads a pointer into a0 and calls another routine, then if a global word is non-zero, sets a global byte and calls another routine.
- `test_and_call_2`: Tests a byte at a global address, and if it is not zero, calls another routine with d0 set to 39 and then branches to another location.
- `execute_indirect_calls_3`: Calls a routine via a pointer, loads an address into a0, calls another routine, and then returns.
- `check_global_condition`: Compares a word at a global address to 6 and, if not equal, sets d0 to 15 and branches.
- `handle_event_logic`: Pushes d0 to the stack, calls another routine with a byte from a global address, uses the returned pointer to load a word into d1, pushes d0 to the stack, calls another routine with a byte from a global address, and if bit 3 of d1 is not set, calls another routine with d0 set to 289.
- `set_conditional_byte_4`: Checks if a word at a global address is non-zero, otherwise it sets a byte in a global variable to fifteen, calls a routine, loads an address into a0, calls another routine, and returns.
- `jump_to_handler_7`: Calls a routine, loads an address into a0, and jumps to another routine.
- `jump_to_address_11`: Loads an address into a0 and jumps to another routine.
- `populate_structure_table`: If a global byte is zero, it calls two routines, calculates two offsets based on global words, stores these offsets and a byte into a structure at a global address, then enters a loop that performs 24 iterations of storing a longword and calling a routine.
- `load_table_values`: Loads two words from memory, compares a byte at a global variable to 2, and if the byte is not positive, loads a third word into d2 and calls another routine.
- `calculate_shifted_offsets_2`: Loads two words from a table using an index from a global variable, performs multiplication and bit shifting on these values, modifies two global longwords by subtracting and adding the results, and calls another routine.
- `update_handler_and_jump`: Calls another routine, stores a long value into a global variable, loads an address into a0, and jumps to another routine.
- `compare_and_call_handler`: Performs arithmetic on two values from global variables, compares the results against constants, and if the conditions are met, loads a pointer into a0 and calls another routine.
- `update_global_pointer_5`: Loads a pointer from a global variable into a0 and calls another routine.
- `calculate_and_dispatch`: Performs arithmetic on two sets of values from global variables, calls two routines, and jumps to an address contained in a global pointer.
- `validate_range_and_call`: Calls another routine, then checks if the result in d0 satisfies a range condition relative to a global word at offset 3704, and if the value in d1 satisfies a range condition relative to a global word at offset 3706, before calling another routine.
- `calculate_structure_offsets`: Calculates and stores three 16-bit values into a structure at offsets zero, two, and four, using values derived from two global variables and applying bounds checks.
- `calculate_structure_offsets_alt`: Calculates and stores three 16-bit values into a structure at offsets zero, two, and four, using different arithmetic constants and bounds checks compared to another routine.
- `process_table_entries_3`: Compares a byte in a global variable to one, iterates through a table using a0, performs several conditional checks on table entries, and if conditions are met, calls one routine, then calls another routine using a byte from a global pointer, then calls a third routine using a byte from another global pointer, and finally calls a fourth routine.
- `increment_and_call_2`: Increments d7 and compares it to 32, calls another routine if it is less than 32, sets the sign bit of d5 based on the result, and restores registers from the stack before returning.
- `execute_conditional_logic_10`: Saves registers to the stack, calls another routine, and if the result is not negative, calls another routine; if that routine returns a non-zero value, it checks if a byte from a global pointer is 13 and if a byte at offset 1 of that pointer is 20, otherwise it calls another routine, stores a longword from a global pointer into a global variable, calls another routine, and restores the global variable.
- `execute_and_restore`: Calls a routine, sets the high bit of d7, and restores d0, d1, and d7 from the stack before returning.
- `lookup_and_call`: Saves d0 and a0, loads a byte from a table using a0 and d0, and if that byte is not zero, calls another routine, masks d0, calls another routine, and sets d0 to 255 before returning.
- `check_attribute_conditions`: Calls two routines and, if the results meet specific criteria involving a byte from a global structure and specific bit tests, it sets a global byte to 255, calls another routine, and then sets the global byte to zero.
- `perform_conditional_sequence`: Calls another routine, and if the zero flag is set, calls another routine before returning.
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
- `dispatch_complex_blit_operation`: Jumps to another routine that performs two separate blitter operations using parameters in d0 and d1, updates a global variable, and calls execute_complex_blit.
- `process_scaled_tile_loop`: Iterates through a table using a loop, performing a series of calls to update structure fields, modify bytes based on table lookups, and execute three consecutive callbacks for each entry.
- `call_and_jump_duplicate`: Calls another routine and then jumps to another routine.
- `update_scaled_value_2`: Calls a routine, checks if d0 is non-zero, if so it extends d0 to a word, stores it in a global variable, adds it to a pointer in a0, checks a byte at that pointer, and if non-zero clears it, calls two routines, extends d0, stores it in a global variable, loads an address into a0, calls another routine, and branches if the global word is non-zero.
- `set_global_pointers`: Writes two words to a global structure, fetches a byte from a table, calls a routine, and then swaps two global words before jumping to another routine.
- `transfer_word_and_jump`: Loads a word from a global variable into a location pointed to by a register, loads another word into the next location, and jumps to another routine.
- `swap_byte_at_offset`: Calls a routine, and if d0 is non-zero, extends d0 to a word, stores it in a global variable, calculates an address by adding d0 to a global pointer, and if the byte at that address is zero, stores a byte from a global variable there and clears that global variable; otherwise, it swaps the byte at that address with a global variable.
- `swap_and_store_byte`: Moves a byte from a source to a destination, extends it to a word, stores it in a global variable, and calls three different routines.
- `branch_with_word_load`: Loads a word from a global pointer into d0 and branches to another location.
- `process_conditional_pointers`: Sets a global variable to zero, calls a routine, and if a global byte is zero, calls two other routines using pointers retrieved from a table and then calls a fourth routine based on a bit test.
- `set_d0_and_branch_2`: Loads the value nineteen into d0 and branches to another routine.
- `execute_handler_sequence_7`: Loads the value 18 into d0, calls two routines, and then jumps to another routine.
- `execute_sequence_of_calls_5`: Loads a byte from an offset of a structure into d0, then calls three consecutive routines before performing an indirect jump.
- `initialize_handler_state`: Stores the value 3 in a global word, then calls three routines and jumps to another routine, provided that a global pointer and a value at an offset of 38 from that pointer are both non-zero.
- `modify_status_flags`: Calls a routine to retrieve a pointer, then conditionally sets bit 7 at offsets 0, 6, or 12 of a structure in a0 based on the value of a byte at a global address and a byte in a1.
- `set_status_bit_and_call`: Calls another routine, and if the zero flag is set, sets bit 7 of a byte at offset 6 of a structure in a0, then calls a second routine.
- `dispatch_conditional_routines`: Calls two routines, checks a byte at a global address, and if that byte is non-zero, calls a third routine before branching to another location.
- `load_and_dispatch_words`: Sets the sign bit of a byte at a global address, loads two words from a structure into global variables, and if the third word is not zero, writes it to a global variable and calls two other routines.
- `program`: Calls an initialization routine, a main routine, and an exit routine before returning.
- `check_attribute_and_call_2`: Tests a byte at a global address, branches if non-zero, compares a word at a global address to 8, and if it is not greater than or equal to 8, sets d0 to 176 and calls a routine before returning.
- `process_attribute_list_and_calculate`: Iterates through a list of six bytes, calling two different routines for each non-zero byte to retrieve and process data.
- `calculate_scaled_value_19`: Performs arithmetic on d7 using a subtraction, a multiplication, and a division, then branches to another instruction.
- `set_register_and_clear_global`: Sets d0 to 24, calls a routine, sets d7 to 95, clears a word in a global variable, and branches to another instruction.
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
- `execute_hardware_call_and_jump`: Calls a routine via a pointer at a fixed address and then performs a long jump to another routine.
- `set_status_bit`: Checks a byte in a global variable; if it is not zero, it sets bit 7 of a byte at offset 12 of a structure pointed to by a register.
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
- `call_callback_handler`: Calls a routine at a specific address and then returns.
- `process_nested_tile_data`: Checks if the high byte of a parameter is non-zero, then calls another routine to retrieve a pointer, performs arithmetic involving multiplications by 39 and 7 to calculate offsets into a table, calls a routine to process a range, and finally calls another routine before resetting two global 16-bit values.
- `modify_control_flags`: Sets d0 to 255, compares a byte at a global address to 2, and if it is not less than 2, checks a byte at a global address; if that byte is non-zero, it clears bits 4 and 5 in d0 before branching.
- `modify_global_flags`: Checks bits in a global variable to clear specific bits in register d0, then calls several routines to update a global word and perform scaled blitting operations.
- `call_conditional_routine_3`: Loads 224 into d0, 125 into d1, and 30 into d3, calls a routine, and if the result is not zero, calls another routine before returning.
- `toggle_global_bit_field`: Checks if the high byte of d0 is non-zero and if a bit is set in a global byte, then retrieves a byte from a table, performs a bitwise change on a global word, and calls two other routines.
- `validate_and_process_tile_index`: Checks if a high byte in a register is non-zero and if a corresponding bit is set in a global variable, then performs arithmetic on a value, updates two global words, and calls three different routines.
- `branch_on_bit`: Checks a bit in a global variable and, if it is not set, loads a constant address into a0 and branches to another location; otherwise it branches to another location.
- `reset_global_value`: Loads a pointer into a0, calls another routine with a constant in d0, and then sets a global variable to zero.
- `loop_and_set_global`: Saves registers, calls another routine, and then enters a loop that checks bits in a byte from a global variable to set a global word to either 28 or 31.
- `loop_and_test_bits`: Writes 31 to a global word, loads a1 into a0, sets d0 to 65, calls a routine, updates a1 with a0, loads a byte from a2 into d0, clears bit 7 of d0, and performs conditional branches based on d0 and bit 2 of d2.
- `branch_on_bit_alternative`: Checks a bit in d2 and, if it is not set, loads a constant address into a0 and branches to another location; otherwise it branches to another location.
- `call_and_loop_counter`: Loads a pointer into a0, calls another routine, and then enters a loop that increments d1 and d7, returning when d7 reaches 6.
- `update_data_from_buffer`: Checks a byte at offset 2 of a buffer; if the lower two bits are not 2 or 3, it stores a byte from the buffer into a global word, then optionally calls another routine twice to store results into global bytes at offsets 34 and 35.
- `store_dual_values`: Copies the value from d2 into a global variable via a routine call, then repeats the process with a different constant in d1 to store a second value in another global variable.
- `call_two_routines_2`: Pushes a longword from the stack, calls two different routines using bytes from global variables, and then restores the stack.
- `process_and_clear_buffer`: Retrieves a pointer via a routine, then enters a loop that clears 70 consecutive bytes starting at that pointer.
- `execute_multiple_subroutines_5`: Sets three global words to zero, calls two routines, and if a byte in a global variable is zero, performs a sequence of calls and pointer retrievals.
- `call_with_constant_3`: Loads the value 119 into d0, calls another routine, and returns.
- `execute_conditional_blits_2`: Loads a byte from a global variable, calls two routines, and if two specific global bytes are both zero, calls three more routines using bytes from three different global locations before jumping to another routine.
- `dispatch_conditional_metadata_updates`: Checks if a global pointer and a value at offset 38 of its structure are non-zero, then calls process_data_structure, initialize_and_update_tile_metadata, and three other routines while setting a global variable to 255.
- `execute_conditional_sequence_2`: Calls two routines and then jumps to a third routine if a global pointer and a value at offset 38 of its structure are both non-zero; if the condition is met, it calls three additional routines, setting a global variable to 255 between the first and second calls.
- `update_sprite_and_sync_state`: Retrieves a structure via a pointer, updates two global words with values from offsets 24 and 26 of that structure, and performs a series of calculations and calls to update sprite or object properties.
- `test_and_dispatch_list`: Tests a byte at a global address, and if it is not zero, sets the sign bit of that byte, calls three other routines, and jumps to another routine.
- `update_structure_flags_4`: Calls two different routines, sets various bits in a structure based on the results, and then performs several conditional bit sets in the structure based on values retrieved from a table and global variables.
- `process_event_callback_chain`: Loads a byte from a global structure, calls a routine with a constant in d7, and if the result is zero, checks a byte in another global structure to decide whether to call a sequence of three routines or a different sequence of three routines based on the status of two additional bytes in the global structure.
- `dispatch_conditional_callbacks`: Calls four routines in sequence, tests a byte at a memory location, and if it is zero, compares a word from a memory location to negative 1, calling another routine if the word is not negative 1.
- `dispatch_conditional_call_2`: Calls a routine and, if the result is not negative, sets d0 to 107 and calls another routine.
- `initialize_and_dispatch_event`: Writes several values to a structure pointed to by d6, calls two routines, and then conditionally calls a third routine if a value in d0 is not negative, followed by a check of a byte in a global variable to decide whether to call a final routine.
- `execute_multiple_subroutines_3`: Loads the word 114 into d0 and calls five different subroutines in sequence before returning.
- `initialize_event_structure_3`: Sets several global values, clears a word at offset 24 of a structure in a0, calls two routines, and performs conditional logic based on the result of a routine call and a global byte.
- `branch_to_subroutine_3`: Loads the word 114 into d0, calls a routine, and then branches to another location.
- `call_subroutines_conditionally`: Tests a byte at a global address and, if it is zero, calls four subroutines in sequence; otherwise, it calls four subroutines after branching.
- `dispatch_event_data_2`: Pushes a value to the stack, retrieves a byte from a global variable to use as an index for a routine call, and then uses the resulting pointer to check a bitmask.
- `test_bit_and_call_routine_3`: Tests bit 1 of a register; if it is not set, it loads 99 into a register and calls another routine, then branches to another location; otherwise it branches to another location.
- `test_bit_and_call_routine_2`: Tests bit 10 of a register; if it is not set, it loads 100 into a register, calls another routine, and then branches to another location, otherwise it branches to another location.
- `check_and_call_routine_2`: Tests a word at a global address and, if it is zero, branches to another location; otherwise, it loads 97 into d0, calls another routine, and branches to another location.
- `dispatch_event_sequence`: Calls a routine, and if the result is positive, calls another routine with a constant in d0.
- `compare_and_call_10`: Compares a word at a global address with 6, and if it is less than 6, loads 121 into d0, calls another routine, and then branches to another location.
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
- `test_and_call_callback`: Tests a byte at a global address and branches if non-zero, otherwise calls another routine and branches.
- `call_structure_field_initializer`: Calls initialize_structure_field and then returns.
- `setup_structure_and_call`: Pushes a0 to the stack, copies a long value from a global address to another global address, clears a global long, writes a constant to a global long, loads a pointer into a0, writes a word to the address in a0, clears a word at offset 18 of the structure in a0, calls another routine, restores a0 from the stack, and returns.
- `execute_callback_loop_3`: Calls a routine, calls another routine, and then jumps to another routine.
- `initialize_and_update_tile_metadata`: Sets four 16-bit values in a table, calls several routines to retrieve a structure, and updates two global words with values from offsets 24 and 26 of that structure.
- `calculate_and_update_tile_data`: Calculates a value based on a structure offset and a multiplication by 6, then updates two global words with values from offsets 24 and 26 of a structure.
- `parse_blit_metadata_stream`: Calculates and stores several word-sized offsets and counts in global variables based on values read from a data stream, then iterates through the stream to perform table lookups and conditional calls to update structure values.
- `validate_and_lookup_blit_metadata`: Checks if a value in d2 is within bounds, then uses it to index into a table to retrieve a new value in d2 via a call.
- `process_blit_data_stream_metadata`: Iterates through a data stream to extract and validate metadata, performing table lookups and conditional calls to modify structure values based on bitmask checks and specific byte values.
- `execute_three_routines_3`: Calls three consecutive routines and then branches to another location.
- `check_bits_and_set_d2`: Loads a byte from offset 2 of a structure in a0, masks it with 3, and compares it to 2.
- `process_blit_data_stream`: Checks a bit in a global variable and, if set, executes a loop that iterates up to 32 times, calling another routine to process data and updating a pointer in a0 by an offset.
- `process_loop_with_callbacks`: Compares a global word to 6, otherwise enters a loop that calls another routine and checks its result, performs a second routine call with specific values in d0, d1, and d2, and then clears two global words before jumping to another routine.
- `perform_sequential_updates`: Loads three bytes from consecutive global addresses into d0, calls a routine for each byte, calls another routine, and then jumps to another routine.
- `execute_three_callbacks`: Loads the value 11 into d0, then calls three consecutive routines via pointers before jumping to another routine.
- `process_table_and_jump`: Calls two routines, checks if a global word is greater than or equal to 250, clears a global word if so, increments a global word by 4, and if a specific table entry is zero, resets that global word to 0 before calling two more routines and jumping to another routine.
- `increment_global_word`: Increments a 16-bit value stored in a global variable by one.
- `calculate_table_indices`: Performs arithmetic on two global words to calculate new indices, checks if these indices fall within a specific range defined by two other global words, and if so, stores them in a global table and calls another routine.
- `call_conditional_routine_7`: Calls a routine, moves d0 to d7, tests a byte at a global address, and if it is zero, sets d0 to 51 and calls another routine before branching to another routine.
- `branch_if_not_zero`: Checks a byte at a global address, and if it is non-zero, stores the value 89 in d0, calls another routine, and then branches to another routine.
- `update_table_values_7`: Calls a routine, and if the result is non-zero, retrieves three bytes from a table at an offset calculated from a pointer in d6, stores them in a global structure, calls another routine, and then performs arithmetic on the difference of the stored values to either call another routine or enter a loop that calls a routine multiple times.
- `divide_register_value`: Extends the 16-bit value in d3 to a 32-bit value, divides it by 4, and branches to another location.
- `execute_looping_calls_4`: Decrements a counter in d3 in a loop, calling two different routines during each iteration before returning.
- `execute_conditional_calls_3`: Calls another routine, checks if d0 is positive, and if not, calls another routine before branching to another location.
- `compare_and_call_2`: Compares a word in d0 with a value from a global pointer, and if they are not equal, calls another routine and branches to another location.
- `update_tile_offset`: Calls a routine, stores the result in a global variable, pushes d0 to the stack, loads a byte from a global address, calls another routine, calculates an offset by multiplying d0 by 20 and adding it to a global pointer, stores the constant 27 in a global variable, and calls a final routine.
- `set_d0_to_one`: Sets d0 to 1 and branches to another location.
- `set_d0_to_three`: Sets d0 to 3 and branches to another location.
- `set_d0_to_five`: Sets d0 to 5 and branches to another location.
- `set_d0_to_seven`: Sets d0 to 7 and branches to another location.
- `dispatch_indexed_callbacks`: If d0 is non-zero, it masks d0, decrements it, and uses the result to index into a table of pointers to call a routine; if bit 0 of d0 is set, it uses a second table to call another routine, and finally jumps to another routine.
- `decrement_global_word_conditionally`: Decrements a 16-bit global variable by one if a byte in a global variable is non-zero and the 16-bit variable itself is non-zero.
- `increment_global_word_range`: Increments a 16-bit global variable by two if a byte in a global variable is non-zero and the variable is less than the difference between another 16-bit global variable and nineteen.
- `increment_global_word_limit`: Increments a 16-bit global variable by one if a byte in a global variable is non-zero and the variable is less than the difference between another 16-bit global variable and thirteen.
- `decrement_global_word_two`: Decrements a 16-bit global variable by two if a byte in a global variable is non-zero and the 16-bit variable is non-zero.
- `process_callbacks_and_jump`: Calls two routines and then jumps to another routine.
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
- `check_blit_parameters`: Compares d0 to 8, and if not equal, checks bit 1 of a byte at a global address; otherwise, it branches to another location.
- `process_scaled_blit_parameters`: Checks bits in d0 and a bit in a table, then calculates a destination address using a multiplication, calls a routine, and performs a loop that calls another routine and a third routine while updating d0 and d1.
- `call_decrement_structure_byte`: Loads a 32-bit pointer from a global variable into d0, calls decrement_structure_byte, and restores the previous register state before returning.
- `perform_complex_bitmasking`: Checks d2 against several bounds, and if valid, executes a loop that performs multiple routine calls, bitwise AND operations between a global byte and a table value, and writes results to a calculated address.
- `fetch_and_store_byte_2`: Calls another routine using a value from a global pointer and a constant in d1, stores the resulting byte in a global variable, and sets the sign bit of a global variable.
- `execute_two_routines_4`: Pushes d0 to the stack, calls a routine, loads a byte from a global pointer into d0, calls another routine, and returns.
- `process_structure_data_8`: Checks a global byte, and if non-zero, calls two routines using values from a structure, then updates the global byte.
- `set_buffer_bits`: Checks a global byte, calls another routine to retrieve a pointer, and then executes a loop to add values from two tables to d0 and d1 before calling two more routines.
- `set_bits_in_buffer`: Performs two nested loops to call a routine for bit manipulation and then calls another routine to clear a byte from a global address.
- `compare_and_set_byte`: Compares a word at a global variable location to six; if not equal, it branches to another routine, otherwise it sets a register to two hundred fifty-five and branches to another routine.
- `calculate_and_test_bit`: Calculates an index using a multiplication, retrieves a pointer, calls another routine, and then tests a bit in a byte retrieved from the pointer's offset.
- `set_bit_in_buffer`: Calculates a bit position and index within a buffer based on two input values and two global width/height values, then sets that bit in the buffer and updates a global variable if the bit was not already set.
- `calculate_and_store_data`: Clears a global word, calls another routine to get a pointer, loads a word from a structure at an offset, calculates a pointer into a table using a global word and multiplication, and enters a loop that tests bits and writes a four-byte sequence to a destination based on several conditional checks.
- `test_bit_and_branch_2`: Loads a word from a global address into d4, adds 5 to it, tests bit 4 of d0, and if bit 4 is not set, sets d0 to 2 and branches to another location; otherwise it returns.
- `compare_and_branch_byte`: Loads a byte from offset 1 of the structure in a1 into d0, and if d0 is zero, branches to another location; otherwise, it compares d0 with d6 and branches to another location if d0 is negative, or sets d0 to 3 and branches to another location.
- `calculate_and_fill_data`: Divides d6 by d0, compares the result to 100, and if it is not less than 100, enters a loop that writes four bytes to a destination pointed to by a0, increments pointers, and conditionally calls two routines if a loop counter reaches 31.
- `call_update_scaled_parameters`: Calls update_scaled_parameters and then returns.
- `update_scaled_parameters`: Sets register values and calls two different routines, retrieves a word from a table using a value from a global variable and an offset, stores that word at offset 18 of a structure in a0, calls another routine, and jumps to another routine.
- `process_list_element`: If a global word is non-zero, it calls another routine, loads a word from a global variable into d1, adds d1 to itself, calls another routine, stores the result from d0 into a memory location at offset -2 from a pointer in a0 plus d1, and calls a final routine before returning.
- `process_blit_sequence`: Loads several constants into d0 through d4 and calls another routine, then jumps to another routine.
- `process_data_sequence`: Loads several constants into registers, calls another routine, calculates a difference between two global values, and if the difference is greater than 16, sets a register to 16, loads two addresses into a1 and a2, shifts d0 left by 2, adds the result to a1, and branches to another routine.
- `branch_on_byte_test`: Tests a byte in a1 and branches; otherwise it writes the value 31 to a global word and branches.
- `write_scaled_data_sequence`: Writes a constant to a global word, performs a series of calls to process d0, and enters a loop that writes a sequence of bytes and a zero byte to a buffer at a0 based on a counter in d2.
- `check_and_store_tile_byte`: Shifts d0 right by 8 bits, and if non-zero, checks if a calculated address is within bounds and points to a zero byte, then stores a byte from that address into a global variable, performs a routine call, sets two global variables, performs a routine call with a calculated value, and calls three more routines.
- `setup_clipping_parameters`: Checks if the high byte of d0 is non-zero and within a specific range, then calculates an address in a table, and if the byte at that address is non-zero, sets several registers and calls another routine.
- `compare_and_branch_7`: Compares a byte at the address in a0 to one; if they are not equal, it branches to another location, otherwise it sets d3 to 27 and branches to another location.
- `branch_with_constant`: Loads the constant 241 into d3, calls a routine at a pointer, and then branches to another location.
- `setup_blit_parameters_2`: Subtracts one from a byte at offset 3 of a structure in a0, calls another routine, sets two words at a global address to 0 and 65535, calculates new values based on d7 and d1, calls another routine, and finally sets a global word to 0.
- `calculate_index_and_call_2`: Saves registers, calculates an index into a table using two global words, retrieves a byte from that table, and if bit 7 is clear and bit 6 is set, performs a series of operations involving setting global words, calling a routine, and potentially calling another routine with a shifted d0 value.
- `update_status_and_call_2`: Shifts d1 left by 8 bits and stores it in d0, then calls a routine, stores a byte from a pointer in a1 into a global variable, sets a global word to a constant, loads a pointer from a global address into a0, calls another routine, and if d0 is not negative, stores d0 in a global variable and stores two words from a global address into two global variables before branching to another location.
- `test_and_set_global`: Tests bit 0 of d0 and, if set, checks a byte at a global address; if that byte is non-zero, it writes the word 284 to a global address and branches.
- `set_event_parameters`: Stores two values into global memory, calls a routine, and then calculates a value using a bit shift and stores it in a global word.
- `execute_indirect_calls_2`: Stores a value in a global variable, loads a pointer from a table using an offset in d0, and if the pointer is non-zero, calls it; then tests a global byte and calls another routine if non-zero.
- `fetch_byte_from_table_conditionally`: Checks two global bytes for zero; if both are non-zero, it loads a pointer from a global address, retrieves a byte from an offset of 27 from that pointer into a register, and branches to another routine; otherwise, it branches to one of two other locations.
- `execute_short_sequence`: Loads a word from a global address into d0, calls another routine, sets the sign bit of a byte in a global variable, and branches to another routine.
- `calculate_and_update_structure_3`: Calculates an offset into a global table using values from two global words, retrieves a byte from that table, and calls another routine.
- `call_with_parameter`: Loads the word 248 into d0, calls a routine, restores registers from the stack, and checks a byte in a global variable before returning.
- `execute_callback_loop`: Iterates through a list, checking a bitmask against a global value, and if a bit is set, calls a routine, executes a function pointer from a table, and calls another routine.
- `validate_graphics_and_decrement`: Saves registers, calls a routine to retrieve a pointer, calculates an offset into a structure, checks a bit in a table, and if a byte at offset 1 of a structure is not 255, decrements it before calling two more routines and performing several comparisons.
- `validate_and_execute_conditional_logic`: Fetches a byte from a global pointer, calls two routines, and checks if the byte equals five and if two values in a table are equal.
- `update_graphics_parameters_2`: Subtracts 40 from d0, subtracts 9 from d0, calls two routines, moves d2 into d0, calls a third routine, and restores registers from the stack before returning.
- `call_handler_and_execute_callback`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0, calls another routine, and then calls a third routine before returning.
- `check_structure_condition`: Checks if bit 14 and 15 are set in a word at offset 30 of a structure in a0, and if so, retrieves a pointer from a global variable, fetches a byte from a table, and if that byte is non-negative, performs a division and calls two other routines.
- `call_handler_and_load_address`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0, calls another routine, and then calls a third routine before returning.
- `clear_bit_in_word`: Checks if specific bits are set in a word at offset 30 of a structure; if not, it retrieves a byte from a global pointer and clears the corresponding bit in the word.
- `call_handler_and_branch`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0 and then performs a branch to another location.
- `call_two_routines_4`: Loads a fixed address into a0, then calls two routines in sequence before returning.
- `test_bits_and_call`: Loads a word from offset 30 of the structure in a0 into d0, tests bit 15 of d0, and if bit 15 is set, branches to another location; otherwise it tests bit 14 and if bit 14 is clear, sets d0 to 250 and calls a routine before branching to another location.
- `test_bit_and_call_5`: Tests bit 13 of d0, and if it is set, branches to another location; otherwise, it sets d0 to a constant and calls a routine before branching to another location.
- `set_structure_flags_3`: Clears bit 13 of d0, stores d0 at offset 30 of the structure in a0, stores the word 1 at offset 202 of the structure in a0, calls a routine, loads 57 into d0, calls another routine, and then calls a third routine using d7 in d0.
- `modify_status_bit`: Loads a word from offset 30 of the structure in a0, checks bit 13, and if it is set, clears bit 13, sets bit 14, stores the result back to offset 30, and calls a routine.
- `call_handler_and_branch_address`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0 and then performs a branch to another location.
- `load_address_and_call`: Loads a fixed address into a0 and calls a routine before returning.
- `modify_word_and_call`: Loads a word from offset 30 of the structure in a0 into d0, tests bit 14 of d0, and if it is set, branches to another location; otherwise, it sets d0 to a constant, calls a routine, and branches to another location.
- `set_status_bits_5`: Clears bit 14 of d0, sets bit 13 of d0, stores the word in d0 at offset 30 of the structure in a0, and calls a routine.
- `modify_status_bit_2`: Checks bit 14 of a word at offset 30 of a structure in a0; if set, it clears bit 14 and sets bit 15 of that word before calling another routine and returning.
- `call_handler_and_execute_callback_alt`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0 and calls another routine before returning.
- `modify_word_and_call_alt`: Loads a word from offset 30 of the structure in a0 into d0, tests bit 15 of d0, and if it is set, branches to another location; otherwise, it sets d0 to a constant, calls a routine, and branches to another location.
- `clear_and_set_bits`: Clears bit 15 and sets bit 14 of a word at offset 30 of a structure in a0, then calls another routine and returns.
- `call_routine_and_branch_conditionally`: Calls a routine and, if the zero flag is set, branches to a different location; otherwise, it loads a global address into a register and performs an unconditional branch.
- `call_two_routines_alt`: Loads a fixed address into a0, then calls two routines in sequence before returning.
- `check_bits_and_call_3`: Loads a word from offset 30 of the structure in a0 into d0, performs a bitwise AND with a constant in d1, and if the result is zero, branches to another location; otherwise, it tests bit 15 of d0, and if set, branches to another location; then it tests bit 14 of d0, and if not set, it sets d0 to a constant, calls a routine, and branches to another location.
- `update_structure_values_17`: Clears bit 13 of a word at offset 30 of a structure in a0, adds two words from offsets 204 and 206 of the structure to a third word at offset 202, then calls three different routines.
- `toggle_status_bits_2`: Checks bit 13 of a word at offset 30 of a structure in a0; if set, it clears bit 13 and sets bit 14 of that word before calling another routine and returning.
- `process_conditional_callback_chain`: Calls a routine, and if the zero flag is not set, loads a pointer from a global address and calls two subsequent routines in sequence before returning.
- `update_structure_on_bit_set`: Checks if a word at offset 30 of a structure has specific bits cleared and bit 6 set; if so, it calls another routine twice and then calls a third routine.
- `call_handler_and_branch_conditional`: Calls a routine, and if the zero flag is set, calls another routine, otherwise it performs a branch to an earlier location.
- `process_pointer_table_data`: Retrieves a byte from a table using a pointer and index, calls another routine with that byte, and then calculates an offset into a structure to perform arithmetic and call two more routines.
- `compare_and_call_8`: Subtracts a byte from offset 1 of a structure in a0 from a byte at offset 28 of a structure in a1; if the result is positive, it pushes d0 to the stack, calls another routine, restores d0, sets d0 to 257, and calls another routine.
- `calculate_and_update_byte`: Calls two subroutines, compares a byte from a structure in a2 to d0, and if d0 is not positive, performs a multiplication, swaps the result, and adds it to a byte at offset 1 of a structure in a0.
- `call_handler_and_update_table`: Calls a routine, and if the zero flag is not set, subtracts two from a word loaded from a global variable, uses the result to index into a table to load two bytes into d1 and d0, and then calls two routines before returning.
- `execute_conditional_callback_chain`: Calls a routine via a pointer, and if the zero flag is set, loads a pointer from a global address and calls two subsequent routines in sequence before returning.
- `increment_structure_values`: Checks if a word at offset 30 of a structure masked with 58496 is zero and if a word at offset 26 is not 32767; if both conditions are met, it increments the word at offset 26, adds 250 to a longword at offset 270, and calls another routine.
- `call_conditional_handler_2`: Calls a routine and, if the zero flag is set, loads a new address into a0 and calls another routine before returning.
- `process_graphics_list_2`: Checks bits 13 and 14 of a word at offset 30 of a structure in a0, and if they are clear, performs a sequence of routine calls, manipulates a global byte, and enters a loop that iterates up to 10 times to process structure data and call routines.
- `process_attribute_updates`: Calls a routine and, if the zero flag is set, performs several operations including loading values from a structure, calling two different routines, and conditionally updating two words in a structure based on bit tests.
- `call_with_constant_4`: Loads the value 66 into d0 and calls another routine.
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
- `branch_on_handler_result`: Calls a routine and, if the zero flag is not set, calls another routine and then branches to another location.
- `process_table_and_bit`: Retrieves a byte from a table using an index in d7 and a pointer in a0, calls another routine, calculates an offset by multiplying a word from a global address by 6 and adding it to a new pointer, then checks bit 1 of a byte at that offset and calls another routine if the bit is clear.
- `clear_bit_and_call_subroutines`: Clears bit 1 of a byte at offset 3 of the structure in a0, then calls two subroutines using a global address and a value from a global address.
- `branch_on_handler_result_alt`: Calls a routine and, if the zero flag is not set, calls another routine and then branches to another location.
- `update_structure_and_call_3`: Retrieves a byte from a table using an index from a global word, calls a routine, calculates an offset into a structure, checks a bit in a table, and if bit 6 is clear, calls another routine and then calls a routine with the value 269 in d0.
- `update_structure_pointers_7`: Copies a pointer and a word from a structure to another location, calls two routines, sets a byte to one, and then calls three more routines based on a conditional check.
- `call_with_constant_large`: Loads the value 272 into d0 and calls another routine.
- `update_structure_and_call_7`: If a condition is met, it retrieves a byte from a table using a value from a global structure, calls a routine with that byte, calculates an offset into a structure using a value from a global word multiplied by six, sets bit 0 of a byte at that offset, copies a pointer and a word to a new location, calls two more routines, and finally calls a routine with a pointer to a global address.
- `fetch_table_byte_and_call`: If a condition is met, it retrieves a byte from a table at an offset derived from a global word and then calls a routine with d7 set to five.
- `process_conditional_structure_data`: If a condition is met, it retrieves a byte from a global structure, calls a routine with that byte, then calls another routine with the value from a structure, and if bit 3 of that value is zero, calls a routine with the constant 440.
- `process_conditional_call`: Calls a routine, compares the result to five, and if equal, calls another routine before calling a third routine and returning.
- `call_global_address_handler`: If a condition is met, it loads a global address into a0 and calls a routine.
- `initialize_scaled_blit_parameters`: Sets d0 to 48, d1 to 64, d2 to 4, d3 to 4, and clears a0, then calls another routine; subsequently sets d0 to 64, d1 to 80, d4 to 24, d5 to 3, d6 to 2, and d7 to 32, loads a table address into a0, and calls another routine; if a byte at a global address is not 2, it loads a word from a global address into d3, increments it, multiplies it by 48, subtracts 13, and branches to another location.
- `calculate_scaled_offset_8`: Performs arithmetic on a global word in d3, calls a routine, sets two global variables, performs a routine call with a value derived from d3, clears a global variable, and calls another routine before jumping to another routine.
- `compare_and_call_routine_17`: Compares a byte at a global address with 2, and if the result is not negative, loads 258 into d0, calls a routine, and branches to another location.
- `load_table_and_call_routines`: If a condition is met, it retrieves a byte from a table using an index derived from a global word, calls a routine, and then calls another routine with a global address loaded into a0.
- `fetch_table_byte_and_call_six`: If a condition is met, it retrieves a byte from a table using an index derived from a global word minus thirteen, and then calls a routine with d7 set to six.
- `call_two_routines_with_constants`: If a condition is met, it calls a routine with d1 set to 180 and d7 set to five, then calls another routine with d7 set to six.
- `execute_conditional_call_2`: Tests a longword at a global address and, if the lower word is zero, loads 283 into d0, calls another routine, and branches to another location.
- `check_byte_and_call_2`: Compares a byte at a global address with 1, and if the result is equal, loads 244 into d0, calls a routine, and branches to another location.
- `execute_conditional_sequence_3`: If a condition is met, it calls two routines with d0 set to 46, and if the result is two, it sets a global word to one, calls a routine, and calls another routine with d0 set to five.
- `call_two_routines_from_address`: If a condition is met, it loads a global address into a0, calls a routine, and then calls another routine.
- `calculate_and_verify_offset`: Checks if bits 14, 15, and 16 of a word at offset 30 of a structure in a0 are zero; if so, it calls another routine, calculates a new value for a word at offset 102 of the structure based on a multiplication and subtraction, calls another routine, and if a global byte and a global word match specific values, calls two more routines.
- `call_address_handler_from_list`: If a condition is met, it loads a global address into a0 and calls a routine.
- `execute_multiple_blits`: Clears a 16-bit value in a global variable, then performs three separate calls with specific register parameters and finally jumps to another routine.
- `initialize_callback_parameters`: Writes a word to a global variable, sets several registers to constant values, calls another routine, and then conditionally sets d7 and d1 based on a value retrieved from a global pointer.
- `loop_and_draw`: Loads a pointer into a0, calls another routine, sets d0 to 36 and calls another routine, then enters a loop that increments d1 and d2 and decrements d7 until d7 reaches negative 1.
- `dispatch_list_processing_task`: Calls pop_and_process_list and then calls decrement_and_update_pointer if a global pointer is not equal to a specific address and the value at the decremented pointer location is non-zero.
- `test_and_call_routine_2`: Sets d0 and d1 to specific values, calls a routine twice, and if the result is non-zero, calls another routine before returning.
- `perform_scaled_operations`: Shifts d0 right by 8, checks if d0 is zero, adds a value from a global pointer to d0, stores d0 in a global word, performs several routine calls with various parameters, and finally calls another routine using d0 as an index.
- `execute_conditional_blit`: Checks if the high byte of d0 is non-zero and if adding a value from a global pointer to d0 results in a value less than 32, then performs a series of calculations, calls three routines, and sets two global words.
- `call_handler_and_process_structure`: Calls a routine and, if the zero flag is not set, loads a pointer from a global address into a0, then calls two subsequent routines.
- `process_structure_value`: Checks if bit 14 and 15 are set in a word at offset 30 of a structure in a0, retrieves a byte from a table using a value from a global pointer, and if a word at offset 210 of the structure is non-zero, performs a division and calls two other routines.
- `call_handler_and_load_address_2`: Calls a routine and, if the zero flag is not set, loads a pointer from a global address into a0, then calls two subsequent routines.
- `reset_and_calculate_sum`: Clears a word at offset 30 of a structure, calculates the sum of two words at offsets 204 and 206 of that structure, stores the result at offset 202, and jumps to another routine.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_conditional_callbacks_7`: Calls a routine and, if the zero flag is not set, loads a pointer into a register and calls two subsequent routines before returning.
- `update_table_entry`: Checks if a word at offset 30 of a structure masked with 58496 is zero; if so, it calls another routine, calculates a value based on the result, compares it to a word at offset 2 of a table, and updates the table entry if the value is smaller, then calls another routine.
- `validate_and_update_attribute_table`: Validates two 16-bit values against a table, checks if a byte at a specific offset matches two constant values, and if so, calls another routine with a constant value in d0.
- `process_table_and_loop`: Calls another routine, checks a byte in a global variable, and if it is at least 2, calls another routine, performs several stack operations, calls three different routines in a loop, and finally calls another routine and stores 255 in a global variable.
- `update_conditional_bytes`: Calls a routine, stores a byte at offset 13c of a global variable, and then calls another routine if the status flag is set.
- `execute_conditional_callback_sequence_2`: Calls a routine and, if the zero flag is not set, calls two additional routines in sequence after loading a pointer from a table into a0.
- `set_structure_bit_2`: Checks if a word at offset 30 of a structure masked with 58496 is zero; if so, it sets bit 5 of that word and calls another routine.
- `update_table_with_offsets`: If a global byte is zero, it calls two routines, calculates two offsets using global words and bit shifts, stores these and a byte into a structure, and then enters a loop of 24 iterations that increments d0 and d1 while storing a longword and calling a routine.
- `update_graphics_and_coordinates`: Retrieves a byte from a table, calls several routines to perform arithmetic and table lookups, and then calculates a pointer to a structure to copy two words into it before calling another routine.
- `update_table_values`: Compares two 8-bit values against a 16-bit value in a table at an index derived from a register, and if the comparison passes, writes the two 8-bit values into the table.
- `initialize_and_process_scaled_dimensions`: Sets two 16-bit values and clears four 16-bit values in global memory, then branches based on a byte value to either scale two 16-bit values by doubling them or copy them directly into a destination, before calling several routines and performing bitwise operations on global variables.
- `update_scaled_values_4`: Doubles two words from a global table, stores them in another location, calls a routine, and then performs four calls to another routine while passing bytes from a structure in a6.
- `calculate_scaled_offsets_3`: Stores two words from global memory into another location, calls three routines, and then calculates and stores two different values based on comparisons of the original words.
- `process_structure_flags`: Loads a byte from a global address into d0, calls another routine, calls a third routine, and then jumps to another routine.
- `setup_pointer_swap`: Loads specific values into d0, d1, d2, and d3, then calls two routines and jumps to another routine.
- `update_clipping_and_counters`: Calls a routine with a pointer from a table, performs several calculations involving global values and bit tests, updates a counter in a global variable, and calls two more routines before returning.
- `dispatch_function_table`: Extracts an index from the high byte of a register, calls a routine from a table at that index, and if the returned value has bit zero set, calls a second routine from a different table at the same index.
- `decrement_global_word_two_conditionally`: Decrements a 16-bit global variable by two if a byte in a global variable is non-zero and the 16-bit variable is at least two.
- `decrement_global_word`: If a global byte is non-zero, it decrements a 16-bit value in a global variable by 2, provided the result is not negative.
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
- `sync_and_update_table`: Calls three routines and then jumps to another routine.
- `decrement_and_skip_element_counters`: Iterates through a table of structures, decrementing a byte at offset 6 of each structure and skipping to the next entry if the byte reaches zero, then performs a second pass to skip entries where a byte is non-zero.
- `check_bits_and_jump`: Calls one routine and then jumps to another routine.
- `execute_multiple_callbacks_5`: Calls two routines and then jumps to another routine.
- `process_event_data`: Retrieves a word from a global variable, calls a routine, and stores the result in another global variable.
- `conditional_callback_execution`: Tests a byte at a global address, and if it is non-zero, calls another routine, stores a byte at a global address, calls another routine, and then jumps to another routine.
- `store_byte_and_jump`: Stores a byte from a global address into memory and then jumps to another routine.
- `execute_multiple_calls_7`: Saves registers to the stack, calls four different routines in succession, and then loads a word from a global variable into d0 before branching.
- `loop_and_call_routines_3`: Stores a word from d0 into a global variable, enters a loop that calls another routine and decrements d0 until d0 reaches negative one, calls another routine, restores registers from the stack, and returns.
- `process_table_and_decrement`: Calls another routine, uses the result to calculate various offsets and values from a table, performs a loop 32 times to traverse data, updates several global longwords and words, and finally calls another routine or jumps to another routine.
- `copy_words_conditionally`: Copies two words from offsets 13572 and 13574 to offsets 2114 and 2116, then checks a byte at offset 1330; if that byte is zero, it copies two more words from offsets 1374 and 1376 to offsets 1372 and 1374.
- `copy_global_words`: Compares a byte at a global address to 1 and, if not equal, copies three words from one set of global variables to another set of global variables before branching.
- `swap_global_pointers`: Copies three 32-bit values from one set of global variables to another set of global variables.
- `copy_words_inverse`: Copies two words from offsets 2114 and 2116 to offsets 13572 and 13574, then checks a byte at offset 1330; if that byte is zero, it copies two more words from offsets 1372 and 1374 to offsets 1374 and 1376.
- `copy_global_words_inverse`: Compares a byte at a global address to 1 and, if not equal, copies three words from one set of global variables back to another set of global variables before branching.
- `swap_global_pointers_inverse`: Copies three 32-bit values from one set of global variables back to the original set of global variables.
- `update_status_flag_3`: Calls another routine, checks a byte at a global address, and if it is non-zero, checks bit 6 of a word at a global address and sets d7 accordingly.
- `update_entity_status_flags_4`: Iterates through an entity list to find a match for two specific bytes, then checks bit flags and coordinate values in a loop to update a bit in a status word at offset 30 of an entity structure.
- `process_loop_with_calls`: Pushes d0 to the stack, stores a byte at a global address, clears a 16-bit word at a global address, and enters a loop that calls a routine and conditionally updates d0 from a global word based on a byte check.
- `set_global_word_and_pop`: Sets a 16-bit global variable to 4 and pops a 32-bit value from the stack into register d0.
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
- `branch_on_byte_test_4`: Tests a byte at a global address and, if it is not zero, loads a pointer into a0 and branches to another routine.
- `jump_to_handler_6`: Loads a pointer from a global variable into a0 and jumps to the address stored at that location.
- `invert_and_jump`: Calls a routine, inverts a byte in a global variable, calls another routine, and then jumps to another routine.
- `set_structure_bits`: Checks a byte at a global address and compares another global byte to 2; if the global byte is less than 2, it sets bit 7 of several bytes at offsets 0, 2, 4, 6, 10, 12, 14, and 16 from a0.
- `set_bits_in_structure`: Sets the seventh bit of four different bytes located at offsets 2, 6, 10, and 14 from a pointer in a0.
- `dispatch_conditional_function`: Calls a routine at a pointer in a0, loads a 16-bit word from a global variable into d0, and if d0 is not equal to 4, calls another routine and branches to a different location; otherwise it calls a routine at a different pointer.
- `call_table_function_3`: Compares a byte at a global address to two, and if it is less than two, loads a pointer into a0, calls a routine, and branches to another routine.
- `call_table_function`: Shifts a 16-bit value in d0 left by 2, uses it as an index into a table pointed to by a global variable to fetch a pointer, and calls that pointer if it is not zero.
- `execute_indirect_jump`: Stores a byte at a global address and then jumps to the address contained at that same global address.
- `set_sign_flag_global`: Sets the sign flag of a global long variable and returns.
- `call_and_jump_handler`: Calls one routine and then jumps to another routine.
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
- `clear_d0_if_negative`: Tests if d0 is negative; if it is not, it sets d0 to 0 and branches.
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
- `set_d0_on_bit_test`: Tests bit 6 of d0 and, if it is not set, sets d0 to 1 before branching.
- `set_constant_word`: Writes the value 2 to a longword at a specific memory location and restores the stack.
- `process_structure_data_7`: Saves registers, checks a global byte, and if it is non-zero, performs a series of table lookups and calls to two different routines before updating a global word and calling a third routine.
- `execute_conditional_routines`: Saves registers, checks a byte in a global variable, and if it is not 1, calls a routine using a pointer from a global variable, then calls another routine with d0 set to 0, and finally calls a third routine using a pointer from a global variable before branching.
- `load_scaled_table_pointer`: Retrieves a byte from a structure, calls another routine to fetch a pointer, and then uses a third routine to load a pointer from a table using a scaled index.
- `process_table_and_invert`: Iterates through a table starting at a global address, calling another routine when a byte matches a specific value, and also performs a loop that checks a byte at an offset of a table and potentially inverts it.
- `loop_and_store_byte`: Decrements d0 and branches if not zero, otherwise it inverts a byte at offset -1 of a1, calls another routine, stores d0 into a0 and increments a0 by 4, then enters a loop that increments d7 and repeats until d7 reaches 9.
- `loop_and_store_modified_values`: Decrements a value from a global word to use as a loop counter, calls another routine within a loop to modify d1, stores the result in a table at offset 2 of a0, calls another routine again, and stores a byte at offset 1 of a0.
- `search_matching_entry`: Pushes d2 to the stack, loads three words from global addresses into d0, d1, and d2, calls another routine, and then restores d2 from the stack before returning.
- `find_matching_entry`: Loads a pointer from a global address, sets d3 to 0 and d7 to 31, and enters a loop that checks if the data at the pointer matches d2, d0, and d1; if a match is found, it moves a byte from the pointer into d0 and a value from d3 into d1.
- `decrement_and_loop_pointer`: Increments d3, increments a0 by 6, and loops based on the value in d7 before returning.
- `execute_conditional_callback_8`: Checks a byte at a global address, and if it is not zero, stores a byte at a global address, checks if a byte at a global address is greater than or equal to 2, and if so, calls a routine.
- `execute_callback_function`: Calls a routine via a pointer and then returns.
- `execute_multiple_pointer_fetches`: Retrieves two pointers from a table via a routine, extracts bytes from the resulting structure, and uses those bytes to call a routine with constants 7 and 8 to store values in specific global variables, then calls another routine and performs a final fetch and store.
- `update_global_from_file`: Calls a routine to retrieve a byte, stores it in a global variable, and if a global byte is not one, calls another routine and stores its result in a global variable.
- `process_byte_sequence_2`: Pushes d0 to the stack, retrieves bytes from two global pointers to call a routine twice, and if a byte at a global address is non-zero, calls the routine a third time with a byte from a third global pointer.
- `call_subroutines_sequentially`: Moves a byte from a global address into d0, calls a routine, then checks a global word; if it is not 1, it moves bytes from two other global addresses into d0 and calls a routine twice.
- `execute_sequence_of_calls_8`: Calls several routines, stores a constant pointer in a global variable, calls two more routines, sets a global word to all ones, and then calls three more routines while clearing a global variable.
- `execute_nested_blit_table`: Checks a global byte for a non-zero value, then iterates through an eight by ten nested table structure, calling another routine for each non-zero byte encountered.
- `process_nested_blit_tables`: Retrieves two pointers via another routine and iterates through a nested loop structure of eight by ten iterations.
- `perform_nested_arithmetic`: Increments d0, decrements d3, increments d5, and executes a loop using d6, then modifies d1, d5, and a value at an offset from a pointer before executing another loop using d7.
- `process_nested_blit_table`: Iterates through a nested table structure using two loops, calling a routine for each non-zero byte encountered.
- `execute_nested_loops`: Calls another routine, then enters a nested loop structure that checks a bit in a value retrieved from a table using an offset derived from a word in a list, and calls another routine if the bit is set.
- `increment_longword`: Increments a longword stored at a specific memory location.
- `calculate_offset_product`: Performs a series of subtractions and a multiplication using values from three different memory locations and returns the result in d5.
- `extract_scaled_address_2`: Saves registers, tests a byte at a global address, and if it is not zero, pushes d0 to the stack, retrieves a byte from a global variable, calls a routine, and then loads a word from a global variable into d7 before branching.
- `calculate_scaled_offset_9`: Executes a loop to perform multiplications on values from a table, calculates an offset using values from two global variables and a table, calls a routine, and then calls another routine after pushing a byte from a global variable onto the stack.
- `extract_scaled_address`: Pushes a register to the stack, calls a routine, retrieves a byte from a global variable, performs bitwise operations on a value from another global variable, and if the result is zero, calculates a new value using a multiplication and an addition before branching.
- `calculate_scaled_destination`: Calculates a destination address using multiplication and an offset, loads values from a table, calls a routine, and then calls another routine using a value from the stack.
- `call_clipping_routine`: Compares d2 to 1, and if it is not 1, it pushes d2 through d4 to the stack, modifies d2 and d3 by adding 15, and calls another routine before restoring the registers and branching.
- `execute_dual_routines`: Masks d2, calls another routine, sets d5 to 5, and calls another routine before returning.
- `execute_dual_routines_alt`: Saves registers, calls another routine, sets d5 to 5, and calls another routine before restoring registers and returning.
- `process_blit_structure`: Saves registers, calculates an offset into a structure using a value from d2, clears bit 3 of a byte at offset 34 of that structure, and then performs conditional calls based on values within the structure.
- `branch_to_subroutine_2`: Calls a routine and then branches to another routine.
- `handle_blit_parameters`: Copies d0 and d1 to d6 and d7, retrieves two 16-bit words from offsets 10 and 12 of the structure in a1, and calls a routine.
- `perform_scaled_blit`: Executes a loop to increment a pointer based on a byte value, then checks a byte at a0, and if not zero, performs bitwise tests on values from a table to determine if it calls a routine with specific parameters and then calls another routine.
- `set_bit_and_blit`: If d0 equals 5, it sets bit 3 of a byte at an offset from a1, then performs a scaled blit operation and writes a 16-bit value to a global variable before returning.
- `execute_conditional_blit_5`: Saves registers to the stack, loads a byte from a table using an offset in d2, and if the byte is not negative, calls a routine and then branches to another routine.
- `loop_and_scale_blit`: Runs a loop that calculates an address by multiplying values from a table, then performs a scaled blit operation using values derived from a table and registers.
- `process_table_entry`: Uses a value from a5 to index into a table, performs bit tests and conditional jumps based on table data, and updates a pointer in a0 using a multiplication loop.
- `compare_global_values`: Saves registers to the stack, compares a word at a global address to 6, and if not equal, compares a word at a global address to a word at another global address, and if the first is not greater than or equal to the second, subtracts 4 from the value in d4 before branching.
- `calculate_clipping_bounds_2`: Stores a value in a global variable, performs two conditional calls to another routine based on calculated bounds, and then enters a loop that calls another routine while updating several registers.
- `perform_dual_calculations`: Calculates two different values using d0, d6, and d7, then calls another routine twice with these values.
- `perform_dual_calls`: Saves registers, performs two conditional checks on d1 and d3, and if both pass, calls a routine twice with different values in d0 and d2 derived from d3 and d4.
- `validate_coordinate_ranges`: Saves registers to the stack, sets d2 to negative one, compares a byte at a global address to 1, and if not equal, performs several comparisons of d0 and d1 against 1 and against values at two global addresses, and if all conditions are met, compares a word at a global address to 6 and branches accordingly.
- `check_status_bits`: Calls a routine, checks a byte at a global address, and if it is non-zero and a global word equals seven, performs bit tests on d0 and d1 to set d2.
- `check_and_update_status`: Saves registers to the stack, compares a word in a global variable to 6, and if not equal, performs several comparisons and calls a routine, then potentially calls another routine multiple times based on bit tests and a global byte, before restoring registers and returning.
- `update_status_and_counters`: Loads two bytes from a table using an offset in d7, extends them to words, adds them to d0 and d1, calls a routine, and if the zero flag is not set, updates two word fields in a global structure, checks a global byte, and if it is one, adds the extended bytes to two other global words before calling three more routines and clearing a global longword.
- `update_status_value`: Loads a word from a table, increments it if it is less than 3, decrements it if it is 3 or greater, updates a byte in a table, and calls another routine.
- `update_status_word`: Decrements d0 by 1; if the result is not negative, it adds 2 to d0 and writes the result to a global longword, otherwise it writes d0 plus 1 to the global longword and sets a global byte to 0.
- `jump_to_handler_9`: Sets d7 to 3 and jumps to another routine.
- `set_d7_to_one`: Loads the value 1 into register d7 and jumps to another routine.
- `set_d7_to_five`: Loads the value 5 into register d7 and jumps to another routine.
- `set_d7_to_seven`: Loads the value 7 into register d7 and jumps to another routine.
- `set_d7_to_zero`: Loads the value 0 into register d7 and jumps to another routine.
- `set_d7_to_four`: Loads the value 4 into register d7 and jumps to another routine.
- `set_d7_to_two`: Loads the value 2 into register d7 and jumps to another routine.
- `set_d7_to_six`: Loads the value 6 into register d7 and jumps to another routine.
- `blit_table_data`: Checks a global byte; if non-zero, it loops 32 times to compare values from a table against d2 and d4, and if they match, writes a byte from the table to a destination buffer calculated using a multiplier of 13.
- `clear_memory_block_short`: Fills 54 words starting at a specific memory location with zero using a loop, and then clears one additional byte.
- `clear_memory_block_long`: Fills 71 words starting at a specific memory location with zero using a loop, and then clears one additional byte.
- `process_pointer_data`: Fetches a pointer from a table using a value from a global variable, calls another routine, retrieves a word from offset 30 of the returned structure, and then performs a second sequence of fetching a pointer, calling another routine, and retrieving a word from the returned structure.
- `calculate_offset_increment`: Checks bit 1 of d0, and if set, loads a word from a table, adds a value derived from a global word at offset 370a shifted left by 4, and stores the result in a global word at offset 91a.
- `update_offset_value`: Checks bit 2 of d0, a global word at offset 3734, and a global word at offset 370a; if these conditions are met, it calculates a new value by shifting a word from offset 370c left by 4 and adding 32, then stores it in a global word at offset 091a.

The function list continues in: g2157-recovery-functions-2.md, g2157-recovery-functions-3.md, g2157-recovery-functions-4.md, g2157-recovery-functions-5.md.
