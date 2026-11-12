# Game 2162 (ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02162). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 635 of 635 named (0 author symbols), 635 of 635 described (69 by rule, 184 by the local model) |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Keymap | 1 | 3 | 1 |
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
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_BLIT | 11 | 252 | 6 |
| d | 1 | 3 | 1 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_CPU | 10 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon | 8 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon_install | 7 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |

## Cases handled

- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)(disk 01 of 10)(a)[a]/Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Keymap

- `stub_bad_instruction_data`: Contains only a bad-instruction trap with no usable instructions, so this is stub data rather than a working routine.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen 32-bit words from a fixed offset in a base register into a buffer whose address is passed on the stack, then calls a routine through an indirect pointer.
- `update_table_entry_and_copy_range_2`: Updates a structure or table entry with a new pointer and range-end value, and then copies a sequence of bytes from a source buffer to a destination buffer, terminating the copy with a specific byte value.
- `update_table_entry_and_copy_range`: Reads a byte from a table using an index stored in a structure, updates a pointer and a range-end value in that structure, and then enters a loop to copy a sequence of bytes from a source table to a destination table, calling another routine during the process.
- `update_structure_and_branch_offset_2`: Updates a structure or table by calculating a new range, then iterates through a loop to copy bytes from a source table to a destination table using an offset-based index, and finally updates a pointer or value at a specific offset in the structure.
- `update_structure_and_branch_offset`: Updates a table or structure using an offset-based index, copies certain longwords between structure members, and then enters a loop that checks a value against a counter, potentially calling another routine and updating further structure members or a global variable.
- `increment_d1_with_offset`: Sets d1 to one and adds a longword from a structure offset 84 of a1 to d1, then branches to another routine.
- `test_structure_member`: Tests if a longword at offset 40 of a1 is zero, otherwise it branches to another routine.
- `mask_structure_bits_and_compare`: Checks if certain bits are set in a structure member at offset 48 of a1, and if not, sets the upper 24 bits of that member to all ones.
- `update_structure_pointer_and_clear_table`: Updates a pointer in a structure, calls another routine, and if the new pointer value is non-zero, proceeds to update a table entry, reset several fields in a structure, and conditionally decrement a value in a table or array based on a pointer check.
- `update_structure_pointers_2`: Updates a pointer in a structure, calls a routine, and conditionally clears or decrements a value in a table or array based on a pointer check.
- `update_structure_pointers`: Updates a pointer in a structure, calls a routine, and conditionally clears or decrements a value in a table or array based on a pointer check.

### Install

- `scan_list_and_install_entry`: Walks a linked list to find the largest length field, then sets up a block sized from that value with a repeating fill pattern.
- `copy_memory_block_3`: Copies a fixed count of words from a memory location to a destination buffer if the destination pointer is not null, before transferring control to another routine.
- `load_and_check_pointer`: Loads an address into a1, calls a library routine, and if d0 is not zero, loads a pointer from a structure in a0 into a0 and then returns.
- `set_d0_and_branch`: Sets d0 to negative 256 and then branches to another routine.
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

- `find_max_and_build_buffer`: Iterates through a linked list to find the maximum value in a field and, if a condition is met, writes a header followed by a sequence of incrementing values and a copied block of data into a buffer.
- `copy_memory_block_4`: Copies a fixed number of words from a source pointer to a destination pointer.

### Delete

- `update_linked_list_header_and_copy_data`: Finds the maximum value in a linked list, writes a new header containing this value and an incrementing counter, then copies a block of data from one memory location to another if certain conditions are met.
- `copy_small_data_block`: copies a fixed number of words from a memory location offset from a pointer to a stack-based or temporary buffer.
- `update_table_index_and_traverse_2`: Calculates a new index or pointer for a table in a1, clears certain fields in a1 and a2, and calls another routine to check or update a secondary table or list, potentially entering a loop that traverses or updates entries based on values read from a table in a0.
- `update_list_index_from_table_2`: Reads a value from a table using an offset, calculates a new index, and stores two bytes or a word into a structure at offset 544 and 548, then branches to another routine.
- `execute_two_subroutines_with_offset`: Increments a value in a structure pointed to by a1, then calls another routine using a pointer from a4, and repeats this process using a different value from a1, before branching back to a previous instruction.
- `process_list_with_conditional_call`: Increments a value in a structure, then checks if a certain value in a list or a global variable matches a certain condition, calling another routine if certain values are encountered or if a list-traversal condition is met.
- `validate_and_increment_list_index_2`: Checks if a value at offset 544 of a structure in a1 matches certain constants, and if so, branches to other routines; otherwise, it increments a value at offset 544 of the structure in a1 and continues execution.
- `update_table_entry_and_branch`: Increments a byte at an offset of 536 in a1, compares a longword from a2 to a longword in a1, and if a certain condition is met, loads a byte from a table using a scaled index and jumps to a routine in a6.
- `branch_on_null_pointer`: Checks if a longword at an offset of 640 in a structure pointed to by a2 is zero, otherwise it jumps to another location, then clears d1 and jumps to the address in a6.
- `update_pointer_and_validate_index`: Increments a counter in a structure, then enters a loop that reads an index from a table and checks if that index or a subsequent byte matches certain constant values, updating a global variable if a match is found or calling another routine if certain conditions are met; otherwise, it continues or returns via an indirect jump.
- `branch_to_target_address_4`: Performs a short branch to a relative offset, effectively jumping to a fixed destination.
- `update_pointer_and_branch_conditionally`: Calculates a new index by adding a value from offset 548 of a structure in a1 to a constant, uses this to fetch a byte from a table in a0, and stores it in offset 552 of a1.
- `validate_and_update_index_table`: Checks if a value loaded from a structure offset against a list of prohibited or permitted constants, and if a match is found, updates a value in a structure and calls another routine; otherwise, it enters a loop that iterates through a table, updating structure members and calling another routine if certain byte values are encountered.
- `add_offset_to_d1_2`: Sets d1 to one, adds a longword from a structure at offset 544 in a1 to d1, then jumps to another location.
- `jump_to_offset`: Performs a relative jump to a certain offset.
- `update_global_pointer`: If a certain long-word is not zero, it sets a global variable at offset 676 of a structure in a2 to a constant value and then jumps to another routine.
- `search_and_update_list`: Copies a 32-bit value from a structure in a2 to offset 4 of a1, then loops to check if a value at a certain offset in a0 matches a value in a1, updating a pointer in a1 if a match is not found.
- `update_and_store_pointer`: Increments a 32-bit value at an offset of a structure in a2, calculates a new address using a value from a structure in a2 and a pointer from a1, stores a value at that address, and jumps to an indirect address in a6.
- `set_index_to_max`: Compares a value from a structure in a2 with another value from a structure in a2, and if the first is not greater, sets a value at offset 624 of a2 to negative 255 and branches to another routine, otherwise it calls another routine.
- `update_index_and_load_byte`: Increments a 32-bit value at an offset in a structure, then loads a byte from a table into another offset in that structure using a calculated index.
- `execute_conditional_calls`: Compares a value in a structure to a constant and calls a routine twice if they match, otherwise calls the routine once and jumps to an indirect address.
- `copy_and_call_routine`: Copies a long-word from a structure in a2 to a pointer in a1, copies another long-word from a structure in a2 to an offset of a1, modifies a pointer in a4, and calls another routine.
- `set_global_to_max`: Sets d1 to negative one, stores this value as a longword into a global variable at offset 668 in a2, then jumps to the address in a6.
- `execute_two_routines`: Calls a routine twice with different pointer offsets in a4 and jumps to an indirect address.
- `update_pointer_and_set_value`: Calls a routine, updates a pointer, and if a certain value in a structure does not match a constant, sets a 32-bit value in a structure to a negative constant before calling another routine.
- `branch_on_index_value`: Checks a value from a structure in a1 against several constants, calling other routines or setting a value in a structure in a2 if certain conditions are met, and finally jumps to an indirect address.
- `update_structure_member_on_condition`: Clears a 4-byte value at an offset of 4 from a pointer in a1 and calls another routine, then checks if a value at offset 656 of a structure in a2 matches certain constants; if certain conditions are met, it calls another routine, updates a pointer in a1, and performs a memory write or further routine calls before jumping to a routine in a6.
- `modify_and_store_byte`: Calculates an offset using a value from a structure in a2 and a pointer in a1, stores a byte from a structure in a2 into a memory location, copies a long-word from a structure in a2 to a pointer in a1, modifies a pointer in a4, and calls another routine.
- `load_and_call_routine`: Loads a value from a structure in a2 into d2, loads a value from a structure in a1 into d1, sets a4 to an offset of a4, and calls another routine before branching to another routine.
- `nop_and_jump`: Performs a no-operation and then a relative jump to another location.
- `traverse_and_replace_list`: A loop that replaces a value in a list with a byte from a table and updates a pointer, continuing until a null pointer is encountered.
- `branch_to_target_address_3`: Stores a long-word from d1 into an offset of a pointer in a1, then if d1 is not zero, jumps to another routine; otherwise, it overwrites d1 with d2 and jumps to an address in a6.
- `jump_to_offset_2`: Performs a relative jump to a certain offset.
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

