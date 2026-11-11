# Game 511 (Iron Lord): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00511). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802400 top-level bytes in 113 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 28 of 28 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 1263 of 1263 named (0 author symbols), 1263 of 1263 described (136 by rule, 1058 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1042 of 1042 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 70913 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 23867 of 24530 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1287844 |
| code | 84542 |
| data | 367862 |
| padding | 25800 |
| unused | 36352 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 117 | 2 |
| binddrivers | 11 | 652 | 35 |
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
| Iron Lord (1989)(UBI Soft)[cr](Disk 1 of 2).adf:ilord | 3 | 6725 | 442 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 10 | 1375 | 109 |
| Iron Lord (1989)(UBI Soft)[cr](Disk 1 of 2).adf:samplesdatas | 1 | 2 | 1 |
| A590HD | 2 | 1853 | 100 |
| Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:fgaulbaire.1989 | 1 | 2028 | 51 |
| Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:wargame | 2 | 2752 | 78 |
| 9 | 1 | 2 | 1 |
| diskfont.library | 15 | 1040 | 61 |
| Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:strt2 | 3 | 2165 | 126 |
| Iron Lord (1989)(UBI Soft)[cr](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| Iron Lord (1989)(UBI Soft)[cr](Disk 1 of 2).adf:ILwxzxw (raw code file, load address unknown) | 1 | 867 | 1 |
| Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:scenario.sce (raw code file, load address unknown) | 1 | 0 | 0 |
| Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:ILwxzxw (raw code file, load address unknown) | 1 | 867 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- iron lord (1989)(ubi soft)[cr](disk 1 of 2)/Iron Lord (1989)(UBI Soft)[cr](Disk 1 of 2).adf:ILwxzxw: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- iron lord (1989)(ubi soft)[cr](disk 2 of 2)/Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:scenario.sce: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- iron lord (1989)(ubi soft)[cr](disk 2 of 2)/Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:ILwxzxw: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- iron lord (1989)(ubi soft)[cr](disk 2 of 2)/Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 1 never-loaded region(s), 38145 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `manage_system_resources` (fgaulbaire.1989): Performs file operations and manages system state by updating several global variables and interacting with hardware registers. Kind: disk hardware/track check. Pass path: the branch following the call to an address (the routine that performs the hardware/track validation).
- `call_function_and_update_globals` (fgaulbaire.1989): Calls a function via a pointer and then updates four global memory locations using a constant value and bitwise OR operations. Kind: disk hardware/track check. Pass path: the instructions immediately following the call (setting globals/pointers).
- `configure_dma_hardware` (ILwxzxw (raw code file, load address unknown)): A large routine that accesses the ADKCON and DMACON hardware registers. Kind: disk hardware/controller configuration. Pass path: not determined.
- `configure_dma_transfer` (ILwxzxw (raw code file, load address unknown)): This routine modifies the DMA controller and address configuration registers. Kind: disk hardware/DMA configuration check. Pass path: The branch following the routine that continues normal loading/execution.

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies sixteen words from a source address to a destination address using a loop.

### binddrivers

- `initialize_fast_memory_and_parse_args`: Attempts to locate a fast memory block and retrieves the command line arguments from the operating system.
- `bind_expansion_drivers`: Opens expansion.library and icon.library to obtain a list of expansion devices.
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
- `load_stack_pointer`: Copies a longword from the stack into a register.
- `compute_checksum`: Initializes a 32-bit accumulator to all ones and decrements it by one for each byte in a null-terminated string.
- `find_character_in_string`: Searches for the first occurrence of a specific character within a null-terminated string and returns a pointer to that character or null if not found.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `verify_checksum_and_offset`: Computes a checksum of a string and compares it against a value, returning a modified sum if the comparison succeeds.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.

### assign

- `update_linked_list_and_copy_data`: Traverses a linked list to find a maximum value and then updates a structure with this value and a sequence of incrementing words.
- `copy_memory_block_3`: Copies a block of sixteen bytes from a source address to a destination address and then calls another routine.

### EndCLI

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field, then expands a data structure by writing a header and a sequence of incrementing values.
- `copy_data_block_2`: Copies sixteen bytes from a source pointer to a destination pointer and then calls another routine.

### execute

- `update_linked_list_and_copy_data_3`: Traverses a linked list to find the maximum value in a specific field, then updates a buffer header and performs a series of memory copies between a source and a destination based on the discovered maximum value.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then executes a function pointer.

### Copy

- `update_linked_list_and_copy_data_4`: Traverses a linked list to find the maximum value in a specific field, then expands a buffer by writing a header and a sequence of incrementing values.
- `copy_memory_block_4`: Copies sixteen bytes from a memory location to a destination buffer.

### makedir

- `update_linked_list_and_copy_data_5`: Traverses a linked list to find the maximum value in a specific field and updates a header structure.
- `copy_memory_block_5`: Transfers a fixed number of words from a source address to a destination.

### c3ddata.bin

- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d3, then merges 130 into d0.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer.

### FastFileSystem

- `dispatch_callback_table_2`: Iterates through a table of function pointers and executes them based on comparisons between structure members and values retrieved from a global variable.
- `update_linked_list`: Checks if a pointer is non-zero, then updates a list entry with a value from a table and calls another routine.
- `calculate_volume_geometry_and_mount`: Calculates block counts, sector offsets, and buffer requirements based on filesystem metadata.
- `traverse_and_execute_callbacks`: Traverses multiple linked lists and executes function pointers stored at various offsets within a structure.
- `pop_linked_list`: Iterates through a loop to pop elements from a linked list by updating a pointer in a table until a null pointer is encountered.
- `validate_and_mount_filesystem`: Verifies a filesystem header for specific magic numbers and structural integrity, then traverses a linked list of blocks to mount the volume.
- `initialize_buffer_structure`: Calculates a value based on a table entry and an input, then populates several fields in a structure pointed to by a global variable.
- `calculate_offset_size`: Retrieves a value from a nested pointer structure and returns it multiplied by four plus four.
- `copy_buffer_and_update_linked_list`: Copies a block of memory from one location to another using a loop, then updates a pointer structure by storing the new address and a calculated size value.
- `update_table_entry`: Searches a table for a specific value and replaces it with a value derived from a pointer multiplication.
- `copy_and_execute`: Copies a longword from a table and then executes three consecutive function pointers.
- `calculate_bit_density`: Calculates a cumulative value by repeatedly calling another routine in nested loops based on a value extracted from a structure, then counts the number of zero bits in a specific bitfield of a word.
- `count_leading_zeros`: Calculates a value by counting how many times a bit can be shifted right before encountering a non-zero bit.
- `scan_bitmask_for_free_blocks`: Iterates through a bitmask stored in memory to locate the first unset bit, representing an available resource or block.
- `find_and_clear_bit_2`: Searches through a memory range for a set bit, clears it, and updates a global structure with the bit position and metadata.
- `find_and_clear_bit`: Searches through a memory range for a set bit, clears it, and updates a global structure with the bit position and related metadata.
- `clear_bit_and_store_state`: Clears a specific bit in a value and stores several parameters into a global structure.
- `search_bitmask`: Scans a bitmask for a set bit and updates a global structure with the resulting index and state information.
- `find_first_set_bit`: Iterates through a memory range to find the first occurrence of a set bit and updates a structure with the result.
- `search_bitmask_and_update_state`: Scans a bitmask for the first set bit and updates a global structure with the resulting index and state information.
- `search_bit_range`: Scans a memory area for the first set bit in a bitmask and updates a structure with the index and state when found.
- `return_all_ones_4`: The routine returns a value consisting entirely of set bits.
- `calculate_fragment_details`: Computes a value based on the difference between two numbers and returns a concatenated result containing quotient, remainder, and bitwise mask information.
- `align_value_down`: Subtracts the remainder of a value divided by a table entry from the original value.
- `set_bit_and_decrement`: Sets a bit in a buffer and decrements a counter in a table.
- `decrement_and_store`: Stores a value at a calculated offset and decrements a counter in a table.
- `store_register_to_offset_pointer`: Takes a value from a data register and stores it into a memory location calculated by adding an offset to a value returned by another routine.
- `flush_and_sync_buffer_cache`: Clears a dirty flag and synchronizes a data buffer by copying metadata and performing bulk block transfers.
- `update_and_sync_block_metadata`: Clears and copies a set of longwords into a destination buffer, then performs a bulk transfer of data blocks based on a calculated count.
- `calculate_table_index`: Performs a mathematical calculation on an input value to derive an index and returns a longword from a table located via a pointer in a global structure.
- `rotate_list_elements`: Manipulates pointers within a linked list structure to reorder elements and executes function pointers stored within those elements.
- `update_linked_list_head`: Stores a value from a global structure into a destination, sets a pointer to the stack, and updates a global structure with the new destination.
- `rotate_list_head`: Moves a pointer from a structure to a new location and sets a field in a linked list to a stack address.
- `replace_list_head`: Replaces a pointer in a structure with a new address and clears a field in the previous node.
- `insert_into_linked_list`: Traverses a linked list to find a specific node and performs a pointer swap to insert a new element into the list.
- `initialize_node_pointers`: Sets multiple fields in a structure to a new pointer value and returns a constant.
- `check_status_and_initialize_structure`: Calls another routine and, based on the return value, either clears specific fields in a structure or sets a field to a constant value.
- `populate_linked_list`: Fills a series of memory locations at regular intervals with values from a structure and links them together.
- `execute_callback_loop`: Checks a condition in a global structure and either executes a single function pointer or enters a loop calling two different function pointers until one returns a non-zero value.
- `link_list_nodes`: Updates a structure with a constant value and links it into a list by manipulating pointers based on the state of a global field.
- `update_linked_list_node_2`: Copies a pointer from a global structure to a destination and manipulates the next pointer of a node within a linked list.
- `update_linked_list_2`: Manipulates pointers within a linked list structure to insert or update nodes.
- `update_linked_list_node`: Updates a linked list by moving a pointer from one structure to another and setting a field to a stack address.
- `recursively_validate_linked_list`: Performs a recursive depth-first traversal of a linked data structure to verify pointer integrity and node constraints.
- `validate_list_integrity_3`: This routine performs multiple checks on a data structure and its associated linked lists, involving recursive calls and pointer manipulations based on specific integer constraints.
- `validate_list_integrity_2`: This routine iterates through a linked list to check specific integer constraints and pointer states, performing various list manipulations and node updates based on the results.
- `validate_list_integrity`: Traverses linked lists to verify node integrity and checks specific integer constraints within the list elements.
- `validate_linked_list`: Traverses a linked list to verify node integrity and checks specific integer constraints within the list elements.
- `validate_filesystem_node_integrity`: Traverses a linked list of nodes to verify that specific member offsets contain non-zero values and satisfy integer comparison constraints.
- `call_tiny_move`: Calls another routine and returns.
- `recursively_validate_linked_list_nodes`: Traverses a linked list to verify that node members satisfy specific non-zero and integer comparison constraints.
- `traverse_and_validate_linked_list`: Iterates through a linked list to check for specific conditions in node members, such as zero values or integer comparisons, and performs a pointer redirection if a specific offset contains a non-null value.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a7)+ to d2, then returns.
- `set_conditional_word_2`: Checks a word in a structure and stores either one or three into a different offset within that same structure before calling another routine.
- `set_conditional_word`: Checks a word in a structure and stores either one or three into a different offset within that same structure before calling another routine twice.
- `set_word_and_call`: Stores a word value into a structure and calls another routine twice.
- `relink_and_initialize_node`: Traverses a linked list to find a node with a specific status, then updates that node's internal pointers, status fields, and data values using values from registers and a global table.
- `update_list_nodes`: Traverses a list to find specific nodes and updates their internal pointers and status fields with new values.
- `execute_recursive_call`: The routine performs a direct call to itself and then returns.
- `manage_list_nodes`: Traverses a list and performs complex updates to node members, including setting specific status values and pointers when certain conditions are met.
- `compare_and_recurse`: Compares a value in a register against a value at an offset within a structure and performs recursive calls based on the result.
- `execute_recursive_call_2`: This routine performs a direct recursive call to itself before returning.
- `validate_linked_list_nodes`: Traverses a linked data structure to verify node constraints and pointer integrity, updating specific fields within the structure if certain conditions are met.
- `initialize_structure_fields_2`: Populates several fields of a structure, including a pointer to itself plus an offset, with specific values and data from other registers.
- `manage_linked_list_nodes_2`: Evaluates a return value to conditionally insert a new node into a linked list and then initializes a structure with various metadata and pointers.
- `initialize_and_clear_bitfield`: Fills a memory buffer with all ones and then clears a specific bit within a bitstream based on a calculated offset and bit index.
- `clear_bitfield_flags`: Iterates through a buffer to find set bits and clears them in a corresponding bitmask table.
- `clear_bitfield_and_validate_metadata`: Iterates through a bitstream to locate and clear specific bits within a buffer, then validates file metadata by checking directory entry ranges and updating bitmask tables.
- `update_bitmask_and_validate_file`: Clears specific bits within a bitmask table and validates file metadata by checking directory entries against range constraints.
- `process_bitmask_and_validate_directory`: Validates directory header status and iterates through a list of indices to clear corresponding bits within a bitmask table.
- `scan_and_clear_bitmask`: Iterates through a bitmask buffer to identify set bits and clears them one by one.
- `traverse_and_clear_bitmask`: Iterates through a list of indices to check if specific bits are set within a bitmask table.
- `clear_bitmask_and_iterate_buffer`: Checks and clears specific bits within a bitmask stored in a table.
- `clear_bitmask_and_iterate_buffer_2`: Iterates through a bitmask table to identify set bits and clears them one by one.
- `set_bit_in_range`: Checks if a bit index is within a specified range and sets the bit in a buffer if it is.
- `set_bit_in_buffer`: Sets a specific bit within a byte located at an offset calculated from a pointer and a bit index.
- `return_error_constant`: Returns a longword containing a specific constant value.
- `validate_directory_status_2`: Checks specific fields within a structure to return integer constants based on the state of the directory header.
- `validate_directory_status`: Compares values within a structure to return specific integer constants based on the state of certain fields.
- `return_small_integer`: Returns the integer value two.
- `compare_structure_fields`: Compares two longwords at specific offsets within a structure and returns an error constant if they are equal.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `copy_and_loop_string`: Copies a null-terminated string to a new location and then executes another routine in a loop until it returns zero.
- `increment_string_values`: Iterates through a list of integers, performing modulo-like arithmetic on them and updating a string by incrementing character values.
- `process_volume_chain_2`: Iterates through a chain of structures, performing arithmetic on integer values within the nodes and updating status flags.
- `process_volume_chain`: Iterates through a chain of structures, updating specific fields and performing arithmetic on values within the nodes.
- `process_file_chain`: Iterates through a linked list of structures, updating values and performing arithmetic on specific offsets based on conditional checks.
- `update_volume_data`: Populates a structure with calculated values and calls a function pointer in a loop until a termination condition is met.
- `process_volume_data_2`: Iterates through a range of values, updating a structure and calling another routine until a termination condition is met.
- `process_volume_data`: Copies a null-terminated string and a subsequent block of memory into a local buffer before calling another routine.
- `copy_memory_and_check`: Copies a block of memory and then conditionally calls another routine based on values in a structure.
- `update_pointer_link`: Stores a value from a structure into a memory location and updates a pointer within that structure.
- `set_conditional_word_3`: Sets a word in a structure to either one or three based on the value of a short at a specific offset.
- `call_relink_and_initialize_node`: This routine acts as a wrapper that calls a function to traverse a linked list, update node status fields, and modify internal pointers and data values.
- `call_relink_routine_unreached`: This routine is unreached in the code flow and calls another routine.
- `call_relink_and_initialize_node_2`: This routine calls another routine and then returns.
- `execute_error_callback`: Repeats a routine until it returns zero, then calls a function pointer stored at an offset within a structure if a register contains a negative value.
- `execute_callback_on_error`: Executes a routine in a loop and then calls a function pointer from a structure if a register value is negative.
- `manage_library_resources_2`: Initializes a data structure and conditionally performs a sequence of operations involving memory management, function pointer calls, and library-related tasks.
- `initialize_data_structure`: Writes a series of bytes and longwords to a memory location to initialize a structure.
- `call_indirect_function_29`: Calls a function pointer stored at an offset from a register.
- `copy_block_of_longwords`: Copies thirty-two longwords from a source memory location to a destination memory location using a loop.
- `clear_memory_block`: Clears a block of memory by writing zero to thirty-two consecutive double-word entries.
- `infinite_loop_calls`: An infinite loop that repeatedly calls another routine.
- `sum_table_elements_2`: Iterates through a table of four-word entries and calculates the sum of all values contained within them.
- `update_list_entry`: Stores two values into a structure and updates a linked list by setting a pointer to a value from a global table.
- `relink_list_node`: Replaces a pointer in a structure and updates a linked list by moving a pointer from one node to another.
- `return_without_action_3`: The routine consists of six instructions and its decompiled body is a bare return with no operations, so it does nothing when called.
- `compare_byte_sequences`: Compares two memory areas byte by byte in a loop, returning a failure code if a mismatch is found or a zero terminator is reached.
- `check_status_flags`: Writes a value to an offset of a pointer and returns a specific constant if two conditions regarding a structure are met.
- `calculate_coordinates`: Performs arithmetic on a value from a structure to derive three derived values, then stores them in two different locations.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `initialize_structure_fields`: Sets several fields in two different structures and then calls an indirect function pointer.
- `insert_into_linked_list_2`: Calculates an address from a table and an index, then inserts a pointer into a linked list at that location.
- `replace_list_node`: Traverses a linked list to find a specific node and replaces its pointer with a value from another location.
- `copy_long_word`: Copies a long word from an offset of a pointer into another memory location.
- `traverse_linked_list_3`: Follows a chain of pointers starting from a table-indexed entry until a null pointer or a specific value is encountered.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies a0 to d0, then returns.
- `link_structure_elements`: Sets a pointer to zero and then links two structures together by updating pointers based on whether a specific field is zero.
- `write_pointer_twice`: Writes a value to a pointer and then writes that same value to the same location again.
- `insert_into_list_structure`: Updates pointers within two structures to link them together or insert a value into a list.
- `insert_into_list`: Copies a value to a new location and updates the next pointer of a structure to include the new element.
- `store_word_at_offset`: Stores a longword from a pointer into a location offset by four bytes from the target address.

