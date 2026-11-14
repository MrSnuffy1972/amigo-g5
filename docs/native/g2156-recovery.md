# Game 2156 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02156). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 57 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 41 of 41 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 6041 of 6041 named (0 author symbols), 6041 of 6041 described (149 by rule, 95 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 240 of 240 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 116401 of 116405 instructions identical in start and length, 4 mismatches |

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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Keymap | 1 | 3 | 1 |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_BLIT | 11 | 252 | 6 |
| d | 1 | 3 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_CPU | 10 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon | 8 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon_install | 7 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_BLIT#unpacked | 7 | 52210 | 2715 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_CPU#unpacked | 6 | 47660 | 2691 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_BLIT: crunched executable decrunched by XFDMaster (Imploder (xfd), 161876 -> 359116 bytes), 123 of 167 windows confirmed in reference RAM: recovered as a hunk executable
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_CPU: crunched executable decrunched by XFDMaster (Imploder (xfd), 160772 -> 347516 bytes), 121 of 165 windows confirmed in reference RAM: recovered as a hunk executable
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon: Imploder (xfd) stream found by ancient scan (5784 -> 10488 bytes) but only 0 of 6 sampled windows are in reference RAM: not used
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon_install: Imploder (xfd) stream found by ancient scan (10908 -> 22940 bytes) but only 0 of 11 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 operand
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `call_through_address_register` (More): Calls a routine with a global argument, calls a second routine, then jumps through a function pointer held in address register A3. Kind: indirect call/control flow obfuscation. Pass path: the instruction at the address held in A3.

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Keymap

- `stub_bad_instruction_data`: The bytes do not decode to a valid routine.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_table_block_then_call`: Copies sixteen longwords from a fixed table into the caller's buffer when a buffer pointer is given, then calls another routine.
- `update_table_entry_and_copy_range_2`: Updates a structure or table entry with a new pointer and range-end value, and then copies a sequence of bytes from a source buffer to a destination buffer, terminating the copy with a specific byte value.
- `update_table_entry_and_copy_range`: Reads a byte from a table using an index stored in a structure, updates a pointer and a range-end value in that structure, and then enters a loop to copy a sequence of bytes from a source table to a destination table, calling another routine during the process.
- `update_structure_and_branch_offset_2`: Updates a structure or table by calculating a new range, then iterates through a loop to copy bytes from a source table to a destination table using an offset-based index, and finally updates a pointer or value at a specific offset in the structure.
- `update_structure_and_branch_offset`: Updates a table or structure using an offset-based index, copies certain longwords between structure members, and then enters a loop that checks a value against a counter, potentially calling another routine and updating further structure members or a global variable.
- `add_offset_and_branch`: Loads the constant 1 into d1, adds the longword at offset 84 of the structure in a1 to it, and then branches unconditionally to a later point in the code.
- `test_field_and_branch`: Tests the longword at offset 40 of the structure in a1 and branches to one of two later points depending on whether it is nonzero.
- `mask_structure_bits_and_compare`: Checks if certain bits are set in a structure member at offset 48 of a1, and if not, sets the upper 24 bits of that member to all ones.
- `update_structure_pointer_and_clear_table`: Updates a pointer in a structure, calls another routine, and if the new pointer value is non-zero, proceeds to update a table entry, reset several fields in a structure, and conditionally decrement a value in a table or array based on a pointer check.
- `update_structure_pointers_2`: Updates a pointer in a structure, calls a routine, and conditionally clears or decrements a value in a table or array based on a pointer check.
- `update_structure_pointers`: Updates a pointer in a structure, calls a routine, and conditionally clears or decrements a value in a table or array based on a pointer check.

### Install

- `install_command_entry`: Looks up the largest existing entry number in a linked list, allocates a new entry through the system, writes a header followed by a sequence of marker values, and copies a function table into the new entry.
- `copy_memory_block_and_call_8`: Copies sixteen bytes from a memory location to a destination pointed to by a stack address, then executes a function via a pointer.
- `load_and_check_pointer`: Loads an address into a1, calls a library routine, and if d0 is not zero, loads a pointer from a structure in a0 into a0 and then returns.
- `loads_minus_one_and_branches`: Loads d0 with the value -1 and performs an unconditional branch to a later instruction.
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

- `expand_and_populate_table_2`: Finds the maximum value in a linked list and uses it to expand a table structure by writing a header and a sequence of incrementing values.
- `copy_memory_block_4`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Delete

- `traverse_linked_list_and_update_table`: Traverses a linked list to find the maximum value in a specific field and then updates a table structure.
- `copy_memory_block_and_call_10`: Copies sixteen bytes from a memory location to a destination pointer using a loop, then calls another routine.
- `update_table_index_and_traverse_2`: Calculates a new index or pointer for a table in a1, clears certain fields in a1 and a2, and calls another routine to check or update a secondary table or list, potentially entering a loop that traverses or updates entries based on values read from a table in a0.
- `update_list_index_from_table_2`: Reads a value from a table using an offset, calculates a new index, and stores two bytes or a word into a structure at offset 544 and 548, then branches to another routine.
- `execute_two_subroutines_with_offset`: Increments a value in a structure pointed to by a1, then calls another routine using a pointer from a4, and repeats this process using a different value from a1, before branching back to a previous instruction.
- `process_list_with_conditional_call`: Increments a value in a structure, then checks if a certain value in a list or a global variable matches a certain condition, calling another routine if certain values are encountered or if a list-traversal condition is met.
- `validate_and_increment_list_index_2`: Checks if a value at offset 544 of a structure in a1 matches certain constants, and if so, branches to other routines; otherwise, it increments a value at offset 544 of the structure in a1 and continues execution.
- `update_table_entry_and_branch`: Increments a byte at an offset of 536 in a1, compares a longword from a2 to a longword in a1, and if a certain condition is met, loads a byte from a table using a scaled index and jumps to a routine in a6.
- `tests_field_then_jumps_2`: Tests the long word at offset 640 of the structure in a2 and branches to a second routine if it is nonzero.
- `update_pointer_and_validate_index`: Increments a counter in a structure, then enters a loop that reads an index from a table and checks if that index or a subsequent byte matches certain constant values, updating a global variable if a match is found or calling another routine if certain conditions are met; otherwise, it continues or returns via an indirect jump.
- `branch_forward_to_offset_routine`: Executes a single unconditional branch to a fixed location near the start of the code section, with no other work; the routine is only reachable as a gap in the disassembly.
- `update_pointer_and_branch_conditionally`: Calculates a new index by adding a value from offset 548 of a structure in a1 to a constant, uses this to fetch a byte from a table in a0, and stores it in offset 552 of a1.
- `validate_and_update_index_table`: Checks if a value loaded from a structure offset against a list of prohibited or permitted constants, and if a match is found, updates a value in a structure and calls another routine; otherwise, it enters a loop that iterates through a table, updating structure members and calling another routine if certain byte values are encountered.
- `adds_offset_then_branches_2`: Sets d1 to 1 and adds the long word at offset 544 of the structure in a1 to it.
- `branches_into_copy_routine`: Performs an unconditional branch to a point inside another routine, transferring control without any register or memory changes of its own.
- `update_global_pointer`: If a certain long-word is not zero, it sets a global variable at offset 676 of a structure in a2 to a constant value and then jumps to another routine.
- `search_and_update_list`: Copies a 32-bit value from a structure in a2 to offset 4 of a1, then loops to check if a value at a certain offset in a0 matches a value in a1, updating a pointer in a1 if a match is not found.
- `update_and_store_pointer`: Increments a 32-bit value at an offset of a structure in a2, calculates a new address using a value from a structure in a2 and a pointer from a1, stores a value at that address, and jumps to an indirect address in a6.
- `set_index_to_max`: Compares a value from a structure in a2 with another value from a structure in a2, and if the first is not greater, sets a value at offset 624 of a2 to negative 255 and branches to another routine, otherwise it calls another routine.
- `update_index_and_load_byte`: Increments a 32-bit value at an offset in a structure, then loads a byte from a table into another offset in that structure using a calculated index.
- `execute_conditional_calls`: Compares a value in a structure to a constant and calls a routine twice if they match, otherwise calls the routine once and jumps to an indirect address.
- `copy_and_call_routine`: Copies a long-word from a structure in a2 to a pointer in a1, copies another long-word from a structure in a2 to an offset of a1, modifies a pointer in a4, and calls another routine.
- `stores_minus_one_and_jumps_2`: Loads the value -1 into d1 and stores it as a long word at offset 668 of the structure in a2.
- `execute_two_routines`: Calls a routine twice with different pointer offsets in a4 and jumps to an indirect address.
- `update_pointer_and_set_value`: Calls a routine, updates a pointer, and if a certain value in a structure does not match a constant, sets a 32-bit value in a structure to a negative constant before calling another routine.
- `branch_on_index_value`: Checks a value from a structure in a1 against several constants, calling other routines or setting a value in a structure in a2 if certain conditions are met, and finally jumps to an indirect address.
- `update_structure_member_on_condition`: Clears a 4-byte value at an offset of 4 from a pointer in a1 and calls another routine, then checks if a value at offset 656 of a structure in a2 matches certain constants; if certain conditions are met, it calls another routine, updates a pointer in a1, and performs a memory write or further routine calls before jumping to a routine in a6.
- `modify_and_store_byte`: Calculates an offset using a value from a structure in a2 and a pointer in a1, stores a byte from a structure in a2 into a memory location, copies a long-word from a structure in a2 to a pointer in a1, modifies a pointer in a4, and calls another routine.
- `load_and_call_routine`: Loads a value from a structure in a2 into d2, loads a value from a structure in a1 into d1, sets a4 to an offset of a4, and calls another routine before branching to another routine.
- `no_op_then_branch`: Executes a no-operation and then branches unconditionally to a point inside another routine.
- `traverse_and_replace_list`: A loop that replaces a value in a list with a byte from a table and updates a pointer, continuing until a null pointer is encountered.
- `branch_to_target_address_3`: Stores a long-word from d1 into an offset of a pointer in a1, then if d1 is not zero, jumps to another routine; otherwise, it overwrites d1 with d2 and jumps to an address in a6.
- `branches_into_search_routine`: Performs a single unconditional branch to a point inside another routine, with no register or memory changes of its own.
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

- `run_avail_command`: Program entry point for the Avail command.
- `cleanup_program_and_reply`: Stores a result code in the control block, then closes the open file handle when present, frees the memory block, closes the library and replies to the owning message.
- `sum_memory_region_sizes`: Adds up the sizes of every memory region in a linked list whose attribute flags match the given mask, and returns the total.
- `print_memory_availability_table`: Prints the memory availability table for the Avail command, showing available, in-use, maximum and largest-block figures for each memory type.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_call_with_stack_slot`: Passes two arguments to another routine together with the address of a stack slot, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_hook_then_measure_string`: Calls a function pointer loaded from a global table, then counts the characters of a stack buffer up to its terminating zero byte.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `add_disk_buffers_command`: Command entry point for adding disk buffers to a drive.

### Path

- `traverse_list_and_populate_table_4`: Traverses a linked list to find the maximum value in a specific field, then uses that value to initialize a table structure.
- `copy_memory_block_and_call_11`: Copies sixteen bytes from a memory location to a destination address via a loop, then executes another routine.

### Resident

- `traverse_list_and_expand_table`: Traverses a linked list to find the maximum value in a specific field and then expands a table by writing a header and copying data blocks.
- `copy_memory_block_and_call_12`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then executes another routine.
- `validate_and_update_structure_pointer`: Calculates a new pointer or index, updates a field in a structure, and calls several subroutines to check or update further structure members; if certain conditions are met, it checks a bitmask in a table and updates a field before performing an indirect jump or calling another routine.
- `clear_d1_and_jump_indirect`: Clears the d1 register to zero and then jumps indirectly to the address held in a6, without returning to its caller.

### Assign

- `traverse_list_and_expand_table_2`: Traverses a linked list to find the maximum value in a specific field and then expands a table by writing a header and copying data blocks.
- `copy_memory_block_and_call_13`: Copies sixteen bytes from a memory location offset from a pointer into a destination buffer and then calls another routine.
- `update_linked_list_node_and_table_2_2`: Calculates an index-based pointer, updates a structure or table with new bounds or pointers, and traverses a linked list or table-based chain, updating node pointers and checking for end-of-list or invalid index conditions, calling another routine to perform certain updates or checks during the traversal.
- `update_linked_list_node_and_table_2`: Updates a linked list node by copying a pointer from one index to another, then enters a loop that traverses and updates subsequent nodes in.
- `update_linked_list_node`: Updates a pointer in a structure or list, checks a condition or flag, and if certain criteria are met, updates a value in a structure or list to a constant or a value from another structure, then calls another routine.
- `update_linked_list_node_and_table`: Updates a linked list node by copying a pointer from one index to another, then enters a loop that traverses and updates subsequent nodes in the list.
- `assign_table_values`: Updates several values in a table using offsets derived from a structure, calls another routine multiple times to check or clear certain flags, and finally jumps to an address in a6.

### EndCLI

- `expand_and_populate_table`: Traverses a linked list to find a maximum value, then expands a table by writing a header and a sequence of incrementing values.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location offset from a register to a destination pointer, then calls another routine.
- `modify_list_and_table`: Modifies a bitmask in a structure, updates a pointer in a structure, and conditionally decrements a value in a table if it is greater than zero.

### Rename

- `find_max_and_populate_table`: Traverses a linked list to find the maximum value in a specific field and then populates a table with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_2`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer.
- `update_pointer_and_call`: Updates a pointer in a structure, calls a routine, and conditionally updates a value in a structure or table.

### Dir

- `traverse_list_and_update_table`: Traverses a linked list to find the maximum value in a specific field and then updates a table structure.
- `copy_memory_block_and_call_3`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then calls another routine.
- `update_entity_bounds_and_state`: Updates several pointer-based offsets in a structure in a1 to define a memory range, clears certain fields, and conditionally updates a structure in a2 with values from a1 or a table, then calls another routine or returns.
- `update_structure_field_from_registers`: If a longword at offset 332 of the structure in a1 is non-zero, it loads three values from offsets 88, 48, and 48 (the same offset) into d3, d2, and d1, then calls another routine using a pointer-adjusted a4, and finally writes the value from d1 into offset 320 of the structure in a1 before branching away.
- `clear_field_and_branch`: Tests the longword at offset 320 of the structure in a1 and branches away when it is zero.
- `update_entity_property_and_loop`: Updates a field in a structure using a value from another structure, then calls a routine multiple times with different offsets to update or check further fields, and finally loops through a range of indices to clear or set values in a table, or otherwise returns.
- `update_entity_state_and_pointer_2`: Checks a value in a table, and if it equals two, calls a routine to update a field in a structure, then updates a byte in a table using a pointer.
- `update_entity_state_and_pointer`: Checks a value in a table, and if it equals two, calls a routine to update a field in a structure, then updates a byte in a table using a pointer, and subsequently calls several other routines to update or reset various fields in the structure, including updating a pointer-based index and a byte-based index, before finally branching to another routine.
- `update_structure_range_and_set_value`: Checks if certain pointers in a structure in a2 and a structure in a1 are non-zero, then if a certain value is not one, calls a routine, and if a certain flag in a2 is zero, initializes a range of values in a structure in a1 by looping and setting each entry to one, before calling another routine and updating a value in a2 from a structure in a1.
- `set_structure_value_with_offset`: Calculates a pointer or offset by adding a value from a structure in a1 to d1, shifts it right by two, and calls a routine using a pointer from a2.
- `set_structure_value`: Calls a routine, and if a certain value is zero, sets a value at an offset in a structure.

### NewCLI

- `traverse_list_and_populate_buffer`: Traverses a linked list to find the maximum value in a specific field and then populates a destination buffer with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_4`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by the stack.
- `recalculate_bounds_and_copy_table_entries`: Updates a table of bounds or limits by recalculating or copying pointer-indexed values, and then performs a loop to copy a range of entries from one table or array to another, potentially incrementing a counter if a certain value is non-zero.
- `update_table_and_recalculate_bounds_2`: Updates a table entry by copying a value from one pointer-indexed location to another and increments a counter if it is non-zero, then calls another routine to perform further updates or resets on a structure, potentially involving a loop that searches for a zero-value entry in a table or updates a pointer-based bound or counter.
- `update_table_and_recalculate_bounds`: Iterates through a table, updating or resetting certain values, and if a certain condition is met, it updates a pointer or value in a structure and calls another routine multiple times.
- `add_offset_and_jump_back`: Loads the constant 1 into d1, adds the longword at offset 300 of the structure in a1 to it, and then branches unconditionally to an earlier point in the code.
- `update_table_and_decrement_logic_branch`: If a certain value in a structure is zero, it calls a routine three times with a constant value in d0 and a pointer from a table in a2; otherwise, it performs a series of pointer-based assignments and calls a routine multiple times, including a calculation that shifts a pointer-based address right by two bits before calling the routine again.
- `update_table_and_decrement_logic`: Checks if a value at an offset in a1 is below a certain threshold, and if so, performs several pointer-based assignments and calls a routine using a pointer from a2.
- `update_table_and_decrement`: Updates a value in a table, calls several routines, and conditionally clears or decrements a value in a table if a certain condition is met.

### NewShell

- `traverse_list_and_populate_buffer_2`: Traverses a linked list to find the maximum value in a specific field, then uses that value to calculate a buffer size and populate a table with a sequence of incrementing values.
- `copy_memory_block_and_call_5`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then executes another routine via a function pointer.
- `update_structure_index_and_table`: Updates a structure's index and table by copying values from a source table to a destination table, and increments or decrements a counter in a global structure if a certain condition is met.
- `sync_structure_index_and_update_table`: Checks if a table entry or structure index is non-zero, copies a value from one table entry to another, and increments or decrements a counter in a structure.
- `update_structure_index_and_sync_values`: Checks if a table entry is zero and updates it with a calculated value, then enters a loop that updates a structure's index or clamps a value to 800, while calling another routine and performing several conditional updates to a structure and a table.
- `add_offset_and_branch_forward`: Loads the constant 1 into d1, adds the longword at offset 304 of the structure in a1 to it, and then branches unconditionally to a later point in the code.
- `sync_and_update_structure_value`: Checks if a value at offset 692 of a structure in a1 is zero; if not, it clamps a value at offset 276 of a1 to 800, synchronizes a secondary field at offset 268 with a value from offset 276, and calls a routine using a pointer from a2, then performs further updates to a table and structure using a pointer from a2 and a pointer from a4.
- `clamp_and_sync_structure_value`: Ensures a value in a structure does not exceed 800, synchronizes a secondary field with that value, and then executes a series of calls to update various structure members and table-based pointers or counters based on whether the primary value is zero or non-zero.
- `update_structure_and_table`: Updates a value in a structure, calls several routines, and conditionally clears or decrements a value in a table if a certain condition is met.

### Echo

- `traverse_list_and_populate_buffer_3`: Traverses a linked list to find the maximum value in a specific field and then populates a destination buffer with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_6`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then calls another routine.
- `update_table_range_and_iterate_2`: Updates a range or pointer values in a structure by calling a routine to retrieve new indices, then enters a loop that iterates through a range of indices, updating a structure's range or pointer values.
- `update_table_range_and_iterate`: Updates a structure's range or pointer values by calling a routine to retrieve new indices, then enters a loop that iterates through a range of table entries, calling a routine at each step and updating a pointer or index in the structure.
- `copy_field_and_branch`: Copies the longword at offset 336 of the structure in a1 into the longword at offset 328 of the same structure, then branches unconditionally to a later point in the code.
- `adjust_list_bounds_and_iterate`: Sets a longword at offset 328 in a1 to 1, checks certain values in a structure, and if certain conditions are met, calculates a new index and branches back to an earlier instruction or to another routine.
- `copy_field_and_jump_forward`: Copies the longword at offset 336 of the structure in a1 into the longword at offset 332 of the same structure, then branches unconditionally to a later point in the code.
- `update_list_bounds_and_iterate`: Updates two pointer-sized values in a structure, checks if a certain range is valid, and if so, iterates through a table to read a byte from an offset calculated from a value in the structure, then calls another routine and updates a value in the structure before branching.
- `check_structure_and_call`: Checks a value at an offset in a1, and if it is not zero, calls a routine using a pointer from a2.
- `conditional_routine_calls`: Checks a value in a structure and calls a routine or a sequence of routines if certain conditions are met.

### cls

- `open_dos_library`: Opens the DOS library through the exec library base, runs two library calls on it, and returns zero when the second call's result is not negative.

### Execute

- `traverse_list_and_update_table_2`: Traverses a linked list to find the maximum value in a specific field, then updates a table entry with this value and populates it with a sequence of incrementing longwords.
- `copy_memory_block_and_call_7`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by the stack, then executes another routine.

### Copy

- `find_max_and_populate_table_2`: Traverses a linked list to find the maximum value in a specific field, then writes this value and a sequence of incrementing integers into a table.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer.
- `update_table_index_and_traverse`: Calculates a new index or size for a table entry in a structure, clears certain fields in a secondary structure, and then enters a loop that traverses a table of pointers, checking for specific byte-encoded command or jump values and calling another routine if certain conditions or specific values are met.
- `update_list_index_from_table`: Calculates a new index using a value from a table, retrieves two bytes from a data structure using this index, and stores these bytes into a structure at offset 544 and 548, then branches to another routine.
- `execute_two_subroutines_with_offset_2`: Increments a value in a structure pointed to by a1, then calls another routine using a pointer from a4, and repeats this process using a different value from a1, before branching back to a previous instruction.
- `process_list_with_conditional_call_2`: Increments a value in a structure, then checks if a certain value in a list or a global variable matches a certain condition, calling another routine if certain values are encountered or if a list-traversal condition is met.
- `advance_list_loop_with_dispatch`: Loops over a step list in a structure, incrementing a counter at offset 344 and, on each pass, reads a 16-bit value from a table and a byte from a list, then dispatches to a jump table or calls a routine on a matching condition, returning through the jump target when the list is exhausted or a limit is reached.
- `copy_range_to_structure`: Increments a value at an offset of 536 from a1, compares a value from a2 to that new value, and if it is not greater or equal, checks a byte from a table using a shifted offset from a1 and jumps to a routine in a6 if a certain condition is met.
- `tests_field_then_jumps`: Tests the long at offset 684 of the structure in a2 and branches to a later instruction if it is nonzero.
- `update_index_table_and_validate_constants`: Increments a counter in a structure, then iterates through a list of offsets to check if the bytes loaded from a table match certain permitted or prohibited constants, updating a global value if a match is found or calling another routine if certain conditions are met; otherwise, it returns or jumps to another location.
- `branch_to_shared_tail`: Contains a single unconditional branch that jumps forward to a later shared code location, so it performs no data movement itself.
- `update_pointer_value_and_verify`: Updates a value in a structure at offset 552 using a byte read from a table, then compares this new value against a global variable at offset 700; if they match, it performs a secondary table lookup and calls another routine.
- `dispatch_script_opcodes_per_tick`: Repeatedly reads opcodes through a pointer in a0 indexed by values in the structure at a1, and for opcodes 35, 37, 40, 37 and 124 it invokes a handler via an indirect call, with a comparison against a global variable deciding whether a further call is made.
- `adds_offset_then_branches`: Sets d1 to 1 and adds the long at offset 544 of the structure in a1 to it.
- `branches_to_copy_routine`: Performs an unconditional branch to a later instruction, passing control onward without changing any data.
- `set_constant_d2`: Sets a 32-bit value in a structure at offset 424 of a2 to a constant and then jumps to another routine.
- `compare_and_update_pointer`: Compares a value from a structure in a1 with d1, and if certain conditions are met, calculates a new address using a value from a structure in a2 and calls a routine.
- `increment_and_copy_value`: Increments a value in a structure and copies a word from a source pointer to a destination pointer in a table.
- `compare_and_set_constant`: Compares a value from a structure in a2 with another value from a structure in a2, and if certain conditions are met, sets a value in a structure in a2 to a constant and branches.
- `increment_and_store_byte`: Increments a value in a structure and stores a byte from a source pointer into a destination pointer in a structure.
- `check_table_and_call`: Compares a value in a structure to a constant and calls a routine if they are equal.
- `copy_structure_data`: Copies a 32-bit value from a structure in a2 to a pointer in a1, copies another 32-bit value from a2 to an offset in a1, and calls another routine before jumping to another routine.
- `stores_minus_one_and_jumps`: Loads d1 with the value -1 and stores it as a long at offset 712 of the structure in a2.
- `execute_two_routines_2`: Calls a routine twice with different pointer offsets in a4 and jumps to an indirect address.
- `update_conditional_value`: Calls a routine, updates a pointer-based value, and if a certain value in a structure in a2 does not equal 41, sets a value at offset 704 of that structure to -1 before calling another routine and jumping to an indirect address.
- `dispatch_structure_state_handler`: Reads the long word at offset 4 of the structure in a1 and, if it is not -1, compares it with 35, 40 and 41/124.
- `execute_subroutine_loop`: Clears a 4-byte value at an offset of 4 in a pointer-based structure, calls another routine, and then enters a loop that updates a 4-byte value at an offset of 8 in that structure, continuing or branching based on a comparison with a value in a structure in a2, before calling another routine and jumping to a location in a6.
- `update_pointer_and_store`: Calculates an offset using a value from a structure in a2 and a value from a structure in a1, stores a byte from a structure in a2 at an offset in a0, copies a 32-bit value from a2 to a1, and calls another routine before jumping to another routine.
- `load_parameters_and_call_subroutine`: Loads a longword from a global variable into d2 and a longword from a structure offset 8 into d1, sets d0 to a constant, updates a pointer in a4, and calls another routine before branching to a different code location.
- `branches_past_no_operation`: Executes a no-operation, then branches unconditionally to a later instruction.
- `copy_linked_list_bytes`: Loops through a list of bytes, copying a byte from a source table to a destination, and updates a pointer in the list, continuing until a null pointer is encountered, then jumps to an indirect address.
- `branch_to_target_address_3_2`: Stores a long-word from d1 into an offset of a pointer in a1, then if d1 is not zero, jumps to another routine; otherwise, it overwrites d1 with d2 and jumps to an address in a6.
- `branches_to_search_routine`: Performs an unconditional branch to a later instruction, passing control onward without changing any data.
- `copy_and_replace_bytes`: Loops through a list of bytes, copying a byte from a source table to a destination, and updates a pointer in the list, continuing until a zero byte is found, then copies a byte from a different offset and jumps to an indirect address.
- `clear_and_populate_list`: Clears several 32-bit values in a structure, loads a byte from a table using a pointer and an offset, and then loops to clear a range of bytes in a table.
- `execute_three_subroutines`: Calls a routine three times with different offsets of a4, then negates a value from a structure in a2 and jumps to an indirect address.
- `copies_bytes_and_stores_sum`: Copies a byte from a source table in a0 into a destination table, using the index values from two 32-bit fields in the structure in a1, and repeats this for a counted range.
- `store_sum_to_table_2`: Calculates the sum of two 32-bit values from a structure and stores the lower byte of that sum into a table at an offset determined by a third value in the structure.
- `copies_bytes_with_callback`: Copies a byte from one table offset to another in the buffer addressed by a0, then repeats a loop that calls the routine addressed by a5 and stores the byte in d1 into the buffer at an index taken from the structure in a1.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `swap_and_store_pointer_2`: Calls a routine, sets a value in a structure, and if a certain value is zero, calls another routine twice and sets a pointer to a new address.
- `swap_and_store_pointer`: Copies a 32-bit value from a source table to a location in a structure in a2, then copies a value from a list into that source table, and finally updates a pointer in that structure, then jumps to an indirect address.

### Makedir

- `traverse_list_and_populate_table`: Traverses a linked list to find the maximum value in a specific field, then uses that value to calculate a size and populate a table with a header and subsequent data.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset by a value in a register to a destination pointed to by a stack pointer.
- `execute_conditional_logic_2`: Performs several routine calls and updates various offsets in a structure based on values retrieved from a source table.
- `execute_conditional_logic`: Performs several routine calls and updates various offsets in a structure in a1 based on values retrieved from a source table, including conditional logic that sets a value to 10 if a certain check fails, before jumping to an indirect address.

### List