- `parse_command_arguments`: Allocates a message block and a task port, then splits the command argument text into tokens, expanding quoted strings and asterisk escapes for newline and escape.
- `store_result_and_reply_message`: Stores its argument as the result in a message record, then frees the record's resources and replies to the message port with the stored result.
- `sum_sizes_matching_flags`: Walks a linked list of memory entries and adds the size of each entry, taken as its end minus its start, whose attribute word shares a bit with the mask argument.
- `print_memory_availability_table`: Prints a table with the columns type, available, in-use, maximum and largest, choosing chip, fast or total as the row from the first argument and printing a usage line when the argument is wrong.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_args_to_stack_slot`: Forwards its two arguments to a second routine together with the address of a stack slot, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `format_stack_text_and_emit`: Calls exec's RawDoFmt to fill a 140-byte stack buffer, then measures the text up to its terminator and passes the text and its length to an output routine when the length is nonzero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### addbuffers

- `set_or_report_drive_buffers`: Parses a drive name argument, rejects a name that is not an existing device with an invalid-name message, and then reads or adjusts that device's buffer count, printing the count.

### Path

- `update_linked_list_header_and_buffer`: Finds the maximum value in a linked list, then writes a header containing this value and a sequence of incrementing or constant values into a memory block.
- `copy_block_and_call_subroutine`: copies a fixed number of words from a memory location pointed to by a register into a stack-allocated buffer and then executes a function pointer previously held in a register.

### Resident

- `calculate_max_and_reconstruct_list`: Iterates through a linked list to find the maximum value in a specific field, then writes a header and reconstructs a new list or buffer containing updated or copied list nodes.
- `copy_block_and_execute_2`: copies a fixed number of words from a memory offset to the stack and then calls another routine.
- `validate_and_update_structure_pointer`: Calculates a new pointer or index, updates a field in a structure, and calls several subroutines to check or update further structure members; if certain conditions are met, it checks a bitmask in a table and updates a field before performing an indirect jump or calling another routine.
- `clear_d1_and_jump`: Clears d1 and jumps to the address in a6.

### Assign

- `calculate_max_and_populate_header`: Traverses a linked list to find the maximum integer value in a field, then writes a header or table entry containing this maximum value and a sequence of incrementing or fixed values.
- `copy_memory_block_5`: Copies a fixed number of words from a memory location to a destination pointer.
- `update_linked_list_node_and_table_2_2`: Calculates an index-based pointer, updates a structure or table with new bounds or pointers, and traverses a linked list or table-based chain, updating node pointers and checking for end-of-list or invalid index conditions, calling another routine to perform certain updates or checks during the traversal.
- `update_linked_list_node_and_table_2`: Updates a linked list node by copying a pointer from one index to another, then enters a loop that traverses and updates subsequent nodes in.
- `update_linked_list_node`: Updates a pointer in a structure or list, checks a condition or flag, and if certain criteria are met, updates a value in a structure or list to a constant or a value from another structure, then calls another routine.
- `update_linked_list_node_and_table`: Updates a linked list node by copying a pointer from one index to another, then enters a loop that traverses and updates subsequent nodes in the list.
- `assign_table_values`: Updates several values in a table using offsets derived from a structure, calls another routine multiple times to check or clear certain flags, and finally jumps to an address in a6.

### EndCLI

- `find_max_and_prepare_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then writes a header or a certain amount of data into a buffer.
- `copy_fixed_size_block`: copies a fixed number of dwords from a memory location offset by a pointer to a stack or memory buffer, then calls another routine.
- `modify_list_and_table`: Modifies a bitmask in a structure, updates a pointer in a structure, and conditionally decrements a value in a table if it is greater than zero.

### Rename

- `find_max_and_build_header`: Traverses a linked list to find the maximum value in a specific field, then writes a header or metadata block to a calculated memory location.
- `copy_block_to_stack`: copies a fixed number of dwords from a memory location offset from a pointer into a stack-allocated buffer before calling another routine.
- `update_pointer_and_call`: Updates a pointer in a structure, calls a routine, and conditionally updates a value in a structure or table.

### Dir

- `update_list_header_and_copy_data`: Finds the maximum value in a linked list, writes a new header containing this value and an incrementing sequence, and then copies a block of data from a source buffer to a destination buffer while updating a function pointer in the header.
- `copy_block_to_stack_2`: copies a fixed-size block of data from a memory offset to the current stack pointer.
- `update_entity_bounds_and_state`: Updates several pointer-based offsets in a structure in a1 to define a memory range, clears certain fields, and conditionally updates a structure in a2 with values from a1 or a table, then calls another routine or returns.
- `update_structure_field_from_registers`: If a longword at offset 332 of the structure in a1 is non-zero, it loads three values from offsets 88, 48, and 48 (the same offset) into d3, d2, and d1, then calls another routine using a pointer-adjusted a4, and finally writes the value from d1 into offset 320 of the structure in a1 before branching away.
- `clear_structure_field`: Tests if a longword at offset 352 of a1 is zero, otherwise it clears a longword at that same offset and branches to another routine.
- `update_entity_property_and_loop`: Updates a field in a structure using a value from another structure, then calls a routine multiple times with different offsets to update or check further fields, and finally loops through a range of indices to clear or set values in a table, or otherwise returns.
- `update_entity_state_and_pointer_2`: Checks a value in a table, and if it equals two, calls a routine to update a field in a structure, then updates a byte in a table using a pointer.
- `update_entity_state_and_pointer`: Checks a value in a table, and if it equals two, calls a routine to update a field in a structure, then updates a byte in a table using a pointer, and subsequently calls several other routines to update or reset various fields in the structure, including updating a pointer-based index and a byte-based index, before finally branching to another routine.
- `update_structure_range_and_set_value`: Checks if certain pointers in a structure in a2 and a structure in a1 are non-zero, then if a certain value is not one, calls a routine, and if a certain flag in a2 is zero, initializes a range of values in a structure in a1 by looping and setting each entry to one, before calling another routine and updating a value in a2 from a structure in a1.
- `set_structure_value_with_offset`: Calculates a pointer or offset by adding a value from a structure in a1 to d1, shifts it right by two, and calls a routine using a pointer from a2.
- `set_structure_value`: Calls a routine, and if a certain value is zero, sets a value at an offset in a structure.

### NewCLI

- `find_max_and_prepare_buffer_3`: Traverses a linked list to find the maximum integer value in a specific field and, if a certain condition is met, writes a header and copies a fixed number of words from a source buffer to a destination buffer, then updates a table entry with a function pointer.
- `copy_stack_block`: copies a fixed number of words from a memory location offset by a register or pointer into the current stack frame.
- `recalculate_bounds_and_copy_table_entries`: Updates a table of bounds or limits by recalculating or copying pointer-indexed values, and then performs a loop to copy a range of entries from one table or array to another, potentially incrementing a counter if a certain value is non-zero.
- `update_table_and_recalculate_bounds_2`: Updates a table entry by copying a value from one pointer-indexed location to another and increments a counter if it is non-zero, then calls another routine to perform further updates or resets on a structure, potentially involving a loop that searches for a zero-value entry in a table or updates a pointer-based bound or counter.
- `update_table_and_recalculate_bounds`: Iterates through a table, updating or resetting certain values, and if a certain condition is met, it updates a pointer or value in a structure and calls another routine multiple times.
- `increment_d1_with_offset_2`: Sets d1 to one and adds a longword from a structure offset 300 of a1 to d1, then branches to another routine.
- `update_table_and_decrement_logic_branch`: If a certain value in a structure is zero, it calls a routine three times with a constant value in d0 and a pointer from a table in a2; otherwise, it performs a series of pointer-based assignments and calls a routine multiple times, including a calculation that shifts a pointer-based address right by two bits before calling the routine again.
- `update_table_and_decrement_logic`: Checks if a value at an offset in a1 is below a certain threshold, and if so, performs several pointer-based assignments and calls a routine using a pointer from a2.
- `update_table_and_decrement`: Updates a value in a table, calls several routines, and conditionally clears or decrements a value in a table if a certain condition is met.

### NewShell

- `find_max_and_initialize_buffer`: Iterates through a linked list to find the maximum value in a specific field, then writes a header and copies data or function pointers into a destination buffer if certain conditions are met.
- `copy_block_and_execute`: copies a fixed number of words from a memory location offset from a pointer into a stack-based buffer and then calls another routine.
- `update_structure_index_and_table`: Updates a structure's index and table by copying values from a source table to a destination table, and increments or decrements a counter in a global structure if a certain condition is met.
- `sync_structure_index_and_update_table`: Checks if a table entry or structure index is non-zero, copies a value from one table entry to another, and increments or decrements a counter in a structure.
- `update_structure_index_and_sync_values`: Checks if a table entry is zero and updates it with a calculated value, then enters a loop that updates a structure's index or clamps a value to 800, while calling another routine and performing several conditional updates to a structure and a table.
- `increment_d1_with_offset_3`: Sets d1 to one and adds a longword from a structure offset 304 of a1 to d1, then branches to another routine.
- `sync_and_update_structure_value`: Checks if a value at offset 692 of a structure in a1 is zero; if not, it clamps a value at offset 276 of a1 to 800, synchronizes a secondary field at offset 268 with a value from offset 276, and calls a routine using a pointer from a2, then performs further updates to a table and structure using a pointer from a2 and a pointer from a4.
- `clamp_and_sync_structure_value`: Ensures a value in a structure does not exceed 800, synchronizes a secondary field with that value, and then executes a series of calls to update various structure members and table-based pointers or counters based on whether the primary value is zero or non-zero.
- `update_structure_and_table`: Updates a value in a structure, calls several routines, and conditionally clears or decrements a value in a table if a certain condition is met.

### Echo

