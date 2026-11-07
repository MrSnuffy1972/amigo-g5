# Game 1201 (iron lord (1989)(ubi soft)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01201). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802400 top-level bytes in 115 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 29 of 29 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 1233 of 1233 named (0 author symbols), 1233 of 1233 described (200 by rule, 975 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1042 of 1042 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 70913 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 23702 of 24365 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1288194 |
| code | 84022 |
| data | 372184 |
| padding | 26256 |
| unused | 31744 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 117 | 2 |
| binddrivers | 11 | 652 | 35 |
| type | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| makedir | 2 | 117 | 2 |
| c3ddata.bin | 1 | 2 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| FastFileSystem | 2 | 1913 | 123 |
| Ram-Handler | 2 | 10 | 1 |
| ramdrive.device | 3 | 345 | 17 |
| Iron Lord (1989)(UBI Soft)[cr FLT](Disk 1 of 2).adf:ilord | 3 | 6443 | 410 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 10 | 1375 | 109 |
| Iron Lord (1989)(UBI Soft)[cr FLT](Disk 1 of 2).adf:samplesdatas | 1 | 2 | 1 |
| A590HD | 2 | 1853 | 100 |
| Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:fgaulbaire.1989 | 1 | 2028 | 51 |
| Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:wargame | 2 | 2752 | 78 |
| 9 | 1 | 2 | 1 |
| diskfont.library | 15 | 1040 | 61 |
| Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:strt2 | 3 | 2165 | 126 |
| Iron Lord (1989)(UBI Soft)[cr FLT](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| Iron Lord (1989)(UBI Soft)[cr FLT](Disk 1 of 2).adf:ILwxzxw (raw code file, load address unknown) | 1 | 867 | 1 |
| Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:scenario.sce (raw code file, load address unknown) | 1 | 0 | 0 |
| Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:ILwxzxw (raw code file, load address unknown) | 1 | 867 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- iron lord (1989)(ubi soft)[cr flt](disk 1 of 2)/Iron Lord (1989)(UBI Soft)[cr FLT](Disk 1 of 2).adf:ILwxzxw: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- iron lord (1989)(ubi soft)[cr flt](disk 2 of 2)/Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:scenario.sce: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- iron lord (1989)(ubi soft)[cr flt](disk 2 of 2)/Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:ILwxzxw: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- iron lord (1989)(ubi soft)[cr flt](disk 2 of 2)/Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 1 never-loaded region(s), 38145 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `manage_file_and_interrupts` (fgaulbaire.1989): Performs file operations and updates several global variables while manipulating DMA and interrupt enable registers. Kind: disk hardware/track check. Pass path: the branch following the call to an address (specifically the code path that continues execution rather than entering the error/trap state).
- `call_function_and_update_globals` (fgaulbaire.1989): Calls a function via a pointer and then updates four global memory locations using a constant value and bitwise OR operations. Kind: disk hardware/track check. Pass path: the instructions immediately following the call (setting globals/registers for the next stage).
- `configure_dma_hardware` (ILwxzxw (raw code file, load address unknown)): A large routine that accesses the ADKCON and DMACON hardware registers. Kind: disk hardware/controller configuration (likely part of a custom trackloader or hardware-based check). Pass path: not determined.
- `configure_dma_hardware_2` (ILwxzxw (raw code file, load address unknown)): A large routine that accesses the ADKCON and DMACON hardware registers. Kind: disk hardware/controller check. Pass path: unknown (decompilation failed).

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of one register to another and then performs a relative branch.

### binddrivers

- `initialize_fast_memory_and_parse_args`: Attempts to locate a fast memory block and retrieves the command line arguments from the operating system.
- `bind_expansion_drivers`: Opens expansion.library and icon.library to iterate through a list of expansion devices.
- `find_driver_entry`: Scans a table of short values starting from an offset derived from a parameter, decrementing a counter and advancing a pointer until it finds an entry whose first short equals a constant and whose second short points back to the entry itself, then returns the pointer to that entry; otherwise it returns null.
- `iterate_driver_data`: Iterates through a buffer or list, performing arithmetic on a counter and updating a value via another routine.
- `calculate_string_index`: Uses a series of calls to determine an index and a value, then performs conditional logic to return a result or zero.
- `traverse_handler_chain`: Iterates through a linked list of handlers and updates a pointer in a structure if a certain bit is set in a memory location.
- `follow_parent_chain`: The routine receives two pointers and walks a linked list by repeatedly following a field located at a fixed offset from the current node.
- `skip_non_slash_colon`: The routine checks a character string for a leading slash or colon.
- `parse_decimal_integer`: The routine reads a null-terminated string through a pointer and accumulates a decimal integer from consecutive ASCII digit characters, stopping at the first non-digit or null byte.
- `clamp_value_by_offset`: The routine calls another routine with the first two arguments and receives a result.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_alert`: Library stub: loads its arguments and the exec library base, calls Alert and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `compare_two_strings`: The routine compares the byte strings pointed to by two registers byte by byte until a zero byte is found.
- `load_address_to_register`: Loads a 32-bit value from an offset of the stack pointer into a register.
- `compute_checksum`: Initializes a 32-bit accumulator to all ones and decrements it by one for each byte in a null-terminated string.
- `find_character_in_string`: Searches for the first occurrence of a specific character within a null-terminated string and returns a pointer to it, or returns null if the character is not found.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `calculate_checksum_offset`: The routine compares two strings and, if a condition involving a 32-bit value is met, returns the difference between a sum and that value.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.

### type

- `find_max_and_populate_buffer`: Traverses a linked list to identify the maximum value in a specific field, then uses that value to initialize a header and populate a buffer with a sequence of incrementing data.
- `transfer_and_branch`: Moves a value from one register to another and then performs a short relative branch.

### assign

- `update_linked_list_buffer`: Traverses a linked list to find the maximum value in a specific field and then allocates or prepares a buffer based on that value.
- `move_and_branch_2`: Moves a long value from one register to another and then performs a short branch.

### EndCLI

- `find_max_and_populate_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then uses that value to initialize a buffer with a repeating pattern.
- `move_and_branch_3`: Copies a long value from one register to another before executing a short branch.

### execute

- `find_max_and_populate_buffer_3`: Traverses a linked list to determine the maximum value in a specific field and then initializes a data buffer with a header and incremental values.
- `move_and_branch_4`: Transfers a long value between registers and then branches to another location.

### Copy

- `find_max_and_populate_buffer_4`: Traverses a linked list to identify the maximum value in a specific field and then writes a header and a sequence of incrementing values into a buffer.
- `move_and_branch_5`: Copies a longword from one data register to another and then performs a short relative branch.

### makedir

- `allocate_and_initialize_buffer`: Iterates through a linked list to find a maximum value and then allocates a memory buffer based on that value.
- `transfer_and_jump`: Moves a longword from a source register to a destination register before executing a short branch.

### c3ddata.bin

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Disk-Validator

- `update_display_buffer`: Updates a character buffer with numeric values converted from an integer.

### FastFileSystem

- `process_filesystem_callbacks`: Initializes a structure and executes a series of nested function calls and loops.
- `update_linked_list`: Checks if a pointer is non-zero, then updates a list entry with a value from a table and calls another routine.
- `calculate_volume_geometry_and_mount`: Calculates block counts, sector offsets, and buffer requirements based on filesystem metadata.
- `traverse_and_execute_callbacks`: Traverses multiple linked lists and executes function pointers stored at various offsets within a structure.
- `pop_linked_list`: Iterates through a loop to pop elements from a linked list by updating a pointer in a table until a null pointer is encountered.
- `validate_and_mount_filesystem`: Verifies a filesystem header for specific magic numbers and structural integrity, then traverses a linked list of blocks to mount the volume.
- `update_structure_fields`: Updates several fields in a structure, including a calculated longword, a longword from a table, a word, and a longword, before calling another routine.
- `calculate_offset_size`: Retrieves a value from a nested pointer structure and returns it multiplied by four plus four.
- `copy_buffer_and_update_linked_list`: Copies a block of memory from one location to another using a loop, then updates a pointer structure by storing the new address and a calculated size value.
- `update_list_node_value`: If a value is non-zero, it executes a function pointer, traverses a list by following scaled pointers, updates a node with a value from a table, and executes more function pointers.
- `copy_and_execute`: Copies a longword from a table and then executes three consecutive function pointers.
- `calculate_bit_density`: Calculates a cumulative value by repeatedly calling another routine in nested loops based on a value extracted from a structure, then counts the number of zero bits in a specific bitfield of a word.
- `count_leading_zeros`: Calculates a value by counting how many times a bit can be shifted right before encountering a non-zero bit.
- `scan_bitfield_for_free_block`: Iterates through a bitfield to locate the first unset bit representing an available resource.
- `update_index_and_pointer`: Checks a condition based on a comparison of two longwords, then updates a pointer and a value using offsets from a structure.
- `search_and_clear_bit`: Iterates through memory to find a specific bit set in a longword, clears it, and updates a structure with the resulting index and pointer.
- `modify_memory_and_increment`: Clears a bit in a register, stores a longword at a calculated address, updates a global variable, and moves a value into a register before branching.
- `search_and_clear_bit_2`: Iterates through a bitfield to find a set bit, clears it, and updates several fields in a global structure.
- `compare_and_branch`: Loads a longword from a calculated address, branches if not equal to zero, and otherwise adds a value to a register before branching again.
- `test_bit_and_loop`: Tests a bit in a register and, if not set, enters a loop that increments a counter and compares it against a value.
- `find_bit_and_update_structure`: Searches through memory for a set bit within longwords and, upon finding it, clears the bit and updates a structure with the position and address.
- `return_error_constant`: Returns a constant value of negative one.
- `calculate_fragment_details`: Computes a value based on the difference between two numbers and returns a concatenated result containing quotient, remainder, and bitwise mask information.
- `align_value_down`: Subtracts the remainder of a value divided by a table entry from the original value.
- `set_bit_and_branch_2`: Saves registers to the stack, calls another routine, sets a bit in a register using a value from a table, and branches based on the result.
- `decrement_and_store`: Stores a value at a calculated offset and decrements a counter in a table.
- `store_register_to_offset_pointer`: Takes a value from a data register and stores it into a memory location calculated by adding an offset to a value returned by another routine.
- `synchronize_block_metadata_buffer`: Clears a destination buffer and copies a set of longwords from a source pointer.
- `update_and_sync_block_metadata`: Clears and copies a set of longwords into a destination buffer, then performs a bulk transfer of data blocks based on a calculated count.
- `calculate_table_index`: Performs a mathematical calculation on an input value to derive an index and returns a longword from a table located via a pointer in a global structure.
- `reorder_linked_list`: Manipulates pointers within a linked list to move elements between two different list heads.
- `update_linked_list_head`: Stores a value from a global structure into a destination, sets a pointer to the stack, and updates a global structure with the new destination.
- `clear_structure_pointer`: Loads a pointer from a global structure, retrieves a second pointer from that location, clears the second pointer, and branches.
- `update_linked_list_head_2`: Copies a longword from a pointer into a destination, clears the longword at the source, and branches to another routine.
- `insert_into_linked_list`: Traverses a linked list to find a specific node and performs a pointer swap to insert a new element into the list.
- `initialize_node_pointers`: Sets multiple fields in a structure to a new pointer value and returns a constant.
- `check_status_and_initialize_structure`: Calls another routine and, based on the return value, either clears specific fields in a structure or sets a field to a constant value.
- `populate_linked_list`: Fills a series of memory locations at regular intervals with values from a structure and links them together.
- `execute_callback_loop`: Checks a condition in a global structure and either executes a single function pointer or enters a loop calling two different function pointers until one returns a non-zero value.
- `transfer_and_call`: Copies a word to a destination, moves a pointer to the stack, and calls another routine.
- `transfer_structure_field`: Copies a longword from one memory location to another and updates a pointer within a structure.
- `update_linked_list_2`: Copies a value from a structure, updates a field in a different structure, and sets a pointer to the current stack location.
- `check_and_branch`: Tests a word at an offset from a pointer and branches based on the result, otherwise loads a value into a register and branches.
- `transfer_pointer_and_branch`: This routine moves a value from the stack to a register, loads a longword from an offset of a base pointer, and branches to another location based on a comparison.
- `validate_and_call_structure_subroutine`: Checks a longword at an offset within a structure and, if certain conditions are met, calls a subroutine.
- `validate_and_call_subroutines`: Checks multiple memory offsets for non-zero values and, if all conditions are met, calls two subroutines in sequence.
- `execute_conditional_calls_3`: Loads a longword from a memory offset and, if not zero, performs two subroutine calls using that value as a pointer.
- `execute_conditional_calls_2`: Moves a longword from an offset of a pointer into a register, and if not zero, performs two subroutine calls using that value and a modified version of it.
- `traverse_linked_list_node`: Checks a pointer and its internal fields to determine if it should follow a link to another node.
- `copy_structure_field`: Copies a longword from a source location to a destination location and increments an internal pointer within a structure.
- `traverse_linked_list_6`: Loads a pointer from a structure, checks if it is null or points to a value less than or equal to zero, and if valid, updates the pointer to the next element in a loop.
- `transfer_control_flow`: Loads a longword from a pointer into a register, calls another routine, and then branches to a new location.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a7)+ to d2, then returns.
- `clear_and_copy_word`: Clears a word at an offset from a pointer and then copies a longword from a global structure to that same location.
- `set_structure_word`: Compares a word from a structure to a constant and, if it matches, stores a different value into that structure.
- `set_word_and_call`: Stores a word value into a structure and calls another routine twice.
- `process_conditional_logic_2`: Iterates through a structure while performing various conditional checks, field initializations, and subroutine calls based on specific values.
- `execute_conditional_subroutine`: Compares a byte at an offset, and if it matches a specific value, calls two different subroutines before branching.
- `call_subroutine_and_jump`: Moves a value from a register to a pointer and calls another routine before jumping to a new location.
- `execute_conditional_branch_2`: Tests a longword at an offset from a pointer and, if non-zero, calls another routine before branching to a new location.
- `execute_conditional_subroutines_2`: Tests a longword and compares a pointer, then conditionally calls two different routines before branching to a new location.
- `call_and_jump`: Calls another routine and then jumps to a new location.
- `execute_conditional_subroutines_3`: Calls multiple subroutines based on the result of a test and performs a branch.
- `initialize_structure_fields_2`: Populates several fields of a structure, including a pointer to itself plus an offset, with specific values and data from other registers.
- `manage_linked_list_nodes`: Evaluates a return value to conditionally insert a new node into a linked list and then initializes a structure with various metadata and pointers.
- `initialize_and_update_bitmask`: Fills a memory range with all ones and updates a bitmask by setting or clearing a bit based on a calculated index and bit position.
- `increment_and_compare_loop`: A loop that increments a register and compares it against a value from a global variable, resetting registers to zero if the condition is met.
- `test_and_increment_bitmask`: Tests a bit in a table and branches if set.
- `modify_bitmask_and_branch`: Clears a bit in a table using two registers, decrements a counter, and then conditionally calls another routine or performs a branch based on a value loaded from memory.
- `clear_bitmask_entries`: Iterates through a list of values and clears their corresponding bits within a bitmask table.
- `clear_bitmask_range`: Iterates through a set of values and clears corresponding bits within a bitmask table.
- `scan_and_clear_bitmaps`: Iterates through a range of values to check if specific bits are set within a bitmask table.
- `process_bit_and_call`: Calls another routine, clears a bit in a longword, and if the bit was set, calls another routine and moves a value from the stack.
- `clear_and_update_bitmaps`: Iterates through a bitmask table to identify and clear specific bits within a range of values.
- `compare_and_branch_range`: Saves registers and compares a value in a register against two different values stored in a global structure.
- `set_bit_in_buffer`: Performs bitwise operations on a value and sets a specific bit in a buffer at an offset determined by a shifted version of that value.
- `return_stack_value`: This routine returns the value currently residing at the top of the stack.
- `validate_header_type`: Compares a longword at an offset from a pointer against two specific constant values and branches if a match is found.
- `validate_structure_fields`: Checks if specific longwords in a structure are zero or equal to each other before calling another routine.
- `load_small_constant`: Loads the value two into a register and branches to another location.
- `compare_and_branch_2`: Compares two longwords from a structure and branches if they are equal.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `copy_and_loop_string`: Copies a null-terminated string to a new location and then executes another routine in a loop until it returns zero.
- `increment_string_values`: Iterates through a list of integers, performing modulo-like arithmetic on them and updating a string by incrementing character values.
- `process_complex_structure_loop`: Iterates through a loop that performs multiple conditional checks, recursive calls, and updates various fields within a structure.
- `traverse_and_modify_linked_list`: Iterates through a linked list by following pointers stored within structure nodes.
- `process_file_metadata_and_update_counts`: Iterates through a structure linked via a pointer to perform conditional updates on word and longword values based on specific field comparisons.
- `update_structure_node_data`: Updates a structure with values from a source object, including a calculated offset and a pointer.
- `process_structure_loop`: Updates several fields in a structure within a loop and calls another routine until a condition is met or a null character is found.
- `copy_string_and_verify`: Copies a null-terminated string from a global variable to a local buffer and then copies a block of memory based on a pointer found in a nested structure.
- `copy_memory_and_check`: Copies a block of memory and then conditionally calls another routine based on values in a structure.
- `update_pointer_field`: Stores a longword from a global structure into a location and updates a pointer at an offset from that location.
- `assign_status_value`: Stores a word into a location based on a comparison of a shortword at an offset from a pointer.
- `call_relink_and_initialize_node`: Acts as a wrapper that calls a function to traverse a linked list, update node status fields, and modify internal pointers and data values.
- `call_relink_routine_unreached`: This routine is unreached in the code flow and calls another routine.
- `call_relink_and_initialize_node_2`: Calls another routine and then returns.
- `check_constant_value`: Compares a value in register d0 against a specific constant.
- `execute_conditional_task`: Loads three pointers, calls a subroutine, and then conditionally calls another routine via a jump table based on the result.
- `manage_library_resources_2`: Initializes a data structure and conditionally performs a sequence of operations involving memory management, function pointer calls, and library-related tasks.
- `initialize_data_structure`: Writes a series of bytes and longwords to a memory location to initialize a structure.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `copy_block_of_longwords`: Copies thirty-two longwords from a source memory location to a destination memory location using a loop.
- `clear_memory_block`: Clears a block of memory by writing zero to thirty-two consecutive double-word entries.
- `branch_to_sub_routine`: Performs a relative branch to another routine.
- `sum_table_elements`: Iterates through a table of four-word entries and calculates the sum of all values contained within them.
- `update_list_entry`: Stores two values into a structure and, if the first value is non-zero, updates a pointer within a linked list and calls another routine.
- `update_list_node`: Copies a longword from a pointer into a destination, clears the source longword, and updates a pointer within a linked list structure.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `compare_byte_sequences`: A loop that compares bytes at two pointers and continues as long as the bytes match and are non-zero, returning a failure value if they differ.
- `check_status_flags`: Writes a value to an offset of a pointer and returns a specific constant if two conditions regarding a structure are met.
- `calculate_coordinates`: Performs arithmetic on a value from a structure to derive three derived values, then stores them in two different locations.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `initialize_structure_fields`: Sets several fields in two different structures and then calls another routine.
- `insert_into_linked_list_2`: Calculates an address from a table and an index, then inserts a pointer into a linked list at that location.
- `calculate_offset_and_compare`: Calculates an offset based on a bitwise operation on a word, adds it to a pointer, and compares the resulting address to another value.
- `copy_longword`: Copies a longword from an offset of a pointer into another location.
- `traverse_table_entry`: Calculates an offset into a table, compares values at that location against a register, and branches based on the results.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies a0 to d0, then returns.
- `transfer_pointer_to_destination`: Clears a longword at a destination, copies a longword from a source to that destination, and conditionally updates a pointer.
- `write_pointer_twice`: Writes a value to a pointer and then writes that same value to the same location again.
- `conditionally_move_longword`: Pushes a register onto the stack and conditionally moves a longword from one location to another based on a value at an offset.
- `store_longword_at_offset`: Copies a longword from a source to a destination pointed to by a value at an offset, then conditionally stores a different longword.
- `store_word_at_offset`: Stores a longword from a pointer into a location offset by four bytes from the target address.