- `traverse_list_and_populate_table_2`: Iterates through a linked list to find the maximum value in a specific field, then uses that value to initialize and populate a structured table.
- `copy_memory_block_3`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.
- `update_table_and_traverse_links`: Updates a structure's size or index field, then enters a loop that traverses a linked list or table of pointers, checking for specific byte-encoded command or jump values and calling another routine if certain conditions or specific command bytes are encountered.
- `update_list_index_and_cache_values`: Calculates a new index using a base offset and a value from a table, then reads two bytes from a structure at that calculated offset and stores them into a structure at offsets 544 and 548, before branching to another routine.
- `advance_list_entry_and_dispatch`: Adds a longword from offset 540 of the structure in a1 into d1, passes the constant 564 in d0 with a4 advanced by 536 to the routine in a5 once, then loads the longword at offset 548 of a1 into d1, repeats the same call, and branches back to the earlier dispatch loop, which increments a counter at offset 536 of a1 and processes table entries.
- `advance_list_entry_and_dispatch_2`: Increments a longword counter at offset 540 of the structure in a1, then compares it against a global value; when the counter is below that value it calls a routine through a jump table after resetting a byte count, and otherwise it stores 1 into the counter field and reuses a table lookup.
- `advance_list_cursor_and_dispatch`: Loops over a list of byte-coded entries: it increments a counter in the structure at a1, compares it against a global variable, and while it exceeds the threshold it advances a secondary index and reloads the list entry, branching to a jump table when the index runs past the stored limit.
- `copy_range_to_structure_alt`: Increments a value at an offset of 536 from a1, compares a value from a2 to that new value, and if it is not greater or equal, checks a byte from a table using a shifted offset from a1 and jumps to a routine in a6 if a certain condition is met.
- `test_long_then_branch`: Tests the long word at offset 612 of the structure in a2 for zero and branches to another routine when it is nonzero.
- `validate_and_update_index_table_2`: Iterates through a list of indices, checking if certain bytes loaded from a table match a set of permitted or prohibited constants, and if a match is found, updates a value in a structure and calls another routine; otherwise, it continues or returns via an indirect jump.
- `adds_offset_word_branches`: Sets d1 to 1 and adds the long word at offset 544 of the structure in a1 to it, then branches unconditionally to another routine that follows.
- `update_pointer_and_branch_on_value`: Calculates a new index by adding a value from offset 548 of a structure in a1 to a base offset, retrieves a byte from a table in a0 using this index, and stores it in offset 552 of the structure in a1.
- `scan_table_and_update_state`: Loops over a table of bytes indexed from a structure field at offset 548 and per-entry values, repeatedly reading and writing structure fields at offsets 136 through 140 and a global variable, and dispatching through an indirect call when a fetched code matches a conditional value.
- `add_offset_then_branch`: Sets d1 to 1 and adds the long word at offset 544 of the structure in a1 to it, then branches to another routine.
- `branch_to_shared_handler`: Branches unconditionally to a routine located in the middle of another routine.
- `set_global_pointer`: Sets a global variable at offset 616 of a structure in a2 to a constant value, otherwise it returns.
- `update_and_store_pointer_2`: Sets a value in a structure, and then loops to compare a value from a table against a value in a structure, potentially updating a pointer and a table value.
- `increment_and_store_pointer`: Increments a value at offset 676 of a structure in a2, calculates an offset using a value from a structure in a2 and a pointer from a1, and stores a value from a1 at that calculated address before jumping to a target.
- `update_index_and_load_byte_2`: Compares a value from a structure and, if it is within a certain range, sets a value in a structure to a constant.
- `update_value_from_table`: Increments a value at offset 696 of a structure in a2, calculates an offset using a value from a structure in a2 and a pointer from a0, and stores a byte from that table into a value at offset 692 of a structure in a2 before jumping to a target.
- `conditional_subroutine_execution`: Compares a value at offset 692 of a structure in a2 with a constant, and if they are equal, calls a subroutine twice using a pointer or a value derived from a pointer before jumping to a target.
- `copy_structure_members`: Copies a 32-bit value from a structure in a2 to a pointer in a1, copies another 32-bit value from a structure in a2 to an offset of 4 in a1, then calls another routine and branches to a new location.
- `store_minus_one_and_jump`: Stores the value minus one as a long word at offset 640 of the structure in a2, loads the long word at a1 into d1, then jumps through a6.
- `call_two_pointers_then_jump`: Calls the routine pointed to by a5 twice, once with 32 in d0 and once with 20 in d0 after offsetting a4 by 260.
- `update_structure_with_subroutine_result`: Calls a subroutine, copies the result back to a pointer in a1, and if a value at offset 628 of a structure in a2 is not 41, sets a value at offset 640 of a structure in a2 to -1, then calls another subroutine before jumping to a target.
- `dispatch_on_structure_type`: Reads a long word at offset 4 of the structure in a1 and compares it to -1, 35, 40, 41 and 124, calling a routine through a5 and a jump table in the matching cases.
- `execute_conditional_logic_3`: Clears a 32-bit value at an offset of 4 in a pointer, calls another routine, and then enters a loop that calls another routine while checking if a value in a structure in a2 matches certain constants, or otherwise calls another routine and jumps to a location in a6.
- `update_table_and_call`: Calculates an offset using a value from a structure in a2 and a pointer in a1, copies a byte from a table in a0 to a pointer in a1, copies a 32-bit value from a structure in a2 to a pointer in a1, then calls another routine and branches to a new location.
- `load_and_call_subroutine`: Loads a long-word from an offset of 632 in a2 into d2, loads a long-word from an offset of 8 in a1 into d1, loads the constant 24 into d0, loads a pointer or address or offset 128 in a4 into a4, calls a routine using the address in a5, and branches to another routine.
- `pad_and_jump_forward`: Executes a no-operation, then unconditionally branches forward by a word-sized displacement into another routine's code.
- `process_linked_list_and_swap`: Loops through a list where each element contains a pointer to the next, reading a byte from a table and writing it to a certain offset, and swaps a byte from a pointer in a1 into a table before checking if the pointer is null.
- `store_d1_then_branch`: Writes the long value in d1 to offset 8 of the structure at a1, then tests d1: if it is nonzero it branches to a point inside another routine, otherwise it copies d2 into d1 and jumps through a6.
- `branch_into_list_handler`: Branches unconditionally to a point in another routine that handles list entries.
- `traverse_and_replace_list_element`: Loops through a list, replacing a value in the list with a byte from a table, and once a certain byte is found, writes a byte from a pointer in a1 into a table before jumping to a target.
- `initialize_and_clear_table`: Clears several 32-bit or 16-bit values in a structure pointed to by a2, copies a byte from a table (indexed by d1 multiplied by 4) into a structure field, and then loops to clear a range of bytes in a table (indexed by d1 and a pointer from a2) until a certain value is reached.
- `execute_subroutines_and_invert_value`: Calls a subroutine three times with different parameters or pointers, and then calculates the bitwise NOT of a value at offset 640 of a structure in a2 before jumping to a target.
- `tests_pointer_then_jumps`: Checks the long word at the address in a1 and, if it is zero, calls a jump-table routine and returns.
- `branch_to_dispatch_target`: Branches unconditionally to a dispatch routine at a fixed offset.
- `set_result_minus_one_and_jump`: Sets d1 to minus one and jumps through a6.
- `set_result_zero_and_jump`: Clears d1 to zero and jumps through a6.
- `check_value_against_list`: Loads a value from a pointer in a1 and compares it against several constant values, otherwise it jumps to another routine.
- `test_and_jump_indirect`: Tests the long word at the address in a1 and, if it is nonzero, branches to a later point in the code; otherwise it clears d1 and jumps through the address in a6.
- `process_list_and_call`: Copies a byte from a table in a0 to a pointer in a1, updates a value in a structure in a1, and if a certain condition is met, calculates a new offset, copies a byte from a table in a0 to a pointer in a1, and calls another routine.
- `add_offset_and_branch_2`: Sets d1 to 1, adds the long word at offset 8 of the structure in a1 to it, and branches forward to a later point in the code.
- `clear_d1_and_jump`: Clears d1 to zero and then jumps through the address held in a6.
- `reset_string_bounds_and_update_display`: Resets several string index bounds and pointer-based array limits in a structure pointed to by a1, calls a routine to initialize string bounds, and then updates certain display-related pointer or size fields in the structure before branching to another routine.
- `initialize_string_index_bounds`: Sets certain array elements in a structure pointed to by a1 to a default value and then uses a routine to check or copy data from a source string or block, potentially updating a global or structure-based index or terminator.
- `parse_string_and_update_index_2_2`: Reads a pointer from a structure, checks if the target string or data block is null, and if not, copies bytes from that source or a secondary source into a destination buffer while updating an index and a length or end-marker flag in the structure.
- `branch_to_string_parser`: Branches unconditionally to a later point in the code, passing control onward without changing any registers.
- `parse_string_and_update_index_2`: Parses a string or a sequence of bytes from a structure, updating several offset-based pointers and flags in a structure, and checks for a termination character or a specific length limit, otherwise it loops through the string or a pointer-based sequence.
- `parse_string_and_update_index`: Checks if a certain flag in a structure is zero and if a pointer in a table is non-null, then calls another routine to process a string.
- `update_table_and_rebuild_index_2`: Updates a byte in a table using an index-based offset, updates a pointer or counter in a structure, and then enters a loop that copies bytes.
- `update_table_and_rebuild_index`: Updates a byte in a table using an index-based offset, updates a pointer or counter in a structure, and then enters a loop that copies bytes from a source table to a destination table while updating a secondary index or pointer.
- `modify_table_byte`: Decrements a longword loaded from an offset of 1988 in a1, shifts a longword loaded from an offset of 1448 in a1 left by two, and stores the resulting byte into a table at a calculated offset in a0, then branches to another routine.
- `update_pointer_logic_and_display_index`: Clears a byte in a table using an index, checks a flag in a structure, and if certain conditions are met, calculates and stores a new pointer or a bitwise-inverted address into a structure, then calls a routine to update a pointer or display logic.
- `compute_shifted_address_offset`: Loads the address at offset 4648 from a4 into a3, copies it to d1, shifts d1 right by 2 bits, and branches forward to a later point in the code.
- `update_pointer_logic_and_display`: Updates a pointer in a structure using an index, potentially updates a table entry with a calculated address, and executes a sequence of calls to a routine using a constant or a pointer-derived value, followed by conditional updates to a structure's field and a final check of a pointer-based value to update a certain field or set a constant value.
- `update_pointer_logic`: Calls another routine using a pointer from a structure, then checks a byte in a table; if the byte is zero, it calls another routine and updates a value in a structure using a pointer-based offset, otherwise it calls another routine and updates a pointer-based value in a structure.
- `update_display_bounds_and_offset_4`: Calculates a bit-shifted value from a pointer-based address, stores it in a structure, and calls another routine to check if the resulting value is zero; if zero, it resets a different structure field to zero using another routine call, otherwise it updates a pointer-based offset or boundary.
- `reset_display_bounds_and_offset_2`: Updates a display boundary or offset in a structure in a1 by calling another routine to retrieve a value, then checks if that value is zero to either reset a display parameter or calculate a new boundary or offset based on a pointer in a2 and calls several other routines to update display parameters or reset certain values.
- `reset_display_bounds_and_offset`: Checks if a display offset or boundary (stored at offset 584 of a structure in a1) is negative, and if so, updates a value in a structure (a), then resets certain display parameters or calculates a new display boundary or offset (via a call to another routine) and updates a structure (a).
- `update_display_bounds_and_offset_logic`: Checks if a display offset or boundary (stored at offset 584 of a structure in a1) is negative, and if so, updates a value in a structure (at offset 648 of a1) with the bitwise NOT of that offset.
- `update_scaled_display_offset`: Calculates a new scaled display offset by loading a value from a pointer in a1, shifting it left by two, and storing it into a structure offset in a0, then updates a pointer in a1 with a value from a2 and calls another routine.
- `store_shifted_address_and_branch`: Loads the address at offset 4788 from a4 into a3, copies it to d1, shifts d1 right by 2 bits, stores the result as a long word at offset 604 of the structure in a2, and branches forward to a later point in the code.
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
- `call_indirect_then_jump`: Loads 45 into d1 and 24 into d0, loads the long word at offset 224 of the structure in a2 into a4, calls the routine at the address in a5, and then jumps through the address in a6.
- `branch_to_display_update`: Contains a single unconditional branch that jumps forward to the display index and parameter update code.
- `test_and_mask_bit_one`: Tests if the least significant bit of a register is set and, if so, clears all but the first bit of that register before branching or moving a value into it.
- `call_then_jump_through_pointers`: Loads a4 from the longword at offset 224 of a2 and sets d1 to 45 and d0 to 24, then calls the routine addressed by a5 and jumps to the address held in a6 without returning here.
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

- `traverse_list_and_update_table_3`: Iterates through a linked list to find the maximum value in a specific field, then updates a table entry with this value and populates it with a sequence of incrementing longwords.
- `copy_memory_block_and_call_9`: Copies sixteen bytes from a memory location offset from a register to a destination address on the stack, then executes another routine.
- `update_structure_and_call_subroutines`: Updates a bit in a byte using a bit-shift, modifies several pointer-based offsets in a structure, and executes a sequence of conditional calls to other routines based on the contents of a pointer-indexed table or a structure-relative offset.
- `check_structure_and_call_2`: Calculates a bit-shifted address from a4, calls another routine, and if a certain value at an offset in a1, scaled by a pointer, is non-zero, calls another routine and updates a global variable with a value from a pointer in a2.
- `set_buffer_and_draw`: Loads a pointer from a2 plus 1544, calls a routine with a length of 720, and if the loaded value is zero, calls another routine and sets a value at a1 plus 704.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `traverse_list_and_populate_table_3`: Walks a linked list to find the maximum value in a specific field and then populates a table with a header and a sequence of incrementing values.
- `copy_multiple_words`: Copies a fixed number of words from a source pointer to a destination pointer located on the stack.
- `update_object_state_and_indices`: Updates a pointer in a structure, clears several fields at certain offsets, and then checks a series of table-indexed values to conditionally update or reset other fields in the structure, including checking for a specific character '$' in a table.
- `check_table_index_and_set_value_2`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a structure in a0; if that byte is non-zero, it compares a value of 10 in d2 against a longword at offset 548 in a2 and, if less, sets d1 to 255 before branching to another routine.
- `check_table_index_and_set_value`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a structure in a0; if not zero, it compares a value in a2 to 20 and, if less than, sets d1 to a certain value before branching.
- `traverse_and_update_structure_pointers`: Traverses a linked-list-like structure or table-based hierarchy by updating pointers in a structure in a1 based on values found in a table in a0, and calls another routine if certain character or pointer conditions are met.
- `validate_and_update_structure_state`: Checks if certain characters in a table-driven structure match a specific character, and if so, calls another routine to update or reset structure members.
- `branch_to_later_code`: Executes a single unconditional word-sized branch that jumps forward to a later point in the code, so control never falls through into the following bytes.
- `test_structure_member_and_branch`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a0; if not zero, it compares a longword at offset 388 in a1 to 1 and, if not equal, sets d1 to a certain value and branches.
- `branch_on_structure_field`: The routine is a single unconditional branch instruction (bra.w) that jumps to a later point in the code, with no other work performed in this routine.
- `test_structure_member_bits_branch_logic`: Checks if a longword at offset 356 of the structure in a1 is equal to one, and if not, tests if that same longword is zero to determine whether to branch to another routine, branch to a different routine, or branch to a third routine with d1 set to a negative value.
- `update_object_index_and_call_subroutines`: Checks if a pointer-indexed array entry is non-zero, calls a routine, and updates a field in a structure with the returned pointer-index.
- `reset_and_initialize_object_state`: Resets a flag in a structure, checks a pointer-indexed array, and if certain conditions are met, clears a field, sets a counter to ten, and calls another routine.
- `update_object_bounds_and_visibility`: Updates a size or count in a structure, then checks or updates bitmasks or visibility flags in a structure, potentially looping to update a value in a global variable until a certain condition or limit is met, and finally calls another routine.
- `update_object_bounds_and_visibility_2`: Calls another routine to update a value in a structure, loops to check if that value is certain constants, and if certain flags in a1 are zero, calls another routine to update a value in a structure and calls another routine to update a bitmask in a structure.

### LoadWB

- `parse_command_line_and_run`: Program entry point.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_workbench_task`: Starts the Workbench task.
- `compare_strings_ordering`: Compares two zero-terminated strings byte by byte and returns minus one when the first sorts before the second, one when it sorts after, and zero when they are equal.
- `build_linked_list_from_table`: Walks a table of entries, allocating a small node for each one, storing a value from the entry in the node, and chaining the nodes together.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_offset_84`: Calls a function via a pointer located at an offset from a library base address.
- `execute_indirect_function`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `dispatch_task_queue`: Calls the exec dispatch function through the stored library base, which selects the next task from the ready queue to run.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_through_pointer`: Writes the low byte of the data register to the memory location held in the address register, then returns.
- `compare_string_prefixes`: Checks if a2 is null or certain, then checks if a byte at a2 plus 20 is '2'. If so, it compares a string at a2 with a global string, potentially padding with dots if a null terminator is found.
- `write_version_padding`: Writes a period followed by four spaces into a fixed field of the buffer it is given.
- `copy_cstring_to_buffer`: Copies a zero-terminated string byte by byte from the source into the destination buffer, including the terminating zero.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends with a return.

### Version

- `run_version_command`: Program entry point for the Version command.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses the command arguments, opens the named library or device, reads its version and revision from the library base, and compares them with the user-supplied minimums.
- `call_fixed_code_then_forward`: Calls a routine with its first argument and a fixed code, then passes the second argument on to a further routine and returns.
- `parse_signed_decimal`: Reads a signed decimal number from a string by skipping an optional leading plus or minus sign and accumulating digits until a non-digit character, then applies the sign to the result.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `measure_cstring_length`: Counts the bytes that come before the terminating zero byte of a string and returns that count.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatch_offset`: Calls a routine using a pointer retrieved from a global variable with a specific offset.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `allocate_memory_block`: Calls the exec memory allocation function through a stored library base pointer, passing its arguments through and returning the address of the block.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414`: Loads a library base pointer from a global variable and calls the library function at a fixed negative offset from that base, returning its result.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Fetches a function pointer from a location calculated by an offset from a global variable and executes it.
- `invoke_and_verify_stack_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over the local stack buffer until a null terminator is reached.
- `store_byte_via_pointer`: Stores a byte from a register into a memory location pointed to by another register.

### SetMap

- `parse_command_string`: Parses a command string by splitting it into tokens based on whitespace and handling quoted substrings.
- `update_state_and_trigger_callbacks`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `execute_indirect_function_3`: Loads a function pointer from a global variable and executes it.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `reschedule_tasks`: Calls the exec reschedule function through the stored library base, which lets the scheduler reconsider which task should run next.
- `free_saved_blocks`: Releases each recorded block in a counted table of saved pointers, then releases the remaining saved blocks and the block passed in, using a release routine for each.
- `save_arguments_and_run`: Stores its two arguments in global variables, then calls a routine with fixed size and count values and a final routine with a fixed code.
- `load_keymap_for_console`: Loads a keyboard keymap for the console from the keymaps directory or from the keymap resource.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_5`: Fetches a function pointer from a location relative to a global variable and executes it.
- `execute_indirect_function_6`: Fetches a function pointer from a specific offset within a global structure and executes it.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `execute_indirect_function_7`: Fetches a function pointer from a location within a global table and executes it.
- `execute_indirect_call`: Calls a function pointer retrieved from an offset within a structure pointed to by a global variable.
- `call_library_vector_minus_444`: Loads an Amiga library base pointer from a global variable and calls the library function at offset minus 444 from it.
- `call_library_vector_minus_552`: Loads an Amiga library base pointer from a global variable and calls the library function at offset minus 552 from it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_minus_366`: Loads an Amiga library base pointer from a global variable and calls the library function at offset minus 366 from it.
- `call_library_vector_minus_384`: Loads an Amiga library base pointer from a global variable and calls the library function at offset minus 384 from it.
- `call_library_vector_minus_96`: Loads an Amiga library base pointer from a different global variable and calls the library function at offset minus 96 from it.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_argument_to_library_stub`: Passes its one incoming argument, a library base pointer read from a global variable, and the address of its own stack argument to the library call stub at offset minus 132.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `measure_stack_string_and_forward`: Calls a function pointer stored in a global variable, then counts the characters of a 140-byte stack buffer up to its null terminator.
- `store_byte_through_a3_pointer`: Writes the low byte of data register D0 to the memory address held in address register A3 and returns.

### More

- `parse_command_line_and_start`: Program entry that opens the dos library and allocates a control block, then splits the command line into argument strings, honoring double quotes and asterisk escapes.
- `release_file_and_library_state`: Fetches a state pointer from a global library base, stores the given value in it, and conditionally closes a file, closes a library, frees memory and replies to a message based on flags in that state.
- `run_file_pager_session`: Sets up a pager session for one file, allocating its control block, opening the file and an output window, and choosing the display mode from the arguments.
- `reschedule_four_global_slots`: Calls the reschedule library stub four times, each time pairing a field of its structure argument with one of four consecutive global words.
- `scan_buffer_for_control_bytes`: Fills a buffer of 80 bytes through another routine, makes a second call with zero and minus-one arguments, then scans the bytes.
- `dispatch_pager_key_input`: Waits for a console input signal, reads one key and dispatches it through a large switch on character codes.
- `execute_page_routines`: Checks if a value at a2 plus 60 is positive, then calls a routine using a2, calls another routine using a2, and jumps back to the start.
- `call_through_address_register`: Calls a routine with a global argument, calls a second routine, then jumps through a function pointer held in address register A3.
- `initialize_session_and_loop`: Sets a counter to 1 and clears a flag in its structure, stores a routine result in one field, and repeats another routine call while the counter stays positive and a size comparison holds.
- `update_entity_and_increment_counter`: Calls two routines on a buffer inside its structure and stores the second result in a field of that structure.
- `read_line_convert_formfeed`: Reads up to 128 bytes into a buffer, scans to the next newline or form-feed, and repositions the stream past that line.
- `run_step_loop_until_quit`: Runs a loop of 9, 13 or 14 steps, with the count chosen by flags in its structure, calling routines from a global table of pointers on each step.
- `search_stream_until_match`: Formats a search message from a flag-selected global string, then reads and tests the stream until a match is found or the attempt counter drops below one.
- `wrap_call_with_setup_teardown`: Runs a setup call, a call using its structure and a global, and an optional second global call.
- `call_pair_then_flagged_once`: Calls another routine with two arguments, then calls it again with its first argument alone only when a short flag at offset 98 in the structure is zero.
- `call_stream_then_window_bound`: Calls one routine with a stream handle field, a second field and minus one, then calls another routine with a window bound value read through a structure pointer.
- `call_stream_then_decrement_bound`: Calls one routine with a stream handle field, zero and one, then calls another routine with the window's bound value minus one.
- `display_text_lines_up_to_count`: Reads text in chunks of at most 128 bytes and counts newline characters until the requested number of lines is reached.
- `reschedule_with_structure_field`: Passes its second argument to another routine, then calls the reschedule stub with the structure's field at offset 16, a global address and its first argument.
- `mark_flag_then_forward`: Sets a short value in a linked structure to one when a field at offset 4 is zero, then calls the reschedule stub with the structure's field at offset 16 and a global address.
- `select_reschedule_arguments_by_flag`: Chooses its second and third arguments by a flag in its structure: either two global addresses, or a global address and a window bound plus one.
- `reschedule_with_flag_arguments`: Calls the reschedule stub with a structure field and either three global addresses or one global address and a window bound value, chosen by a flag in the structure.
- `call_twice_and_clamp_result`: Calls another routine twice with the same first argument and different trailing arguments, then returns the second result with any negative value replaced by zero.
- `consume_buffered_bytes`: Copies up to the requested number of bytes from the structure's buffer into the destination, refilling through another routine first when the buffer is short.
- `refill_read_buffer`: When the 4096-byte read buffer is exactly full, passes its pending bytes to another routine first.
- `update_scroll_window`: Computes a target scroll position from its offset argument, then compares it with the visible window range in its structure.
- `read_line_into_buffer`: Reads characters one at a time through another routine into a destination buffer, storing the newline and stopping there or after 127 characters.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination one at a time, including the terminating null byte.
- `convert_string_to_uppercase`: Converts each lowercase ASCII letter in a null-terminated string to uppercase in place by subtracting 32, and leaves all other characters unchanged.
- `parse_signed_decimal_string`: Reads an optional plus or minus sign followed by decimal digits from a string and returns the signed integer value.
- `call_library_if_pointers_set`: Calls a library routine with a pointer from its structure, its first argument and minus one, but only when that first argument and the structure's field at offset 4 are both nonzero.
- `close_pager_state_and_free`: Tears down a pager session by closing its file and window handles and restoring saved pointers, with a nonempty text argument and a flag causing input to be read until a newline.
- `call_with_single_value_buffer`: Gets a pointer from another routine, stores its argument in a one-slot stack buffer, and passes that buffer with code 994 and length one to a second routine.
- `allocate_block_and_copy_fields`: Allocates a 36-byte block and, if that succeeds, passes a value derived from its address to a library call through a stack buffer.
- `create_block_from_source_words`: Allocates a 68-byte object, copies a given number of words from a source array into it, and links it to a handle from another routine.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_exec_reschedule_vector`: Calls the Exec library Reschedule function through an exec base pointer kept in a global variable, at offset minus 48.
- `call_exec_switch_vector`: Calls the Exec library Switch function through an exec base pointer kept in a global variable, at offset minus 54.
- `call_exec_dispatch_vector`: Calls the Exec library Dispatch function through an exec base pointer kept in a global variable, at offset minus 60.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_exec_allocmem_vector`: Calls the Exec library AllocMem function through an exec base pointer kept in a global variable, at offset minus 198.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_minus_132`: Calls a library routine at offset minus 132 from a library base pointer held in a global variable.
- `call_library_vector_minus_138`: Calls a library routine at offset minus 138 from a library base pointer held in a global variable.
- `call_library_vector_minus_198`: Calls a library routine at offset minus 198 from a library base pointer held in a global variable.
- `call_library_vector_minus_210`: Calls a library routine at offset minus 210 from a library base pointer held in a global variable.
- `call_library_vector_minus_294`: Calls a library routine at offset minus 294 from a library base pointer held in a global variable.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_minus_366_2`: Calls a library routine at offset minus 366 from a library base pointer held in a second global variable.
- `call_library_vector_minus_372`: Calls a library routine at offset minus 372 from a library base pointer held in a global variable.
- `call_library_vector_minus_384_2`: Calls a library routine at offset minus 384 from a library base pointer held in a global variable.
- `call_library_vector_minus_414`: Calls a library routine at offset minus 414 from a library base pointer held in a global variable.
- `call_library_vector_minus_552_2`: Calls a library routine at offset minus 552 from a library base pointer held in a global variable.
- `call_library_vector_minus_624`: Calls a library routine at offset minus 624 from a library base pointer held in a global variable.
- `forward_with_stack_slot_pointer`: Passes its two arguments to another routine together with a pointer to a fixed slot 12 bytes into the caller's stack frame.
- `read_one_byte_or_minus_one`: Calls another routine to read one byte into a small stack buffer.
- `measure_stack_string_and_forward_2`: Calls a function pointer from a global variable to fill a 140-byte stack buffer, then counts the characters up to the null terminator.
- `store_byte_through_a3_pointer_2`: Writes the low byte of data register D0 to the memory address held in address register A3 and returns.
- `divide_32_bit_unsigned`: Divides a 32-bit unsigned value in D1 by the divisor in D2 and returns the quotient and remainder in two registers.
- `multiply_32_bit_integers`: Multiplies the 32-bit values in D0 and D1 from their 16-bit halves and returns the low 32 bits of the product.
- `call_adjacent_routine_and_return`: Calls one unnamed routine and returns, passing no arguments.
- `call_helper_return_register`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag_2`: Handles the signs of two values, calls a helper, and negates the result when the flag the helper leaves in an address register is nonzero before returning.
- `negate_result_on_flag`: Calls another routine and returns its first result, negated when the second result register is nonzero.
- `call_rawdofmt_from_global`: Calls the Exec RawDoFmt function through a function pointer stored at a fixed offset in a global variable.
- `store_byte_to_pointer`: Stores a byte from a register into a memory location pointed to by a register.
- `call_library_vector_minus_276`: Calls a library routine at offset minus 276 from a library base pointer held in a global variable.
- `allocate_signal_message_block`: Allocates a signal bit and, if that succeeds, allocates a 34-byte message block.
- `free_signal_message_block`: Marks a 34-byte signal message block as deleted, first calling another routine on it when its field at offset 10 is nonzero.
- `search_substring_within_line`: Searches a null-terminated line for a pattern string, stopping at a newline, and returns 1 if the pattern occurs as a substring and 0 otherwise.
- `initialize_empty_exec_list`: Initializes a list header in place: the first word points to the following word, the second word is cleared, and the last word points back to the header.