### Ram-Handler

- `execute_indirect_jump`: An entry routine that performs an indirect call through a pointer retrieved from a memory location.

### ramdrive.device

- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `configure_ramdrive_device_parameters`: Iterates through a list of device structures to validate metadata and copy configuration strings into a global buffer.
- `populate_ramdrive_metadata`: Copies a sequence of bytes from a source buffer into a global variable and populates a structure with a list of values.
- `initialize_ramdrive_buffer_and_structure`: Calculates a memory size based on several values, performs memory copies to populate a structure, and updates a linked list of device objects.
- `initialize_ramdrive_device`: Initializes a device structure by storing pointers to internal buffers and control fields, and links it into a global list.
- `execute_device_initialization`: Calls a function pointer located at a fixed offset from a base address twice.
- `handle_interrupt_chain`: Traverses a linked list to find a matching bitmask, updates a counter, and calls a function pointer while modifying the interrupt enable register.
- `process_interrupt_chain`: Traverses a linked list of structures to find one matching a bitmask, then updates a counter and potentially modifies a global variable and the interrupt enable bit.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_counter`: Decrements a byte at a specific offset in a structure and sets a global variable to a constant if the byte reaches zero.
- `store_word_to_pointer`: Stores a 16-bit value from a register into a memory location pointed to by a register plus an offset.
- `process_device_message`: Executes a function pointer from a global structure, looks up a value in a table based on a field in a message structure, stores the result back in the message, and conditionally calls another function pointer before a final cleanup routine.
- `return_negative_one_5`: Returns the constant value negative one.
- `return_negative_one`: Returns a constant value of negative one.
- `handle_device_request`: Decrements a counter, modifies a linked list, traverses a list of pointers, and replaces a null terminator in a string with a colon.
- `replace_null_with_colon`: Updates a linked list, calls a function via a pointer, and replaces a null terminator in a string with a colon before calling another function.
- `terminate_string_with_colon`: Iterates through a string until a null terminator is found, replaces it with a colon, and executes a function pointer.

### Iron Lord (1989)(UBI Soft)[cr](Disk 1 of 2).adf:ilord

- `clear_system_state`: Fills a large memory block with zeros and sets several global variables using values returned from system library calls.
- `initialize_game_state_tables`: Initializes a large block of memory by setting numerous table entries to a global variable and clearing specific fields.
- `process_callback_and_hardware_state`: Iterates through multiple tables of function pointers to execute specific callbacks with varying arguments.
- `execute_sample_callback`: Checks a value in a table and, if zero, executes a callback function and stores the result in a global variable.
- `dispatch_callback_table`: Iterates through a table of forty pointers and calls a function for each non-zero entry using associated data from another table.
- `store_word_pair`: Stores two consecutive 16-bit values from a register into a memory location offset from a pointer.
- `populate_data_structure_2`: Copies multiple values from various tables into a structure based on an index and calls another routine.
- `populate_data_structure`: Copies several values from tables into a structure based on an index and then calls another routine.
- `call_library_function`: Calls a library function via a pointer and executes another routine if the result is zero.
- `execute_callback_sequence_2`: Checks a value in a table and executes a callback function if the value is zero, storing the result in a global variable.
- `execute_callback_sequence_3`: Calls a dispatch routine and then calls another routine four times with different word values passed on the stack.
- `execute_recursive_calls`: Recursively calls itself four times before returning.
- `execute_callback_chain_2`: Calls two different callback dispatch routines in succession.
- `process_callback_chain`: Iterates through a series of function pointers and executes them sequentially.
- `clear_audio_registers`: Clears several audio hardware registers and DMA control registers by writing zero to them.
- `set_register_to_one`: Calls another routine and then stores the value one into a memory location offset from a pointer.
- `clear_audio_hardware`: Clears audio hardware and DMA control registers by writing zero to them and sets a byte in a memory location to one.
- `reset_audio_and_set_flag`: Clears audio hardware registers and then sets a byte to one at a location calculated from a register and a constant offset.
- `manage_audio_buffer`: Performs a series of callback executions to allocate or update a buffer and stores the resulting pointer in a global variable.
- `return_negative_one_long`: Returns a 32-bit constant value of negative one.
- `clear_memory_and_copy_data_2`: Clears a large block of memory by zeroing out words in a loop and copies a sequence of four words from a global variable to a specific memory buffer.
- `clear_memory_and_update_flags`: Clears a large block of memory using a loop and updates specific bitfields in memory via pointers.
- `initialize_memory_buffer_and_copy_data`: Clears a large memory range with zeros and performs a series of bitwise operations on specific memory locations.
- `process_complex_data_loop`: Performs conditional loops, modifies specific bit patterns in memory via offsets, and copies data from a global table to a destination.
- `update_memory_and_clear_bits`: Writes specific bit patterns to memory locations via a pointer and performs a loop to zero out a large block of memory.
- `clear_memory_and_copy_table`: Zeroes out a large block of memory starting from a specific offset and then copies a sequence of values from a global table to a destination buffer if a calculated absolute difference meets a threshold.
- `wait_for_bit_clear`: Writes values to a memory location and then loops until a specific bit in a memory location is cleared.
- `load_constant_to_d2`: Loads the sum of two word constants into register d2 and then returns.
- `check_bit_conditions`: Compares a value from a pointer against a local variable and checks if specific bits are set in a register before returning a boolean result.
- `return_all_ones`: returns a value consisting of all bits set to one.
- `find_interleaved_pattern`: Searches through a memory buffer for a specific magic constant and returns an interleaved bit pattern from subsequent words if found.
- `interleave_bits_masked`: Checks a magic constant at a pointer and returns an interleaved bit pattern from two subsequent words if the constant matches.
- `interleave_bits`: performs a bitwise operation on two words pointed to by a register to interleave their bits.
- `validate_interleaved_data_sequence`: Searches a memory buffer for a specific magic constant and then verifies that the preceding data follows a bit-interleaved pattern matching a counter.
- `verify_interleaved_sequence_2`: Iterates through a memory buffer to validate an interleaved bit sequence against a counter and a magic constant.
- `verify_interleaved_sequence`: iterates through a memory buffer, performing bit interleaving and comparing the results against a counter and a specific value.
- `tiny_swap_move_rts`: Small routine of 3 instructions that swaps the halves of d3, then copies d3 to d6, then returns.
- `initialize_hardware_parameters`: Writes several multi-byte values to a memory block and waits for a specific bit in a byte to be cleared.
- `wait_for_bit`: loops until a specific bit in a byte at a memory offset is cleared.
- `clear_bit_and_wait`: Clears the third bit of a byte and waits for a specific bit in another byte to be cleared.
- `set_bit_and_wait`: Sets the third bit of a byte and waits for a specific bit in another byte to be cleared.
- `modify_and_loop`: writes a value to a memory location and enters a loop that decrements a counter while checking a bit in a buffer.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 0 into d1, then returns.
- `wait_for_bit_clear_4`: Loops until a specific bit in a byte at a memory location is cleared.
- `modify_byte_and_wait`: Sets and clears specific bits in a byte and loops until a bit in another byte is cleared.
- `modify_bits_and_wait`: Clears the second and third bits and sets the first bit of a byte before waiting for a specific bit to be cleared.
- `decrement_loop`: decrements a register from a starting value down to negative one.
- `wait_for_bit_clear_2`: loops until a specific bit in a byte at a memory location is cleared.
- `tiny_moveq_moveq_lsl`: Small routine of 3 instructions that loads 3 into d0, then loads 14 into d1, then shifts d0 left by d1.
- `copy_data_block_3`: Performs a loop to zero out a memory range, calculates an absolute value, and conditionally copies a block of data from a global variable to a fixed address.
- `copy_memory_block_2`: sets two short values to a specific constant and then copies a block of data from a global variable to a fixed memory address.
- `update_indexed_data`: Copies a longword from an indexed location in a table to two different destinations and toggles a byte via an XOR operation.
- `execute_repeated_function_call`: Repeatedly calls a function pointer while incrementing a counter until a limit stored in a global variable is reached.
- `execute_repeated_calls`: calls a function pointer repeatedly while incrementing a value in memory until it reaches a limit stored in a global variable.
- `call_function_repeatedly`: Calls a function a specific number of times based on a value stored in a table.
- `iterate_and_call_function`: Iterates through a loop based on a value stored at a memory offset from a pointer.
- `increment_and_call_loop`: Calls a function and increments a counter in a table until a limit is reached.
- `execute_repeated_function_calls`: Repeats a call to a function pointer a number of times determined by a value stored at a specific offset from a base address.
- `execute_repeated_function_calls_2`: Iterates through a loop based on a value stored in a table and calls a function pointer a specific number of times.
- `loop_with_counter_increment`: Executes a function and increments a value in a table within a loop.
- `update_table_values`: Copies two words from one location to another, waits for a bit flag to be set, and then updates the original location with values from a pointer-based table.
- `calculate_derived_values`: Retrieves multiple values from a table via a function, stores them in several locations, and performs arithmetic to store a calculated result.
- `write_to_global_offsets`: Calls a function multiple times using a global variable and various fixed offsets.
- `call_function_with_table_data`: Iterates four times, calling a function with a pointer retrieved from a table.
- `execute_callback_sequence`: Iterates through a series of memory addresses, passing each one as an argument to a single function pointer retrieved from a base address.
- `process_list_of_addresses`: Calls a function repeatedly using a sequence of addresses from a table.
- `allocate_and_initialize_string_buffer_2`: Allocates a memory block based on a calculated size and initializes it with a specific four-byte signature.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `sync_directory_and_execute_callback`: Retrieves the current directory and executes a function pointer stored within a nested structure.
- `retrieve_directory_and_execute_indirect`: Retrieves the current directory via a DOS function and executes a function pointer retrieved from a nested offset within a structure pointed to by a global variable.
- `copy_data_block`: Copies a large sequence of values from a source pointer into several destination locations and a global variable.
- `setup_blitter_params`: Configures multiple Blitter registers including source, destination, and size parameters, and enables the DMA controller.
- `configure_blitter_and_call_indirect`: Configures Blitter registers and DMA parameters, calls a library function, and triggers a function call through a pointer in a table.
- `perform_blitter_transfer`: Sets Blitter width, height, and dimension registers before initiating a transfer and calls a library function and an indirect function pointer.
- `configure_blitter_transfer`: Sets specific Blitter registers for width, height, and dimensions before initiating a transfer.
- `loop_blitter_parameters`: Executes a loop that calls a function with parameters from a table while checking a bit in a global variable, followed by a final function call.
- `loop_blitter_operations`: Executes a loop that calls another routine with various parameters from a table, incrementing and decrementing specific values until a condition is met.
- `parse_command_input`: Iterates through a character buffer to convert alphanumeric characters and specific punctuation marks into a normalized integer index.
- `map_punctuation_to_tokens`: Iterates through a text buffer and replaces specific punctuation marks and special characters with unique numeric identifiers.
- `tokenize_text_punctuation`: Iterates through a text buffer to identify punctuation marks and special control characters, replacing them with specific numeric identifiers and associated metadata.
- `map_text_punctuation_to_tokens`: Iterates through a text buffer to identify punctuation marks and special control characters, replacing them with specific numeric identifiers.
- `encode_text_metadata_2`: Iterates through a text buffer to identify punctuation marks and special control characters, replacing them with specific numeric identifiers and a metadata flag.
- `convert_punctuation_to_metadata_2`: Iterates through a text buffer to identify punctuation marks and special control characters, replacing them with specific numeric identifiers and associated metadata values.
- `map_punctuation_to_metadata_2`: Iterates through a text buffer to identify punctuation marks and special control characters, replacing them with specific numeric identifiers and a constant metadata tag.
- `convert_punctuation_to_metadata`: Iterates through a text buffer to identify punctuation marks and special control characters, replacing them with specific numeric identifiers and a common metadata tag.
- `encode_text_metadata`: Iterates through a text buffer to identify punctuation marks and special control characters, replacing them with specific numeric identifiers and associated metadata values.
- `map_punctuation_to_metadata`: Iterates through a text buffer to identify punctuation marks and special control characters, replacing them with specific numeric identifiers and status flags.
- `parse_punctuation_and_text`: Identifies punctuation marks and control characters within a buffer to assign them specific numeric values and status flags.
- `map_control_characters`: Identifies specific punctuation and control characters within a buffer to assign them corresponding numeric values and status flags.
- `parse_command_control_characters_5`: Identifies specific control characters within a buffer to set a status byte and a length value.
- `process_command_control_characters`: Identifies specific control characters within a buffer to set a status byte and a length value.
- `parse_command_control_characters_4`: Identifies specific control characters within a buffer to set a status byte and a length value.
- `parse_command_control_characters_3`: Identifies specific control characters within a buffer to set a status byte and a length value.
- `parse_command_control_characters_2`: Identifies specific control characters within a buffer to set a status byte and a length value.
- `parse_command_control_characters`: Identifies specific control characters within a buffer to set a status byte and a length value.
- `parse_input_command_sequence`: Identifies control characters to set a status byte and length value, then recursively processes a buffer of characters.
- `process_input_buffer`: Checks for specific control characters in a buffer to set a status byte and a length value, then recursively calls itself using a calculated offset.
- `parse_input_command`: Iterates through a character buffer to skip whitespace and null characters while converting lowercase alphabetic characters into zero-based indices.
- `jump_to_normalize_and_skip_text`: Jump stub: passes control straight on to normalize_and_skip_text.
- `normalize_and_skip_text`: Iterates through a buffer to skip whitespace and null characters while normalizing alphabetic characters to a zero-based index by subtracting their base ASCII values.
- `update_joystick_state`: Reads the joystick port and updates several memory locations based on specific bit patterns.
- `update_input_state_2`: Tests specific bits within a value and updates several memory locations using offsets from a base pointer.
- `check_input_status`: Checks a bit in a register and updates a global variable or a memory location based on the result.
- `poll_condition_loop`: Repeatedly calls another routine as long as a specific bit in a memory location remains set.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `process_data_loop`: Clears a block of memory and then iterates through a table to call two different subroutines.
- `return_constant_one`: Empty routine that simply returns the value one.
- `process_input_event_loop_2`: Polls an input source through a function pointer and processes the resulting values in a loop.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `poll_input_and_process_events_3`: Repeatedly polls an input source through a function pointer and processes the resulting values in a loop.
- `return_zero`: An empty routine that returns zero.
- `process_input_event_loop`: Repeatedly polls an input source through a function pointer and processes the resulting values in a loop.
- `poll_input_and_process_events_2`: Repeatedly polls an input source through a function pointer and processes the resulting values in a loop.
- `poll_input_and_process_events`: Repeatedly polls an input source through a function pointer, using a loop to consume multiple consecutive input values based on a specific mode.
- `poll_input_and_update_state`: Repeatedly polls an input source and checks a status flag to determine whether to exit, loop, or execute specific data updates.
- `process_input_loop`: Executes a loop that polls an input state and repeatedly calls a function to retrieve values until a zero result is returned.
- `synchronize_and_filter_table_buffer`: Synchronizes a data buffer from a source table based on a conditional flag and then iterates through a secondary table to filter and copy specific entries into a destination buffer.
- `populate_filtered_table_buffer`: Iterates through a table to find entries meeting specific criteria and copies them into a destination buffer.
- `filter_and_copy_table_entries_2`: Iterates through a table to find entries meeting specific criteria and copies them into a destination buffer.
- `select_and_copy_table_entries`: Iterates through a table to compare multiple values against a threshold, copying selected entries into a destination buffer.
- `filter_and_copy_table_entries`: Iterates through a table to compare values against a threshold and copies selected entries into a destination buffer.
- `sort_and_copy_table_entries`: Iterates through a table to select and copy specific data entries based on comparison logic.
- `iterate_and_copy_data`: Iterates through a table using an index, calls two other routines, and performs a block copy of data from one memory location to another when specific counters reach a limit.
- `copy_table_entries`: Copies forty-eight entries from one memory table to another if a flag is not already set.
- `wait_for_flag`: Updates two memory locations with provided values and then waits for a specific bit in a global variable to be set.
- `execute_conditional_callbacks`: Compares two values and executes a series of different routines based on the contents of a specific memory location.
- `tokenize_command_string`: Iteratively parses a space-delimited string into individual tokens by skipping whitespace and copying characters into a local buffer.
- `parse_command_arguments_2`: Iteratively extracts space-delimited string tokens from a buffer and copies them into a local structure.
- `tokenize_and_execute_command`: Iteratively parses a space-delimited string into individual tokens and copies them into a local buffer.
- `parse_command_arguments`: Iteratively extracts space-delimited tokens from a string and processes them by calling a lookup routine.
- `tokenize_and_process_command`: Parses a space-delimited string into individual tokens and copies them into a local buffer.
- `parse_command_string`: Iterates through a sequence of space-delimited tokens extracted from a string to perform arithmetic calculations and update memory locations.
- `process_command_tokens`: Iterates through a sequence of tokens extracted from a string, performing arithmetic calculations and updating various memory locations based on the content of each token.
- `set_short_value`: Stores the value one into a two-byte location relative to a pointer.
- `parse_command_token`: Iterates through a string to extract a single word by skipping leading spaces and copying characters until a space or end-of-string is encountered.
- `parse_command_string_2`: Iterates through a space-delimited string to extract individual words into a local buffer.
- `process_tokenized_string_loop`: Iterates through a string to extract words into a local buffer, performing arithmetic calculations and updating values via calls to other routines for each word found.
- `process_tokenized_string`: Iterates through a string to extract words by skipping spaces and copying characters into a local buffer.
- `jump_to_return_from_subroutine_2`: Jump stub: passes control straight on to return_from_subroutine_2.
- `parse_and_copy_string`: Calculates a value based on string length and arithmetic offsets, then iterates through a string to skip leading spaces and copy characters into a destination buffer until a space or a length limit is reached.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `process_conditional_data`: Calls a routine with parameters from a table, stores the result in a local variable, and recursively calls itself if that result is non-zero.
- `update_sprite_attribute_table_via_index`: Selects a value from a lookup table based on an index and passes it, along with other parameters, to a routine that updates sprite attributes.
- `execute_indexed_callback`: Stores two words into a memory location before calling a function via a pointer with parameters fetched from a table.
- `invoke_indexed_callback`: Calls a function via a pointer using parameters retrieved from a table and stores the return value in a global variable.
- `update_sprite_attribute_table`: Selects a pair of values from a lookup table based on an index and writes them to a memory location.
- `process_indexed_data`: Calls a routine using an index to fetch a parameter from a table, stores the result, and performs a conditional recursive call.
- `process_data_and_call_handler`: Calls a routine with parameters from a table, performs a conditional recursive call, and finally executes a function pointer from a table.
- `process_global_data`: Calls a routine using a global variable as a parameter, stores the result, and executes a function pointer from a table if the result is non-zero.
- `execute_conditional_calls`: checks a byte and calls two different subroutines if the value is non-zero.
- `interleave_buffer_data_2`: Distributes bytes from a single source buffer into seven separate destination buffers in a round-robin sequence, or performs the inverse operation based on a mode flag.
- `interleave_buffer_data`: Distributes bytes from a single source buffer into seven separate destination buffers in a round-robin sequence, or performs the inverse operation to reconstruct a single source buffer from seven interleaved destination buffers.
- `interleave_source_bytes`: Reads bytes from a source buffer and distributes them into seven different destination buffers in a round-robin fashion.
- `interleave_source_bytes_2`: Iterates through a loop to copy one byte from each of seven different source memory locations into a destination buffer, incrementing the destination index for each byte.
- `write_buffer_to_file`: Calls a routine to write a specific amount of data and then executes two other routines using pointers from a local structure.
- `fetch_byte_via_offset`: Calculates an address by adding a high-word value from a parameter to a base address and returns the byte stored at that location.
- `dereference_offset_pointer`: Calculates an address by adding a high-word offset to a base address and returns the word stored at that location.
- `fetch_long_from_table`: returns a four-byte longword from a table using a base address and an offset provided in a register.
- `resolve_nested_table_indices`: Selects values from a nested table structure by performing conditional pointer dereferencing and stores the resulting indices into a destination buffer.
- `populate_buffer_from_nested_table`: Iterates through a nested table structure to select specific values based on comparisons and copies them into a destination buffer.
- `filter_and_buffer_table_elements`: Iterates through a nested table structure to select specific values based on comparisons and copies them into a destination buffer.
- `select_and_buffer_table_elements`: Iterates through a nested table structure to select specific values based on comparisons and copies them into a destination buffer.
- `populate_buffer_from_table_3`: Iterates through a table of values to select and copy specific elements into a destination buffer.
- `copy_list_elements`: Iterates through a table of values and copies them into a destination buffer using a counter.
- `iterate_and_copy_elements`: Iterates through a list of values, calling a routine to copy data into a destination buffer while managing a counter and a secondary loop for resetting values.
- `loop_function_calls`: executes a specific subroutine sixteen times while incrementing a counter.
- `update_combined_value`: Combines two short values into a single word stored in a global variable.
- `update_global_state`: Updates a global word value based on a local status and stores bytes into two different tables using an offset from a local variable.
- `update_data_structure`: updates a global word value and stores bytes into a table at an offset determined by a local value.
- `execute_conditional_function`: Calls another routine and then executes a specific function if a global word value equals one.
- `parse_map_asset_header_3`: Synchronizes a data stream to a specific marker and parses structured asset metadata.
- `parse_map_asset_header_2`: Synchronizes a data stream to a specific marker and parses structured asset metadata.
- `parse_map_asset_header`: Synchronizes a data stream to a specific marker and parses structured asset metadata.
- `parse_map_asset_data`: Identifies specific marker bytes within a data stream to extract structured asset information.
- `parse_asset_header_stream`: Reads a stream of data to identify specific marker bytes and extract structured asset information.
- `parse_compressed_asset_stream`: Reads a stream of data using a custom decompression routine to extract multi-layered pixel or color information.
- `expand_multi_layer_rle_stream`: Expands a multi-layered run-length encoded data stream into a destination buffer.
- `decompress_rle_graphics_stream`: Expands a run-length encoded data stream into a destination buffer by reading control bytes to determine pixel repetition counts.
- `decompress_rle_stream`: Expands a multi-layered run-length encoded data stream into a destination buffer.
- `decompress_nested_rle_stream`: Expands a multi-layered run-length encoded data stream into a destination buffer.
- `expand_compressed_data_stream`: Reads a stream of bytes from a source pointer and expands them into a destination buffer based on length values found within the stream.
- `expand_rle_data_buffer`: Iterates through a nested structure of data to expand run-length encoded streams into a destination buffer.
- `populate_nested_data_buffer`: This routine performs a nested loop to fill a calculated memory buffer with values obtained from a function call.
- `decompress_run_length_encoded_stream`: Decodes a data stream using a run-length encoding scheme where bytes determine either a sequence of repeated values or a series of literal values.
- `populate_buffer_from_table_2`: Iterates through a nested loop structure to read bytes from a source via a function call and write them into a destination buffer.
- `expand_compressed_data_stream_2`: This routine decodes a variable-length encoded data stream by reading bytes through a function call and writing them into a destination buffer.
- `populate_buffer_from_table`: This routine iterates through a nested loop structure to read bytes from a source via a function call and write them into a destination buffer.
- `decode_run_length_encoded_stream`: Iterates through a data stream to expand run-length encoded values into a destination buffer.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d1.
- `manage_dynamic_memory_allocation`: Allocates a block of memory and stores the resulting pointer in a global variable, or releases the allocated memory and clears the global variable if a specific condition is met.
- `reset_memory_pointer`: calls two subroutines and then clears a value in a global variable.
- `execute_recursive_call_4`: Performs a single recursive call to itself.
- `process_signed_data_structures`: This routine calls two subroutines to check the sign of values within structures, negate them if necessary, and populate arrays or set flags.
- `process_signed_value_and_copy_table`: Checks the sign of a value within a structure, negates it if negative, and sets a flag.
- `process_signed_data_and_populate_array`: Checks the sign of a value within a structure, potentially negating it and setting a flag.
- `parse_formatted_string_6`: Iterates through a format string to extract and convert numeric, string, or hexadecimal values into a destination array of shorts.
- `parse_printf_format_string_4`: Iterates through a format string to identify and process conversion specifiers for integers, strings, and hexadecimal values.
- `parse_printf_format_string_3`: Iterates through a format string to identify and process conversion specifiers for integers, strings, and hexadecimal values.
- `parse_printf_format_specifiers`: Iterates through a format string to identify and process conversion specifiers for integers, strings, and hexadecimal values.
- `process_printf_format_specifiers`: Iterates through a format string to identify and process conversion specifiers for integers, strings, and hexadecimal values.
- `parse_printf_format_string_2`: Iterates through a format string to identify and process numeric conversion specifiers such as integers, strings, and hexadecimals.
- `parse_printf_format_string`: Iterates through a format string to identify and process numeric conversion specifiers.
- `parse_numeric_format_specifiers`: Iterates through a format string to identify and process numeric conversion specifiers such as integers, strings, and hexadecimals.
- `parse_numeric_format_specifier`: Processes a format string to extract and convert numeric values, handling sign prefixes and decimal digits.
- `parse_formatted_string_data`: Iterates through a string to identify and extract numeric values based on specific format specifiers.
- `parse_formatted_string_2`: Parses a string containing format specifiers to extract numeric values and update a state structure.
- `parse_format_string_specifiers_3`: Iterates through a string to identify and process format specifiers, extracting integer, floating-point, or hexadecimal values.
- `parse_formatted_string_3`: Parses a string to extract numeric values and variables using format specifiers.
- `parse_format_string_specifiers_2`: Iterates through a string to identify and process format specifiers, extracting integer, floating-point, or hexadecimal values.
- `parse_formatted_string_4`: Parses a string for format specifiers, extracting numeric values, floating-point numbers, or data from a pointer-based list.
- `parse_format_string_specifiers`: Iterates through a string to identify and process format specifiers such as integers, floating-point numbers, and hexadecimal values.
- `parse_formatted_string_5`: Iterates through a string to identify and extract numeric values based on format specifiers.
- `parse_format_string`: Parses a string containing formatted specifiers, including integers, floating-point numbers, and hexadecimal values.
- `fetch_offset_word`: Retrieves a two-byte value from a memory location calculated by adding a negative offset to a pointer held in a register.
- `initialize_and_call_callback`: Checks a global variable and, if zero, calls another routine to populate that variable with a returned value.
- `format_floating_point_scientific`: Converts a floating-point value into a null-terminated ASCII string using scientific notation.
- `format_scientific_notation_string_5`: Converts a floating-point value into a null-terminated ASCII string using scientific notation.
- `format_scientific_notation_string_4`: Converts a floating-point value into a null-terminated ASCII string using scientific notation.
- `format_scientific_notation_string_3`: Converts a floating-point value into a null-terminated ASCII string using scientific notation.
- `format_scientific_notation_string_2`: Converts a numeric value into a null-terminated ASCII string using scientific notation.
- `format_scientific_string`: Converts a numeric value into a null-terminated ASCII string using scientific notation formatting.
- `format_scientific_notation_string`: Converts a numeric value into a null-terminated ASCII string, handling decimal point placement and scientific notation formatting with exponent signs and digits.
- `format_numeric_string`: Converts a signed integer into a null-terminated ASCII string, handling signs, decimal points, and scientific notation formatting.
- `format_integer_string_2`: Converts a signed integer into a null-terminated ASCII string, handling signs and decimal points.
- `format_integer_string`: Converts a 16-bit integer into a null-terminated ASCII string, including a sign character and handling values up to three digits.
- `convert_integer_to_string`: converts a 16-bit integer value into a null-terminated ASCII string representation.
- `dispatch_indirect_function_call`: Checks if a cached function pointer is null, attempts to initialize it by calling another routine, and then executes the resulting function via an indirect call using values retrieved from a stack-based structure.
- `dispatch_indirect_function_call_2`: Checks if a specific value in a memory location is zero, then attempts to initialize it by calling another routine.
- `dispatch_indirect_function_call_3`: Checks a value in a memory location and, if zero, executes a sequence of other routines to populate it.
- `initialize_and_call_indirect_function`: Checks if a specific value is zero, then attempts to initialize it by calling another routine.
- `resolve_and_call_indirect_function`: Checks if a value is stored in a specific memory location, and if not, executes a series of routines to populate it.
- `dispatch_indirect_function_call_4`: Checks if a cached function pointer is null and, if so, attempts to retrieve it through a series of fallback routine calls.
- `initialize_and_call_indirect_function_2`: Checks if a specific value in a memory structure is zero, and if so, executes a series of initialization routines and updates that value.
- `dispatch_indirect_function_call_5`: Checks a value in a memory location and, if zero, executes a sequence of fallback routines to populate it.
- `dispatch_indirect_function_call_6`: Checks a cached value in a structure and, if zero, executes a sequence of fallback routines to populate it.
- `execute_indirect_call_6`: Retrieves a function pointer from a structure and executes it after performing several conditional checks and assignments.
- `execute_indirect_callback_2`: Checks a global variable and executes a routine at a calculated address via an indirect call.
- `chain_result_to_output`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_indirect_function`: performs an indirect call to a function pointer retrieved from a global structure.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `increment_and_dereference_pointer`: Compares two values at a memory location and, if the first is less than the second, increments the first value and returns the byte stored at the address it previously contained.
- `increment_and_dereference`: Increments a pointer stored at a memory location and returns the value previously pointed to by that pointer.
- `process_structure_list`: Clears a bit in a byte within a structure and recursively iterates through a linked list of structures.
- `process_structure_data`: Clears a bit in a byte within a structure, iterates through a list of structures based on a value, and updates a pointer and longword values based on the result of a sub-routine.
- `set_bit_in_byte`: Sets the fifth bit of a byte located at an offset from a provided pointer.
- `update_pointer_and_sum`: Updates a memory location with a value from an offset, calculates a sum with another value, and returns a concatenated result.
- `compare_and_return_result`: Calls a string comparison routine and returns zero if the comparison fails, otherwise returns the result of another routine using the comparison result as an argument.
- `call_string_comparison`: Loads two values from offsets relative to a register and passes them as arguments to another routine.
- `compare_string_list_2`: Iterates through a list of strings to find a match against a provided string.
- `compare_string_list`: Compares strings within a list and, if a match is found, stores a value and a constant into a structure pointed to by a member.
- `update_structure_member`: Checks a value from a pointer, and if it is not negative one, stores that value into a byte at an offset and stores a constant into another offset within a structure.
- `write_bytes_to_structure`: Writes a byte and a constant value to specific offsets within a structure pointed to by a register.
- `execute_conditional_logic_3`: Performs a switch-based operation that either clears a value, calls another routine via a pointer, or updates a global variable based on a condition.
- `execute_conditional_logic_2`: Performs a switch-based operation that either clears a value, calls another routine via a pointer, or updates a global variable based on a comparison.
- `execute_conditional_logic`: Performs different actions based on a value in a register, including calling another routine via a pointer and writing a zero to a memory location.
- `call_and_return_short`: Calls another routine using a pointer from a memory location and returns a 16-bit value from an offset.
- `process_memory_list`: Iterates through a list of values, performs pointer arithmetic, and executes a function pointer found during the traversal.
- `return_immediately`: This routine performs no operations and returns immediately to its caller.
- `pass_argument_unchanged`: Passes its argument to another routine unchanged and returns.
- `update_entity_table_entries`: Iterates through a table of entries, checking for empty slots and attempting to populate them by calling various subroutines based on bit flags in a parameter.
- `update_status_table`: Iterates through a table to check bit flags and updates entries with a longword and a word based on specific conditions.
- `update_file_status_table`: Checks bit flags in a structure to determine whether to perform file operations or update a table with a longword and a word based on an index.
- `update_table_entry_5`: Updates a table with a longword and a word based on an index and conditional logic.
- `update_table_entry_4`: Writes a longword and a word into a table at an offset calculated from an index and conditionally calls another routine.
- `update_table_entry_3`: Writes a longword and a word into a table at an offset calculated from an index, then conditionally calls another routine.
- `manage_file_state`: Checks bit flags in a structure to determine whether to call another routine or update a table of values.
- `update_table_entry_2`: Writes a longword and a word to a table at an offset determined by an index, then conditionally calls another routine based on a bit flag.
- `perform_io_read_wrapper`: A 19-instruction routine that calls a routine to attempt an I/O read and checks a library function result against a global variable.
- `perform_io_read_2`: Attempts to read data using parameters from a structure and handles errors by updating a global variable if the operation fails.
- `perform_io_read`: Attempts to read data using parameters from a structure and, if the operation fails, calls another routine to handle the error and stores the result in a global variable.
- `return_immediately_8`: This routine performs no operations and returns control to the caller immediately.
- `process_directory_entries`: Iterates through a list of directory entries to populate a table with derived data and updates specific memory locations.
- `traverse_directory_entries`: Iterates through a list of directory entries, performing a loop to populate a table with data derived from the entries.
- `return_stack_value`: This routine returns the value currently held at the top of the stack.
- `open_and_traverse_dos_file`: Opens a file via a library function and iterates through its directory entries until a null pointer is found.
- `store_register_to_memory`: stores the contents of a data register into a memory location pointed to by an address register.
- `call_alert_2`: Wrapper that calls the exec library function Alert and returns its result to the caller.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_and_initialize_memory`: Clears a block of memory using a loop and updates several global pointers and values.
- `return_from_subroutine_3`: This routine performs a single return instruction to exit the current execution flow.
- `parse_quoted_string_list`: Parses a string into a list of individual tokens, handling both space-separated words and text enclosed in double quotes.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit_3`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `manage_window_state`: Uses multiple indirect calls and library functions to retrieve values from a structure and store them in a provided parameter block.
- `signed_divide_longwords`: Performs a signed division by calling an unnamed unsigned routine and negating the result when the operand signs differ.
- `perform_signed_longword_division`: Performs a signed 32-bit division by calling an unsigned division routine and adjusting the sign of the result.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `divide_longwords`: Performs division on 32-bit values using a combination of word-based division and bit manipulation.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `execute_recursive_call_3`: The routine performs a single call to itself before returning.
- `dispatch_subroutine_calls`: Calls a routine to update a buffer via a pointer, and if a specific bit is set in a byte from a table, calls another routine to manage buffer data.
- `return_immediately_9`: This routine is empty and returns to its caller without performing any operations.
- `update_buffer_via_pointer`: Increments a pointer stored in a global variable and stores a byte from another location into the address previously pointed to.
- `increment_pointer_and_store_byte`: Increments a pointer stored in a global variable and stores a byte from another location into the address previously pointed to.
- `traverse_address_range`: Iterates through a range of memory addresses by incrementing a pointer by a fixed offset.
- `release_structure_and_free_children`: Given a pointer to a structure, returns 65535 if the pointer is null.
- `clear_structure_fields_2`: Clears several fields within a structure and performs conditional routine calls based on bit flags.
- `manage_buffer_data`: Checks status flags in a structure and performs conditional updates to a buffer pointer, including clearing fields or inserting a byte based on specific state conditions.
- `manage_buffer_state`: Checks status flags and updates a buffer pointer and its associated data based on specific conditions and library results.
- `process_buffer_data`: Manages a data buffer by copying bytes from a source or updating internal pointers and status flags based on specific conditions.
- `update_buffer_and_check_status`: Updates a structure by copying a byte from a source to a destination pointed to by a value in a table, or modifies structure fields and resets them based on the result of another routine.
- `update_list_and_store_byte`: Updates a structure by modifying several fields, including a bitwise flag, and stores a byte from a source into a destination pointed to by a member of that structure.
- `find_empty_structure`: Iterates through a list of structures to find the first one where a specific byte is zero, then clears the first three longwords of that structure.
- `search_list_for_null_flag`: Iterates through a list of structures until it finds one where a specific byte is zero, then clears the first three longwords of that structure.
- `clear_structure_fields`: Sets the first three longwords of a structure pointed to by a global variable to zero.
- `set_structure_status`: Modifies specific bytes and longwords within a structure based on the result of another routine.
- `set_structure_fields`: Sets a longword and modifies specific bytes within a structure, then conditionally sets a bit in a byte based on a routine return value.
- `traverse_linked_list_5`: Iterates through a linked list of pointers, calling another routine for each element, and clears a global variable upon completion.
- `traverse_and_call_linked_list`: Iterates through a linked list of pointers, calling another routine for each element, and finally clears a global variable.
- `insert_linked_list_node_2`: Allocates or retrieves a new structure, populates it with data, and inserts it into a linked list while updating a global pointer.
- `copy_and_update_structure_2`: Copies two longwords from different memory locations into a new structure and updates a global pointer to point to this structure.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_linked_node`: Traverses a linked list to find a specific node and removes it by updating the preceding pointer in the chain.
- `traverse_pointer_chain`: Iterates through a linked list of pointers until a specific target is found or the end of the chain is reached, then stores the result.
- `store_pointer_value`: Copies a value from one memory location to another, or to a global variable if the destination pointer is null.
- `store_pointer_to_memory`: Stores a value from a register into a memory location calculated from a global variable and an offset.
- `check_table_status`: Validates an index against a range and checks a table entry to determine if a specific status is active, returning a boolean result.
- `check_interactivity_status`: Retrieves a pointer from a table and calls another routine to check a status value, returning a boolean result.
- `return_zero_value_2`: This routine performs no operations and returns zero to the caller.
- `handle_file_deletion`: Calls another routine and, if it returns zero, calls a second routine to retrieve an error value and stores it in a global variable.
- `return_zero_5`: This is an empty routine that returns zero to the caller.
- `perform_io_write_wrapper`: A 19-instruction routine that calls a routine to attempt an I/O write and checks a library function result against a global variable.
- `perform_io_write`: Attempts to write data using parameters from a structure and updates a global variable with an error code if the write fails.
- `write_and_handle_error`: Attempts to write data using a pointer and, if an error occurs, calls an error routine and stores the result in a global variable.
- `return_without_action_2`: The routine consists of two instructions and does nothing but return to its caller, so it performs no I/O, error handling, or state change of its own.
- `call_library_with_check`: Calls a library function and, if a specific bit is set in the result, checks a global variable and potentially calls another routine.
- `call_and_return`: Calls another routine and returns zero.
- `execute_stored_callback`: Checks if a global pointer is non-zero and, if so, executes the routine at that address.
- `execute_callback_chain`: Checks various global pointers and executes a sequence of indirect function calls and library functions based on their values.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: Validates a file handle against a global table and calls a routine to close the file if the handle is valid.
- `call_dos_close`: Calls a function through a pointer retrieved from a global table.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_dos_current_dir`: Calls a function through a pointer retrieved from a global table to retrieve the current directory.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `delete_file`: Calls a function to delete a file and returns the result.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_execute`: Wrapper that calls the dos library function Execute and returns its result to the caller.
- `call_exit`: Wrapper that calls the dos library function Exit and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `report_io_error`: Calls a function to report an input/output error.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `open_file`: Calls a function to open a file.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `read_file`: Calls a function to read data from a file.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `unlock_file`: Calls a function to unlock a file.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `write_file`: Calls a function to write data to a file.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_indirect_library_function`: Performs an indirect call to a function located at an offset from a pointer stored in a global variable.
- `call_library_offset_414`: Calls a library function using an offset derived from a global pointer.
- `allocate_and_initialize_structure`: Allocates a memory block and populates several fields within that block using provided parameters and results from other routines.
- `modify_data_structure`: Checks a value in a structure, potentially recurses, and then writes specific byte and long values to that structure before calling another routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_indirect_library_function_2`: Performs an indirect call to a function located at a specific offset from a pointer stored in a global variable.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_indirect_function`: Retrieves a function pointer from a nested table structure and executes it.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_indirect_function_2`: Performs an indirect call to a function located via an offset from a pointer in a table.
- `execute_indirect_table_function`: Dereferences a pointer from a table located at an offset from a base address, then executes the function found at a secondary offset within that target structure.
- `call_indirect_function_3`: Executes a function call by dereferencing a pointer found at a specific offset within a table.
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
- `call_indirect_function_4`: Calls a subroutine using a pointer retrieved from a table based on a calculated offset.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_indirect_function_5`: Uses a pointer from a table to jump to another routine.
- `call_indirect_function_6`: Triggers a function call through a pointer stored in a table at a specific offset.
- `call_indirect_function_7`: Performs an indirect jump to a routine using a pointer from a data table.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_indirect_function_8`: Calls a routine by dereferencing a pointer located at an offset from a base address in a table.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_indirect_function_9`: Executes a function via an indirect pointer retrieved from a table.
- `call_indirect_function_10`: Performs an indirect call to a routine using a pointer from a table.
- `call_indirect_function_11`: Calls a function by following a pointer stored at a specific offset within a table.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_indirect_function_12`: Performs an indirect call to a function located via an offset from a pointer stored in a global structure.
- `call_indirect_function_13`: Executes a function call using a pointer retrieved from a table with a specific offset.
- `call_indirect_function_14`: Calls a subroutine by dereferencing a pointer found at a specific offset within a global data structure.
- `call_indirect_function_15`: Uses a pointer from a global structure to perform an indirect function call.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_indirect_function_16`: Triggers an indirect call to a function based on an offset from a pointer in a data hunk.
- `parse_and_update_data`: Parses a format string and increments a value in a global table while storing a byte into a location pointed to by that table value.
- `increment_and_store_byte`: Increments a value in a global table and stores a byte from a parameter into the location pointed to by the original table value.
- `update_value_via_callbacks`: Calls a routine via a pointer, stores the result in a global variable, and then executes two more indirect routine calls.
- `execute_indirect_call_10`: Calls a routine via a pointer stored in a structure and passes the result to another routine.
- `update_cached_callback`: Checks if a value is stored in a global variable and, if not, calls another routine to populate it.
- `update_cached_callback_2`: Checks a global variable and, if zero, calls another routine to populate it with a value.
- `update_cached_function_pointer`: Checks if a value in a global variable is zero, then calls another routine to populate that variable with a new value.
- `clear_structure_on_error`: Clears specific bits in a structure and, if certain conditions are met, resets the structure's first two words to zero.
- `initialize_and_test_value`: Clears two consecutive memory locations and returns a value based on the result of another routine.
- `update_offset_and_seek`: Adjusts a value in a structure based on the difference between two elements in a pointer-referenced array and performs a seek operation.
- `return_zero_6`: This routine performs no operations and returns a value of zero to its caller.
- `calculate_array_difference`: Calculates a value by adding or subtracting the difference between elements of an array to a value returned by another routine.
- `subtract_array_difference`: If the first element of an array is non-zero, subtracts the difference between the first two elements from a stored longword and returns it.
- `execute_seek_operation`: Performs a seek operation and handles potential errors by calling an error routine and updating a global variable.
- `seek_and_handle_error`: Attempts to seek to a position and, if unsuccessful, calls an error routine and stores the result in a global variable; otherwise, it seeks to the start of a file.
- `call_seek_wrapper`: Calls a routine with three arguments, where the first argument is a pointer retrieved from a data structure.
- `call_indirect_function_17`: Performs an indirect call to a function using a pointer offset from a global variable.
- `execute_indirect_call_7`: Performs an indirect call to a function pointer located at an offset within a structure pointed to by a register.
- `call_indirect_function_18`: Executes a function call through a pointer retrieved from a data structure offset.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_indirect_function_19`: Calls a function indirectly by accessing a pointer through a specific offset in a global structure.
- `execute_indirect_call`: Performs an indirect call to a function address retrieved through multiple pointer dereferences.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.

### mathtrans.library

- `halt_execution`: Library code that performs an unimplemented instruction to stop execution.
- `store_register_to_global`: Library code that stores the value of a register into a global variable.
- `manage_memory_list`: Library code that manages a linked list by updating pointers and setting a bit in a structure before calling a memory free routine.
- `set_byte_bit`: Library code that sets the fourth bit of a byte located at an offset from a pointer.
- `increment_reference_count`: Library code that increments a short value at an offset within a structure and conditionally calls another routine if the value reaches one.
- `return_all_ones_2`: Library code that returns a value consisting of all bits set to one.
- `decrement_counter_and_call`: Library code that decrements a 16-bit counter at a specific offset in a structure.
- `return_from_subroutine_9`: This routine performs a single return from a subroutine to the calling function.
- `calculate_bitfield_difference`: Library code that performs a loop to calculate a value based on bitwise shifts and subtractions involving a running difference and an accumulator.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `mask_library_result`: Library code that calls a function through a pointer and returns the result with a specific bitmask applied.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `execute_callback_sequence_4`: Invokes a series of function pointers retrieved from a table via offsets relative to a base pointer.
- `dispatch_library_callbacks`: Library code that executes a sequence of function pointers from a table and returns a masked result.
- `execute_conditional_callback_chain`: Executes a sequence of function pointers from a table based on the values of several registers.
- `dispatch_conditional_callbacks`: Library code that executes various routines through pointers based on the values of several registers and returns a masked result.
- `call_multiple_and_mask`: Library code that calls various routines through pointers based on the value of a register and returns a result with a bitmask applied.
- `call_and_mask_result`: Library code that calls a function through a pointer and returns the result of another routine with a bitmask applied.
- `double_call_and_mask`: Library code that calls the same function through a pointer twice and returns a masked result from another routine.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `execute_indirect_call_2`: Library code that performs an indirect call to a function pointer retrieved from a table.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `execute_indirect_call_3`: Library code that executes a function via a pointer located at a specific offset within a table.
- `perform_arithmetic_loop`: Library code that performs bit-shifting and arithmetic operations within a loop, potentially invoking a callback routine.
- `execute_arithmetic_loop`: Library code that performs a loop involving bit-shifting and arithmetic operations, potentially calling another routine.
- `execute_loop_with_branching`: Library code that executes a loop performing bit-shifting and arithmetic, potentially calling other routines if a counter reaches a certain value.
- `perform_complex_math`: Library code that performs bitwise shifts and arithmetic operations on a value using a loop and a global table.
- `execute_math_calculation`: Library code that performs conditional checks on a byte and executes a loop involving bitwise shifts and additions.
- `normalize_and_index_table`: Library code that performs bitwise shifts on a value based on its sign and magnitude, then uses the result to traverse a table of integers through addition and subtraction until a target value is reached.
- `perform_complex_arithmetic`: This library code performs a series of conditional checks and bitwise operations on a value before iterating through a table to modify registers.
- `perform_math_calculation`: Library code that performs complex conditional checks and bitwise operations on a value to determine a multiplier, then iterates through a global table to perform arithmetic.
- `transform_arithmetic_list_elements`: Traverses a pointer-based list to perform bitwise shifts and arithmetic updates on two 32-bit values.
- `perform_arithmetic_loop_2`: Library code that performs complex arithmetic and bitwise operations while traversing a pointer-based list.
- `traverse_arithmetic_list`: Library code that performs bitwise shifts and arithmetic operations while traversing a pointer-based list and making recursive calls.
- `perform_recursive_arithmetic_traversal`: Library code that performs bitwise shifts and arithmetic operations on values while traversing a pointer-based list and making recursive calls based on the sign and value of a stack-based integer.
- `execute_empty_routine_2`: This library code is empty.
- `recursive_call_wrapper`: Calls itself and then returns, with no other operations.
- `execute_recursive_calls_2`: Library code that calls itself twice before returning.
- `multiply_and_shift_value`: Library code that performs bitwise shifts and multiplications on a value based on its magnitude and sign.
- `process_value_with_library_call`: This library code performs bitwise shifts and multiplications on a value and calls another library function.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_recursive_callbacks`: Library code checks a byte value and performs multiple recursive calls to another routine based on bitwise evaluations and boolean flags.
- `return_from_subroutine_7`: This routine performs an immediate return to the caller.
- `return_from_subroutine_8`: This routine performs an immediate return to the calling function.
- `test_and_branch_on_word`: Loads a word from the stack into register d6, tests if it is positive, and if so, branches to another routine; otherwise, it clears register d7 and branches to another routine.
- `set_negative_flag_and_return`: Sets the negative flag in the condition code register and restores registers from the stack before returning to return_immediately_10.
- `set_status_and_return`: Loads a specific 32-bit constant into a register, adjusts the stack pointer by 30 bytes, and returns.
- `process_concatenated_values`: Library code that performs bitwise operations and multiple recursive calls to manipulate and concatenate values.
- `execute_multiple_recursive_calls`: Library code that calls itself four times in succession before returning.
- `accumulate_and_subtract_table`: Library code that iterates through a table to add entries to an accumulator and then subtracts from it.
- `subtract_table_from_accumulator`: This library code walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `execute_complex_math`: Library code that performs bit-shifting, arithmetic loops, and repeated multiplication based on input values.
- `return_immediately_3`: This routine performs no operations and returns immediately to the caller.
- `perform_complex_math_loop`: Library code that executes a series of callbacks and performs a loop involving bit-shifting and arithmetic.
- `complex_loop_with_branching`: Library code that executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `execute_conditional_recursion`: Library code that performs bitwise shifts and conditional checks on a value before invoking another routine multiple times.
- `process_complex_bit_manipulation`: Library code performs several conditional checks and bitwise shifts on a value before invoking another routine multiple times.
- `transform_register_d7_and_return`: Adds the value of d7 to itself, performs a series of bitwise operations including an exclusive OR, an arithmetic shift, a subtraction, a word swap, and a rotate left, then returns.
- `frag_swap_ror_eori_add_bvs_addq_rts`: Short code fragment that swaps the halves of d7, then rotates d7 right by 7, then exclusive-ors 128 into d7, then adds d7 to d7, then branches if the overflow flag is set, then adds 5 to d7, then branches if the overflow flag is set, then exclusive-ors 128 into d7, then rotates d7 right by 1, then returns.
- `dispatch_math_library_call`: Library code in libs/mathtrans.library that is reached through a pointer stored in data and is called from call_library_vector_90.
- `empty_routine_2`: The library code is empty.
- `frag_cmpi_bne_lsr_bra`: Short code fragment that compares d7 with 254, then branches if the last result was not zero, then shifts d7 right by 8, then shifts d7 right by 1, then branches if the last result was not zero, then jumps to another point in the code.
- `modify_d7_and_branch`: Performs bitwise shifts and rotations on register d7 and then branches to another location.
- `return_from_subroutine_4`: This routine is an empty function that performs no operations before returning.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `return_input_value`: Library code that calls a routine and returns the original value from a register.
- `return_register_value_2`: Library code that simply returns the value currently held in a register.
- `invoke_callback_and_return_2`: Library code that calls a routine pointed to by a register and returns the original value from another register.
- `invoke_callback_wrapper`: Library code that calls a routine through a pointer and returns the original value from a register.
- `invoke_callback_wrapper_2`: Library code that calls a routine via a pointer in a register and returns the original value from a register.
- `invoke_callback_and_return_3`: This library code calls a routine pointed to by a register and returns the original value from a data register.
- `execute_callback_and_return_3`: Invokes a routine via a pointer provided in a register and returns the original value held in a data register.
- `store_register_via_pointer`: This library code invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register.
- `execute_callback_and_return`: Invokes a routine via a pointer and returns the original value held in a data register.
- `execute_callback_and_return_2`: Calls a routine via a pointer and returns the original value held in a data register.
- `pass_through_callback`: Library code that calls another routine and returns the original value from a register.
- `execute_recursive_callback`: This routine performs a recursive call to another routine before returning a value held in a register.
- `pass_through_register`: An empty routine that returns the value originally held in a register.
- `return_passed_register`: Library code that calls a routine through a pointer in a register and returns the original value of that register.
- `call_indirect_routine`: Library code calls a routine via a pointer in A0 and returns the value from D0.
- `invoke_callback_routine`: Library code calls a routine via a pointer in A0 and returns the original value from D0.
- `execute_callback_routine`: Calls a routine via a pointer provided in a register and returns the original value from that register.
- `call_offset_routine`: Library code calls a routine at an address calculated by adding an offset to a global pointer and returns the result in D0.
- `execute_indirect_offset_call`: Calls another routine by adding a negative offset to a pointer retrieved from a global variable.
- `invoke_concat_callback`: Library code calls a routine via a global pointer and returns the concatenation of D0 and D1.
- `execute_callback_and_concatenate`: Invokes a routine via a pointer stored in a global variable and returns the concatenation of the resulting high and low 32-bit values.
- `execute_indirect_offset_call_2`: Retrieves a pointer from a global variable, applies a negative offset to it, and executes the resulting address.
- `execute_indirect_offset_call_3`: Retrieves a pointer from a global variable, applies a negative offset to it, and executes the resulting address.
- `invoke_callback_and_concatenate`: Executes a routine via a pointer retrieved from a global variable and returns the concatenation of the two input registers.
- `invoke_callback_and_concatenate_2`: This library code invokes a routine via a pointer stored in a global variable and returns the concatenation of two input values.
- `execute_callback_and_concatenate_2`: Invokes a routine via a pointer stored in a global variable and returns the concatenation of the resulting two 32-bit values.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_error_code`: Library code that returns a constant value of all bits set.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `manage_library_refcount`: Library code that updates a linked list of references and sets a status bit in a structure if a condition is met.
- `set_status_bit`: Library code that performs a bitwise OR operation to set a specific bit in a byte at an offset from a pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_and_store_results`: Library code that stores two values from a pointer into a structure, executes a function via a pointer from a register, and then updates two global variables with the results if the function returns a non-zero value.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `execute_empty_routine`: Library code that contains an empty routine.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `recurse_with_incremented_parameter`: Calls another routine using the same first argument while incrementing the second argument by a constant before returning.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `invoke_recursive_call`: This routine performs a direct recursive call to itself before returning.
- `trigger_recursive_call`: Triggers a recursive call to itself without any conditional branching or state modification.
- `call_other_routine`: This library code calls another routine and then returns.
- `search_memory_for_pattern_3`: This routine calls another routine to locate a specific sequence of bytes within a memory range.
- `call_string_to_numeric_conversion`: This routine calls a function that converts an ASCII string into a numeric value.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_byte`: Performs a bitwise OR operation on a register with zero.
- `update_icon_attributes`: Library code that retrieves a structure, calls another routine to populate data, and then copies a block of values from the stack into the structure.
- `copy_and_update_structure`: Copies a block of data from one memory location to another and updates several fields within a structure using values from the source.
- `return_zero_stub`: Clears register d0 to zero, restores registers d2 through d4 and a2 from the stack, unlocks a6, and returns.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_attribute_structure_2`: Traverses an icon attribute structure to update various integer members and pointers by invoking a callback routine.
- `update_icon_attribute_structure`: Traverses a structure to update various integer members and pointers by invoking a callback routine.
- `update_structure_members`: Library code that traverses a structure and invokes a callback to update various integer members and pointers.
- `execute_handler_loop`: Library code that repeatedly calls a function through a pointer and updates a structure based on the results.
- `call_and_return_zero`: Library code that calls another routine and returns zero.
- `return_without_effect_2`: Moves a longword from d4 to d0, restores registers d2 through d5 and a2 through a5 from a stack offset, unlocks a6, and returns.
- `traverse_icon_structure_2`: Recursively iterates through a hierarchical icon data structure by following nested pointers and lists.
- `traverse_icon_structure`: Iterates through a hierarchical icon data structure by following nested pointers and lists.
- `iterate_icon_elements`: Library code that iterates through a table and secondary lists, invoking a callback for each entry and updating a counter.
- `iterate_and_callback`: Iterates through a null-terminated table and a secondary list, invoking a callback routine for each entry and updating a counter.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list`: Recursively traverses a linked list structure by following pointers and executing a routine on each node.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `invoke_library_functions`: Library code that calls other library functions using offsets and passes parameters to them.
- `recursive_call_wrapper_2`: Calls itself and then returns, with no other operations.
- `return_zero_in_d0`: Sets register d0 to zero and restores the previous register states and stack pointer before returning.
- `update_list_element_2`: Library code that decrements a counter and stores two values into a table at an offset determined by that counter.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `manage_icon_attributes`: Library code that manipulates icon attribute structures by calling various internal library functions and managing pointers.
- `execute_empty_wrapper`: This library routine acts as a wrapper that calls another routine and then returns zero.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `search_equals_character`: This library code iterates through a list of pointers to find a specific character within a memory buffer.
- `find_equal_sign`: This library code iterates through a memory buffer to find the position of an equals sign character.
- `find_equals_character`: Library code that iterates through a memory buffer to locate the position of an equals sign character.
- `search_memory_for_pattern_2`: Iterates through a memory range to locate a specific sequence of bytes.
- `search_memory_offsets`: This library code iterates through memory blocks to find specific offsets and compares them against a value.
- `search_memory_blocks`: This library code performs a loop that compares memory buffers and calculates offsets.
- `execute_loop_logic`: Library code that performs a loop involving arithmetic and conditional calls to another routine.
- `search_memory_offsets_2`: Library code that iterates through memory to find specific offsets and compares them against a target value.
- `convert_string_to_numeric`: Library code that parses an ASCII string to convert it into a numeric value and copies the result into a destination buffer.
- `parse_numeric_string`: This library code converts a sequence of ASCII characters into a numeric value by iterating through a string and multiplying the accumulator by ten for each digit.
- `convert_ascii_numeric_string`: This library code converts a sequence of ASCII characters into a numeric value.
- `convert_ascii_string`: Library code that iterates through a string to convert ASCII characters into a numeric value.
- `convert_ascii_digit`: Library code that converts an ASCII character to its numeric value or returns negative one if the input is out of range.
- `return_negative_one_3`: This library routine returns a value of negative one.
- `execute_callback_offset_c6`: This library routine calls a function pointer located at an offset from a global base address.
- `execute_indirect_callback`: Fetches a function pointer from an offset within a global variable and executes it.
- `execute_indirect_function_2`: Fetches a function pointer from a location within a global table and executes it.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a location calculated by an offset from a global variable and executes it.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `execute_library_handler`: Library code that calls a function through a pointer retrieved from a specific offset in a data structure.
- `execute_library_handler_2`: Library code that calls a function through a pointer retrieved from a different offset in a data structure.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `update_list_element`: Library code that calls another routine and, if successful, writes a value to a specific offset in a structure and updates a pointer.
- `return_zero_2`: Library code that returns zero.
- `compare_and_return_zero`: Compares a passed value against a fourth parameter and returns zero if they are not equal, otherwise returns the result of the comparison.
- `tiny_move_rts_3`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `compare_and_return_zero_2`: Library code that calls another routine and returns zero if the result does not match a provided parameter.
- `return_nothing`: An empty library routine that performs no actions and returns.
- `rotate_bits`: This library routine performs a bitwise rotation on a 32-bit value through sixteen iterations.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_4`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_delimiter_in_string`: This library code scans a string for a specific delimiter character.
- `return_zero_7`: This library code is empty and always returns zero.
- `copy_c_string_2`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_string`: This library routine calculates the length of a source string and then copies its contents to a destination pointer until a null terminator is reached.
- `compare_memory_buffers`: Library code that compares two memory buffers for equality up to a specified length.
- `return_constant_one_2`: Library code that returns the value one.
- `decrement_and_traverse`: This library code iterates through a memory buffer while decrementing a counter.
- `fetch_function_pointer`: Loads a function pointer from a memory location into a register.
- `copy_string_with_padding`: Library code that copies a string from one location to another and pads the remaining length with null bytes.
- `return_nothing_2`: This library routine is empty.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `traverse_linked_structure`: Library code that iterates through a linked structure by checking members and following pointers.
- `check_structure_member`: Library code that iterates through a structure and returns one if a specific offset contains zero, otherwise returns zero.
- `return_zero_3`: A library routine that returns zero.
- `calculate_complex_value`: Library code that performs a series of nested calls and arithmetic operations to populate a field in a structure.
- `return_negative_one_2`: A library routine that returns a value of all bits set.
- `iterate_list_with_callbacks`: Library code that traverses a linked list and invokes a routine for each element while updating structure members.
- `traverse_and_update_list`: Library code that iterates through a list using callbacks, updating structure members and managing local state.
- `iterate_linked_list_with_callbacks`: Iterates through a linked list by invoking a callback for each element.
- `return_all_ones_3`: This library routine returns a value consisting of all bits set to one.