### Ram-Handler

- `execute_indirect_jump`: Loads a function pointer from an offset of a global variable and jumps to it.

### ramdrive.device

- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `configure_ramdrive_device_parameters`: Iterates through a list of device structures to locate a specific entry, then copies metadata and configuration strings into global buffers.
- `initialize_ramdrive_device_structure_2`: Copies a block of data from a source buffer into a global variable and populates a device structure with metadata.
- `initialize_ramdrive_device_structure`: Calculates a memory size based on several values and uses it to initialize a device structure.
- `initialize_ramdrive_device`: Initializes a device structure by storing pointers to internal buffers and control fields, and links it into a global list.
- `initialize_device_buffers`: Performs two consecutive calls to a routine using different values in a register and different pointers in a register.
- `traverse_linked_list_4`: Traverses a linked list to find a specific value and then calls a routine via a pointer.
- `modify_interrupt_enable`: Calculates an address based on a register, calls a routine, and conditionally modifies a hardware register to enable or disable interrupts.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_offset_3`: Clears a register and performs a relative branch to another location.
- `store_dword_at_offset`: Stores a longword from a register into a memory location offset from a pointer.
- `process_device_message`: Executes a function pointer from a global structure, looks up a value in a table based on a field in a message structure, stores the result back in the message, and conditionally calls another function pointer before a final cleanup routine.
- `return_negative_constant_2`: Returns a constant negative integer value.
- `return_negative_constant`: Returns a constant value representing a negative integer.
- `modify_list_and_string`: Decrements a value in a structure, performs linked list manipulations, iterates through a list to find a match, and replaces a null terminator in a string with a colon.
- `move_and_compare_pointer`: Moves a longword from one memory location to another and performs a comparison loop using a pointer.
- `terminate_string_with_colon`: Iterates through a string until a null terminator is found, replaces it with a colon, and executes a function pointer.

### Iron Lord (1989)(UBI Soft)[cr FLT](Disk 1 of 2).adf:ilord

- `zero_memory_and_initialize_globals`: Clears a block of memory by looping through a range and setting values to zero.
- `initialize_game_state_tables`: Initializes a large block of memory by setting numerous table entries to a global variable and clearing specific fields.
- `process_callback_and_hardware_state`: Iterates through multiple tables of function pointers to execute specific callbacks with varying arguments.
- `execute_sample_callback`: Checks a value in a table and, if zero, executes a callback function and stores the result in a global variable.
- `dispatch_callback_table`: Iterates through a table of forty pointers and calls a function for each non-zero entry using associated data from another table.
- `store_word_pair`: Stores two consecutive 16-bit values from a register into a memory location offset from a pointer.
- `update_structure_from_table`: Updates several fields in a structure by retrieving values from multiple tables using an index.
- `populate_structure_from_table`: Populates a structure with values retrieved from multiple offsets in a table based on an index.
- `call_library_function`: Calls a library function via a pointer and executes another routine if the result is zero.
- `execute_callback_sequence_2`: Checks a value in a table and, if zero, executes a callback function and stores the result in a global variable.
- `execute_empty_sequence`: Calls a routine five times in succession.
- `execute_recursive_calls_2`: Recursively calls itself four times before returning.
- `execute_callback_chain_2`: Calls two different callback dispatch routines in succession.
- `process_callback_chain`: Iterates through a series of function pointers and executes them sequentially.
- `clear_audio_registers`: Clears several audio hardware registers and DMA control registers by writing zero to them.
- `set_register_to_one`: Calls another routine and then stores the value one into a memory location offset from a pointer.
- `clear_audio_hardware`: Clears audio hardware and DMA control registers by writing zero to them and sets a byte in a memory location to one.
- `reset_audio_and_set_flag`: Clears audio hardware registers and then sets a byte to one at a location calculated from a register and a constant offset.
- `manage_audio_buffer`: Performs a series of callback executions to allocate or update a buffer and stores the resulting pointer in a global variable.
- `update_indexed_data`: Copies a longword from an indexed location in a table to two different destinations and toggles a byte via an XOR operation.
- `clear_stack_word_and_branch`: Creates a stack frame, clears a word at a negative offset from the stack pointer, and performs a short branch to another routine.
- `execute_repeated_calls`: Calls a function pointer repeatedly while incrementing a value in memory until it reaches a limit stored in a global variable.
- `call_function_repeatedly`: Calls a function a specific number of times based on a value stored in a table.
- `clear_stack_word`: Clears a word at a specific offset on the stack and performs an unconditional branch.
- `loop_with_counter_increment`: Executes a function and increments a value in a table within a loop.
- `execute_repeated_function_calls`: Repeats a call to a function pointer a number of times determined by a value stored at a specific offset from a base address.
- `initialize_stack_frame_branch`: Initializes a stack frame and clears a word at a negative offset from the stack pointer before branching.
- `iterate_and_increment_table_entry`: Executes a function call repeatedly while incrementing a short integer stored in a table.
- `update_table_values`: Copies two words from one location to another, waits for a bit flag to be set, and then updates the original location with values from a pointer-based table.
- `calculate_derived_values`: Retrieves multiple values from a table via a function, stores them in several locations, and performs arithmetic to store a calculated result.
- `write_to_global_offsets`: Calls a function multiple times using a global variable and various fixed offsets.
- `call_function_with_table_data`: Iterates four times, calling a function with a pointer retrieved from a table.
- `execute_callback_sequence`: Iterates through a series of memory addresses, passing each one as an argument to a single function pointer retrieved from a base address.
- `process_list_of_addresses`: Calls a function repeatedly using a sequence of addresses from a table.
- `allocate_and_initialize_string_buffer`: Allocates a memory block based on a calculated size and initializes it with a specific four-byte signature.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `process_directory_file_metadata`: Iterates through a directory structure to compare file names and extract metadata.
- `retrieve_directory_and_execute_indirect`: Retrieves the current directory via a DOS function and executes a function pointer retrieved from a nested offset within a structure pointed to by a global variable.
- `copy_d0_to_d5`: Copies the contents of data register zero to data register five.
- `setup_blitter_params`: Configures multiple Blitter registers including source, destination, and size parameters, and enables the DMA controller.
- `configure_blitter_and_call_indirect`: Configures Blitter registers and DMA parameters, calls a library function, and triggers a function call through a pointer in a table.
- `perform_blitter_transfer`: Sets Blitter width, height, and dimension registers before initiating a transfer and calls a library function and an indirect function pointer.
- `configure_blitter_transfer`: Sets specific Blitter registers for width, height, and dimensions before initiating a transfer.
- `modify_stack_and_pointer`: Adjusts values on the stack and updates a value at an offset from a pointer using arithmetic operations.
- `loop_blitter_operations`: Executes a loop that calls another routine with various parameters from a table, incrementing and decrementing specific values until a condition is met.
- `parse_command_string`: Iterates through a character buffer to normalize alphanumeric characters and map specific punctuation symbols to internal numeric identifiers.
- `map_character_to_token`: Reads a character from a source string and converts it into a specific numeric token or control value.
- `validate_and_modify_byte`: Compares a byte at a negative offset from a pointer against two constant values.
- `check_and_set_byte`: Compares a byte at a negative offset from a pointer to a constant and, if it matches, writes a word to a different negative offset and a byte to the original offset.
- `compare_and_modify_byte`: Compares a byte at a negative offset from a pointer against two different constants and, if a match is found, modifies a value at a negative offset from that same pointer.
- `update_state_on_match`: Compares a byte at a negative offset from a pointer to a constant.
- `update_byte_and_word`: Compares a byte at a negative offset from a register to a constant and, if they match, overwrites that byte and an adjacent word with new values.
- `compare_and_update_byte_and_word`: Compares a byte at a negative offset from a register to a constant and, if they match, overwrites that byte and an adjacent word with new values before branching to another routine.
- `update_stack_values_3`: This routine compares a byte at a specific offset from a pointer to a constant and, if they match, overwrites that byte and an adjacent word with new values.
- `validate_and_update_stack_value`: Compares a byte at a specific offset from a pointer to a constant and, if it matches, overwrites that byte and an adjacent word with new values.
- `patch_stack_bytes`: Compares a byte on the stack to a constant and, if it matches, overwrites two adjacent stack values with new constants.
- `patch_stack_value`: Compares a byte on the stack to a constant and, if it matches, overwrites subsequent stack values with new constants.
- `validate_and_patch_stack_value`: Compares a byte located at a negative offset from a frame pointer to a constant.
- `modify_stack_values`: Compares a byte on the stack to a constant and, if it matches, overwrites a word and a byte at nearby stack locations.
- `update_stack_values_2`: Compares a byte at a negative offset from a frame pointer to a constant and, if it matches, writes a word and a byte to the stack before branching.
- `update_stack_byte`: Compares a byte on the stack to a constant and, if it matches, writes a new word and a new byte to the stack before branching.
- `update_stack_values`: Compares a byte at a stack-relative offset to a constant and, if equal, updates two values at stack-relative offsets.
- `compare_and_update_stack_byte`: Compares a byte at a stack-relative offset to a constant value.
- `compare_and_update_byte_2`: Compares a byte from a stack-relative location to a constant.
- `compare_and_update_byte`: Compares a byte from the stack to a constant and, if equal, writes a word and a byte to stack-relative locations.
- `compare_and_update_stack`: Compares a byte from a stack offset against a constant and, if they match, updates a stack value and a different stack offset before branching.
- `jump_to_offset`: Performs a short branch to a specific offset within another routine.
- `parse_input_string`: Iterates through a character buffer to extract and normalize alphanumeric input.
- `check_joystick_bit`: Clears three words in memory and then checks a specific bit in a hardware register.
- `test_bits_and_update_memory`: Tests specific bits in a register and performs bitwise operations to update values at two different memory locations.
- `check_input_status`: Checks a bit in a register and updates a global variable or a memory location based on the result.
- `check_and_call_handler`: Checks a bit in a value loaded from a pointer and calls another routine if the bit is set.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `process_data_loop`: Clears a range of memory using a loop and then iterates through a table to call a routine with values from that table.
- `load_register_and_branch`: Loads a small constant into a data register and performs a short branch.
- `poll_input_and_update_state_2`: Repeatedly polls a status function through a pointer to check for input events based on a mode parameter.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `poll_input_state_loop`: Repeatedly polls a status function through a pointer to update a local variable until a non-zero value is returned.
- `branch_to_routine`: Clears a register and branches to another routine.
- `poll_input_event_loop_2`: Repeatedly polls a status function through a pointer to update a local variable until a non-zero value is returned.
- `poll_input_event_loop`: Repeatedly polls a status function through a pointer to update a local variable until a non-zero value is returned.
- `process_input_event_loop`: Repeatedly polls a status function through a pointer to update a local variable until a non-zero value is returned.
- `poll_input_and_update_state`: Repeatedly polls a status function and updates a local variable until a non-zero value is returned or a specific state condition is met.
- `execute_subroutine_sequence`: Performs two subroutine calls, pushes a value from a global variable onto the stack, calls a third subroutine, adjusts the stack pointer, and branches to another location.
- `sync_and_shuffle_table_data`: Copies a block of data from one memory location to another based on a conditional flag.
- `update_linked_list_pointer`: Checks a word at an offset from a pointer; if it is non-zero, it branches to another location.
- `process_table_data_2`: Iterates through a table to fetch and process values using a series of nested loops and conditional checks.
- `extract_table_element`: Calls another routine and then uses the result to fetch a longword from a table using an offset.
- `copy_nested_table_elements`: Iterates through a two-dimensional data structure to extract and copy specific elements into a destination buffer.
- `process_nested_data_structures`: Iterates through nested loops to access elements from a table and copy data blocks between memory locations.
- `process_nested_loops`: Iterates through data structures using multiple counters and nested loops to perform subroutine calls and copy blocks of data between memory locations.
- `copy_table_entries`: Copies forty-eight entries from one memory table to another if a flag is not already set.
- `wait_for_flag`: Updates two memory locations with provided values and then waits for a specific bit in a global variable to be set.
- `execute_conditional_callbacks`: Compares two values and executes a series of different routines based on the contents of a specific memory location.
- `parse_text_string_2`: Iterates through a string to extract words by skipping spaces and copying characters into a local buffer.
- `call_and_update_status`: Pushes a longword from a pointer onto the stack, calls another routine, and updates a longword in memory with the sign-extended result of that routine.
- `compare_and_increment_pointer`: Compares a value from a memory location with a value from a pointer and, if the condition is met, checks if a byte at a specific address is a space character.
- `parse_and_process_string`: Parses a string by trimming leading and trailing spaces and copying the content into a local buffer.
- `decrement_and_loop`: Decrements a word at a negative offset from a pointer and enters a loop if a value at a specific memory location does not match a constant.
- `clear_conditional_word`: Tests a word at a negative offset from a pointer and clears a different word at a negative offset if the test result is not carry.
- `parse_input_string_tokens`: Iterates through a string pointed to by a data structure, skipping whitespace to extract contiguous non-space character sequences into a local buffer.
- `set_flag_value`: Stores a constant value into a memory location relative to a pointer.
- `parse_command_token`: Skips leading whitespace in a string and extracts the next contiguous sequence of non-space characters into a local buffer.
- `skip_space_characters`: Iterates through a sequence of bytes pointed to by a value in a data structure, incrementing the pointer as long as the byte is not a space character.
- `test_and_clear_word`: Tests a longword at a negative offset from a pointer and clears a word at a different negative offset if the test result is not lower than zero.
- `parse_input_token`: Iteratively extracts substrings from a source string by skipping whitespace and copying characters into a local buffer.
- `return_immediately_6`: This routine is empty and performs no operations before returning to the caller.
- `parse_text_string`: Skips leading whitespace in a string and copies a sequence of characters into a local buffer.
- `return_immediately_7`: An empty routine that performs no operations and returns immediately to the caller.
- `process_conditional_data`: Calls a routine with parameters from a table, stores the result in a local variable, and recursively calls itself if that result is non-zero.
- `dispatch_indexed_value`: Selects a constant based on an input value, calls a routine using a table-based address, and stores the result in a memory location.
- `execute_indexed_callback`: Stores two words into a memory location before calling a function via a pointer fetched from a table.
- `execute_indexed_callback_2`: Calls a function using an index to retrieve parameters from a table and stores the return value in a global variable.
- `dispatch_indexed_jump`: Compares a longword to a constant, performs a bit shift and a memory fetch to calculate an offset, and then jumps to an address from a table.
- `process_indexed_data`: Calls a routine using an index to fetch a parameter from a table, stores the result, and performs a conditional recursive call.
- `process_data_and_call_handler`: Calls a routine with parameters from a table, performs a conditional recursive call, and finally executes a function pointer from a table.
- `process_global_data`: Calls a routine using a global variable as a parameter, stores the result, and executes a function pointer from a table if the result is non-zero.
- `execute_conditional_calls`: Checks if a byte is non-zero and calls two different subroutines if it is.
- `transpose_interleaved_buffer`: Performs a bidirectional deinterleaving and reinterleaving of data buffers using a seven-byte stride.
- `deinterleave_and_reinterleave_buffer`: This routine performs a two-stage buffer transformation by first deinterleaving bytes from seven different source offsets into a temporary workspace, and then reinterleaving them back into a destination buffer.
- `copy_interleaved_data_block`: Copies bytes from a source buffer into a destination buffer using an interleaved pattern, incrementing the source index by two for every one byte written to each of several destination offsets.
- `populate_buffer_from_offsets`: Iterates through a loop to copy bytes from seven different source offsets into a destination buffer, incrementing a destination index for each byte.
- `execute_nested_functions`: Calls a routine with parameters from a structure, stores the result, and executes two function pointers using values from that structure.
- `fetch_byte_via_offset`: Calculates an address by adding a high-word value from a parameter to a base address and returns the byte stored at that location.
- `dereference_offset_pointer`: Calculates an address by adding a high-word offset to a base address and returns the word stored at that location.
- `fetch_long_from_table`: Returns a four-byte longword from a table using a base address and an offset provided in a register.
- `initialize_and_store_address`: Sets a value at a negative offset from a register and compares a value at a specific offset to a constant.
- `update_linked_list_pointer_2`: Checks a word at an offset from a pointer; if it is non-zero, it calculates a new address from a base register and stores it into a location relative to that same pointer.
- `iterate_table_backwards`: Processes table entries in a descending loop, performing multiple function calls and comparisons for each element.
- `compare_table_elements`: Calls another routine and then uses the result to index into a table to compare a value against a threshold.
- `update_table_counter_and_values`: Iterates through a table of values to compare elements against a threshold, updating a counter and a secondary value based on conditional logic.
- `process_table_entries`: Iterates through a table of values and performs conditional logic to update a counter and a secondary value.
- `iterate_and_process_table`: Iterates through a table of values, calling a routine for each entry and performing conditional logic based on a counter and table comparisons.
- `loop_function_calls`: Executes a specific subroutine sixteen times while incrementing a counter.
- `update_composite_value`: Calculates a 16-bit value from two 8-bit components retrieved from memory.
- `conditionally_update_global_value`: Tests a word at an offset from a pointer; if non-zero, it branches to another routine, otherwise it adds a word from that pointer to a global variable and branches to another routine.
- `update_data_structure`: Updates a global word value and stores bytes into a table at an offset determined by a local value.
- `execute_conditional_function`: Calls another routine and then executes a specific function if a global word value equals one.
- `execute_subroutine_with_extension`: Calls another routine using a pointer from a register, extends the result to a longword, and stores it in a structure before conditionally returning.
- `parse_asset_stream`: Reads a sequence of bytes from a data stream and parses them based on specific character markers.
- `loop_on_status_check`: This routine repeatedly calls another subroutine using a pointer from the stack and branches back to the start if the returned word matches a specific constant.
- `validate_and_set_status`: Pushes a pointer from a local frame onto the stack, calls another routine, and compares the returned word against a constant.
- `call_and_verify_result`: Pushes a longword from the stack, calls another routine, and then compares the returned value against a constant to set a register.
- `call_and_check_result`: Pushes a longword from the stack, calls another routine, and then compares the result in a register against a constant to determine the next branch.
- `decrement_and_call_routine`: Clears a word at an offset from a pointer, calls another routine, and then decrements that word while checking if it reaches a specific value.
- `decompress_bitstream_data`: Expands a compressed data stream into a destination buffer using a custom variable-length encoding scheme.
- `clear_word_and_branch`: Clears a word at a location relative to a pointer and performs a short branch to another routine.
- `unpack_compressed_data`: Iterates through a data stream to expand compressed bytes into a destination buffer.
- `branch_to_target`: Performs a short branch to another routine.
- `increment_and_call`: Clears a word, calls a subroutine, and increments a counter while conditionally clearing another word based on a comparison.
- `clear_and_branch_2`: Clears a word in a local structure and performs a short branch.
- `populate_nested_buffer_from_lookup`: Performs a multi-level nested loop to calculate a destination buffer address and fill it with values retrieved from a lookup routine.
- `populate_nested_data_structure_2`: Performs a multi-level nested loop to iterate through indices and populate a destination memory buffer.
- `update_structure_field`: Compares a byte from a structure against a threshold and, if it exceeds it, calls another routine before storing updated values back into the structure.
- `populate_nested_data_structure`: This routine performs a multi-level nested loop to iterate through indices and populate a destination memory buffer.
- `increment_and_call_routine`: Increments two different values in a structure and calls another routine if specific comparison conditions are met.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d1.
- `manage_memory_allocation`: Allocates memory blocks of specific sizes and updates a global pointer based on the results.
- `reset_memory_pointer`: Calls two subroutines and then clears a value in a global variable.
- `execute_recursive_call_2`: The routine performs a single recursive call to itself before returning.
- `load_pointer_from_structure`: Compares a word in a structure to a constant and, if not equal, loads a long word from a pointer found in that structure into a register.
- `check_and_load_word`: Tests a word in a structure and, if not negative, loads a word from a pointer found in that structure into a register.
- `process_signed_data_and_populate_array`: Checks the sign of a value within a structure, potentially negating it and setting a flag.
- `parse_formatted_string_3`: Parses a string containing format specifiers to extract integer, floating-point, or string values.
- `clear_and_branch_7`: Clears a word at a negative offset from a register and performs a short branch.
- `parse_bytecode_stream_3`: Iterates through a stream of command bytes to interpret instructions and operands.
- `clear_and_branch_6`: Clears a word at a specific offset on the stack and performs a relative branch.
- `parse_bytecode_stream_2`: Iterates through a stream of command bytes and executes specific logic based on the value of each byte.
- `parse_bytecode_stream`: Interprets a stream of command bytes by performing a series of switch-case operations.
- `parse_format_string_tokens`: Iterates through a string to identify and process format specifiers and numeric values.
- `jump_via_table`: Performs arithmetic on a register and uses the result to fetch a word from a table and jump to that address.
- `parse_formatted_string_2`: Parses a string containing numeric values and format specifiers, updating a counter and a state structure based on the characters encountered.
- `branch_away`: Performs a short branch to another location.
- `execute_looping_call`: Calls a function via a pointer and enters a loop that continues until the return value is not equal to a specific constant.
- `decrement_and_clear_value`: Decrements a value in a structure and, if it is greater than a threshold, clears a different value in the same structure.
- `iterate_and_call`: Increments a pointer in a table, retrieves a byte, and calls a function via a pointer within a loop.
- `update_structure_value`: Increments a value in a structure, compares it against a threshold, and conditionally adds the result to another field in the structure.
- `call_with_parameter`: Calls a function with a constant parameter and enters a loop that continues until the return value is not equal to a specific constant.
- `increment_and_compare_value`: Increments a value in a structure, compares it against a threshold, and branches based on the result.
- `parse_formatted_string_loop`: Calls a routine to parse a string and loops if the return value is not negative one.
- `increment_and_branch`: Increments a value at an offset from a pointer and branches to another location.
- `branch_to_parser`: Loads a word from an offset of a pointer into a register and branches to another routine.
- `dispatch_callback_via_library_function`: Checks a global variable and, if zero, calls a library function to retrieve a base address which is then stored back to that variable.
- `format_floating_point_string`: Converts a signed integer and its fractional components into a formatted string representation.
- `execute_conditional_word_moves`: This routine performs a sequence of word moves and calls another routine twice based on a comparison, potentially updating a value in a structure.
- `update_conditional_word`: Compares a word at an offset from a pointer and, if certain conditions are met, moves a word to a stack location and potentially clears a value at an offset.
- `adjust_numeric_value`: Compares a value at an offset of a pointer and performs arithmetic on a value at another offset of that pointer.
- `format_numeric_string_complex`: Converts a numeric value into a string by managing decimal points, fractional digits, and scientific notation.
- `clear_and_branch_5`: Clears a word in a structure and performs a short branch.
- `increment_structure_word`: Increments a word stored in a structure and stores the result back into a different location in that structure.
- `format_numeric_string_2`: Converts a numeric value into a string representation by iterating through digits and handling decimal points and signs.
- `format_numeric_string`: Converts a numeric value into a string representation, handling signs, decimal points, and multiple digits by writing characters to a buffer.
- `write_character_to_buffer`: Writes specific characters to a buffer pointed to by a register, potentially adjusting the character based on a signed value.
- `convert_integer_to_string`: Converts a 16-bit integer value into a null-terminated ASCII string representation.
- `execute_indirect_handler`: Checks a global variable and, if zero, calls several routines to initialize data before performing an indirect call using values from a stack structure.
- `execute_indirect_handler_2`: Performs an indirect call using a retrieved pointer and manages stack-based data after checking a global variable.
- `execute_indirect_handler_3`: Retrieves a function pointer from a structure and executes it after conditional initialization of a global variable.
- `execute_indirect_handler_4`: Uses a global variable to determine whether to perform a series of initialization calls before executing an indirect function.
- `dispatch_indirect_function_call`: Checks a value in memory and, if zero, executes a sequence of other routines to initialize it.
- `dispatch_indirect_function_pointer`: Checks a cached value in memory and, if not present, executes a sequence of routines to populate it.
- `dispatch_indirect_function_call_2`: Checks a value in memory and, if zero, executes a sequence of other routines to populate it.
- `dispatch_indirect_function_call_3`: Checks a value in a memory location and, if zero, executes a sequence of other routines to initialize it.
- `dispatch_indirect_function_call_4`: Checks a cached function pointer in a memory location and executes it if it is zero or invalid.
- `execute_indirect_call`: Retrieves a function pointer from a structure and executes it after performing several conditional checks and assignments.
- `initialize_library_callback`: Loads a library base from a global variable, calls a library function, and executes a stored callback routine if certain conditions are met.
- `chain_result_to_output`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `validate_structure_pointer`: Compares a longword at an offset of a pointer against another value and calls another routine if the comparison fails.
- `increment_and_extract_byte`: Increments a value through a pointer, then extracts and masks a byte from a register.
- `check_structure_bit_and_extract`: Checks a bit within a byte of a structure and, if the bit is set, returns a specific value after restoring registers.
- `process_structure_and_list`: Clears a bit in a structure and iterates through a list of structures to perform operations based on specific bitmasks.
- `set_byte_and_branch`: Performs a bitwise OR operation on a byte at an offset from a pointer and then branches.
- `update_pointer_and_extract_byte`: Updates a value through a pointer using a calculation from another memory location and then extracts a masked byte from a register.
- `call_and_check_result_2`: Calls another routine using a pointer from a register and returns zero if the result is null, otherwise returns the result.
- `push_arguments_and_branch`: Pushes two longwords from an offset of a register onto the stack and calls another routine.
- `compare_strings_and_update_structure`: Compares two null-terminated strings and, if they match, calls another routine and updates a structure with the result.
- `compare_and_increment_pointer_2`: Compares bytes from two different memory locations and increments a pointer based on the result of the comparison.
- `process_conditional_stack_data`: This routine pushes two words onto the stack, calls another routine, and then either clears a register or performs a branch based on the result.
- `update_structure_bytes`: Loads a pointer from a structure offset, writes a byte from a register to an offset within that pointer, then reloads the pointer to write a constant byte to a different offset before branching to another routine.
- `execute_conditional_branch`: Calls another routine and performs a conditional branch based on the result, potentially writing a value to a global variable.
- `dispatch_status_logic`: Evaluates a value from a structure to perform conditional assignments or calls another routine based on specific cases.
- `dispatch_via_table`: Compares a value to a constant and uses it to calculate an offset into a jump table to execute a specific routine.
- `call_library_function_8`: Pushes a parameter onto the stack, calls a subroutine, and then branches.
- `traverse_and_free_linked_list`: Checks a value against a constant and iterates through a list of pointers to perform memory deallocation and pointer updates.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `pass_argument_to_table_search`: This routine passes its argument to a routine that searches a table of pointers and potentially updates an entry.
- `initialize_local_context`: Saves registers to the stack, loads a pointer from an offset of a register, calls another routine, and then loads a value from a global variable into a register before branching.
- `increment_and_compare_value_2`: Multiplies a register by six and enters a loop that increments a value until it reaches a limit stored in a global variable.
- `manage_file_operations`: Performs file system operations including opening, locking, and deleting files based on status flags and error checks.
- `call_library_function_12`: Pushes a parameter onto the stack, calls a subroutine, and branches to another routine based on the result in register d0.
- `call_library_and_branch`: Calls a library function and, if the result is not zero, calls another routine before branching.
- `store_array_element`: Stores a long word and a word into a table at an offset determined by a register, then optionally calls another routine.
- `update_status_flags_2`: Checks specific bits in a value and either calls another routine or writes data to a memory location based on those bits.
- `store_and_recurse_data`: Stores data into a structure and recursively calls itself if a specific bit is set in a byte within that structure.
- `perform_io_validation`: A nineteen instruction routine that calls library functions and validates status against bitmasks and global variables.
- `validate_io_status`: Checks a word in a structure against a bitmask and, if the condition is not met, writes a value to a global variable.
- `perform_io_operation`: Executes an input/output operation and, if it returns an error, calls another routine to handle the error and store the result.
- `move_and_branch_6`: Moves a longword from one register to another and then performs a short branch.
- `adjust_pointer_offset`: Saves registers to the stack, compares a word at a pointer to a constant, and either branches or increments that pointer.
- `traverse_and_modify_list`: Iterates through a list of structures and, for each non-zero entry, performs a series of memory writes and pointer updates.
- `return_stack_value_2`: Returns the value currently stored at the top of the stack.
- `handle_library_error`: Attempts to interact with a library and, if unsuccessful, traverses a linked list to store a value at the end.
- `store_long_word`: Stores a long word from a register into a memory location pointed to by another register.
- `call_alert_2`: Wrapper that calls the exec library function Alert and returns its result to the caller.
- `safe_memory_copy`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_and_initialize_memory`: Clears a block of memory using a loop, updates several global pointers, and performs system calls to handle errors or library functions.
- `return_from_subroutine_2`: An empty routine that performs no operations and returns.
- `parse_and_tokenize_string`: Parses a string by identifying words or quoted substrings, handling whitespace and escape characters.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `set_d0_and_branch`: Loads the maximum signed word value into a data register and performs a short branch to another routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `query_window_attributes`: Performs a sequence of library calls to retrieve and store specific values from a structure into another structure.
- `signed_divide_longwords`: Performs a signed division by calling an unnamed unsigned routine and negating the result when the operand signs differ.
- `negate_and_branch`: Saves registers, negates the values in two registers if they are negative, and then calls another routine before branching.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `divide_longwords`: Performs division on longword values using a loop of shifts and subtractions or direct division.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `execute_recursive_call`: The routine performs a single call to itself before returning.
- `execute_conditional_logic_sequence`: A routine that performs multiple calls to other subroutines including short branches and structure bit testing.
- `short_branch`: Performs a single short branch to another location.
- `execute_conditional_logic_2`: A fifteen instruction routine that calls another routine to increment a pointer and test a bit in a structure.
- `increment_and_copy_byte`: Increments a value at a pointer, copies a byte from an offset to a new location, and then branches.
- `traverse_address_range`: Iterates through a range of memory addresses by incrementing a pointer by a fixed offset and calling another routine.
- `check_pointer_validity`: Loads a pointer from an offset of the stack frame and returns it in register d0 if it is non-zero, otherwise returns negative one.
- `clear_structure_fields_2`: Recursively calls routines based on bit flags in a byte, then clears several fields within a structure.
- `test_structure_bit_and_branch`: This routine sets up a stack frame, saves registers, and branches to a different location if a specific bit in a structure is not set.
- `update_buffer_state_2`: Modifies a structure by clearing bits, resetting integer values, or updating a pointer and a byte value based on various conditional checks and recursive calls.
- `clear_status_flags`: Checks a value at an offset of a pointer and clears specific bits or longwords in a target location if a condition is met.
- `update_buffer_state`: Updates a structure by either copying a byte from a specific offset or incrementing a pointer and updating a table with new values and status bits.
- `update_pointer_and_byte`: Increments a pointer stored in a table, updates a value at an offset within that table by adding a short value, and stores a byte from another location into the memory pointed to by the incremented pointer.
- `find_and_clear_entry`: Iterates through a list of longwords to find an entry where a specific byte is zero, then clears the entire entry.
- `branch_to_offset_2`: Performs a short relative branch to a specific instruction offset.
- `clear_and_branch_3`: Clears three longwords at offsets of a pointer, moves that pointer to a register, and performs a short relative branch.
- `update_parameter_status`: Calls a subroutine and updates various fields within a structure based on the returned value.
- `execute_indexed_callback_4`: Modifies values in a structure and calls another routine using a pointer retrieved from that structure, potentially looping based on the result.
- `load_and_branch_2`: Links a register, saves registers to the stack, loads a value from a global variable into a register, and performs a short relative branch.
- `traverse_and_call_linked_list`: Iterates through a linked list of pointers, calling another routine for each element, and finally clears a global variable.
- `link_and_store_node`: Creates a new node by copying data from a global variable, links it to a list, and updates a global pointer.
- `store_and_branch`: Stores a value from a global variable into a memory location, copies a longword from one location to another, updates the global variable with the new pointer, and performs a short relative branch.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `initialize_pointer_traversal`: Saves registers and loads a pointer from a location relative to a register before branching.
- `traverse_pointer_list`: Compares a pointer to a value in a structure and enters a loop that follows a chain of pointers until a null pointer is encountered.
- `conditionally_copy_long_word`: Moves a long word from a location pointed to by one register to a location pointed to by another register, provided a specific register is not zero.
- `call_library_function_9`: Loads a value from a global variable, prepares two arguments on the stack, calls a routine via a relative jump, and performs a short relative branch.
- `execute_indexed_callback_3`: Validates an index against a table size and, if valid, calls a routine via a pointer retrieved from that table.
- `fetch_and_call_indexed`: Multiplies a value, uses it as an index to fetch a pointer from a table onto the stack, and calls a routine via that pointer.
- `clear_d0_and_branch`: Clears the d0 register and performs a short relative branch to another routine.
- `handle_io_error`: Calls a routine, and if the result is not zero, calls another routine to handle an error and stores the result in a global variable.
- `clear_register_and_branch`: Sets a register to zero and performs a short relative branch.
- `check_library_status_2`: Checks a bit in a word from a structure and updates a global variable based on the result and a library status check.
- `test_bit_and_set_error`: Performs a bitwise AND on a word from a structure and, if the result is zero, writes a value to a global variable.
- `perform_write_operation`: Calls a routine to perform a write, and if it fails, calls another routine and stores the result in a global variable.
- `move_and_branch_7`: Moves the contents of a register to another register and performs a short relative branch.
- `check_library_status`: Calls a library function and, if a specific bit is set in the result and a global variable is zero, calls another routine.
- `call_and_branch`: Calls a routine, sets a register to zero, and performs a short relative branch.
- `execute_stored_callback`: Checks if a global pointer is non-zero and, if so, executes the routine at that address.
- `execute_callback_chain`: Checks various global pointers and executes a sequence of indirect function calls and library functions based on their values.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: Validates an index against a table, calls a routine to close a file if valid, and clears a table entry.
- `call_close_via_table`: Calls a function through a pointer retrieved from a library table.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_function`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `execute_indirect_function_2`: Performs an indirect call to a function pointer retrieved from a nested table structure via an offset from a global base address.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_execute`: Wrapper that calls the dos library function Execute and returns its result to the caller.
- `call_exit`: Wrapper that calls the dos library function Exit and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `execute_indirect_callback`: Retrieves a function pointer from a nested structure within a table and executes it.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_3`: Retrieves a function pointer from a nested structure within a data table and executes it.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested structure within a data table and executes it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_unlock`: Calls a function through a pointer located within a structure.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_4`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function_2`: Calls a library function using an offset from a base address stored in a global variable.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_initialize_structure`: Allocates a memory block via a library call and populates it with provided parameters and a pointer to another routine.
- `modify_data_structure`: Checks a value in a structure, potentially recurses, and then writes specific byte and long values to that structure before calling another routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a pointer from a memory location offset from a global variable and executes the function at that address.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: Retrieves a function pointer from a nested memory structure and executes it.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_function_5`: Retrieves a pointer from a nested memory structure and executes the function located at that address.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_390_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `parse_and_update_data`: Parses a format string and increments a value in a global table while storing a byte into a location pointed to by that table value.
- `increment_and_store_byte`: Increments a value in a global table and stores a byte from a parameter into the location pointed to by the original table value.
- `update_value_via_callbacks`: Calls a routine via a pointer, stores the result in a global variable, calls another routine to update it again, and then executes two more indirect routine calls.
- `execute_indirect_call_4`: Calls a routine via a pointer stored in a structure and passes the result to another routine.
- `dispatch_library_callback`: Checks if a value in a global variable is zero, and if so, updates it by calling another routine.
- `fetch_and_execute_library_offset_callback`: Checks a global variable and, if zero, calls another routine to populate that variable with a returned value.
- `execute_conditional_library_callback`: Checks if a value in a global variable is zero, and if so, updates it with the result of a routine call.
- `validate_and_reset_structure`: Checks a bit in a structure and, if not set or if a sub-routine returns zero, clears specific fields within that structure.
- `branch_to_offset_4`: Performs a short branch to a specific instruction offset.
- `reset_and_update_values`: Subtracts a difference between two values in a pointer-indexed array from a structure member, then clears the first two elements of that array.
- `zero_d0_and_branch`: Clears register d0 and performs a short relative branch to another routine.
- `calculate_array_offset`: Subtracts the difference between the first two elements of an array from a value, or adds the difference between the first and third elements, depending on a bit test.
- `subtract_array_difference`: If the first element of an array is non-zero, subtracts the difference between the first two elements from a stored longword.
- `handle_error_and_status`: Calls a library function and performs a seek operation, conditionally calling another routine to handle errors and updating a global variable.
- `seek_and_handle_error`: Calls a routine to perform a seek, and if it fails, calls another routine to handle the error and stores a value in a global variable.
- `call_routine_with_argument`: Clears the stack, pushes a value from a pointer into it, calls another routine, and then branches.
- `execute_indirect_function_6`: Fetches a pointer from a nested table structure and executes the function located at that address.
- `execute_indirect_call_2`: Performs an indirect call to a function pointer retrieved from a nested memory location.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.