### Format

- `parse_command_line_and_start_2`: Program entry for the Format utility that opens the dos library and allocates a control block, then splits the command line into argument strings, honoring double quotes and asterisk escapes.
- `update_and_cleanup_state`: Retrieves a pointer from a global variable and stores a parameter into a specific slot within the pointed-to structure.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_exec_allocmem_vector_2`: Calls the Exec AllocMem function through an exec base pointer kept in a global variable, at offset minus 198.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `call_exec_switch_vector_2`: Calls the Exec Switch function through an exec base pointer kept in a global variable, at offset minus 54.
- `call_exec_forbid_vector`: Calls the Exec Forbid function through an exec base pointer kept in a global variable, at offset minus 132.
- `call_exec_dispatch_vector_2`: Calls the Exec Dispatch function through an exec base pointer kept in a global variable, at offset minus 60.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_exec_schedule_vector`: Calls the Exec Schedule function through an exec base pointer kept in a global variable, at offset minus 42.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_exec_allocabs_vector`: Calls the Exec AllocAbs function through an exec base pointer kept in a global variable, at offset minus 204.
- `call_exec_reschedule_vector_2`: Calls the Exec Reschedule function through an exec base pointer kept in a global variable, at offset minus 48.
- `format_disk_from_arguments`: Main routine of the Format utility: it reads the DRIVE, NAME and option arguments, checks the drive, and asks the user to insert a disk.
- `release_format_buffers_and_handles`: Releases the buffers and handles that the format routine allocated, calling a release routine for each field that is set.
- `report_disk_error_and_retry`: Performs a disk operation and loops while it returns error code 28 and the retry request succeeds.
- `store_matched_table_byte`: Searches an eight-entry global byte table for the given value and records the matching entry in a field of a structure obtained from another routine.
- `find_drive_list_entry`: Walks a linked list of drive entries until one matches the given number or pointer, marks that entry, and returns 1.
- `calculate_string_length`: Returns the number of characters in a null-terminated string before its terminator.
- `copy_cstring_to_buffer_2`: Copies a null-terminated string into a destination buffer one byte at a time, including the terminating zero byte.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32.
- `compare_strings_ignoring_case`: Compares two null-terminated strings one character at a time after converting each to uppercase.
- `truncate_string_at_colon`: Replaces the first colon in a string with a null terminator, cutting the string short.
- `call_with_single_value_status`: Passes its first argument in a one-slot stack buffer to another routine with code 31 and length one.
- `create_block_from_source_words_2`: Allocates a 68-byte object, copies up to seven words from a source array into it, and links it to a handle from another routine.
- `call_library_vector_minus_354`: Calls a library routine at offset minus 354 from a library base pointer held in a global variable.
- `execute_indirect_function_pointer`: Fetches a function pointer from a specific offset within a global data structure and executes it.
- `call_library_vector_minus_330`: Calls a library routine at offset minus 330 from a library base pointer held in a global variable.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_minus_450`: Calls a library routine at offset minus 450 from a library base pointer held in a global variable.
- `execute_indirect_function_8`: Fetches a function pointer from a global table and executes it.
- `call_library_vector_minus_456`: Calls a library routine at offset minus 456 from a library base pointer held in a global variable.
- `execute_indirect_function_9`: Fetches a function pointer from a specific offset within a global variable and executes it.
- `execute_indirect_function_10`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_vector_minus_336`: Calls a library routine at offset minus 336 from a library base pointer held in a global variable.
- `call_indirect_via_offset`: Fetches a function pointer from a global table using a fixed offset and executes it.
- `call_library_vector_minus_444_2`: Calls a library routine at offset minus 444 from a library base pointer held in a global variable.
- `execute_indirect_function_11`: Fetches a function pointer from a location within a global data structure and executes it.
- `call_library_vector_minus_366_3`: Calls a library routine at offset minus 366 from a library base pointer held in a global variable.
- `call_library_vector_minus_360`: Calls a library routine at offset minus 360 from a library base pointer held in a global variable.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_minus_306`: Calls a library routine at offset minus 306 from a library base pointer held in a global variable.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_indirect_via_pointer`: Calls a routine through a pointer stored at an offset within a global variable.
- `forward_argument_to_library_stub_2`: Passes its one argument to a library call stub along with a library base pointer from a global variable and the address of its stack argument.
- `call_library_vector_minus_348`: Calls a library routine at offset minus 348 from a library base pointer held in a global variable.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_tracks_and_verify`: Formats a disk track by track, building the track data in a buffer and writing it through the disk device, then verifies the tracks.
- `write_value_to_output_handle`: Calls two output routines with fixed arguments of 20, 30 and 1 on a handle taken from a structure, then passes a value from another routine through the same handle.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_via_pointer_2`: Stores a single byte from a data register into a memory location pointed to by an address register.
- `divide_32_bit_unsigned_2`: Divides a 32-bit unsigned value in D1 by the divisor in D2 and returns the quotient and remainder in two registers.
- `run_restoring_division_steps`: Performs a 16-step restoring division: each step shifts a 32-bit value left through the carry and subtracts the divisor from the high half when it fits, setting a quotient bit.
- `multiply_32_bit_integers_2`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products and combines the results.
- `forward_call_to_helper`: Saves d2 on the stack, copies d1 into d2 and d0 into d1, calls the division helper, then restores d2 from the stack and returns.
- `call_helper_return_register_2`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag_3`: Calls the division helper, then negates the 32-bit result in d0 when the 16-bit value left in a0 is nonzero, and returns it.
- `negate_on_flag_2`: Calls a routine and keeps its first result, which is negated when the second result is non-zero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer_3`: Stores a byte from a data register into a memory location pointed to by an address register.
- `allocate_and_initialize_block`: Calls a library function to allocate a memory block, fills in fields including a stored parameter and flag bytes, and calls further library functions, returning the block address or zero.
- `reset_object_and_recurse`: Recursively processes a linked structure when a pointer field is non-zero, sets a byte field to all ones and a four-byte field to all ones, then calls another routine on a byte field value and again with a constant second argument.
- `forward_argument_with_code_48`: Passes its one argument to another routine together with the constant 48.
- `forward_single_argument`: Passes its one argument on to another routine and returns nothing.
- `initialize_system_structures`: Initializes several global pointers and variables by retrieving values from a memory block, potentially setting them to default values if the block is null, and performs a series of function calls to manage these structures.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `call_library_vector_minus_240`: Calls a library routine at offset minus 240 from a library base pointer held in a global variable.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `allocate_node_with_type_and_pointer`: Allocates a node of the size given by its second argument.
- `reset_node_then_forward`: If the pointer is nonzero, sets the node's byte at offset 8 to 255 and writes minus one into two longword fields, then passes the node and the word at offset 18 to another routine and returns its result.
- `call_library_vector_minus_90`: Calls a library routine at offset minus 90 from a library base pointer held in a global variable.
- `call_library_vector_minus_78`: Calls a library routine at offset minus 78 from a library base pointer held in a global variable.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_and_start_3`: Program entry for the CLI utility that opens the dos library and allocates a control block, then splits the command line into argument strings, honoring double quotes and asterisk escapes.
- `call_exec_supervisor_vector`: Calls the Exec Supervisor function through an exec base pointer kept in a global variable, at offset minus 30.
- `call_exec_exitintr_vector`: Calls the Exec ExitIntr function through an exec base pointer kept in a global variable, at offset minus 36.
- `call_switch_task`: Calls a function pointer located at an offset from a global variable to perform a task switch.
- `call_dispatch_handler`: Executes a function pointer retrieved from a global variable via a specific offset.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_exec_makefunctions_vector`: Calls the Exec MakeFunctions function through an exec base pointer kept in a global variable, at offset minus 90.
- `call_exec_findresident_vector`: Calls the Exec FindResident function through an exec base pointer kept in a global variable, at offset minus 96.
- `call_exec_enable_vector`: Calls the Exec Enable function through an exec base pointer kept in a global variable, at offset minus 126.
- `call_exec_allocentry_vector`: Calls the Exec AllocEntry function through an exec base pointer kept in a global variable, at offset minus 222.
- `open_cli_window_and_run_startup`: Creates the shell's control block and a console window from a CON or NEWCON specification, then runs the s:CLI-Startup or s:Shell-Startup script when the argument is zero.
- `build_linked_node_chain`: Walks a list of entries, allocating an 8-byte node for each and storing a value from the entry in it, then links the nodes together with shifted pointers.
- `free_linked_node_chain`: Walks the linked node chain built earlier, releasing each node's secondary item through another routine and then freeing the node with its 8-byte size.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_a`: Calls a function pointer stored at a calculated offset within a global table.
- `call_library_function_b`: Invokes a function pointer by accessing a global variable with a specific offset.
- `call_library_function_c`: Performs a call to a function pointer derived from a global variable and an offset.
- `call_library_function_d`: Triggers a function pointer located at a specific offset from a global variable.
- `call_library_function_e`: Calls a function through a pointer found at a designated offset in a global variable.
- `call_library_function_f`: Executes a function pointer retrieved from a global variable using a specific offset.
- `call_library_function_g`: Invokes a function pointer by calculating an offset from a global variable.
- `call_library_vector_minus_96_2`: Calls a library routine at offset minus 96 from a library base pointer held in a global variable.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_3`: Computes the 32-bit product of two 32-bit integers using 16-bit partial products.
- `forward_call_to_inverse`: Saves d2 on the stack, copies d1 into d2 and d0 into d1, calls the modular inverse routine, then restores d2 from the stack and returns.
- `call_helper_return_register_3`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_inverse_on_flag`: Calls the modular inverse routine, then negates the 32-bit result in d0 when the 16-bit value left in a0 is nonzero, and returns it.
- `negate_on_flag`: Calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it into a buffer.

### Ram-Handler

- `execute_indirect_call_2`: Fetches a function pointer from a global variable and executes it.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_BLIT

- `reset_blit_state_at_entry`: Program entry for AM2_BLIT that calls routines and a library function in a loop from 9 down to 6, then clears a word at a fixed location.
- `decrement_until_minus_one`: Decrements a 16-bit counter register in a loop until it reaches minus one, then returns.
- `unpack_stream_and_clear_cache`: Runs seven passes of bit-driven copying within a buffer, which looks like a decompression loop, then calls the Exec CacheClearU function when a 16-bit global field exceeds 36.
- `decode_bitstream_length_code`: Decodes a variable-length code from a packed bit stream using a small table of global bytes, and stores the result in a global counter.
- `expand_packed_bitstream_block`: Repeatedly reads length and distance codes from a packed bit stream and copies earlier output bytes within the buffer, updating a global count.
- `push_and_branch`: Modifies a byte in d0, then pushes d0, d1, and d2 onto the stack before restoring all used registers and branching to another routine.

### d

- `null_routine`: This routine is empty and contains no executable instructions.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_CPU

- `clear_cpu_state_at_entry`: Program entry for AM2_CPU that calls routines and a library function in a loop from 8 down to 5, then clears a word at a fixed location.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `update_array_with_loop_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 6, calling a routine if a certain condition is met.
- `update_array_with_loop`: Checks if a value in d0 is negative, otherwise it calls another routine and, if d0 is not zero, enters a loop that adds a value from a pointer in a3 to a table and decrements a counter until it reaches zero.
- `branch_or_copy_pointer_step`: Branches forward when the previous comparison was equal, otherwise copies a3 into a1, calls a subroutine, and branches forward to a later point of the flow.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `expand_bitstream_sequence_2`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting exp.
- `expand_bitstream_loop`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `set_bitstream_counts_and_branch`: Sets d0 to 5 and d7 to 3, then branches forward into the bitstream expansion routine.
- `set_bitstream_counts_variant_two`: Sets d0 to 4 and d7 to 2, then branches forward into the bitstream expansion routine.
- `set_bitstream_counts_variant_three`: Sets d0 to 3 and d7 to 1, then branches forward into the bitstream expansion routine.
- `expand_bitstream_sequence`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or or compressed data to a destination.
- `modify_d1_and_branch`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `expand_and_copy_bitstream_2`: Reads a bitstream-encoded sequence where certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or decompressed byte sequence to a destination buffer.
- `expand_and_copy_bitstream`: Reads a bitstream or compressed-style bit-count-based sequence, expanding or skipping data based on a bit-mask, and copies the resulting bytes from a source pointer to a destination pointer while decrementing a counter in d0.
- `store_bytes_and_restore_registers`: Stores the low byte of d0 and d1 and the word in d2 into memory at decreasing addresses through a4, restores d0-d3, d5-d6 and a3-a4 from the stack, then branches forward to a later point of the flow.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon

- `reset_game_state_at_entry`: Program entry for Ambermoon that calls routines and a library function in a loop from 6 down to 3, then clears a word at a fixed location.
- `decrement_d6_until_negative_one`: Decrements the value in register D6 until it reaches negative one and then returns.
- `update_a1_from_a3_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 4, calling a routine if a certain condition is met.
- `loop_copy_longword_pass`: Repeatedly calls the routine that dispatches a pointer and adds a longword from a register to a longword in the buffer at a3, advancing an offset by the high word of each returned value until the low word equals 1.
- `dispatch_then_copy_loop`: Tests whether the byte at the pointer in a0 is zero (per the surrounding flow), copies the pointer in a3 into a1, calls the routine at a fixed offset, then branches back to the start of the routine's own flow.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop_2`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `decode_lzss_bitstream_block`: Reads control bits from a compressed byte stream through a global bit buffer, and either copies literal bytes backward into the output or, when the bit pattern selects a back-reference, reads a length and distance and copies a run of earlier output bytes.
- `expand_bitstream_loop_2`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `loads_lzss_lengths_and_branches`: Loads the constant 5 into d0 and the constant 3 into d7, then branches to a later point in the code.
- `loads_lzss_lengths_variant_and_branches`: Loads the constant 4 into d0 and the constant 2 into d7, then branches to a later point in the code.
- `loads_lzss_lengths_minimal_and_branches`: Loads the constant 3 into d0 and the constant 1 into d7, then branches to a later point in the code.
- `decode_lzss_match_copy`: Reads bit-packed length and distance fields from the compressed stream at a0 using a table of bit counts in a global variable, then copies a run of previously decoded bytes forward in the output buffer, wrapping the destination pointer back to the buffer start when it passes the end.
- `modify_d1_and_branch_2`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `decode_lzss_control_stream`: Repeatedly reads control bits from the packed input at a0, using a table in a global variable to decide whether to emit literal bytes or copy a back-referenced run into the output buffer at a3, with the source offset and run length derived from the bit-decoded values.
- `decode_lzss_style_stream`: Unpacks a compressed byte stream by reading control bits from the input pointer, copying literal bytes or back-referenced runs into the output buffer through a3, and wrapping the output pointer against a ring boundary.
- `store_bytes_and_word_descending`: Stores the low bytes of d0 and d1 and the word in d2 into memory below the pointer in a4, moving a4 downward, then restores several registers from the stack and branches to another routine.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:Ambermoon_install

- `reset_install_state_at_entry`: Program entry for Ambermoon_install that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_d6_to_negative_one`: Decrements the value in register D6 by one in a loop until the value reaches negative one.
- `modify_table_elements_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 3, calling a routine if a certain condition is met.
- `runs_blit_copy_loop`: Repeatedly calls a routine that yields a value and, while the loop condition holds, adds a longword from a register into a memory location indexed by an accumulated offset, then calls a routine through a register pointer and returns to the caller at the end of its outer loop.
- `branches_into_copy_loop`: Branches forward past the copy instruction when the zero flag is set, otherwise it copies a3 into a1, calls a routine at a nearby address, and branches back into the code that follows.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `unwinds_copy_block_backward`: Compares a2 with a6 and, unless they are equal, copies four bytes at a time from decreasing source addresses to increasing destination addresses through a1, repeating the groups while the counter in d0 is nonzero.
- `unpack_lzss_literal_and_match_loop`: Runs the LZSS decompression loop for one block, reading control bits from a global bit buffer and the compressed input in a0, copying literal bytes backward from the input into the output at a4 and copying earlier output bytes when a match is decoded.
- `decodes_lzss_bit_stream`: Shifts one bit at a time from the byte at the pre-decremented address in a0 into the extended flag, rolling bits into d0 and d1 over two passes, and then adds 6 to d0 before branching to the copy stage.
- `sets_lzss_parameters_five_three`: Loads the constant 5 into d0 and the constant 3 into d7, then branches to the shared code at the decompression stage.
- `sets_lzss_parameters_four_two`: Loads the constant 4 into d0 and the constant 2 into d7, then branches to the shared code at the decompression stage.
- `sets_lzss_parameters_three_one`: Loads the constant 3 into d0 and the constant 1 into d7, then branches to the shared code at the decompression stage.
- `decode_lzss_compressed_block`: Reads bits from the compressed input through the shift helper and the length-code table, then computes a copy offset and length and begins copying previously decoded bytes into the output buffer, wrapping the destination pointer around the ring buffer end.
- `shifts_compressed_bits_into_register`: Shifts one bit out of d1 with the extend flag and adds it into d2, reading a new byte backwards from the input at a0 when d1 becomes empty, then branches to a later point in the code.
- `decode_lz_bitstream_loop`: Repeatedly reads bits from the compressed input in a0 into a counter in a register, and then checks a table of length codes, copying bytes from a back-reference in the output buffer.
- `unpack_lz_stream_to_buffer`: Decodes a compressed bit stream read backward from the input pointer in a0, expanding length and distance codes and copying bytes from a back-reference window into the output at a3 until the count in d5 runs out.
- `stores_bytes_and_word_backward`: Stores the low byte of d0 and then the low byte of d1 into memory through the predecrementing a4 pointer, then stores the low word of d2 the same way, and reloads registers d0-d3, d5-d6 and a3-a4 from the stack.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `verify_and_dispatch_data`: Compares a longword at a pointer against a constant and branches if they match.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A).adf:AM2_BLIT#unpacked