- `find_max_and_prepare_buffer_4`: Iterates through a linked list to find the maximum integer value in a field, then writes a header containing this maximum value and an incrementing sequence into a destination buffer.
- `copy_fixed_size_block_2`: copies a fixed number of dwords from a memory location certain offset from a pointer to a stack-based or temporary buffer before calling another routine.
- `update_table_range_and_iterate_2`: Updates a range or pointer values in a structure by calling a routine to retrieve new indices, then enters a loop that iterates through a range of indices, updating a structure's range or pointer values.
- `update_table_range_and_iterate`: Updates a structure's range or pointer values by calling a routine to retrieve new indices, then enters a loop that iterates through a range of table entries, calling a routine at each step and updating a pointer or index in the structure.
- `copy_longword_to_offset`: Copies a longword from a structure offset 336 of a1 to a structure offset 328 of a1, then branches to another routine.
- `adjust_list_bounds_and_iterate`: Sets a longword at offset 328 in a1 to 1, checks certain values in a structure, and if certain conditions are met, calculates a new index and branches back to an earlier instruction or to another routine.
- `copy_longword_to_offset_2`: Copies a longword from a structure offset 336 of a1 to a structure offset 332 of a1, then branches to another routine.
- `update_list_bounds_and_iterate`: Updates two pointer-sized values in a structure, checks if a certain range is valid, and if so, iterates through a table to read a byte from an offset calculated from a value in the structure, then calls another routine and updates a value in the structure before branching.
- `check_structure_and_call`: Checks a value at an offset in a1, and if it is not zero, calls a routine using a pointer from a2.
- `conditional_routine_calls`: Checks a value in a structure and calls a routine or a sequence of routines if certain conditions are met.

### cls

- `open_dos_and_check_status`: Opens dos.library through exec, then calls two dos functions through the library base.

### Execute

- `update_max_list_header_and_copy_data`: Traverses a linked list to find the maximum value in a field, then writes a header and a sequence of incrementing values into a buffer.
- `copy_list_field_to_stack`: copies a fixed number of dwords from a memory offset or linked list field to the stack.

### Copy

- `update_linked_list_header_and_copy_buffer`: Finds the maximum value in a linked list and writes a new header containing this value into a calculated table position.
- `copy_memory_block`: copies a fixed number of dwords from a source pointer-offset to a destination stack or memory location.
- `update_table_index_and_traverse`: Calculates a new index or size for a table entry in a structure, clears certain fields in a secondary structure, and then enters a loop that traverses a table of pointers, checking for specific byte-encoded command or jump values and calling another routine if certain conditions or specific values are met.
- `update_list_index_from_table`: Calculates a new index using a value from a table, retrieves two bytes from a data structure using this index, and stores these bytes into a structure at offset 544 and 548, then branches to another routine.
- `execute_two_subroutines_with_offset_2`: Increments a value in a structure pointed to by a1, then calls another routine using a pointer from a4, and repeats this process using a different value from a1, before branching back to a previous instruction.
- `process_list_with_conditional_call_2`: Increments a value in a structure, then checks if a certain value in a list or a global variable matches a certain condition, calling another routine if certain values are encountered or if a list-traversal condition is met.
- `validate_list_index_type`: Checks if a value loaded from an offset in a structure matches certain predefined constants, branching to different subroutines if a match is found or otherwise continuing execution.
- `copy_range_to_structure`: Increments a value at an offset of 536 from a1, compares a value from a2 to that new value, and if it is not greater or equal, checks a byte from a table using a shifted offset from a1 and jumps to a routine in a6 if a certain condition is met.
- `check_long_value`: Tests if a long-word in a structure is non-zero, otherwise it jumps to another routine and then jumps to a location held in a6.
- `update_index_table_and_validate_constants`: Increments a counter in a structure, then iterates through a list of offsets to check if the bytes loaded from a table match certain permitted or prohibited constants, updating a global value if a match is found or calling another routine if certain conditions are met; otherwise, it returns or jumps to another location.
- `branch_to_offset_target`: Performs a short branch to a relative offset, effectively redirecting execution flow to a target address.
- `update_pointer_value_and_verify`: Updates a value in a structure at offset 552 using a byte read from a table, then compares this new value against a global variable at offset 700; if they match, it performs a secondary table lookup and calls another routine.
- `validate_and_branch_on_index`: Checks if a value loaded from a structure offset against a set of hardcoded constants and, if a match is found, either calls another routine or performs a branch to a fixed destination.
- `add_offset_to_d1`: Sets d1 to one, adds a long-word from a structure in a1 to d1, and then branches to another routine.
- `branch_to_target`: Performs a relative branch to another routine.
- `set_constant_d2`: Sets a 32-bit value in a structure at offset 424 of a2 to a constant and then jumps to another routine.
- `compare_and_update_pointer`: Compares a value from a structure in a1 with d1, and if certain conditions are met, calculates a new address using a value from a structure in a2 and calls a routine.
- `increment_and_copy_value`: Increments a value in a structure and copies a word from a source pointer to a destination pointer in a table.
- `compare_and_set_constant`: Compares a value from a structure in a2 with another value from a structure in a2, and if certain conditions are met, sets a value in a structure in a2 to a constant and branches.
- `increment_and_store_byte`: Increments a value in a structure and stores a byte from a source pointer into a destination pointer in a structure.
- `check_table_and_call`: Compares a value in a structure to a constant and calls a routine if they are equal.
- `copy_structure_data`: Copies a 32-bit value from a structure in a2 to a pointer in a1, copies another 32-bit value from a2 to an offset in a1, and calls another routine before jumping to another routine.
- `set_range_and_jump`: Sets a long-word at an offset of 2c8 in a structure in a2 to negative one, loads a long-word from a pointer in a1 into d1, and then jumps to a location held in a6.
- `execute_two_routines_2`: Calls a routine twice with different pointer offsets in a4 and jumps to an indirect address.
- `update_conditional_value`: Calls a routine, updates a pointer-based value, and if a certain value in a structure in a2 does not equal 41, sets a value at offset 704 of that structure to -1 before calling another routine and jumping to an indirect address.
- `validate_and_branch_on_type_id`: Checks a longword at offset 4 of a structure against several constant values; if the value matches certain constants, it calls another routine, potentially updates a pointer or a global variable, and then jumps to an indirect address; otherwise, it sets a global variable to negative one and jumps to an indirect address.
- `execute_subroutine_loop`: Clears a 4-byte value at an offset of 4 in a pointer-based structure, calls another routine, and then enters a loop that updates a 4-byte value at an offset of 8 in that structure, continuing or branching based on a comparison with a value in a structure in a2, before calling another routine and jumping to a location in a6.
- `update_pointer_and_store`: Calculates an offset using a value from a structure in a2 and a value from a structure in a1, stores a byte from a structure in a2 at an offset in a0, copies a 32-bit value from a2 to a1, and calls another routine before jumping to another routine.
- `load_parameters_and_call_subroutine`: Loads a longword from a global variable into d2 and a longword from a structure offset 8 into d1, sets d0 to a constant, updates a pointer in a4, and calls another routine before branching to a different code location.
- `nop_and_branch`: Performs a no-operation and then branches to another routine.
- `copy_linked_list_bytes`: Loops through a list of bytes, copying a byte from a source table to a destination, and updates a pointer in the list, continuing until a null pointer is encountered, then jumps to an indirect address.
- `branch_to_target_address_3_2`: Stores a long-word from d1 into an offset of a pointer in a1, then if d1 is not zero, jumps to another routine; otherwise, it overwrites d1 with d2 and jumps to an address in a6.
- `branch_to_target_2`: Performs a relative branch to another routine.
- `copy_and_replace_bytes`: Loops through a list of bytes, copying a byte from a source table to a destination, and updates a pointer in the list, continuing until a zero byte is found, then copies a byte from a different offset and jumps to an indirect address.
- `clear_and_populate_list`: Clears several 32-bit values in a structure, loads a byte from a table using a pointer and an offset, and then loops to clear a range of bytes in a table.
- `execute_three_subroutines`: Calls a routine three times with different offsets of a4, then negates a value from a structure in a2 and jumps to an indirect address.
- `copy_and_sum_bytes_2`: Reads bytes from a table using offsets in a structure, stores them in a structure, and enters a loop that copies bytes from one table location to another while updating a counter and potentially calculating a sum of bytes.
- `store_sum_to_table_2`: Calculates the sum of two 32-bit values from a structure and stores the lower byte of that sum into a table at an offset determined by a third value in the structure.
- `copy_and_call_routine_2`: Copies a byte from a table to another table location, updates a structure, and enters a loop that calls a routine via a pointer in a5 and copies a byte from a table to another table location.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `swap_and_store_pointer_2`: Calls a routine, sets a value in a structure, and if a certain value is zero, calls another routine twice and sets a pointer to a new address.
- `swap_and_store_pointer`: Copies a 32-bit value from a source table to a location in a structure in a2, then copies a value from a list into that source table, and finally updates a pointer in that structure, then jumps to an indirect address.

### Makedir