### mathtrans.library

- `trigger_illegal_instruction_exception`: Executes an illegal instruction to force an exception trap.
- `store_register_to_global`: Library code that stores the contents of a register into a global variable.
- `update_linked_list_node`: As library code, it updates a pointer within a structure and executes a function through a pointer if a specific condition is met.
- `set_byte_bit`: Library code that sets a specific bit in a byte located at an offset from a pointer.
- `increment_reference_counter`: As library code, it increments a value in a structure and calls another routine if the value reaches one.
- `return_all_ones`: Library code that returns a value consisting of all bits set to one.
- `decrement_counter_and_call`: Library code that decrements a 16-bit counter at an offset in a structure, and if it reaches zero, conditionally calls a routine based on a flag byte and then calls another routine with a global value.
- `extract_bitfield_value`: Performs a bitwise extraction of a specific field from a source word using a mask and shift operations.
- `calculate_bitfield_value`: As library code, it performs a loop to calculate a value based on bitwise operations and shifts.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `modify_and_call_routine`: As library code, it tests a byte, calls another routine, and then modifies specific bits in register d0 before branching.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_library_register`: As library code, it clears register d1 and performs a relative branch.
- `execute_conditional_pointer_chain`: Performs a sequence of indirect function calls through offsets from a base pointer, where the specific sequence and selection of calls are determined by conditional checks on register values and previous return states.
- `execute_library_jump_table`: Library code that executes a series of function pointers from a table based on register values and conditional logic.
- `execute_conditional_pointers`: Library code that conditionally calls several routines through pointers stored at various offsets from a base address based on register values.
- `call_external_function_with_constants`: Library code calls a routine through a pointer, then conditionally calls two other routines using specific constant values in registers.
- `pass_argument_to_routine`: As library code, it moves a value into d0, calls another routine, and then moves the result back into a register before branching.
- `conditionally_load_constant`: As library code, it checks a long value and either branches or loads a constant into a register before branching.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `perform_bit_shift_loop`: Library code that performs a bit-shifting loop to calculate a value, involving conditional branches and potential recursive calls.
- `perform_bit_shifts`: Library code that performs a logical shift right on a longword and initializes several registers before branching.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_and_branch_library`: Library code that pushes a word onto the stack and performs a short branch.
- `push_all_ones_and_branch`: Pushes a word containing all bits set to one onto the stack and performs a short relative branch.
- `push_constant_and_branch`: Pushes a word constant onto the stack and performs a short relative branch into another routine.
- `validate_and_return`: This library code compares a byte to a constant, branches if it is higher, adjusts the stack pointer, and returns.
- `perform_math_transformations_2`: This library code performs complex arithmetic operations using a loop that processes values from a table and manipulates registers.
- `perform_math_transformations`: As library code, it performs complex arithmetic operations using a loop that processes values from a table and manipulates registers based on bitwise shifts and additions.
- `perform_complex_arithmetic_2`: As library code, it executes a loop performing arithmetic operations on registers and memory values using a pointer to a table.
- `perform_complex_arithmetic`: Library code that executes a loop performing arithmetic operations on registers and memory values using a pointer to a table.
- `perform_mathematical_loop`: Library code that executes a loop performing arithmetic operations on registers and memory values, including conditional recursive calls.
- `set_register_and_branch`: This library code loads a constant value into a register and then performs a short branch.
- `recursive_call_wrapper`: Library code that calls itself and then returns.
- `transfer_and_branch_2`: Library code that saves a register to the stack, calls another routine, restores a register, and performs a relative branch.
- `clamp_register_value`: Library code moves a value, checks if it is negative, and then clamps it to zero if it is within a specific range.
- `shift_and_branch`: This library code performs arithmetic shifts and increments on a register before conditionally branching or jumping.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_recursion`: Library code checks a byte value and, if certain bitwise conditions are met, performs a sequence of recursive calls.
- `rotate_and_branch`: This library code loads a constant into a register, rotates the register right, and then performs a short branch.
- `set_register_and_branch_2`: This library code loads a constant into a register and then performs a short branch.
- `test_and_branch_2`: This library code pops a word from the stack, tests it, and branches based on the sign bit.
- `empty_stub_7`: The library code contains five instructions and performs no observable work; it ends immediately.
- `return_immediately_2`: This library routine consists of four instructions and performs no observable work, immediately returning to its caller.
- `execute_recursive_logic`: Library code that performs multiple recursive calls and conditional operations based on the results and bitwise manipulations of a value.
- `execute_recursive_calls`: Library code that performs four consecutive, identical calls to itself before returning.
- `shift_and_accumulate_table_values`: Library code increments a word, performs arithmetic shifts on two longwords, and enters a loop that adds the shifted values to two other registers while subtracting values from a table and decrementing a counter.
- `subtract_and_accumulate_table_entries`: This library code subtracts two pairs of longwords, then enters a loop that adds values from a table to a register while decrementing a counter until it reaches a sentinel value.
- `test_and_branch_logic`: Library code that tests a byte, branches based on the result, and calls another routine after performing a bitwise AND operation.
- `load_and_branch`: The library code loads a constant into a register and performs a short branch.
- `process_complex_arithmetic`: Library code that performs a loop involving bit-shifting and subtractions, followed by a secondary loop that performs repeated multiplications.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_and_branch_3`: Library code that tests a byte, performs a bitwise AND, and branches to another routine based on the result.
- `process_bit_manipulation`: Library code performs a series of conditional checks and bitwise shifts on a value before executing several other routines.
- `perform_no_operation`: This library routine contains no instructions and performs no operation.
- `manipulate_byte_bits`: Library code that performs a series of swaps, rotations, and logical operations on a byte, using conditional branches based on the overflow flag.
- `modify_byte_via_comparison`: Library code compares a byte against two constants and, if a match is found, performs arithmetic and a rotation on that byte before branching.
- `test_and_branch`: The library code performs a bitwise AND on a register, tests the result, and branches based on the outcome.
- `test_and_shift_byte`: Library code that compares a byte to a constant and performs logical shifts and rotations on a register based on the result.
- `shift_and_rotate`: The library code performs a logical shift left, loads a constant, and rotates a register.
- `clear_and_branch`: The library code clears a register and performs a short branch.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `invoke_callback_and_return_2`: Library code calls a routine using a pointer in a register and returns the original value from another register.
- `execute_callback_and_return_3`: Invokes a routine via a pointer passed in a register and returns the original value from that register.
- `invoke_callback_library`: As library code, it calls another routine and returns the original value from register d0.
- `invoke_callback`: Library code that receives a pointer in a register and a value in another register, calls the routine pointed to, and returns the original value.
- `return_register_value`: Library code that returns the value held in a register.
- `invoke_callback_library_code`: This is library code that invokes a callback routine and returns.
- `execute_callback_and_return_4`: Invokes a procedure via a pointer provided in a register and returns the original value held in a data register.
- `store_register_via_pointer`: This is library code that invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register.
- `execute_callback_and_return`: Invokes a routine via a pointer provided in a register and returns the original value held in a data register.
- `execute_callback_and_return_2`: Calls a routine via a pointer provided in a register and returns the original value held in a data register.
- `invoke_callback_2`: Library code that calls a routine via a pointer and returns the original value from a register.
- `invoke_callback_wrapper_2`: Library code that receives a pointer and a value, calls the routine at that pointer, and returns the original value.
- `invoke_callback_via_library`: Library code calls a routine via a pointer in a register and returns the original value from a data register.
- `invoke_callback_wrapper`: Library code that calls a routine via a pointer in a register and returns the original value from a register.
- `call_library_vector`: Library code calls a routine at a specific offset from a global library base and returns the result in a data register.
- `execute_callback_routine`: Library code executes a function pointed to by a register and returns the initial value of a data register.
- `dispatch_library_function`: Library code performs a call to a routine using a pointer in a register and returns the value held in a data register.
- `call_offset_routine`: Library code calls a routine at an address determined by adding an offset to a pointer stored in a global variable.
- `invoke_offset_function`: Library code executes a routine located at a global pointer plus a specified offset.
- `load_address_and_short_branch`: Library code loads the address of a global variable into a register and performs a short branch to another routine.
- `load_address_branch`: Library code loads a constant address into a register and then executes a short branch.
- `call_global_offset`: Library code calls a routine at an address calculated from a global pointer and an offset.
- `execute_global_offset_call`: Library code performs a call to a routine using an offset applied to a global pointer.
- `load_address_and_branch`: This library code loads a specific address into a register and then performs a short branch.
- `load_pointer_and_branch`: This library code loads a pointer from a global variable into a register and then performs a short branch to another routine.
- `load_address_and_branch_2`: This library code loads another address into a register and then performs a short branch.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_negative_one`: This library routine returns a value of negative one.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_library_structure`: Library code that either modifies a linked list structure or sets a specific bit in a byte within a data structure.
- `set_byte_bit_2`: This library routine sets the third bit of a byte located at an offset from a pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_and_store`: This library routine copies two pointers from a table, executes a function via a pointer, and stores the result or the original pointers into global variables.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_register_data`: This library routine performs a logical OR operation on a data register with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `recurse_with_increment`: This library routine calls another routine recursively.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `execute_icon_handler`: As library code, this routine calls another routine and then returns.
- `perform_incremental_recursion_2`: This library routine performs a recursive call to another routine.
- `call_other_routine`: This library code calls another routine and then returns.
- `dispatch_library_call_2`: This library code routine saves registers to the stack and calls another routine to dispatch a library call.
- `call_string_parser`: Library code that calls a routine to parse a string and copy data to a destination.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_d0_register`: Sets the least significant byte of the data register zero to zero.
- `update_icon_attributes_2`: This library code routine calls several other routines to evaluate conditions, copy data to a structure, and update a pointer within that structure.
- `copy_and_update_structure`: Library code that copies eleven words from one location to another and then performs several single-byte and multi-byte stores to a destination structure, potentially updating a pointer in a global table.
- `return_zero_stub`: The routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_attribute_structure_2`: Updates an icon attribute structure by allocating memory for new data and copying string or pointer information into the structure's fields.
- `update_icon_attribute_structure`: Updates various fields within an icon attribute structure by calling external functions through pointers.
- `update_structure_via_callbacks`: Library code that repeatedly calls functions through pointers to populate various offsets within a structure.
- `execute_callback_loop_2`: Library code that repeatedly calls a function through a pointer and updates a value in a structure based on the returned result.
- `clear_register_and_branch_2`: Library code that clears a register with a constant and performs an unconditional branch.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `recursively_process_icon_structure`: Iterates through a hierarchical icon data structure by traversing nested pointers, tables of entries, and child elements.
- `traverse_icon_structure`: Iterates through a hierarchical icon data structure, processing nested pointers and tables of icon entries.
- `iterate_callback_tables`: Library code that traverses multiple null-terminated tables of pointers, executing callbacks for each entry and updating a counter.
- `iterate_and_callback`: Library code that traverses a null-terminated table of pointers, executes a callback for each entry, and iterates through a secondary list of pointers while performing further callbacks.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `push_pointer_and_branch`: Library code that pushes a longword from a register onto the stack, loads a pointer from a stack offset into a register, increments that pointer by two bytes, and performs a short branch to another routine.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `execute_library_logic`: Library code that evaluates conditions using various library vectors and returns a value based on the results of multiple conditional calls.
- `call_routine_with_arguments`: This library code pushes several registers onto the stack, calls another routine with a pointer and two arguments, and then returns either zero or a value from a register based on the result of that call.
- `return_zero_3`: This library routine is empty and simply returns zero to the caller.
- `update_list_element_2`: Library code that decrements a counter and stores two values into a table at an offset determined by that counter.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `update_icon_attributes`: Library code that conditionally updates icon attribute structures and manages list elements based on library function returns.
- `branch_to_routine_2`: Library code that performs a short branch to another location.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `execute_subroutine_with_parameters`: Library code that saves registers, retrieves two values from the stack, passes one to another routine, and branches based on the result.
- `search_byte_sequence`: This library code calls a routine to compare bytes and then enters a loop that increments a pointer and checks for a specific byte value.
- `compare_byte_sequence`: Library code that increments a pointer and compares values in a loop, returning zero if a specific byte is not found.
- `dispatch_library_call`: Saves registers to the stack, loads two values from the stack into registers, pushes one of those values onto the stack, calls another routine, and then restores the stack pointer before branching.
- `calculate_pointer_difference`: Library code that calls another routine and calculates the difference between a returned value and a value stored on the stack.
- `traverse_linked_list_5`: This library code iterates through a structure, testing values and incrementing pointers or counters based on the results of comparisons and other routine calls.
- `increment_register_if_not_zero`: Library code that tests a register and, if it is not zero, copies it to another register and increments it.
- `test_register_and_restore_context`: Tests if a register is zero and branches if it is not, otherwise clears a register and restores multiple registers from the stack before returning.
- `parse_and_copy_string`: Library code that processes a string to extract a numeric value and then copies segments of data to a destination.
- `process_string_and_branch`: Library code that pushes two pointers onto the stack, calls a routine, and performs conditional branching based on the return value.
- `clear_register_and_branch_3`: Library code that clears a register and performs a short branch.
- `parse_numeric_string`: This library code processes a string to calculate a value, then performs conditional checks and calls other routines based on the result.
- `clamp_value_to_range`: Library code that compares a value from the stack against two constants and returns either a constant or a calculated difference.
- `return_all_ones_3`: This routine returns a value consisting of all bits set to one.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `assign_value_to_list`: Library code that calls other routines to check a condition and, if successful, writes a constant value to a memory location and a pointer.
- `return_zero`: This library routine returns a value of zero.
- `call_subroutine_with_arguments`: This library routine pushes arguments onto the stack, calls a subroutine, and then performs conditional logic based on the result.
- `tiny_move_rts_3`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `call_library_function_with_stack`: Library code that pushes several values onto the stack, calls a routine, and then performs a comparison and conditional zeroing of a register.
- `return_empty`: This library routine performs no actions and returns.
- `rotate_32bit_value_2`: This library routine performs a bitwise rotation on a 32-bit value through sixteen iterations.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_4`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_byte_from_table`: Library code that loads a pointer and a value from the stack, compares a byte from the pointed memory to the value, and decrements the pointer if they match.
- `zero_register_and_branch`: Library code that clears a data register and performs an unconditional short branch.
- `copy_c_string_2`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_string_bytes`: Library code that loops through memory, testing bytes until a zero is found, and then branches based on the result.
- `compare_byte_sequences_2`: Library code that compares a byte from one memory location against a byte from another memory location.
- `set_signed_byte_constant`: Library code that sets a register to a small positive constant or a negative constant based on a comparison.
- `test_byte_and_branch`: Library code that tests a byte at a pointer and branches based on whether that byte is zero.
- `execute_empty_routine`: This library routine is empty and performs no operations before returning.
- `copy_bytes_with_decrement`: This library routine copies bytes from a source to a destination while decrementing a counter, eventually clearing a byte if the counter reaches zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `load_stack_parameters`: Library code that saves registers to the stack and loads three values from offsets of the stack pointer into registers.
- `traverse_linked_structure`: Library code repeatedly calls a routine and, if it returns a non-zero value, follows a pointer offset by twelve bytes within a structure.
- `return_zero_2`: Library code that returns zero.
- `execute_nested_calculations`: Library code that performs a series of conditional subroutine calls and mathematical operations, storing a result in a structure offset.
- `return_negative_one_2`: Library code that returns a value of all bits set.
- `save_registers_and_branch`: Library code that saves several registers to the stack, loads three values from the stack into registers, and performs a short branch.
- `invoke_callback_with_stack_setup`: Library code that pushes several registers and a constant onto the stack, calls a routine, and then performs conditional checks on a register.
- `process_nested_callbacks`: Library code executes a nested loop that calls various routines and updates values within a structure based on their return results.
- `return_all_ones_2`: This library routine returns a value consisting of all bits set to one.

### Iron Lord (1989)(UBI Soft)[cr FLT](Disk 1 of 2).adf:samplesdatas

- `mask_and_set_registers`: This routine performs a bitwise AND on a word and a bitwise OR on a byte.

### A590HD

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_io_request_and_check_status`: Executes a routine via a pointer and then calls another routine to update a data field.
- `setup_io_request_buffer`: Initializes and links two IORequest structures with timing, buffer, and data pointer information.
- `initialize_io_request`: Populates an IORequest structure with timing and buffer data, calls functions via a base pointer, and initializes two linked list elements with specific data fields.
- `call_indirect_offset`: Calls a function through a pointer located at a specific offset from a base address.
- `execute_repeated_calls_2`: Calls a function through a pointer multiple times within a loop and then clears two longwords at a specific offset within two different structures.
- `return_zero_4`: This routine is empty and simply returns a zero value to the caller.
- `initialize_structure_data`: This routine populates several fields of a structure with specific byte and word values, assigns a global variable to a pointer within that structure, and returns a pointer to the structure based on subsequent conditional checks.
- `copy_string_blocks`: Routine that performs multiple passes of copying null-terminated strings and calls functions through pointers in a loop.
- `call_indirect_offset_2`: Calls a function through a pointer located at a specific offset from a base address held in a register.
- `write_multiple_values`: This routine writes several values to memory locations using offsets and registers.
- `calculate_string_length`: Calculates the length of a null-terminated string and returns it.
- `compare_memory_blocks`: This routine calls another routine to push a value onto the stack, compares bytes between two memory locations using a loop, and returns a value based on the comparison result.
- `return_negative_one_3`: This routine returns a value consisting of all bits set.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `call_library_vector_198_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_indirect_offset_medium`: Calls a function through a pointer located at a medium negative offset from a base address.
- `branch_and_call_subroutine`: Saves registers, loads a pointer from an offset, and conditionally calls another routine based on a signed comparison.
- `update_numeric_status`: Updates a value in a structure based on the result of another routine and then increments a word at an offset within a structure while clearing a byte at a different offset within another structure.
- `increment_and_clear_byte`: Increments a word at an offset within a structure and then clears a byte at a different offset within another structure.
- `validate_numeric_input`: Checks if a value is zero when a specific flag is unset, or if it represents a two-digit number where both digits are between zero and seven when the flag is set.
- `clear_register_if_zero`: This routine tests a long word in a register and clears it if the value is zero.
- `search_linked_list`: Traverses a linked list to find an entry where two specific bytes match the digits of a given integer.
- `parse_input_and_update_state`: Parses character input from a buffer and updates a structured data object based on specific key presses.
- `validate_structure_bytes`: Routine that checks specific bytes at offsets within a structure and branches if they do not match expected values.
- `parse_linked_list_structure`: Iterates through a linked list of data structures, validating specific byte patterns and bitmasks within the nodes.
- `validate_and_extract_byte`: Checks a word at an offset from a pointer and, if non-zero, extracts a masked byte from a secondary pointer and compares it to a constant.
- `evaluate_byte_and_loop`: Compares a byte to a constant and branches if not equal.
- `evaluate_multiple_conditions`: Routine that checks a byte against multiple constants and executes a loop if certain conditions are met.
- `insert_linked_node_2`: Routine that initializes a structure and inserts it into a linked list by updating pointers.
- `insert_linked_node`: This routine updates pointers to insert a new node into a linked list and calls two other routines via pointers.
- `call_library_function_10`: This routine calls a library function using an offset from a base address and modifies a pointer register.
- `insert_linked_list_node`: Inserts a new node into a linked list and updates the surrounding pointers.
- `initialize_complex_structure`: Populates a structure with various values and calls several functions through pointers retrieved from a base address.
- `decrement_short_value`: Decrements a 16-bit value stored at a specific offset within a structure.
- `query_timer_device`: Interacts with a device to check a status and executes two different routines if a returned value is zero.
- `process_structured_data`: This routine checks a byte in a structure and, if certain conditions are met, iterates through a table to calculate a sum before returning a result.
- `sum_table_elements_2`: Iterates through a table of integers to calculate their sum and returns zero if the sum is non-zero.
- `sort_and_copy_data`: Routine that manages a data structure, performs a loop to iterate through a table, and copies values between memory locations.
- `sort_data_table_2`: This routine performs a bitwise-based sorting algorithm on a table of values located at an offset from a pointer.
- `process_event_chain_and_update_state`: Iterates through a linked list of structures, performing conditional checks on flags and executing function pointers stored within the elements.
- `process_event_queue_and_update_state`: Iterates through a linked list or queue of structures, performing conditional checks on flags and calling various subroutines based on the data contents.
- `allocate_and_link_event_node`: Checks a resource identifier and, if necessary, allocates a new structure.
- `insert_linked_node_3`: Inserts a new node into a linked list and performs a bitmask-based copy of data from the node to another location.
- `process_structured_data_and_copy`: Checks a byte in a structure and, if certain conditions are met, iterates through a table to calculate a sum.
- `copy_data_and_update_pointers`: Copies a block of data from a source offset to a destination, then updates several fields within a structure.
- `sum_and_recurse_table_values`: Iterates through a table to accumulate a sum while decrementing values and updating pointers.
- `nop_placeholder_routine`: This routine is empty and performs no operations.
- `traverse_nested_loops`: Executes nested loops to traverse data structures, checking specific byte values and bit flags to return a counter.
- `sort_data_table`: Routine that performs a bitwise-based sorting algorithm on a table of values located at an offset from a pointer.
- `increment_and_branch_2`: Increments a 32-bit register and performs a short relative branch.
- `return_from_subroutine_3`: This routine performs an immediate return to the calling routine.
- `initialize_event_node_structure`: Allocates and populates a new event structure by retrieving data from a source object and setting various status flags, function pointers, and linked list pointers.
- `process_event_queue`: Checks a condition involving a pointer, then repeatedly calls a routine to retrieve an object and executes a function pointer from that object or processes a list of routines.
- `update_state_structure`: Checks specific bits in a byte to conditionally clear a value and copy multiple bytes from one structure to another.
- `initialize_and_process_event_node`: Allocates and initializes a new event structure by setting various status flags and pointers within a parent state structure.
- `process_state_transition_table`: Reads a byte from a data structure to either execute a function from a jump table or update a series of state parameters and control flags.
- `write_parameter_sequence`: Clears a byte in memory and then performs a series of writes to two specific memory offsets using various immediate values.
- `execute_table_function`: Calculates an index using bitwise operations on registers, retrieves a function pointer from a table, and calls it with a modified argument.
- `update_event_linked_list_2`: Manages a circular doubly linked list of event structures by inserting new nodes and updating pointers.
- `process_event_state_machine`: Evaluates conditions within an event structure to determine the next state or trigger a callback.
- `update_status_values`: Tests bytes at specific offsets from two different pointers and, if certain conditions are met, writes several constant bytes to memory locations relative to those pointers.
- `traverse_linked_list`: Traverses a linked list searching for a matching byte and returns a pointer to a specific offset within a node.
- `reset_status_flags`: Writes specific byte values to two different memory locations and waits for a bit to be set in a third location.
- `update_memory_sequence`: Writes a series of specific byte values to two different memory locations and then copies a block of data from a pointer into a third location using a loop.
- `store_and_loop`: Stores a byte from a pointer into a memory location and decrements a counter for a loop.
- `clear_flag_and_word`: Sets a byte to zero at one location and clears a 16-bit word at another.
- `initialize_complex_structure_2`: Performs a series of pointer assignments and byte copies to initialize multiple linked data structures and internal offsets.
- `update_buffer_size_and_metadata`: Calculates the difference between two memory values and updates a buffer pointer and its length.
- `decrement_and_increment_memory`: This routine decrements a register in a loop until a specific bit is cleared, then increments two memory locations and updates one of them with a byte from a pointer stored in another memory location.
- `update_memory_values`: This routine tests a bit in a value, compares a register against a constant, and then adds a register value to two different memory locations before updating pointers.
- `update_state_and_counters`: Increments two memory locations by a value in a register and performs a bit shift to update a third location.
- `update_multiple_values`: Updates several memory locations by adding a value to an existing integer, performing a bit shift, and storing a sequence of bytes into a table.
- `process_event_linked_list`: Iterates through a linked list of structures to evaluate status flags and perform conditional pointer swaps.
- `process_event_linked_list_node_updates`: Iterates through a linked list of event structures to evaluate status flags and perform conditional pointer swaps or updates.
- `process_linked_list_event`: Iterates through a linked list of structures to check status flags and perform conditional pointer swaps.
- `modify_list_element`: Updates a byte in a structure pointed to by a register and conditionally writes a different byte value based on a comparison.
- `process_linked_list_state_machine`: Manages a state machine by traversing a linked list of structures and performing conditional updates to their members.
- `recalculate_buffer_metadata`: Calculates the size of a data buffer by subtracting two memory values and updates the buffer's start pointer and length.
- `fetch_and_store_byte`: Fetches a byte from a table using an offset derived from a pointer and stores it into two different memory locations.
- `dispatch_event_callback`: Iterates through a linked list of event structures to match a specific identifier against a table of function pointers.
- `process_event_linked_list_2`: Iterates through a linked list of structures to evaluate status flags and execute conditional logic.
- `process_event_linked_list_3`: Iterates through a linked list of structures to evaluate status flags and perform conditional pointer swaps or updates.
- `update_event_linked_list`: Iterates through a linked list of structures to update status flags, perform conditional pointer swaps, and copy data bytes.
- `loop_conditional_copy`: Clears a byte at an offset from a pointer and enters a loop that decrements two registers based on a bit test.
- `copy_bytes_conditionally`: Copies five bytes from a source pointer to a destination table, provided a specific bit in a control byte is set.
- `process_event_linked_list_4`: Iterates through a linked list of structures to evaluate status flags and perform conditional pointer swaps.
- `apply_dos_relocation_hunks`: Iterates through a series of memory segments to perform relocation and data patching.
- `copy_data_conditionally`: Compares two words and, if the first is not greater than the second, stores a zero into a memory location pointed to by a register and increments both the pointer and the comparison value.
- `execute_conditional_loop`: Executes a loop that calls a subroutine via a pointer and performs conditional branching and pointer assignments based on the returned value.
- `execute_conditional_subroutine_2`: Checks if a register is non-zero, calls a subroutine, and then performs several pointer assignments and a second subroutine call based on values from the stack.
- `return_offset_value`: Returns a 32-bit value stored at an offset from a pointer held in a register.
- `branch_on_register_status`: Checks if a specific register contains a non-zero value to determine whether to call another subroutine.
- `call_subroutine_with_stack_cleanup`: Adjusts the stack pointer, saves several registers, and calls a subroutine via a pointer; if the return value in a register is four, it restores a value from the stack before returning.
- `execute_conditional_subroutine_3`: Calls a subroutine via a pointer and performs conditional logic based on the return value.
- `execute_unreachable_branches`: Performs conditional and unconditional calls to another routine based on the value of a register.
- `update_pointer_data`: Calls a subroutine via a pointer and, if the result is non-zero, stores a calculated value into the memory location pointed to by the result.
- `tiny_movea_move_rts`: Small routine of 3 instructions that copies d0 to a0, then copies (a7)+ to (a0)+, then returns.
- `call_stored_function`: Checks if a register contains a non-zero value and, if so, executes the function pointed to by a value at a specific offset.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:fgaulbaire.1989

