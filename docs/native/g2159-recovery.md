# Game 2159 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02159). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 58 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 41 of 41 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 6041 of 6041 named (0 author symbols), 6041 of 6041 described (149 by rule, 1554 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 240 of 240 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 116401 of 116405 instructions identical in start and length, 4 mismatches |

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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_BLIT#unpacked | 7 | 52210 | 2715 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_CPU#unpacked | 6 | 47660 | 2691 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_BLIT: crunched executable decrunched by XFDMaster (Imploder (xfd), 161876 -> 359116 bytes), 123 of 167 windows confirmed in reference RAM: recovered as a hunk executable
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_CPU: crunched executable decrunched by XFDMaster (Imploder (xfd), 160772 -> 347516 bytes), 121 of 165 windows confirmed in reference RAM: recovered as a hunk executable
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon: Imploder (xfd) stream found by ancient scan (5784 -> 10488 bytes) but only 0 of 6 sampled windows are in reference RAM: not used
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon_install: Imploder (xfd) stream found by ancient scan (10908 -> 22940 bytes) but only 0 of 11 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 operand
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a4]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `call_two_routines_then_jump` (More): Calls one routine with a global value and another with no arguments, then makes an indirect call through a pointer. Kind: checksum or integrity trap. Pass path: the jump to the function pointer (*unaff_A3).
- `initialize_hardware_and_copy_entry` (AM2_BLIT#unpacked): Disables DMA channels, audio/disk control, and interrupts through the custom chip registers, then fetches a value from another routine and copies 12 bytes from a table entry indexed by that value into a global variable area. Kind: checksum/data integrity trap. Pass path: the loop that copies data from the calculated offset to the destination buffer.

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

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a4].adf:AM2_BLIT#unpacked