- `find_max_and_initialize_header`: Iterates through a linked list to find the maximum value in a field, then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_memory_block_2`: Checks if a pointer is non-null and copies a fixed number of words from a source pointer to a destination pointer.
- `execute_conditional_logic_2`: Performs several routine calls and updates various offsets in a structure based on values retrieved from a source table.
- `execute_conditional_logic`: Performs several routine calls and updates various offsets in a structure in a1 based on values retrieved from a source table, including conditional logic that sets a value to 10 if a certain check fails, before jumping to an indirect address.

### List

- `find_max_and_prepare_buffer_5`: Iterates through a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a calculated buffer.
- `copy_fixed_size_block_3`: copies a fixed number of words from a memory location offset from a pointer into a stack-based or local buffer, then calls another routine.
- `update_table_and_traverse_links`: Updates a structure's size or index field, then enters a loop that traverses a linked list or table of pointers, checking for specific byte-encoded command or jump values and calling another routine if certain conditions or specific command bytes are encountered.
- `update_list_index_and_cache_values`: Calculates a new index using a base offset and a value from a table, then reads two bytes from a structure at that calculated offset and stores them into a structure at offsets 544 and 548, before branching to another routine.
- `update_list_index_and_branch`: Increments a byte at offset 536 of a structure in a1, calculates a new index using a value from a structure in a1, and calls another routine twice before branching back to the start of the routine.
- `process_list_index_and_traverse_table`: Increments a byte at offset 536 of a structure in a1, then checks if a longword at offset 548 of that structure is non-zero to decide whether to call another routine.
- `filter_list_index_type`: Checks if a value loaded from an offset in a structure matches certain predefined constants, and if so, branches to different subroutines or a common exit point.
- `copy_range_to_structure_alt`: Increments a value at an offset of 536 from a1, compares a value from a2 to that new value, and if it is not greater or equal, checks a byte from a table using a shifted offset from a1 and jumps to a routine in a6 if a certain condition is met.
- `test_and_branch_on_zero`: Tests if a long-word at an offset of 644 from a2 is zero, otherwise it jumps to a different location, then sets d1 to zero and jumps to the address in a6.
- `validate_and_update_index_table_2`: Iterates through a list of indices, checking if certain bytes loaded from a table match a set of permitted or prohibited constants, and if a match is found, updates a value in a structure and calls another routine; otherwise, it continues or returns via an indirect jump.
- `branch_to_offset_target_2`: Performs a short branch to a relative offset, effectively redirecting execution flow to a target address.
- `update_pointer_and_branch_on_value`: Calculates a new index by adding a value from offset 548 of a structure in a1 to a base offset, retrieves a byte from a table in a0 using this index, and stores it in offset 552 of the structure in a1.
- `validate_and_filter_index_value`: Checks if a value loaded from a structure offset against a list of prohibited or allowed constants, and if a match is found, branches to a specific destination or calls another routine.
- `add_and_branch_to_target`: Sets d1 to one, adds a long-word from an offset of 544 of a1 to d1, then branches to another routine.
- `branch_to_target_address`: Performs a short branch to a fixed offset in another routine.
- `set_global_pointer`: Sets a global variable at offset 616 of a structure in a2 to a constant value, otherwise it returns.
- `update_and_store_pointer_2`: Sets a value in a structure, and then loops to compare a value from a table against a value in a structure, potentially updating a pointer and a table value.
- `increment_and_store_pointer`: Increments a value at offset 676 of a structure in a2, calculates an offset using a value from a structure in a2 and a pointer from a1, and stores a value from a1 at that calculated address before jumping to a target.
- `update_index_and_load_byte_2`: Compares a value from a structure and, if it is within a certain range, sets a value in a structure to a constant.
- `update_value_from_table`: Increments a value at offset 696 of a structure in a2, calculates an offset using a value from a structure in a2 and a pointer from a0, and stores a byte from that table into a value at offset 692 of a structure in a2 before jumping to a target.
- `conditional_subroutine_execution`: Compares a value at offset 692 of a structure in a2 with a constant, and if they are equal, calls a subroutine twice using a pointer or a value derived from a pointer before jumping to a target.
- `copy_structure_members`: Copies a 32-bit value from a structure in a2 to a pointer in a1, copies another 32-bit value from a structure in a2 to an offset of 4 in a1, then calls another routine and branches to a new location.
- `set_and_jump_to_pointer`: Sets d1 to negative one, stores this value as a long-word at an offset of 640 from a2, loads a long-word from a1 into d1, and jumps to the address in a6.
- `execute_two_subroutines`: Sets d0 to 32, loads a pointer from a4 into a4, calls a routine, loads a long-word from a1 into d2, sets d0 to 20, loads a pointer from an offset of 260 of a4 into a4, calls a routine, loads a long or word from a1 into d1, and jumps to the address in a6.
- `update_structure_with_subroutine_result`: Calls a subroutine, copies the result back to a pointer in a1, and if a value at offset 628 of a structure in a2 is not 41, sets a value at offset 640 of a structure in a2 to -1, then calls another subroutine before jumping to a target.
- `check_type_and_update_structure`: Loads a longword from a structure offset 4 in a1 and compares it against several constants, branching to other routines or performing a jump if certain values match or if a certain value in a structure at offset 576 in a2 does not match 41.
- `execute_conditional_logic_3`: Clears a 32-bit value at an offset of 4 in a pointer, calls another routine, and then enters a loop that calls another routine while checking if a value in a structure in a2 matches certain constants, or otherwise calls another routine and jumps to a location in a6.
- `update_table_and_call`: Calculates an offset using a value from a structure in a2 and a pointer in a1, copies a byte from a table in a0 to a pointer in a1, copies a 32-bit value from a structure in a2 to a pointer in a1, then calls another routine and branches to a new location.
- `load_and_call_subroutine`: Loads a long-word from an offset of 632 in a2 into d2, loads a long-word from an offset of 8 in a1 into d1, loads the constant 24 into d0, loads a pointer or address or offset 128 in a4 into a4, calls a routine using the address in a5, and branches to another routine.
- `nop_and_branch_2`: Performs a no-operation and then branches to another routine.
- `process_linked_list_and_swap`: Loops through a list where each element contains a pointer to the next, reading a byte from a table and writing it to a certain offset, and swaps a byte from a pointer in a1 into a table before checking if the pointer is null.
- `store_and_branch_on_d1`: Stores the contents of d1 into a structure at offset 8 in a1, then if d1 is not zero, branches to a nearby instruction, otherwise it copies d2 to d1 and jumps to the address in a6.
- `branch_to_target_address_2`: Branches to a fixed address in another routine.
- `traverse_and_replace_list_element`: Loops through a list, replacing a value in the list with a byte from a table, and once a certain byte is found, writes a byte from a pointer in a1 into a table before jumping to a target.
- `initialize_and_clear_table`: Clears several 32-bit or 16-bit values in a structure pointed to by a2, copies a byte from a table (indexed by d1 multiplied by 4) into a structure field, and then loops to clear a range of bytes in a table (indexed by d1 and a pointer from a2) until a certain value is reached.
- `execute_subroutines_and_invert_value`: Calls a subroutine three times with different parameters or pointers, and then calculates the bitwise NOT of a value at offset 640 of a structure in a2 before jumping to a target.
- `branch_to_target_3`: Performs a relative branch to a nearby instruction.
- `validate_value_against_constants`: Loads a value from a pointer in a1 and compares it against a list of constant values, otherwise it calls another routine.
- `set_d1_and_jump`: Sets d1 to negative one and jumps to the address in a6.
- `set_d1_to_zero_and_jump`: Sets d1 to zero and jumps to the address in a6.
- `check_value_against_list`: Loads a value from a pointer in a1 and compares it against several constant values, otherwise it jumps to another routine.
- `check_pointer_and_jump`: Checks if a pointer in a1 is null, otherwise it clears d1 and jumps to another routine.
- `process_list_and_call`: Copies a byte from a table in a0 to a pointer in a1, updates a value in a structure in a1, and if a certain condition is met, calculates a new offset, copies a byte from a table in a0 to a pointer in a1, and calls another routine.
- `add_structure_offset_to_d1`: Sets d1 to one, adds a value from a structure at offset 8 of a1 to d1, and branches back to a previous instruction.
- `set_d1_and_jump_2`: Sets d1 to zero and jumps to a location stored in a6.
- `reset_string_bounds_and_update_display`: Resets several string index bounds and pointer-based array limits in a structure pointed to by a1, calls a routine to initialize string bounds, and then updates certain display-related pointer or size fields in the structure before branching to another routine.
- `initialize_string_index_bounds`: Sets certain array elements in a structure pointed to by a1 to a default value and then uses a routine to check or copy data from a source string or block, potentially updating a global or structure-based index or terminator.
- `parse_string_and_update_index_2_2`: Reads a pointer from a structure, checks if the target string or data block is null, and if not, copies bytes from that source or a secondary source into a destination buffer while updating an index and a length or end-marker flag in the structure.
- `branch_to_string_parsing`: Performs a short branch to a location within another routine.
- `parse_string_and_update_index_2`: Parses a string or a sequence of bytes from a structure, updating several offset-based pointers and flags in a structure, and checks for a termination character or a specific length limit, otherwise it loops through the string or a pointer-based sequence.
- `parse_string_and_update_index`: Checks if a certain flag in a structure is zero and if a pointer in a table is non-null, then calls another routine to process a string.
- `update_table_and_rebuild_index_2`: Updates a byte in a table using an index-based offset, updates a pointer or counter in a structure, and then enters a loop that copies bytes.
- `update_table_and_rebuild_index`: Updates a byte in a table using an index-based offset, updates a pointer or counter in a structure, and then enters a loop that copies bytes from a source table to a destination table while updating a secondary index or pointer.
- `modify_table_byte`: Decrements a longword loaded from an offset of 1988 in a1, shifts a longword loaded from an offset of 1448 in a1 left by two, and stores the resulting byte into a table at a calculated offset in a0, then branches to another routine.
- `update_pointer_logic_and_display_index`: Clears a byte in a table using an index, checks a flag in a structure, and if certain conditions are met, calculates and stores a new pointer or a bitwise-inverted address into a structure, then calls a routine to update a pointer or display logic.
- `calculate_shifted_address`: Loads an address from a structure at offset 4648 of a4 into a3, copies it to d1, shifts d1 right by two, and branches back to a previous instruction.
- `update_pointer_logic_and_display`: Updates a pointer in a structure using an index, potentially updates a table entry with a calculated address, and executes a sequence of calls to a routine using a constant or a pointer-derived value, followed by conditional updates to a structure's field and a final check of a pointer-based value to update a certain field or set a constant value.
- `update_pointer_logic`: Calls another routine using a pointer from a structure, then checks a byte in a table; if the byte is zero, it calls another routine and updates a value in a structure using a pointer-based offset, otherwise it calls another routine and updates a pointer-based value in a structure.
- `update_display_bounds_and_offset_4`: Calculates a bit-shifted value from a pointer-based address, stores it in a structure, and calls another routine to check if the resulting value is zero; if zero, it resets a different structure field to zero using another routine call, otherwise it updates a pointer-based offset or boundary.
- `reset_display_bounds_and_offset_2`: Updates a display boundary or offset in a structure in a1 by calling another routine to retrieve a value, then checks if that value is zero to either reset a display parameter or calculate a new boundary or offset based on a pointer in a2 and calls several other routines to update display parameters or reset certain values.
- `reset_display_bounds_and_offset`: Checks if a display offset or boundary (stored at offset 584 of a structure in a1) is negative, and if so, updates a value in a structure (a), then resets certain display parameters or calculates a new display boundary or offset (via a call to another routine) and updates a structure (a).
- `update_display_bounds_and_offset_logic`: Checks if a display offset or boundary (stored at offset 584 of a structure in a1) is negative, and if so, updates a value in a structure (at offset 648 of a1) with the bitwise NOT of that offset.
- `update_scaled_display_offset`: Calculates a new scaled display offset by loading a value from a pointer in a1, shifting it left by two, and storing it into a structure offset in a0, then updates a pointer in a1 with a value from a2 and calls another routine.
- `update_structure_with_shifted_address`: Loads an address from a structure at offset 4788 of a4 into a3, copies it to d1, shifts d1 right by two, stores d1 into a structure at offset 604 of a2, and branches back to a previous instruction.
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
- `execute_indirect_call`: Sets d1 to 45, sets d0 to 24, loads an address from a structure at offset 224 of a2 into a4, calls a routine at the address in a5, and jumps to a location stored in a6.
- `update_display_index_parameters`: Updates a display index or offset by checking a pointer-based table, performing conditional checks on a structure's members, and updating a global value with a calculated or table-retrieved address or index.
- `test_and_mask_bit_one`: Tests if the least significant bit of a register is set and, if so, clears all but the first bit of that register before branching or moving a value into it.
- `execute_indirect_calls`: Loads a pointer from a structure in a2 into a4, calls the routine pointed to by a5, and then jumps to the routine pointed to by a6.
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

- `find_max_and_prepare_buffer_6`: Traverses a linked list to find the maximum value in a specific field and then writes a header containing this maximum value and a sequence of incrementing identifiers into a buffer.
- `copy_block_to_stack_3`: copies a fixed number of words from a memory location or structure member to the current stack pointer and then calls another routine.
- `update_structure_and_call_subroutines`: Updates a bit in a byte using a bit-shift, modifies several pointer-based offsets in a structure, and executes a sequence of conditional calls to other routines based on the contents of a pointer-indexed table or a structure-relative offset.
- `check_structure_and_call_2`: Calculates a bit-shifted address from a4, calls another routine, and if a certain value at an offset in a1, scaled by a pointer, is non-zero, calls another routine and updates a global variable with a value from a pointer in a2.
- `set_buffer_and_draw`: Loads a pointer from a2 plus 1544, calls a routine with a length of 720, and if the loaded value is zero, calls another routine and sets a value at a1 plus 704.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `update_header_buffer_and_copy_data`: Finds the maximum value in a linked list, writes a new header or certain count-based metadata to a buffer, and copies a block of data from an external source or a stack-based buffer into that newly prepared header area.
- `copy_buffer_from_pointer`: Copies a fixed number of words from a memory location to a destination in the stack.
- `update_object_state_and_indices`: Updates a pointer in a structure, clears several fields at certain offsets, and then checks a series of table-indexed values to conditionally update or reset other fields in the structure, including checking for a specific character '$' in a table.
- `check_table_index_and_set_value_2`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a structure in a0; if that byte is non-zero, it compares a value of 10 in d2 against a longword at offset 548 in a2 and, if less, sets d1 to 255 before branching to another routine.
- `check_table_index_and_set_value`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a structure in a0; if not zero, it compares a value in a2 to 20 and, if less than, sets d1 to a certain value before branching.
- `traverse_and_update_structure_pointers`: Traverses a linked-list-like structure or table-based hierarchy by updating pointers in a structure in a1 based on values found in a table in a0, and calls another routine if certain character or pointer conditions are met.
- `validate_and_update_structure_state`: Checks if certain characters in a table-driven structure match a specific character, and if so, calls another routine to update or reset structure members.
- `branch_to_target_if_non_zero`: Performs a conditional branch to a target address if a longword at offset 40 of the structure pointed to by a1 is non-zero; otherwise, it returns immediately via a relative branch.
- `test_structure_member_and_branch`: Loads a longword from a1, shifts it left by 2, and checks a byte at that offset in a0; if not zero, it compares a longword at offset 388 in a1 to 1 and, if not equal, sets d1 to a certain value and branches.
- `branch_if_non_zero_member`: Performs a conditional branch to a distant instruction if a longword at offset 40 of the structure pointed to by a1 is non-zero; otherwise, it returns immediately.
- `test_structure_member_bits_branch_logic`: Checks if a longword at offset 356 of the structure in a1 is equal to one, and if not, tests if that same longword is zero to determine whether to branch to another routine, branch to a different routine, or branch to a third routine with d1 set to a negative value.
- `update_object_index_and_call_subroutines`: Checks if a pointer-indexed array entry is non-zero, calls a routine, and updates a field in a structure with the returned pointer-index.
- `reset_and_initialize_object_state`: Resets a flag in a structure, checks a pointer-indexed array, and if certain conditions are met, clears a field, sets a counter to ten, and calls another routine.
- `update_object_bounds_and_visibility`: Updates a size or count in a structure, then checks or updates bitmasks or visibility flags in a structure, potentially looping to update a value in a global variable until a certain condition or limit is met, and finally calls another routine.
- `update_object_bounds_and_visibility_2`: Calls another routine to update a value in a structure, loops to check if that value is certain constants, and if certain flags in a1 are zero, calls another routine to update a value in a structure and calls another routine to update a bitmask in a structure.

### LoadWB

- `parse_args_and_open_handle`: Allocates a message and task record, then splits the argument text into tokens, expanding quoted strings and asterisk escapes for newline and escape.
- `close_handles_and_reply`: Stores its argument as the result in a message record, closes a DOS handle and a library when they are set, frees the record's memory, and replies to the message port with the stored result.
- `start_workbench_tool`: Opens the library dependencies through exec, applies a byte-wise XOR pass to a string twice so it is left unchanged, and starts a Workbench tool.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte.
- `build_node_list_from_table`: Walks a table of pointer and value pairs, allocating a cleared eight-byte node for each pair and storing a value looked up through a library call in it.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `invoke_library_vector_132`: Loads a library base from a global variable and calls the library function at offset minus 132 from it, with no arguments and no result.
- `invoke_library_vector_138`: Loads a library base from a global variable and calls the library function at offset minus 138 from it, with no arguments and no result.
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
- `invoke_exec_dispatch`: Calls the exec Dispatch function through the library base held in a global variable and returns nothing.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is unable to be found.
- `store_low_byte_at_pointer`: Stores the low byte of the data register at the address held in an address register.
- `compare_string_prefixes`: Checks if a2 is null or certain, then checks if a byte at a2 plus 20 is '2'. If so, it compares a string at a2 with a global string, potentially padding with dots if a null terminator is found.
- `write_period_and_spaces`: Writes a period followed by four spaces into a buffer at offset seventeen, one byte at a time.
- `advance_while_prefix_matches`: Compares a global zero-terminated string with the bytes at a pointer, advancing both until the global string ends or a byte differs.
- `return_without_action`: Contains only a return and performs no observable operation.

### Version

- `parse_directory_string`: Parses a directory path string or string literal, handling quoted strings and converting certain escape-like sequences, and stores the resulting tokens into a message record or a temporary buffer.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version_against_args`: Parses the command arguments for a library or device name with optional version and revision thresholds, opens it, and compares its version and revision with those values.
- `call_library_vector_210`: Calls the library function at offset minus 210 from a global library base with the first argument and the value 128, then calls a second routine with the other argument.
- `parse_signed_decimal_string`: Converts a string to an integer by reading an optional plus or minus sign and then decimal digits, applying the sign to the result.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `measure_null_terminated_length`: Returns the number of bytes before the zero terminator of a string by scanning forward until a zero byte is found.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_dispatch_function_2`: Calls a function pointer retrieved from an offset in a global variable.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `invoke_library_vector_198`: Loads a library base from a global variable and calls the library function at offset minus 198 from it, with no arguments and no result.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `invoke_library_vector_414`: Loads a library base from a global variable and calls the library function at offset minus 414 from it, with no arguments and no result.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `invoke_library_vector_552`: Loads a library base from a global variable and calls the library function at offset minus 552 from it, with no arguments and no result.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_register_to_memory`: Stores the contents of a register into a memory location pointed to by another register.