- `initialize_bitplane_copper_list`: Allocates memory for a copper list and populates it with a sequence of display parameters, including bitplane offsets, color values, and display timing commands.
- `manage_control_flow_loop`: Iterates through a table of bytes while checking for specific values and updates a global variable and a counter.
- `set_jump_table_entry`: Writes two 16-bit values to a table and updates a global variable while enabling DMA via the DMACON register.
- `set_multiple_jump_entries`: Writes several 16-bit values to a table and updates a global variable while enabling DMA via the DMACON register.
- `perform_logical_or`: Performs a bitwise logical OR operation on a register with zero.
- `set_extended_jump_entries`: Writes a series of 16-bit values to a table and updates a global variable while enabling DMA via the DMACON register.
- `manage_file_and_interrupts`: Performs file operations and updates several global variables while manipulating DMA and interrupt enable registers.
- `fill_global_with_decrement`: Repeatedly writes a decrementing value to a global variable, which affects a hardware color register.
- `combine_source_words`: Iterates two thousand times, performing a bitwise OR on four values from a source table and storing the result in a destination table.
- `modify_pixel_data`: Iterates through a large buffer and updates a table of values by incrementing specific nibbles based on comparisons with the buffer content.
- `clear_d0_register_2`: Performs a bitwise OR operation on a register with zero.
- `decrement_table_values`: Performs a nested loop that subtracts values from specific bit segments of words stored in a table.
- `update_interrupt_handler_table`: Checks a hardware status bit and performs a conditional swap of two 16-bit values within a table structure.
- `configure_hardware_state`: Clears a memory location and conditionally swaps two 16-bit values in a table before writing to a register.
- `update_interrupt_status`: Checks specific bits in a register and decrements global counters, updating a hardware register if a counter reaches zero.
- `clear_data_byte`: Sets the least significant byte of a data register to zero.
- `call_function_and_update_globals`: Calls a function via a pointer and then updates four global memory locations using a constant value and bitwise OR operations.
- `manage_library_resources`: Updates several global variables, calls a function via a pointer, and interacts with library-related memory and system calls.
- `configure_blitter_registers`: Waits for a bit in a global variable to clear before writing several values to hardware registers, including blitter and DMA registers.
- `populate_blitter_registers`: Polls a global variable until a bit is cleared, then populates hardware registers with specific values from memory.
- `update_audio_and_blitter_parameters`: Calculates new pitch and volume values based on global variables and updates audio hardware registers.
- `store_register_pairs`: Searches a table for a zero word and, upon finding it, stores two input words into the table at that location.
- `process_table_data`: Iterates through a table to perform arithmetic on its elements and updates various memory locations based on specific conditions.
- `calculate_and_store_offsets`: Calculates several memory offsets using arithmetic on input registers and stores the results into a structure in memory.
- `update_input_state`: Reads joystick and button data from hardware registers to update several global status variables.
- `load_and_initialize_graphics_assets`: Loads graphic data from disk into memory and populates various tables with sprite and decoration metadata.
- `execute_subroutine_sequence_2`: Performs a sequence of subroutine calls, including conditional execution and a loop that waits for a specific value in a global variable.
- `select_parameter_values`: Sets two different values in memory based on the value of a global variable and then calls another routine.
- `initialize_data_structures`: Sets multiple values in a structure and performs a loop to populate a table with incrementing memory addresses.
- `setup_blitter_transfer`: Configures hardware registers for a blitter operation and fills a table with incrementing memory addresses through a loop.
- `search_and_copy_data`: Searches for a byte within a memory range and copies matching bytes into a destination table while calculating offsets.
- `process_text_input`: Evaluates a character from a buffer to update a text string, manage a cursor position, or reset state variables.
- `update_buffer_index`: Adjusts a global index value based on its current range and performs bitwise operations to update another global variable.
- `validate_and_copy_data_block`: Validates a data structure by checking specific byte values and bitwise conditions before performing a memory copy.
- `modify_structure_fields`: Writes specific values to a structure and enters a loop that waits for a bit in a memory location to change.
- `return_from_subroutine_4`: This routine performs a single return instruction to exit the current execution flow.
- `check_condition_status`: Compares values from a pointer and a register, then performs bitwise checks to return a status value.
- `compare_and_branch_data`: Sets a register to a constant, calls another routine, compares a value from a pointer against a register, and performs a conditional branch.
- `interleave_bits`: Combines bits from two consecutive words in a table using bitwise shifts and masks.
- `validate_data_structure`: Searches a memory block for a specific value and then validates a sequence of bit-swapped words against an incrementing counter.
- `initialize_data_structures_2`: Calls a subroutine and then performs multiple writes to various offsets in a table or structure, including clearing some words and setting others to specific values.
- `wait_for_bit_flag`: Polls a specific bit in a byte located at an offset from a pointer until that bit is cleared.
- `clear_bit_and_branch`: Clears a specific bit in a byte pointed to by a register and then branches to another routine.
- `set_bit_and_branch`: Sets a specific bit in a byte pointed to by a register and then branches to another routine.
- `write_encoded_byte`: Calculates an encoded byte from an input and writes it to a memory location while waiting for a bit flag.
- `execute_conditional_subroutines`: Tests a bit in a byte moved from a register and conditionally calls two different subroutines.
- `modify_bit_and_decrement`: Sets a specific bit in a byte pointed to by a register, decrements a word in a data register, and branches to another location.
- `modify_bits_and_branch`: Clears and then sets specific bits in a byte pointed to by a register while incrementing a value in a data register before branching.
- `push_and_test_bit`: Pushes a value onto the stack and tests a bit in a byte pointed to by a register.
- `clear_memory_and_copy_data`: Fills a memory range with zeros and then conditionally copies a block of data from the current instruction location to a destination buffer.
- `clear_memory_and_call`: Fills a block of memory with zeros and then executes a routine at a fixed address.

### Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:wargame

- `initialize_game_environment`: Clears a local stack area and initializes several global variables.
- `call_function_in_loop`: Iterates through a loop, calling a function via a pointer with varying arguments derived from a table and registers.
- `parse_string_to_integer_array`: Parses a string containing numeric characters into an array of bytes, handling potential negative signs and digit conversion.
- `execute_indirect_jump_via_table`: Calculates an offset by multiplying a value in a data register by eight and uses the result to call a function from a jump table.
- `return_from_subroutine_5`: The routine performs no operations and immediately returns to the caller.
- `return_immediately_3`: This routine performs no operations and returns to the caller immediately.
- `perform_no_operation_2`: This routine is empty and performs no actions before returning.
- `return_empty_subroutine`: This routine performs no operations and immediately returns to its caller.
- `return_from_subroutine_6`: the routine performs no operations and immediately returns to the caller.
- `return_from_subroutine_7`: The routine is empty and performs no operations before returning.
- `return_from_subroutine_10`: The routine performs a single instruction that returns control to the calling function.
- `return_from_subroutine_8`: This routine performs an empty operation and returns to the caller.
- `perform_empty_operation`: This routine performs no operations and immediately returns to the caller.
- `execute_indirect_call_5`: Loads a function pointer from a table and executes it.
- `sync_and_update_entity_data`: Copies several fields from one data structure to another and performs conditional updates to memory locations based on flag checks and pointer offsets.
- `compare_stack_word_and_branch`: Loads a word from a stack offset and compares it against two constant values, branching to different locations if a match is found or returning if no match occurs.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `clear_structure_fields_3`: Tests a byte at a stack offset and, if non-zero, clears two longwords in a structure pointed to by another stack offset before branching.
- `call_library_function_11`: Loads a pointer from a table, restores a stack frame, calls a subroutine via a pointer, and then checks a register before returning.
- `calculate_bit_rotated_quotient`: Performs a bitwise rotation on a byte from a global variable and calculates a quotient and remainder based on that value and an input parameter.
- `initialize_memory_offsets`: Sets several memory locations with offsets derived from a register, populates a table with incrementing values, and configures a structure with various constants.
- `process_offset_value`: Loads a value from a stack offset, retrieves a byte from a table using that value, and performs arithmetic and conditional branching based on the result.
- `increment_memory_value`: Increments a 16-bit value stored at a location relative to a pointer and returns a calculated product.
- `copy_and_setup_blitter`: Initializes several memory locations, performs a loop to add a constant to a series of values in a table, and then calls multiple subroutines via pointers.
- `execute_indirect_calls`: Retrieves function pointers from a table via an offset and executes them sequentially.
- `process_input_and_update_state`: Reads joystick input and evaluates bitmask flags from a structure to perform conditional pointer updates and data transfers.
- `update_linked_list_and_metadata`: Iterates through a linked list of structures to check status flags and perform conditional pointer swaps.
- `process_metadata_block`: Clears a byte at a negative offset from a pointer, loads a longword from a memory location, and performs several conditional checks.
- `process_entity_metadata_list`: Iterates through a linked list of data structures to compare and update specific fields based on conditional flags and string comparisons.
- `search_for_non_zero_value`: Iterates through a table of integers to find a non-zero value and then checks for a negative value.
- `update_byte_conditionally`: Checks a byte at a pointer offset, branches if it is zero, and otherwise writes a value to that location and increments the pointer.
- `load_word_repeatedly`: Loads a 16-bit word from a location determined by a pointer and an index into a register multiple times.
- `reconcile_block_metadata_counters`: Iterates through a list of data blocks to compare and decrement counter values based on a reference value.
- `decrement_and_branch`: Decrements a word at an offset from a pointer and branches based on the result.
- `compare_and_decrement_value`: Compares a word from a table with a register and performs conditional branches.
- `process_conditional_branch`: Decrements a word via a pointer, clears a longword, calls another routine, and performs a series of conditional checks and branches based on memory values.
- `update_entity_state_table`: Iterates through a table of data structures to modify specific byte and word values based on matching criteria.
- `check_byte_and_set_register`: Loads a byte from an offset of a pointer, compares it to zero, and branches if it is non-zero.
- `compare_and_branch_alt`: Compares a byte in a register to one and branches if they are not equal.
- `test_byte_and_set_register`: Compares a byte in a register to a constant and branches if they are not equal.
- `update_entity_state_and_timers`: Updates entity status flags and decrements internal timers based on a scaled quotient.
- `calculate_scaled_decrement`: Decrements a value on the stack and subtracts a quotient derived from a word at an offset of a pointer from a global variable.
- `manage_linked_list_and_decrement_counters`: Updates a linked list by reassigning pointers and copying data fields from one structure to another.
- `execute_nested_loops`: Decrements a value on the stack and performs multiple nested loops involving indirect function calls and updates to global memory locations.
- `execute_indirect_loop_2`: Sets a byte in memory, calls another routine, and then executes a loop that performs indirect calls through a pointer retrieved from a structure.
- `update_local_counter`: Increments a local variable and compares it against a constant value.
- `execute_indirect_jump_2`: Calculates an address by multiplying a value from a local variable by a constant offset and then performs an indirect call to that location.
- `nop_padding_sequence`: The routine is empty and performs no operations.
- `update_counter_bounds`: Compares two bytes in a structure and increments or decrements one of them while adjusting a 16-bit value within specific limits.
- `prepare_blitter_area_data`: Calculates memory offsets for a series of data buffers based on dimensions and stores them in a table.
- `shift_memory_left`: Shifts a block of seven bytes left by one position within a structure.
- `shift_memory_left_2`: Shifts a block of seven bytes left by one position starting from an offset in a structure.
- `iterate_and_update_block_metadata`: Iterates through a sequence of blocks using a counter, updating specific global values and calling a routine to recalculate coordinate and status bit-fields for each entry.
- `update_buffer_metadata`: Stores two shortwords into a structure and calculates a bit-packed value to update two destination pointers.
- `empty_routine_2`: This routine contains no executable instructions.
- `update_state_machine_logic`: Iterates through a sequence of bytes to update a state variable based on specific control characters.
- `convert_number_to_string`: Converts a numeric value into a decimal string and handles leading zero formatting.
- `move_and_push_address`: Moves a word from a memory location indexed by a register into a data register and pushes a relative address onto the stack.
- `update_state_from_data_stream`: Reads a sequence of bytes from a data structure to modify internal state variables.
- `execute_indirect_loop`: Iterates through a loop based on a counter, performing an indirect call to a function pointer retrieved from a structure via a pointer.
- `return_immediately_4`: This routine performs no operations and returns to its caller immediately.
- `execute_indirect_call_3`: Performs an indirect call to a function pointer calculated from a register.
- `read_joystick_input`: This routine reads the current state of the first joystick port via a hardware register.
- `process_joystick_input_state`: Processes a joystick input value by applying bitwise transformations and updating a state structure with new direction values.
- `update_status_flags`: Calls two external functions to update specific word values in a structure and then sets a high bit in a byte at a fixed offset.
- `calculate_blitter_buffer_offsets`: Calculates a series of memory addresses for blitter operations by applying a multiplier and bit-swapping logic to input values.
- `calculate_blitter_source_offsets`: Calculates a series of memory offsets by applying a bit-swapping transformation and a multiplier to input values.
- `parse_iff_resource`: Parses an IFF file structure to locate and extract specific data chunks.
- `zero_data_block`: Contains a sequence of instructions that clear the data register.
- `copy_data_from_table`: Iteratively copies various word and long values from a table into several global variables.
- `pop_stack_elements`: Extracts a series of values from a stack-like structure pointed to by a register and stores them into several global variables.
- `setup_blt_registers`: Configures several hardware registers associated with the Blitter and performs library calls.
- `check_byte_condition`: Reads a byte from an offset in a structure and calls another routine if the value is non-negative.
- `update_structure_fields_2`: Reads a character, stores it in a structure, and if non-negative, updates several other fields within that structure using library calls.
- `update_structure_fields_3`: Writes a byte, a longword, and a word into a memory structure pointed to by a register at specific offsets.
- `clear_d0_register_3`: Performs a logical operation to clear the contents of a register.
- `clear_data_register_byte`: Sets the lowest byte of a data register to zero.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.