### Iron Lord (1989)(UBI Soft)[cr](Disk 1 of 2).adf:samplesdatas

- `modify_bitfields`: This routine performs bitwise AND and XOR operations on bytes at specific offsets from two different pointers.

### A590HD

- `tiny_sub`: Small routine of 1 instruction that subtracts d0 from d0.
- `initialize_dual_io_requests_2`: Allocates and populates two linked data structures with timing, buffer, and pointer information.
- `initialize_dual_io_requests`: Allocates and populates two linked data structures with timing, buffer, and pointer information.
- `initialize_io_request`: Populates an IO request structure with timing and buffer data before performing operations on two linked structures.
- `execute_callback_offset_6c`: This routine calls a function pointer located at an offset from a value in a register and returns zero.
- `execute_and_clear_pointers`: Calls a function via a pointer multiple times within a loop and then clears two long values at a specific offset.
- `return_zero_value`: Clears d0 and restores several registers from the stack before returning.
- `initialize_structure_data`: This routine populates several fields of a structure with specific byte and word values.
- `concatenate_and_search_list`: Concatenates two strings and repeatedly compares them against entries in a linked list until a match is found or a counter expires.
- `call_indirect_offset`: Calls a function through a pointer located at a specific offset from a base address.
- `update_table_and_length`: Performs bitwise operations on a table entry, copies data between memory locations, and calculates the length of a null-terminated string.
- `calculate_string_length`: Calculates the length of a null-terminated string and returns it.
- `compare_two_strings_2`: Compares two null-terminated strings and returns zero if they are identical, one if the second is shorter, or a negative value if the first is shorter.
- `return_negative_one_4`: Returns a value of all ones.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `call_indirect_offset_large`: Calls a function through a pointer located at a large negative offset from a base address.
- `call_indirect_offset_medium`: Calls a function through a pointer located at a medium negative offset from a base address.
- `process_conditional_update`: Checks a signed integer and, if non-negative, updates a long value and increments a word within a structure.
- `update_structure_fields_3`: Updates a long value in a structure and increments a word at a specific offset if certain conditions are met.
- `increment_and_clear_byte`: Increments a word at an offset within a structure and then clears a byte at a different offset within another structure.
- `validate_digit_range`: Checks if a value is zero or if its tens and units digits are both within the range of zero to seven.
- `check_register_zero`: Returns zero if a register contains zero, otherwise returns a value of all ones.
- `search_linked_list`: Traverses a linked list to find an entry where two specific bytes match the digits of a given integer.
- `initialize_input_buffer_structure`: Initializes a complex data structure by populating it with various offsets and pointers, then processes an input buffer by checking for specific character matches and control flags.
- `process_input_buffer_chain`: Parses a linked list of input structures by checking character matches and control flags.
- `manage_input_buffer_linked_list`: Manages a linked list of input buffer nodes by evaluating bitwise properties and byte values within the structure.
- `traverse_and_modify_linked_list`: Iterates through a linked list structure by evaluating specific byte values and bitwise properties of node members.
- `manage_linked_list_nodes_3`: Iterates through a linked list to clear structure members and perform bitwise searches for specific power-of-two values.
- `manage_linked_list_nodes_4`: Inserts a new node into a linked list by performing a bitwise search for a specific power of two.
- `insert_node_with_bit_search`: Initializes a structure, performs a bitwise search to find a specific power of two, and inserts a new node into a linked list.
- `insert_linked_node`: Updates a structure with two values and performs a linked list insertion by updating pointers between nodes.
- `call_empty_routine`: Library code that calls another routine twice and returns zero.
- `insert_linked_list_node`: Inserts a new node into a linked list and updates the surrounding pointers.
- `initialize_complex_structure`: Populates a structure with various values and calls several functions through pointers retrieved from a base address.
- `decrement_short_value`: Decrements a 16-bit value stored at an offset from a pointer.
- `query_timer_device`: Interacts with a device to check a status and executes two different routines if a returned value is zero.
- `validate_table_data`: Checks a byte in a structure and, if certain conditions are met, iterates through a table to sum its elements.
- `sum_table_elements`: Iterates through a table of integers to calculate their sum and returns zero if the sum is non-zero.
- `populate_integer_table`: Checks if a value is zero and, if so, attempts to populate a table by iterating through a source structure.
- `process_integer_table`: This routine manipulates a table of values by copying data and incrementing a value at a specific offset.
- `process_event_chain_and_update_state`: Iterates through a linked list of event structures, performing conditional checks on flags and executing function pointers stored within the nodes.
- `process_event_queue`: Iterates through a linked list or queue of structures, performing conditional checks on flags and calling various subroutines.
- `manage_event_chain`: Traverses or extends a linked list of structures by allocating new nodes and copying data based on a bitmask.
- `insert_linked_node_2`: Inserts a new node into a linked list and performs a bitmask-based copy of data from the node to another location.
- `copy_structure_data`: Copies a block of data from one structure to another and updates several fields within a destination structure.
- `copy_data_and_update_pointers`: Copies a block of data from a source offset to a destination, then updates several fields within a structure.
- `sum_table_elements_3`: Iterates through a table to sum values based on a count, updating table entries and pointers during the process.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `traverse_nested_loops`: Executes nested loops to traverse data structures, checking specific byte values and bit flags to return a counter.
- `sort_integer_table_2`: Performs a heap-based sort on a table of integer values.
- `sort_integer_table`: Performs a sorting operation on a table of values using a heap-based algorithm.
- `return_immediately_4`: This routine is empty and performs no operations before returning to its caller.
- `instantiate_event_node_chain`: Allocates and initializes a complex event structure by populating it with data from a source table and linking multiple sub-nodes.
- `process_event_queue_2`: Checks a condition against a value in a structure and executes a function pointer if the condition is met.
- `update_state_structure`: Checks specific bits in a byte to conditionally clear a value and copy multiple bytes from one structure to another.
- `instantiate_and_process_event_node`: Allocates and initializes a new event structure by populating its members from passed parameters and calling associated function pointers.
- `process_nested_jump_table`: Iterates through a data structure and performs a nested table lookup to execute a function pointer based on bitwise manipulation of a byte.
- `manage_linked_list_nodes`: Iterates through a data structure to insert new nodes into a doubly linked list and updates pointer offsets.
- `execute_jump_table_entry`: Calculates an address using a nested table lookup based on bits from a register and executes the resulting code.
- `update_event_queue_and_state`: Manages a linked list of event structures by inserting new elements and updating pointers.
- `process_event_dispatch_logic`: Iterates through a linked list of event structures to validate state flags and perform conditional branching.
- `process_event_queue_loop`: Iterates through a linked list of event structures, performing pointer manipulations to remove elements and updating state flags.
- `traverse_linked_list_2`: Traverses a linked list searching for a matching byte and returns a pointer to a specific offset within a node.
- `reset_status_flags`: Writes specific byte values to two different memory locations and waits for a bit to be set in a third location.
- `update_buffer_state_and_copy`: Sets a series of status flags in two memory locations before performing a loop that copies bytes from a pointer into a destination buffer.
- `copy_byte_to_memory`: Stores a byte from a pointer into a memory location offset from a global variable.
- `clear_flag_and_word`: Sets a byte to zero at one location and clears a 16-bit word at another.
- `initialize_complex_structure_2`: Performs a series of pointer assignments and byte copies to initialize multiple linked data structures and internal offsets.
- `calculate_transfer_parameters`: Calculates the remaining length and alignment requirements for a data block transfer.
- `update_transfer_status_and_copy`: Updates counters and status flags for a data transfer operation, then performs a byte-by-byte copy from a source pointer to a destination buffer while polling a status bit.
- `transfer_data_buffer`: Transfers a sequence of bytes from a source pointer to a destination buffer while updating status flags and counters.
- `update_entity_attributes`: Increments two integer values at specific memory offsets and performs a bitwise right shift on a value before storing it.
- `update_multiple_values`: Updates several memory locations by adding a value to an existing integer, performing a bit shift, and storing a sequence of bytes into a table.
- `process_event_queue_and_update_state_2`: Iterates through a linked list of event structures, performing pointer manipulations and flag updates based on event type codes.
- `process_event_queue_and_update_state_6`: Iterates through a linked list of event structures to update status flags, reorder elements, and execute indirect function calls based on event types.
- `process_event_queue_and_update_state`: Iterates through a linked list of event structures, performing specific pointer manipulations and flag updates based on event type codes.
- `dispatch_event_queue_and_update_state`: Iterates through a linked list of event structures, executing function calls via a jump table based on event type flags.
- `process_event_dispatch_loop`: Manages a linked list of event structures by checking status flags and dispatching function calls via a jump table.
- `update_buffer_metadata`: Calculates the remaining length of a data block and updates various control structures and status flags.
- `fetch_and_store_byte`: Fetches a byte from a table using an offset derived from a pointer and stores it into two different memory locations.
- `dispatch_event_queue_and_update_state_2`: Iterates through a linked list of event structures, performing pointer manipulations and flag updates based on event type codes.
- `process_event_queue_and_update_state_3`: Iterates through a linked list of event structures, performing pointer manipulations and flag updates based on event type codes.
- `process_event_queue_and_update_state_4`: Iterates through a linked list of event structures, updating status flags and performing pointer manipulations to reorder or remove elements.
- `process_event_queue_and_dispatch`: Iterates through a linked list of event structures to update status flags and perform pointer manipulations.
- `copy_bytes_conditionally_2`: Iterates through a memory range and copies five bytes from a source pointer to a destination table if a specific bit in a control byte is set.
- `copy_bytes_conditionally`: Copies five bytes from a source pointer to a destination table, provided a specific bit in a control byte is set.
- `process_event_queue_and_update_state_5`: Iterates through a linked list of event structures, updating status flags and performing pointer manipulations to reorder or remove elements based on event type codes.
- `apply_dos_hunk_relocations`: Parses a memory range for AmigaDOS hunk types to perform relocation and code patching.
- `execute_conditional_table_calls`: Clears a range of memory and then enters a loop that executes routines from a table based on the return value of an indirect call.
- `dispatch_conditional_calls`: Executes a loop that conditionally calls routines based on the return value of an indirect call and performs table-based lookups.
- `fetch_conditional_offset_value`: Checks if a register is non-zero to call another routine, then retrieves a longword from a memory location using an offset from a pointer if a specific register equals negative one.
- `return_offset_value`: Returns a 32-bit value stored at an offset from a pointer held in a register.
- `execute_conditional_indirect_call`: Checks if a register is non-zero to conditionally call another routine, then proceeds to call a routine via a longword retrieved from a memory location using an offset from a pointer.
- `process_indirect_call_result`: Executes an indirect call and checks the return value.
- `validate_indirect_call_target`: Checks if a value multiplied by four exceeds a specific threshold and, if not, executes an indirect call through a pointer stored at an offset from a register.
- `execute_conditional_indirect_calls`: Performs two consecutive conditional indirect calls based on the state of a register.
- `update_pointer_value`: Retrieves a pointer via an indirect call and, if non-zero, stores a calculated value derived from a register into the memory location pointed to by that pointer.
- `tiny_movea_move_rts`: Small routine of 3 instructions that copies d0 to a0, then copies (a7)+ to (a0)+, then returns.
- `call_stored_function`: Checks if a register contains a non-zero value and, if so, executes the function pointed to by a value at a specific offset.
- `return_no_op_2`: The routine consists of a single return instruction and performs no work.

### Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:fgaulbaire.1989

- `initialize_copper_list`: Allocates memory and populates a copper list with instructions for color and display control.
- `close_library_and_loop`: Closes a library and enters a complex loop structure that performs conditional checks on a table and increments a global variable until a specific value is reached.
- `set_jump_table_entry`: Writes two 16-bit values to a table and updates a global variable while enabling DMA via the DMACON register.
- `set_multiple_jump_entries`: Writes several 16-bit values to a table and updates a global variable while enabling DMA via the DMACON register.
- `initialize_jump_table`: Populates a series of memory locations with specific 16-bit values and sets a global variable.
- `set_extended_jump_entries`: Writes a series of 16-bit values to a table and updates a global variable while enabling DMA via the DMACON register.
- `manage_system_resources`: Performs file operations and manages system state by updating several global variables and interacting with hardware registers.
- `fill_global_with_decrement`: Repeatedly writes a decrementing value to a global variable, which affects a hardware color register.
- `combine_source_words`: Iterates two thousand times, performing a bitwise OR on four values from a source table and storing the result in a destination table.
- `modify_pixel_data`: Iterates through a large buffer and updates a table of values by incrementing specific nibbles based on comparisons with the buffer content.
- `clear_register_byte`: Performs a bitwise OR operation on a register with zero.
- `decrement_table_values`: Performs a nested loop that subtracts values from specific bit segments of words stored in a table.
- `update_interrupt_state_and_swap_data`: Checks a hardware status bit and a global flag to perform a series of data swaps and bounds-checked updates to a table.
- `update_internal_state`: Swaps values between memory locations and writes a value to a hardware register.
- `update_interrupt_status`: Checks specific bits in a register and decrements global counters, updating a hardware register if a counter reaches zero.
- `set_global_parameters`: Stores a constant value into a global variable and several other global locations using bitwise operations.
- `call_function_and_update_globals`: Calls a function via a pointer and then updates four global memory locations using a constant value and bitwise OR operations.
- `manage_library_resources`: Updates several global variables, calls a function via a pointer, and interacts with library-related memory and system calls.
- `configure_blitter_registers`: Waits for a bit in a global variable to clear before writing several values to hardware registers, including blitter and DMA registers.
- `configure_blitter_registers_2`: Polls a global variable until a bit is cleared, then populates hardware registers with specific values from memory.
- `update_audio_and_blitter_parameters`: Calculates new pitch and volume values based on global variables and updates audio hardware registers.
- `store_register_pairs`: Searches a table for a zero word and, upon finding it, stores two input words into the table at that location.
- `process_table_data`: Iterates through a table to perform arithmetic on its elements and updates various memory locations based on specific conditions.
- `calculate_and_store_offsets`: Calculates several memory offsets using arithmetic on input registers and stores the results into a structure in memory.
- `update_input_state`: Reads joystick and button data from hardware registers to update several global status variables.
- `load_and_initialize_graphics_assets`: Loads graphic data from disk into memory and populates various tables with sprite and decoration metadata.
- `execute_subroutine_sequence`: Performs a sequence of subroutine calls, including conditional execution and a loop that waits for a specific value in a global variable.
- `select_parameter_values`: Sets two different values in memory based on the value of a global variable and then calls another routine.
- `initialize_data_structures`: Sets multiple values in a structure and performs a loop to populate a table with incrementing memory addresses.
- `setup_blitter_transfer`: Configures hardware registers for a blitter operation and fills a table with incrementing memory addresses through a loop.
- `search_and_copy_data`: Searches for a byte within a memory range and copies matching bytes into a destination table while calculating offsets.
- `process_text_input`: Evaluates a character from a buffer to update a text string, manage a cursor position, or reset state variables.
- `update_buffer_index`: Adjusts a global index value based on its current range and performs bitwise operations to update another global variable.
- `synchronize_and_copy_memory_block`: Polls hardware status registers and waits for specific bit changes before performing a memory block copy.
- `modify_structure_fields`: Writes specific values to a structure and enters a loop that waits for a bit in a memory location to change.
- `set_constant_value`: Loads the word value 54579 into d2 and returns.
- `check_condition_status`: Compares values from a pointer and a register, then performs bitwise checks to return a status value.
- `search_memory_for_pattern`: Iterates through a memory range looking for a specific word, then performs bitwise operations on data at a specific offset if a magic constant is found.
- `interleave_bits_2`: Combines bits from two consecutive words in a table using bitwise shifts and masks.
- `validate_data_structure`: Searches a memory block for a specific value and then validates a sequence of bit-swapped words against an incrementing counter.
- `initialize_memory_block`: Writes several values to a series of offsets within a memory block and then waits for a specific bit in a byte to be cleared.
- `wait_for_bit_clear_3`: Enters a loop that polls a specific bit in a memory location until it is cleared.
- `clear_memory_bit`: Clears a specific bit in a byte at a memory location and polls a bit in another memory location until it is cleared.
- `set_memory_bit`: Sets a specific bit in a byte at a memory location and polls a bit in another memory location until it is cleared.
- `write_encoded_byte`: Calculates an encoded byte from an input and writes it to a memory location while waiting for a bit flag.
- `poll_memory_bits`: Polls a memory location by checking specific bits in a loop until they are cleared.
- `modify_byte_and_poll`: Performs bitwise operations to set and clear specific bits in a byte and then waits for a bit in another memory location to be cleared.
- `modify_memory_bits`: Performs multiple bitwise operations on a byte at a memory location and polls a bit in another memory location until it is cleared.
- `decrement_register_loop`: Decrements a 16-bit register in a loop until it reaches a specific value.
- `clear_memory_and_copy_data`: Fills a memory range with zeros and then conditionally copies a block of data from the current instruction location to a destination buffer.
- `clear_memory_and_jump`: Fills a block of memory with zeros using a loop and then executes code at a fixed address.

### Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:wargame

- `initialize_system_resources`: Clears a block of memory and performs a series of checks to load essential system libraries and resource pointers.
- `call_function_in_loop`: Iterates through a loop, calling a function via a pointer with varying arguments derived from a table and registers.
- `parse_numeric_string_2`: Parses a numeric string from a character buffer, handling negative signs and converting characters to integer values.
- `execute_indirect_call_8`: Calculates an offset based on a 16-bit integer and calls a routine at that calculated address.
- `frag_lea_moveq_rts`: Short code fragment that loads the address of a location in the code into a0, then loads 96 into d2, then returns.
- `load_address_and_return`: Loads a constant address into a0, sets d2 to 90, and returns.
- `load_address_and_return_2`: Loads a constant address into a0, sets d2 to 72, and returns.
- `return_without_action`: The routine consists of three instructions that perform no work and immediately return to the caller.
- `load_address_and_return_3`: Loads a constant address into a0, sets d2 to 15, and returns.
- `return_without_effect_3`: The routine as decompiled is empty, but the binary holds three instructions at its entry, which the pseudo-code does not show.
- `load_address_and_return_4`: Loads a constant address into a0, sets d2 to 30, and returns.
- `frag_lea_moveq_rts_2`: Short code fragment that loads the address of a location in the code into a0, then loads 70 into d2, then returns.
- `frag_lea_moveq_rts_3`: Short code fragment that loads the address of a location in the code into a0, then loads 92 into d2, then returns.
- `process_entity_data`: Loads a value from a table, performs arithmetic to calculate offsets and values for a structure in a3, calls calculate_blitter_buffer_offsets, and calls two other routines via a pointer in a6 to conditionally branch to update_entity_status_and_coordinates or process_game_state_data.
- `update_entity_state_2`: Iterates through a list of entities to check status flags and update their internal data.
- `update_command_state_variables`: Checks a status flag and iterates through a buffer of command bytes to modify internal state variables.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work, so it does nothing when called.
- `process_command_sequence`: Iterates through a buffer of command bytes to update internal state variables and numeric fields.
- `process_game_state_data`: Iterates through a data structure to check specific flag bits and byte values.
- `calculate_bit_rotated_quotient`: Performs a bitwise rotation on a byte from a global variable and calculates a quotient and remainder based on that value and an input parameter.
- `initialize_data_structures_2`: Sets several memory addresses based on an offset, populates a structure with specific values, and performs a loop to increment values in a memory block.
- `increment_and_calculate_product`: Increments a 16-bit value stored at a location relative to a pointer and returns a calculated product based on a loop.
- `increment_memory_value`: Increments a 16-bit value stored at a location relative to a pointer and returns a calculated product.
- `initialize_blitter_data`: Populates a series of memory locations with specific values, performs a loop to offset data, and configures blitter registers.
- `execute_indirect_calls`: Retrieves function pointers from a table via an offset and executes them sequentially.
- `update_entity_status_and_coordinates`: Iterates through a global table of data structures to modify specific numeric fields and bit-flags based on matching character attributes.
- `update_entity_state_table_2_2`: Iterates through a global table of data structures to modify specific numeric fields based on matching character flags.
- `update_entity_state_table_2`: Iterates through a global table of data structures to modify specific numeric fields based on matching character flags.
- `update_entity_state_table_3`: Iterates through a linked list of entity structures to update status flags, counters, and coordinate values based on specific condition checks.
- `search_for_non_zero_value`: Iterates through a table of integers to find a non-zero value and then checks for a negative value.
- `clear_string_content`: Iterates through a null-terminated string starting at an offset from a pointer and replaces each character with a space.
- `scan_unit_list_for_best_candidate`: Walks a null-terminated linked list of unit records, skipping entries whose owner byte differs from the current side and entries flagged as inactive, and keeps the candidate with the lowest priority value or the highest counter in the scanned record.
- `decrement_and_validate_entity_counters`: Iterates through a linked list of data structures to decrement specific counter fields.
- `update_entity_state_table`: Iterates through a table of data structures to modify specific fields based on conditional checks.
- `decrement_and_validate_list_nodes_2`: Decrements counter values within specific structures and performs conditional comparisons against other structure members.
- `decrement_and_validate_list_nodes`: Decrements a counter at a specific offset within a structure and performs conditional comparisons against other structure members.
- `process_entity_attribute_table`: Iterates through a table of pointers to update specific byte and word values based on matching criteria.
- `process_event_state_and_timers`: Updates event structure properties and decrements internal timer values based on a status byte.
- `update_event_timers`: Iterates through a linked list of event structures to decrement their internal timer values and update their properties.
- `process_event_timers`: Iterates through a linked list of event structures to decrement their internal timers and update their properties.
- `update_event_list_and_decrement_timers`: Manages a linked list of event structures by inserting new elements and updating their internal data.
- `decrement_counters_and_execute_callbacks`: Decrements two separate counters, ensuring the second does not fall below zero by subtracting a value derived from a third counter, and then executes a sequence of indirect function calls via pointers stored in a table.
- `update_linked_list_node_3`: Updates a linked list by reassigning pointers and copying data fields from one node to another.
- `decrement_counter_and_execute_callbacks`: Decrements two different counters, ensuring one does not fall below zero, and then executes a series of functions via indirect pointers from a jump table.
- `execute_jump_table_2`: Sets a byte in memory and then executes a series of functions retrieved from a jump table via an indirect pointer within a loop.
- `increment_and_wrap_counter`: Increments a word value stored at a memory location pointed to by a register.
- `execute_indexed_function`: Calculates an offset by multiplying a word value by a constant and calls the resulting address from a table.
- `return_immediately_11`: This routine performs an immediate return without executing any instructions.
- `update_bounded_values`: Compares two bytes in a structure and increments a third word-sized value while adjusting one of the bytes, ensuring the word-sized value stays within a specific range.
- `prepare_blitter_graphics_data`: Calculates memory offsets for several data buffers and populates a table with Blitter parameters.
- `shift_memory_left`: Shifts a block of seven bytes left by one position within a structure.
- `shift_memory_left_2`: Shifts a block of seven bytes left by one position starting from an offset in a structure.
- `iterate_and_update_block_metadata`: Iterates through a sequence of blocks using a counter, updating specific global values and calling a routine to recalculate coordinate and status bit-fields for each entry.
- `update_coordinate_and_status_buffer`: Calculates new coordinate values by adding offsets to a base structure and packs these values into a bit-field.
- `nop_padding`: This routine is empty and contains no executable instructions.
- `copy_and_update_state`: Copies a block of memory to a global buffer and then iterates through a data structure to modify internal state variables based on specific byte values.
- `convert_to_string`: Converts a 16-bit integer into a decimal string representation stored in a buffer, handling leading zeros by replacing them with spaces.
- `update_entity_state`: Reads data from a table to modify internal state variables based on specific byte values.
- `update_entity_state_from_data`: Reads a sequence of command bytes from a data buffer to modify internal state variables.
- `iterate_and_execute_jump_table`: Decrements a counter in a loop while executing a function selected from a jump table via an indirect pointer.
- `frag_move_subq_mulu_adda_rts`: Short code fragment that copies d6 to d5, then subtracts 1 from d5, then multiplies d5 by 72 (unsigned), then adds d5 to a2, then returns.
- `execute_jump_table`: Performs an indirect jump to a function selected from a table based on a provided index.
- `update_joystick_state_2`: Reads the joystick port from a hardware register, compares values against a byte in a structure, and calls update_entity_state_from_data or process_joystick_movement based on the input and specific byte comparisons.
- `process_joystick_movement`: Updates a coordinate structure by applying directional offsets derived from a global variable and clamps the resulting values within defined minimum and maximum boundaries.
- `update_status_flags`: Calls two external functions to update specific word values in a structure and then sets a high bit in a byte at a fixed offset.
- `calculate_blitter_buffer_offsets`: Calculates a series of memory offsets based on a multiplier and bit-swapped values, then stores these addresses into a table.
- `prepare_blitter_area_data`: Calculates a set of four destination memory addresses based on input values and bit-swapping logic.
- `parse_iff_resource_chunks`: Parses an IFF data structure to identify and extract specific tagged chunks.
- `pop_data_from_stack`: This routine performs a series of pointer dereferences and increments a pointer to extract multiple words and longwords from a stack-like structure into global variables.
- `load_data_to_ram`: Extracts a series of values from a pointer list and populates a set of global RAM addresses with the retrieved data.
- `extract_buffer_data`: Iterates through a list of pointers to copy multiple values from various offsets into a set of global variables.
- `setup_blt_registers`: Configures several hardware registers associated with the Blitter and performs library calls.
- `check_byte_condition`: Reads a byte from an offset in a structure and calls another routine if the value is non-negative.
- `update_structure_fields`: Reads a character, stores it in a structure, and if non-negative, updates several other fields within that structure using library calls.
- `update_structure_fields_2`: Writes a constant byte, a four-byte value, and a two-byte value into specific offsets of a structure pointed to by a register.
- `return_register_value`: This routine returns the current value held in a data register.
- `return_undefined_byte`: This routine is empty and returns an undefined byte value.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.