### SetMap

- `parse_args_and_open_handle_2`: Allocates a message and task record and splits the argument text into tokens, expanding quoted strings and asterisk escapes.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `execute_dispatch_callback`: Executes a function pointer located at a fixed offset from a global address.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `invoke_exec_reschedule`: Calls exec's Reschedule function through the library base held in a global variable and returns nothing.
- `release_stored_table_entries`: Walks a counted table of stored values from the last entry down to the first, passing each nonzero value, shifted right by two bits, to a release routine.
- `store_arguments_then_call`: Stores its two arguments in global variables, then makes one routine call with eight arguments including the values 440 and 60, and finishes with a call that passes the value 20.
- `load_console_keymap_from_args`: Takes a keymap name from the arguments or the icon tool types, lowercases it, and opens the keymap resource and console device.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_function`: Invokes a function pointer located at a fixed offset from a global address.
- `invoke_library_vector_450`: Loads a library base from a global variable and calls the library function at offset minus 450 from it, with no arguments and no result.
- `call_library_subroutine`: Executes a function pointer stored at a specific offset relative to a global variable.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `invoke_library_vector_294`: Loads a library base from a global variable and calls the library function at offset minus 294 from it, with no arguments and no result.
- `invoke_library_vector_210`: Loads a library base from a global variable and calls the library function at offset minus 210 from it, with no arguments and no result.
- `invoke_library_vector_444`: Loads a library base from a global variable and calls the library function at offset minus 444 from it, with no arguments and no result.
- `execute_indirect_function`: Calls a function pointer retrieved from a global structure offset.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `invoke_library_vector_366`: Loads a library base from a global variable and calls the library function at offset minus 366 from it, with no arguments and no result.
- `invoke_library_vector_384`: Loads a library base from a global variable and calls the library function at offset minus 384 from it, with no arguments and no result.
- `invoke_library_vector_96`: Loads a library base from a global variable and calls the library function at offset minus 96 from it, with no arguments and no result.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_with_global_and_stack`: Passes its argument, a value held in a global variable and the address of a stack slot to a routine that calls the library function at offset minus 132.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable and then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_address`: Stores a byte from a register into a location pointed to by a register.

### More

- `parse_filesystem_path`: Parses a string representing a file or directory path, handling quoted strings and converting certain escape sequences like newline or escape characters, while populating a structure with pointers to the parsed path components.
- `update_state_and_run_callbacks`: Updates a state field in a structure via a pointer retrieved from a global variable and executes a sequence of conditional cleanup callbacks or task-related cleanup routines, passing a specific value or identifier as a parameter or context before each call.
- `page_through_file_contents`: Opens a window and shows a named file page by page, with a filename prompt, a percentage indicator and a More prompt, and it quits when the q key is pressed.
- `pass_global_words_to_routine`: Calls another routine four times, passing a structure member from its argument together with each of four consecutive global array words in turn.
- `validate_buffer_text_bytes`: Calls a routine twice with buffer arguments derived from its input, then scans up to eighty bytes of the buffer.
- `dispatch_pager_key_command`: Waits for input with a timeout, then reads one key and maps its code to an action such as paging, searching, moving or quitting.
- `execute_page_routines`: Checks if a value at a2 plus 60 is positive, then calls a routine using a2, calls another routine using a2, and jumps back to the start.
- `call_two_routines_then_indirect`: Calls one routine with a global variable, calls a second routine with no arguments, then calls through an address register and returns.
- `initialize_scan_state_and_loop`: Sets a counter field to one and a flag field to zero, stores the result of a library call in the structure, and calls a routine on it.
- `combine_results_and_increment_counter`: Calls one routine on a buffer in its argument, passes that result with the buffer to a second routine, and stores the final result in a field.
- `scan_line_and_mark_formfeed`: Reads up to one hundred twenty-eight bytes through a library call and finds the first newline or form-feed character.
- `show_page_lines_until_quit`: Repeats a display pass up to a line count computed from its structure, printing table entries while the scroll position is within range, then shows the More prompt.
- `run_search_pass_until_found`: Chooses a search prompt from a flag, runs search passes while a counter remains positive, and stops when a pass reports a match.
- `run_sequence_and_return_result`: Makes a fixed sequence of calls that prepares a block, runs one routine with the caller's arguments and then finishes the block, returning the result of that middle call.
- `call_pair_then_optional_single`: Calls a routine with two arguments, then calls it again with the first argument alone when a short flag in the block is zero.
- `call_with_block_fields`: Calls a routine with two fields from the block and the value minus one, then calls a second routine with a short field from a nested structure.
- `call_pair_with_decremented_field`: Calls a routine with the block's handle, zero and one, then calls a second routine with a short field from a nested structure minus one.
- `print_page_lines_from_buffer`: Prints up to a set number of lines from a text buffer, scanning up to one hundred twenty-eight bytes at a time for newline and form-feed characters to count lines.
- `forward_block_call_with_global`: Calls a routine with the second argument, then calls a routine whose address is taken from a field of the block, passing a global value and the first argument.
- `set_flag_and_forward`: When a field in the block is zero it sets a short flag in a nested structure to one.
- `dispatch_by_flag_argument`: Calls a routine from a field of the block with a global value, passing a second argument that is either a global or a short field from a nested structure plus one, depending on a flag in the block.
- `dispatch_text_output_by_flag`: Calls a routine from a field of the block with either three global references or a global reference and a short field from a nested structure, depending on a flag in the block.
- `clamp_negative_result`: Calls a routine twice with the same first argument, once with zero and one and once with zero and minus one, then returns the second result with negative values clamped to zero.
- `copy_buffered_bytes_forward`: Copies up to a requested number of bytes from a buffer region in the block to the destination, limited to the bytes still available.
- `refill_buffer_and_reset_offsets`: Returns zero while the buffer is partly filled, and when it is full calls a routine to write out the buffered data.
- `update_viewport_position`: Computes a target position from the offset argument, which is absolute, relative to the current position or relative to the end, and checks it against the visible window bounds.
- `read_line_into_buffer`: Reads characters one at a time through a routine into a destination buffer until a newline or a limit of roughly 127 characters is reached.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination until and including the zero terminator.
- `convert_string_to_uppercase`: Walks a zero-terminated string and changes each lowercase letter from a to z to its uppercase form, leaving other bytes unchanged.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_with_mask`: When a pointer argument is nonzero and a field in the structure is set, calls the library routine held in that field with the pointer and minus one.
- `process_entry_and_write_text`: Processes an entry by calling routines on its linked fields, then writes either the given text or the entry's own text to its output handle.
- `pass_single_word_to_library`: Stores its argument in a one-word stack buffer and passes it, with the constant 994 and a size of one, to a library function reached through a pointer from another routine's result.
- `copy_fields_from_allocated_block`: Allocates a cleared 36-byte block and passes its shifted address with the constant 25 to a library function.
- `allocate_linked_node_with_words`: Allocates a cleared 68-byte node, links it to a looked-up handle and copies a count of words from the source into it.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `execute_indirect_function_pointer`: Loads a function pointer from a location relative to a global pointer and executes it.
- `invoke_exec_switch`: Calls exec's Switch function through the library base held in a global variable and returns nothing.
- `execute_indirect_function_2`: loads a function pointer from a table and executes it.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `invoke_exec_allocmem`: Calls exec's AllocMem function through the library base held in a global variable and returns nothing.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `execute_indirect_function_3`: loads a function pointer from a global table using a fixed offset and executes it.
- `call_indirect_handler`: Loads a function pointer from a global structure and calls it.
- `call_indirect_handler_2`: Executes a routine located at an offset within a global structure.
- `call_indirect_handler_3`: Performs an indirect call to a routine pointer stored in a global table.
- `call_indirect_handler_4`: Calls a routine via a pointer retrieved from a global memory location.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_indirect_handler_5`: Retrieves and calls a function pointer from a global structure offset.
- `invoke_library_vector_372`: Loads a library base from a global variable and calls the library function at offset minus 372 from it, with no arguments and no result.
- `call_indirect_handler_6`: Invokes a routine stored in a global pointer or table.
- `call_indirect_handler_7`: Executes a routine pointer located at a fixed offset from a global variable.
- `call_indirect_handler_8`: Performs an indirect call using a pointer from a global structure.
- `invoke_library_vector_624`: Loads a library base from a global variable and calls the library function at offset minus 624 from it, with no arguments and no result.
- `recurse_with_stack_slot_2`: calls another routine passing two parameters and a pointer to a specific stack location.
- `read_one_signed_byte`: Calls a routine with a small stack buffer and the count one, returning the byte as a signed value when one byte was transferred and minus one otherwise.
- `invoke_and_checksum_buffer`: Invokes a function pointer stored relative to a global variable and then computes a rolling checksum over a stack-allocated buffer until a null terminator is reached.
- `store_register_to_pointer`: copies a byte from a data register into a memory location pointed to by an address register.
- `divide_longword_with_remainder`: Divides a 32-bit dividend by a divisor, using bit-by-bit shift and subtract when the divisor exceeds 16 bits and two 16-bit divide steps otherwise.
- `multiply_32bit_integers`: Multiplies two 32-bit values using 16-bit partial products, returning the 32-bit product in its high part and the second operand with its halves swapped in its low part.
- `call_own_entry_then_return`: Calls its own entry point with no arguments and then returns, with no other effect.
- `call_helper_return_register`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag_2`: Handles the signs of two values, calls a helper, and negates the result when the flag the helper leaves in an address register is nonzero before returning.
- `negate_result_on_flag`: Calls a routine with no arguments and returns its result, negated when the second register value it leaves is nonzero.
- `invoke_exec_rawdofmt`: Calls exec's RawDoFmt through the library base held in a global variable and returns nothing.
- `store_register_to_pointer_2`: stores the contents of a data register into a memory location pointed to by a register.
- `invoke_library_vector_276`: Loads a library base from a global variable and calls the library function at offset minus 276 from it, with no arguments and no result.
- `create_message_port_with_signal`: Allocates a signal bit, then a cleared 34-byte message port whose fields are set from the arguments, with the current task stored in it.
- `delete_message_port_and_free`: Removes a message port from the system when its link field is set, marks its status fields as deleted, frees its signal bit and releases its 34-byte memory block.
- `search_text_for_pattern`: Searches text for a zero-terminated pattern one position at a time and returns one when the pattern is found.
- `initialize_empty_list_header`: Initializes a list header in a block by storing the header's address plus four in its first word, clearing its second word and pointing its third word back at the header.