### 9

- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 100 into d0, then returns.

### diskfont.library

- `set_font_handle`: Library code that stores a value into a structure and calls another routine if a condition is met.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `open_font_library`: Library code that attempts to open a library and initializes a structure with a pointer if successful.
- `unload_font_resources`: This library routine iterates through a linked list of resources to remove those with no remaining references.
- `update_font_resource_status`: Checks a status flag and a counter within a data structure to determine if a resource is active.
- `set_bit_and_clear_d0`: Sets a specific bit in a memory location and clears a register.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `check_library_status_3`: This library routine decrements a value at an offset from a pointer and branches based on the result, then checks a bit in another offset before jumping to a destination.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_scaled_offset`: Library code that computes an integer from two structures using subtraction, scaling, and a loop that subtracts values from a global table based on byte comparisons, then returns the result.
- `load_and_install_disk_font`: Opens a disk font file, reads its header, allocates memory for the font data, and loads the segment.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read_2`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock_2`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_2`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir_2`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err_2`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `branch_to_offset`: This library routine loads two values from the stack into registers and performs a short branch to a specific offset.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `compare_byte_strings`: Compares two null-terminated strings and returns a value indicating their relative order.
- `skip_zero_bytes`: This library routine tests a byte at the pointer in a register and branches if it is non-zero, otherwise it clears a register and branches elsewhere.
- `compare_and_branch_byte`: This library routine compares a byte from a pointer with a value in a register and branches based on the result.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `load_and_branch_3`: Loads a long word from an offset of the stack into a register, loads a byte from another stack offset into a data register, and performs an unconditional branch to a relative offset.
- `compare_and_move_pointer`: This library routine compares two bytes and, if they are not equal, moves a pointer from a register into another register before branching.
- `increment_and_check_byte`: This library routine increments a pointer, compares the byte at the new location with a register, and branches if the byte is non-zero.
- `find_last_occurrence_in_string`: The routine receives a pointer to a zero-terminated string and a byte value, determines the string length via a library call, then walks backwards from the end to find the last occurrence of the byte and returns its pointer.
- `find_delimiter_and_advance_pointer`: This library routine searches a string for delimiters and advances the pointer past the first found delimiter, returning the adjusted pointer.
- `parse_font_metadata_stream`: Iterates through a linked list of font resource descriptors to extract character string data and metadata.
- `initialize_loop_parameters`: This library code loads a pointer from a table into a register, sets a register to a constant value, and performs a short branch.
- `parse_font_data_stream`: Iterates through a linked list of data structures to extract and copy character strings into a destination buffer.
- `process_font_resource_stream`: Iterates through a stream of font metadata to extract character information and string data.
- `parse_font_resource_stream`: Iterates through a font resource stream to extract character metadata and string data.
- `populate_font_metadata_table`: Iterates through a parsed font resource stream to extract character metadata, including offsets and attributes, and stores them into a destination table.
- `parse_font_resource_data`: Iterates through a data stream to identify specific font resource markers and extracts character metadata.
- `parse_font_metadata_and_strings`: Iterates through a font resource stream to extract character metadata and string data.
- `return_zero_stub_2`: As part of library code, this routine returns zero and performs no operations on registers, memory, or hardware.
- `build_disk_font_table`: Parses a font file from disk to populate a memory-allocated table.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `execute_empty_call`: This library code performs an unconditional call to another routine and then returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:strt2