- `jumps_to_program_body`: Performs a single absolute jump to the address 44 in the program image, transferring control away without changing any register.
- `validates_header_and_dispatches`: Compares the longword at the address in a0 against a fixed magic value and returns an error code when it differs.
- `writes_fixed_addresses_to_global_slots`: Stores a series of fixed addresses into a global longword one after another, calling another routine after each store, then enters an endless loop that repeatedly stores three more fixed addresses and calls routines.
- `clears_d0_and_runs_dma_update`: Sets d0 to zero, calls a routine that updates DMA and slot state, then calls a second routine and falls through into the code that follows.
- `frees_tables_then_jumps_handler`: Calls the routine that frees the table blocks, then resets the stack pointer from a global longword, loads d0 from another global longword and a0 from a third, and jumps indirectly through a0.
- `clears_globals_then_copies_table`: Zeroes a block of 16-bit global variables, then sets two flags from a bit test, copies a 16-bit table of 50 entries into a global variable block, and fills six global bytes with values taken from the table or zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_records_until_zero_word`: Copies 3-word records from the buffer in a0 to the buffer in a1, counting records in a global variable up to 511, and stops at a zero word or the counter limit, then writes a zero word to the destination.
- `scans_table_for_matching_word`: Walks a table of 6-byte records in a global variable, stepping three words at a time until it reaches a zero word.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `appends_record_under_limit`: Scans a table of four-word records for one matching the values in d7, d0 and d1; if found, it stores d2 into that record's fourth word, and if not found and the global counter is below 512, it increments the counter and appends a new record with those values and a zeroed final word.
- `advances_pointer_six_bytes_branch`: Adds 6 to the address register a0 and then branches unconditionally to a nearby point in the program.
- `append_record_if_under_limit`: If a global counter is below 512, increments it and writes a record of four words to the pointer in a0, taking the word from d7, two bytes from d0 and d1, and the word from d2, then clears the fourth word; otherwise it returns without writing.
- `saves_registers_and_matches_stored_word`: Saves d0, d1, d3, d6, a0 and a1 on the stack, then if the byte at address 1332 equals 1 it loads the address of a table at 3396 into a0 and compares the word at its offset 2 with d7, branching on mismatch.
- `advances_pointer_and_loops`: Adds 4 to the address register a0, then decrements the counter in d6 and branches back to an earlier point of the code inside another routine when it has not reached minus one.
- `stores_conditional_byte_or_word`: Checks that d7 matches a global value and that the words in d0 and d1 are within 1 and two global limits, then stores the word in d2 or the low byte of d2 into a table inside a structure selected by a helper routine, depending on a global flag.
- `stores_byte_into_offset_table`: Stores the byte in d2 into a table of bytes at an offset formed from the word in d1 plus a fixed base of 332, using a pointer obtained from a called routine.
- `conditionally_stores_word_from_structure`: Stores the low word of d0 into a global word when it is below 11 and a global byte has a particular value, then calls a routine and, if another global byte is nonzero, stores a byte looked up from a global table and clears a second global word.
- `clear_upper_byte_conditionally`: Takes the high byte of d0 and, when it is nonzero and differs from a global word, calls another routine; d0 is returned unchanged, with d0 and d7 saved and restored around the work.
- `updates_flag_and_global`: Reads a byte from a global table indexed by d7 and, if it is nonzero, calls routines and tests a word at offset 30 of a returned structure against a mask.
- `loads_constant_256_and_jumps`: Loads the constant 256 into d0 and jumps to another routine, which conditionally adjusts d0 and returns it.
- `loads_constant_512_and_jumps`: Loads the constant 512 into d0 and jumps to another routine, which conditionally adjusts d0 and returns it.
- `loads_constant_768_and_jumps`: Loads the constant 768 into d0 and jumps to another routine, which conditionally adjusts d0 and returns it.
- `loads_constant_1024_and_jumps`: Loads the constant 1024 into d0 and jumps to another routine, which conditionally adjusts d0 and returns it.
- `loads_constant_1280_and_jumps`: Loads the constant 1280 into d0 and jumps to another routine, which conditionally adjusts d0 and returns it.
- `loads_constant_1536_and_jumps`: Loads the constant 1536 into d0 and jumps to another routine, which conditionally adjusts d0 and returns it.
- `stores_bytes_and_copies_buffer`: Takes the high byte of d0 and, when it is nonzero and the matching global byte is set, calls a routine that yields a structure, reads its flag word and a byte, and then stores the global byte and a word before calling further routines based on the masked flags.
- `stores_bytes_and_calls_copy_routines`: Stores the byte in d2 to a fixed address and the word in d1 to another fixed address, then calls a routine and masks d3 with 1160.
- `loads_buffer_address_and_calls_copy`: Loads the address of a fixed buffer into a0 and calls a routine that copies bytes from that buffer into a structure, then restores the saved registers and returns.
- `load_flag_and_jump`: Sets d0 to the constant 256 and jumps to a shared handler that processes the global byte and related structure fields.
- `load_flag_and_jump_200`: Sets d0 to the constant 512 and jumps to a shared handler that processes a different global byte and related structure fields.
- `load_constant_768_and_jump`: Sets d0 to the constant 768 and jumps to the shared handler that reads a separate global byte and copies state through a structure.
- `load_constant_1024_and_jump`: Sets d0 to the constant 1024 and jumps to the shared handler that tests its own global byte before updating state.
- `load_constant_1280_and_jump`: Sets d0 to the constant 1280 and jumps to the shared handler, which checks a further global byte and copies state when it is set.
- `load_constant_1536_and_jump`: Sets d0 to the constant 1536 and jumps to the shared handler that uses the last of the per-slot global bytes.
- `dispatches_callback_index_1`: Sets d0 to 1 and jumps to a shared dispatch routine that selects a callback from a table for index 1.
- `dispatches_callback_index_2`: Sets d0 to 2 and jumps to a shared dispatch routine that selects a callback from a table for index 2.
- `dispatches_callback_index_3`: Sets d0 to 3 and jumps to a shared dispatch routine that selects a callback from a table for index 3.
- `dispatches_callback_index_4`: Sets d0 to 4 and jumps to a shared dispatch routine that selects a callback from a table for index 4.
- `dispatches_callback_index_5`: Sets d0 to 5 and jumps to a shared dispatch routine that selects a callback from a table for index 5.
- `dispatch_slot_six_handler`: Loads the value 6 into d0 and jumps to a shared dispatcher, which handles the slot identified by that value.
- `dispatch_slot_seven_handler`: Loads the value 7 into d0 and jumps to a shared dispatcher, which handles the slot identified by that value.
- `dispatch_slot_eight_handler`: Loads the value 8 into d0 and jumps to a shared dispatcher, which handles the slot identified by that value.
- `dispatch_slot_nine_handler`: Loads the value 9 into d0 and jumps to a shared dispatcher, which handles the slot identified by that value.
- `tests_global_slots_calling_handler`: Saves d7 and a0, then walks six bytes starting at a fixed global address, and for each nonzero byte calls the indexed-slot storing routine.
- `updates_flag_and_clears_global`: Reads a word at offset 30 of the structure returned by a called routine; if bit 5 is clear and a global byte is non-zero, it clears that byte and calls another routine, otherwise it sets the global byte to -1.
- `clear_flags_and_copy_buffer`: Clears a global longword counter, then scans a table of five bytes and, when a nonzero entry is found, calls another routine with its word value and tests a status word at offset 30 against a mask.
- `store_init_constants_and_call`: Stores the constant 222 into a global word and the constant 2352 into a global longword, loads the address of a buffer into a0, calls two routines, then restores registers and returns.
- `copies_buffer_into_structure_fields`: Calls two other routines, then copies a byte from the buffer in a0 into the structure in a global pointer at offset 54 (or stores 1 there when a global flag is set) and copies the following byte to offset 55.
- `copies_table_and_blits_masked_rows`: Makes a long sequence of routine calls, then copies 32 words from a table pointed to by a routine's result into a global area, sets a global counter to 31 and a global word, and makes further calls before returning.
- `chains_render_and_restores`: Calls a pair-swap routine, then a clip-and-blit routine, then a parameter-storing routine, then the pair-swap routine again, then clears a global variable and makes the same sequence of calls once more before returning the result of the first call.
- `sets_flag_and_dispatches_helpers`: Clears a local byte to zero, calls one routine, and if that byte is nonzero it calls a second routine; the byte is never changed by the code itself.
- `calls_helpers_on_flag_byte`: Sets a local flag to false, calls a routine, and calls another routine only when the flag is true.
- `tests_table_entry_and_releases_slot`: Reads the byte in a global table indexed by the low word of d0 and, when it is nonzero, calls two routines, reads a halfword from the record they return at offset 30, and then calls another routine.
- `tests_table_entry_with_mask_variant`: Reads the byte in a global table indexed by the low word of d0 and, when it is nonzero, calls two routines, reads a halfword from the record they return at offset 30, and then calls another routine.
- `tests_table_entry_masked_store`: Reads the byte in a global table indexed by the low word of d0 and, when it is nonzero, calls two routines, reads a halfword from the record they return at offset 30, and then calls another routine.
- `conditionally_store_slot_index`: Reads a byte from a global table indexed by the low word of d0 and, when nonzero, checks a field of a structure and a bit mask in d1; if the mask test passes it stores the index into a global variable, otherwise it leaves that variable unchanged.
- `stores_word_when_masked_clear`: Ands d1 with 1162 and, if the result is zero, stores the low word of d0 into a global variable and calls a routine; otherwise it returns.
- `stores_index_when_table_nonzero`: Loads the address 8 into a0 and tests the byte at a0 plus d0 minus 1; when that byte is nonzero it stores the low word of d0 into a global variable and calls a routine, otherwise it returns.
- `calls_chain_preserving_d1`: Saves d1 on the stack, loads a byte from a fixed absolute address into d1, calls a fixed routine, restores d1 from the stack and returns.
- `calls_three_routines_sequentially`: Calls three separate routines one after another, with no direct data movement, and then returns with d0 and d1 as they were left.
- `calls_chain_with_byte_from_48`: Saves d1 on the stack, loads a byte from a fixed absolute address 48 into d1, calls a fixed routine, restores d1 and returns.
- `calls_chain_with_byte_from_46`: Saves d1 on the stack, loads a byte from a fixed absolute address 46 into d1, calls a fixed routine, restores d1 and returns.
- `calls_chain_with_byte_from_47`: Saves d1 on the stack, loads a byte from a fixed absolute address 47 into d1, calls a fixed routine, restores d1 and returns.
- `calls_three_routines_in_sequence`: Calls three other routines one after another, with no data movement of its own, and then returns.
- `copy_block_then_adjust_counter`: Calls the routine that loads a fixed data block into a1 and copies its zero-terminated bytes, then calls the table-lookup routine on the low byte of d0, which conditionally increments or decrements the byte at offset 19 of the structure in a0 depending on whether the value is 200.
- `loads_source_then_copies_bytes`: Loads the address of a fixed data block into a1, calls the zero-terminated byte copy routine, decrements a0 by one, then restores d0 and a1 from the stack and returns.
- `copies_text_then_restores_pointer`: Calls the zero-terminated byte copy routine with a1 loaded from a fixed second data block, then decrements a0 by one and restores d0 from the saved value.
- `loads_second_source_then_copies`: Loads the address of a second fixed data block into a1, calls the zero-terminated byte copy routine, decrements a0 by one, then restores d0 and a1 from the stack and returns.
- `calls_helper_with_constant_100`: Saves d1 on the stack, loads the constant 100 into d1, calls a fixed routine at a low absolute address, and then restores d1 before returning.
- `saves_registers_and_clears_d7`: Saves d0 to d2 and d7 on the stack, then if the word at address 14132 equals 6 it sets d7 to zero, and otherwise branches to a later point in the code.
- `clamps_d7_by_word_comparison`: Tests whether the word in d0 is positive; when it is, it branches to a later point, and otherwise it sets d7 to minus one before branching on to the following code.
- `compares_word_and_selects_value`: Compares the word in d0 with the word in d1 and, when d0 is not less than d1, sets d7 to zero and branches to a later point; the other path is handled elsewhere.
- `compares_scaled_random_word`: Copies d0 into d2, calls the pseudo-random word advance routine, multiplies d0 by d1 and swaps its halves, then sets d7 to 255 if d2 is greater than the new high word and to 0 otherwise.
- `fills_twenty_entries_random_values`: Writes 20 to a byte at a fixed address offset by d0, then runs a loop 20 times that calls the pseudo-random word advance routine, storing a table-derived shifted byte at offset 2 from a0 and masked values into two more bytes per iteration, decrementing a counter until it reaches -1.
- `conditionally_updates_slot_counter`: When the table index in d7 selects a nonzero global byte and a global mode value is not 6, it looks up a structure, subtracts the word in d0 from a word at offset 202 of that structure, and either leaves it or stores the result and writes the low byte of d0 and the value 10 into two global bytes indexed by d7.
- `stores_word_and_byte_into_table`: Stores the word in d2 at offset 202 of the structure in a0, then calls a routine with the low byte of d0, and writes that byte into a fixed table at an index from d7 offset by -1, and the constant 10 at offset 5 of the same indexed slot.
- `marks_slot_flag_and_clears_word`: When a global byte indexed by d7 is nonzero, it sets bit 13 of the word at offset 30 of a structure, clears the word at offset 202, calls three helpers and, when d7 matches a global word and a global byte is zero, calls one more helper.
- `build_blitter_list_entries`: Computes a row offset from d7 and stores values into global variables, then calls several helpers to fetch a pointer and copies a zero-terminated string from it into a global buffer up to four characters.
- `clears_indexed_slot_and_counter`: Calls several helpers, clears the byte at a1 and the indexed global byte and word slots, decrements a global byte counter, and calls one more helper when d7 matches a global word.
- `initializes_blit_state_and_copies_string`: Computes a value from a global counter and stores it into two global variables, then calls several helpers and copies a null-terminated string from a table entry into a global buffer of up to five bytes.
- `shift_entries_when_flag_set`: When bit 6 of a word at offset 30 of the structure in a0 is set, clears that bit and copies each entry of two tables in the structure forward by one slot, eight entries at offset 42 and ten at offset 122, each entry being four words apart.
- `advances_word_toward_limit`: If the byte at a global table indexed by d7 is non-zero, it reads two words at offsets 204 and 206 of a structure from a called routine, adds them, and when the stored word at offset 202 is below that sum it adds d0 to it, capping at the sum.
- `advances_word_toward_limit_alt`: If the byte at a global table indexed by d7 is non-zero, it reads two words at offsets 212 and 210 of a structure from a called routine, adds them, and when the stored word at offset 208 is below that sum it adds d0 to it, capping at the sum.
- `add_signed_byte_from_table`: Adds the signed byte from a global table, indexed by the low two bits of d2 times two, to the word in d0 and returns the sum in d0.
- `add_signed_byte_from_table_eight`: Adds the signed byte from a global table, indexed by the low three bits of d2 times two, to the word in d0 and returns the sum in d0.
- `clear_128_bytes_global`: Saves d7 and a0, then writes zero to 128 consecutive bytes of a global variable block starting at a fixed location, and restores the saved registers.
- `sets_clears_or_toggles_bit`: When d1 indexes a valid entry in a global table and d2 is 0, 1 or 2, it clears, sets or toggles the bit numbered by the low three bits of d0 in the byte at the computed position in a table, leaving the other bits unchanged.
- `set_bit_or_toggle_bit`: Compares d2 with 1 and, if equal, sets the bit numbered by d1 in the byte at a0; otherwise it branches to a later point that toggles that bit in the same byte instead, then returns.
- `toggles_bit_when_d2_is_three`: Compares d2 with 3; if they are not equal it branches to a later point, otherwise it toggles the bit numbered by d1 in the byte at a0 and branches unconditionally to another later point.
- `return_saved_registers_unchanged`: Compares d2 with 3 and, if it is not equal, branches away; otherwise it tests bit d1 of the byte at a0, then restores d0, d1 and a0 from the stack and returns.
- `update_global_counters_loop`: Walks a global variable table five times, and for each nonzero entry it calls another routine to get a structure and updates the 16-bit field at offset 24 and the 32-bit field at offset 270 when a set of flag bits is clear, then returns the accumulated total in d0.
- `scan_table_and_decrement_slots`: Walks a global byte table of six entries, and for each non-zero entry calls another routine to fetch a record, increments its word at offset 24 and adds 5 to its long at offset 270, then decrements the matching 16-bit slot in the table, and repeats the scan while the counter in d0 and the count of non-zero slots remain non-zero.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `scan_and_stage_table_entries`: Walks a table of six slots in a global variable, and for each non-zero slot calls another routine to fetch a structure, adds 1 to the word at offset 26 and adds 250 to the long at offset 270, and decrements the staged count, looping until the requested count reaches zero or no staged entries remain.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `waits_then_calls_helpers`: Calls a helper routine, runs a countdown loop of 23 iterations on a 16-bit counter, then calls the helper routine again and returns the value held in d0.
- `waits_then_returns_register_pair`: Calls a helper routine, runs a countdown loop of 23 iterations on a 16-bit counter, calls the helper routine again, and returns the values in d0 and d1 combined as one 64-bit result.
- `scans_structure_chain_for_zero`: Calls a helper routine to obtain a pointer, then steps that pointer forward by 6 bytes per iteration for up to 24 iterations, breaking early when the word at offset 4 is zero, and then calls another routine before returning.
- `counts_active_entries_in_global_table`: Walks a global table of six byte-flagged entries and, for each nonzero flag, calls a helper with a counter, tests a bit selected by a byte at offset 2 and a mask in a word at offset 30, increments a counter when both tests pass, and then returns.
- `scans_records_and_counts_matches`: Walks a sequence of 6 bytes using a global flag array, clearing each counter byte and incrementing it while a comparison of record fields against a global table holds.
- `scans_record_and_increments_counters`: Loops over bytes at a0, writing zero to each byte of a buffer until it finds a nonzero source byte.
- `iterates_entries_advancing_coordinates`: Loops over a counter in a global variable, and for each step increments a byte counter in the structure at a3, derives a value from a divisor word at offset 226 and stores it at offset 17, then calls another routine to obtain a delta that is added to the coordinate words at offsets 202 and 204 of the structure in a1.
- `updates_entry_coordinates_and_blits`: Runs a loop that advances a byte counter in the structure at a1, recomputes a row value from a divisor word and a global variable, and repeatedly adds offsets from a helper routine to several 16-bit coordinate fields.
- `loops_over_entries_updating_coordinates`: Runs an outer loop that repeatedly calls the helper routines, then for each count byte read through a pointer it advances a byte counter in a structure, computes a value by dividing it by a 16-bit divisor stored in that structure, and adds helper results to 16-bit coordinate fields in several structures.
- `clears_bit0_and_sets_global_byte`: Saves d0 on the stack, clears bit 0 of a global word, sets a global byte to 255, loads 255 into d0 and calls the DMA and slot state routine, then restores d0 and returns.
- `sets_flag_clears_word_then_calls`: Saves d0, sets bit 0 of the word at absolute address 14648 in place, clears a global byte, calls one routine, and then restores d0 from the stack before returning.
- `saves_registers_and_selects_slot`: Saves registers d0 through d7 and a0 through a3 on the stack, copies d0 into d4 and points a0 at a global structure.
- `collects_active_record_slot_pointers`: Scans a table of 6-byte records and counts those whose word at offset 4 is nonzero and whose flag byte at offset 32 of the structure in a0 lacks bit 1, then, if the count is nonzero, calls several routines and writes a pointer to each qualifying record's structure offset into a pointer table.
- `builds_active_record_pointer_table`: Walks a table of records, and for each record whose word at offset 4 is nonzero and whose flag at offset 32 of the structure in a0 lacks bit 1, stores a pointer to that record's data into a pointer table.
- `stores_pointer_and_constant_pair`: Loads a word from offset 4 of a1 into d0 and, when it is nonzero, decrements it, multiplies it by 60, and tests a bit at offset 32 of a0 plus that index; if the bit is clear it writes a pointer into a0 plus offset 40 and a constant to the postincremented a2 buffer, otherwise it branches past those writes.
- `stores_constant_and_branches`: Stores the longword constant 33889 through the postincrementing a2 pointer, then branches unconditionally to a later point in the program.
- `fills_slot_pointer_table`: Walks a table of records in a1 and, for each record whose word at offset 4 is nonzero and whose flag at offset 32 of the structure in a0 lacks bit 1, stores a pointer into a table at a2 and a fixed address after it.
- `advances_counter_and_stores_byte`: Loads a byte from a global variable, and if it is zero calls a routine that returns a byte in d0 and stores it into a global byte; it then stores the old global byte plus one back into the same global byte and returns.
- `decrements_global_timer_byte`: Saves d0 and d7 on the stack, and when a global byte is nonzero decrements it, calling a routine with d0 loaded from another global byte when the decremented value reaches zero, then stores the byte back and restores the saved registers.
- `dispatch_zero_byte_check`: Saves d0 and a0, then branches to a comparison of a global word against 3 and, if it differs, compares the byte at the pointer in a global longword with d0.
- `run_indexed_handler_and_advance`: If the byte at a0 is zero, calls a handler selected from a table by a global counter, increments that counter, and points a global pointer at a global variable; it then stores a0 into another global pointer and returns.
- `clears_flag_and_stores_pointer`: Sets d0 to zero and jumps to a shared routine that clears a global byte, stores the address in a0 into a global pointer, and writes 255 to a global variable.
- `loads_code_and_jumps_shared`: Loads the constant 293 into d0 and jumps to a shared routine that clears a global byte, stores the address in a0 into a global pointer, and writes 255 to a global variable.
- `scans_flags_and_sets_global`: Loops over six bytes of a global array and, for each non-zero entry, calls a helper and sets bit 5 of a word at offset 30 of the structure it returns.
- `indexes_table_and_calls_handler`: Calls a series of other routines, then shifts the low word of d0 left by 3 to index a global table, copies a longword from that table into a global variable, and calls the handler pointer stored in the same table entry when it is nonzero.
- `blits_fixed_rectangle_one`: Sets d4 to 28 and the corner coordinates in d0 to d3 to fixed values, then calls the rectangle clipping and blit routine once and returns.
- `blits_fixed_rectangle_two`: Sets d4 to 28 and the corner coordinates in d0 to d3 to a different set of fixed values, then calls the rectangle clipping and blit routine once and returns.
- `blits_two_fixed_rectangles`: Sets d4 to 28 and two sets of corner coordinates in d0 to d3, calling the rectangle clipping and blit routine once for each set, then returns.
- `jumps_to_blit_clip_check`: Jumps unconditionally to a fixed address in the blitter code, and that code is itself the only action of this routine, so control leaves it at once.
- `set_rectangle_and_blit`: Loads register d4 with 28, sets d0 to 16, d1 to 138, d2 to 303 and d3 to 192, then calls the clipping rectangle routine with those corner coordinates.
- `blit_two_rectangles_sequentially`: Loads register d4 with 28 and sets the first corner coordinates to 15, 43, 191 and 122, calls the clipping rectangle routine, then sets d0 to 208, d1 to 43, d2 to 303 and d3 to 122 and calls it again.
- `set_rectangle_and_clip`: Loads register d4 with 28, sets d0 to 16, d1 to 50, d2 to 191 and d3 to 193, then calls the clipping rectangle routine with those corner coordinates.
- `clip_two_offset_rectangles`: Loads register d4 with 28 and sets the first corner coordinates to 16, 43, 303 and 122, calls the clipping rectangle routine, then sets d0 to 16, d1 to 144, d2 to 193 and d3 to 193 and calls it again.
- `stores_long_at_indexed_slot`: Stores the 32-bit value a fixed address into a global table at an index formed from the word in d7 minus one, scaled by four.
- `runs_two_row_loops_after_setup`: Calls a routine once, then runs a countdown loop that calls a routine six times, from 5 down to 0.
- `syncs_flags_and_updates_bytes`: Copies a word from one global variable to another and, when they differ, clears the previous slot to a constant and sets the new one.
- `computes_sprite_dimensions_and_copies`: Computes a position from a global index multiplied by 48 plus 16, calls two routines, and branches on a per-index global byte to choose between two positioning paths.
- `update_sprite_state_flags`: Reads a 16-bit flag word from a record selected by a global index and, depending on whether its high bits are set, either clears or sets bit 2 of a status byte and flags a state change.
- `decrements_nonzero_slot_counters`: Calls a helper, then walks six global byte entries, decrementing each nonzero counter and calling another routine when a counter reaches zero.
- `saves_registers_and_branches_forward`: Pushes registers d7 and a0 onto the stack and then branches to a later point in another routine.
- `waits_blit_and_scans_flags`: Repeatedly calls a blitter-waiting routine and a blit routine, then scans six global flag bytes from the start, restarting the outer loop when a nonzero byte is found.
- `sets_blit_flag_when_nonzero`: Calls a clipping and blit routine, then if the low word of d0 is nonzero it stores 2 into a global longword and calls two further routines, returning with d0 and d1 unchanged.
- `updates_slot_table_counter`: Reads a byte counter from a table indexed by d7, computes a longword offset from d7 multiplied by 48, appends a pointer to a list, and then loops 20 times over a global record table, adjusting byte fields at offsets 1 and 2 and decrementing the counter byte when a slot's flag is set before writing the counter back.
- `fill_slot_table_entries`: Loops over a word counter in d7, stores bytes from d1 into the structure at a1 with a per-iteration decrement of the byte at offset 2, and steps a global list pointer back through a fixed address check.
- `scans_table_writes_indexed_bytes`: Walks a table of flag bytes in a loop controlled by a counter, checking each entry against a bitmask, and stores an index or a constant byte through the pointers in a1 and a0 at each step.
- `scans_bits_and_writes_bytes`: Loops over a bitmask in d5 and a counter in d7, searching a global-variable table for a set bit and storing the matching index and a value of 10 plus that index through the pointers in a1 and a3.
- `write_strided_word_and_byte`: Writes the word in d2 to the address in a3 and the low byte of d2 plus 10 to the address in a1, then advances a0, a1 and a3 and increments d7.
- `increments_global_word_counter`: Adds one to a global 16-bit counter addressed by absolute long, leaving all other registers and memory unchanged, then returns.
- `iterates_blit_entries_by_flag`: Walks five 4-byte entries starting from a global variable, and for each entry calls one routine when a word is zero or three routines otherwise, advancing through the entries using the address returned in a1.
- `clamps_word_and_branches`: Compares d7 with 180 and, if d7 is less than that, branches to the code after; otherwise it loads 12 into d7 and branches forward.
- `blits_rows_until_count_exhausted`: Repeatedly calls three routines, then steps through a linked structure, decrementing a word counter taken from the stack until it reaches minus one.
- `saves_registers_and_sets_mask`: Saves registers d0 through d7 and a0 on the stack, stores 31 into a global word, and compares another global word with 6 to choose which word is loaded into d3.
- `updates_masks_and_draws_rows`: Inverts a global word and ORs it into a second global word, then conditionally calls helper routines depending on bits 0 through 5 of the combined value.
- `tests_flags_and_draws_masked_rows`: Branches on bits 0 through 5 of two saved halfword registers to choose among helper calls, writes an all-ones word and a zero word to a global word around a call, stores a constant byte through a0, and returns the saved stack longword.
- `sets_blit_params_then_branches`: Tests bit 2 of d3 and, when it is clear, saves d2 and d3 on the stack, loads constants 272, 73, 303, 104 and 28 into d0 through d4, calls one routine, restores d2 and d3, and then branches to a later point in the code.
- `tests_flag_bits_and_draws_rows`: Tests bit 0 of d2 and bit 0 of d3 to choose one of two code paths, each calling helper routines, then conditionally tests bits 4, 5, 1 and 0 of d2 and d3 and calls further helper routines.
- `dispatches_on_flag_bits`: Tests bit 0 of d2 and then bit 0 of d3, branching to one of two code paths when either bit is set, and otherwise branches unconditionally to a later point in the code.
- `draws_fixed_rectangle_then_branches`: When bit 0 of d3 is clear it saves d2 and d3 on the stack, loads fixed coordinate and size constants into d0 to d4, calls a drawing routine, restores d2 and d3, and branches onward; otherwise it branches directly.
- `loads_parameters_and_branches_onward`: Loads the word 208 into d0 and the word 73 into d1, sets small constants 24, 3, 2 and 32 into d4 to d7, and points a0 at a table.
- `conditionally_draws_text_markers_2`: Tests bits 1, 4 and 5 of the two 16-bit values in d2 and d3, calling routines for each set condition and writing the byte 58 through the pointer in a0 when bit 5 is set.
- `conditionally_draws_text_markers`: Tests bits 1, 4 and 5 of the two 16-bit values in d2 and d3, calling routines for each set condition and writing the byte 58 through the pointer in a0 when bit 5 applies.
- `blits_fixed_rectangle_when_flagged`: Tests bit 1 of d3, and when it is clear saves d2 and d3, loads fixed coordinates and sizes into d0 through d4, calls a rectangle routine, restores d2 and d3, and then branches onward.
- `selects_fixed_blitter_address`: Tests the byte at a global address and, when it is nonzero, branches forward; otherwise it loads a fixed address into a0 and branches to a shared continuation.
- `invokes_blitter_with_fixed_parameters`: Loads a0 with a fixed address, sets d0 to 240, d1 to 49, d4 to 24, d5 to 3, d6 to 2 and d7 to 32, then calls a routine.
- `stores_fetched_byte_in_global`: Saves d0 and d1 on the stack, loads the constant 14 into d1, calls a routine that returns a byte, stores that byte into a global byte, restores d0 and d1, and returns.
- `blits_then_updates_byte_slot`: Calls a blit routine, then a routine that handles a byte value in d0 and may increment a byte at offset 19 of a0, then a routine that may decrement the same byte, before returning with d0 and d1 unchanged.
- `initializes_structure_with_offset`: Stores the register a0 into the longword at offset 4 of the structure in a1, then stores the sum of a value from another routine and 274 into the longword at offset 0 of the same structure and clears the word at offset 8.
- `initializes_slot_pointer_record`: Calls several routines, stores a longword from a0 at offset 4 of the structure in a1, and writes a word computed from a routine result plus 274 into the structure's first longword.
- `stores_offset_plus_274`: Calls two routines, then writes the result plus 274 as a longword through the pointer in a0.
- `stores_adjusted_value_and_pointer`: Calls one routine, adds 274 to its result and stores the sum as a longword at the address in a0.
- `resets_pair_and_swaps_globals`: Sets two global words to 65535, then swaps two longword globals, copies the first into a third global, and inverts a global byte, calling several other routines along the way before a final jump.
- `swaps_buffers_then_blits`: Calls two routines and a third with a0 as argument, then exchanges two longword globals, copies the first into a third global and inverts a global byte.
- `conditionally_adjusts_table_entry`: Takes the low byte of d0 and, when it is non-zero and not 255, computes an index from a global variable and that byte, reads a word from a table at a fixed offset, and calls two routines if that word is non-zero, storing a register value into a global longword.
- `reads_indexed_table_word`: When the low byte of d0 is non-zero, computes a table index from that byte minus 1, reads a word from a table entry at offset 294 of a structure, and calls two routines, storing a register value into a global longword when the word is non-zero.
- `reads_offset_table_word`: When the low byte of d0 is non-zero and not 255, adds a global variable and that byte plus 8 to form an index, reads a word from a table entry at offset 294 of a structure, and calls two routines, storing a register value into a global longword when the word is non-zero.
- `adjusts_index_and_calls_handler`: Branches out when d0 is zero or 255, masks d0 to a byte, decrements it, adds a global word, and when the result equals a global word it calls one routine, then branches onward.
- `tests_value_and_stores_word_global`: Calls two routines, then calls a third and keeps its word result in d1 and its test result.
- `forwards_to_slot_release`: Calls a routine that releases a slot and then returns immediately.
- `dispatch_by_index_value`: Tests the byte in d0 and returns immediately when it is zero; otherwise it masks d0 to a byte, decrements it, and compares it with a global variable word.
- `calls_three_routines_then_conditional`: Calls three other routines in sequence, then calls a further routine and stores the low word of data register D1 into a global variable only when that routine's result is non-zero, after which it calls one more routine before returning.
- `forwards_to_slot_release_alt`: Calls a routine that releases a slot and then returns immediately.
- `stores_sentinel_and_calls_routine`: Saves d0 and a0, multiplies d0 minus 1 by 60 to form an offset into a fixed buffer, stores the word 2 into a global variable, adds 40 to the pointer, calls a routine, and restores the saved registers before returning.
- `waits_and_clips_blit_pair`: Runs a counted loop of twelve iterations, each calling a wait routine, then makes a final call to another routine.
- `accumulates_record_word_over_range`: Loops 12 times calling a helper routine that looks up a record and returns a word, keeps the last returned word, and returns it incremented by one.
- `computes_centered_blit_mask`: Calls a set of helper routines to locate a 6-byte record from d0 minus 1 in a table, reads the word at offset 4 of that record and, when nonzero, calls further routines and sets a global byte to 2 when a byte at offset 0 is not 1.
- `locates_table_record_and_blits`: Computes the address of a 6-byte record at index d0 minus 1 from a table base offset by 290, calls a routine, and when the record's word at offset 4 is nonzero calls routines using that count minus 1 scaled by 60.
- `advances_byte_stream_cursor`: Decrements a global countdown word and, when it reaches zero, advances a global stream index and reloads the countdown with 4.
- `advances_byte_table_cursor`: Decrements a global countdown word and, when it reaches zero, increments a global table index and reloads the countdown with 4.
- `select_and_decrement_counter`: Chooses a value from a global byte, calling a helper with 0, 1 or 2 as a parameter, then calls another helper and, when the byte equals 200 and a field of a structure is nonzero, decrements that field.
- `dispatch_text_or_table_value`: Compares the low byte of d0 with 1 and, when they differ, saves d0 and loads a global byte into d0 before calling another routine; it then loads a pointer into a1 offset by 15360 and a2 from a fixed address, and branches to a later point in the code.
- `dispatches_on_text_or_table_value`: Compares d0 with 2 and, when different, saves d0 and loads a byte from a global location into d0 before calling a routine that returns a pointer into a1.
- `decrements_slot_byte_conditionally`: Calls one routine and compares a global byte with 200; if the value is below 200 it calls another routine, and when the original value equals 200 it decrements the byte at offset 19 of the structure in a0 if that byte is nonzero.
- `set_slot_and_chain_jump`: Calls three routines in sequence, stores the value 31 into a global variable, calls one more routine, and returns with d0 as it was left by the first call.
- `stores_registers_and_resets_global_byte`: Copies a0 into a global longword, copies d6 and d7 into two further globals, then calls two routines and clears a global byte to zero before returning.
- `stores_registers_and_sets_global_byte`: Copies a0 into a global longword, copies d6 and d7 into two further globals, then calls two routines and sets a global byte to 255 before returning.
- `releases_slot_when_active`: Compares a longword global variable against -1 and, when it differs, calls routines, clears a global byte flag check and then clears the same longword global to -1.
- `calls_when_flag_nonzero`: Calls one routine, then tests a global-variable byte and, when it is nonzero, calls another routine.
- `compute_clamped_offset_store`: Saves d0, d1 and d7, stores d1 minus 7 into the word at offset 2 of the structure at a0, and computes a value from d7 times 6 minus 1 halved and negated, added to a global word.
- `stores_clamped_word_complement`: Adds d7 and d0 with a one-unit reduction, and if the sum is 320 or greater, replaces d0 with 320 minus d7.
- `transforms_word_run_through_helper`: Saves d0, d1, d7 and a0 on the stack, copies d0 into d1, advances a0 by 2, then 31 times reads the word at a0, passes it through a helper call, and stores the result back at a0 with post-increment.
- `saves_registers_and_selects_blit_record`: Saves registers d0-d7 and a0-a2 on the stack, returns immediately if d0 is 9 or greater, and otherwise copies d0 into d4, increments d4, and loads two table addresses into a1 and a2 before branching to a later point.
- `blits_centered_masked_rows`: Looks up a word from a table using d0 and a global value, then calls a blitter routine three times for each of four row groups, with a row count taken from the low three bits of a returned word plus eight.
- `clears_table_and_counts_entries`: Sets a global word to -1 and a global byte to 255, then copies non-sentinel words from a table into a0 for 1024 entries, counting them into a global word.
- `calls_two_routines_in_sequence_5`: Calls one routine and then a second routine in sequence, then returns to its caller.
- `copies_byte_and_jumps_onward`: Copies the byte at a fixed absolute address into d0, calls a routine with that value, and then jumps to a fixed later address without returning.
- `swaps_globals_and_toggles_flag`: Calls a sequence of routines, exchanges two longword globals, copies one of them into a third global, inverts a byte global by complementing it, then calls a final set of routines and returns.
- `conditionally_runs_two_cleanups`: Tests a 16-bit global counter and, when it is nonzero, calls two routines; otherwise it returns.
- `computes_text_row_offset`: Reads a global word and subtracts a second global word from it, clamping the difference to 16 when it is at least 17, and then calls several routines.
- `sets_word_or_clears_bit`: Loads a constant into a0 and a word from the pointer in a1 into d0, sets d1 to 6, then calls a bit helper.
- `blits_text_rows_and_decrements_byte`: Sets a global word to 31 and calls two routines, then loops from d7 to the start of its block adding 7 to d3 for each pass, calling a bit-level routine on each pass.
- `calls_helpers_on_flag_byte_variant`: Loads constants 144 and 164 into d0 and d1 and 30 into d3, then calls a routine.
- `run_bit_and_rectangle_bookkeeping`: Stores the value 2 into a global variable, calls a routine, and if that call leaves a flag set it makes a series of calls, including a check of a global byte and a global address dereferenced through a0.
- `copies_indexed_table_entry_to_global`: Takes the high byte of d0 and, when it is nonzero, adds a global byte to it and compares the result to a longword global; when within bounds it loads a word from a table at that index into a global, sets two global words, calls routines for the blit, and clears a global byte.
- `clears_scroll_counter_and_sets_mask`: When the byte in d0 is nonzero and the sum of a global byte and the high byte of d0 is at most a global limit, it calls four routines, clears one global word, sets another global word to 65535, and calls two more routines before clearing the first again.
- `clears_counter_and_sets_mask`: Calls two routines, then clears one global word and sets another global word to 65535.
- `calls_pair_and_swaps_result`: Calls one routine, saves the word it returns in d0, calls a second routine and then a third, and returns the saved word.
- `compares_strings_until_null`: Calls two routines and then compares bytes at the pointers in a0 and a1 one by one, advancing both.
- `swaps_two_word_table_entries`: Doubles the indices in d5 and d6 to word offsets, then exchanges the 16-bit entries at those two offsets relative to a6 with each other, using d0 as a temporary; d0, d5 and d6 are preserved.
- `selects_slot_and_checks_limit`: Saves the registers d0, d1, a0 and a1 on the stack, reads a byte from a global variable, and calls a routine to obtain a pointer in a1.
- `counts_down_then_clears_byte`: Decrements the word in d0 in a loop until it reaches minus one, then calls a routine, clears the byte at the address in a0 and calls another routine.
- `matches_run_length_entries`: Reads a count word from the pointer returned by a call and then walks length-prefixed byte entries, comparing each entry's bytes against a string at a0 by a nested loop.
- `initialize_copy_pointers`: Clears d6, loads a byte from the buffer at a1 into the low byte of d6, copies that count to d5, copies a0 into a2 and a1 into a3, and then jumps into the loop-entry code of another routine.
- `decodes_run_length_buffer`: Reads bytes from a buffer through a1 and a lookup table relative to a4, comparing each decoded byte against the byte at a2 and advancing a3 and the counters.
- `release_slot_and_store_globals`: Reads a global byte indexed by the high byte of d0 and, when it is nonzero, calls routines and stores that byte and the low word of d1 into globals before calling further routines.
- `store_byte_and_word_globals`: Stores the low byte of d2 into a global byte and the low word of d1 into a global word, then calls three helper routines and returns.
- `load_constant_and_branch`: Loads the constant 256 into d0 and branches to a shared routine at a fixed address, which takes over from there.
- `load_constant_and_branch_2`: Loads the constant 512 into d0 and branches to a shared routine at a fixed address, which takes over from there.
- `load_constant_and_branch_3`: Loads the constant 768 into d0 and branches to a shared routine at a fixed address, which takes over from there.
- `load_constant_and_branch_4`: Loads the constant 1024 into d0 and branches to a shared routine at a fixed address, which takes over from there.
- `load_constant_and_branch_5`: Loads the constant 1280 into d0 and branches to a shared routine at a fixed address, which takes over from there.
- `load_constant_and_branch_6`: Loads the constant 1536 into d0 and branches to a shared routine at a fixed address, which takes over from there.
- `conditionally_release_slot_and_store`: Like the routine that stores globals, but it only stores the byte and word globals and calls the helpers when the flag word of a returned structure has the masked bits clear; otherwise it only calls one routine.
- `conditionally_store_byte_and_word`: Tests d3 against the mask 1162 and, when the result is zero, stores the low byte of d2 and the low word of d1 into globals and calls three helper routines; otherwise it returns.
- `load_constant_and_branch_conditional`: Loads the constant 256 into d0 and branches to a second shared routine at a fixed address, which takes over from there.
- `load_constant_and_branch_conditional_2`: Loads the constant 512 into d0 and branches to a second shared routine at a fixed address, which takes over from there.
- `load_constant_and_branch_conditional_3`: Loads the constant 768 into d0 and branches to a second shared routine at a fixed address, which takes over from there.
- `load_constant_and_branch_conditional_4`: Loads the constant 1024 into d0 and branches to a second shared routine at a fixed address, which takes over from there.
- `store_constant_then_branch`: Loads the constant 1280 into d0 and branches to a shared later routine, which performs the stores and calls.
- `store_constant_then_branch_alt`: Loads the constant 1536 into d0 and branches to the same shared later routine as the preceding routine.
- `load_seven_loop_sequence_hooks`: Calls two helper routines with the value 6 in d0, then fetches a pointer and, when a global pointer is nonzero and a longword at offset 38 of the structure it points to is nonzero, sets a global byte to 255 and calls further helpers.
- `clears_flag_then_jumps`: Calls the pair-swap routine and then the slot-release routine, clears a global byte, calls three more routines and tail-jumps to another routine.
- `dispatches_status_update_sequence`: Calls four routines in sequence: a slot synchroniser, a counter decrementer, a mask comparer and a status writer, then jumps to a final routine.
- `sets_high_bit_if_flags_set`: Calls a routine to obtain a pointer, masks the word at offset 30 of that structure with 1162, and sets bit 7 of the byte at the address in a0 when the masked result or the byte at offset 9 is nonzero.
- `chains_release_and_handler_call`: Calls one routine to release a linked slot and its callbacks, then calls a handler through the code pointer at offset 30 of a global structure when that pointer is non-null, and finally jumps to a further routine.
- `copies_state_and_updates_slot_table`: Calls two helper routines, then copies one byte from the buffer in a0 into a structure field at offset 54 and a second byte at offset 55.
- `dispatches_slot_by_byte_value`: Tests the byte in d0 and returns immediately to the caller when it is zero.
- `searches_byte_table_for_bit_match`: Reads a word from a structure at a fixed offset and tests its top bits, then scans a global byte table two bytes at a time, testing each entry's bit against a mask and counting down to the target match.
- `scans_byte_table_for_bits`: Loads the address of a global variable into a0 and reads bytes from it in a loop that adds two to the address each pass.
- `store_word_and_test_bitmask`: Stores the word in d0 into a global variable, copies the byte of d1 into d0, and loads the constant 58510 into d1.
- `stores_word_pointer_and_calls_routines`: Stores the word 4 into a fixed absolute address and points a0 at a table.
- `releases_slot_then_returns`: Calls the slot-release routine and then returns.
- `draw_status_text_block`: Calls several helper routines to draw a block of text rows, writing the values 22 and then 2 to a global byte around the drawing, and it tests bit 0 of the byte at offset 8 of the structure in one address register to decide whether to call another routine.
- `walks_bit_table_calls_handler`: Iterates over a table of byte pairs in a global variable, ending at a sentinel value of 255, and calls a routine for each entry whose first byte indexes a set bit in d6.
- `calls_routine_with_saved_registers`: Saves d0 on the stack, loads a global-variable byte into d0 and calls a routine, then restores d0 and the registers d0 through d7 and a0 through a4 before returning.
- `blits_status_text_row`: Calls several helpers, writes 16 and then 2 to a global byte, and tests bit 0 of a byte from the structure in a1 offset by 2 against a word-shifted mask.
- `draws_status_text_and_masks`: Calls several helpers and tests a bit selected by a byte read from the structure in a1 against a word value; when the bit is clear it runs five further helper calls.
- `writes_status_chars_to_a_pointer`: Calls a clipped rectangle blit and a masked blit, stores the byte 47 through a pointer in a0, calls further routines, writes 47 again through a second pointer when a byte at offset 4 of a1 is nonzero, then writes 32 through a third pointer and sets a global word to 2 or to another global depending on two flags.
- `sets_timing_word_around_drawing`: Stores 2 into a global word, calls routines to draw, and stores either 2 or a global longword into the same word depending on a global flag.
- `sets_timing_word_and_calls`: Stores the value 2 into a global word, calls several routines, stores 2 into the same global word again, and then calls three more routines before returning the stack value unchanged.
- `dispatch_to_jump_chain`: Calls one routine and then jumps to another fixed address without returning here directly.
- `scan_and_expire_slots`: Walks a table of 40-byte entries up to 6 at a time, skipping entries whose first byte is zero.
- `dispatch_slot_by_type`: Checks a type byte in the structure at a1 and, for type 4 or 7, compares selected fields against entries of a table, clearing an active slot's type byte and setting its link word to 65535 when a comparison fails.
- `scan_active_slots_and_reset`: Walks a table of 40-byte entries in steps of 40, skipping entries whose first byte is zero, and for each active entry calls a helper with the slot index and checks a type byte at offset 4 of the helper's result.
- `run_inventory_display_sequence`: Calls a series of helper routines, including one that indexes a global table, then when a global pointer is nonzero and a longword in its structure is nonzero, sets a global byte to 255 and calls further helpers, returning the value obtained earlier.
- `loads_structure_and_runs_hooks`: Calls one routine, then if a global pointer is nonzero and the longword at offset 38 of that structure is nonzero, it sets a global byte to 255 and calls three routines.
- `pops_stack_then_redraws`: Calls the pair-swap routine, then a stack-pop routine that restores a pointer, then the slot-release routine, clears a global byte and tail-jumps to another routine.
- `updates_display_marks_from_structure`: Calls five routines, sets a global byte to 2, then reads a word at offset 270 and words at offsets 42 and 46 from a structure returned by a call.
- `copies_buffer_into_slot_and_releases`: Calls a routine and a second routine, copies a byte from the buffer in a0 into a structure field at offset 54 (or stores 1 when a global flag is set) and a second byte at offset 55, then copies nine longwords between two tables with the sentinel replaced by the previous entry.
- `calls_three_routines_then_handler`: Calls two routines, then calls a handler through the code pointer at offset 30 of a global structure when that structure and pointer are non-null, and finally jumps to a further routine.
- `dispatch_sprite_slot_update`: Reads a 16-bit value from a global variable and, when it is non-negative, computes a 6-byte table entry address and branches on a state byte in that entry.
- `conditionally_updates_sprite_animation`: Checks a global flag and, when it is clear, tests a bit in a table entry and a byte at offset 1 of the structure in a2 to choose among several branches.
- `tests_flag_bit_then_branches`: Reads a byte at offset 3 of the structure in a0 into d0 and a word at offset 34 of the structure in a2 into d1, then tests bit d0 of d1.
- `tests_bitmask_and_stores_record_values`: Computes a 5-byte record offset from the structure in a2 and tests bits of the longword read from a global table against a mask taken from a global variable; on the set-bit path it copies two bytes from the structure into global words and stores 255 into a global byte, otherwise it calls a single routine and returns the value it produces.
- `conditionally_calls_fixed_routine_2`: Reads a global word, adds 5 and tests that bit in d0, skipping a conditional branch; otherwise it loads 240 into d0 and calls one routine, then branches forward to a later address.
- `updates_record_and_calls_handlers_2`: Reads two global words as indices into a table of 40-byte records, copies bytes between that record and a global variable, sets a byte field of the record to 7 in one branch, and calls several other routines before clearing a global byte and returning.
- `updates_record_and_calls_handlers`: Calls another routine, then uses two global words as indices into a table of 40-byte records, copying bytes between that record and global variables and storing a value into one of its bytes.
- `checks_type_then_tests_bit`: Compares the byte at offset 1 of the structure in a2 with 19 and, when it matches, reads a global word into d0 and the byte at offset 21 of the structure into d1 and tests bit d1 of d0.
- `sets_bit_and_calls_helpers`: Sets a bit selected by d1 in the byte in d0, stores the low word of d0 to a global word, reads a global word into d0, adds 1, and calls one routine; it then loads 43 into d0, calls another routine, and branches to a later point in the code.
- `dispatches_on_structure_type_byte`: Checks the byte at offset 1 of the structure in a2 against the values 8 and 20: for 8 it makes six calls, for 20 it tests a global word and a global table bit and makes up to four calls, and for any other value it makes a single call before returning the result of the last call.
- `checks_byte_and_calls_fixed_routine`: Compares the byte at offset 1 of the structure in a2 with 20, and if equal and a global word is nonzero it loads 63 into d0 and calls one routine, otherwise it branches elsewhere.
- `dispatches_conditional_routine_calls`: Tests a global byte, a global flag word and a bit of a global word against a bit number stored at offset 21 of the structure in a2, then makes either three calls followed by a fourth or a single call followed by one more call.
- `saves_word_around_calls`: Loads the constant 282 into d0, calls one routine, pushes the returned word onto the stack, reads a byte from a global variable into d0 and calls a second routine, then pops the saved word back into d0 and returns.
- `select_table_entry_and_flag`: Sets a global byte to 28, reads a global word and returns it immediately when negative.
- `stores_record_fields_and_sets_flag`: Sets a global byte to 1, and when bit 2 of the byte at offset 32 of the structure in a1 is set and the byte at a0 is not 1, it copies that byte and the word at offset 4 of a0 into global variables, stores 40 into another global, clears two globals, and calls another routine.
- `adjusts_counter_and_stores_pointer`: Calls two routines, loads a word from a structure at offset 24 into a global longword, sets two global variables to 32 and 1 and clears a third to 0.
- `adjusts_counter_scaled_by_250`: Calls two routines, loads a word from a structure at offset 26 into a global longword, and sets two global variables to 33 and 2 and clears a third.
- `loads_table_entry_into_globals_2`: Calls two routines, sets a global byte to 31 and reads a global word.
- `adjusts_record_position_by_five`: Looks up a record from the table-value routine and subtracts the amount in a global variable, multiplied by five, from the 16-bit field at offset 24 and the 32-bit field at offset 270 of that record, then adds the same scaled amount back to a record found through a second lookup.
- `transfers_scaled_amount_between_fields`: Calls a routine, stores a pointer to a global variable, and calls a routine returning a value.
- `adjusts_record_position_by_250`: Looks up a record from the table-value routine and subtracts the amount in a global variable, multiplied by 250, from the 16-bit field at offset 26 and the 32-bit field at offset 270 of that record, then adds the same scaled amount back to a record found through a second lookup.
- `transfers_250_scaled_amount_between`: Calls a routine, stores a pointer to a global variable, and calls a routine returning a value.
- `copy_entry_fields_into_globals`: Sets a global byte to 84 and reads a global word, returning it when negative.
- `calls_helpers_conditionally_preserving_d0`: Loads a word from a global variable into d0, increments it, and calls a routine, then if the word in d2 is nonzero it calls another routine.
- `transfers_amount_to_counter_record`: Reads the 16-bit field at offset 24 of a looked-up record and, when it is nonzero, stores 86 into a global variable, sets another global to 1 and clears a third.
- `sets_counter_and_adjusts_fields`: Stores 86 into a global variable, sets another global to 1 and clears a further global, then copies the low word of d0 into a global and calls a routine.
- `transfers_amount_to_second_counter`: Reads the 16-bit field at offset 26 of a looked-up record and, when it is nonzero, stores 87 into a global variable, sets another global to 2 and clears a third.
- `sets_counter_and_adjusts_fields_alt`: Stores 87 into a global variable, sets another global to 2 and clears a further global, then copies the low word of d0 into a global and calls a routine.
- `draws_entry_and_updates_flags`: Reads a 16-bit index from a table entry selected by the word in d2, and when that index is nonzero and the matching record lacks bit 5 of its flag byte, sets bit 1 of a byte in the entry at offset 3 and then either copies a rectangle or calls helpers depending on a comparison with 10.
- `computes_offset_and_calls_helpers`: Computes a table index from d2 minus a global word minus 9, calls three routines in sequence while preserving the stack, and returns with the saved registers restored.
- `initializes_pointer_globals`: Stores the result of a call into a global longword, adds 274 to it, and stores the address of a global variable plus 40 into a second global pointer.
- `fills_words_from_counter_loop`: Computes a slot address from the word in d0 minus a global variable, loads two words from it into globals, and when a record index is nonzero copies those words into a buffer at a0 through a loop of up to nine iterations, then runs a further loop that decrements the counter in d7 until it reaches minus one.
- `copies_words_until_limit`: Loops calling three routines per pass, storing two preset 16-bit globals into the first two words at the address in a0 and calling another routine each pass.
- `loops_helpers_across_counter`: Repeatedly calls three helper routines, decrementing the word counter in d7 until it reaches minus one, then restores the saved registers and returns.
- `copies_points_into_buffer_loop`: Computes a slot address from the word in d0 minus a global variable, loads two words from it into globals, and when a record index is nonzero stores those words into a buffer at a0 in a loop of up to nine iterations, passing a computed value to a helper each pass, then runs a counter loop that decrements the word in d7 until it reaches minus one.
- `stores_point_pair_and_loops`: Loops calling routines and storing two preset 16-bit globals into the first two words at the address in a0 each pass, passing a stored longword to another routine.
- `loops_three_calls_decrementing_counter`: Reads a byte from a global variable into d0, makes three calls, decrements the 16-bit counter in d7 and repeats the loop until the counter reaches -1.
- `store_word_from_structure`: Saves d0, d1 and a0, loads a fixed address into a0 and doubles a global word into d1, calls a helper that reads a word from the structure pointed to by a global longword, and stores that word at the computed offset from a0 before restoring the saved registers and returning.
- `updates_display_values_and_marks`: Calls routines to fetch a pointer and reads two words and a longword from a structure pointed to by a result.
- `scan_table_flag_bits`: Calls another routine to obtain a pointer, then sets the high bit of several status bytes in the structure it addresses based on global flags and bitmask tests.
- `initializes_layout_pointer_and_globals`: Saves d0, d1, a1 and a2, stores the pointer in a0 into a global variable, clears a global to 65535, sets two other globals to 344 and to the address of a global, and loads a global word from a table indexed by a global counter before calling helpers.
- `gathers_records_and_blits_rectangle`: Calls a helper eight times, looks up a structure through the result plus one, and checks the word at offset 316 and the word at offset 330 against zero, calling further routines when each is nonzero.
- `clears_global_word_and_calls_helpers`: Clears a global word, loads 44 into d0, calls one routine, then calls a second routine and returns.
- `clears_flag_and_calls_three_routines`: Sets the byte at a global address to zero and then calls three routines in sequence before returning.
- `sets_global_byte_from_bit_test`: Calls a routine, then tests bit (high byte of d0) in a global word; when that bit is set it stores the high byte of d0 into a global byte and calls a routine that releases a linked slot.
- `scan_slots_build_mask`: Loops over a global array of slot flags starting at index 1, and for each active slot other than the one held in d7 calls a routine to fetch a record, then tests bits and fields of that record and sets the corresponding bit of a result mask.
- `scan_slots_build_mask_alt`: Performs the same slot scan as another routine, but compares the record's field against a different constant multiplier, so the bit it sets in the result mask depends on a different threshold.
- `set_shift_flag_writes_nine`: Saves registers, sets a flag byte in a global variable, and loads the address of a hardware-adjacent location into a0.
- `write_nine_bytes_sequence`: Writes the byte 9 to the address in a0 and advances a0, increments d7 and repeats while d7 is below 7, then restores saved registers and returns.
- `renders_multi_field_text_block`: Derives a 60-byte record offset from a global byte, stores the negated low bit of a flag into a global variable, and loops over ten bit positions of a 16-bit mask, calling two routines for each set bit.
- `conditionally_calls_helpers_twice`: Tests whether the low bit of a global byte is set or a second global byte equals 6, and if so clears a local flag, makes a call, and calls a second helper only when that flag is set.
- `draws_text_block_with_markers`: Computes a row offset from a 16-bit global counter, stores the word 2 and the word 17 into a global slot, and calls many routines to draw text lines, conditionally drawing sub-blocks based on bytes in a table entry.
- `sets_slot_and_writes_byte_through_pointer`: Calls two routines, stores the word 2 into a global longword slot, writes the byte 41 through the pointer in a0, then calls five routines in sequence.
- `conditionally_reinitialises_display_state`: Tests a global byte and, when it is non-zero, calls a routine, copies a global word into another global word, and calls another routine.
- `conditionally_negates_low_byte`: Tests a global byte and, when it is nonzero, negates the low byte of d0; the subsequent return leaves the routine in either case.
- `dispatches_table_record_to_blit_setup`: Takes the low byte of d0, and when it is nonzero it looks up a 6-byte record in a table, skipping the record when its word at offset 4 is zero.
- `conditionally_calls_fixed_helper`: Tests a global byte and, when it is zero, branches; otherwise it tests bit 3 of a byte at offset 32 of the structure in a1 and, when that bit is clear, loads 56 into d0 and calls a helper, then jumps to a later address.
- `copies_state_and_blits_rectangle`: Copies a longword and a word from the buffer at a2 into two global variables, makes six calls, and copies two global values into other globals, including a stored word from d6.
- `saves_byte_calls_helper`: Pushes the word in d0 onto the stack, loads a byte from a global variable into d0, calls a helper routine, then restores the saved word into d0 and returns.
- `sets_global_byte_and_branches`: Stores the value 255 into a global byte, then branches unconditionally to a later point inside another routine.
- `reads_slot_flags_and_copies_entry`: Clears a global byte, reads a value through the table-value routine, and when its low byte is neither zero nor minus one computes a record index from it and a global, copies the record's 16-bit value and flag bytes into globals, sets a global to 54, and returns the value read through another lookup.
- `copies_byte_and_conditionally_blits`: Stores a word from the buffer at a2 into a global, sets a global to 54 and clears two others, then stores d0 into a further global and calls a routine that returns a word.
- `saves_byte_calls_helper_copy`: Pushes the word in d0 onto the stack, loads a byte from a global variable into d0, calls a helper routine, then restores the saved word into d0 and returns.
- `copy_words_then_call_handlers`: Copies two words from fixed absolute addresses into a global area, calls five routines and sets a global byte to 2, then reads words at offsets 42 and 46 from a structure returned by a call and writes 75 to two bytes through pointer registers before returning.
- `calls_three_routines_in_sequence_4`: Calls one routine, loads 55 into d0, calls another routine and then a third routine, and returns.
- `swaps_pair_clears_flag`: Clears a global byte flag to zero, calls four other routines with one of them after swapping two longword globals and copying the first into a third, then inverts the flag byte and makes three more calls before returning.
- `dispatch_sprite_blit_state`: Reads a structure's fields and per-slot table values, then either copies a record into a global buffer and calls a series of helpers when the slot is active, or calls a sequence of helpers that test status bits and flags before returning a 16-bit result in d0.
- `copies_record_when_slot_nonzero`: Clears d0, reads the byte at offset 2 of the structure in a1 into d0, and when that value is nonzero branches to another routine.
- `update_record_and_flags`: Computes a slot offset from d0 and, when the slot's word at offset 4 is nonzero, conditionally copies a six-byte record from a global area into a structure through a pointer, then branches through a series of flag and field tests in a4 and a2 before calling several other routines and returning a 16-bit value in d0.
- `conditionally_calls_fixed_routine_ii`: Tests a global byte and, unless it is zero, tests bit 3 of the byte at offset 32 of the structure in a4; if both checks pass it loads 56 into d0 and calls one routine, then branches forward.
- `compares_entry_fields_and_stores_record`: When the byte at the start of the structure in a2 equals 1, it copies a six-byte value into a global longword and word, tests bits in a word at offset 34 and bytes at offsets 1, 3, 5, 6 and 7 of the structures in a0 and a1 to pick a branch, and on success writes the saved six-byte value back to memory at the address in a0 and performs a series of calls.
- `tests_bit_and_calls_fixed_routine`: Reads a byte at offset 3 of the structure in a0 as a bit number and tests the bit at that position in a word at offset 34 of the structure in a1.
- `conditionally_call_fixed_routine`: Reads a byte at offset 1 of the structure in a0 and a byte at offset 4 of the structure in a1, then tests the bit of the second byte selected by the first.
- `conditionally_calls_fixed_routine_with`: Loads a byte at offset 5 of the structure in a1 into d0 and branches away when it is zero; otherwise it computes 2 minus the byte at offset 6 of a0 and, when that is not greater than d0, loads 37 into d0, calls a routine, and branches to a later point in another routine.
- `tests_byte_then_calls_subroutine`: Reads the byte at offset 6 from the structure in a1 and branches out early when it is zero, otherwise it compares it against a value computed from the byte at offset 7 of the structure in a0.
- `fills_slot_or_calls_routines`: When a global flag byte is zero it makes several calls and selects one of two helper calls based on a global word, then returns that word.
- `gathers_record_fields_and_flags`: Loads the address of a global structure into a0, reads a global byte and a global word into registers, and calls several helpers.
- `preserves_word_around_calls`: Calls one routine, then another, saves the 16-bit value in d0 on the stack, reads a byte from a global variable into d0 and calls a further routine, then restores d0 from the stack and returns it.
- `conditionally_calls_and_branches`: Tests a global byte and, when it is zero, branches to a later point in the code; otherwise it reads a global byte into d0, loads a global word into d2, loads a global address into a0, calls one routine and then branches to the same later point.
- `updates_slot_record_with_accumulated`: Reads a value from a called routine and, when the record at the computed slot in a table has a zero word at offset 4, stores the global value and the counter word into it and adds a scaled product to a longword in the structure.
- `adds_scaled_value_and_clears_word`: Adds the low byte of d3 to the byte at the address in a2, multiplies d3 by the word at offset 38 of the structure in a1 and adds the product to the longword at offset 270 of the structure in a0.
- `copies_state_and_accumulates_product`: Copies a longword and a word from the structure in a3 into the structure in a2, adds the byte at a3 multiplied by a word from the structure in a1 into a longword at offset 270 of the structure in a0, clears the byte at a3 and the word at offset 4 of a3, then branches to a shared tail that calls other code.
- `swaps_records_and_accumulates_word`: Saves a longword and a word from the structure in a2 into globals, copies the structure in a3 into a2 and accumulates the byte at a3 times a word from a1 into a longword at offset 270 of a0, then restores the saved values into a3 and makes six calls before reading a global and returning the result of a call.
- `calls_helpers_and_preserves_word`: Calls two routines, then tests a word at offset 4 of the structure in a3 and, when it is zero, calls two more routines.
- `updates_slot_record_with_accumulated_2`: Reads a value from a called routine and, when the record in a table at the slot computed from a global counter has a zero word at offset 4, stores the accumulated global value into that record.
- `tests_bit_then_conditionally_updates`: Reads a value through a helper and tests one bit of a global bitmask indexed by the high byte of that value; when the bit is set it compares the high byte against a global byte and calls further routines conditionally.
- `allocates_or_merges_record_entry`: Uses a structure pointer and a table of 24 six-byte entries.
- `copies_record_and_advances_source`: Copies a longword and a word from one record to another, sets the destination byte at offset 4 to zero, subtracts a byte from the source byte and adds the same value to the destination byte.
- `advances_record_pointer_until_empty`: Copies the pointer in a3 into a4 and loops up to 24 times, advancing a4 by six bytes each time until the word at offset 4 of the current record is zero.
- `copies_record_clears_counter`: Copies a longword and a word from the source pointer in a2 to the destination pointer in a4, decrements the byte at the source pointer, and sets a flag byte at the destination.
- `clears_entry_and_subtracts_fields`: Calls a routine to get a structure pointer and indexes a six-byte entry.
- `subtracts_record_fields_from_global`: Subtracts byte fields of one record from byte and word fields of a second record, reduces a longword at offset 270 by a word field, and clears a word at offset 4 of an address held in a2.
- `subtracts_field_restores_registers`: Calls a routine, subtracts a word field taken from the record in a1 from a longword field at offset 270 of the record in a0, clears a word at offset 4 of the structure in a2, then calls a helper and restores registers before returning.
- `adds_record_fields_into_global`: Stores a longword and a word from a source record into a six-byte slot, sets a flag byte to 1 and adds byte and word fields into position and counter fields of a global record.
- `adds_field_decrements_counter`: Calls a routine, adds a word field from the record in a1 to a longword field at offset 270 of the record in a0, decrements the byte at a4 and clears two bytes after it when the result is zero or the old value was not positive.
- `adds_byte_deltas_to_record_fields`: Adds signed and unsigned byte fields from the record in a1 into word fields of the record in a0: two position words at offsets 206 and 212 receive signed deltas, and words at offsets 46 and 126 receive unsigned deltas indexed by a signed byte selector scaled by eight, when nonzero.
- `subtracts_byte_deltas_from_record_fields`: Subtracts unsigned byte fields from the record in a1 from word fields of the record in a0: words at offsets 206 and 212 are reduced by unsigned deltas, while words at offsets 46 and 126 are reduced by unsigned deltas indexed by a signed byte selector scaled by eight, when nonzero.
- `stores_blitter_state_and_slot`: Saves d0, d1, d2, d5, d6, d7 and a1 with a0 and d0 on the stack, then stores a longword into a global variable, copies two words from globals into a fixed buffer and writes a byte at offset 16 of that buffer.
- `calls_three_setup_routines`: Calls three routines in sequence and returns the value in d0 unchanged.
- `copies_flag_byte_into_slot`: Saves d0 and a0 on the stack, reads a global byte into d0 and calls a routine, then calls a routine and stores the resulting byte into a global variable.
- `blits_masked_rectangle_and_resets_state`: Reads a byte from the structure in a0 and a word from a table entry, calls routines to blit with the selected bitplane data, and calls further routines when the byte at offset 3 has bit 1 set.
- `scans_slots_setting_bits`: Walks a linked list of records whose head is in a global variable, testing a bit mask and several byte, word and longword fields in each record.
- `sets_bit_writes_word`: Sets a bit in d6 selected by d7, reads a byte from a1 offset -1 into d0, and calls a helper routine.
- `updates_table_record_and_flags`: Converts the low byte of the value passed in d0 into a table index, calls several helper routines to look up a record and compute a slot, and when the record's type byte equals 5 calls helpers that adjust a value compared against 10.
- `compute_record_and_blit_mask`: Reads a global byte into d0, sets d3 to 1 and calls two routines, then if d0 equals 5 it moves d2 into d0 and, when d0 is below 10, calls one routine, otherwise it branches past that call.
- `subtracts_offset_then_calls_three`: Subtracts a word global from d0, then subtracts 9 from the result, calls two routines, moves d2 into d0, calls another routine, and restores d0, d2 and a0 from the stack before returning.
- `loads_table_pair_into_globals`: Derives a table index from d0 (reducing it by a global variable when it is 10 or more), then copies a 16-bit pair from a table into two global words.
- `updates_flag_bits_from_table_entry`: Looks up a 16-bit value through a call, then sets bit 7 of bytes in the structures at a0 and a1 depending on bit 4 of a word read from a structure and on a bit test against a global longword.
- `sets_status_flags_from_results`: Calls a routine and sets bit 7 of a byte at offset 6 of a structure when the returned word is zero.
- `stores_handler_and_calls_setup`: Compares the global byte at 1330 with 2 and branches to another routine when it is not less than 2.
- `conditionally_calls_follow_up_routines`: Loads two global words into d0 and d1, then calls one routine and branches on the result to call a second routine, or else call the second routine and then a third routine.
- `dispatches_on_status_byte`: Compares a global byte against 2 and, when it is not less than 2, stores a constant longword into a global variable at a fixed address and loads the address of a table into a0, then calls a routine; otherwise it branches forward to a later point and returns.
- `loads_three_words_and_dispatches`: Copies two global words into d0 and d1, calls a routine, and if the result is zero it calls two more routines; it then returns.
- `dispatch_on_threshold_value`: Compares a global byte against 2 and, if it is not below 2, stores a fixed longword into a global variable and calls a routine at a fixed address before branching to a shared exit.
- `dispatch_conditional_position_calls`: Loads two global words into d0 and d1, calls a routine and returns if that result is nonzero, otherwise calls a second routine and a third routine, and calls a fourth routine only when a conditional test passes; it returns afterwards.
- `scans_bits_and_updates_flag`: Reads a byte from a structure, loops from bit 3 down to bit 0 testing each bit, and on the first set bit stores its index into a global byte, calls a routine, and copies a global word into another global word.
- `decrements_counter_and_branches`: Decrements the low word of d7 and branches back to an earlier instruction while it has not reached minus one, otherwise it branches forward to a later address.
- `clears_flag_and_copies_timer_value`: Calls one routine, and if a global word is nonzero clears a global byte, copies a global word into another global word, and calls one more routine; otherwise it returns after the first call.
- `conditionally_calls_fixed_routine_then`: Tests a global byte, and when it is nonzero it loads 39 into d0 and calls a routine, then branches to a later point in another routine.
- `calls_two_routines_indirectly`: Calls one routine, loads the address of a table into a0, calls a second routine, and returns.
- `store_flag_nibble_from_word`: Clears d0, then if a fixed word global is not 6 it leaves d0 at zero and otherwise loads 15 into d0, and branches to a common exit.
- `stores_flag_nibble_to_global_and_calls`: Calls two routines, then tests bit 3 of a register; when it is clear it calls another routine, and when it is set it stores 15 into a global byte if a word global is nonzero, otherwise it stores the low byte of d0, then calls two routines before returning.
- `stores_flag_nibble_and_calls`: Tests a word global; when it is nonzero, it loads 15 into d0.
- `copies_record_fields_and_cycles_bits`: Calls three routines, then copies a byte and a word from the buffer in a0 into a structure, merges nine longword entries from the buffer with a stored table, then toggles bit 1 of a byte and loops over a global counter, calling routines until the value changes; finally it jumps to a routine with a table address in a0.
- `merges_buffer_entries_into_record`: Copies a byte and a word from the buffer in a0 into a structure, merges nine longword entries from the buffer with a stored table where a value of minus one selects the stored entry, copies two more longwords, and then toggles a flag bit and loops on a global counter before returning.
- `initializes_pointer_list_entries`: When a global flag is zero, it stores two derived values into global variables, then fills 24 consecutive longword entries starting at the buffer in a0, each receiving a value from a helper and a stepped counter.
- `loads_words_calls_index`: Saves registers and loads three global words into d0, d1 and d2 when a global byte is below 2.
- `compute_shifted_column_index`: Calls a helper that computes a shifted column index from d0 and d1 after shifting both right by 9.
- `copy_state_into_structure`: Calls two routines, stores a fixed longword into a global variable, then copies a byte from the buffer in a0 into offset 54 of the structure in d6 (or stores 1 there when a global flag is set) and copies the next byte to offset 55.
- `compares_stores_pointer_conditionally`: Compares two word-sized global values, each offset by a constant and shifted right, against two other globals and then calls another routine on either branch of the comparison.
- `loads_address_and_calls`: Loads the address 23408 (decimal, from the constant at 23346) into a0, calls another routine with it, and then returns.
- `dispatches_through_table_pointer`: Calls two routines, then calls the code address held in a global longword, acting as an indirect dispatch through a stored pointer.
- `check_bounds_then_call`: Compares a global word in d0 and a word in d1 against two stored global words using offset range tests, and calls a routine only when both differences fall inside their small ranges.
- `computes_four_global_coordinate_bounds`: Scales two global counter words by 16 and derives four global word values from each, offset by constants, clamping each result to a fixed lower or upper bound when it falls outside the range.
- `computes_two_sprite_bounds`: Takes a counter from a global variable, multiplies it by 16 and writes four clamped coordinate values into global variables: two offsets at 7 and 72 (with fixed caps) from the first counter, and two offsets at 40 and 105 (with fixed caps) from the second counter.
- `scans_records_for_matching_entry_2`: Loops up to 32 times over 32-byte records in a global array, testing each record's flag byte, a bit mask, and two stored words against d0 and d1, and on a match with the required flags calls several routines and jumps out of the loop; otherwise it calls one routine and returns.
- `advances_pointer_and_tests_counter_2`: Advances the address register a0 by 32 bytes, increments the word counter in d7, and repeats the loop while d7 is below 32, then calls one routine and sets d5 to a true or false byte value before restoring registers and returning.
- `resets_and_conditionally_clears_flag`: Clears a global byte, sets a flag local to the routine, and calls a helper to test it.
- `call_then_restore_saved_registers`: Calls one routine, sets d7 to 255 or 0 from the result's flag, tests d7, then restores d0 and d1 and d7 from the stack and returns.
- `calls_handlers_for_nonzero_table_entry`: Saves d0 and a0, reads a byte from a table indexed by d0, and when it is nonzero calls two routines and returns with d0 unchanged after restoring the saved registers; for a zero entry it skips the calls and restores d0 and a0, then sets the status flags from d0 and returns.
- `clears_and_tests_flag_byte`: Sets a local flag to false and a byte value to 1, calls another routine, and if the flag is still clear, calls another routine and tests a global byte against a bit mask; when the test passes it sets the global byte to 255 and then clears it to zero.
- `call_then_conditionally_call_pair`: Calls a routine, and if its result sets the zero condition it calls a list-walking routine and then a routine that returns immediately when a global word is 1; it then returns.
- `scans_structure_list_and_updates`: Walks a list of 32-byte records through the pointer in a0 while the counter in d7 is below 32, testing flag bits and two 16-bit fields in each active record against values in d0 and d1.
- `conditionally_updates_structure_state`: Tests bit 4 of the byte at offset 2 from a0 and, when it is clear, calls two routines, combines a status bit from a table-derived structure with the previous result, and clears a global variable and writes d7 to a global word when the combined bit is set; otherwise it calls a routine that handles the byte at a0 and then calls two further routines.
- `stores_counter_and_clears_flag_then`: Stores d7 into a word global, clears a global byte, calls two routines, loads the address of a table into a0, calls another routine, sets d5 to -1, and branches to a later point in another routine.
- `dispatches_three_table_helpers`: Takes the low byte of d0 and, when it is below 200, calls a helper and increments a byte at offset 19 of the structure in a0 if the value equals 200.
- `advances_pointer_and_tests_counter`: Adds 32 to the pointer in a0 and increments d7 by one, looping back while d7 is below 32.
- `conditionally_update_global_table`: Returns immediately when a global word is 1.
- `copies_table_entries_into_globals`: Calls several helpers, stores the constant 103 and a handler address into a global table, and when a global index is nonzero either calls a helper or loads a global variable from a table indexed by that index, then stores 255 into another global.
- `stores_index_and_sentinel_globals`: Loads a byte from a table indexed by the low word of d0 into a global, stores a second byte from a table offset by that index into another global, saves the word in d0 to a global, stores 255 into a global variable, then calls two routines and returns.
- `tests_bit_and_clears_global_byte`: Clears a local flag and sets a second local to true, calls a routine, then if the first flag is still clear and a bit of a global byte selected by a range check is zero, it sets a global byte to 255 and calls a routine; otherwise it clears the global byte and returns.
- `initializes_layout_record_and_global`: Stores a0 into a global pointer, clears a global longword and a second global longword, sets a word at a fixed address to 24, clears a word at offset 18 of a structure, and calls a routine before restoring d0 and a0.
- `initializes_globals_and_marks_slots`: Clears two global longwords, then walks a 6-entry global byte array and writes a running index into the array slot named by each nonzero byte, and finally calls four routines and one more with a0 set.
- `clears_table_and_global_byte`: Walks backward through a 12-byte region from a global address, storing the down-counting value into a second table at the index given by each nonzero byte, then runs three calls, clears a global byte to zero, runs three more calls and tail-jumps to another routine.
- `calls_setup_sequence_and_helper`: Calls four routines in order, loads the address of a fixed structure into a0, calls another routine with it, and returns.
- `jumps_to_flag_clear_sequence`: Clears a global byte to zero between two groups of three calls and then tail-jumps to another routine as its final action.
- `loops_over_twelve_flags_with_blits`: Iterates twelve times over a global byte array, reading a word per iteration.
- `calls_handler_through_structure`: Calls one routine, then calls a handler through the code pointer at offset 30 of a global structure when that structure and pointer are non-null, and finally jumps to a further routine.
- `loops_while_reading_slot_word`: Calls a routine and, when the low byte of the result is nonzero, stores it into a global word and reads a global byte at that index; if that byte is nonzero it clears it, calls two routines, stores a second result into a global word, and repeats calling a routine until that word is zero.
- `copies_globals_and_swaps_pointers`: Copies two words from fixed global addresses into a fixed buffer, stores a byte from a table indexed by a global word into another buffer offset, then swaps two longword globals and inverts a global byte flag before calling routines.
- `copies_words_and_jumps_onward`: Copies two 16-bit values from global words into the memory at a0 and at a0 plus 2, then jumps to a later routine.
- `swaps_byte_with_global_word`: Reads a byte from a global variable and, when it is nonzero, sign-extends it into a 16-bit global word, exchanges that word with the byte at an index into a table, then calls four other routines.
- `exchanges_byte_and_stores_extended_word`: Reads the byte at the address in a0, writes the byte in d1 into that address, sign-extends the old byte into a 16-bit global word, then calls four routines and returns.
- `swaps_byte_with_global_word_2`: Loads a global word into d0, reads a global byte into d6, and when that byte is nonzero it stores it in a global word, exchanges the value of a global word with the table entry selected by it, and then makes four calls to other routines.
- `resets_flag_and_sets_mode_constants`: Clears a global word, takes a value from a helper and, when a global byte is zero, passes it to another helper.
- `loads_constant_and_branches`: Loads the constant 19 into d0 and branches to a point inside another routine.
- `sets_mode_and_calls_helpers`: Loads the constant 18 into d0 and calls a helper, then calls two more helpers and stores the constant 3 into a global word.
- `calls_four_routines_then_clears_global`: Reads a byte from a global location into d0, calls a sequence of routines, clears a global byte to zero, and makes a final tail jump, returning the longword result of one of the calls.
- `sets_mode_and_conditionally_clears_flag`: Writes the constant 3 to a global word, calls four routines and a fifth with a0 set to a fixed address, then tests a global pointer and a longword at offset 38 of its structure; when both are nonzero it sets a global byte to 255 and calls three further routines.
- `sets_status_bits_per_flag`: Calls a routine to get a structure pointer and sets bit 7 of a byte at offset 12 when a global byte is non-zero.
- `sets_flag_bit_after_call`: Calls a helper routine, then sets bit 7 of the byte at offset 6 of the structure in a0 if the zero flag is set, and calls a second helper before returning the word result of the first call.
- `waits_then_calls_init_sequence`: Calls two helpers, then tests a global byte and branches to skip one call when it is nonzero, otherwise calling another helper before a branch to the end.
- `clears_flag_and_calls_three_routines_2`: Clears a global byte, loads a pointer into a0 and copies two words from a global block into two global words, then stores a third value from the block into a global word.
- `program`: Calls an initialization routine, a main routine, and an exit routine before returning.
- `tests_flags_and_calls_fixed_routine`: Tests the byte at a fixed global address and, when it is zero, compares a global word with 8; if the byte is zero and the word is below 8 it loads the constant 176 into d0 and calls a routine, then returns.
- `clear_flags_and_update_entries`: Clears a global byte array by walking six bytes and calling a helper for each nonzero entry, then computes a delay count from a word in the structure and spins on it, and for each active entry of a structure at a0 adjusts counters at offsets 26, 106 and 110 or calls helpers, finally zeroing six global bytes and returning.
- `compute_negated_index_and_branch`: Subtracts 7 from d7, negates it, and decrements it by one; if the result is negative it branches forward.
- `loads_constant_and_branches_into_routine`: Stores the constant 24 in d0 and calls a routine, then loads 95 into d7 and clears the word at a fixed global address.
- `updates_entries_and_counters`: Runs a loop of six passes over a byte table, and for each nonzero entry reads a structure pointer and adjusts fields in that structure, using calls to other routines to test status bits and to update a global byte and global counters.
- `updates_structure_entries_in_loop`: Loops six times over a byte table, and for each nonzero entry reads a structure pointer, decrements a 16-bit field at offset 26 and subtracts 250 from a 32-bit field at offset 270, then after the loop zeroes five longword globals and returns.
- `clear_flag_and_decrement_counter`: Decrements a 16-bit field at offset 26 and subtracts 250 from a 32-bit field at offset 270 of the structure in a1, then scans a byte string through a2, calling helper routines and clearing a run of longword globals when it reaches the end of the string.
- `run_flagged_record_update_loop`: Sets a bit in a flags longword and then repeatedly calls several helper routines in a do-while loop that scans a table of 6-byte records, testing a bit in a structure at offset 4 and a byte from a second structure, and updating a 16-bit field at offset 20 of the matched structure before the loop exits once the returned value is zero.
- `tests_structure_bit_and_calls_helper`: Reads a byte at offset 4 of the structure in a0 and a byte from a second structure, tests the bit selected by that byte, and when the bit is clear sets d0 to 21 and calls a fixed routine, then branches to a later instruction in all cases.
- `tests_bit_and_calls_helper`: Reads a longword from a field of the structure in a0 indexed by a byte value at offset 24 of a second structure and tests the bit numbered by a byte at offset 23 of that second structure.
- `scan_records_set_flag_bit`: Compares a 16-bit field at offset 20 of the structure in a0 against a value read from a table indexed by a byte at offset 23 of a second structure, and when the field is not negative it stores the result back and sets a bit in a flags longword selected by a byte at offset 24, looping through a 6-byte-stride record table while the scan result is nonzero, and otherwise it returns after the loop ends.
- `scans_records_and_marks_flag_bits`: Stores a word argument into a field of the structure in a0 and calls helpers that compute a comparison value from two fields.
- `loop_until_flag_clears`: Sets a bit in a flags longword, then repeatedly calls helper routines inside a do-while loop that scans a 6-byte-stride table of records and clears a 16-bit field in a structure, looping back to the start while a derived value stays non-negative.
- `loads_record_values_into_globals`: Calls two routines, looks up a 4-byte entry in a table from the result, and stores its two 16-bit fields into two global words.
- `shift_index_and_call_helpers`: Calls several helpers, then calls one routine with the address of a global-table entry computed from a word argument minus one and the record size of 60 bytes, and calls two more routines before returning.
- `computes_percentage_and_stores_addresses`: Initializes a value to zero, and when a byte in a structure is non-zero computes a percentage of a sum of three words minus another word, clamping to a minimum of 1 and to the remainder.
- `stores_fixed_longwords_through_pointers`: Calls a routine with d0 set from d5, stores the address of a global variable into two global pointers, calls another routine, and returns with the saved stack value restored.
- `conditionally_calls_then_branches_onward`: Tests a global byte and, when it is zero, branches to a later point.
- `selects_grid_parameters_and_branches`: Compares a global byte against 2 and, when it is not lower, branches to a later point in the routine.
- `draws_nested_grid_then_swaps_buffers`: Runs a nested loop of nine by eight iterations calling a helper with fixed coordinate constants, then calls several routines, swaps two longword globals, copies one into a third, and inverts a global byte before calling three more routines.
- `dispatch_flag_then_set_byte`: Tests the global byte at address 230 and, if it is nonzero, calls one routine and loads a0 with a global area address before calling another routine.
- `swaps_globals_and_calls_routines`: Calls two routines, swaps two longword globals and copies one into a third global, then inverts a global byte and calls three more routines before returning.
- `sets_high_bit_conditionally`: If a global flag byte is nonzero, sets the top bit of the byte at offset 12 in the structure pointed to by a0, otherwise it returns without changing it.
- `toggles_bits_and_updates_state_flags`: Reads a 16-bit global word and calls a helper, then XORs the result with the original value to find changed bits.
- `toggles_flag_and_conditionally_reads`: Calls a routine, then calls another routine when bit 1 of a global byte is clear, and if bit 4 of register d7 is set it inverts bit 1 of that global byte.
- `conditionally_processes_two_global_bytes`: Loads one global byte into d0 and calls a routine, then tests bit 0 of another global byte and, if it is clear, loads a different global byte into d0 and calls the same routine again before returning.
- `calls_helpers_then_branches_on_result`: Loads the constant 60 into d0 and calls a routine, then tests the byte result and branches to a later point when it is zero, otherwise calling two routines before a tail jump.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `initializes_slot_and_dispatches_helpers`: Loads the constant 34 into d0 and calls a helper, stores the returned byte into a global byte, writes the constant 96 into a global word and points a0 at a fixed address before calling a helper.
- `conditionally_dispatches_clearing_calls`: Checks a global byte and, when it is zero, loads the constant 95 into d0 and calls another routine, then if that routine returns zero it loads a global byte into d0 and calls a second routine.
- `flushes_flagged_global_entries`: Calls a routine when a global byte is at least two, then loops over six global byte flags, calling a routine with a paired global word and two more routines for each set flag, and finally stores a global word into a pointed-to location before calling three routines.
- `initializes_buffers_and_dispatches_loads`: Calls a helper to obtain a pointer, tests a word at that location and, when it is zero, scans a 9-entry table of 39-byte records for a nonzero byte.
- `dispatches_fixed_address_sequence`: Runs three helper routines, stores a constant into a global byte, then calls helpers again and, when a global word is nonzero and the byte at offset 17 of a structure has bit 0 set, stores a fixed address into a global longword after each helper call inside an endless loop.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `clear_counters_and_swap_buffers`: Clears a global longword, sets a global word to 31, calls several helper routines, then swaps two longword globals, copies the first into a third global and inverts a global byte.
- `blits_then_loops_clearing_flags`: Calls a helper routine, stores 31 into a global word, and then runs a loop of ten iterations that calls helper routines and increments a counter in d3 each time.
- `conditionally_blit_text_and_count`: If the byte in d0 is nonzero and a table entry for the row index is set, it writes 26 and 31 to two blitter-control globals, stores the byte into another global, and calls helper routines to draw the line.
- `clips_and_blits_bounded_rectangle`: Takes the high byte of d0 and, when it is non-zero, calls a helper routine to clip a rectangle, decrements the index, and compares a byte in a table with zero.
- `processes_nonzero_high_byte_of_d0`: When the high byte of d0 is nonzero, stores it in a global and sets a table entry and a second global to fixed constants.
- `forwards_call_then_returns`: Calls the routine at a fixed absolute address and then returns to its caller without changing any registers itself.
- `blits_rectangle_from_high_byte`: Takes the high byte of d0 and, when it is non-zero, calls a helper routine with that value, clears one global longword, sets another to 65535, calls helpers, and resets the second global to zero.
- `selects_flag_masked_mode_value`: Starts d0 at 255, compares a global byte with 2, and branches; when a second global byte is nonzero it clears bits 4 and 5 of d0 and then jumps past the code that would set them, ending with d0 as the selected value.
- `masks_bits_and_swaps_longwords`: Clears bits 4 and 5 of d0 when the corresponding bits of a global byte are zero, and stores the result into that global byte.
- `calls_helpers_on_fixed_coordinates`: Loads the constants 224, 125 and 30 into d0, d1 and d3, then calls a helper and branches to a later point when its result is nonzero.
- `toggles_word_bit_from_byte_table`: When the high byte of d0 is nonzero and the bit it selects in a global byte is set, it looks up a byte in a table, XORs a global word with a bit chosen by that table byte, and calls two routines.
- `clears_scroll_counter_for_flagged_index`: When the high byte of d0 is nonzero and the bit selected by that byte in a global byte is set, it calls a routine, clears one global word, sets another global word to 65535, calls three routines and then clears the first global word.
- `selects_source_address_by_bit`: Tests bit 6 of the 32-bit value in d0; if that bit is clear it branches forward to the next instruction group, otherwise it loads the address 51945 into a0 and branches past the following code.
- `blits_fixed_region_clears_global`: Loads the address of a fixed structure into a0, places 232 in d0, calls a helper routine to blit that region, and then stores zero into a global word.
- `draws_bit_mask_column_sequence`: Reads a global byte as a bit mask and, for each of five bit positions, stores either 28 or 31 into a global word depending on whether that bit is set, calling routines for each bit and for the clipped text blit.
- `blits_bit_selected_source_columns`: Stores 31 into a global word, copies a1 into a0, sets d0 to 65 and calls a routine.
- `tests_bit_and_loads_pointer`: Tests bit 0 of d2 and, when the bit is clear, loads a fixed absolute address into a0 and branches forward; when the bit is set it skips the load and branches to the same forward point.
- `loops_blitting_offset_columns`: Loads a fixed structure address into a0 with 232 in d0 and calls a helper routine to blit it, then adds 7 to d1 and adds 1 to d7.
- `conditionally_stores_byte_and_calls`: If a flag derived from the low two bits of the byte at offset 2 from a0 is negative, it stores the byte at a0 into a global word and then calls two routines in sequence, the first storing its result into one global variable and the second into another, and then returns.
- `stores_two_computed_bytes`: Calls one routine twice, storing each returned byte into a separate global variable, and then returns.
- `saves_and_calls_helper_twice`: Saves d0 on the stack, loads the byte at absolute address 34 into d0 and calls a fixed routine, then loads the byte at absolute address 35 into d0 and calls the same routine again before restoring d0 and returning.
- `clears_forty_eight_words`: Zeroes 72 consecutive 16-bit words starting at the address returned by another routine, then calls another routine and returns the value in d0.
- `clears_global_block_and_flags`: Zeroes three global variables, calls a helper to obtain a value, and then, when a global byte is zero, calls two further helpers and stores the result of another helper into a global variable.
- `loads_constant_and_calls`: Loads the constant 119 into d0, calls one routine, then returns.
- `sequences_flag_and_clears_global`: Makes two calls, conditionally makes two more when a global byte is zero, then makes a call that returns a value, makes three further calls, clears a global byte to zero and makes three final calls, returning the saved value.
- `guards_pointer_then_sets_global`: Calls a helper routine, then if a global pointer is nonzero and the longword at offset 38 of its structure is nonzero, stores 255 into a global byte and calls three further routines.
- `saves_a0_then_guards_pointer`: Calls two helper routines and keeps the result of the first one, then if a global pointer is nonzero and the longword at offset 38 of its structure is nonzero, stores 255 into a global byte and calls three further routines before returning the saved result.
- `refresh_globals_and_process_dirty_table`: Recomputes a 16-bit dirty mask by comparing eight words of a global table against a shadow copy, copies the changed shadow words back, and then calls a routine for each set bit whose table entry has its top bit set.
- `clears_flag_and_copies_table_block`: Clears a global byte when it is set and calls three routines.
- `sets_status_bits_in_records`: Compares a global byte with zero and calls two helper routines, then conditionally sets bit 7 of the byte at offset 8 of a0 and bit 7 of the bytes at offsets 6 and 12 of a second structure.
- `conditionally_clears_global_flags`: Clears a flag that controls a branch, then if it is set and a global byte is nonzero it calls a routine and, when the result is nonzero, calls several more routines.
- `calls_helpers_on_longword_sentinel`: Calls four helpers in sequence, then tests a global byte, and when it is zero compares a global longword against -1.
- `sets_flag_and_calls_handler`: Clears a local flag, calls a routine with d0 cleared, and if the flag was set, calls a routine with 107 in d0; it then returns.
- `initializes_slot_state_and_calls`: Stores constants 108, 344 and 24 into global variables and writes a zero byte to another global, stores the address of a global variable into a global pointer slot, and makes a series of calls to other routines.
- `calls_five_subroutines_sequentially`: Calls five other routines in sequence, with a word constant 114 loaded into d0 before the first call, and then returns.
- `initialize_globals_and_dispatch`: Stores constants into several global variables, stores a pointer into a global, calls helper routines and conditionally calls more helpers depending on a global byte and a global word, then returns.
- `calls_fixed_routine_then_branches`: Loads the word constant 114 into d0, calls a fixed routine, and then branches to a point in another routine.
- `conditionally_calls_three_subroutines`: Tests a global byte and, when it is zero, calls one other routine before calling three more in sequence, then returns.
- `tests_status_bits_sets_marker`: Reads a 16-bit field at offset 30 of the structure returned by a call and, depending on its bits, calls helper routines; when a local flag stays set and a global flag is zero, it stores 255 into a global byte.
- `tests_bit1_calls_fixed_routine`: Tests bit 1 of the byte in d0; when that bit is clear it branches forward to another part of the code, otherwise it loads 99 into d0, calls one fixed routine, and then branches to a later point in the code.
- `tests_bit10_calls_fixed_routine`: Tests bit 10 of the value in d0; when that bit is clear it branches forward to another part of the code, otherwise it loads 100 into d0, calls a fixed routine, and then branches forward to a later instruction.
- `conditionally_marks_global_slot_255`: Tests a global word and, when it is zero, sets d0 to 97 and calls a fixed routine; otherwise it branches to a later instruction.
- `conditionally_sets_global_marker`: Clears a local flag and calls a helper that tests a branch condition, and when the flag is set it calls several helpers and stores 255 into a global byte.
- `compares_word_and_calls_fixed_routine`: Compares a global word with the constant 6 and, if it is not below 6, loads 121 into d0, calls a fixed routine, and then branches to a later point; if the word is below 6 it branches directly to that later point.
- `dispatch_flag_gated_routines`: Calls a routine that returns a pointer, reads the byte it points to, then calls another routine.
- `sets_flag_and_calls_routines`: Loads the constant 5 into d0 and calls a routine, then when the result is non-negative calls a second routine.
- `stores_table_entry_and_flags`: Calls two helpers, reads a 16-bit word at offset 24 of the structure returned by the second helper into a global longword, and stores 110 into a global word.
- `stores_table_entry_with_alternate_offset`: Calls two helpers, reads a 16-bit word at offset 26 of the structure returned by the second helper into a global longword, and stores 111 into a global word.
- `loads_long_from_table`: Reads a byte through a0, shifts it left by 2 to form a table index, and returns the 32-bit longword stored at that index in a table in memory.
- `computes_slot_and_copies_point_pair`: Computes an offset from a 16-bit input and a global counter, copies a pair of 16-bit words from a table into globals, and calls helpers.
- `copies_globals_and_calls_five`: Copies one longword global into another and a second longword global into a third, then calls five other routines and returns the values held in d0 and d1.
- `saves_registers_and_calls_helpers_2`: Saves d0 through d2 and a0 on the stack, points a0 at a fixed address, loads two 16-bit globals into d0 and d1 and the constant 10 into d2.
- `saves_registers_and_scans_flags`: Saves d0-d2, d6-d7 and a0 on the stack, points a0 at a fixed address, sets d6 to 1 and d7 to 5, then tests bytes through a0 and jumps forward to a later instruction when a zero byte is found, otherwise it adds to d6 and loops.
- `builds_bitmask_and_stores_index`: Stores a byte from a global variable through the pointer at a0 minus one, then combines five consecutive global bytes into a 32-bit bitmask by shifting 1 left by each byte value, searches downward from bit 11 for the highest clear bit, and stores that index into a byte at an offset from a global variable.
- `builds_and_draws_indexed_rows`: Computes a 48-byte-stride offset from a register, stores it and fixed values into global variables, copies a null-terminated string from a table into a global buffer, calls a series of routines, then loops from 1 to 36 calling four routines per iteration before a final set of calls.
- `calls_setup_or_wait_routine`: Tests a byte in a global variable, branching when it is nonzero; otherwise it calls a routine at a fixed address and branches forward to return.
- `forwards_init_call_then_returns`: Calls the routine at a fixed absolute address, which stores a0 into a global pointer and clears two global longwords, then returns to its caller.
- `saves_a0_and_initializes_globals`: Pushes a0 onto the stack, clears a global longword, sets a global longword to a fixed pointer value, stores 24 into a word at a fixed address, and clears the word at offset 18 of the structure in a0 before calling another routine and restoring a0 from the stack.
- `runs_twelve_wait_calls`: Makes two calls, then loops twelve times calling a routine on each pass with the counter counting down to zero, and finally makes one more call before returning.
- `updates_dirty_flags_and_hooks`: Stores a set of constants into a global table, calls a sequence of helper routines, and then copies an eight-word global mask into a shadow table while marking changed words as dirty.
- `refresh_dirty_table_and_flags`: Copies a 16-bit dirty mask from a global table of eight words, comparing each word against a shadow copy and writing changed shadow words back, then sets bits in a second mask and calls a callback pointer stored in the table entry.
- `scans_records_and_updates_bitmap`: Walks 32-byte records through a pointer, reading a flag byte and testing bits of a mask for each record, and calls several routines to compute values and update state.
- `scans_records_and_masks_bitmap`: Loops over 32-byte records through a pointer, testing a flag byte and a bit in a bitmask for each record, and calls the bit-manipulation and bitmap-column routines on qualifying entries.
- `scans_bitmap_records_and_updates_state`: Walks a sequence of 32-byte records through a pointer, tests a flag byte and a bit in a bitmask for each record, and when the record qualifies it looks up a byte from a table, calls several routines in a repeating sequence, and branches back to an earlier point of this routine to continue the scan.
- `chains_four_routines_and_loops`: Calls three routines in sequence, then branches back to an earlier point of this routine to repeat the sequence.
- `classifies_record_field_and_selects_code`: Reads a byte at offset 2 of the record in a0, masks it to its low two bits and compares it with 2; for values of 2 or more it loads 17 into d2 when a particular global byte bit is clear, otherwise it selects a fallback value.
- `scans_bitmap_blocks_and_dispatches`: Walks a sequence of 32-byte records through the pointer in a0, skipping records whose flag byte is zero or whose bit in a bitmask is already set, and for qualifying records reads a 16-bit value at offset 42 and a control byte at offset 35, calling the listed routines to act on them.
- `clears_bitmap_flags_and_steps`: Loops up to a counter value, calling two helpers on each pass, and when the counter reaches -1 clears two global words and calls helpers, then if a global pointer and its structure field are nonzero, stores 255 into a global byte and calls more routines before returning.
- `stores_three_bytes_then_sequences`: Reads three consecutive global bytes into d0 in turn, calling a routine after each, then makes a further call, calls one routine, clears a global byte to zero, makes three more calls and returns the saved value.
- `calls_three_routines_then_conditionally`: Calls three other routines in sequence, then if a global pointer is nonzero and the longword at offset 38 of its structure is nonzero, calls another routine, stores 255 into a global byte, and calls three more routines.
- `copies_masked_words_and_updates_state`: Decrements a global word, advances a global longword index by 4 and resets it when the next table entry is zero, then copies eight words from a global pointer into a buffer with the top bit cleared.
- `increments_global_word`: Adds 1 to the 16-bit global variable at offset 602 and returns.
- `scans_table_entry_for_match`: Calls a routine and, when the signed value in d0 is negative, requires a global byte to equal 6 and then computes a grid position from two global words and stores it in two other global words when it lies in range; otherwise it fetches a table entry from the offset in d0, moves its values into global words, and runs a loop that repeats a call while a counter is nonzero, decrementing the counter on each pass.
- `calls_routines_and_branches_on_flag`: Calls a routine and stores its result in d7, then tests a global byte and, when it is zero, loads the constant 51 into d0 and calls another routine before branching forward to a later point.
- `tests_byte_then_calls_routine`: Tests the byte at a fixed global address and, if it is zero, loads the constant 137 into d0 and calls another routine.
- `copy_table_entry_and_step_pointer`: Clears the byte at a fixed global address, stores a0 into a global pointer, and writes 255 to a global variable.
- `scales_word_then_loops`: Sign-extends the word in d3 to a long, divides it by 4 in place, and then branches into the middle of another routine.
- `repeat_call_then_two_calls`: Calls the first routine in a loop controlled by the signed word counter in d3, decrementing it after each pass until it reaches minus one, then calls two further routines in sequence and returns.
- `dispatches_on_returned_value`: Calls a routine and tests the word it returns; when that word is non-negative it branches to another part of the code, otherwise it calls a different routine and then branches forward.
- `compares_word_and_calls`: Compares a global word with d0 and, when they differ, calls one routine and then branches forward to a later point in the code.
- `stores_address_and_sets_counter`: Calls a routine that returns a value, stores the low word of that value into a global word, passes the value to another routine, writes the constant 23 into a global longword, and calls one more routine.
- `dispatches_with_index_one`: Sets d0 to 1 and branches to a shared dispatch routine, then returns the value that routine yields.
- `dispatches_with_index_three`: Sets d0 to 3 and branches to a shared dispatch routine, then returns the value that routine yields.
- `dispatches_with_index_five`: Sets d0 to 5 and branches to a shared dispatch routine, then returns the value that routine yields.
- `dispatches_with_index_seven`: Sets d0 to 7 and branches to a shared dispatch routine, then returns the value that routine yields.
- `dispatches_and_releases_pending_slot`: Dispatches through a table of handler pointers indexed by the low nibble of d0 when that value is nonzero, and if a longword global is not -1 it calls two routines, conditionally calls two more when a global byte is nonzero, calls another routine and sets the longword global to -1.
- `decrements_timer_when_enabled`: When a global flag byte is nonzero and a 16-bit global word is not zero, subtracts 1 from that word; otherwise it leaves the word unchanged and returns.
- `advances_clamped_word_pair`: Preserves d0, and when a global flag byte is nonzero and the word at offset 590 is less than the word at offset 596 minus 19, adds 2 to the word at offset 590; then it restores d0 and returns.
- `steps_word_toward_limit`: Preserves d0, and when a global flag byte is nonzero and the 16-bit global word at offset 592 is less than the word at offset 596 minus 13, adds 1 to the word at offset 592; then it restores d0 and returns.
- `decrements_global_word_by_two`: When a global flag byte is nonzero and the 16-bit global word at offset 590 is not zero, subtracts 2 from that word; otherwise it returns unchanged.
- `waits_releases_then_runs_handler`: Calls a routine that waits for status bits to clear, calls a routine that releases a linked slot, and then tail-jumps to another routine, after which it invokes the handler at offset 30 of the global structure if that pointer is non-null.
- `scans_table_for_matching_entry`: Checks a global word in d6 as a count, computes a pair of coordinates from two global words, and when both are in range it steps through 20-byte table entries in a loop until it finds one whose first two bytes match those coordinates, otherwise it sets the result word to -1.
- `scans_table_entries_for_match`: Walks a table of 20-byte entries from a0, advancing d6 and counting d7 down, until the first two bytes match d4 and d5 or the count runs out, and then compares a global longword against 6 and makes a call that depends on the result.
- `advance_table_entry_and_scan`: Adds 20 to a0 to step past one 20-byte entry, decrements the count in d7 and loops back while it is nonzero, then pushes d0, reads a byte from a fixed absolute address into d0, and calls another routine before restoring d0 and setting d6 to 255.
- `dispatch_table_entry_increment`: When d2 equals 9, calls another routine to obtain a table base, reads a word from a table indexed by the low byte of d3, and loops over 12-byte records following the entry links until it finds a record whose first byte is 2, then calls another routine and stops; otherwise it returns.
- `loads_byte_and_sets_bit`: Clears d0 and copies the byte at offset 2 of a1 into it, then sets d1 to 2 and d2 to 3 and calls a bit-operation routine, branching to a different d2 value of 9 when that call's result is nonzero.
- `saves_registers_and_decrements_byte`: Loads the byte at a fixed absolute address into d0, calls a routine that decrements a byte at offset 19 of a0 under a threshold check, then restores d0 and the saved registers and returns.
- `scans_record_chain_for_type_three`: Walks a linked chain of 12-byte entries starting from a computed base, stopping at the terminator value 65535.
- `conditionally_adjusts_record_sums`: When the byte at offset 2 of the structure in a1 is nonzero and a global byte is zero, it reads two words at offsets 178 and 182 of a record, adds them, and calls two routines, branching to a common exit when the sum is negative, otherwise it calls one routine and returns the saved stack value.
- `loads_flag_byte_and_sets_bit`: Clears d0 and copies the byte at offset 4 of a1 into it, sets d1 to 1 and d2 to 3 and calls a bit-operation routine, then, when the result is zero and the byte at offset 1 of a1 is nonzero, sets d2 to 13 before branching to the bit-operation step.
- `saves_registers_and_decrements_byte_2`: Loads a byte from a different fixed absolute address into d0 with d2 set to 15, calls the same decrement routine as the sibling, then restores the saved registers and returns.
- `draws_grid_cells_with_blits`: Computes a start position from grid-dimension globals and issues several calls to set up drawing, then runs two nested loops over a grid of cells, calling a routine for each non-zero cell and a separate handler when the cell matches the stored target coordinates, and finally clears the state with a few more calls before returning.
- `scans_grid_and_blits_rows`: Repeatedly calls a routine while decrementing a counter, then walks a grid using nested counters and calls a routine for each non-zero cell.
- `scans_grid_and_blits_cells_2`: Repeatedly calls a routine while decrementing a counter in a global, then walks a grid of cells using nested counters, calling another routine for each nonzero cell, and returns the value passed in.
- `scans_grid_and_blits_cells`: Repeatedly calls a routine while decrementing a global counter, then walks a grid of cells with nested counters, calling a blitting routine for each nonzero byte and a separate routine for one marked cell whose choice depends on bit 0 of a global byte.
- `iterates_grid_cells_and_clips_blits`: Calls a routine repeatedly and increments a global word, then iterates over a grid of cells using two global counters: for each nonzero byte it calls a routine, and at one specific cell it either calls one routine or another depending on bit 0 of a global byte, returning the parameter unchanged.
- `steps_pointer_and_clips_blit`: Loops over characters at a pointer in a2 with counters in d4 and a stack word, incrementing two global words on each step and calling a routine when the character is nonzero.
- `forwards_to_masked_rectangle_clip`: Calls one routine and returns, leaving the registers as the called routine set them.
- `adds_offset_and_blits_rows`: Saves d3 and a0, adds a 192-byte multiple selected by the low two bits of the word at absolute address 598 to a0, and calls the row-blitting routine before restoring the saved registers and returning.
- `tests_mask_and_calls_helpers`: Calls six other routines, tests bit 4 of a global longword to optionally call five more, then tests bit 0 to choose between two routines, and returns the d0 and d1 values.
- `calls_clip_and_decrement_pair`: Calls a clipping routine and then a routine that decrements a byte at offset 19 of a0, and returns with the value in d0 unchanged.
- `scans_control_bytes_with_branches`: Reads bytes from a buffer through a0 in a loop, returning when a zero byte is found, and branches on each byte value: 17 tests bit 0 of a global byte, 8 tests bit 1, and values whose bit is set in the mask 112 test bit 2, with other values calling three routines before looping again.
- `tests_bit_and_branches_on_mode`: Compares d0 with 8, branches if not equal, and tests bit 1 of a global byte; depending on the result it branches to one of two code paths, or falls through to an unconditional branch.
- `dispatch_bit_test_loop`: Tests a bit of the value in d0 against a bitmask in d4 and, when it is set, branches to a flag-masking path.
- `saves_word_and_decrements_byte`: Pushes d0 onto the stack, loads a byte from a fixed absolute address into d0, calls a routine that decrements a byte at offset 19 of a0, then restores d0 and returns.
- `masks_and_stores_bitmap_column`: Computes a byte offset from the word in d0 and the stride in a global longword, then ANDs eight bytes from a global buffer with a stepping pointer and stores them into a column of the destination bitmap, calling a routine before and after the loop.
- `clears_flag_byte_after_lookup`: Saves d0 and d1, reads a word from a global variable and calls a routine with a constant in d1, stores the returned byte into a global variable, clears a separate global flag byte, and restores the registers.
- `saves_d0_and_runs_two_routines`: Saves d0 on the stack, calls a routine, reads a global byte into d0, calls another routine, and then restores d0 from the stack and returns.
- `flushes_pending_state_when_flagged`: When a global byte is nonzero, calls three routines, one of them passing a word from a global variable, then clears the global byte to zero and returns.
- `scans_bitmap_cells_with_bit_test_2`: Returns immediately with the d0 and d1 values unchanged when a global byte is nonzero; otherwise it loops over a 3-row by 8-column grid, testing bit 1 of a value from a routine for each column and calling a routine for each cell that passes.
- `scans_bitmap_cells_with_bit_test`: Runs nested loops over a 16-bit counter in d7, calling two routines per cell, and keeps looping while bit 1 of a computed value is set; when the counter reaches -1 it calls another routine and returns.
- `dispatches_blitter_or_returns_minus_one`: Saves d0, d1 and a0 on the stack, then compares a global word at a fixed address with 6.
- `computes_index_and_decrements_byte`: Computes an index from d0 and d1 minus one, multiplies d1 by a global table word, masks the result to three bits, passes it to a routine that returns a table value, then calls the decrement routine and returns.
- `sets_bit_in_bitmap_cell`: Converts the 16-bit values in d0 and d1 to zero-based coordinates, and if both are below the two global limit words, computes a linear bit index using the first limit as row width.
- `builds_slot_record_buffer`: Clears a global word, reads a longword from a structure in a linked list, and loops over 30 table entries of five bytes each, writing a status byte, a bit index, a computed value and a counter byte into an output buffer for every set bit in a mask.
- `tests_bit_of_record_value`: Loads a global word, adds 5 to it, and tests the bit numbered by that result in d0.
- `scans_byte_record_for_threshold`: Reads the byte at offset 1 of the record in a1 into d0, and when it is zero it branches to a later point in the routine.
- `processes_linked_slots_and_draws`: Loops over a table of byte records, writing computed values into an output buffer, and calls helpers and a rectangle-drawing routine.
- `calls_draw_rectangle_routine`: Calls the rectangle-drawing routine and then returns to its caller.
- `draw_rectangle_and_push_pointer`: Calls helper routines to draw a rectangle, loads a global word from a table indexed by another global, swaps two longword globals, copies one into a third and inverts a global byte.
- `stores_word_when_flag_set`: When a global byte is nonzero, it calls two routines, takes the word returned in d0 and stores it through a0 at an offset computed from a global word, then calls one more routine.
- `sets_fixed_rectangle_and_tail_jumps`: Loads four fixed coordinate words into d0 through d3 and a constant into d4, calls a rectangle routine, then jumps to another routine without returning here.
- `draws_text_line_with_fill`: Clips a rectangle, then walks up to 16 entries of a table, selecting one of two fill values per entry, and when the byte at offset 2 of the current record is nonzero it sets a global byte to 40 and writes two bytes through a0 before returning the final register value.
- `set_default_status_word`: Tests the byte at the address in a1; when it is zero, writes the word 31 to a fixed global address, and otherwise branches past that write to a later instruction.
- `copies_text_line_with_fill`: Loops over a count in d2, and when the byte at offset 2 of the structure in a1 is nonzero it stores the value 40 into a global byte, writes 41 and 0 to the buffer in a0 and calls a routine, decrementing d2 each pass until it reaches -1.
- `blit_entry_from_index_table`: When d0 has a nonzero high byte and the summed index is within a limit and its table flag is zero, it stores a table byte into a global word, sets blitter-control globals to 26 and 31, and calls helper routines.
- `clears_blitter_state_for_row_byte`: Adds the high byte of d0 to a global byte and, when the sum is within a global limit, reads a table entry for that index; if the entry is zero it clears a global word and sets another to 65535 around two calls, otherwise it writes 31 to a global word and calls two routines.
- `sets_word_when_byte_equals_one`: Compares the byte at the address in a0 with 1 and, when equal, loads 27 into d3 before branching unconditionally to a fixed location in another routine; otherwise it branches to a different fixed location.
- `set_word_and_branch_back`: Loads 241 into d3, calls another routine, and then branches unconditionally to a fixed location in another routine.
- `calls_helpers_and_zeroes_flags`: Calls three other routines around a sequence that writes zero to one global longword, writes 65535 to another global word, and clears the second word again at the end before returning.
- `selects_and_stores_slot_flags`: Reads a byte from a global table indexed by two global values and tests its top bit and bit 6 to choose among several branches.
- `sets_globals_and_calls_helpers`: Shifts the high byte of d0 into position, calls a helper, stores the bytes and words 240 and 2 into global variables, loads a fixed address into a0 and calls another helper.
- `tests_bit_and_stores_words`: Tests bit 0 of d0 and, depending on the result and a global byte, stores the word 284 into a global word, then branches to a later instruction.
- `stores_constants_and_sets_bit_mask`: Stores the word 235 and the longword 2626 into two global variables, loads a0 with a fixed address and calls a routine, then if a global word is nonzero it shifts 1 left by that value into a global word, otherwise it clears a global byte and branches to a later point.
- `clears_flag_and_conditionally_calls`: Stores constants into two global variables, then when a pointer taken from an indexed table is nonzero and a second pointer from it is nonzero, calls that pointer, and finally clears a global byte and calls another routine if it was nonzero.
- `save_and_branch_conditionally`: Saves d0, d1 and a0 on the stack, then tests a global flag byte; if it is set and a second global flag byte is also set, it loads the byte at offset 27 of the structure whose pointer is in a global longword into d0 and branches on to another routine's code, and otherwise it branches past that load to the same continuation point.
- `branches_into_helper_then_clears`: Loads a word from a global variable into d0, calls a fixed routine, clears a global byte, and then branches to another routine's code.
- `decrements_structure_word_and_sets_flag`: Subtracts a table byte from a word at offset 208 of a structure, clamps it at zero, then adds two words at offsets 194 and 198, stores the sign of the sum as 0 or -1 in a global byte, and calls routines depending on that flag.
- `saves_registers_and_calls_routine_2`: Loads the constant 248 into d0, calls a fixed routine, restores registers from the stack, then tests a global byte and returns.
- `iterates_flagged_slots_calling_handler`: Loops over six slots indexed from 1 upward, and for each slot whose global byte is nonzero and whose bit in d0 is set, copies that byte to a global variable, calls a handler through a0 with d0 and the slot index, and advances the slot pointer.
- `adjusts_byte_field_and_calls_helpers`: Reads a 16-bit index from a table entry, decrements a byte at offset 1 of that entry unless it is already 255, and calls helpers.
- `dispatches_record_checks_and_helpers`: Compares a byte returned by a helper with 5 and, when it matches and two global longwords are equal, subtracts 10 from a word and calls one of two helpers depending on the sign, then calls two further helpers.
- `adjusts_index_and_calls_helpers`: Subtracts a word global from d0, then subtracts 9 from it and calls three routines in sequence.
- `tests_flag_then_calls_routines`: Calls one routine and, if the zero flag it leaves set, calls two further routines; otherwise it returns without making those calls, then returns.
- `conditionally_advances_word_and_returns`: Reads a word from the structure in a0 at offset 30 and masks it with 58496; if that result is zero and a byte from a global table indexed by a global word is non-negative, it calls two other routines and returns the word they produce, otherwise it returns the masked value of the first word read.
- `tests_flag_then_calls_chain`: Calls one routine and, when the zero flag it leaves set is true, calls two further routines before returning; otherwise it returns at once.
- `clears_bit_from_flags_and_fills`: Reads the word at offset 30 of the structure in a0, and when the masked bits are zero it looks up a byte from a table by the low byte of a global word, clears the bit at that index in the flags word, and calls another routine; it then calls a routine that fills twenty random entries using d7.
- `tests_flag_then_branches_forward`: Calls one routine and, when the zero flag is clear, branches forward into another routine; otherwise it loads a fixed address into a0 and branches to a later point.
- `calls_two_fixed_routines`: Loads a fixed address into a0, then calls two routines in sequence before returning.
- `clears_bit13_and_stores_word_state`: Reads a word at offset 30 of the structure in a0, and when bit 15 is set it branches.
- `clears_bit13_saves_a0_calls`: Tests bit 13 of d0 and, when it is clear, saves a0, loads the constant 249 into d0 and calls a fixed routine before branching forward.
- `clears_bit_and_stores_state`: Clears bit 13 of d0 and stores the result to a word at offset 30 of the structure in a0.
- `toggles_bit_13_at_offset_30`: Reads the word at offset 30 of the structure in a0 and, when bit 13 is set, clears that bit, sets bit 14 and writes the word back.
- `tests_flag_then_branches_to_routine`: Calls one routine and branches on the zero flag, either into another routine or after loading a fixed address into a0.
- `calls_fixed_routine_after_slot_loop`: Loads a fixed address into a0, calls a routine and returns.
- `sets_bit13_clears_bit14_store`: Reads the word at offset 30 of the structure in a0 into d0, and when bit 14 is clear it loads the constant 251 into d0 and calls a fixed routine before branching forward to a later point.
- `sets_bits_and_stores_at_offset_30`: Clears bit 14 and sets bit 13 of d0, stores the result to the word at offset 30 of the structure in a0, then loads the constant 253 into d0 and calls a routine before returning.
- `clears_bit14_sets_bit15`: Reads the word at offset 30 of the structure in a0; if bit 14 is set, it clears that bit, sets bit 15, stores the word back, and calls another routine with d0 set to 255, otherwise it returns.
- `tests_flag_then_calls_pair`: Calls one routine and, when the zero flag it leaves set is true, calls a second routine; then it loads a fixed address into a0 and calls a third routine before returning.
- `sets_bit14_clears_bit15_branch`: Reads the word at offset 30 of the structure in a0 into d0, and when bit 15 is clear it loads the constant 252 into d0 and calls a fixed routine before branching forward to a later point.
- `sets_bit14_clears_bit15`: Clears bit 15 of d0, sets bit 14, and stores the result as a word at offset 30 of the structure in a0, then calls another routine with d0 set to 254 before returning.
- `tests_flag_then_calls_fixed_pair`: Calls one routine and branches on the zero flag, either forward past the address load or to a later point after loading a fixed address into a0.
- `calls_two_fixed_routines_2`: Preserves d7 and a0 on the stack, loads a fixed address into a0, calls two routines in sequence and returns.
- `tests_bits15_14_then_calls`: Reads the word at offset 30 of the structure in a0 into d0 and masks it with 57344 into d1, branching forward when the masked result is zero.
- `clears_bit13_sums_offsets`: Clears bit 13 of d0 and stores it at offset 30 of the structure in a0, stores the sum of the words at offsets 206 and 204 at offset 202 of that structure, then calls three other routines, the last after loading d0 with 57 and the first after saving d7 into d0, before returning.
- `moves_bit13_to_bit14`: Checks bit 13 of the word at offset 30 of the structure in a0; if it is set, it clears that bit, sets bit 14 in its place, stores the word back, and calls another routine with d0 set to 256, otherwise it returns.
- `calls_chain_after_flag_test`: Calls one routine and, when the zero flag it leaves is set, calls two further routines in sequence; otherwise it returns after the first call.
- `shifts_entries_when_flag_set`: Reads the word at offset 30 of the structure in a0, and when the masked bits are zero and bit 6 is set it calls a routine that shifts table entries, then calls a routine that fills twenty random entries using d7.
- `tests_flag_then_branches_to_helper`: Calls one routine and branches on the zero flag, either calling a helper routine or branching forward to a later routine.
- `conditionally_updates_byte_offset_one`: Converts the low byte of a value from a table into a word-sized comparison against a byte at offset 1 of a structure in a0, and when the result is positive it adds one to that byte at offset 1 of a0 after multiplying the 16-bit halves of a longword result.
- `conditionally_decrements_offset_one`: Subtracts the byte at offset 1 of a0 from the low byte of a value read from a1, branches to a path that calls routines when the result is positive, and otherwise stores d0 and calls another routine with d0 set to 257 before jumping to a common exit.
- `increments_byte_at_offset_one`: Calls two other routines, obtains a longword result, multiplies its two 16-bit halves, adds the high word of the product plus one to the byte at offset 1 of the structure in a0, then calls two more routines and returns.
- `dispatches_on_byte_then_calls_pair`: Calls one routine and, when the zero flag is set, reads a word from a global variable, subtracts 2 from it and doubles it to index a table, then loads two bytes from that table; it then calls two further routines and returns.
- `tests_flag_then_calls_routines_again`: Calls one routine and, when the zero flag it leaves set is true, calls two further routines; otherwise it returns.
- `advance_counter_and_add_offset`: Tests a masked word field at offset 30 of the structure in a0 and, when the masked bits are zero and the word at offset 26 is not 32767, increments that word and adds 250 to a long at offset 270, then calls another routine which returns the value; otherwise it returns.
- `dispatches_flagged_handlers_conditionally`: Calls a helper that tests flag state and, when the second flag test is also set, calls another routine; otherwise it skips that call and then calls a second routine that stores a fixed address into a0 and calls a further routine before returning.
- `accumulates_flagged_entries_and_calls`: Reads a word at offset 30 of the structure in a0, and when the masked bits are zero, it reads a global byte and calls routines to store it.
- `tests_bit_and_stores_word_pair`: Clears a global byte, derives a shift count from a global variable and tests a bit of a word read from the same source; when the bit is set and the second word also passes, it stores two words into global variables and calls two routines before returning.
- `calls_routine_with_code_66_2`: Sets d0 to 66, calls another routine, and then returns.
- `copies_three_words_into_structure`: After calling a helper that returns a pointer in a0-derived form, writes three global words into a structure at offsets 36, 38 and 40 and then calls two other routines.
- `updates_global_words_from_structure`: Reads a word at offset 38 and a word at offset 40 of a structure, then branches on the second word: when it is zero it calls one routine, and otherwise it compares it with a global word and conditionally stores two global words and calls further routines.
- `stores_words_into_globals`: Calls a routine with d0 set to 260, then compares a global word with d3; if they differ it stores d1 and d2 into two global words and calls another routine, otherwise it branches to a common exit.
- `stores_d1_d2_d3_into_globals`: Calls one other routine, then stores the word values in d3, d1 and d2 into three global words in that order, and calls one more routine before returning.
- `stores_table_pair_into_slot_two`: After the flag test, loads a word from a global, subtracts 14, doubles it, reads two bytes from a table indexed by that offset into d1 and d0, sets the slot selector to 2 and calls a routine to store the pair.
- `stores_table_pair_into_slot_three`: After the flag test, loads a word from a global, subtracts 17, doubles it, reads two bytes from a table indexed by that offset into d1 and d0, sets the slot selector to 3 and calls a routine to store the pair.
- `redraws_and_blits_rows_conditionally`: Calls several routines and, when a global byte is at least 2, computes a row count from a word read through a pointer and loops over it, storing a longword to a global variable and advancing a second word on each pass.
- `blits_rows_with_decrementing_offsets`: Stores a global byte to a set value, and when a global byte is at least 2 it runs a loop that decrements a signed counter in d7, storing a global word from a value and subtracting a step from it on each pass, calling blit helper routines for each row, and otherwise it finishes by calling one further routine.
- `stores_offset_and_calls_blit_helpers`: Loads the constant 64 into d0 and calls a helper, compares a global byte with 2 and calls a second routine when it is not lower, then loads a global word into d0 and the constant 2 into d1 before branching to a later point.
- `loops_blit_rows_with_offsets`: Stores d0 into a global word and runs a loop of calls, subtracting d1 from d0 and adding 2 to d1 on each pass, using d7 as a signed counter that is decremented until it reaches minus one.
- `stores_table_pair_into_slot_four`: After the flag test, loads a word from a global, subtracts 21, doubles it, reads two bytes from a table indexed by that offset into d1 and d0, sets the slot selector to 4 and calls a routine to store the pair.
- `calls_five_routines_when_flagged`: After the flag test, calls five routines in sequence, the first of which is preceded by a comparison of a global byte against 2; the others are plain calls and nothing else is done.
- `branches_to_record_update_routine`: Calls a routine and, when it reports zero, branches forward; otherwise it calls a second routine and then branches unconditionally to a later point in the code.
- `toggles_bit1_indexed_byte`: Computes an address from a global value scaled by six plus a base offset applied to the result of a routine call, clears bit 1 of the byte at offset 3 of that address if it is set, and then calls two other routines.
- `clears_bit_one_of_structure_byte`: Clears bit 1 of the byte at offset 3 of the structure in a0, then calls two other routines, one after saving d0 and loading it from a global word, and returns.
- `branches_to_countdown_dispatcher`: Calls a routine and, when it reports zero, branches forward; otherwise it calls a second routine and then branches unconditionally to another point in the code.
- `copies_entry_fields_when_flag_set`: Reads a six-byte entry from a table indexed by a global word and, when bit 6 of a flag byte in a structure is set, copies its longword and word fields into two globals.
- `copies_longword_pair_and_sets_flag`: Copies two words from the longword pointed to by a0 into two global variables, then sets the byte at the address in a1 to 1 and tests the byte at the address in a0.
- `calls_routine_with_code_272`: Sets d0 to 272, calls another routine, and then returns.
- `calls_helpers_and_updates_record`: Calls a helper with the low byte of d0 and, when the zero flag from that test is set, reads a word from a table slot indexed by a global variable scaled by 6, sets bit 0 of its third byte, and copies its longword and following word into two global variables.
- `tests_flag_then_calls_handler`: Calls a routine that sets the zero flag from its result, and if the flag is set, calls a second routine.
- `dispatches_on_record_bit_three`: Calls a routine and loads a word from the pointer it returns, then calls another routine.
- `calls_routines_and_conditional_helper`: Calls a routine and compares its byte result with 5, calling another routine when equal, then calls a third routine after loading a0 with a fixed address and calling a fourth routine, and returns.
- `loads_fixed_address_and_calls_handler`: Calls a routine and, when the zero flag from that test is set, loads a fixed address into a0 and calls a second routine with it.
- `sets_blit_masks_and_swaps_buffers`: Calls several routines, writes 65535 to one global word and 31 to another, clears a global word, swaps two longword globals and sets a third global to their previous first value, then inverts a global byte.
- `set_blit_masks_and_swap_buffers`: Sets two blitter-control globals to 65535 and 31, calls helper routines to draw, clears a global word, swaps two longword globals, and inverts a global byte.
- `branch_to_speed_lookup_or_set`: Compares the byte at a fixed global address against 2 and, when it is not less than 2, branches to a shared path after loading the constant 258 into d0 and calling a routine; otherwise it branches forward to a separate path.
- `copies_speed_byte_to_global`: Calls a routine and, when the zero flag from that test is set, copies a byte from a table indexed by a global word minus 8 into a global byte, then calls two routines, one of which loads a fixed address into a0 before the final call.
- `stores_table_byte_into_slot_seven`: Calls a routine and, when the zero flag from that test is set, copies a byte from a table indexed by a global word minus 13 into d1, clears d0 and d7 to 6, and calls a routine that stores d1 into a slot.
- `stores_fixed_values_into_slots`: Calls a routine and, when the zero flag from that test is set, sets d0 to 0 and d1 to 180, stores d1 into slot 5 with a call, then stores into slot 6 with a second call.
- `tests_global_then_calls`: Tests a word-sized global variable and, when it is nonzero, loads the constant 283 into d0 and calls a fixed system routine.
- `compare_flag_and_call_handler`: Compares the byte at a fixed global address against 1 and branches forward when they are equal; otherwise it loads the constant 244 into d0, calls a routine, and then branches to the end.
- `conditionally_sets_flag_and_calls`: Calls two routines and then compares a returned byte with 2 when the zero flag from the first test is set.
- `calls_handler_with_fixed_address`: Calls a routine and, when the zero flag from that test is set, loads a fixed address into a0 and calls a second routine, then calls a third routine.
- `updates_health_and_flag_globals`: Masks the word at offset 30 of the structure in a0 with 58496, and when that result is zero it subtracts a scaled value from the word at offset 106, clamps the result to a minimum of 18, and calls several routines, then checks a global flag against a global word before calling further routines.
- `copies_fixed_address_into_a0_and_calls`: Calls a routine and, when the zero flag from that test is set, loads a fixed address into a0 and calls a second routine with it before returning.
- `swap_globals_and_toggle_flag`: Clears a longword global, calls five other routines, then swaps two longword globals and copies the first into a third global.
- `resets_mask_and_calls_helpers`: Stores 31 into a global word, calls two helper routines, computes a count from a global variable clamped to 16 or 32, and then loops that count down, calling two helper routines on each pass until the count reaches zero.
- `loops_blitting_text_lines_downward`: Repeats a loop that loads a fixed table address into a0, calls two other routines with values taken from d0, advances d1 by 7 and d2 by 1 each pass, and decrements a counter in d7 until it reaches minus one.
- `releases_pooled_pointer_then_jumps`: Calls one routine, then if a global pointer is not the base of its table it decrements that pointer by four and calls another routine when the longword it now refers to is nonzero.
- `sets_registers_and_calls_helpers`: Loads fixed constants 192, 164 and 30 into d0, d1 and d3, calls a routine twice, calls another routine when a comparison flag is nonzero, and then calls a final routine and returns.
- `draws_clipped_text_line_from_offset`: Adds the high byte of d0 to a global byte, and when the sum is below 33 stores it in a global word, sets blitter-related globals, calls the clipped blit routine, and conditionally calls a helper depending on a global byte.
- `draws_highlighted_rectangle_if_index`: If the high byte of d0 is non-zero and the sum of that byte and a global variable is below 33, it calls another routine, stores 0 and 65535 into two global words, calls two other routines, and finally stores 0 into one of those words.
- `dispatch_conditional_slot_handlers`: Calls a subroutine, and if the zero flag is set afterward, loads the address of a data item into a0 and calls two further subroutines.
- `advances_word_when_table_entry_set`: Masks the word at offset 30 of the structure in a0 with 58496, and if the result is zero and the word at offset 210 of that structure is non-zero, it calls two other routines and returns the second one's result, otherwise it returns zero.
- `dispatch_conditional_slot_sum`: Calls a subroutine, and if the zero flag is set afterward, loads the address of a different data item into a0 and calls two further subroutines.
- `clear_flag_and_sum_offsets`: Clears the word at offset 30 of the structure in a0, stores the sum of the words at offsets 204 and 206 into the word at offset 202, then jumps to another routine, which fills twenty entries.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `dispatch_conditional_slot_clamp`: Calls a subroutine, and if the zero flag is set afterward, loads the address of another data item into a0 and calls two further subroutines.
- `compute_clamped_word_update`: Returns the word at offset 30 masked with a constant, and when that masked value is zero it calls another routine, adds a scaled value from its result plus one to the word at the pointer in a0, clamps it to the word after it, stores it back, and then calls another routine.
- `runs_conditional_schedule_and_clears`: Calls a routine and tests the flag state, then compares several global bytes and words against constants; when they match it runs a loop over a word-sized range that stores a counter in a global word and calls helpers, then sets a global byte to 255, clears another global byte to 0 and returns.
- `conditionally_runs_blit_schedule`: Calls a routine and then compares a global byte, offset by minus 2, with zero.
- `sets_flag_and_runs_helpers`: Calls one routine, sets a global byte to 255 twice with a second routine call between, then calls a further routine and finally clears the same global byte to zero before returning.
- `dispatch_conditional_slot_bit5`: Calls a subroutine, and if the zero flag is set afterward, loads the address of a further data item into a0 and calls two further subroutines.
- `set_bit5_and_call_handler`: Reads the word at offset 30 of the structure in a0 and, when its masked bits are zero, sets bit 5 of that word and calls another routine, then returns.
- `fills_pointer_slots_with_table_values`: When a global flag is zero, it stores two derived values into global variables, then writes 24 consecutive longword entries starting at the buffer in a0, each taken from a helper and incremented by one, and calls three routines before returning.
- `dispatches_record_updates_and_countdown`: Calls a sequence of helper routines, including one with the 6-byte record selected by the word in d1 minus 1 from a table, then compares a global variable with 200 and, when it is below 200, calls a routine and decrements the byte at offset 19 of a0 if that byte is nonzero and the comparison was equal.
- `stores_pair_in_indexed_slot`: Preserves d7 and a0, and when the low byte of d1 is not less than the word at a computed table slot selected by d7, writes the low byte of d0 and the low byte of d1 into that slot, then restores the registers and returns.
- `reset_clamped_coordinate_globals`: Clears four 16-bit globals and sets a global from the sign of the doubled coordinate values, then computes two clamped coordinate words from the table values when they fall below fixed thresholds.
- `computes_clamped_coordinate_globals`: Doubles two global words, then calls several other routines, and if each doubled value is at or above a fixed threshold it substitutes a half of a computed value from another pair of globals, and finally swaps two longword globals and stores one of them into a third global.
- `clamps_coordinates_and_swaps_longwords`: Copies two global words into two other globals, sets a global byte from sign tests on each offset value and computes replacement words from fixed constants when negative, swaps two longword globals and copies the first into a third, inverts a global byte, and calls several routines.
- `reads_byte_and_clears_slot_entry`: Reads a global byte into d0, calls several routines, decrements a global byte, and then calls more routines, including one that is made only when the longword at offset 36 of the structure returned is nonzero.
- `draws_rectangle_at_fixed_coordinates`: Loads fixed constants 16, 49, 11 and 9 into the data registers and a fixed address into a0, calls a routine and the rectangle-drawing routine, then calls a routine that advances a table index and tail-jumps to a final routine.
- `advance_table_index_and_call`: Computes a table offset from two word globals, passes it to a helper routine and calls further helpers.
- `dispatch_indexed_handlers_conditionally`: If the high byte of the register d0 is nonzero, it computes an index from bits 8 to 11 minus one and calls a handler from a table through a pointer, then calls the same handler a second time when that index is odd.
- `decrement_word_if_flag`: When the global variable at address 1 byte is nonzero and the word global variable at offset 3442 is at least 2, it subtracts 2 from that word and returns.
- `decrement_word_at_0d70_flagged`: When the byte global variable is nonzero and the word global variable at offset 3440 minus 2 is not negative, it subtracts 2 from that word and then returns.
- `increment_word_below_limit`: When the flag byte is nonzero and the word at offset 3440 plus 2, minus the word at offset 3448 minus 71, is negative, it adds 2 to the word at offset 3440 and returns.
- `increment_word_bounded_by_0d7a`: When the flag byte is nonzero and the word at offset 3442 plus 2, minus the word at offset 3450 minus 55, is negative, it adds 2 to the word at offset 3442 and returns.
- `draws_pixel_grid_from_table`: Calls two helper routines, then runs a nested loop over the rows and columns given by two global words, calling a helper and setting a pixel pattern where a second word is nonzero, then calls two more helper routines before returning.
- `scans_grid_for_threshold_bytes`: Calls two helper routines, then walks a byte grid in a nested loop over two global word counts, reading every other byte and calling a helper when a byte is at least 101, and then calls two more helper routines before returning.
- `draws_four_row_pixel_pattern`: Reads a byte from a table, uses a global word to compute a row stride, then runs a four-iteration loop that shifts the byte left and sets or clears a 16-bit mask bit at two addresses per row, stepping by the stride between rows, and finishes by calling another routine.
- `masks_and_steps_pointer_loop`: Ands d1 into a table entry indexed by d6 and into the entry at a0, advances a0 by d5, decrements the loop counter d7 and branches back while it is not minus one, then saves d0, reads a global byte into d0, calls a routine, and restores the registers.
- `draws_rotated_pixel_pattern_in_rows`: Shifts a byte from a table through four iterations, and for each row sets or clears a bit in a 16-bit mask that is rotated right and then left by one bit before the mask is combined with two words, one row apart, at an address derived from a global word.
- `masks_and_rotates_pointer_loop`: Ands d1 into a table entry indexed by d6, rotates d1 right by one bit, ands it into the entry at a0, rotates d1 left by one bit, advances a0 by d5 and repeats while the counter d7 is not minus one, then saves d0, reads a global byte into d0, calls a routine, and restores the registers.
- `compute_and_store_global_byte`: Saves d0, calls a routine, stores the result of a second routine into a global byte, and restores d0, computing a value from a global word and another global word with multiplication and doubling before those calls.
- `initializes_and_blits_bordered_rectangle`: Copies a block of eight words between two global variable areas, sets a global word to 2, and looks up a byte from a structure in a global pointer against a table of eight words before storing the match index in a global.
- `fills_entry_and_clears_global`: Calls a routine eight times in a countdown loop, then calls a routine that returns a structure pointer and writes a flag bit and a byte derived from a global word table into it; it then calls several more routines, clears a global longword, and returns the value from one of its calls.
- `blit_centered_bordered_rectangle`: Calls several helpers, then loops five times calling two routines each pass while counting a word from 4 down to 0.
- `blits_fixed_width_rectangle_and_returns`: Calls a routine with the constant 144 and then several helper routines, computes a value from a returned word plus 2 and passes it to another routine, then returns the word returned by a helper in d0.
- `resets_counter_on_wrap_threshold`: Calls another routine that loads a table entry and sets the zero flag, then if that flag was set and the global word is at least 4 it clears that global word to zero.
- `clamps_counter_up_to_four`: Calls a table-entry helper, and when the global word is below 4 it stores 4 into it, otherwise it leaves the word unchanged and returns.
- `cycles_counter_down_within_group`: Calls a table-entry helper and, when its flag is set, divides the global word by 4, subtracts 1 from the remainder with wraparound to 3, and writes back the group base times 4 plus the adjusted remainder.
- `cycles_counter_up_within_group`: Calls a table-entry helper and, when its flag is set, takes the global word modulo 4, adds 1, and resets it to 0 if the remainder was 3 or more, then writes back the group base times 4 plus the new value.
- `invokes_callback_after_table_check`: Calls a table-entry helper and, when its flag is set, calls another routine and calls a further routine only if that result byte is nonzero.
- `calls_helper_and_returns_word`: Calls one routine with the constant 112, then calls another routine to obtain a word, calls a further routine, and returns that word in d0.
- `loads_table_entry_and_calls_helper`: Multiplies d0 by 6 to index a table at a fixed address, reads two words from it and adds 48 and 49 to them, reads a third word, then calls a routine that writes to a global byte based on a zero test and tail-jumps to a routine.
- `calls_four_routines_then_jumps_2`: Calls three routines in sequence, then calls a routine that copies eight words from a global pointer into a buffer, clearing the top bit of each word, and executes a jump to another routine.
- `decrements_counters_and_scans_flags`: Calls two routines, then walks six global byte counters, decrementing each nonzero one and calling a routine when it reaches zero, then walks six global bytes again and calls routines for each nonzero one, returning the value returned by the first routine.
- `waits_releases_and_pops_record`: Calls a routine that waits for status bits to clear, calls two routines, and if the global record pointer is not the sentinel it decrements a global variable when bit 0 is set, moves the pointer back by 54 bytes, and calls another routine when bit 1 of the old flags is set, then tail-jumps to a releasing routine.
- `releases_slot_and_calls_handler_30`: Calls a routine that releases a linked slot, then tail-jumps to a routine, and invokes the code pointer at offset 30 of the global structure when that pointer and the structure are non-null.
- `reads_status_and_sets_globals`: Calls two helper routines, reads a word through a pointer returned by one of them, and stores a sign value or a small code into a global variable depending on bit 1 of that byte-sized value.
- `clears_flag_and_pops_record`: Releases a linked slot if a global byte is nonzero, sets that byte to 255, calls two routines, and when the global record pointer is not the sentinel it decrements a global variable if bit 0 is set and moves the pointer back by 54 bytes, calling another routine when bit 1 of the old flags is set.
- `sets_flag_brackets_blit_calls`: Sets a global byte to 255, calls three routines, then clears the same byte to 0 and calls three more routines before jumping to a final routine.
- `saves_registers_and_runs_blit_setup`: Saves registers d0 through d7 and a0 through a6 on the stack, then calls four routines in sequence, one of which is a blit-related setup routine.
- `stores_word_and_repeats_calls`: Stores the word in d0 to a hardware register address, then repeatedly calls a routine and decrements d0 with dbra until the counter wraps.
- `locate_table_entries_and_count_down`: Calls a routine to get a base address, derives three global pointer values from table entries indexed by a global byte, scans 32 entries of a 10-byte table accumulating an offset, and decrements a byte at offset 19 of a structure when the low byte of d0 is 200 and that byte is nonzero.
- `copy_words_and_branch_on_flag`: Copies two global words to two other global words, then when a global byte is zero copies two more global words to two further global words and branches to another routine.
- `copy_config_words_when_mode_one`: Compares a global byte against 1, and when it equals 1 copies three words from fixed global addresses into three other global words, then branches past the copy to a later instruction; otherwise it branches directly onward.
- `copy_three_longs_from_1b62`: Copies three 32-bit values from the global area at offset 7010, 7014 and 7018 into the global variables at offsets 1360, 1364 and 1368, then returns.
- `copy_words_back_and_branch_on_flag`: Copies two global words from one pair of global variables to another pair, then when a global byte is zero copies two further global words in the opposite direction and branches to another routine.
- `copy_three_words_to_globals`: Copies three 16-bit words from one set of global variables to another set when a global byte equals 1, and otherwise branches to a different part of the code.
- `copy_three_longs_to_1b62`: Copies three 32-bit values from the global variables at offsets 1360, 1364 and 1368 into the global area at offsets 7010, 7014 and 7018, then returns.
- `returns_word_global_or_flagged_word`: Calls another routine, then returns the word at a fixed global address if a global byte is non-zero, otherwise it returns the value from the previous call.
- `dispatch_sprite_flags_and_timer`: Calls the store_flags_and_call_timer routine and then jumps to update_sprite_flags_and_dispatch, which walks a 20-byte record table and updates flag bits on matching entries.
- `store_flags_and_call_timer`: Pushes d0 onto the stack, sets a global byte to 255, clears a global word, and loads a word from a global variable into d0 for a loop counter.
- `set_word_08e6_to_four`: Stores the constant 4 into the word global variable at offset 2278, then pops a long from the stack into d0 and returns.
- `update_sprite_flags_and_dispatch`: Walks a 20-byte record table, clearing or setting flag bit 9 on entries whose negative-sum test passes, then conditionally updates a global byte from bit 15 of a value and branches on a stored mode value (0 or 7).
- `scan_table_for_matching_entry`: Walks a table of 20-byte records, advancing a0 by 20 each step and decrementing a counter, until it finds a record whose first two bytes match the values held in d0 and d1; the routine then carries on with the matched record.
- `scan_table_then_update_flags`: Walks a table of 20-byte records from the address in a0, advancing by 20 bytes per step and decrementing a counter in d7 until it finds a record whose first two bytes match d0 and d1, then calls another routine on that record.
- `updates_selected_slots_and_flags`: When a global byte equals 7 it iterates over a 6-byte global array, calling routines on each nonzero entry that passes a bit test on a structure word, then conditionally applies bit tests, clears a global byte and sets a global byte to 255 before calling further routines.
- `conditionally_calls_routine_on_bit_test`: Calls two routines, clears a global byte when it equals 1 and the table byte at the index in a global word is nonzero, and then calls another routine.
- `computes_flag_bits_and_stores_global_byte`: Loops over six global bytes, setting a bit in a mask for each non-zero entry whose two words satisfy a comparison, then loops over five more entries looking for one whose flag word has its high bits clear, clears the top bit of the mask on a match, and stores the final byte into a global variable.
- `stores_byte_and_calls_helper`: Saves registers and the low word of d0, loads the byte at offset -1 from a0 into d0, calls a helper, restores d0 and stores d6 as a byte to a fixed absolute address before restoring the saved registers and returning.
- `saves_registers_and_dispatches_by_flag`: Saves registers d1 to d7 and a0 to a1 on the stack, copies the words in d0 and d1 into d4 and d5, and branches on a global byte: if it is 2 or more it jumps to one fixed location, if it is zero it jumps to another, otherwise it loads a global byte into d7 and branches elsewhere.
- `adjust_bits_and_offsets`: Reads a global word into d2 and compares d0 with 1; when d0 is 1 or greater it clears bit 0 of d2 and adds a global word to d4, then branches past the remaining step.
- `adjusts_bits_and_coordinates`: Compares d0 with a global word and, depending on the result, sets or clears bit 0 or bit 1 of d2 and adds or subtracts global words to d4 and d5, then branches.
- `selects_value_from_table_or_record`: Computes an index from two words, reads a byte from a table, and uses it as a value if non-zero; it then scans a global table of 32-byte records for a matching position, and returns that record's longword or the table-derived value depending on flag bits.
- `looks_up_value_from_grid_or_table`: Calls two helper routines and reads a byte from a two-dimensional grid addressed by d4 and d5, mapping values of 101 and above through a global table and returning zero for empty or 255 entries, then scans a global table of 32-byte records for a matching position and flag and returns the matching value when it is not flagged with bit 5.
- `selects_table_value_or_zero`: Loads the address of a global table into a0 and subtracts 101 from d0, then compares d0 with a global word and branches on the result.
- `searches_table_for_matching_record`: Reads a longword from a table indexed by d0 and an offset in a1, then scans a global table of 32-byte records in steps, comparing words at offsets 10 and 12 against d4 and d5 and skipping records flagged in a bitmask, and returns the matched record's longword at offset 6 unless its bit 5 is set.
- `advance_pointer_and_count`: Advances the address register a0 by 32 bytes and increments the word counter in d7, repeating while the counter is below 32.
- `returns_immediately_after_bit_test`: Tests the value in d1 and, when it is nonzero and bit 5 is clear, copies d1 into d0 before restoring the saved registers and returning.
- `copies_seven_words_to_global`: Reads a byte at offset 10 of the structure returned by a helper, stores it into a global word, and copies seven consecutive words from a global table entry selected by that byte into the start of a global block, looping until the counter passes zero.
- `tests_byte_and_loads_pointer`: Tests a global byte and branches on whether it is zero.
- `stores_pointer_and_clears_flags`: Clears a global variable, copies a structure pointer from a global into another global, and calls a function pointer at offset 54 of that structure when it is nonnull.
- `toggles_flag_and_swaps_longwords`: Calls three helper routines around an inversion of a global byte, swaps two longword globals and copies one into a third, inverts the byte again, then calls further helpers and returns.
- `set_top_bits_of_structure_bytes`: When a global byte is nonzero, sets the most significant bit of several byte fields in the structure pointed to by a0, using four fields when the global byte minus 2 is negative and a different set of four fields otherwise.
- `sets_top_bit_of_four_byte_fields`: Sets the most significant bit of the bytes at offsets 2, 6, 10 and 14 from the structure pointed to by the address register a0, then returns.
- `calls_init_then_dispatches_status`: Calls a routine first, then reads a global word and compares it with 4; when it is not equal, it calls another routine, otherwise it calls a different routine, and it ends by branching to a shared exit point.
- `dispatches_when_byte_at_least_two`: Compares a global byte with 2 and, when it is not below 2, loads the address of a fixed global into a0 and calls a routine.
- `dispatch_indexed_pointer_call`: Shifts the word in d0 left by 2, reads a longword pointer from a table reached through a global variable at offset 18 plus that index, and calls the routine at that pointer if it is non-zero, then returns.
- `set_flag_and_jump_to_handler`: Sets a byte in a global variable to 255 and then jumps to a fixed address in a jump-table area.
- `clear_global_flag_byte`: Writes zero to a global byte variable and then returns.
- `releases_slot_and_calls_handler_34`: Calls a routine that releases a linked slot, then tail-jumps to a routine that invokes the word-sized handler at offset 34 of the global structure when that structure and pointer are non-null.
- `dispatch_indexed_handler_call_4`: Calls a helper, then reads a word from a global variable, scales it by four to index a table of pointers, and if the pointer found is non-zero it calls the routine at that address.
- `adjust_accumulator_then_store_offsets`: Reads a global word, offsets and negates it, scales it by a global word and shifts the product left by 13 bits into a global longword, then jumps to a fixed address.
- `computes_offset_pair_and_stores_position`: Calls two routines, subtracts the high longword of a result from a global word and adds a second global word to the low longword.
- `updates_stored_position_pair_clamped`: Calls two routines and computes a new pair of longwords from two global longwords and a returned value, storing them into globals.
- `subtract_scaled_offset_from_accumulator`: Reads a global word, subtracts 128, scales it by a global word, shifts the product left by 13 bits and subtracts it from a global longword, then jumps to a fixed address.
- `shifts_stored_position_pair_clamped`: Calls two routines and subtracts a returned longword from one global longword and the high part from another, then writes the result pair to globals through the same zero-test and 255-flag exit as its neighbours.
- `offsets_position_by_word_scaled_value`: Reads a global word, subtracts 128 and shifts it right by two, then calls two routines.
- `offsets_position_by_word_halved_value`: Reads a global word, subtracts 145 and shifts it right by one, then calls two routines.
- `offsets_position_by_word_arithmetic_shift`: Reads a global word, subtracts 161 and arithmetically shifts it right by one, then calls two routines.
- `subtract_halved_offset_from_accumulator`: Halves a global word after subtracting 128, scales it by a global word, shifts the product and subtracts it from a global longword, then jumps to a fixed address.
- `add_negated_halved_offset_to_accumulator`: Offsets a global word, negates it, halves it, scales the result by a global word, shifts the product left by 13 bits and adds it to a global longword, then jumps to a fixed address.
- `clamp_and_halve_word_in_d0`: Returns zero in d0 when the word there is negative; otherwise it branches to a routine that clamps the word to 47 and halves it, adding 1.
- `halve_clamped_word_plus_one`: Compares the word in d0 against 48 and, if it is 48 or more, replaces it with 47; it then halves the value, adds 1 and returns the result in d0.
- `scale_table_samples_pair`: Multiplies the word in d0 by a global word and shifts the product right by 4, then uses a global word masked to 10 bits, doubled, as an index into a global table to read two signed words, multiply each by the scaled value and by 4, and return both products as a pair of longwords.
- `computes_grid_cell_index_2`: Calls a helper that yields a value in d6 and a word coordinate in the low bits, then when the masked high byte equals a fixed address it derives a grid index.
- `computes_grid_cell_index`: Subtracts 32 from d0 and 49 from d1, divides each by 48, combines the results as the column plus three times the row, and stores the sum as a word in a global variable before calling a helper and returning.
- `dispatch_grid_cell_handler`: When the masked high bytes of d2 equal a fixed value, it derives a cell index from two divided coordinates in d1 and d0 (offset by constants, divided by 48, and combined as a row times three plus column), scales that index by four to look up a pointer in a table, and calls the routine at that pointer if it is non-zero.
- `calls_lookup_and_scan_routines`: Calls three routines in sequence, tests a global byte, and conditionally calls up to three more routines, returning after the final call.
- `calls_helpers_when_key_mismatch`: Loads two words from absolute addresses into d0 and d1, calls a helper, and when the masked high bytes of d2 are not equal to the constant a fixed address, loads a0 with an address and calls another helper before returning.
- `stores_scaled_lookup_byte`: Saves d0 and d1, reads a word from a global variable, calls a routine with the constant 15 in d1, stores the returned byte into a global variable and restores the registers.
- `copies_word_block_into_globals`: Calls several routines, then copies either 32 words or 16 words from a buffer pointed to by a0 into a global word array, with the 16-word case copying the buffer twice, depending on the signed value of a global byte.

The function list continues in: g2156-recovery-functions-2.md, g2156-recovery-functions-3.md, g2156-recovery-functions-4.md, g2156-recovery-functions-5.md.