### Format

- `initialize_filesystem_environment`: Initializes global pointers and parses a directory or file path string into a temporary buffer, handling quoted strings and escaped characters.
- `update_format_state_and_cleanup`: Updates a stored parameter within a structure pointed to by a retrieved pointer and executes a sequence of conditional cleanup callbacks, passing a specific identifier as a parameter to each before calling them if their corresponding pointers in a table are non-null.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_indirect_handler_9`: Calls a routine using a pointer stored in a global structure.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `execute_switched_context`: Executes a function pointer retrieved from an offset of a global variable.
- `invoke_exec_forbid`: Calls exec's Forbid function through the library base held in a global variable and returns nothing.
- `execute_indirect_function_4`: loads a function pointer from a global table and executes it.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `invoke_exec_schedule`: Calls exec's Schedule function through the library base held in a global variable and returns nothing.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `invoke_exec_allocabs`: Calls exec's AllocAbs function through the library base held in a global variable and returns nothing.
- `invoke_indirect_function`: Loads a function pointer from a global table offset and executes it.
- `initialize_disk_from_arguments`: Parses the DRIVE, NAME, FFS, NOICONS and QUICK keywords from the arguments and prints the usage text when they are missing.
- `release_stored_format_buffers`: Releases up to five stored buffers in turn, passing each to a release routine together with its known size where one applies, and then calls a final routine with its argument.
- `map_disk_error_to_message`: Reads a status code from a device call, retries while it reports a particular retry code, and maps the code to one of the messages for a bad sector, a device that cannot be opened, a changed disk, a seek error or a request to try a different disk.
- `match_value_in_table`: Looks for the argument among eight bytes of a global table and stores the matching byte, or zero when none matches, into a field of a structure obtained through a routine.
- `find_drive_entry_by_name`: Walks a list of entries through a pointer chain, matching each entry whose flag field is zero either by a stored value or by a name argument.
- `calculate_string_length`: Counts the bytes before the zero terminator of a string and returns that count.
- `copy_cstring_to_buffer`: Copies a zero-terminated byte string from the source to the destination, including the terminator.
- `convert_char_to_uppercase`: Returns the byte as an uppercase letter when it is a lowercase letter from a to z, and returns it unchanged otherwise.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings after converting each byte to uppercase.
- `cut_string_at_colon`: Terminates a string at its first colon, or at its existing terminator when no colon is present.
- `call_single_argument_with_code`: Passes its first argument, as a one-element stack array with a count of one, to a routine together with the constant 31.
- `allocate_node_limited_words`: Allocates a cleared 68-byte node, links it to a looked-up handle and copies up to seven words from the source into it.
- `invoke_library_vector_354`: Loads a library base from a global variable and calls the library function at offset minus 354 from it, with no arguments and no result.
- `execute_indirect_function_5`: loads a function pointer from a table and jumps to it.
- `invoke_library_vector_330`: Loads a library base from a global variable and calls the library function at offset minus 330 from it, with no arguments and no result.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `execute_indirect_function_6`: loads a function pointer from a global table using a constant offset and executes it.
- `execute_buffered_format_handler`: loads a function pointer from a location relative to a global table and executes it.
- `invoke_library_vector_456`: Loads a library base from a global variable and calls the library function at offset minus 456 from it, with no arguments and no result.
- `call_indirect_function`: loads a function pointer from a global table and executes it.
- `execute_indirect_routine`: Loads a routine pointer from a global memory location offset by a constant and executes it.
- `invoke_library_vector_336`: Loads a library base from a global variable and calls the library function at offset minus 336 from it, with no arguments and no result.
- `call_indirect_function_2`: loads a function pointer from a global table and executes it.
- `execute_indirect_routine_2`: fetches a routine pointer from a table and executes it via an indirect call.
- `execute_indirect_pointer`: loads a pointer from a global variable and executes it.
- `execute_indirect_pointer_2`: fetches a function pointer from a memory offset and jumps to it.
- `invoke_library_vector_360`: Loads a library base from a global variable and calls the library function at offset minus 360 from it, with no arguments and no result.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `invoke_library_vector_306`: Loads a library base from a global variable and calls the library function at offset minus 306 from it, with no arguments and no result.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `execute_indirect_pointer_3`: resolves a pointer from a global offset and performs an indirect call.
- `forward_with_global_and_stack_2`: Passes its argument, a value held in a global variable and the address of a stack slot to a second routine, then returns.
- `invoke_library_vector_348`: Loads a library base from a global variable and calls the library function at offset minus 348 from it, with no arguments and no result.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_tracks_with_progress`: Opens the disk device, reads its geometry, and writes each track in turn, reporting progress with the labels Formatting, Initializing disk and Verifying and the cylinder count.
- `apply_format_settings_sequence`: Makes a fixed sequence of calls through a library base pointer stored in a global variable, passing two constants first, then a flag, then the argument and the result of an inner routine.
- `invoke_global_and_measure_string`: Calls a function pointer held in a global variable, then counts the length of a 140-byte stack buffer up to its null terminator.
- `store_low_byte_at_pointer_2`: Writes the low byte of the data register to the memory address held in the address register and returns.
- `divide_long_with_remainder`: Divides a 32-bit dividend by a divisor and returns quotient and remainder.
- `shift_subtract_divide_loop`: Runs the 16-step shift-and-subtract division loop on a 32-bit dividend and a divisor held in registers, leaving quotient bits in the low half of the working register and the remainder in the high half.
- `multiply_32_bit_integers`: Multiplies two 32-bit values using 16-bit partial products.
- `call_adjacent_routine_and_return`: Calls one unnamed routine and returns, passing no arguments.
- `call_helper_return_register_2`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_quotient_on_remainder`: Adjusts the signs of two 32-bit registers, calls a routine to perform long division, and negates the resulting quotient if a non-zero remainder is detected before returning.
- `negate_result_on_flag_3`: Calls another routine and returns its first result, negated when the second result register is nonzero.
- `invoke_table_entry`: Calls the function whose address is stored at a fixed offset inside a global data block and returns with no result of its own.
- `store_byte_via_address_register`: Stores the low byte of the data register at the address in the address register, then returns.
- `allocate_and_initialize_block`: Allocates a memory block through a library call, fills its pointer, flag and count fields from the arguments, then either sets up a list head or passes the block to another routine.
- `reset_object_and_recurse`: Recursively processes the linked block when its pointer field is set, then marks the byte and long fields as all ones, calls a routine with the stored byte value and calls another routine with the block and a constant.
- `forward_argument_with_constant`: Passes its single argument to another routine together with the constant 48 and returns nothing.
- `forward_single_argument`: Passes its single argument unchanged to another routine and returns nothing.
- `setup_trashcan_with_requesters_disabled`: Clears the process window pointer while it sets up global fields that reference the Trashcan text, runs initial routines and then restores the window pointer.
- `read_signed_byte_result`: Reads one byte through a routine that takes a global argument and a stack buffer.
- `invoke_library_entry_240`: Calls the library function at a fixed negative offset from a global library base, passing no arguments and returning no value in the shown code.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Writes the block's own address plus four into its first long word, clears the second long word and stores the block's original address in the third.
- `store_word_pointer_in_node`: Allocates a node through a library-backed routine when the pointer argument is non-null, then stores the value 5 as a byte, the argument word and the pointer into fixed fields.
- `reset_node_and_release_it`: Returns zero for a null pointer.
- `invoke_library_entry_90`: Calls the library function at a fixed negative offset from a global library base as a plain call, with no argument or result handling in the shown code.
- `execute_indirect_pointer_4`: executes a routine located at a pointer stored in a global variable.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `run_cli_with_parsed_arguments`: Program entry point that allocates a session state block, opens the libraries it needs, tokenises the command line into an argument array with quote and escape handling, runs the command through the shell routines and then releases everything, returning a status code.
- `invoke_supervisor_function`: Calls the Supervisor function of the exec library through a library base stored in a global variable, with no arguments.
- `invoke_exit_interrupt_function`: Calls the ExitIntr function of the exec library through a stored library base to leave an interrupt handler.
- `invoke_task_switch_function`: Calls the Switch function of the exec library through a stored library base to hand control to another task.
- `invoke_dispatch_function`: Calls the Dispatch function of the exec library through a stored library base to run the scheduler.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `invoke_make_functions_table`: Calls the MakeFunctions function of the exec library through a stored library base, which sets up a function table for a library.
- `invoke_find_resident_function`: Calls the FindResident function of the exec library through a stored library base to search for a resident module by name.
- `invoke_enable_interrupts_function`: Calls the Enable function of the exec library through a stored library base, which re-enables interrupts.
- `invoke_alloc_entry_function`: Calls the AllocEntry function of the exec library through a stored library base to allocate an entry structure.
- `open_new_cli_session`: Allocates a 64-byte session block when given a zero argument, otherwise reuses the current session.
- `build_linked_node_list`: Walks a list of entries, allocating an 8-byte node for each one, copying the entry value into the node and chaining the nodes in order.
- `free_linked_node_list`: Walks a linked list, releasing the value attached to each node and then the 8-byte node itself.
- `parse_signed_decimal_string_2`: Parses an optional leading plus or minus sign followed by decimal digits from a null-terminated string and returns the signed value.
- `invoke_library_entry_198`: Calls the library function at a fixed negative offset from a global library base, with no arguments and no result handling in the shown code.
- `execute_indirect_pointer_5`: performs an indirect call using a pointer retrieved from a global address.
- `execute_indirect_pointer_6`: dereferences a pointer located at a specific offset from a global variable and calls it.
- `execute_indirect_pointer_7`: loads a function address from a global variable and transfers control to it.
- `execute_indirect_pointer_8`: invokes a routine via a pointer stored in a global memory location.
- `invoke_library_entry_78`: Calls the library function at a fixed negative offset from a second global library base, with no arguments and no result handling in the shown code.
- `execute_indirect_pointer_9`: executes a routine by dereferencing a pointer found at a global offset.
- `invoke_library_entry_96`: Calls the library function at a fixed negative offset from a secondary global library base, passing no arguments and handling no result.
- `divide_value_with_remainder`: Divides a 32-bit dividend by a divisor and returns quotient and remainder.
- `multiply_32_bit_partial_products`: Multiplies two 32-bit values using 16-bit partial products.
- `call_adjacent_routine_and_return_2`: Calls one unnamed routine and returns, passing no arguments.
- `call_helper_return_register_3`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_quotient_on_nonzero_remainder`: Normalizes the signs of two input registers, calls a routine to perform division, and negates the resulting quotient if the remainder or a flag-related value in a0 is non-zero before returning.
- `negate_on_flag`: Calls another routine and returns its first result value, negated when its second result value is non-zero.