- `initialize_system_state`: Clears a block of memory with zeros and performs conditional calls to external functions based on system flags and return values.
- `initialize_asset_loading_system`: Initializes a data structure for file loading and unpacking, then iterates through a table to process multiple assets.
- `load_and_unpack_file_data`: Opens a file, reads data into a buffer, and unpacks interleaved bytes into a destination table using a loop.
- `reset_audio_hardware`: The routine clears several audio length and location registers and modifies the DMA control register.
- `call_and_set_flag`: The routine calls another routine and then stores a value into a memory location.
- `reset_audio_and_set_flag_2`: This routine resets the audio hardware and sets a byte at a specific offset relative to a register value to one.
- `read_data_into_buffer`: This routine reads data from a source into a destination buffer using a loop and various library calls for file operations.
- `allocate_and_initialize_string_buffer_2`: Calculates a required buffer size based on a pointer and length, then allocates a new memory structure.
- `copy_register_d0_to_d5`: This routine copies the contents of register d0 into register d5.
- `parse_and_initialize_dos_file_structure`: Compares two null-terminated strings and, if equal, traverses a linked list of pointers to calculate a count and populate a new data structure with the resulting list elements.
- `execute_indirect_callback_chain`: This routine performs a sequence of indirect calls by traversing nested pointers and executing a series of callback functions stored in a table.
- `copy_data_register_d0_to_d5`: Copies the contents of one data register to another data register.
- `check_flag_and_call`: This routine checks if a byte at a specific offset relative to a register is non-zero and calls another routine if it is.
- `store_library_results`: This routine retrieves two values from a global structure and stores them into a destination pointed to by a register.
- `push_parameters_to_stack`: This routine copies four passed parameters onto the stack before calling another routine.
- `push_multiple_parameters_to_stack`: This routine copies twelve passed parameters onto the stack before calling another routine.
- `parse_string_and_interleave_bits`: Parses a formatted string and then interleaves the bits of two words using another routine.
- `check_and_load_pointer`: Compares a word at an offset from a pointer to a constant and, if equal, loads a value from a table into a register.
- `test_and_load_value`: Tests a word at an offset from a pointer and, if not negative or zero, loads a word from a table pointed to by another offset into a register.
- `process_signed_value_and_populate_array`: Determines the sign of a value from a structure, stores a sign flag, and enters a loop that populates an array by fetching bytes from a table using values returned by another routine.
- `parse_formatted_string`: Parses a string containing format specifiers to extract and convert data types.
- `chain_result_to_output_2`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write_2`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_552_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `copy_memory_block`: Copies a block of memory from a source to a destination, handling both forward and backward copies depending on the direction of the pointers.
- `return_from_subroutine_9`: This is an empty routine that performs no operations before returning.
- `parse_quoted_string_tokens`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `load_register_value`: loads a value into a register and performs a relative branch.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values_2`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `open_window_resource`: Opens a file using a DOS call and retrieves window information from a library, storing the resulting pointers and values into a structure.
- `signed_divide_longwords_2`: Performs a signed division by calling an unnamed unsigned routine and negating the result when the operand signs differ.
- `process_signed_values`: Clears a register and performs signed negation on two input values before calling another routine.
- `concatenate_division_result`: Calls a division routine and returns a 64-bit value by concatenating the original contents of a data register with the resulting value left in that same register.
- `call_recursive_division`: Calls a routine to perform 32-bit division and then calls itself, resulting in infinite recursion.
- `divide_longwords_2`: performs division on 32-bit values using a loop or direct arithmetic.
- `swap_and_rotate_words_2`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `call_interleave_bits`: The routine calls another routine that interleaves the bits of two words pointed to by a register.
- `execute_multiple_subroutines`: This routine calls three different subroutines to perform conditional checks, update a byte via a pointer, and potentially store a calculated address into a global variable.
- `branch_to_offset_5`: Performs a short relative branch to a nearby instruction.
- `execute_table_operations`: A fifteen instruction routine that calls other routines to update a byte via a pointer and test a bit in a table entry.
- `update_byte_via_pointer`: Increments a pointer in a table and stores a byte from a source offset into the location pointed to by the incremented pointer.
- `traverse_code_block`: Iterates through a sequence of instructions or data by incrementing a pointer by a fixed offset.
- `check_pointer_and_return`: Tests if a pointer retrieved from a structure is non-zero, returning either a specific constant or the value of the pointer.
- `clear_structure_data`: Checks specific bits in a byte at an offset to conditionally call other routines, then clears four consecutive words and a byte in a structure.
- `test_and_store_pointer`: Tests a bit in a table entry and, if not set, stores a calculated address into a global variable.
- `process_conditional_logic`: Evaluates conditions based on values in a table and updates pointers or clears data accordingly.
- `clear_conditional_data`: Compares a word at an offset from a pointer, and if it matches a specific value, clears two longwords at offsets from another pointer before branching.
- `process_conditional_list`: Updates a structure based on various conditions, involving recursive calls, bitwise operations, and pointer manipulations.
- `update_list_element`: Updates a value in a structure by adding a short integer to a long integer, modifies a bit flag, and writes a byte to a pointer retrieved from the structure.
- `clear_structure_elements`: This routine iterates through a list of structures, checking a byte at an offset, and clears three consecutive longword fields if a specific condition is met.
- `branch_to_previous`: Performs a short relative branch to a previous instruction.
- `clear_structure_fields`: Clears three consecutive longword fields at an offset from a pointer and moves that pointer into a register.
- `update_parameter_status_2`: Modifies a parameter structure by setting specific word and byte values based on the result of another routine.
- `update_and_call_validation`: Writes a word and sets a bit in a structure, then calls another routine and conditionally sets a bit based on the result.
- `load_pointer_from_table`: Saves registers and loads a pointer from a table into a register before branching.
- `traverse_linked_list_2`: iterates through a linked list and clears a value at a specific offset in a global structure.
- `update_structure_pointer`: This routine stores a pointer to itself in a location, calls another routine, and if successful, updates a structure with a longword and a value from a register.
- `transfer_structure_data`: Copies a longword from a table to a destination, moves a value from a source to a specific offset in the destination, and updates a table entry with the destination pointer.
- `forward_parameter_field_2`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `calculate_offset_address`: Subtracts one register from another and loads a value from an offset relative to a pointer.
- `traverse_linked_list_3`: Iterates through a linked list by following pointers until a null value is encountered.
- `copy_pointer_conditionally`: Copies a longword from one location to another if a specific condition is met.
- `push_pointer_and_value_to_call`: Pushes a value derived from a pointer and the pointer itself onto the stack before calling another routine.
- `validate_and_recurse`: Checks if a parameter is within a specific range and verifies a value in a table before recursively calling another routine.
- `fetch_and_call_via_table`: This routine multiplies a value from a structure, uses the result to fetch a pointer from a table, pushes it onto the stack, and calls the routine at that address.
- `clear_and_branch_4`: Sets register d0 to zero and branches to another routine.
- `execute_and_handle_error`: This routine calls a routine via a pointer, checks the result, and if non-zero, calls another routine and stores the result in a global variable.
- `clear_d0_and_branch_2`: Clears the d0 register and performs a short branch to another routine.
- `execute_conditional_logic`: A routine consisting of nineteen instructions that calls other routines and is invoked by multiple callers.
- `test_bit_and_branch`: Tests if the third bit of a word retrieved from an offset of a pointer is set.
- `call_with_error_handling`: This routine calls another routine with three parameters and, if the result is negative one, calls a second routine and stores its result in a global variable.
- `move_and_branch_8`: Copies the contents of register d5 into register d0 and branches to another routine.
- `check_flag_and_call_2`: This routine calls another routine with specific arguments and, if a bit in the return value is set, calls a third routine if a global variable is non-zero.
- `call_and_branch_2`: Calls another routine and then branches to another routine after clearing register d0.
- `execute_callback_and_routine`: Calls a function pointer stored in a structure and then calls another routine.
- `traverse_resource_tree`: Recursively processes a hierarchical data structure by checking various pointers and offsets within a control block.
- `return_immediately_5`: This routine performs an immediate return to the caller.
- `validate_and_clear_table_entry`: Checks if an index is within the bounds of a table and if the entry at that index is non-zero.
- `call_dos_close`: calls a function from a library via an offset in a global structure.
- `call_close_3`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc_2`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_function_7`: Retrieves a function pointer from a nested offset within a table and executes it.
- `call_current_dir_3`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay_2`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `execute_indirect_function_8`: Loads a function pointer from a nested structure within a global table and executes it.
- `call_delete_file_2`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock_2`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_execute_2`: Wrapper that calls the dos library function Execute and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err_3`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive_2`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `execute_indirect_function_9`: Retrieves a function pointer from a nested table structure via two offsets and executes it.
- `call_open_3`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_read_3`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek_2`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock_3`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_168_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_246_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_204_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_function_3`: Performs an indirect call to a function located at an offset within a library base retrieved from a global variable.
- `call_library_vector_414_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_function_4`: Executes a function via a pointer found by adding an offset to a library base stored in a global variable.
- `call_library_vector_198_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_function_5`: Invokes a library routine by accessing a pointer through an offset from a base address held in a global variable.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_6`: Calls a routine using a pointer derived from a library base address stored in a global variable.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_function_7`: Uses a library base from a global variable to call a function at a specific offset.
- `call_library_vector_378_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_288_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_96_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_198_9`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_282_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_384_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### Iron Lord (1989)(UBI Soft)[cr FLT](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Iron Lord (1989)(UBI Soft)[cr FLT](Disk 1 of 2).adf:ILwxzxw (raw code file, load address unknown)

- `configure_dma_hardware`: A large routine that accesses the ADKCON and DMACON hardware registers.

### Iron Lord (1989)(UBI Soft)[cr FLT](Disk 2 of 2).adf:ILwxzxw (raw code file, load address unknown)

- `configure_dma_hardware_2`: A large routine that accesses the ADKCON and DMACON hardware registers.