### 9

- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 100 into d0, then returns.

### diskfont.library

- `set_library_handle`: As library code, it checks a value and, if non-zero, stores a provided pointer into a structure and calls another routine.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: As library code, it attempts to open libraries and, if successful, initializes a structure with a self-referencing pointer and zeroed fields.
- `remove_linked_list_nodes`: This library routine iterates through a linked list and removes nodes that meet specific criteria by updating pointers and decrementing a counter.
- `insert_into_linked_list_4`: As part of a library, this routine inserts a value into a linked list by updating pointers and sets a status bit in a structure if certain conditions are not met.
- `set_high_bit`: As library code, it sets the most significant bit of a byte at a specific offset within a structure.
- `return_no_op_handler`: This is a single-instruction library routine that does nothing: its entire body is a bare return, with no register setup, library calls, or hardware access.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_call_callback`: Decrements a short value at a specific offset in a structure and, if it reaches zero and a bit is set, calls a function pointer stored in that structure.
- `tiny_moveq_rts_5`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `compute_scaled_offset`: Library code that computes an integer from two structures using subtraction, scaling, and a loop that subtracts values from a global table based on byte comparisons, then returns the result.
- `load_and_install_disk_font`: Opens a font file from disk, reads its header, and allocates memory for the font data.
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
- `skip_to_string_terminator`: Library code that increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `compare_string_sequences_3`: Library code that compares two null-terminated strings byte by byte and returns a value indicating their relative order.
- `compare_string_sequences`: This library routine compares two consecutive bytes in a string and returns a value based on whether the bytes are equal or if one is less than the other.
- `compare_string_sequences_2`: Library code that compares two strings byte by byte and returns a value indicating equality or the relative order of the bytes.
- `tiny_moveq_rts_6`: Small routine of 2 instructions that loads 1 into d0, then returns.
- `search_string_for_byte`: Iterates through a null-terminated string to find the first occurrence of a specific byte.
- `find_byte_in_string`: This library routine iterates through a null-terminated string to find the first occurrence of a specific byte, returning the pointer to that location or zero if not found.
- `find_byte_in_string_2`: Library code that searches for a specific byte within a null-terminated string and returns a pointer to its location or zero if not found.
- `find_last_occurrence_in_string`: The routine receives a pointer to a zero-terminated string and a byte value, determines the string length via a library call, then walks backwards from the end to find the last occurrence of the byte and returns its pointer.
- `advance_past_delimiters`: This library routine searches a string for specific delimiter characters and returns a pointer advanced past the first found delimiter.
- `populate_glyph_metadata_table`: Iterates through a linked list of font data structures to extract string identifiers and glyph properties into a destination table.
- `populate_font_string_buffer`: Iterates through a linked list of data structures to calculate string lengths and copy character data into a destination buffer.
- `parse_font_data_stream`: Iterates through a linked list of data structures to extract and copy character strings into a destination buffer.
- `parse_font_glyph_metadata_2`: Iterates through a data stream to extract font glyph properties and string identifiers into a structured table.
- `parse_font_resource_stream`: Iterates through a data stream to identify and extract font glyph metadata and pixel data into a structured table.
- `populate_font_glyph_table`: Iterates through a parsed data stream to extract glyph metadata and pixel data into a structured table.
- `parse_font_resource_data`: Iterates through a data stream to identify specific font resource headers and metadata.
- `parse_font_glyph_metadata`: Iterates through a data stream to extract font glyph properties and string identifiers into a structured table.
- `clear_d0_and_restore_registers`: Sets register d0 to zero, restores registers d2 through d6 and a2 through a4 from a stack frame relative to a6, unlocks a6, and returns.
- `parse_disk_font_file`: Reads a font file from disk and parses its contents to build an in-memory font table.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `frag_movem_jsr_addq_rts`: Short code fragment that saves registers on the stack, then calls parse_disk_font_file, then adds 8 to a7, then returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:strt2

- `initialize_global_memory`: Clears a block of memory with zeros and initializes several global pointers and values based on system library results.
- `initialize_game_assets_and_engine`: Opens and parses data files to populate internal tables, initializes audio hardware, and sets up a core execution structure.
- `parse_and_interleave_data`: Opens a file, reads data, and populates a destination table by splitting bytes into high and low nibbles stored in separate fields.
- `reset_audio_hardware`: The routine clears several audio length and location registers and modifies the DMA control register.
- `call_and_set_flag`: The routine calls another routine and then stores a value into a memory location.
- `reset_audio_and_set_flag_2`: This routine resets the audio hardware and sets a byte at a specific offset relative to a register value to one.
- `copy_data_from_memory`: Copies a sequence of bytes from a source memory location to a destination buffer using a loop.
- `allocate_and_initialize_string_buffer`: Calculates a required buffer size based on a pointer and length, then allocates a new memory structure.
- `process_string_and_update_list`: Compares two strings and, if they match, traverses a linked list to count elements and allocates a new structure to store the results.
- `parse_and_load_file_metadata`: Compares a string against a file name and, if a match is found, traverses a linked list of pointers to populate a new structure with file attributes and metadata.
- `execute_indirect_callback_chain`: This routine performs a sequence of indirect calls by traversing nested pointers and executing a series of callback functions stored in a table.
- `copy_register_value`: Copies the contents of one register into another register.
- `check_flag_and_call`: This routine checks if a byte at a specific offset relative to a register is non-zero and calls another routine if it is.
- `store_library_results`: Retrieves two values from a global structure and stores them into a destination pointed to by a register.
- `push_parameters_to_stack`: This routine copies four passed parameters onto the stack before calling another routine.
- `push_multiple_parameters_to_stack`: This routine copies twelve passed parameters onto the stack before calling another routine.
- `parse_string_and_interleave_bits`: Parses a format string and then calls a routine to interleave the bits of two words.
- `populate_signed_data_arrays`: This routine executes two consecutive calls to process signed values and populate arrays using data from a table.
- `populate_signed_array`: Determines the sign of a value, stores a sign flag, and populates an array with bytes from a table based on a loop.
- `process_signed_value_and_populate_array`: Determines the sign of a value from a structure, stores a sign flag, and enters a loop that populates an array by fetching bytes from a table using values returned by another routine.
- `parse_formatted_string`: Processes a format string to extract and convert various data types including integers, floating point numbers, and strings.
- `chain_result_to_output_2`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `call_dos_output`: Performs an indirect call to a function located via a pointer in a table.
- `execute_indirect_function_pointer_3`: Retrieves a pointer from a table located at an offset from a base address and executes the function at that location.
- `execute_indirect_function_pointer_4`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `copy_memory_block_6`: Copies a block of memory from a source to a destination, handling both forward and backward copies depending on the direction of the pointers.
- `return_from_subroutine_6`: This routine performs a return from a subroutine.
- `parse_quoted_string_tokens`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `copy_string_with_limit_4`: Copies bytes from a source pointer to a destination pointer until a null terminator is reached or a maximum length is hit.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values_2`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `load_window_data`: Retrieves data from a nested structure and populates a destination structure with values from a table.
- `signed_divide_longwords_2`: Performs a signed division by calling an unnamed unsigned routine and negating the result when the operand signs differ.
- `perform_signed_division`: Performs a signed 32-bit division by calling a division routine and negating the result if the dividend is negative.
- `concatenate_division_result`: Calls a 32-bit signed division routine and returns a 64-bit value by concatenating the original contents of a register with the result left in that register by the call.
- `recursive_call_2`: The routine calls another routine.
- `divide_longwords_2`: Performs signed division on 32-bit values using a bit-shifting algorithm.
- `swap_and_rotate_words_2`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `call_interleave_bits`: The routine calls another routine that interleaves the bits of two words pointed to by a register.
- `interleave_and_update_list_data`: This routine processes a data structure by traversing a list to update internal pointers and status fields.
- `return_no_op`: The routine consists of a single instruction that returns immediately, performing no work.
- `execute_subroutines`: This routine calls two other routines that handle bit interleaving and byte storage.
- `pop_and_store_byte`: Increments a pointer stored in a global variable and stores a byte from an offset of another pointer into the location previously pointed to by that global variable.
- `traverse_code_block`: Iterates through a sequence of instructions or data by incrementing a pointer by a fixed offset.
- `traverse_and_clear_linked_structure`: Recursively traverses a linked data structure based on bit flags in a control byte, performing bitwise OR operations on return values.
- `clear_recursive_structure`: This routine recursively traverses a data structure based on bit flags in a control byte, performs bitwise OR operations on returned values, and clears the structure's fields and flags.
- `process_recursive_data`: Recursively processes a data structure by comparing values and updating buffer pointers, status flags, and byte contents.
- `process_recursive_data_buffer`: Recursively processes a data structure by comparing values and updating buffer pointers, status flags, and byte contents.
- `process_data_stream_2`: Manages a data stream by resetting pointers and status flags or by copying a byte from a source to a destination and updating a pointer.
- `process_data_stream`: Manages a data stream by either updating a pointer and status flags or performing a recursive call to handle specific data conditions.
- `update_table_and_store_byte`: Updates a table with a sum of two values, sets a bit in a status byte, increments a pointer, and stores a byte from a source into the previous pointer location.
- `search_and_clear_table_2`: Iterates through a table of structures to find the first entry where a specific byte is zero, then clears three longwords in that entry.
- `search_and_clear_table`: Iterates through a table of 22-byte structures, checking a byte at an offset, and clears three longwords in the first entry that meets a zero condition.
- `clear_three_longwords`: Clears three consecutive longwords in a table pointed to by a register.
- `update_status_flags_2`: Updates a structure with specific status values and pointers based on the result of another routine.
- `set_structure_fields_2`: Sets specific fields within a structure and updates a value from a memory location based on a conditional check.
- `traverse_linked_list_6`: Iterates through a linked list by following pointers and calls another routine for each element until a null pointer is reached.
- `traverse_linked_list_4`: Traverses a linked list by following pointers and calls another routine for each element until a null pointer is reached.
- `insert_into_linked_list_3`: Inserts a new element into a linked list by updating pointers and copying data from a source.
- `copy_pointer_data`: Copies two longwords from a memory location to a destination and returns a pointer offset by eight bytes.
- `forward_parameter_field_2`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `traverse_linked_list_7`: Traverses a linked list to find a specific node and updates a pointer within the list structure.
- `unlink_matching_list_node`: Walks a singly linked list starting from the list head pointer held in an address register, comparing each node's address against a target derived from a caller-supplied structure.
- `store_word_value`: Stores a word from one location into another location or a global variable.
- `store_and_call`: Stores a value from a register into a memory location and then calls another routine.
- `check_table_entry`: Validates an index against a table boundary and returns a boolean result based on whether a specific entry is non-zero.
- `check_interactive_status`: Retrieves a value from a table using an offset and returns a boolean result based on whether that value is non-zero.
- `return_zero_long`: Returns a longword value of zero.
- `handle_io_result`: Checks a return value and, if zero, retrieves a shortword from another routine to store in a global variable before returning a longword.
- `return_zero_short`: Returns a shortword value of zero.
- `execute_conditional_logic_4`: Performs conditional calls and validates bit flags based on the return values of other routines.
- `execute_conditional_call`: Checks a bitmask in a structure and either calls another routine or updates a global status variable.
- `execute_indirect_call_9`: Calls a function using parameters from a structure and, if the result is negative one, stores a value from another routine into a global variable.
- `return_no_op_stub`: The routine body is a bare return with no operations, so it performs no work of its own.
- `validate_bit_flag`: Calls another routine and checks if a specific bit is set in the return value before potentially calling a third routine.
- `forward_call_result`: Calls another routine and passes its return value to a function that handles output formatting and writing.
- `execute_callback_and_routine`: Calls a function pointer stored in a structure and then calls another routine.
- `traverse_resource_tree`: Recursively processes a hierarchical data structure by checking various pointers and offsets within a control block.
- `discard_stack_and_return`: Moves a longword from the stack into d4, unlocks a5, and returns.
- `access_table_entry`: Validates an index against a table size, executes a function pointer from the table if the entry is valid, and clears the entry.
- `execute_indirect_call_offset_36`: Calls a function pointer retrieved from a table using a fixed offset.
- `execute_indirect_function_3`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `execute_indirect_function_pointer_5`: Fetches a function pointer from a nested table structure and executes it.
- `execute_nested_pointer_function`: Retrieves a function pointer by traversing two levels of offsets from a base address stored in a register and executes it.
- `execute_indirect_function_4`: Retrieves a pointer from a nested table structure and executes the function located at that address.
- `call_delay_2`: Calls a delay function from a library.
- `execute_indirect_function_pointer_6`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_delete_file_2`: Calls a function to delete a file from a library.
- `call_dup_lock_2`: Calls a function to duplicate a file lock from a library.
- `call_execute_2`: Calls an execution function from a library.
- `call_input_2`: Calls an input function from a library.
- `call_io_error`: Calls an input/output error function from a library.
- `call_is_interactive_2`: Calls a function to check for interactive status from a library.
- `open_file_via_pointer`: Opens a file by calling a function through a pointer retrieved from a structure.
- `call_open_3`: Calls a function to open a file from a library.
- `call_read_3`: Calls a function to read data from a library.
- `call_seek_2`: Calls a function to seek within a file from a library.
- `unlock_dos_file`: Calls a function to unlock a file via the operating system.
- `call_library_function_2`: Performs an indirect call to a function within a library using an offset from a table.
- `call_library_function_3`: Executes a function located at a specific offset within a library structure.
- `call_library_function_4`: Invokes a library routine by dereferencing a pointer from a table.
- `call_library_function_5`: Calls a routine through a pointer stored in a library table.
- `call_library_offset`: Executes a library function using an offset from a base pointer.
- `call_library_function_6`: Uses a table offset to call a specific library function.
- `execute_library_function`: Performs an indirect jump to a function located at a specific offset within a library.
- `call_library_function_7`: Performs an indirect jump to a function within a library.
- `invoke_library_routine`: Calls a routine from a library using a pointer stored at a specific offset.
- `call_library_function_8`: Calls a library routine using an offset from a base pointer.
- `call_library_function_9`: Executes a function by accessing a library table via an offset.
- `execute_indirect_library_function`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_library_function_10`: Calls a function located at a specific offset in a library structure.
- `call_indirect_function_20`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `execute_nested_pointer`: Retrieves and executes a function pointer from a nested offset within a structure.
- `call_indirect_function_21`: Executes a function located at an offset within a table structure.
- `call_indirect_function_22`: Calls a subroutine by accessing a pointer through a base address and a specific offset.
- `call_indirect_function_via_offset`: Retrieves a function pointer from a nested structure within a global table and executes it.
- `call_indirect_function_23`: Retrieves a function address from a table and transfers control to it.
- `call_indirect_function_24`: Performs an indirect jump to a routine found via a table offset.
- `call_indirect_function_25`: Calls a routine by dereferencing a pointer stored in a table.
- `call_indirect_function_26`: Executes a function pointer obtained from a table structure.
- `call_indirect_function_27`: Transfers control to a function address calculated from a table and an offset.
- `call_indirect_function_via_offset_2`: Retrieves a function pointer from a nested offset within a structure pointed to by a register and executes it.
- `call_indirect_function_28`: Retrieves a function pointer from a nested offset within a structure pointed to by a register and executes it.
- `execute_indirect_call_4`: Performs an indirect call to a function pointer retrieved from a table using a different offset.
- `execute_indirect_call_5`: Performs an indirect call to a function pointer retrieved from a table using a third offset.

### Iron Lord (1989)(UBI Soft)[cr](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_stub_3`: The routine consists of a single return instruction and performs no work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `mask_memory_values`: Performs bitwise AND operations on two memory locations using a constant mask.
- `empty_routine_3`: This routine contains no executable instructions.

### Iron Lord (1989)(UBI Soft)[cr](Disk 1 of 2).adf:ILwxzxw (raw code file, load address unknown)

- `configure_dma_hardware`: A large routine that accesses the ADKCON and DMACON hardware registers.

### Iron Lord (1989)(UBI Soft)[cr](Disk 2 of 2).adf:ILwxzxw (raw code file, load address unknown)

- `configure_dma_transfer`: This routine modifies the DMA controller and address configuration registers.