### Disk-Validator

- `check_disk_bitmap_entries`: Starts a disk validator that writes a two-digit count into a text buffer, initialises a table with all-ones values and then walks bitmap words testing individual allocation bits.

### Ram-Handler

- `dispatch_after_copy_memory`: Calls the exec CopyMem routine through a global library base, then makes an indirect jump through a table the decompiler could not recover, so the final target is not visible.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_BLIT

- `restore_patched_word_after_calls`: Runs a loop of library calls through a global base, then checks a long word in a structure referenced by a global.
- `decrement_until_minus_one`: Decrements a 16-bit counter register in a loop until it reaches minus one, then returns.
- `decode_bitstream_copy_runs`: Loops over a bit-stream reader, decodes bit-coded run lengths and copies runs of words through a relative offset in the buffer, then calls the library cache clear routine when a global counter is above a threshold.
- `copy_bytes_backwards_from_stream`: Copies a run of bytes backwards from an input stream into the output buffer, reading bit-coded run lengths between copies.
- `decode_run_length_table_copies`: Reads bit-coded values, selects run lengths from a small stored table and copies runs backwards through the input and output pointers.
- `push_and_branch`: Modifies a byte in d0, then pushes d0, d1, and d2 onto the stack before restoring all used registers and branching to another routine.