- `checks_magic_then_jumps`: Compares the longword at the address in a0 with the constant an address and returns the error code an address when they differ.
- `validates_header_and_dispatches`: Checks the longword at the address in a0 against a fixed magic constant and returns an error code when it differs.
- `initialize_data_tables`: Sets the stack pointer to a specific address, calls eleven different routines, loads a pointer from a global table into a0, and calls several more routines before branching.
- `call_blitter_routines`: Clears register d0 and calls two consecutive routines.
- `free_blocks_then_jump_indirect`: Calls the table-freeing routine, reloads the stack pointer from a global variable, loads a value into d0 from another global variable, and jumps through the address stored in a second global variable.
- `initialize_and_update_data_tables`: Clears a block of memory, performs several conditional checks and calls to other routines, populates a table with incrementing values in a loop, copies a block of data from a source to a destination, and updates a series of values in a table by reading bytes from a structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copies_word_triplets_until_zero`: Copies 3-word records from the source pointer to the destination pointer, stopping when a zero source word is read or after 512 records, then writes a zero word to the destination.
- `scans_short_records_for_match`: Walks a table of 3-word records starting at a global variable until it reaches a zero first word, comparing each record's first word with the value in d7.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `appends_record_to_global_buffer`: Scans a global table of three-word records for a matching entry against d7, the byte in d0 and the byte in d1; if a match is found it updates the word at offset 4 when it differs from d2.
- `advance_pointer_and_branch`: Adds 6 to the address register a0 and branches to a nearby instruction that continues the surrounding flow.
- `appends_record_to_buffer`: If a global counter is below 512, increments it and writes the word in d7, the bytes in d0 and d1, the word in d2 and a zero word into consecutive fields at the pointer in a0.
- `checks_mode_and_advances_pointer`: Saves several registers on the stack, compares a byte at a fixed absolute address with 1, and when equal loads a table address and compares a word in d7 with the word at offset 2 of that table, selecting a byte value into d6 accordingly before continuing.
- `advances_pointer_and_loops`: Adds 4 to the address register a0, decrements the counter d6 and branches back to an earlier instruction in this routine while it has not reached -1, otherwise it branches unconditionally to a later address.
- `stores_byte_in_structure_table`: Checks d7 against a global variable, then validates d0 and d1 against two global limits and returns early if any test fails.
- `stores_byte_into_indexed_structure_field`: Calls a helper to obtain a structure pointer, then stores the byte in d2 at the field offset held in d1 plus 332 within that structure, and calls another routine.
- `process_conditional_data_6`: Compares d0 to 11, stores d0 in a global word if it is less than 11, calls another routine if a global byte is greater than 2, and if a global byte is non-zero, retrieves a byte from a table using d0 as an index, stores it in a global word, and calls another routine.
- `call_conditional_routine_6`: Checks if the high byte of d0 is non-zero and not equal to a value in a global word, then calls another routine if these conditions are met.
- `process_conditional_updates_2`: Uses a byte from a table to call a routine, performs several conditional checks against global values and structure members, and updates a global pointer to a calculated offset in a table.
- `set_d0_and_jump`: Sets d0 to 256 and jumps to another routine.
- `set_d0_and_jump_2`: Sets d0 to 512 and jumps to another routine.
- `set_d0_and_jump_3`: Sets d0 to 768 and jumps to another routine.
- `set_d0_and_jump_4`: Sets d0 to 1024 and jumps to another routine.
- `set_d0_and_jump_5`: Sets d0 to 1280 and jumps to another routine.
- `set_d0_and_jump_6`: Sets d0 to 1536 and jumps to another routine.
- `handle_event_data_2`: Checks if the high byte of d0 is non-zero, retrieves a byte from a table using d0 as an index, calls a routine, and if a specific byte in the resulting structure is non-zero, stores the retrieved byte into a global address and calls another routine.
- `store_and_call_handler`: Stores a byte from d2 into a global address and a word from d1 into another global address, calls a routine, and then calls another routine if a bitmask check on d3 succeeds.
- `call_event_handler`: Loads a pointer into a0 and calls another routine, then restores registers from the stack before returning.
- `set_d0_and_jump_7`: Sets d0 to 256 and jumps to another routine.
- `set_d0_and_jump_8`: Sets d0 to 512 and jumps to another routine.
- `load_event_three_hundred`: Loads the word value 768 into d0 and jumps to another routine.
- `load_event_four_hundred`: Loads the word value 1024 into d0 and jumps to another routine.
- `load_event_five_hundred`: Loads the word value 1280 into d0 and jumps to another routine.
- `load_event_six_hundred`: Loads the word value 1536 into d0 and jumps to another routine.
- `dispatch_event_one`: Checks a global byte, retrieves a 16-bit value from offset 0 of a structure in a pointer, and calls another routine if the value is non-zero and bit 15 is not set.
- `dispatch_event_two`: Checks a global byte, retrieves a 16-bit value from offset 2 of a structure in a pointer, and calls another routine if the value is non-zero and bit 15 is not set.
- `dispatch_event_three`: Checks a global byte, retrieves a 16-bit value from offset 4 of a structure in a pointer, and calls another routine if the value is non-zero and bit 15 is not set.
- `dispatch_event_four`: Checks a global byte, retrieves a 16-bit value from offset 6 of a structure in a pointer, and calls another routine if the value is non-zero and bit 15 is not set.
- `dispatch_event_five`: Checks a global byte, retrieves a 16-bit value from offset 8 of a structure in a pointer, and calls another routine if the value is non-zero and bit 15 is not set.
- `dispatch_event_six`: Checks a global byte, retrieves a 16-bit value from offset 10 of a structure in a pointer, and calls another routine if the value is non-zero and bit 15 is not set.
- `set_event_id_seven`: Loads the value 7 into register d0 and jumps to another routine.
- `set_event_id_eight`: Loads the value 8 into register d0 and jumps to another routine.
- `set_event_id_nine`: Loads the value 9 into register d0 and jumps to another routine.
- `scan_flags_and_call_per_entry`: Saves d7 and a0, then scans seven bytes starting at a fixed global address, calling a routine for each nonzero byte, and restores the saved registers before returning.
- `updates_global_flag_from_status_bit`: Calls a helper to obtain a structure pointer and tests bit 5 of the word at offset 30 of it.
- `process_event_table_entries`: Iterates through a table of up to six entries, calling a routine to resolve a pointer from a byte and another routine to retrieve a word from offset 46 of the resulting structure.
- `set_event_parameters_4`: Stores a word into a global variable, stores a long into another global variable, and calls two different routines using a pointer in a0.
- `process_event_list`: Calls another routine, loads a constant address into register a0, and jumps to another routine.
- `execute_complex_sequence_4`: Performs a series of routine calls with various constant parameters, copies 32 words from one memory location to another in a loop, updates several global values, and calls multiple routines including one that takes specific values in d0, d1, d2, and d3.
- `execute_blit_sequence`: Calls another routine and then jumps to another routine.
- `execute_conditional_calls_11`: Loads two words and a byte into registers, calls a routine, and if the zero flag is not set, calls another routine before returning.
- `execute_conditional_handler_10`: Loads values into d0, d1, and d3, calls a routine, and if the result is not zero, calls another routine before returning.
- `dispatch_object_release_if_flagged`: Reads the byte in a global table at the index in d0 and, if it is nonzero, looks up a structure, tests a flag word at offset 30 against a mask, and when the masked bits are clear stores d0 into a global longword and calls another routine.
- `dispatch_object_release_alt_mask`: Reads the byte in a global table at the index in d0 and, if it is nonzero, looks up a structure and tests its flag word at offset 30 against a different mask from the neighbouring routine, storing d0 into a global longword and calling another routine when the masked bits are clear.
- `dispatch_object_release_mask_copy`: Checks the byte in a global table at the index in d0 and, when it is nonzero, looks up a structure and tests the flag word at offset 30 against the same mask as the preceding routine, then stores d0 into a global longword and calls another routine if the masked bits are clear.
- `process_event_data_3`: Retrieves a byte from a table using d0 as an index, calls another routine, and if the byte is non-zero, calls a second routine and then a third routine that stores d7 into a location at a global address.
- `queue_index_if_bits_clear`: Tests d1 against a fixed bit mask and, when none of the masked bits are set, stores the word in d0 into a global longword and calls another routine.
- `queue_index_if_table_nonzero`: Loads the address of a global table and, when the byte at the index in d0 is nonzero, stores d0 into a global longword and calls another routine.
- `forwards_byte_lookup_to_buffer`: Saves d1 on the stack, loads the byte at a fixed absolute address into d1, calls a helper with that value, restores d1 and returns.
- `calls_three_helpers_in_sequence`: Calls three helper routines one after another without any branches and returns, with the returned values in d0 and d1 left as they are.
- `copies_byte_offset_forty_eight`: Saves d1 on the stack, loads the byte at a fixed absolute address (offset 72 decimal) into d1, calls a helper, restores d1 and returns.
- `copies_byte_from_offset_seventy`: Saves d1 on the stack, loads the byte at a fixed absolute address (offset 70 decimal) into d1, calls a helper, restores d1 and returns.
- `copies_byte_from_offset_seventy_one`: Saves d1 on the stack, loads the byte at a fixed absolute address (offset 71 decimal) into d1, calls a helper, restores d1 and returns.
- `copies_string_into_indexed_buffer`: Saves d0 and a1 on the stack, looks up a pointer from the byte in d1 through a helper, and then adds 274 to it to form the destination of a copy routine, which copies a zero-terminated string into the buffer.
- `copy_fixed_string_then_lookup`: Copies a zero-terminated string from a fixed source address into the buffer in a0, backs a0 up by one byte, then calls the lookup routine on d0 and the counter-decrement routine on the same value, and returns the longword value that was in d0 on entry.
- `copy_string_to_fixed_destination`: Loads a fixed source address into a1, calls the zero-terminated byte copy routine to copy into a0, decrements a0 by one to point at the terminator, then restores d0 and a1 and returns.
- `copy_second_fixed_string`: Copies a different zero-terminated string from a second fixed source address into the buffer in a0, backs a0 up by one byte, then calls the lookup routine and the counter-decrement routine on d0 and returns the value that was in d0 on entry.
- `copy_second_string_to_destination`: Loads a different fixed source address into a1, calls the zero-terminated byte copy routine to copy into a0, decrements a0 by one to point at the terminator, then restores d0 and a1 and returns.
- `calls_routine_with_constant_100`: Saves d1 on the stack, loads the constant 100 into d1, calls a routine at a fixed absolute address, then restores d1 from the stack and returns.
- `sets_flag_by_random_threshold`: Saves registers d0 to d2 and d7 on the stack, compares a word at a fixed absolute address with 6, and when it differs clears d7 and branches forward to a later point in the code.
- `sets_flag_from_word_sign`: Tests d0 as a word and, when it is greater than zero, branches to a later address; otherwise it sets d7 to all ones and branches forward.
- `compare_word_and_select_flag`: Compares the word in d1 with the word in d0 and, if d0 is negative after the comparison, branches to the other path; otherwise it clears d7 to zero and branches forward.
- `compare_scaled_value_flag`: Saves d0 into d2, calls the pseudorandom word advance routine, multiplies d0 by d1 and takes the high word, then sets d7 to all ones if d2 is less than the result, and restores the saved registers before returning.
- `fills_table_with_computed_bytes`: Stores the constant 20 into a global byte at a fixed offset from the address in d0, then runs 20 iterations that call a helper for pseudorandom words and store masked values from the global table and two more helper results through the pointers in a0 and another address register.
- `process_conditional_structure_updates`: Compares a word in a global variable to six, retrieves a byte from a table using a value in a7 as an index, and if the byte is non-zero, calls one routine, then checks if a word at offset 202 of a structure in a0 minus a value from the stack is greater than zero to decide whether to call another routine and then call a third routine.
- `store_bytes_and_counter_into_table`: Stores the word in d2 at offset 202 of the structure in a0, saves d0, calls the counter-decrement routine with the byte in d1, then writes that byte into a table at a computed index using d7 and writes the constant 10 at the next index, and restores the saved registers.
- `process_table_entry_and_set_bit`: Retrieves a byte from a table using d7 as an index, calls two routines, sets bit 13 of a word at offset 30 of a structure, clears a word at offset 202 of that structure, and calls three more routines if certain conditions are met.
- `manage_blit_sequence`: Calculates a value based on d7, stores it in a structure, and then executes a complex sequence of calls including a loop that copies bytes and several routines to manage memory and blitting.
- `process_multiple_table_elements`: Calls a routine, retrieves a byte from a table using d7 as an index, calls several routines, clears a byte and two words at specific offsets in tables using d7 and d0, decrements a global word, and calls two more routines.
- `process_blit_sequence_2`: Calculates a value based on d7, stores it in a local stack area and a global table, calls three routines, copies four bytes from a table to a destination in a loop, and performs several subsequent routine calls including a loop that runs up to three times.
- `shifts_table_entries_when_flag_set`: If bit 6 of the word at offset 30 of the structure in a0 is set, it clears that bit and then copies each word from the fourth word after it down into the current position across 8 entries at offset 42 and 10 entries at offset 122, stepping through 8-byte groups.
- `advance_scaled_offset_when_flagged`: If the global flag array byte indexed by d7 is nonzero, it fetches a structure from the lookup routine, adds the values at offsets 206 and 204 to get a limit, and moves the word at offset 202 toward that limit by the amount in d0 without overshooting, then calls another routine.
- `advance_second_scaled_offset_when_flagged`: If the global flag array byte indexed by d7 is nonzero, it fetches a structure from the lookup routine, adds the words at offsets 210 and 212 to form a limit, and moves the word at offset 208 toward that limit by the amount in d0 without overshooting, then calls another routine.
- `adds_table_byte_indexed_by_low_two_bits`: Returns the sum of the word in d0 and the sign-extended byte from a global table, indexed by the low two bits of d2 multiplied by two.
- `adds_table_byte_indexed_by_low_three_bits`: Returns the sum of the word in d0 and the sign-extended byte from a global table, indexed by the low three bits of d2 multiplied by two.
- `clears_128_bytes_of_global_area`: Writes zero to 128 consecutive bytes of a global area, starting at a fixed address, saving and restoring the registers it uses.
- `modifies_bit_in_indexed_table`: Checks d1 against zero and a global table bound, then computes a byte address from a table entry and the bit index in d0.
- `set_bit_unless_mode_two`: If the byte in d2 equals 1, it sets bit d1 in the byte at the address in a0; otherwise it branches forward past that step and continues on the common path.
- `toggles_bit_or_branches_on_flag`: Compares the byte in d2 with 2 and, when it is not equal, branches forward to the common path; when it is equal it toggles the bit numbered by d1 in the byte at the address in a0, then branches forward past that path.
- `returns_stack_value_unchanged`: Restores the saved registers from the stack and returns, passing back the value found on the stack without modifying it, after testing bit d1 of the byte at a0 when d2 equals 3 and otherwise leaving the flags unused.
- `iterate_five_slots_adjust_values`: Loops five times over a global flag array; for each nonzero flag it fetches a structure from the lookup routine and, when specific flag bytes and words at offsets 2, 9, 24 and 30 are clear, subtracts a scaled amount from the word at offset 24 and from the longword at offset 270, then advances to the next entry.
- `scan_and_decay_entries`: Loops over six entries of a global variable array, and for each nonzero entry calls another routine to fetch a structure, conditionally computes a value from its fields at offsets 42, 46 and 270 and stores it in the array.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `scan_and_update_slot_table`: When the argument in d0 is nonzero, walks a table of six global flag bytes and, for each nonzero entry, calls a routine to fetch a structure, records a nonzero word into a parallel output array, and counts the nonzero entries.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `delay_then_call_helpers`: Calls one helper routine, then counts a 16-bit counter down from 22 to zero in a loop, then calls the helper routine again and returns the value in d0 as it stood at entry.
- `delay_then_return_registers`: Calls one helper routine, runs a 16-bit countdown loop from 22 to zero, calls the helper routine again, and returns the d0 and d1 register values that were present on entry as a pair.
- `scan_struct_array_then_call`: Calls one helper routine to obtain a pointer, then steps that pointer forward in 6-byte strides while the halfword at offset 4 is nonzero, for at most 24 steps, then calls another routine and returns the saved d0 and d1 values.
- `count_active_entries_into_d2`: Walks a global variable block of 6 entries, and for each nonzero flag calls a helper that returns a pointer, increments a 16-bit count in d2 when the bits tested in that entry are clear, then calls another routine before moving to the next entry.
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
- `store_pointers_then_branch`: Reads a halfword at offset 4 from the structure in a1, and if it is nonzero computes an entry address by multiplying a 16-bit index by 60 and writes that address and a constant longword into the buffer at a2, advancing a2 and branching to a later instruction.
- `stores_long_constant_and_jumps`: Writes the longword constant 8461 hex to the address in a2 and then increments a2 by four, and branches unconditionally into the middle of a neighbouring routine.
- `fills_pointer_table_from_entries`: Walks a table of 6-byte entries whose halfword at offset 4 selects a 60-byte record, skipping entries that are zero or have bit 1 set in their record.
- `increment_global_byte_5`: Checks a global byte; if it is zero, it calls a routine to retrieve a value, stores that value in the global byte, and then increments the global byte.
- `decrements_global_byte_if_nonzero`: Saves d0 and d7 on the stack, loads a global byte and, if it is nonzero, decrements it and calls another routine when the result reaches zero.
- `dispatches_zero_byte_to_handler`: Saves d0 and a0 on the stack, checks a global word against 3 and branches away when it matches, otherwise loads a pointer from a fixed low-memory location and compares the byte it points to with d0.
- `dispatches_indexed_handler_and_advances`: If the byte at a0 is zero, it calls the handler whose table entry is selected by a global counter times four, increments that counter, and points a global pointer at a fixed global area.
- `clear_global_then_jump`: Clears d0 to zero and jumps to a shared handler that clears a byte global variable, stores the address of another global variable into a table entry, sets that global variable to 255, and stores a further pointer into the table.
- `load_constant_then_jump`: Loads the constant 293 into d0 and jumps to a shared handler that clears a byte global variable, stores the address of another global variable into a table entry, sets that global variable to 255, and stores a further pointer into the table.
- `scans_flags_and_updates_status`: Calls a helper, then loops five times over a block of global flag bytes, setting bit 5 of a status word in a structure for each nonzero byte and calling a helper.
- `execute_table_callback_4`: Performs several routine calls to process data, then uses a value from the stack to calculate an offset into a table.
- `blit_fixed_rect_upper`: Loads fixed coordinates 208, 49, 303 and 128 into d0 to d3 with the value 28 in d4, calls the rectangle clipping and blit routine, and then returns.
- `blit_fixed_rect_lower`: Loads fixed coordinates 20, 50, 182 and 70 into d0 to d3 with the value 28 in d4, calls the rectangle clipping and blit routine, and then returns.
- `blit_two_fixed_rects`: Loads two sets of fixed coordinates into d0 to d3 with the value 28 in d4, calling the rectangle clipping and blit routine once for each set, and then returns.
- `jump_to_blit_routine`: Jumps unconditionally to a fixed absolute address, transferring control without returning here.
- `set_rect_and_clip_blit`: Loads d4 with 28 and d0, d1, d2, d3 with 16, 138, 303 and 192, then calls the clipping routine and returns.
- `clip_two_rectangles_blit`: Sets d4 to 28 and passes two rectangles to the clipping routine, the first with corners 15, 43, 191, 122 and the second with corners 208, 43, 303, 122.
- `clip_blit_lower_rectangle`: Sets d4 to 28 and passes the rectangle with corners 16, 50, 191, 193 to the clipping routine, then returns.
- `clip_blit_two_offset_rectangles`: Sets d4 to 28 and passes the rectangle with corners 16, 43, 303, 122 to the clipping routine, then passes a second rectangle with corners 16, 144, 193, 193 to it.
- `stores_ffffff_in_indexed_long`: Stores the longword value a fixed address into a global area at an offset of four times the value in d7 minus one, saving and restoring the registers it uses.
- `blit_clipped_grid_cells`: Calls a routine once, then loops six times in a counted loop, calling another routine on each pass, and then calls another routine once more.
- `sync_global_byte_flags_per_cell`: Compares a pair of global index words and, when they differ, saves the new value and clears the old and new slot longwords to a fixed value.
- `dispatch_clipped_blit_sequence`: Calls several routines to compute a cell position from a counter, selects a value from a global table depending on whether D7 matches a global index, copies a null-terminated string from a structure at offset 274 into a global buffer of up to four bytes, and calls further routines to finish the drawing.
- `update_sprite_state_flags`: Reads a 16-bit flag word at offset 30 of a structure selected by a global index and, when its top three bits are clear, computes two scaled ratio bytes from the 16-bit values at offsets 202 to 208 and 210 to 212, storing them and setting bit 2 of the byte pointed to by a4 when they change; otherwise it sets a status byte to 1 and sets bit 2 of the byte in a4.
- `update_element_arrays`: Calls a routine, then enters a loop that checks a byte at an offset of a structure, calls a routine, decrements that byte, and continues until the byte is zero; it then performs a second loop that calls two routines for up to seven iterations.
- `save_registers_and_branch`: Saves registers d7 and a0 to the stack and branches to another routine.
- `execute_blitter_setup_2`: Calls two routines, then enters a loop that checks bytes in a table at a specific address and calls a routine if the byte is non-zero, repeating this up to six times before calling a final routine.
- `execute_scaled_blit_sequence_2`: Calculates a value using multiplication and addition, calls a routine, and if d2 is non-zero, updates a global variable and calls two more routines.
- `update_byte_array_elements`: Calculates a value based on a byte from a table, performs a scaled blit, and enters a loop to update elements in a structure while decrementing a counter.
- `updates_pair_of_byte_counters`: Loops over 14 count-down steps, storing a byte from d1 at one byte offset of the structure in a1 and decrementing a count byte two offsets further.
- `update_bitmask_and_scan`: Walks a table of 12 byte entries using a bit mask from a global variable and a loop counter, writing a found index into a byte array and a global word while adjusting values by 10 and 20 as it goes.
- `scan_bitmask_and_write_bytes`: Scans a 12-entry table of byte values, comparing each entry's bit against a bitmask in d0 and advancing a pointer in a3, and on the first match stores the index to the word at a3 and the index plus 10 to the byte at a1.
- `store_interleaved_bytes_loop`: Repeatedly writes the word in d2 to the address in a3 and stores the low byte of d2 plus 10 to the address in a1, advancing a0 by 1, a1 by 1 and a3 by 2 each pass, while incrementing d7 until it reaches 7 and looping back otherwise.
- `increments_global_counter_word`: Adds one to a global counter word and returns.
- `execute_blitter_loop`: Loads a pointer from a global variable into a1 and another into a3, then enters a loop that runs five times, calling another routine if the value at the pointer in a1 is zero.
- `clamp_word_to_180_limit`: Compares d7 with 180 and, if d7 is below 180, skips ahead to the following branch; otherwise it loads d7 with 12 and branches to a later point.
- `loop_blit_rectangle_until_count`: Repeatedly calls three routines to clip and blit a rectangle, walking a list through A1 and skipping entries whose word at offset 4 is zero, and decrements a counter from a stack word until it reaches -1, then returns the stacked longword.
- `set_status_word`: Sets a global word to 31 and compares a global word at offset 3734 to 6; if it is 6, it branches to another routine, otherwise it loads a value from offset 3736 into d3.
- `toggle_status_bits`: Performs bitwise operations on two global words, and if specific bits are set, calls another routine with specific parameters.
- `setup_blit_parameters_8`: Loads several constant values into registers, loads a pointer from a table into a0, calls another routine, and then branches based on the values of bits 2 and 0 of d2 and d3.
- `execute_conditional_blit_3`: If bit 2 of d3 is set, it branches; otherwise it saves d2 and d3, sets several registers, calls another routine, and branches to another location.
- `calculate_offset_and_call`: Loads a word from a table using an index in d0, multiplies it by 384, adds the result to a0, and calls another routine before branching to another routine.
- `test_bits_and_branch`: Tests bit 0 of register d2 and branches if set, otherwise tests bit 0 of register d3 and branches if set, otherwise branches to another routine.
- `setup_blit_parameters_7`: Tests bit 0 of d3, and if it is not set, saves registers and calls another routine with specific constant values in d0, d1, d2, and d3.
- `setup_blit_parameters_6`: Loads several constant values into registers and calls another routine.
- `calculate_blit_parameters_4`: Calls a routine with specific values in d0-d3, then checks a byte in a global variable and performs arithmetic on a word from another global variable before branching.
- `calculate_blit_parameters_3`: Calculates a value based on a word from a global variable, performs arithmetic involving multiplication and division, and calls three different routines to update various memory locations and structures.
- `initialize_blit_sequence`: Tests bit 1 of d3 and branches if set, otherwise saves d2 and d3 to the stack, loads several constant values into d0, d1, d2, d3, and d4, calls another routine, and branches.
- `select_fixed_region_by_flag`: Tests a byte global variable and, when it is nonzero, branches forward; otherwise it loads a fixed address into a0 and branches to a later point.
- `blit_fixed_rectangle_region`: Loads A0 with a fixed address, sets D0 to 240 and D1 to 49, sets D4, D5, D6 and D7 to constants 24, 3, 2 and 32, calls a blit routine, then restores D0 to D7 and A0 from the stack and returns.
- `call_handler_with_constant`: Saves registers, sets d1 to a constant, calls another routine, and stores the resulting byte from d0 into a global memory location.
- `invoke_three_blit_helpers`: Calls three routines in sequence and then returns the longword pair held in D0 and D1.
- `process_structure_data_6`: Saves registers, calls a routine to retrieve a pointer, fetches a byte from a table using a register as an index, and performs several calls to update a structure and its fields.
- `update_structure_pointer_2`: Saves d0, d7, and a0 through a1 to the stack, loads two pointers into a1 and a0, calls a routine, stores the result in a structure, loads a byte from a table using d7, calls another routine, updates a pointer in a structure, clears a long value, and calls a third routine before finally calling a fourth routine and returning.
- `store_offset_sum_to_pointer`: Calls two routines to obtain a value, adds 274 to it, and stores the result as a longword through the pointer in a0.
- `update_table_and_call_2`: Saves d0 and a0 through a2 to the stack, loads a pointer from a table into a1, adds d7 to a1, reads a byte from a1 into d0, calls a routine, updates a pointer in a structure, calls another routine, calls a third routine, and returns.
- `initialize_and_call_routines`: Sets two longwords at specific global addresses to all ones, loads a word from a third global address into d0, calls a routine, and then jumps to another routine.
- `execute_three_subroutines_2`: Calls one routine, calls another routine, and then jumps to another routine.
- `release_queued_object_by_global_byte`: Ignores the call when d0 is zero or minus one; otherwise it masks a global byte with d0 minus one, reads a word from a table indexed by that value, and when the word is nonzero stores d1 into a global longword and calls another routine.
- `dispatch_indexed_table_entry`: Masks the byte in d0 and, when a helper returns a value at or above 200 in a particular context, reads a 16-bit value from a table indexed by one less than the masked byte times six, then calls another routine when that value is nonzero.
- `dispatch_offset_table_entry`: Ignores d0 values of zero and 255, otherwise adds a global variable and eight to the masked byte, then reads a 16-bit value from a table indexed by the result times six.
- `process_clamped_index`: Tests d0 for zero or 255, masks and decrements d0, adds a word from a global pointer to d0, and calls another routine if the result equals a constant.
- `process_scaled_data_2`: Multiplies d0 by 6, calls another routine using a byte from a global variable, adds a global address to a pointer, retrieves a word from a table, calls another routine, calls a third routine, and if the result is non-zero, stores d1 into a global variable and calls a fourth routine.
- `call_subroutine_and_return`: Calls another routine and then returns.
- `test_and_branch_data`: Tests a byte in d0 and branches if it is zero, otherwise it masks the lower byte, decrements d0, and compares it to a value in a global variable.
- `process_offset_scaled_data`: Multiplies d0 by 6, calls another routine using a byte from a global variable, adds 290 to a pointer, retrieves a word from a table, calls another routine, calls a third routine, and if the result is non-zero, stores d1 into a global variable and calls a fourth routine.
- `call_subroutine_and_return_alt`: Calls another routine and then returns.
- `store_clamped_sprite_entry_2`: Saves d0 and a0, multiplies d0 minus one by 60 to index a fixed table, and offsets a0 by 40 into that entry.
- `repeat_twelve_blit_calls`: Runs a counter from 11 down to 0, calling a subroutine once per pass, then calls that subroutine one more time before returning.
- `loop_graphics_updates`: Executes a routine in a loop for eleven iterations, incrementing d0 on each pass.
- `update_graphics_structure`: Saves registers, performs multiple calculations involving multiplication and table lookups, calls several routines to update data, and modifies a global word at a specific offset.
- `modify_graphics_data`: Saves registers, calculates an offset into a table using d0, calls a routine, performs arithmetic on a value from a table, and calls several routines to update data based on bit tests.
- `cycle_table_index_wrap`: Decrements a countdown global variable and, when it reaches zero, advances a table index and reloads the countdown with 4.
- `advance_indexed_byte_sequence`: Decrements a 16-bit counter in a global variable; when it reaches zero it reloads that counter with 4 and increments an index word, then reads the byte at that index from a table and stores it in another 16-bit global variable.
- `execute_blit_logic`: Checks a global byte, and if it is zero, calculates an offset into a table, calls another routine, and then performs a multiplication and addition to update a pointer in a1.
- `push_and_call_pointer`: If d0 is not equal to 1, it pushes d0 to the stack, retrieves a pointer from a location at offset 40 of a6, calls another routine, moves the result to a1, restores d0 from the stack, and branches to another routine.
- `execute_blit_command_2`: Compares d0 to two, branches if not equal, pushes d0 to the stack, loads a byte from a pointer in d0, calls a routine, sets a1 to the result, loads d0 from the stack, adds a constant to a1, sets a2 to a constant, and branches.
- `decrement_and_call_counter`: Decrements a pointer in a1, calls another routine, loads a byte from a specific location into d0, and jumps to another routine.
- `set_global_and_jump`: Sets a global word to 31 and then performs a triple call to three subroutines before jumping to another routine.
- `store_clamped_sprite_entry`: Calls routines to derive a value and store it in two global variables, saves D6 and D7 into global words, sets a global variable to the derived value, calls two further routines, and then clears that global variable to zero.
- `store_sprite_entry_with_ff`: Calls routines to derive a value and store it in two global variables, saves D6 and D7 into global words, sets a global variable to the derived value, calls two further routines, and then sets that global variable to 255.
- `update_conditional_fields`: Checks a word in a global variable, calls two routines with different pointers if the word is not all ones, and finally writes a constant to a global variable.
- `call_handler_when_flag_set`: Calls one routine unconditionally, then tests a global variable byte and calls another routine only when that byte is nonzero.
- `computes_clamped_offset_and_stores_word`: Saves d0, d1 and d7 on the stack, stores d1 minus 7 at offset 2 of the structure in a0, and multiplies d7 by 6 and subtracts 1.
- `clamp_word_to_range_store`: Adds d0 and d7, subtracts 1, and if the result is 320 or more replaces d0 with 320 minus d7.
- `fills_thirty_one_words_with_random`: Saves registers on the stack, copies d0 into d1, advances the pointer in a0 by 2, sets a counter to 30, then in a loop reads a word through a0, calls a routine to transform it, stores it back and decrements the counter until it reaches minus one, then restores the registers and returns.
- `check_parameter_bounds`: Saves d0 through d7 and a0 through a2 to the stack, compares d0 to nine, and if d0 is not positive, sets d4 to d0 plus one and branches.
- `calculate_scaled_parameters`: Calculates a value from a global word, uses it to index into a table to retrieve two words, calls a routine, and then enters a loop that performs several calculations and calls a routine multiple times based on a counter.
- `copy_data_and_dispatch`: Writes a word to a global variable, stores a pointer, calls a routine, stores a byte from d0 into a global variable, calls another routine, enters a loop that copies words from a0 to a structure and increments counters, calls a third routine, stores a word in a global variable, and if a counter is not zero, calls a fourth routine before branching.
- `execute_chained_routines`: Calls two different routines in succession and then returns.
- `check_and_call_routine`: Loads a byte from a global address into d0, calls a routine, and then jumps to another routine.
- `perform_multiple_blit_calls`: Calls several routines with specific word parameters, updates a pointer from a global variable, and performs a final jump to another routine.
- `process_conditional_list`: Checks if a word in a global variable is non-zero, and if so, calls two routines before returning.
- `computes_scaled_sum_in_loop`: Reads a global word and a second global word, computes their difference clamped to 16, and then loops that many times, each pass calling another routine and storing either 31 or 16 into a global word depending on a flag, and accumulating the value returned by another routine into a word.
- `tests_bit_table_entry_and_sets_word`: Loads the address of a table into a0, reads the next word from the pointer in a1 into d0, sets d1 to 6 and calls a routine, then when the result is zero stores 16 into a global variable and branches to a later point in the code.
- `stores_word_then_calls_helper_routines`: Writes the constant 31 to a global word, calls one routine, sets d0 to 52 and copies d3 to d1, calls a routine at a fixed absolute address, then adds 7 to d3 and loops back on the counter in d7.
- `execute_conditional_blit_operation`: Loads two 16-bit values and a 30-count constant into registers, calls another routine, and if the zero flag is not set, calls another routine before returning.
- `execute_multiple_tasks_2`: Writes 2 to a global variable, performs four separate calls with various values in d0, d1, d2, and d3, and if certain conditions are met involving a global byte and a byte at a pointer in a0, it calls another routine, stores a word in a global variable, and performs three conditional calls to another routine before setting a byte in a global variable and calling another routine.
- `process_scaled_index`: Checks if a value in d0 is within a specific range, calls two routines to fetch and process data, updates a global word, and performs arithmetic to set several global values.
- `clears_clip_bounds_when_flag_set`: Tests whether the byte in d0 is nonzero and, if so, compares the sum of a global byte and the high byte of d0 against a global limit.
- `clears_and_sets_clip_bounds`: Calls a routine, clears a longword global variable, stores all ones into the word after it, calls two routines, then clears the word at the second global location.
- `calls_chain_and_swaps_table_words`: Calls one routine, saves the word it returns in d0, calls a second routine and then a third, and returns the saved word.
- `compares_null_terminated_strings`: Calls two routines to obtain pointers in a0 and a1, then compares the two byte strings character by character, returning early on the first mismatch and otherwise stopping after the terminating zero byte.
- `swap_two_table_words`: Doubles the word indices in d5 and d6, then swaps the 16-bit words at those two offsets in the table addressed from a6, using d0 as temporary storage.
- `compare_limit_against_table_entry`: Saves registers, loads a byte from a fixed absolute address, and calls a routine to obtain a pointer; it then compares a word at that pointer against the saved value and branches on the result.
- `clear_byte_after_countdown`: Decrements d0 as a 16-bit value in a loop until it wraps to 65535, then calls one routine, clears the byte at the address in a0 to zero, and calls another routine before returning.
- `match_run_length_encoded_strings`: Reads a count and a run of length-prefixed byte strings through pointers, comparing each byte through a lookup table against a second buffer and counting entries until a match or the end of the list.
- `copies_byte_count_and_pointers`: Clears d6, reads the next byte from the address in a1 into the low byte of d6, copies that count to d5, and copies the address registers a0 and a1 into a2 and a3.
- `decode_run_length_bytes_loop`: Loops over bytes read through pointers in a3 and a2, translating each byte through a lookup table and comparing it to the next byte in the second string, and on a mismatch or end of count calls another routine and returns.
- `process_event_with_byte_update`: If the high byte of d0 is non-zero and not equal to a global word, it retrieves a pointer from a table, calls a routine, and if a byte at offset 9 of the resulting structure is non-zero, stores d2 into a global byte and calls another routine.
- `update_global_state_2`: Stores d2 into a global byte, stores d1 into a global word, and calls three consecutive routines before returning.
- `set_event_one_hundred`: Sets d0 to a constant value and branches to another routine.
- `load_constant_into_d0`: Loads the constant value 512 into register d0 and branches to another routine.
- `load_event_parameter`: Loads the constant value 768 into register d0 and branches to another routine.
- `load_event_parameter_2`: Loads the constant value 1024 into register d0 and branches to another routine.
- `load_constant_into_d0_2`: Loads the constant 1280 into register d0 and branches to another routine.
- `load_event_parameter_3`: Loads the constant value 1536 into register d0 and branches to another routine.
- `process_event_with_word_check`: If the high byte of d0 is non-zero and not equal to a global word, it retrieves a pointer from a table, calls a routine, and if a byte at offset 9 of the resulting structure is non-zero, stores d2 into a global byte and calls another routine after checking a word at offset 30.
- `update_conditional_state`: Performs a bitwise AND on d3 with a constant, and if the result is non-zero, calls three consecutive routines after storing d2 and d1 into global locations.
- `set_event_parameter_one`: Loads the constant 256 into register d0 and branches to another routine.
- `set_event_parameter_two`: Loads the constant 512 into register d0 and branches to another routine.
- `set_event_parameter_three`: Loads the constant 768 into register d0 and branches to another routine.
- `set_event_parameter_four`: Loads the constant 1024 into register d0 and branches to another routine.
- `set_event_parameter_five`: Loads the constant 1280 into register d0 and branches to another routine.
- `set_event_parameter_six`: Loads the constant 1536 into register d0 and branches to another routine.
- `execute_callback_chain_6`: Loads 6 into d0, calls two routines, loads a pointer from a global address into a0, calls a third routine, and jumps to another routine.
- `execute_blit_and_jump`: Calls another routine and then jumps to another routine.
- `process_complex_updates`: Calls three different routines and then jumps to another routine.
- `set_high_bit_when_flags_nonzero`: Calls one routine to obtain a pointer, reads the word at offset 30 of that structure and masks it with 1162, and sets bit 7 of the byte at the address in a0 when the masked word is nonzero or the byte at offset 9 is nonzero.
- `release_queues_then_run_handler`: Calls another routine to release queued objects, then if a global pointer is nonzero and the handler address at offset 30 of that structure is nonzero, it calls that handler indirectly.
- `manage_event_lists`: Calls another routine, loads a pointer into register a0 from a global address, and then jumps to another routine.
- `test_and_branch_data_2`: Tests a byte in d0 and branches if it is zero, otherwise it masks the lower byte, decrements d0, and compares it to a value in a global variable.
- `process_conditional_data_5`: Calls two different routines, stores a word from d0 into a global variable, and if a specific bit is set in a word from d7, it stores d0 into a global variable and calls another routine.
- `process_data_table_2`: Loads a pointer into a0, moves d0 to d2, and loads a byte from the address in a0 into d1.
- `update_global_word_5`: Stores a word from d0 into a global variable, moves a byte from d1 into d0, and tests a bit in d0 using a constant in d1.
- `dispatch_tile_update`: Moves a word into a global variable, loads a pointer into a0, extends d0, and calls two different routines before branching to another location.
- `execute_conditional_callback`: Calls a routine via a pointer and then returns.
- `render_ui_text_elements_2`: Calls several routines to fetch data from tables and performs a series of blitter operations to draw text characters or symbols, including loops that iterate through character sets and write specific byte values to memory.
- `scan_mask_table_for_bits`: Iterates over a table of byte pairs in a global area until it reaches a terminator value of 255, calling a subroutine for each entry whose bit is set in a register mask, then calls a final subroutine.
- `call_routine_preserving_registers`: Saves d0 on the stack, loads a byte from a fixed absolute address into d0, calls another routine, then restores d0 and the saved registers before returning.
- `process_graphics_parameters`: Calls several routines to retrieve data from a table, updates a global variable, and performs a conditional branch based on a byte at an offset of a structure in a1.
- `render_complex_elements`: Performs several routine calls using various offsets from a structure in a1, including a loop that checks a byte in a table and performs calculations to determine a value for subsequent routine calls.
- `draw_complex_scaled_elements`: Performs a series of scaled blitter operations using various offsets from a structure and updates a global variable based on a comparison.
- `sets_counter_and_calls_helper_sequence`: Stores the constant 2 into a global word, calls two routines, then stores either 2 or the value of another global word into that same global word depending on whether a third global word is zero.
- `sets_counter_and_calls_helpers`: Stores the value 2 into a global variable, calls several routines in sequence, stores 2 into the same global variable again, then calls further routines and returns the value on the stack unchanged.
- `call_external_handler`: Calls a routine via a pointer and then jumps to another routine.
- `validate_sprite_attribute_type_3`: Checks if a byte at offset 4 of a structure equals 3, then verifies that the sum of two 16-bit words at offsets 218 and 220 of a structure pointed to by a retrieved pointer is non-zero; if the check fails, it clears a byte at offset 4 and sets a 16-bit value at offset 20 of the structure to 65535.
- `validate_sprite_metadata`: Checks if a byte in d0 equals 4, then validates a sprite index from a structure in a1 against a table at a fixed address by comparing specific bytes.
- `validate_sprite_attribute_data`: Checks if a byte in d0 equals 7, then uses values from the structure in a0 to index into a table via a1 to validate specific attribute bytes against a secondary data table.
- `process_inventory_data`: Loads 2 into d0, calls a routine, sets d0, d1, and d2 to specific constants, calls three more routines, and then branches based on a byte at a global address.
- `load_address_and_call`: Loads a longword into a0 from a constant address, calls a routine, and then jumps to another routine.
- `execute_triple_call`: Calls two routines in sequence and then jumps to another routine.
- `perform_complex_sequence_2`: Calls four routines in sequence and then jumps to another routine.
- `setup_and_jump`: Calls two routines in sequence, loads a constant into a0, and then jumps to another routine.
- `release_queues_run_table_handler`: Calls one routine to fill a fixed table from a global word, calls another routine to release queued objects, then calls the handler at offset 30 of a global structure pointer when that pointer and handler are nonzero.
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
- `update_structure_from_table_2`: Saves registers to the stack, calculates an offset based on a global value and d0, retrieves two words from a table, performs arithmetic on d0, calls two routines, and then branches back to the start of the routine.
- `loop_structure_updates_with_scale`: Calls another routine, performs arithmetic on d0 using a value from a global variable, calls two more routines, writes two words from global variables to a structure in a0, calls a fourth routine with specific parameters, and enters a loop that repeats these operations using d7 as a counter.
- `loop_structure_updates_3`: Executes a loop where each iteration calls a routine with a byte from a global pointer, performs subtractions, and calls two more routines.
- `stores_word_from_structure_into_table`: Saves d0, d1 and a0 on the stack, points a0 at a fixed table, and doubles a global word into d1.
- `computes_and_stores_two_bytes`: Calls a sequence of helper routines, reads a word at offset 270 and words at offsets 42 and 46 of a structure, and compares their combined value against a thousandth of the offset-270 longword.
- `update_status_flags_from_structure`: Calls a callee, then ORs bit 7 into several status bytes of the structure in a0 (offsets 2, 6, 8, 10, 12, 14, 16) depending on whether a global variable is nonzero, a bit test on the byte at offset 2 of the structure returned by the callee, and whether the 24-word table of values and the 8-word table at the returned structure's offsets 344 and 290 contain any nonzero word.
- `process_graphics_structures_2`: Calls two routines, sets a longword in a global variable, sets a byte in a global variable, sets a longword in a global variable, and copies a word from a table into a structure at offset 18.
- `process_graphics_structures`: Executes a loop to call a routine, then performs multiple conditional checks involving table lookups and structure offsets to call various routines that modify data or perform blitting operations.
- `clear_and_call_routines`: Clears a longword at a global address, sets d0 to 44, and calls two different routines before returning.
- `execute_sequence_calls`: Sets a byte at a global address to zero and calls three different routines in sequence.
- `store_shifted_byte_when_bit_set`: Calls a routine to fetch d0, then tests the bit selected by the upper byte of d0 within a global word.
- `accumulate_flagged_entries_bitmask`: Loops over a global table of flags, and for each nonzero entry other than one excluded index it calls another routine to fetch a record, sets the corresponding bit in a 16-bit result when a field of the record meets a computed threshold, and stores that bitmask into a global variable.
- `accumulate_flagged_entries_alt_bitmask`: Loops over a global table of flags, and for each nonzero entry other than one excluded index it calls another routine to fetch a record, sets the corresponding bit in a 16-bit result when a field of the record meets a computed threshold using a different scale factor, and stores that bitmask into a global variable.
- `writes_marker_bytes_then_flag`: Saves registers, sets the byte at a global address to true, and writes the byte value 8 through a0 with a0 advancing, repeating for a counter in d7 starting at one, depending on bit d7 of a word loaded from a global address; it then restores the registers and returns.
- `write_marker_bytes_loop`: Writes the byte 9 through the pointer in a0 and advances a0 by one, repeating until the counter in d7 reaches 7 (the branch loops back to the start of this routine's sequence while d7 is below 7), then restores d0, d7 and a0 from the stack and returns.
- `update_complex_graphics_data`: Performs a series of calculations and table lookups to call multiple routines that update global pointers, modify bytes in a structure, and execute various scaled blitting and callback operations.
- `check_and_process_blit`: Checks a bit in a global structure, compares a global word to 6, and if both conditions are met, calls another routine with specific parameters and then calls a third routine.
- `process_scaled_tile_data_3`: Calculates an offset using a word from a global variable, updates a global variable, and performs a series of blit operations using bytes from a structure at a calculated address.
- `calculate_string_offset_2`: Fetches a byte from a structure, calls a routine, performs arithmetic on a value multiplied by 30, and calls several routines to update a global word and a structure field.
- `conditionally_reloads_countdown_timer`: Tests a global byte and, when it is nonzero, calls a helper routine, copies a global word into another global word, and calls a second helper.
- `negate_byte_if_global_set`: Tests a global byte and, when it is zero, branches past the negation; otherwise it negates the low byte of d0 before returning.
- `process_indexed_event_handler`: Calls a routine to retrieve an index, uses that index to calculate an offset into a table to fetch a pointer, and if a word at offset 4 of the resulting structure is non-zero, calls another routine after checking a bit in a secondary indexed table.
- `call_decrement_handler_3`: Tests a byte at a global address and, if bit 3 of a byte at offset 32 of a structure in a1 is set, calls a routine with d0 set to 56.
- `copy_and_update_structure`: Copies a longword and a word from a structure in a2 to a structure at offset 1250, calls another routine, calls three more routines, and then updates three global variables from specific sources before calling a final routine.
- `calls_helper_preserving_d0_word`: Saves the word in d0 on the stack, loads a byte from a global address into d0, calls a helper routine, then restores d0 from the stack and returns.
- `store_byte_and_branch`: Stores a byte from d1 into a global address and then branches to another routine.
- `copy_and_update_structure_extended_2`: Checks a global byte, updates a global word, clears a longword, retrieves a word from a structure, and if the word is not 1, updates a global longword with a value from a structure.
- `copy_and_update_structure_extended`: Stores a longword from d0 into a global variable, copies a word from a2 to a global variable, sets a word at offset 22 of a global structure to 54, clears a global byte, calls another routine, and if a global longword is non-zero, copies a structure from a2 to offset 1250 and performs multiple routine calls and global updates.
- `calls_helper_preserving_word_d0`: Saves the word in d0 on the stack, loads a byte from a global address into d0, calls a helper routine, then restores d0 from the stack and returns.
- `copy_words_and_jump`: Loads a pointer into a0 from a global address, copies two words from global addresses into the memory pointed to by a0, and then jumps to another routine.
- `execute_triple_calls`: Calls one routine, sets d0 to 55, and then calls two more routines before returning.
- `execute_four_calls_and_jump`: Sets a byte at a global address to zero, calls three routines in sequence, and then jumps to another routine.
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
- `update_values_and_branch`: Adds a byte from d3 to a memory location pointed to by a2, multiplies a word from a structure in a1 by d3 and adds the result to a longword at an offset of a structure in a0, clears a word at an offset of a structure in a3, and then branches to another routine.
- `copy_and_scale_structure`: Copies a longword and a word from a structure in a3 to a structure in a2, calculates a value by multiplying a byte from a3 by a word from a1 and adding it to a longword at offset 270 of a0, then clears two bytes in a3 and branches to another routine.
- `update_graphics_and_globals`: Copies a structure from a2 to a4, copies a word from a4 to a4, copies a byte from a global variable to d0, copies a word from a global variable to d2, calls several routines including one that uses a3 and a2, updates several global variables from d0, d2, and other globals, and branches to another routine.
- `update_graphics_list`: Calls two routines, checks a word at an offset of a structure in a3, and if non-zero calls two more routines, then loads a byte from a global variable and calls a final routine.
- `update_entity_attribute_and_offset_3`: Compares a value from a global structure against a threshold, and if it exceeds a specific limit, calculates an index to access a table.
- `execute_conditional_callbacks_5`: Calls a routine, then checks a bit in a word from a global variable; if the bit is set, it retrieves a byte from a table using an offset in d0, calls another routine, and then performs a series of conditional calls based on a global word and a comparison with a value in d7.
- `copies_record_into_slot_table`: Calls a helper to get a structure pointer, computes a record offset from a word in the record at a0, and then scans a table of six-byte entries up to 24 times.
- `copy_record_and_transfer_counter`: Copies a longword and a word from one record to another, clears a byte in the destination, subtracts a byte counter from the source, adds it to the destination, and adds a scaled product to a longword in a global structure.
- `advance_record_pointer_until_zero_word`: Sets a pointer register from another, then loops up to 24 times advancing a pointer by 6 bytes until a word at offset 4 is zero, then branches to a jump.
- `copies_record_decrements_counter_offset`: Copies a longword and a following word from the structure at a2 into the structure at a4, decrements the byte at the start of a2, sets the byte at a4 to 1, and adds a word from the structure at a1 to a longword at offset 270 of the structure at a0.
- `subtracts_stats_and_clears_entry`: Calls a helper to get a structure pointer and, when the index in d2 is below 10, subtracts two byte values from fields at offsets 6 and 7 and two more byte values from words at offsets 222 and 224.
- `subtract_record_fields_and_clear`: Subtracts byte fields of one structure from word and longword fields of another at fixed offsets, conditionally adjusts a word selected by an index byte, calls a helper, then clears a word field.
- `subtracts_scaled_offset_clears_word`: Calls a helper routine, subtracts the word at offset 38 of the structure at a1 from the longword at offset 270 of the structure at a0, clears the word at offset 4 of the structure at a2, then calls another helper routine and returns.
- `append_record_and_decrement_count`: Stores a longword and word from a source record into a slot of a table, sets a flag byte, adds fields from a second structure into a third structure's offsets, then decrements a byte counter in the source record and clears fields when it reaches zero.
- `adds_offset_decrements_byte_clears`: Calls a helper routine, adds the word at offset 38 of the structure at a1 to the longword at offset 270 of the structure at a0, then decrements the byte pointed to by a4.
- `accumulate_sprite_offsets`: Adds signed byte fields from the structure at a1 into word fields of the structure at a0 at offsets 206 and 212, then adds unsigned bytes at offsets 216 and 220.
- `subtract_offsets_from_record`: Subtracts unsigned byte fields of the structure at a1 from word fields of the structure at a0 at offsets 206 and 212, then from offsets 216 and 220.
- `update_graphics_state`: Calls a routine to retrieve a byte, stores it in a global variable, and then writes two words and a byte to a structure at offset 73e before calling another routine.
- `update_structure_pointers_5`: Calls a routine with a specific address, calls another routine, and then calls a third routine to process a byte from a global variable.
- `update_structure_byte_2`: Loads a byte from a global variable, calls a routine, then loads a byte from a global variable, calls another routine, and finally stores the result into a global variable at offset d7f.
- `manage_complex_graphics_state`: Saves registers, calls several routines to retrieve values, updates a global pointer and two global words, and performs a series of conditional routine calls and data updates.
- `scans_structures_and_sets_mask`: Walks a linked set of records via a global pointer, testing a bit mask in each record and comparing a timing value with a byte read from the input structure, and sets a bit in d8 for the first match before exiting its scan.
- `sets_bit_and_stores_mask_word`: Sets the bit selected by d7 in d6, calls a helper routine after reading a byte from the structure at a1 minus one, increments d7, and repeats while d7 is below seven.
- `fetch_and_test_structure_flags`: Retrieves a pointer from a routine, calculates an offset into a table using a value from that pointer, and calls another routine if specific bits in a bitfield at the calculated offset are not set.
- `modify_parameter_value`: Loads a byte from a global address, calls two routines, and if d0 is not equal to 5, performs a comparison and potentially calls another routine.
- `perform_triple_call`: Subtracts 40 and 9 from d0, calls another routine, calls another routine, calls another routine using d0, and restores registers before returning.
- `copy_table_elements_2`: Compares a word in d0 to ten, and if it is not less than ten, subtracts a word from a global variable, calculates an offset into a table, copies two words from that table to a new location, and calls three other routines.
- `updates_status_flag_bytes_from_record`: Calls a routine, reads a word from the pointer it returns, then reads a word from the structure at a1 and sets bit 7 of the byte at offset 6 of the structure at a0 when bit 4 of that word is set.
- `sets_high_bits_in_flag_bytes`: Calls a routine and, when it returns zero, sets bit 7 of the byte at offset 6 of the structure at a0.
- `update_pointer_and_call_3`: Compares a byte at offset 84 of a global structure to 2, and if it is negative, loads a long value into a global pointer; otherwise it loads the address of a table into a0, calls another routine, and branches to another routine.
- `load_and_compare_words`: Loads two words from memory, calls another routine, and if the result of a comparison with a global byte is not positive, calls another routine and then calls the first routine again before returning.
- `update_pointer_and_call_3_2`: Compares a byte at offset 84 of a global structure to 2, and if it is negative, loads a long value into a global pointer; otherwise it loads the address of a table into a0, calls another routine, and branches to another routine.
- `load_and_compare_words_2`: Loads two words from memory, calls another routine, and if the result of a comparison with a global byte is not positive, calls another routine and then calls the first routine again before returning.
- `compare_and_branch_sequence`: Compares a byte at a global address to two, branches if the result is negative, otherwise it writes a word to a global address, loads a pointer into a0, calls a routine, and then branches to another routine.
- `execute_conditional_calls_17`: Loads two words from memory into d0 and d1, calls another routine, and then performs a sequence of conditional calls based on the results of the first call and subsequent comparisons.
- `process_event_pointer_2`: Pushes d0 to the stack, retrieves a pointer from a global variable, calls another routine, and then uses the returned pointer to access a byte at offset 4.
- `branch_loop_counter`: Decrements a loop counter in d7 using a value from a memory location and branches to another routine if the counter is not zero.
- `execute_event_logic_2`: Loads a pointer into a0 and calls another routine, then if a global word is non-zero, sets a global byte and calls another routine.
- `test_and_call_routine_5`: Tests a byte at a global address, branches if it is zero, otherwise it sets d0 to 39, calls a routine, and then branches to another routine.
- `execute_and_call_list`: Calls a routine, loads the address of a fixed memory location into a0, calls another routine, and then returns.
- `check_and_set_byte`: Clears d0, compares a word at a global address to six, and if it is not equal, branches to another routine; otherwise it sets d0 to fifteen and branches to another routine.
- `handle_event_logic`: Pushes d0 to the stack, calls another routine with a byte from a global address, uses the returned pointer to load a word into d1, pushes d0 to the stack, calls another routine with a byte from a global address, and if bit 3 of d1 is not set, calls another routine with d0 set to 289.
- `set_conditional_byte_4`: Checks if a word at a global address is non-zero, otherwise it sets a byte in a global variable to fifteen, calls a routine, loads an address into a0, calls another routine, and returns.
- `jump_to_list_handler`: Calls a routine, loads the address of a fixed memory location into a0, and jumps to another routine.
- `jump_to_fixed_address`: Loads the address of a fixed memory location into a0 and jumps to another routine.
- `populate_structure_table`: If a global byte is zero, it calls two routines, calculates two offsets based on global words, stores these offsets and a byte into a structure at a global address, then enters a loop that performs 24 iterations of storing a longword and calling a routine.
- `conditionally_calls_scaled_offset_forward`: Saves registers, loads three words from global addresses into d0, d1 and d2, and if a global byte is below two it loads a word into d2 and calls a routine that uses the scaled offset.
- `forwards_scaled_offset_call`: Calls the scaled offset routine with no arguments of its own and then returns.
- `set_pointer_and_jump`: Calls a routine, stores a long value into a global variable, loads the address of a fixed memory location into a0, and jumps to another routine.
- `stores_pointer_and_calls_routine_2`: Compares two word fields derived from global variables against small constants and calls a routine on either branch of the test.
- `stores_pointer_and_calls_routine`: Loads the address 5b32 into a0, calls another routine, then returns.
- `wait_then_call_indirect_handler`: Calls a wait routine and a release routine, then calls the code pointer stored in a global variable with an indirect call.
- `validate_range_and_call`: Calls another routine, then checks if the result in d0 satisfies a range condition relative to a global word at offset 3704, and if the value in d1 satisfies a range condition relative to a global word at offset 3706, before calling another routine.
- `compute_two_axis_bounds`: Scales two global index variables by 16 with offsets, storing clamped results to four global words for the first and second axes.
- `compute_two_axis_bounds_alt`: Scales two global index variables by 16 with a different set of offsets, storing four clamped global words for the first and second axes.
- `process_table_entries_3`: Compares a byte in a global variable to one, iterates through a table using a0, performs several conditional checks on table entries, and if conditions are met, calls one routine, then calls another routine using a byte from a global pointer, then calls a third routine using a byte from another global pointer, and finally calls a fourth routine.
- `increment_and_call_2`: Increments d7 and compares it to 32, calls another routine if it is less than 32, sets the sign bit of d5 based on the result, and restores registers from the stack before returning.
- `execute_conditional_logic_10`: Saves registers to the stack, calls another routine, and if the result is not negative, calls another routine; if that routine returns a non-zero value, it checks if a byte from a global pointer is 13 and if a byte at offset 1 of that pointer is 20, otherwise it calls another routine, stores a longword from a global pointer into a global variable, calls another routine, and restores the global variable.
- `execute_and_restore`: Calls a routine, sets the high bit of d7, and restores d0, d1, and d7 from the stack before returning.
- `lookup_and_call`: Saves d0 and a0, loads a byte from a table using a0 and d0, and if that byte is not zero, calls another routine, masks d0, calls another routine, and sets d0 to 255 before returning.
- `check_attribute_conditions`: Calls two routines and, if the results meet specific criteria involving a byte from a global structure and specific bit tests, it sets a global byte to 255, calls another routine, and then sets the global byte to zero.
- `perform_conditional_sequence`: Calls a routine, and if the zero flag is set, calls another routine before returning.
- `check_structure_conditions`: Compares a byte at a global address to one, then if not equal, loads a pointer from a global address into a0 and performs several conditional checks against values at offsets 0, 2, 10, and 12 of the structure in a0, eventually calling another routine with d0 set to 169 if certain conditions are met.
- `process_structure_callbacks`: Checks a bit in a byte at offset 2 of a structure in a0, then performs a series of conditional calls using pointers retrieved from global tables and bytes from the structure.
- `update_and_call_structure`: Copies d7 to a global word, sets a byte based on the sign flag, calls another routine, loads a pointer from a global address into a0, calls another routine, sets d5 to 255, and branches to another location.
- `execute_nested_callbacks`: Calls two different routines using pointers retrieved from a table and a byte from a structure, then sets d5 to negative one and branches to another location.
- `update_d7_and_call`: Increments d7, compares it to thirty-two, and if it is less than thirty-two, calls another routine and sets d5 based on the result.
- `validate_global_state`: Compares several global words and bytes, and if they match specific criteria, calls a routine and potentially another routine with d0 set to 120.
- `update_global_state_3`: Calls two routines, writes a word and a pointer to two global variables, calls a third routine using an address from a structure, and if a word from a global variable is one, calls a fourth routine before branching.
- `fetch_and_process_index`: Copies two bytes from a table into global variables, stores a word in a global variable, adds d0 to itself, loads a word from a table into d0, calls a routine, copies a byte into a global variable, stores a longword in a global variable, calls two more routines, and restores registers before returning.
- `process_conditional_file_data`: Saves registers, calls two routines, branches if the first returns a negative value, checks a byte from a global variable against specific values, calls a routine, loads an address into a0, calls another routine, calls a third routine, and if a global byte is zero, copies a word from one global variable to another before returning.
- `initializes_blit_state_and_calls`: Saves d0 and a0, clears a global longword and a second global longword, stores a pointer to a global variable into another global longword, sets a word-sized global value to 24 and clears a word at offset 18 of the structure in a0, then calls another routine and restores d0 and a0 before returning.
- `initialize_and_fill_table`: Clears eight bytes starting at a global address, then iterates through a loop eight times, using a byte from a second global table as an index to write the current loop counter into the cleared memory, and finally calls another routine.
- `loop_and_write_table`: Loads pointers into a0 and a1, sets d7 to 11, and enters a loop that reads a byte from the pointer in a0, extends it to a word, writes the value 11 to a location calculated from the byte and a1, and decrements d7 until it reaches -1 before jumping to another routine.
- `execute_sequence_of_calls_10`: Sets d0 to 10, calls one routine, calls a second routine, loads a pointer from a global address into a0, calls a third routine, calls a fourth routine, sets d0 to 18, and jumps to another routine.
- `dispatch_complex_blit_sequence`: Acts as a jump table entry that calls execute_complex_blit multiple times with varying parameters, updates a global variable, and performs two separate blitter operations.
- `iterates_twelve_entries_calling_handlers`: Runs a countdown loop of twelve iterations over a global byte array, calling another routine for each entry and, whenever the current byte is nonzero, calling a further three routines in sequence.
- `release_queues_then_call_handler`: Calls one routine to release queued objects, then calls the handler at offset 30 of a global structure pointer when that pointer and handler are nonzero, and jumps to a fixed routine to return.
- `update_scaled_value_2`: Calls a routine, checks if d0 is non-zero, if so it extends d0 to a word, stores it in a global variable, adds it to a pointer in a0, checks a byte at that pointer, and if non-zero clears it, calls two routines, extends d0, stores it in a global variable, loads an address into a0, calls another routine, and branches if the global word is non-zero.
- `set_global_pointers`: Writes two words to a global structure, fetches a byte from a table, calls a routine, and then swaps two global words before jumping to another routine.
- `store_words_and_jump`: Loads a pointer into a0 from a global address, stores a word from a global address at the location pointed to by a0, stores another word from a global address at the location pointed to by a0 plus two, and then jumps to another routine.
- `swap_byte_at_offset`: Calls a routine, and if d0 is non-zero, extends d0 to a word, stores it in a global variable, calculates an address by adding d0 to a global pointer, and if the byte at that address is zero, stores a byte from a global variable there and clears that global variable; otherwise, it swaps the byte at that address with a global variable.
- `swap_and_store_byte`: Moves a byte from a source to a destination, extends it to a word, stores it in a global variable, and calls three different routines.
- `load_word_and_branch`: Loads a word from a global address into d0 and branches to another routine.
- `process_conditional_pointers`: Sets a global variable to zero, calls a routine, and if a global byte is zero, calls two other routines using pointers retrieved from a table and then calls a fourth routine based on a bit test.
- `set_d0_and_branch_2`: Sets d0 to 19 and branches to another routine.
- `execute_three_subroutines_3`: Loads the constant 18 into d0, calls three different routines in sequence, and then jumps to another routine.
- `load_and_call_three_routines`: Loads a byte from a longword at a specific address into d0, then calls three different routines before jumping to another routine.
- `initialize_handler_state`: Stores the value 3 in a global word, then calls three routines and jumps to another routine, provided that a global pointer and a value at an offset of 38 from that pointer are both non-zero.
- `sets_high_bit_on_flag_bytes`: Calls a helper to obtain a structure pointer, sets bit 7 of the byte at offset 12 of the structure at a0 when a global byte is nonzero, and sets bit 7 of the bytes at offset 0 and offset 6 when the byte at offset 4 of that structure is zero.
- `sets_flag_bit_then_calls_helpers`: Calls a routine and, if the zero flag it leaves is set, sets bit 7 of the byte at offset 6 in the structure in a0.
- `dispatch_conditional_routines`: Calls two routines, checks a byte at a global address, and if that byte is non-zero, calls a third routine before branching to another location.
- `load_and_dispatch_words`: Sets the sign bit of a byte at a global address, loads two words from a structure into global variables, and if the third word is not zero, writes it to a global variable and calls two other routines.
- `program`: Calls an initialization routine, a main routine, and an exit routine before returning.
- `check_attribute_and_call_2`: Tests a byte at a global address, branches if non-zero, compares a word at a global address to 8, and if it is not greater than or equal to 8, sets d0 to 176 and calls a routine before returning.
- `process_attribute_list_and_calculate`: Iterates through a list of six bytes, calling two different routines for each non-zero byte to retrieve and process data.
- `calculate_scaled_value_19`: Performs arithmetic on d7 using a subtraction, a multiplication, and a division, then branches to another instruction.
- `execute_parameterized_call`: Loads the word 24 into d0, calls another routine, loads the word 95 into d7, clears a long value in a global variable, and branches to another location.
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
- `compute_scaled_value_and_set_pointers`: Starts from zero and, when a byte at offset 4 of the structure in a1 is nonzero, sums two words at offsets 210 and 212, subtracts the word at offset 208, and scales the result by d2 divided by 100 with a minimum of one.
- `stores_pointers_and_calls_handlers`: Loads the address of a global variable into a0, calls another routine, stores that address into two global longword slots at offsets 8 and 16, and then calls a further routine before restoring the saved registers and returning.
- `initialize_parameters_and_branch`: Tests a byte at a global address, and if it is zero, calls another routine, otherwise it sets d0, d1, d2, and d3 to specific values and branches to another routine.
- `initialize_loop_parameters`: Compares a byte at a memory location to 2, and if it is not positive, loads 16 into d0, 49 into d1, 11 into d2, and 9 into d3 before performing a branch.
- `perform_nested_blit_loops`: Executes nested loops that call a routine 10 times in the inner loop and 9 times in the outer loop, performs arithmetic on d1 and d2, calls a routine with a calculated offset, and finally updates a global byte and calls two more routines.
- `test_byte_and_execute_sequence`: Tests a byte in a global variable and, if non-zero, calls another routine, loads a pointer into a0, calls another routine, stores a byte into a global variable, and branches to another routine; otherwise it returns.
- `call_and_jump_3`: Calls a routine and then jumps to another location.
- `set_flag_bit_if_global_set`: When a global byte is nonzero, sets bit 7 of the byte at offset 12 in the structure at a0.
- `process_status_flags_3`: Loads a word from a memory location into d7, calls a routine, and then performs several conditional checks involving bit testing and bitwise inversion on values at various memory locations, calling other routines based on the results.
- `process_status_flags_2`: Calls another routine with a byte from a pointer in d6, checks bit 1 of a byte at a pointer in d6, and if bit 4 of d7 is set, toggles bit 1 of a byte at a pointer in d6 and conditionally calls two other routines.
- `calls_helper_based_on_flag_bit`: Loads a global byte, calls another routine with it, and tests bit 0 of a second global byte.
- `execute_handler_calls_2`: Loads 60 into d0, calls another routine, and if d0 is zero, calls two more routines before performing an indirect jump.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `execute_conditional_logic_8`: Loads 34 into d0, calls a routine, stores the result in a byte at a memory location, writes 96 into a word at a memory location, loads an address into a0, calls another routine, and then performs conditional branches based on a word at a memory location and the byte stored earlier.
- `execute_conditional_calls_4`: Tests a byte in a global variable and, if non-zero, branches to another routine; otherwise, it loads a value into d0 from a pointer in a global variable, calls another routine, and branches to another routine.
- `execute_table_and_data_routines`: Checks a global byte, performs a loop to copy data from a table, calculates an offset, calls several routines, and updates a value in a structure via a pointer.
- `process_pointer_and_clear`: Calls two routines, stores a byte from a register into a memory location, and enters a loop that checks a series of bytes in a structure.
- `update_global_pointer_via_validation`: Validates a value from a global variable and, if non-zero and a subsequent check passes, stores that value into another global variable and calls a final routine via a jump.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `execute_blitter_setup`: Clears a 16-bit word in a global variable, performs two routine calls with specific register parameters, writes 31 to a global variable, performs a third routine call with different register values, and jumps to another routine.
- `blits_rectangle_and_returns_result`: Calls a routine to obtain a value, stores 31 into a global variable, and passes the value to another routine.
- `process_scaled_tile_data_2`: Shifts d0 right by 8 bits, and if the result is non-zero, calls a routine, calculates an offset into a table using d0, checks a byte at that offset, writes d0 to a global variable, sets two global variables, performs a routine call with d0, calls another routine, and finally calls three more routines.
- `clips_and_blits_when_nonzero`: When the high byte of d0 is nonzero, decrements a 16-bit index in d0 and calls a routine to fetch a table entry, then if the byte at offset 2 of that record is nonzero clears and sets two global variables and calls routines.
- `process_scaled_data_and_callbacks`: Shifts d0 right by eight, and if the result is not zero, stores it in a global variable, calls a routine, calculates an offset into a table, performs arithmetic, calls three routines, and then calls another routine after checking a global byte and a byte at a pointer.
- `release_queued_objects_and_return`: Calls one routine that releases queued objects and then returns.
- `blits_rectangle_when_nonzero`: When the high byte of d0 is nonzero, passes the value to a routine, clears one global variable, sets another to 65535, calls routines, and then clears the second global variable.
- `modify_control_flags`: Sets d0 to 255, compares a byte at a global address to 2, and if it is not less than 2, checks a byte at a global address; if that byte is non-zero, it clears bits 4 and 5 in d0 before branching.
- `modify_global_flags`: Checks bits in a global variable to clear specific bits in register d0, then calls several routines to update a global word and perform scaled blitting operations.
- `call_conditional_routine_3`: Loads 224 into d0, 125 into d1, and 30 into d3, calls a routine, and if the result is not zero, calls another routine before returning.
- `toggles_flag_bit_when_set`: Checks that the high byte of d0 is nonzero and that the bit it names is set in a global byte; if so, it XORs a bit selected through a table lookup into a global word and calls two other routines.
- `clears_flag_and_clips_when_bit_set`: Tests whether the high byte of d0 is nonzero and whether the bit numbered by that byte is set in a global byte.
- `branches_to_clear_blit_routine`: Tests bit d6 of d0 and, when it is clear, loads a fixed structure address into a0; otherwise it branches past that load to a shared continuation.
- `blits_rectangle_and_clears_flag`: Loads a fixed structure address into a0, passes the constant 232 in d0 to a blit routine, and clears a global variable to zero before returning.
- `draws_bit_mask_rectangles`: Reads a global byte and, for each of five bits starting at bit 1, writes 28 or 31 into a global word depending on whether the bit is set, calls a rectangle routine, and calls another routine for set bits.
- `blits_rectangle_per_bit_in_mask`: Writes the constant 31 to a global word, copies a1 into a0, sets d0 to 65 and calls a routine at a fixed absolute address, then copies a0 back into a1.
- `branches_to_blit_loop_routine`: Tests bit d0 of d2 and, when it is clear, loads a fixed structure address into a0; otherwise it branches past that load to a shared continuation.
- `blits_rectangle_in_loop`: Loads a fixed structure address into a0, passes the constant 232 in d0 to a blit routine, adds 7 to d1, and increments a counter in d7 until it reaches 6.
- `update_data_from_buffer`: Checks a byte at offset 2 of a buffer; if the lower two bits are not 2 or 3, it stores a byte from the buffer into a global word, then optionally calls another routine twice to store results into global bytes at offsets 34 and 35.
- `store_dual_values`: Copies the value from d2 into a global variable via a routine call, then repeats the process with a different constant in d1 to store a second value in another global variable.
- `preserves_d0_around_two_helpers`: Pushes d0 onto the stack, calls a helper with the global byte at a fixed address, calls the same helper with the next global byte, then pops d0 back and returns.
- `zeroes_48_words_then_calls`: Writes zero to 48 consecutive words starting at the address returned by a call, using a countdown loop, then calls another routine and returns the value in d0.
- `execute_multiple_subroutines_5`: Sets three global words to zero, calls two routines, and if a byte in a global variable is zero, performs a sequence of calls and pointer retrievals.
- `call_with_constant_3`: Loads the word 119 into d0, calls another routine, and returns.
- `execute_conditional_blits_2`: Loads a byte from a global variable, calls two routines, and if two specific global bytes are both zero, calls three more routines using bytes from three different global locations before jumping to another routine.
- `call_and_jump`: Calls one routine and then jumps to another routine.
- `call_two_and_jump`: Calls two different routines in sequence and then jumps to another routine.
- `update_sprite_and_sync_state`: Retrieves a structure via a pointer, updates two global words with values from offsets 24 and 26 of that structure, and performs a series of calculations and calls to update sprite or object properties.
- `test_and_dispatch_list`: Tests a byte at a global address, and if it is not zero, sets the sign bit of that byte, calls three other routines, and jumps to another routine.
- `sets_status_bits_in_structures`: Calls a routine, then conditionally sets bit 7 of a byte in the structure in a0 and sets bit 7 of bytes at offsets 6 and 12 in another structure.
- `process_event_callback_chain`: Loads a byte from a global structure, calls a routine with a constant in d7, and if the result is zero, checks a byte in another global structure to decide whether to call a sequence of three routines or a different sequence of three routines based on the status of two additional bytes in the global structure.
- `dispatch_conditional_callbacks`: Calls four routines in sequence, tests a byte at a memory location, and if it is zero, compares a word from a memory location to negative 1, calling another routine if the word is not negative 1.
- `dispatch_conditional_call_2`: Calls a routine and, if the result is not negative, sets d0 to 107 and calls another routine.
- `initialize_and_dispatch_event`: Writes several values to a structure pointed to by d6, calls two routines, and then conditionally calls a third routine if a value in d0 is not negative, followed by a check of a byte in a global variable to decide whether to call a final routine.
- `execute_multiple_subroutines_3`: Loads the word 114 into d0 and calls five different subroutines in sequence before returning.
- `initialize_event_structure_3`: Sets several global values, clears a word at offset 24 of a structure in a0, calls two routines, and performs conditional logic based on the result of a routine call and a global byte.
- `call_and_branch_6`: Loads the word 114 into d0, calls another routine, and branches to another location.
- `call_subroutines_conditionally`: Tests a byte at a global address and, if it is zero, calls four subroutines in sequence; otherwise, it calls four subroutines after branching.
- `dispatch_event_data_2`: Pushes a value to the stack, retrieves a byte from a global variable to use as an index for a routine call, and then uses the resulting pointer to check a bitmask.
- `dispatch_conditional_event_2`: Tests bit 1 of d0; if the bit is not set, it branches to another routine, otherwise it loads 99 into d0, calls another routine, and branches to another location.
- `dispatch_conditional_event`: Tests bit 10 of d0; if set, it loads the constant 100 into d0, calls another routine, and branches to another location, otherwise it branches to another location.
- `test_and_call`: Tests a word at a global address, otherwise if the value is not zero, loads the word 97 into d0, calls another routine, and branches to another location.
- `dispatch_event_sequence`: Calls a routine, and if the result is positive, calls another routine with a constant in d0.
- `compare_and_call_4`: Compares a word at a global address with 6, otherwise if the value is not negative, loads the word 121 into d0, calls another routine, and branches to another location.
- `fetch_and_test_pointer_byte`: Calls a routine to retrieve a pointer, fetches a byte from that pointer, and then calls another routine to check that byte; if the byte is non-zero, it calls a third routine with the value 115 in d0.
- `test_and_store_byte`: Loads the value 5 into d0, calls a routine, and if the result is positive, stores a byte from a pointer in d6 into a global variable before calling another routine.
- `update_event_state`: Calls a routine, stores a pointer from the result into a global variable, writes 110 into a memory location, sets a byte at a memory location to 1, clears a longword at a memory location, calls a routine, and performs several conditional checks and calls based on the state of a memory location.
- `update_event_state_variant`: Calls a routine, stores a pointer from the result into a global variable, writes 111 into a memory location, sets a byte at a memory location to 2, clears a longword at a memory location, calls a routine, and performs several conditional checks and calls based on the state of a memory location.
- `load_table_entry_by_byte`: Reads the byte at the address in a0, scales it by 4, and returns the 32-bit longword from a global table at that offset, preserving the caller's a1 value.
- `scale_and_copy_data`: Saves registers, calculates an offset into a table using d2, copies two words from that table into global variables, calls two routines, calculates a new offset using multiplication, copies a byte from a memory location into a structure, and calls three more routines.
- `copy_and_process_data`: Copies two words from a source address to a destination address, calls three subroutines, and uses d0 and d1 to pass values to a subroutine using a global address in a0.
- `process_scaled_coordinates`: Saves registers, loads an address into a0, loads two words from memory into d0 and d1, sets d2 to 10, and calls three routines before returning.
- `loop_and_test_byte`: Performs a loop that runs up to six times, testing a byte at a pointer in a0 and incrementing d6 if the byte is not zero, then branches to another routine.
- `update_graphics_and_table`: Stores a byte at a negative offset of a pointer in a0, calculates an offset into a table using a global word and a value in d0, calls a routine, and then enters a loop that sets bits in a bitmask based on bytes read from a table.
- `update_graphics_structure_3`: Calculates a value based on d7, stores it in a local stack area and a global word, calls a routine, and then enters a loop that copies bytes from a table to a global location, performing various routine calls and updates to a global word and a structure byte.
- `test_and_call_routine`: Tests a byte at a specific address; if it is nonzero, it calls a routine and then branches to another routine, otherwise it calls a routine and then branches to another routine.
- `calls_blit_state_initializer`: Calls the blit state initialization routine and then returns, without touching any data itself.
- `resets_blit_globals_and_sets_height`: Saves a0 on the stack, clears a global longword and another global longword, stores a pointer to a global variable into a third global longword, and writes 24 into a word-sized global at offset 0 of a fixed address, then clears the word at offset 18 from that address.
- `call_two_and_jump_2`: Calls two different routines and then jumps to another routine.
- `initialize_and_update_tile_metadata`: Sets four 16-bit values in a table, calls several routines to retrieve a structure, and updates two global words with values from offsets 24 and 26 of that structure.
- `calculate_and_update_tile_data`: Calculates a value based on a structure offset and a multiplication by 6, then updates two global words with values from offsets 24 and 26 of a structure.
- `parse_blit_metadata_stream`: Calculates and stores several word-sized offsets and counts in global variables based on values read from a data stream, then iterates through the stream to perform table lookups and conditional calls to update structure values.
- `validate_and_lookup_blit_metadata`: Checks if a value in d2 is within bounds, then uses it to index into a table to retrieve a new value in d2 via a call.
- `process_blit_data_stream_metadata`: Iterates through a data stream to extract and validate metadata, performing table lookups and conditional calls to modify structure values based on bitmask checks and specific byte values.
- `call_three_and_branch`: Calls three different routines in sequence and then branches to another routine.
- `check_bits_and_set_d2`: Loads a byte from offset 2 of a structure in a0, masks it with 3, and compares it to 2.
- `process_blit_data_stream`: Checks a bit in a global variable and, if set, executes a loop that iterates up to 32 times, calling another routine to process data and updating a pointer in a0 by an offset.
- `process_loop_with_callbacks`: Compares a global word to 6, otherwise enters a loop that calls another routine and checks its result, performs a second routine call with specific values in d0, d1, and d2, and then clears two global words before jumping to another routine.
- `perform_sequential_updates`: Loads three bytes from consecutive global addresses into d0, calls a routine for each byte, calls another routine, and then jumps to another routine.
- `load_constant_and_call_three`: Loads the constant 11 into d0, calls three different routines in sequence, and then jumps to another routine.
- `process_table_and_jump`: Calls two routines, checks if a global word is greater than or equal to 250, clears a global word if so, increments a global word by 4, and if a specific table entry is zero, resets that global word to 0 before calling two more routines and jumping to another routine.
- `increment_word_counter`: Adds 1 to a 16-bit global variable at a fixed address, then returns.
- `calculate_table_indices`: Performs arithmetic on two global words to calculate new indices, checks if these indices fall within a specific range defined by two other global words, and if so, stores them in a global table and calls another routine.
- `call_conditional_routine_7`: Calls a routine, moves d0 to d7, tests a byte at a global address, and if it is zero, sets d0 to 51 and calls another routine before branching to another routine.
- `test_and_call_parameter`: Tests a byte in a global variable, and if it is zero, loads the word 137 into d0, calls another routine, and then branches to another routine.
- `update_table_values_7`: Calls a routine, and if the result is non-zero, retrieves three bytes from a table at an offset calculated from a pointer in d6, stores them in a global structure, calls another routine, and then performs arithmetic on the difference of the stored values to either call another routine or enter a loop that calls a routine multiple times.
- `extend_and_divide_counter`: Extends the sign of a 16-bit value in d3 to a 32-bit value, divides it by 4, and then branches to another routine.
- `execute_looping_calls`: Decrements a counter in d3 in a loop, calling one routine during each iteration, and then calls two more routines before returning.
- `call_and_branch_on_sign`: Calls another routine, tests the sign of d0, and if it is negative, calls a third routine before branching to another routine.
- `compare_and_call`: Compares a word at a specific address with d0; if they are not equal, it calls a routine and then branches to another routine, otherwise it calls a routine and then branches to another routine.
- `update_tile_offset`: Calls a routine, stores the result in a global variable, pushes d0 to the stack, loads a byte from a global address, calls another routine, calculates an offset by multiplying d0 by 20 and adding it to a global pointer, stores the constant 27 in a global variable, and calls a final routine.
- `set_callback_index_one`: Sets register d0 to 1 and branches to another routine.
- `set_callback_index_three`: Sets register d0 to 3 and branches to another routine.
- `set_callback_index_five`: Sets register d0 to 5 and branches to another routine.
- `set_callback_index_seven`: Sets register d0 to 7 and branches to another routine.
- `dispatch_indexed_callbacks`: If d0 is non-zero, it masks d0, decrements it, and uses the result to index into a table of pointers to call a routine; if bit 0 of d0 is set, it uses a second table to call another routine, and finally jumps to another routine.
- `decrement_word_if_flag`: If the byte in global variable d1 is nonzero and a 16-bit global word is nonzero, subtracts 1 from that word; otherwise it returns.
- `advance_word_by_two_limited`: Saves d0, and if the byte flag is nonzero and the global word at offset 594 is greater than the word at offset 596 minus 19, it adds 2 to that word; then it restores d0 and returns.
- `advance_word_by_one_limited`: Saves d0, and if the byte in d1 is nonzero and a global word is less than a second global word minus 13, it adds 1 to the first word; then it restores d0 and returns.
- `decrement_word_by_two_if_flag`: If the byte in d0 is nonzero and a 16-bit global word is nonzero, subtracts 2 from that word, then returns.
- `waits_releases_then_calls_handler`: Calls the flag-wait routine and the object queue release routine, then calls the handler at offset 30 of the structure pointed to by a global longword, if that pointer and handler are both nonzero.
- `searches_record_table_for_coordinates`: Computes two coordinates from global words, and if both are nonzero and within bounds, scans a table of 20-byte records for one whose first two bytes match them, stopping early when a flag is clear.
- `searches_table_for_matching_pair`: Scans a table of twenty-byte entries from the start pointer in a0, comparing the first two bytes with the values in d4 and d5 and advancing the index in d6 while decrementing the count in d7.
- `advances_pointer_and_calls_helper`: Advances the pointer in a0 by 20 bytes and decrements the loop counter in d7 to skip ahead to the next table entry, then saves d0, loads the byte at absolute address 40 into d0 and calls a routine, restores d0 and the saved registers, and returns with d6 set to 255 in the low byte of d0.
- `dispatches_indexed_value_handler`: Compares d2 with 9 and, when equal, calls a routine to fetch a value, looks up a word from a table indexed by the low byte of d3 and walks a linked chain of 12-byte records, calling a routine when a record's first byte is 2 until the next-link word is 65535, then calls one more routine.
- `loads_byte_and_selects_bit_index`: Clears d0, loads a byte at offset 2 of the structure at a1 into d0, sets d1 to 2 and d2 to 3, and calls a routine.
- `saves_registers_and_calls_routine`: Pushes d0 onto the stack, loads the byte at the absolute address 40 into d0, calls a routine, then restores d0 and the saved registers before returning.
- `scans_linked_entries_for_type_three`: When d2 equals 13, it walks a linked chain of 12-byte entries in a global table, and on finding an entry whose type byte is 3 with a nonzero flag byte and a clear global byte, it sums two words from a structure and calls routines depending on the sign of that sum.
- `adjusts_pointer_when_flag_set`: When the byte at offset 2 of the structure in a1 is nonzero and a global byte is zero, it reads two words from a structure returned by another routine, sums them, and calls another routine, skipping the rest when the sum is negative.
- `loads_byte_and_branches`: Clears d0, loads the byte at offset 4 of the structure in a1 into d0, sets d1 to 1 and d2 to 3, calls another routine, then branches on its result and tests the byte at offset 1 of a1 to choose the value placed into d2 before a short branch.
- `saves_registers_then_calls_routine`: Pushes d0, loads the byte at absolute address 40 into d0, calls a routine with d2 set to 15, then restores d0 and the saved registers before returning.
- `prepare_scaled_blit_parameters_3`: Calculates dimensions and offsets for scaled graphics transfers by manipulating values from a table and calls a routine to prepare specific memory regions.
- `process_blit_loops`: Calls a routine, increments a global word, and enters a loop that decrements d2 and calls a routine if a byte in a buffer is non-zero, then performs further calculations and loops using values from a global variable and a pointer in a1.
- `perform_blit_operations`: Calls a subroutine, increments a global word, and enters a loop that decrements d2 and calls a subroutine if a byte in a buffer is non-zero; it then performs further operations involving a pointer in a1 and a global word.
- `execute_blit_loop`: Increments a global word, performs a loop that calls a routine, and then enters a nested loop that calls a routine if a byte in a buffer is non-zero.
- `execute_scaled_blit_sequence_3`: Enters a loop that increments a global word and calls a routine, then performs a series of scaled blit operations using various global pointers and tables, including a nested loop that checks a global word against a table value before calling a routine.
- `perform_scaled_blit_loop`: Calculates a destination address using division and multiplication, executes a scaled blit, and then enters a nested loop to increment global counters and call routines.
- `calculate_blit_offset_5`: Retrieves two bytes from a table, performs a conditional check, and if certain bits are set, calculates a destination address and calls a scaled blit routine.
- `offsets_source_pointer_and_blits`: Saves d3 and a0, computes a row offset of 192 times the low two bits of a global word, adds it to a0, calls a clipped bitplane blit routine, and then restores the saved registers and returns.
- `execute_complex_blit_sequence_4`: Saves registers, retrieves a byte from a global address, calls several subroutines with various parameters to perform scaled operations, and conditionally executes further subroutines based on bits in a global word.
- `execute_blit_and_modify`: Calculates a destination address through division and multiplication, calls a scaled blit routine, and then calls another routine to modify a value from a global pointer.
- `check_blit_parameters_2`: Saves registers, calls a routine to retrieve a pointer, sets up several registers with constants, and then checks a byte from a structure to decide whether to branch to different locations.
- `check_blit_condition`: Compares register d0 to 8, tests bit 1 of a memory location, and branches based on the results.
- `process_scaled_blit_parameters`: Checks bits in d0 and a bit in a table, then calculates a destination address using a multiplication, calls a routine, and performs a loop that calls another routine and a third routine while updating d0 and d1.
- `saves_byte_then_calls_routine`: Pushes d0, loads the byte at absolute address 61 into d0, calls a routine, then restores d0 and the registers d0-d7 and a0-a4 before returning.
- `combines_bitplane_bytes_into_buffer`: For values of d2 between 1 and 19, it saves registers, walks a 4-iteration loop reading globals, then writes eight bytes of a global mask ANDed with source bytes into a buffer, and optionally returns early on a flag, otherwise storing a decremented byte value.
- `fetch_and_store_byte_2`: Calls another routine using a value from a global pointer and a constant in d1, stores the resulting byte in a global variable, and sets the sign bit of a global variable.
- `execute_two_routines_4`: Pushes d0 to the stack, calls a routine, loads a byte from a global pointer into d0, calls another routine, and returns.
- `process_structure_data_8`: Checks a global byte, and if non-zero, calls two routines using values from a structure, then updates the global byte.
- `walks_bit_grid_calling_handlers`: When a global byte is zero, it calls two routines, then performs eight passes testing bit 1 of a value returned by another routine and, when it is clear, calling a handler three times, and finally calls one more routine.
- `loops_over_cells_and_marks_bits`: Runs nested countdown loops over a grid, calling routines for each cell and testing bit 1 of a value read from a register, returning when the outer counter reaches the end.
- `saves_registers_and_dispatches_index`: Saves d0, d1 and a0 on the stack, compares a global word against 6, and when it is not equal branches to a routine; otherwise it sets d0 to 255 and branches to another routine.
- `computes_index_and_calls_routines`: Computes a table index from d0 and d1 minus one, multiplied by a global width value and masked to a 3-bit value, passes it to a routine, then calls another routine and returns.
- `set_bitmap_cell_and_mark_dirty`: Converts the 1-based coordinates in d0 and d1 into a bit index using a global width, and if the bit in the buffer at a0 is clear, sets it and writes 255 to a global variable.
- `calculate_and_store_data`: Clears a global word, calls another routine to get a pointer, loads a word from a structure at an offset, calculates a pointer into a table using a global word and multiplication, and enters a loop that tests bits and writes a four-byte sequence to a destination based on several conditional checks.
- `test_bit_and_branch_2`: Loads a word from a global address into d4, adds 5 to it, tests bit 4 of d0, and if bit 4 is not set, sets d0 to 2 and branches to another location; otherwise it returns.
- `compare_and_branch_byte`: Loads a byte from offset 1 of the structure in a1 into d0, and if d0 is zero, branches to another location; otherwise, it compares d0 with d6 and branches to another location if d0 is negative, or sets d0 to 3 and branches to another location.
- `calculate_and_fill_data`: Divides d6 by d0, compares the result to 100, and if it is not less than 100, enters a loop that writes four bytes to a destination pointed to by a0, increments pointers, and conditionally calls two routines if a loop counter reaches 31.
- `call_subroutine_and_return_2`: Calls another routine and then returns.
- `update_scaled_parameters`: Sets register values and calls two different routines, retrieves a word from a table using a value from a global variable and an offset, stores that word at offset 18 of a structure in a0, calls another routine, and jumps to another routine.
- `process_list_element`: If a global word is non-zero, it calls another routine, loads a word from a global variable into d1, adds d1 to itself, calls another routine, stores the result from d0 into a memory location at offset -2 from a pointer in a0 plus d1, and calls a final routine before returning.
- `process_blit_sequence`: Loads several constants into d0 through d4 and calls another routine, then jumps to another routine.
- `updates_rows_and_writes_marker_bytes`: Calls the clipped blit routine and the row-writing routine, then computes a row count from two global bytes capped at 16 and loops over table entries, storing 31 or 28 into a global byte depending on a table byte.
- `tests_byte_and_loads_constant`: Tests the byte addressed by a1 and, if it is nonzero, branches into the code at a later address.
- `loops_rows_writing_bytes_to_pointer`: Repeatedly sets a global byte to 28 and loops over entries, where when a flag byte at offset 2 is nonzero it stores 41 and 0 through a0 and calls another routine, returning once the counter reaches zero.
- `check_and_store_tile_byte`: Shifts d0 right by 8 bits, and if non-zero, checks if a calculated address is within bounds and points to a zero byte, then stores a byte from that address into a global variable, performs a routine call, sets two global variables, performs a routine call with a calculated value, and calls three more routines.
- `marks_range_entries_and_resets_pair`: If d0 high byte is nonzero and a computed index is within a limit, it either clears a global longword pair with 0 and 65535 around two calls or sets a global byte to 31 and calls two routines, depending on a table entry.
- `selects_constant_when_byte_is_one`: Compares the byte at the address in a0 with 1 and, if it differs, loads 27 into the low byte of d3 and branches to a later point.
- `sets_d3_and_branches_back`: Loads the constant 241 into the low word of d3, calls a routine, and then branches unconditionally to a later point in another routine.
- `clears_flag_pair_around_calls`: Calls another routine, writes zero to a global longword, writes the value 65535 to a second global longword, calls another routine twice, then clears that second global longword to zero before returning.
- `calculate_index_and_call_2`: Saves registers, calculates an index into a table using two global words, retrieves a byte from that table, and if bit 7 is clear and bit 6 is set, performs a series of operations involving setting global words, calling a routine, and potentially calling another routine with a shifted d0 value.
- `update_status_and_call_2`: Shifts d1 left by 8 bits and stores it in d0, then calls a routine, stores a byte from a pointer in a1 into a global variable, sets a global word to a constant, loads a pointer from a global address into a0, calls another routine, and if d0 is not negative, stores d0 in a global variable and stores two words from a global address into two global variables before branching to another location.
- `test_and_set_global`: Tests bit 0 of d0 and, if set, checks a byte at a global address; if that byte is non-zero, it writes the word 284 to a global address and branches.
- `set_event_parameters`: Stores two values into global memory, calls a routine, and then calculates a value using a bit shift and stores it in a global word.
- `execute_indirect_calls_2`: Stores a value in a global variable, loads a pointer from a table using an offset in d0, and if the pointer is non-zero, calls it; then tests a global byte and calls another routine if non-zero.
- `fetch_byte_from_table_conditionally`: Checks two global bytes for zero; if both are non-zero, it loads a pointer from a global address, retrieves a byte from an offset of 27 from that pointer into a register, and branches to another routine; otherwise, it branches to one of two other locations.
- `load_and_call`: Loads a word from a global address into d0, calls another routine, sets a byte at a global address, and branches to another location.
- `calculate_and_update_structure_3`: Calculates an offset into a global table using values from two global words, retrieves a byte from that table, and calls another routine.
- `call_with_parameter`: Loads the word 248 into d0, calls a routine, restores registers from the stack, and checks a byte in a global variable before returning.
- `iterates_seven_slots_and_dispatches`: Loops over seven bytes of a global array, and for each nonzero byte whose matching bit in d0 is set, copies that byte into a global variable, calls a routine and an indirect handler, then continues.
- `validate_graphics_and_decrement`: Saves registers, calls a routine to retrieve a pointer, calculates an offset into a structure, checks a bit in a table, and if a byte at offset 1 of a structure is not 255, decrements it before calling two more routines and performing several comparisons.
- `validate_and_execute_conditional_logic`: Fetches a byte from a global pointer, calls two routines, and checks if the byte equals five and if two values in a table are equal.
- `update_graphics_parameters_2`: Subtracts 40 from d0, subtracts 9 from d0, calls two routines, moves d2 into d0, calls a third routine, and restores registers from the stack before returning.
- `call_handler_and_execute_callback`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0, calls another routine, and then calls a third routine before returning.
- `returns_masked_word_or_advances`: Masks the word at offset 30 of the structure in a0 with 58496, and if the result is zero and a signed byte from a table indexed by a global word is nonnegative, calls two other routines and returns the second result; otherwise it returns the masked value.
- `call_handler_and_execute_callback_2`: Calls a routine, and if the zero flag is not set, loads a pointer from a table into a0, calls another routine, and then calls a third routine before returning.
- `clears_flag_bit_and_fills_table`: Reads the word at offset 30 of the structure in a0 and, if the 58496 mask bits are clear, reads a byte from a table indexed by a global word and clears the bit it names in offset 30, then calls a routine.
- `call_and_load_address_2`: Calls a routine, otherwise if the result is not zero, loads the address of a global variable into a0 and branches to another location.
- `call_two_routines_5`: Loads a fixed address into a0, then calls two routines in sequence before returning.
- `test_bits_and_call`: Loads a word from offset 30 of the structure in a0 into d0, tests bit 15 of d0, and if bit 15 is set, branches to another location; otherwise it tests bit 14 and if bit 14 is clear, sets d0 to 250 and calls a routine before branching to another location.
- `test_bit_and_call_2`: Tests bit 13 of d0, and if it is set, loads the word 249 into d0, calls another routine, and then branches to another routine.
- `set_structure_flags_3`: Clears bit 13 of d0, stores d0 at offset 30 of the structure in a0, stores the word 1 at offset 202 of the structure in a0, calls a routine, loads 57 into d0, calls another routine, and then calls a third routine using d7 in d0.
- `modify_status_bit`: Loads a word from offset 30 of the structure in a0, checks bit 13, and if it is set, clears bit 13, sets bit 14, stores the result back to offset 30, and calls a routine.
- `call_and_load_address`: Calls another routine, and if the result is non-zero, loads a fixed address into a0 and branches to another routine.
- `load_address_and_call_2`: Loads a long constant into register a0 and calls another routine before returning.
- `modify_word_and_call`: Loads a word from offset 30 of the structure in a0 into d0, tests bit 14 of d0, and if it is set, branches to another location; otherwise, it sets d0 to a constant, calls a routine, and branches to another location.
- `set_status_bits_5`: Clears bit 14 of d0, sets bit 13 of d0, stores the word in d0 at offset 30 of the structure in a0, and calls a routine.
- `modify_status_bit_2`: Checks bit 14 of a word at offset 30 of a structure in a0; if set, it clears bit 14 and sets bit 15 of that word before calling another routine and returning.
- `frag_jsr_bne_lea_rts_5`: Calls another routine, branches if the zero flag is not set, otherwise loads a long constant into register a0, calls another routine, and returns.
- `modify_word_and_call_alt`: Loads a word from offset 30 of the structure in a0 into d0, tests bit 15 of d0, and if it is set, branches to another location; otherwise, it sets d0 to a constant, calls a routine, and branches to another location.
- `clear_and_set_bits`: Clears bit 15 and sets bit 14 of a word at offset 30 of a structure in a0, then calls another routine and returns.
- `call_and_load_address_alt`: Calls a routine, otherwise if the result is not zero, loads the address of a global variable into a0 and branches to another location.
- `call_two_routines_alt`: Loads a fixed address into a0, then calls two routines in sequence before returning.
- `check_bits_and_call_3`: Loads a word from offset 30 of the structure in a0 into d0, performs a bitwise AND with a constant in d1, and if the result is zero, branches to another location; otherwise, it tests bit 15 of d0, and if set, branches to another location; then it tests bit 14 of d0, and if not set, it sets d0 to a constant, calls a routine, and branches to another location.
- `update_structure_values_17`: Clears bit 13 of a word at offset 30 of a structure in a0, adds two words from offsets 204 and 206 of the structure to a third word at offset 202, then calls three different routines.
- `modify_status_bit_3`: Loads a word from offset 30 of the structure in a0, checks bit 13, and if it is set, clears bit 13, sets bit 14, stores the result back to offset 30, and calls a routine.
- `frag_jsr_bne_lea_rts_21`: Calls another routine, and if the result is non-zero, loads a fixed address into a0, calls two more routines in sequence, and then returns.
- `conditionally_calls_two_routines`: Reads the word at offset 30 of the structure in a0; if the 58496 mask bits are clear and bit 6 is set, it calls two routines, otherwise it calls a routine that sets d0 from d7 and returns.
- `call_two_routines_and_branch`: Calls another routine, branches if the zero flag is set, otherwise calls another routine and branches to an earlier instruction.
- `process_pointer_table_data`: Retrieves a byte from a table using a pointer and index, calls another routine with that byte, and then calculates an offset into a structure to perform arithmetic and call two more routines.
- `compare_and_call_8`: Subtracts a byte from offset 1 of a structure in a0 from a byte at offset 28 of a structure in a1; if the result is positive, it pushes d0 to the stack, calls another routine, restores d0, sets d0 to 257, and calls another routine.
- `calculate_and_update_byte`: Calls two subroutines, compares a byte from a structure in a2 to d0, and if d0 is not positive, performs a multiplication, swaps the result, and adds it to a byte at offset 1 of a structure in a0.
- `call_handler_and_update_table`: Calls a routine, and if the zero flag is not set, subtracts two from a word loaded from a global variable, uses the result to index into a table to load two bytes into d1 and d0, and then calls two routines before returning.
- `frag_jsr_bne_lea_rts_alt_3`: Calls another routine, and if the result is non-zero, loads a fixed address into a0, calls two more routines in sequence, and then returns.
- `conditionally_advance_counter_and_add`: Reads the word at offset 30 of the structure in a0 and, when its masked bits are zero and the word at offset 26 is not 32767, increments that word at offset 26 and adds 250 to the longword at offset 270, then calls another routine and returns; otherwise it returns with d0 set to the masked value.
- `frag_jsr_bne_lea_rts_6`: Calls another routine, branches if the zero flag is not set, otherwise loads a long constant into register a0, calls another routine, and returns.
- `process_graphics_list_2`: Checks bits 13 and 14 of a word at offset 30 of a structure in a0, and if they are clear, performs a sequence of routine calls, manipulates a global byte, and enters a loop that iterates up to 10 times to process structure data and call routines.
- `process_attribute_updates`: Calls a routine and, if the zero flag is set, performs several operations including loading values from a structure, calling two different routines, and conditionally updating two words in a structure based on bit tests.
- `call_callback_66`: Loads the word 66 into d0 and calls another routine.
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
- `call_conditional_subroutines`: Calls a subroutine, branches if the zero flag is set, calls another subroutine, and then performs an unconditional branch to another routine.
- `process_table_and_bit`: Retrieves a byte from a table using an index in d7 and a pointer in a0, calls another routine, calculates an offset by multiplying a word from a global address by 6 and adding it to a new pointer, then checks bit 1 of a byte at that offset and calls another routine if the bit is clear.
- `clear_bit_and_call_subroutines`: Clears bit 1 of a byte at offset 3 of the structure in a0, then calls two subroutines using a global address and a value from a global address.
- `call_sequence`: Calls a routine, otherwise if the result is zero, calls another routine and then branches to another location.
- `update_structure_and_call_3`: Retrieves a byte from a table using an index from a global word, calls a routine, calculates an offset into a structure, checks a bit in a table, and if bit 6 is clear, calls another routine and then calls a routine with the value 269 in d0.
- `update_structure_pointers_7`: Copies a pointer and a word from a structure to another location, calls two routines, sets a byte to one, and then calls three more routines based on a conditional check.
- `call_callback_272`: Loads the word 272 into d0 and calls another routine.
- `update_structure_and_call_7`: If a condition is met, it retrieves a byte from a table using a value from a global structure, calls a routine with that byte, calculates an offset into a structure using a value from a global word multiplied by six, sets bit 0 of a byte at that offset, copies a pointer and a word to a new location, calls two more routines, and finally calls a routine with a pointer to a global address.
- `fetch_table_byte_and_call`: If a condition is met, it retrieves a byte from a table at an offset derived from a global word and then calls a routine with d7 set to five.
- `process_conditional_structure_data`: If a condition is met, it retrieves a byte from a global structure, calls a routine with that byte, then calls another routine with the value from a structure, and if bit 3 of that value is zero, calls a routine with the constant 440.
- `process_conditional_call`: Calls a routine, compares the result to five, and if equal, calls another routine before calling a third routine and returning.
- `frag_jsr_bne_lea_rts_7`: Calls a subroutine, branches if the zero flag is not set, loads the address of a fixed memory location into a0, calls another subroutine, and then returns.
- `initialize_scaled_blit_parameters`: Sets d0 to 48, d1 to 64, d2 to 4, d3 to 4, and clears a0, then calls another routine; subsequently sets d0 to 64, d1 to 80, d4 to 24, d5 to 3, d6 to 2, and d7 to 32, loads a table address into a0, and calls another routine; if a byte at a global address is not 2, it loads a word from a global address into d3, increments it, multiplies it by 48, subtracts 13, and branches to another location.
- `calculate_scaled_offset_8`: Performs arithmetic on a global word in d3, calls a routine, sets two global variables, performs a routine call with a value derived from d3, clears a global variable, and calls another routine before jumping to another routine.
- `check_and_call_258`: Compares a byte at a global address to 2, branches if it is not positive, otherwise loads the word 258 into d0, calls another routine, and branches to another location.
- `load_table_and_call_routines`: If a condition is met, it retrieves a byte from a table using an index derived from a global word, calls a routine, and then calls another routine with a global address loaded into a0.
- `fetch_table_byte_and_call_six`: If a condition is met, it retrieves a byte from a table using an index derived from a global word minus thirteen, and then calls a routine with d7 set to six.
- `call_two_routines_with_constants`: If a condition is met, it calls a routine with d1 set to 180 and d7 set to five, then calls another routine with d7 set to six.
- `test_and_call_routine_8`: Tests a word at a global address, branches if it is zero, otherwise loads the constant 283 into d0, calls another routine, and branches to another location.
- `check_and_call_244`: Compares a byte at a global address to 1, branches if it is equal, otherwise loads the word 244 into d0, calls another routine, and branches to another location.
- `execute_conditional_sequence_3`: If a condition is met, it calls two routines with d0 set to 46, and if the result is two, it sets a global word to one, calls a routine, and calls another routine with d0 set to five.
- `frag_jsr_bne_lea_rts_22`: Calls another routine, branches if the result is not zero, loads a specific address into a0, calls another routine, calls a third routine, and returns.
- `calculate_and_verify_offset`: Checks if bits 14, 15, and 16 of a word at offset 30 of a structure in a0 are zero; if so, it calls another routine, calculates a new value for a word at offset 102 of the structure based on a multiplication and subtraction, calls another routine, and if a global byte and a global word match specific values, calls two more routines.
- `frag_jsr_bne_lea_rts_alt`: Calls a subroutine, branches if the zero flag is not set, loads the address of a different fixed memory location into a0, calls another subroutine, and then returns.
- `execute_multiple_blits`: Clears a 16-bit value in a global variable, then performs three separate calls with specific register parameters and finally jumps to another routine.
- `draws_seven_pixel_row_pairs`: Stores 31 into a global variable, calls another routine, then computes a count from a global variable and repeatedly calls two routines in a loop, counting down until the count reaches minus one and then returns.
- `loops_drawing_rows_with_offsets`: Repeatedly calls two other routines, incrementing the value in d1 by 7 and the value in d2 by 1 per pass, and loops over the count held in d7 until it reaches minus one.
- `call_and_jump_list`: Calls a subroutine and then jumps to another routine.
- `test_and_call_routine_2`: Sets d0 and d1 to specific values, calls a routine twice, and if the result is non-zero, calls another routine before returning.
- `perform_scaled_operations`: Shifts d0 right by 8, checks if d0 is zero, adds a value from a global pointer to d0, stores d0 in a global word, performs several routine calls with various parameters, and finally calls another routine using d0 as an index.
- `clears_flag_pair_if_byte_in_range`: If the high byte of d0 is nonzero and the sum of that byte and a global byte is below 33, calls another routine, writes zero to a global longword, writes 65535 to another global longword, calls two other routines, and clears that second longword to zero.
- `frag_jsr_bne_lea_rts_23`: Calls another routine, branches if the result is not zero, loads a specific address into a0, calls another routine, calls a third routine, and returns.
- `returns_masked_word_if_nonzero`: Masks the word at offset 30 of the structure in a0 with 58496; if the result is zero and the word at offset 210 of the same structure is nonzero, calls two other routines and returns the second result, otherwise it returns the masked value.
- `execute_callback_and_update_structure`: Calls a routine, branches if the zero flag is not set, loads a pointer into a0 from a global address, then calls a routine to fetch a byte from a table and another routine to clear flags and sum offsets within the structure in a0 before returning.
- `clear_flags_and_sum_offsets`: Clears the word at offset 30 of the structure in a0, stores the sum of the words at offsets 204 and 206 into the word at offset 202, then sets a byte in a global variable area from d7 and jumps to another routine.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_conditional_callback_sequence_2`: Calls a routine, then branches if the zero flag is not set, otherwise loads a pointer into a0, calls a routine to fetch a byte from a table, calls a routine to advance a clamped word in a structure, and returns.
- `advance_clamped_word_in_struct`: When the masked bits of the word at offset 30 of the structure in a0 are zero, it stores a word from a pseudorandom-advanced value plus 1 into the word at offset 0 of a0, clamped to the word at offset 2, then calls another routine and returns its result.
- `validate_and_update_attribute_table`: Validates two 16-bit values against a table, checks if a byte at a specific offset matches two constant values, and if so, calls another routine with a constant value in d0.
- `process_table_and_loop`: Calls another routine, checks a byte in a global variable, and if it is at least 2, calls another routine, performs several stack operations, calls three different routines in a loop, and finally calls another routine and stores 255 in a global variable.
- `update_conditional_bytes`: Calls a routine, stores a byte at offset 13c of a global variable, and then calls another routine if the status flag is set.
- `execute_conditional_callback_sequence_3`: Calls a routine, then branches if the zero flag is not set, otherwise loads a pointer into a0, calls a routine to fetch a byte from a table, calls another routine to increment structure values, and returns.
- `set_bit5_flag_and_call`: When the masked bits of the word at offset 30 of the structure in a0 are zero, sets bit 5 of that word, then calls another routine and returns; otherwise it does nothing with the structure.
- `update_table_with_offsets`: If a global byte is zero, it calls two routines, calculates two offsets using global words and bit shifts, stores these and a byte into a structure, and then enters a loop of 24 iterations that increments d0 and d1 while storing a longword and calling a routine.
- `update_graphics_and_coordinates`: Retrieves a byte from a table, calls several routines to perform arithmetic and table lookups, and then calculates a pointer to a structure to copy two words into it before calling another routine.
- `store_pair_if_within_limit`: Compares the low byte of d1 with a word in a table entry selected by d7, and if it is not greater, stores the low byte of d0 and the low byte of d1 into that entry, then returns.
- `initialize_and_process_scaled_dimensions`: Sets two 16-bit values and clears four 16-bit values in global memory, then branches based on a byte value to either scale two 16-bit values by doubling them or copy them directly into a destination, before calling several routines and performing bitwise operations on global variables.
- `update_scaled_values_4`: Doubles two words from a global table, stores them in another location, calls a routine, and then performs four calls to another routine while passing bytes from a structure in a6.
- `calculate_scaled_offsets_3`: Stores two words from global memory into another location, calls three routines, and then calculates and stores two different values based on comparisons of the original words.
- `load_and_process_list`: Loads a byte from a global variable into d0, calls two subroutines, and then jumps to another routine.
- `setup_pointer_swap`: Loads specific values into d0, d1, d2, and d3, then calls two routines and jumps to another routine.
- `update_clipping_and_counters`: Calls a routine with a pointer from a table, performs several calculations involving global values and bit tests, updates a counter in a global variable, and calls two more routines before returning.
- `dispatch_table_handlers_by_nibble`: If the high byte of d0 is nonzero, it takes the low nibble minus 1, indexes a table of code pointers by a masked value, and calls the selected handler, calling it a second time when the adjusted value is odd.
- `decrement_word_by_two_if_flag_nonnegative`: If the byte in d1 is nonzero and a global word minus 2 is not negative, subtracts 2 from that word and returns; otherwise it returns.
- `decrement_counter_if_flag`: If the byte in a global variable is nonzero and the word counter in a global variable minus 2 stays non-negative, subtracts 2 from that counter; otherwise it returns without changing it.
- `increment_counter_if_below_limit`: If the byte in a global variable is nonzero and the word counter plus 2 remains below another global word minus 71, adds 2 to the counter; otherwise it returns without changing it.
- `advance_second_counter_bounded`: If the byte in a global variable is nonzero and a second global word counter plus 2 stays below another global word minus 55, adds 2 to that counter; otherwise it returns unchanged.
- `fills_grid_with_conditional_callbacks`: Masks d0 to a byte, calls two routines, then walks a grid of rows and columns sized by two global words, reading a word at offset 2 of each cell pointed to by a0 and calling another routine for each nonzero value, then returns the register pair d0 and d1.
- `scans_grid_for_high_byte_cells`: Masks d0 to a byte, calls two routines, then steps through a grid sized by two global words two bytes at a time, calling another routine for each cell whose byte is at least 101, and finally returns the register pair d0 and d1.
- `draws_bit_pattern_into_four_rows`: Reads a byte from a table indexed by the low nibble of d2, derives a row stride from a global word, then for four rows shifts the pattern left and sets or clears the bit selected by d0 in the word pair at the computed address, advancing by the stride each row.
- `mask_store_and_advance_pointers`: Ands d1 into a table entry indexed by d6 and into the word at the address in a0, advances a0 by d5, decrements d7 and loops back while it is not below zero, then saves d0, reads a byte from a global variable, calls a routine and restores the registers.
- `draws_rotated_bit_pattern_into_rows`: Reads a byte from a table indexed by the low nibble of d2, computes a row stride from a global word, then for four rows shifts the pattern left and sets or clears a rotating single bit in two words at the computed address, rotating the mask right by one bit per row.
- `mask_rotate_store_and_advance`: Ands d1 into a table entry indexed by d6, rotates d1 right by one bit, ands it into the word at the address in a0, rotates d1 left by one bit, advances a0 by d5, and loops decrementing d7 until it is below zero; then it saves d0, reads a byte from a global variable, calls a routine and restores the registers.
- `compute_scaled_value_into_global`: Saves d0, reads a word from a global variable, adds 7, shifts right by 3, multiplies by 10 and by a second global word, doubles the result, calls two other routines, stores the low byte of d0 into a global byte, then restores d0.
- `process_blit_sequence_3`: Copies eight words from one location to another, calls two routines with specific parameters, stores a byte from a register into a global variable, and then enters a loop to compare a value against a table before updating a global variable and jumping to another routine.
- `process_list_and_table`: Iterates through a list of bytes calling a routine for each, then retrieves a value from a table based on a global word and calls several other routines before jumping to a new address.
- `run_callback_sequence_2`: Performs two blit-related calls with specific parameters, then enters a loop that calls a routine multiple times while incrementing values from a table, and finally calls another routine and jumps to a third.
- `calls_sequence_then_returns_coordinates`: Calls a fixed sequence of routines, passing the constant 144 and a value one greater than the one returned by an earlier call, and then returns the word produced by its fourth-to-last call in d0.
- `clear_global_word_2`: Calls a routine, and if the return value is not zero and a word in a global variable is at least four, clears that word.
- `set_global_word_2`: Calls a routine, and if the return value is zero and a word in a global variable is less than four, sets that word to four.
- `divide_global_word`: Calls a routine, and if the return value is zero, divides a word in a global variable by four and updates it using a calculation involving the quotient and remainder.
- `increment_global_word_3`: Calls a routine, and if the return value is zero, divides a word in a global variable by four and updates it with a new value based on the quotient and a modulo-based offset.
- `process_table_callback`: Calls a routine, and if the return value is non-zero, calls another routine if a second routine returns a non-zero value.
- `execute_nested_calls_2`: Pushes a constant to the stack, calls a routine to get a pointer, uses that pointer to call another routine with an offset, and then calls a third routine after pushing and popping a byte from a global variable.
- `fetch_table_data`: Calculates an offset into a table by multiplying d0 by six, retrieves two words from that table, adds constants to them, and jumps to another routine.
- `execute_triple_call_jump`: Calls three subroutines in sequence and then jumps to another routine.
- `call_and_jump_sequence`: Calls a subroutine and then jumps to another routine.
- `waits_releases_and_pops_structure`: Calls the flag-wait routine and the object queue release routine, then, when a global longword is not the address of a global variable, decrements a global byte if bit 0 of the structure byte is set, steps the structure pointer back by 54 bytes, and calls a routine if bit 1 of the old byte is set.
- `releases_queues_and_calls_handler`: Calls the object queue release routine and then the flag-wait routine, then calls the handler at offset 30 of the structure pointed to by a global longword, when that pointer and handler are nonzero.
- `process_event_data`: Retrieves a word from a global variable, calls a routine, and stores the result in another global variable.
- `clears_flag_and_pops_structure`: If a global byte is nonzero it calls the flag-wait routine, then sets another global byte to 255 and calls the flag-wait routine.
- `store_and_jump`: Stores a byte into a global variable and then jumps to another routine.
- `execute_multiple_calls_7`: Saves registers to the stack, calls four different routines in succession, and then loads a word from a global variable into d0 before branching.
- `loop_and_call_routines_3`: Stores a word from d0 into a global variable, enters a loop that calls another routine and decrements d0 until d0 reaches negative one, calls another routine, restores registers from the stack, and returns.
- `compute_offsets_and_update_counter`: Calls another routine, derives several global offset words from a table and a byte at offset 12 of the structure, scans 32 ten-byte entries accumulating a running offset, and when the low byte of d0 is 200 and a byte at offset 19 is nonzero, decrements that byte.
- `copy_words_and_longs_to_globals`: Copies two words from one pair of global variables into another pair, then, depending on a global byte, either copies two more words or branches to another routine that copies three longwords.
- `copies_three_words_to_globals`: Compares the byte in a fixed global variable with 1 and, if equal, copies three words from one block of global variables into a second block.
- `copy_three_longs_to_globals`: Copies three 32-bit values from one block of global variables into a second block of three 32-bit global variables, then returns.
- `copy_words_and_longs_from_globals`: Copies two words from one global pair into another, then depending on a global byte either copies two words back or branches to another routine that copies three longwords back in the reverse direction.
- `copies_three_words_back_to_globals`: Compares the byte in a fixed global variable with 1 and, if equal, copies three words from one set of global variables into another, in the opposite direction.
- `copy_three_longs_from_globals`: Copies three 32-bit values from a block of global variables into another block of three 32-bit global variables, in the reverse direction of the adjacent copy routine, then returns.
- `selects_word_from_global_flag`: Preserves d7, calls another routine, clears d7, and if a global byte is nonzero loads a word from a global variable into d0 and then returns with d7 restored, with the result word left in d0.
- `update_entity_status_and_flags`: Searches an entity list for a match based on two bytes, then iterates through a list of structures to check specific bitmasks and coordinate values, potentially setting a bit in a status word at offset 30 of a structure.
- `process_loop_with_calls`: Pushes d0 to the stack, stores a byte at a global address, clears a 16-bit word at a global address, and enters a loop that calls a routine and conditionally updates d0 from a global word based on a byte check.
- `set_global_word_to_four`: Stores the constant 4 into a 16-bit global variable, then pops a longword from the stack into d0 and returns.
- `search_and_update_entity_list`: Calls another routine to retrieve a pointer, then iterates through a list of structures to find an entry matching two specific bytes.
- `validate_and_set_attribute`: Compares two bytes at the address in a0 with d0 and d1, and if they match, loads a byte from offset 3 of the structure into d0, then calls another routine twice with different parameters and calls a third routine to write the value 144 into d0.
- `update_entity_attributes_and_flags`: Iterates through a list of structures, performing attribute comparisons and bitwise updates based on global status flags and table lookups.
- `process_complex_table_logic`: Performs multiple conditional checks against global words and bytes, iterates through a table to perform calculations involving multiplication and division, and calls several routines to update global state and handle bitwise operations.
- `check_attribute_and_call`: Calls two routines, compares a global byte to one, and if it matches, retrieves a byte from a table and calls another routine if the byte is non-zero.
- `computes_byte_flags_from_structure_values`: Iterates six global flag bytes, and for each set entry compares a structure's word sums against a divided longword to set a bit in an accumulator; then scans five more flagged entries for a cleared status mask and stores the resulting byte into a global variable.
- `stores_byte_via_subroutine`: Saves d0, reads the byte at offset -1 from the pointer in a0 into d0, calls a subroutine at a fixed address, restores d0, then decrements d7 and branches back to an earlier point of the routine until it expires.
- `saves_registers_and_dispatches`: Saves registers d1 to d7 and a0 to a1 on the stack, copies the words in d0 and d1 into d4 and d5, and then branches on a flag byte in a global variable, selecting a further path by comparing that byte with 2 and testing whether it is zero.
- `adjusts_d2_bit_and_offset`: Reads the word in a global variable into d2, and when d0 is less than 1 clears bit 1 of d2, otherwise sets bit 0 of d2 path is replaced by clearing bit 0 of d2; it then adds a global word to d4 and branches to a later point in another routine.
- `adjusts_coordinates_by_table_limit`: Compares d0 with a global word and sets bit 0 or clears bit 1 of d2 depending on the sign of d1, subtracts a global word from d4, adds another global word to d5, and branches forward to a fixed offset.
- `looks_up_value_from_coordinates`: Computes a tile value from a byte table indexed by two coordinates, dereferences a longword table if the byte is nonzero, scans a 32-entry global record array for a matching coordinate pair, and returns the chosen longword value.
- `reads_grid_cell_value_by_coordinates`: Calls two routines, reads a byte from a grid computed from the words in d4 and d5 and a global word stride, maps values of 101 and above through a global table, scans a 32-entry global record array for a matching entry, and returns a longword from that scan or the mapped value.
- `selects_table_entry_or_zero`: Loads the address of a fixed table into a0, subtracts 101 from d0, and compares d0 with a global word; if d0 is below that limit it branches to a later point, otherwise it clears d0 to zero and branches to a later point.
- `searches_entries_by_coordinates`: Reads a longword from a table indexed by d0 and a1, then calls another routine twice and clears a result register.
- `advances_pointer_and_tests_flag`: Adds 32 to the address in a0 and increments the word counter in d7, then compares d7 with 32 and branches back when it is below.
- `return_immediately_after_bit_test`: Tests a longword in d1 and bit 5 of its low byte, then restores registers d1 through d7 and a0 through a1 from the stack and returns.
- `copies_fifteen_byte_block_to_global`: Reads a byte at offset 10 from a structure referenced by a pointer, saves it into a global variable, then copies seven words starting at a table entry indexed by that byte times 14 into the start of a global variable array.
- `branch_if_not_zero`: Tests a byte at a global address, branches if it is zero, otherwise loads a specific address into a0 and branches to another location.
- `load_and_jump_pointer`: Loads a long value from a global variable into a0 and jumps to the address stored at that location.
- `invert_and_jump_2`: Calls another routine, inverts a byte in a global variable, calls another routine, and jumps to another location.
- `set_high_bits_in_buffer`: If a global variable is nonzero, sets the top bit of bytes in the buffer at a0: when the variable is less than 2 it sets bytes at offsets 0, 2, 4, 6, 10, 12, 14 and 16, otherwise it sets bytes at offsets 2, 6, 10 and 14.
- `set_high_bit_of_four_offsets`: Sets the top bit of the byte at offsets 2, 6, 10 and 14 from the address held in a0, then returns.
- `dispatch_conditional_function`: Calls a routine at a pointer in a0, loads a 16-bit word from a global variable into d0, and if d0 is not equal to 4, calls another routine and branches to a different location; otherwise it calls a routine at a different pointer.
- `compare_and_call_routine_2`: Compares a byte at a specific address to 2, branches if it is less than 2, otherwise loads a constant address into a0, calls a routine, and performs an unconditional branch.
- `dispatch_handler_from_table`: Shifts d0 left by 2 to index a pointer table in a global variable at offset 18, and if the entry is nonzero calls the routine at that address.
- `sets_flag_and_jumps_through_table`: Writes 255 to a global byte, calls another routine, then jumps to a fixed address in a jump table entry.
- `clear_global_byte_flag`: Clears a byte-sized global variable to zero and returns.
- `releases_queues_and_calls_offset34`: Calls the object queue release routine, then calls the handler stored at offset 34 of the structure pointed to by a global longword, when that pointer and handler are nonzero.
- `invokes_handler_from_indexed_table`: Calls another routine, then reads the word from a global variable, uses it to index a table of longword pointers, and calls the pointed-to routine through a0 when that pointer is nonzero, then returns.
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
- `clamps_word_to_forty_seven`: Tests the signed word in d0 and, if it is non-negative, jumps to a comparison.
- `clamp_scaled_value_halved`: Compares the 16-bit value in d0 with 48 and, if it is below 48, leaves it unchanged; otherwise it sets d0 to 47.
- `compute_scaled_sample_pair`: Scales the 16-bit value in d0 by a global variable and shifts the result right by 4, then multiplies two signed 16-bit entries from a global table by that result and by 4, returning both products in a register pair.
- `computes_grid_index_from_coordinates_2`: Calls a routine to fetch a record, and when the record's flag bits in d2 match a fixed address it stores a grid index into a global word, using either a clamped subtraction path selected by a global byte or a plain division by 48 of offset coordinates.
- `computes_grid_index_from_coordinates`: Subtracts 32 from d0 and 49 from d1, divides each by 48, and combines them as the column plus three times the row into a global variable.
- `dispatches_handler_by_grid_position`: Calls another routine, tests whether the upper bytes of d2 equal a fixed constant, then computes a cell index from d1 and a value returned by another routine, divides each by 48, and calls the handler stored in the table at that index if it is nonzero.
- `execute_multiple_subroutines_7`: Loads a pointer into a0, calls three different routines, then performs a series of conditional calls based on values loaded from two global words.
- `conditionally_dispatches_on_key_value`: Loads two words from fixed absolute locations into d0 and d1, calls a routine, and masks d2 to its upper 24 bits.
- `fetch_and_store_byte_alt`: Calls another routine using a value from a global pointer and a constant in d1, then stores the resulting byte in a global variable.
- `copy_data_block_7`: Saves multiple registers, performs several routine calls to retrieve data, and then enters a loop to copy 32 words from a source structure to a destination table if a byte in a global variable is less than two.
- `copies_sixteen_words_twice`: Copies 16 words from the buffer addressed by a0 into a global variable area, advancing the source and destination pointers each step, then copies 16 further words from the same source into a second area.
- `sets_mode_word_from_flag_bits`: Reads a word through a pointer returned by a routine and calls another routine.
- `sets_flag_word_from_bit_six`: Tests bit 6 of the byte in d0 and, if the bit is clear, branches forward.
- `set_global_word_to_two`: Stores the constant 2 into a fixed 16-bit global variable, then restores the saved register and returns.
- `process_structure_data_7`: Saves registers, checks a global byte, and if it is non-zero, performs a series of table lookups and calls to two different routines before updating a global word and calling a third routine.
- `selects_value_then_restores_stack_state`: Writes 0 or 1 to a global variable, calls several routines to step through the state, stores 31 into a global longword, calls a final routine, and returns the eight bytes of d0 and d1 saved at entry.
- `saves_and_restores_offset_state`: Calls two routines, stores 31 into a global variable, calls another routine, and returns the eight-byte value saved on the stack at entry.
- `updates_random_bit_flags`: Walks a block of longword-spaced global counters, incrementing each and wrapping it at a limit, and for a second set of counters toggles a byte in a table when a counter is zero or reaches a threshold.
- `fills_strided_bytes_from_random_bits`: Decrements d0 and, when it is not zero, branches back to an earlier point; otherwise it complements the byte at offset -1 from a1 and calls a routine.
- `steps_random_word_and_stores_bits`: Repeatedly advances a 16-bit global pseudorandom word for a count held in a global variable, then ORs a bit selected from the current value into the word at offset 2 of the structure in a0.
- `saves_d2_and_searches_record`: Preserves d2 on the stack, loads three global words into d0, d1 and d2, calls a routine that searches the global record table, restores d2 and returns.
- `finds_global_record_by_key_bytes`: Walks a global table of 6-byte records starting at its beginning, comparing a word at offset 4 with d2 and bytes at offsets 1 and 2 with d0 and d1, for up to 32 entries.
- `advances_pointer_in_loop`: Increments d3 by 1 and adds 6 to a0 in a loop that repeats until the counter in d7 expires, then clears d0 and restores d3, d7 and a0 from the stack before returning.
- `execute_conditional_callback_8`: Checks a byte at a global address, and if it is not zero, stores a byte at a global address, checks if a byte at a global address is greater than or equal to 2, and if so, calls a routine.
- `call_and_return`: Calls a routine and then returns.
- `execute_multiple_pointer_fetches`: Retrieves two pointers from a table via a routine, extracts bytes from the resulting structure, and uses those bytes to call a routine with constants 7 and 8 to store values in specific global variables, then calls another routine and performs a final fetch and store.
- `update_global_from_file`: Calls a routine to retrieve a byte, stores it in a global variable, and if a global byte is not one, calls another routine and stores its result in a global variable.
- `dispatches_three_bytes_through_handler`: Saves d0 on the stack, passes three successive bytes from fixed low-memory locations in d0 to a shared handler, and passes the fourth byte only when a global flag byte is zero before branching past the rest.
- `conditionally_calls_three_handlers`: Calls a routine with the byte at a fixed low-memory address in d0, then compares a global variable with 1 and, when it differs, calls the same routine twice more with other bytes loaded into d0.
- `execute_sequence_of_calls_8`: Calls several routines, stores a constant pointer in a global variable, calls two more routines, sets a global word to all ones, and then calls three more routines while clearing a global variable.
- `execute_nested_blit_table`: Checks a global byte for a non-zero value, then iterates through an eight by ten nested table structure, calling another routine for each non-zero byte encountered.
- `process_nested_blit_tables`: Retrieves two pointers via another routine and iterates through a nested loop structure of eight by ten iterations.
- `perform_nested_arithmetic`: Increments d0, decrements d3, increments d5, and executes a loop using d6, then modifies d1, d5, and a value at an offset from a pointer before executing another loop using d7.
- `process_nested_blit_table`: Iterates through a nested table structure using two loops, calling a routine for each non-zero byte encountered.
- `iterates_grid_cells_with_flagged_entries`: Loops over an 8-by-10 grid of words, and for each nonzero word whose referenced entry has bit 6 set in its flag longword it calls a routine.
- `increment_global_longword`: Adds 1 to a 32-bit global counter and returns.
- `compute_offset_from_globals`: Loads several 16-bit global variables, subtracts and adds them to d5 with one multiplication by a global word, adjusting d5 by a constant at two points, then returns.
- `extract_scaled_address_2`: Saves registers, tests a byte at a global address, and if it is not zero, pushes d0 to the stack, retrieves a byte from a global variable, calls a routine, and then loads a word from a global variable into d7 before branching.
- `calculate_scaled_offset_9`: Executes a loop to perform multiplications on values from a table, calculates an offset using values from two global variables and a table, calls a routine, and then calls another routine after pushing a byte from a global variable onto the stack.
- `extract_scaled_address`: Pushes a register to the stack, calls a routine, retrieves a byte from a global variable, performs bitwise operations on a value from another global variable, and if the result is zero, calculates a new value using a multiplication and an addition before branching.
- `calculate_scaled_destination`: Calculates a destination address using multiplication and an offset, loads values from a table, calls a routine, and then calls another routine using a value from the stack.
- `blits_masked_rows_when_mode_one`: Saves d5 and a0, and when d2 equals 1 it saves d2 through d4, copies d0 and d1 into d2 and d3 with 15 added to each, clears d4, calls a blitting routine, restores registers and branches; otherwise it skips straight to the restore step.
- `masks_word_and_blits_rows`: Masks d2 to its low byte and calls a routine, then loads 5 into d5 and calls another routine to blit a run of rows.
- `saves_registers_and_blits_shifted_mask`: Saves d5 and a0 on the stack, calls a routine, then loads 5 into d5 and calls another routine that programs the blitter with a shifted mask.
- `process_blit_structure`: Saves registers, calculates an offset into a structure using a value from d2, clears bit 3 of a byte at offset 34 of that structure, and then performs conditional calls based on values within the structure.
- `call_and_branch_4`: Calls a routine and then performs an unconditional branch to another location.
- `handle_blit_parameters`: Copies d0 and d1 to d6 and d7, retrieves two 16-bit words from offsets 10 and 12 of the structure in a1, and calls a routine.
- `perform_scaled_blit`: Executes a loop to increment a pointer based on a byte value, then checks a byte at a0, and if not zero, performs bitwise tests on values from a table to determine if it calls a routine with specific parameters and then calls another routine.
- `set_bit_and_blit`: If d0 equals 5, it sets bit 3 of a byte at an offset from a1, then performs a scaled blit operation and writes a 16-bit value to a global variable before returning.
- `execute_conditional_blit_5`: Saves registers to the stack, loads a byte from a table using an offset in d2, and if the byte is not negative, calls a routine and then branches to another routine.
- `loop_and_scale_blit`: Runs a loop that calculates an address by multiplying values from a table, then performs a scaled blit operation using values derived from a table and registers.
- `return_without_action`: The routine consists of a single return instruction and performs no reads, writes, or calls; it simply returns to its caller.

The function list continues in: g2159-recovery-functions-2.md, g2159-recovery-functions-3.md, g2159-recovery-functions-4.md, g2159-recovery-functions-5.md.