### d

- `stub_bad_instruction_data_2`: Holds only bad instruction data from the file start, so the routine is a stub with no recoverable behaviour.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:AM2_CPU

- `swap_patched_word_after_calls`: Calls a function pointer and a library routine in sequence, then checks a long word in a structure referenced by a global.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `update_array_with_loop_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 6, calling a routine if a certain condition is met.
- `update_array_with_loop`: Checks if a value in d0 is negative, otherwise it calls another routine and, if d0 is not zero, enters a loop that adds a value from a pointer in a3 to a table and decrements a counter until it reaches zero.
- `update_pointer_and_call_2`: If a flag is set, it copies a pointer from a3 to a1, calls another routine, and then branches to a certain address.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `expand_bitstream_sequence_2`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting exp.
- `expand_bitstream_loop`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `set_loop_counters`: Sets d0 to 5 and d7 to 3, then branches to a certain address.
- `set_loop_counters_2`: Sets d0 to 4 and d7 to 2, then branches to a certain address.
- `set_loop_counters_3`: Sets d0 to 3 and d7 to 1, then branches to a certain address.
- `expand_bitstream_sequence`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or or compressed data to a destination.
- `modify_d1_and_branch`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `expand_and_copy_bitstream_2`: Reads a bitstream-encoded sequence where certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or decompressed byte sequence to a destination buffer.
- `expand_and_copy_bitstream`: Reads a bitstream or compressed-style bit-count-based sequence, expanding or skipping data based on a bit-mask, and copies the resulting bytes from a source pointer to a destination pointer while decrementing a counter in d0.
- `push_data_and_restore_registers`: Stores a byte from d0, a byte from d1, and a word from d2 onto the stack, restores several registers from the stack, and then branches to a certain address.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon

- `initialize_system_memory`: Executes a sequence of function calls and a loop that decrements a counter, then checks if a specific memory pattern exists at a calculated offset to update a global pointer and clear a memory flag.
- `decrement_until_negative_one`: Decrements a 16-bit register or local variable until it reaches negative one and then returns.
- `update_a1_from_a3_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 4, calling a routine if a certain condition is met.
- `iterate_and_accumulate_pointer_values`: Checks a value in d0, and if certain conditions are met, calls another routine and enters a loop that calls another routine, increments a counter, and adds a value from a table (indexed by d0 and d2) to a pointer-based destination, continuing until a counter reaches zero.
- `update_pointer_and_call_3`: If a flag is set, it copies a pointer from a3 to a1, calls another routine, and then branches to a certain address.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop_2`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `expand_bitstream_header_logic`: Decodes a bitstream-encoded sequence by checking a byte-length-prefix and expanding or copying bits, or otherwise updates a global variable and sets a register to a constant value before branching to another routine.
- `expand_bitstream_loop_2`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `set_d0_d7_constants`: Loads the value 5 into d0 and the value 3 into d7, then branches to another routine.
- `set_d0_d7_constants_2`: Loads the value 4 into d0 and the value 2 into d7, then branches to another routine.
- `set_d0_d7_constants_3`: Loads the value 3 into d0 and the value 1 into d7, then branches to another routine.
- `expand_bitstream_variable_length_header`: Decodes a variable-length bitstream or compressed sequence by checking if a bit or a byte-length-prefix (encoded via certain bit-sequences) or a table-based length-prefix exists, and updates a destination offset or pointer in a0 accordingly.
- `modify_d1_and_branch_2`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `expand_bitstream_to_buffer`: Reads a bitstream from a source pointer, expanding or decompressing certain bit-sequences to determine a length and a destination offset, then copies a calculated number of bytes or bits into a target buffer.
- `expand_and_copy_bitstream_3`: Reads a bitstream or compressed-style sequence where certain bytes determine the number of bits to expand, then copies or writes the resulting or decoded bytes to a destination buffer, decrementing a counter in d0 and potentially restoring registers from the stack before returning to another routine.
- `push_data_and_restore_registers_2`: Stores a byte from d0, a byte from d1, and a word from d2 onto the stack, restores several registers from the stack, and then branches to a certain address.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:Ambermoon_install

- `reset_install_state_at_entry`: Program entry for Ambermoon_install that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_until_negative_one_2`: Decrements a 16-bit or 32-bit register or local variable until it reaches negative one and then returns.
- `modify_table_elements_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 3, calling a routine if a certain condition is met.
- `update_table_with_offset`: Performs a loop that calls another routine, checks a condition, and if certain conditions or a counter in d1 are met, calls another routine, calculates an offset, and adds a value from a pointer in a3 to a table, or calls another routine and jumps back to a previous instruction or branches to another routine.
- `initialize_pointer_and_call`: If a condition is met, copies a pointer from a3 to a1, calls another routine, and then jumps back to a previous instruction.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_bytes_backwards`: Copies a certain number of bytes from a source buffer pointed to by a2 to a destination buffer pointed to by a1, decrementing a2 and incrementing a1, then loads a byte from a1 into d0 before returning.
- `expand_bitstream_data`: Expands a bit-encoded or compressed bitstream by reading a certain number of bits from a source buffer and writing the expanded or decoded values into a destination buffer, while also handling a block-copying or padding-related loop if a certain counter reaches zero.
- `process_bitstream_loop`: Sets a loop counter and checks if a byte in d1 is non-zero, otherwise it decrements a byte in d1 and loops, or otherwise it loads a byte from a pointer in a0 into d1, adds d1 to d1 with carry, adds d0 to d0 with carry, and loops using a decrementing counter, finally adding 6 to d0 and jumping to another routine.
- `initialize_bitstream_parameters`: Sets d0 to 5, sets d7 to 3, and jumps to another routine.
- `initialize_bitstream_parameters_alt`: Sets d0 to 4, sets d7 to 2, and jumps to another routine.
- `initialize_bitstream_parameters_min`: Sets d0 to 3, sets d7 to 1, and jumps to another routine.
- `decode_variable_length_bitstream`: Decodes a bit-stream or bit-encoded data source, using a bit-by-bit or bit-grouping method to calculate a certain number of bits, and then copies the resulting data or modifies registers or pointers.
- `modify_registers_and_branch`: Doubles the value in d1, then if d1 is not zero, branches to another routine; otherwise, it loads a byte from the memory location pointed to by a0 into d1, doubles d1, doubles d2, and branches to another routine.
- `expand_bitstream_and_copy_data`: Decodes a bit-stream or bit-encoded data source, using a bit-by-bit or bit-grouping method to calculate a certain number of bits, and then copies a block of bytes from a source buffer to a destination buffer.
- `expand_and_copy_bitstream_4`: Reads a bitstream or compressed-style data, expanding bit-encoded values into a destination buffer, and then copies a block of bytes from a source range to a destination range, finally restoring a saved register and stack state.
- `push_registers_and_restore_stack`: Pushes two bytes from d0 and d1, a word from d2, and a longword-sized block of registers and stack pointer onto the stack using a4 as a pointer, then branches to another routine.

### Ambermoon (1993)(Thalion)(De)(Disk 01 of 10)(A)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `compare_and_branch_on_longword`: Compares a longword at a pointer with a constant value and branches if they are equal.
