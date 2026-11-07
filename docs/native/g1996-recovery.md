# Game 1996 (operation stealth (1990)(u. s. gold)(de)[cr trsi]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01996). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2707184 top-level bytes in 44 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 524 of 524 named (0 author symbols), 524 of 524 described (47 by rule, 462 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2136 of 2136 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 22282 of 22282 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1813221 |
| code | 77256 |
| data | 795829 |
| padding | 10126 |
| unused | 10752 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| TRSI | 7 | 29 | 2 |
| Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 1 of 3).adf:endcli | 2 | 117 | 2 |
| Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 1 of 3).adf:delphine | 3 | 20104 | 428 |
| Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 1 of 3).adf:add21k | 3 | 637 | 38 |
| Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 2 of 3).adf:TRSI-Intro | 5 | 231 | 6 |
| TRSI | 3 | 57 | 2 |
| TRSI | 7 | 29 | 2 |
| Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 2 of 3).adf:bootblock | 1 | 126 | 6 |
| Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 3 of 3).adf:bootblock | 1 | 126 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 964 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |

## Cases handled

- operation stealth (1990)(u. s. gold)(de)[cr trsi](disk 3 of 3)/Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 3 of 3).adf: file SD02 (header 537112 bytes, blocks hold 299144): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 464 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- operation stealth (1990)(u. s. gold)(de)[cr trsi](disk 3 of 3)/Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 3 of 3).adf: blocks 2-659 (bytes 1024-337920) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.10)
- operation stealth (1990)(u. s. gold)(de)[cr trsi](disk 3 of 3)/Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 3 of 3).adf: blocks 665-670 (bytes 340480-343552) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.50)
- operation stealth (1990)(u. s. gold)(de)[cr trsi](disk 3 of 3)/Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 3 of 3).adf: blocks 1070-1077 (bytes 547840-551936) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.86)
- operation stealth (1990)(u. s. gold)(de)[cr trsi](disk 3 of 3)/Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 3 of 3).adf: blocks 1331-1759 (bytes 681472-901120) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.12)
- operation stealth (1990)(u. s. gold)(de)[cr trsi](disk 2 of 3)/Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 2 of 3).adf:TRSI-Intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 2 of 3).adf:TRSI-Intro

## Copy protection found

- `initialize_jump_table_and_copy_data` (runtime segment an address): Populates a jump table by calculating relative offsets and then performs a series of block memory copies and pointer updates using a loop. Kind: jump table and code relocation/initialization. Pass path: the loop that populates the jump table (the first while loop) and the subsequent data copying (the do-while loops) or the branch where bVar25 remains false..

## Functions


### TRSI

- `initialize_and_free_memory`: The routine serves as the program entry point.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 1 of 3).adf:endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: copies a fixed number of longwords from a source address to a destination address.

### Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 1 of 3).adf:delphine

- `clear_memory_and_check_library`: Clears a block of memory and checks a library-retrieved value to decide whether to execute another routine or return.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_routine`: the routine is empty and performs no operations before returning.
- `set_library_parameters`: Writes a value from a library function or a global pointer to a memory location, and clears or sets other memory locations with specific constants.
- `validate_and_process_event_state`: Checks a return value and certain global flags to decide whether to execute a sequence of recursive or conditional routine calls.
- `pass_through_routine`: This routine calls another routine with the same arguments and then returns.
- `read_input_flags`: Returns a byte derived from a global variable, testing bits 64 and 4 and combining the results with a logical OR.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `set_global_value`: Sets a global variable to a constant value.
- `transform_and_split_value`: Updates a global variable with a transformed value and returns a result split into high and low words based on a divisor.
- `handle_keyboard_serial`: Returns a byte computed from the bitwise complement of a global variable when a flag bit is unable to be cleared, performs a decrementing delay loop, and clears a flag bit in the global variable.
- `lookup_table_value`: Compares a value against a table and, if a match is found, retrieves a corresponding value from another table.
- `copy_and_update_table`: Copies a block of data from a table into a destination buffer and then writes several or even a loop-based set of values derived from input parameters into that buffer.
- `initialize_memory_block`: Writes a series of specific constant values to a block of memory pointed to by a global pointer.
- `initialize_memory_values`: Writes specific constant values to a memory location derived from a global pointer and calls library functions.
- `get_pointer_from_memory`: Returns a pointer or value stored at a specific offset from a global memory location.
- `update_buffer_with_raster_sync`: Updates a buffer with values derived from input parameters, using a routine that waits for a specific raster line or pointer access.
- `copy_source_to_destination`: Copies a sequence of words from a source buffer into a destination buffer.
- `store_parameter_block`: Stores a large number of passed parameters or partial register values into a block of memory pointed to by a global pointer.
- `return_without_action`: The routine is a minimal stub that performs no visible work beyond returning to its caller, as shown by the decompiler output.
- `return_immediately`: This routine is empty and performs no operations before returning.
- `execute_unreachable_copy`: An unreachable routine that calls a routine with specific constants and then returns.
- `execute_nested_calls`: Performs a sequence of calls to another routine using values passed or stored on the stack.
- `get_and_increment_index`: Reads a byte from a table using an index, then increments that index stored in a pointer.
- `clear_data_table`: Clears a block of memory by zeroing out several fields in a repeating loop, then conditionally executes a sequence of recursive calls and pointer-based updates.
- `execute_conditional_logic`: Calls a routine with a specific error code and, if successful, executes further routines and returns a value from a memory offset.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `get_info_and_return_value`: Calls a routine to check a status and, if successful, executes further routines to retrieve a value from a memory offset.
- `traverse_linked_list_and_compare_2`: A routine that performs string copying, negation, and linked list traversal while checking device status.
- `tiny_neg_move`: Small routine of 2 instructions that negates d6, then copies d0 to d0.
- `check_input_flags`: Reads a byte from a global variable and returns a value containing the logical OR of bits 64 and 4.
- `initialize_game_state_tables`: Initializes several global memory tables and buffers by zeroing out or setting constant values.
- `perform_no_operation`: The routine is empty and performs no actions before returning.
- `update_buffer_with_sync`: Swaps two values, then copies a sequence of words or scaled indices into a destination buffer based on a condition.
- `set_memory_via_pointer`: Retrieves a pointer from a table using an offset from a global variable and writes a constant value to an offset of that pointer.
- `process_function_table`: Iterates through a table, calling another routine with arguments derived from the table, and clears table entries.
- `insert_product_record`: Searches a table for the first empty entry and populates it with a calculated product, a bit-shifted value, and several constant or passed parameters.
- `update_product_table`: Calculates a product of two short values and stores the result and several other pieces or bit-shifted values into a destination table.
- `update_table_with_product`: Calculates a product of two short values and stores the result and several other pieces or bit-shifted values into a destination table, while incrementing a global counter.
- `insert_table_entry`: Calculates a table offset using a product of two parameters, then writes a pointer and several related values into that table if the target pointer is non-zero.
- `update_table_entry`: Copies several words or a bit-shifted word from a source structure to a destination table, then increments a global counter.
- `traverse_and_clear_tree_node`: Recursively traverses a linked structure by calculating a new pointer from a table-based product, then clears the current node's data fields and resets certain values before proceeding to the next node.
- `update_recursive_structure`: Calculates a product of two values from a table and stores it in a pointer-based location, then recursively calls another routine while clearing several adjacent memory locations and decrementing a global counter.
- `loop_until_limit_2`: Increments a register and calls another routine in a loop until a limit from a table is reached.
- `loop_until_limit`: Increments a register and compares it against a value stored in a table until a certain condition is met.
- `clear_global_word`: clears two consecutive dwords in a global memory area to zero.
- `traverse_and_clear_list`: Traverses a linked list of pointers, calling a routine that processes a block of memory, and clears a global area.
- `insert_into_linked_list_2`: Traverses a linked list to find a certain node, then inserts a new node containing data copied from a source table and parameters into that.
- `update_object_state_on_mismatch`: Checks if a pointer or specific short-integer fields in a structure have changed, and if so, swaps the pointer or value using a pointer-swapping routine while updating associated metadata or offsets.
- `swap_pointer_reference`: Swaps a pointer or value between two memory locations and copies certain short values to an offset within a referenced structure.
- `search_and_replace_node`: Searches a linked list for a node matching certain criteria and replaces it with its successor if found.
- `update_linked_list_with_priority`: Iterates through a linked list of structures to remove or update nodes based on a comparison of a priority-like value stored in a table.
- `clear_two_words`: Clears two 32-bit words in a global memory area by setting them to zero.
- `traverse_and_call_list`: Iterates through a linked list and calls another routine for each node encountered.
- `insert_into_linked_list`: Traverses a linked list to find a certain node, then inserts a new node containing data copied from a source table and parameters into that list.
- `copy_structure_data`: Copies a series of words from a source table (indexed by a short value) to a destination structure, and swaps a pointer or value in a linked list.
- `clear_one_word`: Clears a 32-bit word in a global memory area by setting it to zero.
- `traverse_linked_list_2`: Iterates through a linked list of pointers, calling another routine for each node, and finally executes a routine without arguments.
- `initialize_structure_fields_2`: Traverses a linked list to find a certain node, then initializes a new node with several words from a source structure, clearing certain trailing fields to zero and swapping a pointer.
- `initialize_structure_fields`: Populates a destination structure with several words from a source structure, clearing certain trailing fields to zero and swapping a pointer.
- `clear_global_values`: clears a global word to zero and sets another global word to all ones.
- `traverse_linked_list_3`: iterates through a linked list of pointers, calling another routine for each node, and finally calls a routine without arguments.
- `prepend_linked_list_node`: Traverses a linked list to find the end, then prepends a new node containing a copied block of data and a new pointer to the head of the list.
- `search_and_set_node`: Traverses a linked list to find a node matching a specific value and sets a field in that node to all ones if found.
- `check_node_value`: Compares a value from a table with a value at an offset in a structure, setting that offset to all ones if they match, and returns a constant or all ones.
- `update_node_value`: Checks if a value at a specific offset in a structure matches a provided value, and if so, sets that value to all ones.
- `return_constant_value`: Returns a constant value of all bits set.
- `remove_nodes_by_value`: Iterates through a linked list and removes any node whose certain offset contains a certain value by updating the previous pointer.
- `traverse_linked_list_recursively`: Iterates through a linked list of pointers, recursively calling another routine for each node, and replaces certain list nodes with a pointer to the previous node if a specific condition is met.
- `reset_memory_values_alt`: Clears a 32-bit word to zero and sets a 16-bit word in a global memory area to all ones.
- `traverse_linked_list_4`: traverses a linked list of pointers, calling another routine for each element, and finally calls a routine without arguments.
- `execute_custom_bytecode`: Traverses a linked list to find the end, prepends a new node, initializes certain fields to zero, and then executes a routine using a pointer and a value from a table.
- `search_and_set_node_flag`: traverses a linked list of structures to find a node containing a specific short-integer value, then sets that value to all ones if a match is found.
- `compare_and_set_word`: Checks if a word at a specific offset in a structure matches a value from a table, and if so, sets that word to all ones.
- `compare_and_set_flag`: compares a word at a pointer-offset location with another word and, if equal, sets that word to all ones.
- `return_all_ones`: The routine returns a 32-bit value where all bits are set to one.
- `traverse_linked_list_recursively_2`: Iterates through a linked list, checking a specific member of each node for a certain value, and recursively calls another routine if a condition is met while updating pointers to bypass certain nodes.
- `traverse_linked_table_structure`: Recursively traverses a table-based linked structure by calculating offsets from a base pointer and checking a null-terminating or conditional pointer.
- `execute_indirect_calls_2`: Performs a sequence of indirect calls using values retrieved from a table or memory location.
- `update_coordinate_table`: Stores or calculates several 16-bit values into a global memory area using provided parameters and a fixed offset.
- `traverse_split_range`: Recursively traverses or splits a range of values, calculating new pointers and arguments based on a fixed offset and a limit, and calls another routine with these new parameters.
- `scan_and_increment_table`: Iterates through a range of memory, checks a bitmask, and increments a value in a table if a certain condition or value match is found.
- `compute_bitmask_from_table`: Computes an address from its parameters, reads four consecutive 16-bit words together with a high word from a register, tests four bit positions derived from the first parameter, builds a 16-bit mask from the results, and returns it combined with an input register.
- `test_bit_in_table`: Computes an index from the low word of the first parameter, a second parameter and the high word of the third parameter, then reads a 16-bit value from a table at that index.
- `compute_bitmask_large_stride`: Computes an address from its parameters, reads 16-bit words at large, fixed offsets, tests bit positions, and builds a mask returned with an input register.
- `initialize_game_state_and_buffers`: Clears or initializes a series of global memory buffers and resets certain state variables or pointers.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `empty_zero_word`: This routine is empty, containing only a single instruction that does nothing and returns immediately.
- `process_linked_list_nodes`: Traverses a linked list of nodes to check or update flags and update a global state or pointer-based table.
- `parse_entity_table_and_update_state`: Reads an entity or object count from a global variable and iterates through a table to check or update properties.
- `search_table_entry`: Iterates through a table to find an entry that matches certain provided values.
- `find_matching_collision_bounds`: Iterates through a linked list of structures to check if a given rectangular area or point intersects with a defined bounding box or collision zone.
- `traverse_linked_list_and_update_nodes`: Iterates through a linked list of structures and performs conditional recursive calls or updates based on a type or ID field.
- `traverse_linked_list_and_recurse`: Iterates through a linked list of structures, checking for specific type or ID matches.
- `traverse_linked_list_5`: Recursively traverses a linked list and calls another routine if a certain field in the current node does not equal a specific value.
- `traverse_linked_list`: Follows a chain of pointers through a list in memory until a null pointer is encountered.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `update_object_iterator_and_state`: Increments a counter or resets it to zero, then recursively traverses a linked list or table-based structure to update status flags and clear certain global or structure-based error/status indicators.
- `determine_return_value`: Checks specific offsets within a structure and returns a constant value based on those checks.
- `update_priority_list_and_table`: This routine updates a linked list of structures by comparing priority-like values and scans a memory range to increment values in a table based on a bitmask match.
- `update_boundary_status`: Updates two offsets in a structure with zero or certain status codes based on comparisons between a range and a size or position.
- `traverse_bytecode_list`: Iterates through a linked list of pointers and recursively calls another routine if a certain value in a structure offset is not negative.
- `traverse_bytecode_list_conditional`: Checks a bit in a byte at a specific offset within a structure before proceeding to recursively traverse a linked list of pointers.
- `parse_bytecode_instructions`: Iterates through a custom bytecode stream, decoding instruction opcodes and associated operands to call specific subroutines or update a data table.
- `parse_bytecode_instructions_2`: Initializes a memory buffer with a default value and then executes a custom instruction stream, using a switch-based dispatcher to modify a jump table or skip certain bytes based on the instruction opcode.
- `update_sprite_collision_and_position`: Calculates a new pointer or offset for a sprite or object and checks for bitwise collisions or overlaps.
- `return_parameter_unchanged`: The routine receives a single value in a register and returns it unchanged.
- `update_bitmask_and_pointer`: Calculates a new pointer and bitmask using bitwise shifts and logical OR operations on a sequence of memory values.
- `return_parameter_unchanged_2`: The routine receives a single value in a register and returns it unchanged without performing any operations or calling other routines.
- `apply_bitmask_to_buffer`: This routine performs a bitwise masking and logical OR operation on a range of memory locations, using a bitmask generated by bit-shifting and inverting a source value.
- `return_parameter_unchanged_3`: The routine receives a single value in a register and returns it unchanged, performing no operations on the data.
- `store_constant_values`: The routine stores several constant 32-bit values into a memory location relative to a register.
- `apply_bitmask_to_buffer_with_rotation`: Calculates a bitmask by rotating the bitwise negation of a buffer element and applies this mask to a target buffer using a bitwise AND. The routine also updates a global variable with the high-order bits of the mask and modifies a counter or pointer-related value based on the mask's properties.
- `apply_bitmask_to_buffer_2`: Iterates through a buffer and applies a bitwise AND mask to each word, while also updating a global variable with a bit-rotated or masked value.
- `pass_register_value`: the routine returns the value passed in a register without modification.
- `pass_parameter_block_arguments`: Passes a fixed set of arguments, including a constant or a register-derived value and a hardcoded limit, to a routine that stores these values into a memory block.
- `initialize_struct_members`: Sets or clears several fields in a structure located via a pointer, including setting certain values to a fixed constant or an offset or a parameter.
- `execute_four_parameter_blocks`: Executes a routine four times, passing a modified parameter or an offset or a constant value to a block of memory.
- `return_empty_routine`: The routine performs no operations and immediately returns to its caller.
- `set_bitplane_parameters`: Checks specific bits in a parameter to set certain values to all ones, then calls another routine with different bitplane or offset parameters.
- `calculate_blitter_area_parameters`: calculates a set of blitter-related parameters including a width or height and a bitmask, then recursively calls a routine to apply these parameters to four different memory-offset-based drawing or masking operations.
- `calculate_parameter_block_indices`: Calculates a set of parameters including a memory-offset-based address and a value derived from input parameters, then calls a routine to store these or partial register values into a memory block.
- `configure_blitter_params`: A routine that executes 76 instructions and calls a routine to store a block of parameters.
- `calculate_recursive_offsets`: Calculates a set of offset-based parameters and recursively calls a routine with incrementing memory or address offsets.
- `parse_string_parameters`: Parses a string to calculate or update a horizontal offset or window size, using character-based lookups in a table to determine pointer or bitmask updates.
- `parse_string_segments`: Calculates the length of a string and iterates through segments delimited by spaces, updating a global counter and checking string lengths.
- `calculate_max_string_segment_length`: The routine parses a string and calculates the maximum length of a segment, where segments are delimited by certain characters, using a table to increment a counter.
- `mask_table_words`: The routine iterates eight times over two pointers, reads 16-bit words from one pointer, rotates them according to a nibble from a third parameter, combines them with the inverse and stores the result masked with a 32-bit word from the other pointer.
- `unnamed_routine`: The routine contains 81 instructions and is called by 5 other routines.
- `find_or_create_node`: Traverses a linked list to find a node matching specific criteria, or allocates and initializes a new node if no matching node exists.
- `update_structure_node`: Checks a set of conditions on a structure or memory block and, if failed, initializes a new block with specific values and updates a pointer.
- `update_struct_members`: Updates several fields in a structure or memory block using values from a pointer and a global variable.
- `traverse_and_replace_node_2`: Traverses a linked list until a null pointer or a specific termination condition is met.
- `process_string_and_branch_parameters`: Iterates through a null-terminated string, checking a secondary lookup table for active flags, and executes a sequence of recursive calls with calculated pointer offsets and bitmask parameters based on the string's content and a global state variable.
- `parse_string_and_update_table_2`: Iterates through a string or a sequence of characters, checking against a lookup table and updating a global counter or pointer.
- `parse_display_command_stream`: Parses a stream of command bytes to calculate a maximum display width or height and then executes a sequence of blit-like or drawing operations.
- `render_sprite_tile_buffer`: Iterates through a buffer to check for specific byte values and, if a match is found, calculates and applies blitter-based drawing parameters or bitmasking to a target memory area.
- `render_sprite_tile_buffer_2`: Iterates through a source buffer to identify specific byte values and uses a lookup table to calculate destination addresses and bitmasks for blitting or drawing pixels.
- `update_blitter_parameter_blocks`: Iterates through a memory buffer to calculate and store blitter-related area parameters and bitmasks, using a lookup table to determine bitplane offsets and masking values.
- `initialize_and_map_string_indices`: The routine clears a table of certain-sized entries and then, if a certain condition is met while scanning a source string, copies that string to a destination buffer and stores an index in another table.
- `get_table_value`: Checks a value from a routine and returns a word from a table or a modified value based on a range check.
- `update_blitter_params`: Updates a value and executes a loop that calls a routine with a memory address and a constant value until a certain condition is met.
- `scale_and_clamp_fields`: The routine doubles several fields of two 32-bit parameters and then computes three derived values from the first parameter using nibble extraction and clamping logic with the second parameter, updating intermediate variables accordingly.
- `process_buffer_with_raster_sync`: Iterates through a buffer, performing a bitwise operation on each word, and then sets a global flag or value.
- `scale_buffer_indices`: Iterates through a loop that calculates or retrieves source pointers based on a counter, then writes results to a destination buffer.
- `update_buffer_from_table_2`: Iterates through a range and writes a value obtained from another routine into a destination buffer.
- `update_buffer_from_table`: Reads a certain number of words from a source table and writes them to a destination buffer.
- `rotate_or_copy_buffer`: Depending on a condition, this routine performs a left or right rotation of a sequence of words or copies a sequence of words from one memory location to another.
- `copy_buffer_backwards`: Copies a sequence of words from a source buffer to a destination buffer by decrementing a counter and writing or overwriting memory.
- `rotate_and_copy_buffer_elements`: If a condition in a table-referenced value is met, it performs a left-rotation of a sequence of words within a buffer, moving the first element to the end, and then calls itself recursively with a modified pointer.
- `copy_buffer_forwards`: Copies a sequence of words from a source buffer to a destination buffer by incrementing a counter.
- `copy_buffer_range`: Copies a sequence of words from one memory location to another using a range or limit from a table.
- `copy_buffer_contents`: Copies a sequence of words from a source buffer to a destination buffer, using a count or limit from a table.
- `update_state_and_clear_buffer_entries`: Updates a global variable using a returned value from a routine, then clears a sequence of memory-mapped or buffered entries using a calculated offset and a loop.
- `recursively_traverse_linked_list`: Checks if a pointer or flag in a table or memory location is valid and then recursively calls itself using pointers or data retrieved from a table.
- `copy_words_to_four_pointers`: The routine copies a block of words from a source pointer to four destination pointers that are spaced apart in the destination buffer.
- `find_string_match_limit`: Iterates through a string, converting characters to uppercase, and checks if a certain pattern or length limit is met or exceeded.
- `search_string_matches`: Iterates through a table to find a matching string or a sequence of strings that match a provided parameter, returning the index of the first failed match or a certain value.
- `find_string_sequence`: Searches a table for a specific value and then checks if a sequence of strings starting at a calculated offset matches a provided parameter, returning an index or a certain value.
- `search_table_with_retry`: Performs a search using a parameter and executes a sequence of calls or retries the search with an offset if a negative result is returned.
- `validate_pointer_and_clamp_value`: Checks the sign of a value retrieved from a pointer and, if certain conditions are met, calculates a new address or returns a maximum unsigned value.
- `resolve_pointer_value`: Retrieves a value from a pointer and stores it in a memory location.
- `return_max_unsigned`: Returns a value representing the maximum unsigned integer.
- `retrieve_and_clamp_value`: Retrieves a value from a pointer, updates a memory location, and clamps the result to a maximum value if it is negative.
- `return_all_ones_2`: returns a value where all bits are set to one.
- `return_all_ones_3`: Returns a 32-bit value consisting entirely of set bits.
- `update_table_with_offset_data`: Calculates an offset into a table using a parameter, then calls a routine to update a global variable with a value from that table and a constant, followed by another call to update a global variable with a different value from the same table entry.
- `copy_bytes_and_advance_source`: The routine copies a specified number of bytes from a source buffer whose address is stored in a global variable to a destination buffer, then updates the stored source pointer to the next byte after the copied data.
- `expand_nibble_mask`: The routine returns zero when the input is zero, otherwise it expands each nibble of the input into a three-bit pattern and packs the results back into a 16-bit value.
- `update_entity_state_and_buffer`: Checks a conditional pointer or value and, if valid, performs a recursive search, swaps certain global pointers, and updates a buffer or table with a sequence of values.
- `update_render_parameters_and_buffer`: Updates a set of global pointers and parameters, potentially swapping or updating values in a block of memory, and then populates a small buffer with values from a loop or another routine.
- `update_buffer_with_data`: Performs several pointer-based updates, copies a block of words to multiple destination pointers, and fills a buffer with values from a loop.
- `parse_string_and_update_table_3`: Parses a null-terminated or dot-terminated string to check for a specific character, then updates a memory table with a sequence of words or values based on a calculated offset or pointer.
- `update_coordinate_table_and_buffer`: Checks a table for a valid value, swaps certain global pointers, and then updates a coordinate or buffer table by iterating through a loop that writes values or results of other routines into a memory block.
- `execute_recursive_operations`: Calls another routine with a fixed value, and if successful, performs several recursive calls involving calculated or passed parameters.
- `unpack_bitstream_to_memory`: Reads a bitstream, extracting or shifting bits to reconstruct or decode data, and writes the resulting bytes back to a memory buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bits_across_words`: The routine shifts a 32-bit value right by one bit repeatedly for a given count, refilling from the next word in a pointer when the value becomes zero.
- `process_event_queue_and_update_state`: Iterates through a table of pending event or action pointers, executing any non-zero entries and clearing them after execution.
- `process_recursive_table_data`: Recursively traverses a table-based structure to perform conditional updates and pointer-based memory modifications.
- `process_event_queue_and_update_state_2`: Iterates through a list of pending event or parameter blocks, clearing them after dispatching, and then executes a conditional update or swap of certain global pointers and values based on a table lookup or a comparison against a hardcoded limit.
- `process_and_clear_parameter_table`: Iterates through a table to recursively call another routine with stored parameters, clearing or resetting those parameters or pointers after use.
- `process_conditional_callbacks`: Checks certain bytes in memory and, if non-zero, executes a series of recursive or parameterized routine calls.
- `store_zero_byte`: Writes a zero byte to the memory location pointed to by a0 and then returns.
- `clear_and_reset_function_tables`: Iterates through several tables of function pointers, calling each routine with specific arguments and then zeroing out the table entries.
- `reset_memory_block`: Clears or initializes a large range of memory or a table with zero or specific values using loops and direct assignments.
- `initialize_display_parameter_table_2`: Initializes a table of display parameters by copying a sequence of words from a source memory area and setting certain flags or size limits.
- `initialize_display_parameter_table`: Checks a flag in a data structure and, if certain conditions are met, initializes a sequence of memory or register values using a loop and a series of hardcoded offsets and lengths.
- `patch_attribute_table`: Updates a table of attribute or property values by applying a series of offsets and lengths.
- `reset_object_list_properties`: Iterates through linked lists of objects to reset or modify specific member offsets, including a loop that traverses a list to increment a counter and a second loop that applies property updates to a list of objects.
- `update_object_list_properties`: Iterates through multiple linked lists of objects to apply property modifications or updates to specific member offsets.
- `traverse_object_lists`: Iterates through multiple linked lists of objects to perform recursive updates or property modifications on specific member offsets.
- `process_object_linked_lists`: Iterates through multiple linked lists of objects, performing certain operations or updates on specific offsets within each object, and updates a counter or secondary value during the traversal.
- `traverse_object_linked_lists_2`: Recursively traverses multiple linked lists or object chains, updating a counter or a secondary value during the traversal.
- `traverse_object_linked_lists`: Iterates through multiple linked lists by following pointers, performing recursive calls or secondary traversals while incrementing a counter and resetting a pointer, and finally executes a loop that calls another routine a fixed number of times.
- `traverse_linked_lists_recursively`: Iterates through multiple linked lists, performing recursive calls or secondary traversals while incrementing a counter and resetting a pointer to a global address.
- `traverse_linked_list_6`: Traverses a linked list to execute a routine on each element, increments a counter, and resets a pointer to a global address.
- `traverse_linked_list_and_process`: This routine traverses a linked list to count its elements and then performs a second pass to call another routine on each element, followed by a loop that calls another routine a fixed number of times.
- `traverse_linked_list_and_increment`: Traverses a linked list by following pointers, updating a global pointer, and increments a counter in a loop until a certain limit is reached.
- `increment_pointer_loop`: Calls a function pointer from a memory location and increments a value in a global variable until it reaches a certain limit.
- `process_conditional_buffer_logic`: Checks a value in a buffer and either executes a loop to copy data or calls a routine to copy a range of data, returning a status or a constant.
- `execute_with_offset_data`: Calls a routine using a pointer from a global table and returns a constant value.
- `parse_display_commands`: Executes several routines to copy data into local buffers and calls a routine to process a command stream or check a return value.
- `execute_indirect_table_pointer`: Calls a routine using a pointer from a global table and then calls another routine via a function pointer.
- `clear_event_and_function_tables`: Iterates through several tables of function pointers and associated data, executing each routine if a valid pointer is found and subsequently clearing the pointer and its associated data.
- `clear_and_reset_function_tables_2`: Iterates through several tables of function pointers and associated data, executing each routine if a valid pointer is found and subsequently zeroing out the pointer and its associated parameters.
- `initialize_data_buffer_loop`: Initializes a memory buffer by setting a counter and then executing a routine repeatedly to clear or populate a range of memory.
- `process_function_table_and_copy_data`: Iterates through a table of function pointers to execute secondary routines, followed by a series of conditional checks and updates to a global variable.
- `execute_and_copy_data_table`: Iterates through a table of function pointers to execute secondary routines, followed by a series of conditional checks and updates to a global pointer.
- `initialize_data_table_and_copy_values`: Iterates through a pointer-based table to execute or validate secondary routines, then performs a series of conditional checks and updates a global variable.
- `process_event_table_and_copy_data`: Iterates through a pointer-based table to execute or validate secondary routines, then performs a series of conditional checks and updates a global buffer by copying a sequence of words from a source table.
- `traverse_and_update_event_tables`: Iterates through a series of pointer-based tables or linked structures to execute or validate secondary routines, while performing conditional updates to a counter and copying a sequence of words from a source table to a destination buffer.
- `traverse_and_update_linked_list`: Traverses a null-terminated linked list to find its last element, then replaces the last element's pointer with a new node, and subsequently updates a global variable with a value fetched from a table using an offset-based calculation.
- `traverse_and_replace_node`: Identical to the first routine, performing a linked list traversal and replacing a node with a new pointer before calling another routine with various parameters.
- `insert_node_and_update_table`: Traverses a linked list to find a terminator, then inserts a new node and updates a table entry with a value from a calculated offset.
- `update_pointer_table`: Updates a pointer or a value in a table using an offset calculated from a value retrieved from a table.
- `traverse_linked_list_and_update_data`: Iterates through a linked list and updates a longword in a structure using a value fetched from a table.
- `return_empty_routine_2`: The routine performs no operations and immediately returns.
- `prepend_linked_list_node_2`: Traverses a linked list to find the last node, then inserts a new node containing a pointer from a routine call at the head of the list.
- `update_event_data_table_2`: Traverses a linked list, replaces a node with a new pointer, and updates a structure member with a value from a table using an offset.
- `update_event_data_table`: Updates a table or structure by copying a value from a pointer-based source and then performing a series of recursive or conditional calls with incrementing offsets.
- `traverse_linked_list_and_update_data_2`: Iterates through a linked list and updates a longword in a structure using a value fetched from a table with a different stride.
- `return_from_subroutine`: The routine is empty and performs no operations.
- `swap_list_node_pointer`: Traverses a linked list to find the end, then swaps a pointer in the list with a new pointer returned by another routine.
- `swap_pointer_reference_2`: Swaps the contents of a pointer or a value in a table with another pointer or a value in a table after performing a recursive call.
- `replace_list_node_and_pointer`: Traverses a linked list to find the end, swaps a pointer in the list with a new pointer, and updates a global pointer to point to a new structure.
- `update_event_data_pointer`: Updates a pointer in a data structure with a return value and a constant, then performs a pointer swap or reassignment involving a global or external memory location.
- `insert_node_and_update_pointer`: Traverses a linked list to find the end, inserts a new node, and updates a global pointer to point to a new structure.
- `update_event_state_pointer`: Updates a pointer and a short value in a local structure or object, then performs a recursive call to another routine.
- `search_and_process_table_entries`: Iterates through a table of pointers and length-limited data, checking for valid or specific termination values.
- `tiny_subq_addq`: Small routine of 2 instructions that subtracts 1 from d5, then adds 2 to d0.
- `traverse_linked_list_and_update`: Traverses a linked list of pointers, checking a condition and calling other routines that update memory or use library functions with calculated offsets.
- `update_global_and_execute_sequence`: Updates a global variable with a value from a parameter if it meets a certain condition, then executes a sequence of routines with hardcoded or calculated arguments, followed by a loop that executes routines and checks a return value or a bit-shifted value for a termination condition.
- `loop_until_flag_cleared`: Repeatedly calls a routine while checking a flag, then enters an infinite loop of routine calls.
- `sync_raster_and_store_params`: Waits for a specific raster line, updates a buffer with values from a pointer, and stores a block of parameters into a memory area.
- `parse_string_and_update_bounds_2`: Iterates through a null-terminated string to calculate the maximum length of any segment separated by a delimiter, then uses that length to calculate a display width and horizontal offset.
- `parse_string_and_update_bounds`: Iterates through a null-terminated string to calculate its length and skip spaces, while simultaneously updating a global counter and recalculating window or bounding box dimensions based on the string length.
- `configure_string_based_display_parameters`: Calculates and updates display or blitter-related parameters, such as horizontal offsets and block sizes, by parsing a string-based data structure.
- `parse_string_and_configure_blitter`: Parses a string-based data structure to calculate or update display or blitter parameters, such as horizontal offsets and block sizes, while updating a global pointer or state block.
- `parse_data_tree_node`: Recursively traverses a custom data structure, performing conditional branching and updating pointer-based counters or offsets.
- `process_data_tree_node`: A recursive routine that parses a custom data structure, performing conditional nested calls and updating pointer-based counters or offsets based on the value of the current byte being processed.
- `traverse_data_tree`: A recursive routine that traverses a table of bytes to perform conditional, nested calls.
- `traverse_and_process_data_table`: Iterates through a table of bytes, performing recursive or nested calls to a routine that processes or copies data or parameters based on the values read.
- `render_string_to_buffer_2`: Iterates through a sequence of characters, checking if they fall within printable ASCII ranges, and writes them or zero-padding or calculate.
- `render_string_to_buffer`: Iterates through a sequence of characters, checking if they fall within printable ASCII ranges, and writes them or zero-padding or calculated attribute/position values into a destination buffer.
- `update_sprite_attribute_table_2`: Iterates through a buffer to calculate and store new attribute or position values, while simultaneously updating a secondary table with character or sprite-related bytes.
- `execute_two_routines`: Calls two different routines in sequence.
- `update_table_and_copy_data`: Checks if a value in a table matches a parameter, then updates a global variable with a calculated difference and copies a sequence of words from a table to a destination buffer.
- `update_display_buffer`: Writes certain values to a global memory location and then loops to copy or transform data from one memory area to another, specifically updating a buffer or table.
- `configure_interrupts_and_globals`: Stores a frame pointer into a global variable, writes several constants to global variables, and returns the concatenation of two input registers while enabling interrupts via the INTENA register.
- `set_global_constants`: Stores a constant value into a global variable and writes to DMACON and INTENA hardware registers.
- `zeroed_data_slot`: Contains a single zero word that decodes as ori.b #0, d0 and is referenced by address from other routines; it does not execute any operation and returns immediately.
- `check_interrupt_and_loop`: Checks a bit in a global variable and enters an infinite loop if a certain condition is met.
- `update_mouse_position`: Calculates a difference or change in mouse movement from a hardware port and updates several global variables or memory locations based on certain bounds.
- `update_clocks_and_registers_2`: Updates several global variables with new values calculated from register or memory contents, including a check for a certain range and a write to a specific memory address.
- `update_clocks_and_registers`: Calculates new values based on comparisons and writes the results to a global variable and several memory locations, including a touch to the COP1LCH register.
- `setup_bitplane_blit`: Computes blitter parameters from two points and writes them to multiple custom chip registers.
- `copy_cstring`: Copies a null-terminated byte string from a source pointer to a destination pointer, byte by unable, and terminates the destination with a zero byte.
- `traverse_linked_list_and_compare`: Traverses a linked list of pointers, and for each node, performs a string copy and a string comparison using the provided parameter.
- `find_trackdisk_device`: Calls the exec library to locate the trackdisk.device and extracts a value from a device structure, returning it combined with an input register.
- `validate_list_and_execute_function`: Traverses a list while checking a counter, and if a certain condition is met, executes a function pointer or returns an error code.
- `traverse_and_increment_counter`: Traverses a linked list while checking a bitmask and increments a local counter until it reaches a certain value.
- `update_state_and_counter`: Updates a memory location with a value from a routine and checks a bitmask to either store a high-word or a constant.
- `update_loop_with_condition`: Updates a memory location with a high-word from a pointer and enters a loop that increments a counter and checks a flag or a value.
- `traverse_linked_list_and_extract`: Traverses a linked list while checking a condition and extracting a high-word or a value from a register into a local variable.
- `update_pointer_and_extract_high_word`: Stores a value from another routine into a memory location and updates a nearby or related memory location with either a high-word or a constant value based on a bitmask check, repeating until a certain condition is met.
- `extract_high_word_and_update_pointer`: Extracts the high word from a memory location and updates a pointer or a global variable based on a bitmask check.
- `extract_high_word_loop`: Extracts the high word from a memory location and loops while updating a pointer or a value in a global variable.
- `extract_and_verify_bits`: Extracts a high-word value from a memory location and repeatedly calls another routine, updating a memory location with a new value and checking certain bits.
- `check_value_and_branch`: Extracts a high-word from a pointer and checks a returned value against a constant before potentially calling another routine.
- `rebuild_node_tree_links_2`: Recursively traverses and updates a tree-like structure by validating node pointers and clearing or updating a fixed-size table of associated pointers.
- `rebuild_node_tree_links`: Recursively traverses a tree-like structure to validate or update node pointers, clearing or updating a fixed-size table of associated pointer pairs.
- `traverse_and_reset_pointer_table_2`: Recursively traverses a tree-like structure to validate node pointers, clearing a fixed-size table of associated pointer pairs and updating a global status flag if no valid nodes are found.
- `traverse_and_reset_pointer_table`: Recursively traverses a linked list or tree-like structure to validate node pointers and clear a fixed-size table of associated pointer pairs.
- `recursively_validate_and_clear_table`: This routine recursively traverses a pointer-based structure to check certain memory conditions, clears a fixed-size table of pointers, and then performs a secondary pass to reset or update table entries based on the results of a secondary recursive traversal.
- `initialize_and_traverse_pointer_table`: Clears a table of pointers and then iterates through a linked list or offset-based table, accumulating a sum of returned values while recursively calling another routine.
- `traverse_and_update_pointer_table`: Iterates through a table of pointers, recursively calling another routine and accumulating a sum in a local or global variable.
- `set_global_flag`: Sets a specific memory location to a constant value.
- `traverse_and_fetch_pointer`: Checks a return value and certain global flags to decide whether to execute a sequence of recursive or conditional routine calls.
- `validate_and_fetch_pointer`: Checks a return value and certain global flags to decide whether to execute a sequence of recursive or conditional routine calls, potentially updating a pointer or size in a destination location.
- `return_constant_one`: Returns the constant value one.
- `load_file_and_store_pointer`: Calculates a pointer or size from a table, performs file operations, and if successful, stores a pointer into a memory location.
- `return_constant`: An empty routine that returns a constant value.
- `perform_file_operations`: Calls a routine to check a table, then calls routines that perform file seeking and reading using parameters or values from a table.
- `find_minimum_weight`: Iterates through a table to calculate a weighted value for each entry and identifies the index with the lowest calculated value.
- `process_linked_list_tables`: Iterates through multiple linked lists to calculate their lengths and then performs a secondary pass to execute or update data or pointers within those lists.
- `copy_and_advance_pointer`: Copies a sequence of bytes from a source pointer to a destination pointer and updates a pointer in a passed memory location to the new position.
- `insert_node_and_copy_segments`: Traverses a linked list to find a target, inserts a new node, and copies several segments of data from a source buffer to a destination buffer.
- `insert_node_and_copy_segments_variant`: Traverses a linked list to find a target, inserts a new node, and copies several segments of data from a source buffer to a destination buffer, then loads a value from a table.
- `insert_node_and_copy_string`: Traverses a linked list to find a target, copies a block of data, and inserts a new node into the list.
- `insert_node_and_update_list`: Traverses a linked list, inserts a new node, copies a block of data, and updates a pointer in the list structure.
- `update_linked_list_node`: Traverses a linked list until a null pointer is found, copies a block of memory, and replaces a node in the list with a new pointer.
- `copy_and_update_scroll_state`: Copies a block of 32-bit words from a source pointer into a destination word buffer when a flag is clear, then updates several global variables that hold scroll-related state, including a base offset, a pointer to a word table, a pointer to a word buffer and a signed index.
- `manage_file_and_memory`: Uses library calls to examine, lock, and read a file, while also managing memory allocation and clearing a series of memory locations.
- `examine_and_lock_file`: Uses dos library calls to examine a file, lock it, read an attribute from the file info, unlock it, and free allocated memory.
- `set_flag`: Stores the constant value one into a global variable and returns.
- `wait_flag`: Sets a global flag to one if it is zero and then busy-waits until the flag reaches a specific value.
- `update_resource_pointer_and_recurse`: Checks a global flag, resets two global variables, and updates a global pointer using an offset, then recursively calls itself and a library function via a pointer, finally setting a global flag to one.
- `shutdown_audio`: Checks a global flag and, if set, performs an indirect call through a library base to initialize audio DMA channels and clears the flag.
- `clear_and_free_memory`: If certain global variables are non-zero, it calls a free memory routine, clears a global variable, and then zeros out a series of memory locations via a loop.
- `compute_max_indexed_value`: The routine scans a block of bytes starting from a table located via a global variable and a fixed offset, keeping the largest byte value seen.
- `modify_global_bit`: Performs a bitwise AND on a global variable and then copies a value from a hardware register back into that global variable.
- `clear_pointer_table_entries`: The routine iterates a fixed number of times over an array of pointers referenced through a global variable and writes zero to the memory location each pointer refers to.
- `reset_audio_and_custom_state`: The routine writes zero to four audio volume registers and a custom register, and clears several global variables used for audio state.
- `initialize_audio_dma_channels`: The routine clears audio volume registers and a custom register, sets length registers to one, points four channel pointer registers to the same global variable, and writes a value to a global variable.
- `return_zero_constant`: The routine always returns a zero value.
- `iterate_audio_pointers`: Increments a global counter and, if it reaches a certain value, loops through a series of addresses while calling another routine.
- `recurse_and_return`: This routine performs a recursive call to itself before returning.
- `write_audio_channel_pointers`: Loops over four iterations, calling another routine with a decreasing short and an address that steps by sixteen each time, while touching the AUD0LCH hardware register.
- `process_command_and_clamp_value`: Decodes a command byte from a buffer to perform conditional pointer or bitwise updates, while also implementing a clamping routine that adds a value to a memory location and constrains it within a specified range.
- `clamp_value_to_limit`: Adds a value to a memory location and clamps the result so it does not exceed a certain limit stored in another memory location.
- `execute_command_from_table`: Decodes a command byte from a buffer to either call an indirect function, adjust a pointer, or modify a bit in a global variable.
- `set_fire_button_bit`: Performs a bitwise OR on a global variable to set a specific bit.
- `clear_button_bit`: Clears a specific bit in a global variable using a bitwise AND operation.
- `search_and_copy_data`: Initializes a memory area and searches a global table for a matching value, then copies a word from a calculated offset.
- `copy_word_with_signed_offset`: Computes a signed word offset from an input value, selects the sign based on a flag, and copies a word from a source address offset by that amount to a destination address derived from another register.
- `store_word_to_offset`: Stores a word from a register into a memory location offset by a fixed amount from a pointer.
- `search_and_copy_word`: Searches a global table for a matching value and copies a word from a calculated offset.
- `call_indirect_from_table`: Calculates an address from a global table and calls the resulting pointer.
- `find_and_copy_word`: Iterates through a global table to find a matching pointer and then copies a word from an offset of that pointer to a destination.
- `find_and_copy_word_2`: iterates through a table of words to find a match for a value located at an offset from a pointer, then copies a word from a secondary offset of that pointer to a destination.
- `find_and_copy_word_masked`: Iterates through a global table to find a matching pointer and then copies a word from a masked offset of that pointer to a destination.
- `copy_word_from_offset`: Copies a word from a memory location offset by a fixed amount to a destination offset from a pointer.
- `adjust_pointer_by_nibble`: Extracts a nibble from a pointer, and if non-zero, either adds or subtracts that value from the pointer before storing the result.
- `decrement_short_by_nibble`: Masks the fourth byte of a short pointer with 15 and, if non-zero, subtracts that nibble from the pointed short and stores the result to an offset in another argument.
- `update_audio_and_dma_registers`: Updates certain hardware registers, including a DMA controller, and modifies a global pointer or table based on a loop counter.
- `update_entity_attribute_table`: Calculates a table offset based on a type-specific multiplier and updates a pointer-based structure with new attribute or position values.
- `update_pointer_data`: Updates a pointer-based structure by calculating a new value, performing conditional clamping, and updating a global variable via a bitwise OR.
- `update_dma_parameters_2`: Updates several memory locations with new values and performs a bitwise OR on a global variable.
- `update_dma_parameters`: Checks a pointer, updates several memory locations with new values, and performs a bitwise OR on a global variable while touching the DMA controller.
- `update_multiple_values`: Writes a longword, a shortword, and a bit-shifted shortword to different memory offsets, and performs a bitwise OR on a global variable.
- `execute_library_call`: Increments a register to a certain value, stores a pointer to a global variable, and calls a library function using a library base from a global variable.
- `increment_and_store_pointer`: Increments a register until it reaches a certain value, stores a pointer to a global variable, and writes a constant to a memory location before calling another routine.
- `initialize_sound_state`: Initializes sound channel parameters and clears a word in RAM.
- `initialize_sound_channel_state_2`: Clears a word in RAM and initializes sound channel parameters or state.
- `decrement_and_initialize_sound`: Decrements a word at an offset, clears a word in RAM, clamps a value to a maximum, and calls another routine with parameters from a pointer.
- `clear_indexed_memory_2`: Checks if a value at an offset is non-zero and, if so, sets a global pointer to zero and clears a memory location calculated from that pointer and an offset.
- `clear_indexed_memory`: Checks a value at an offset and, if non-zero, sets a global pointer to zero and clears a memory location calculated from that pointer and an offset.
- `clear_global_pointer_and_array_element`: Sets a global pointer to zero and then clears a value in an array or memory block located via that pointer, using an offset or index provided in a register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `open_cia_resource_and_init_audio`: Opens a resource, initializes audio DMA channels, and clears audio pointers and counters.
- `conditional_library_reset_audio`: Checks a global word and, if non-zero, calls an indirect library function via a global pointer, calls another routine, and clears the global word.
- `modify_and_assign_global`: Performs a bitwise AND and OR on a global variable and then assigns a value from a memory location back to that global variable.
- `clear_audio_pointers_and_counters`: Clears four audio channel length and volume registers and four pointer variables and zeroes four RAM counters.
- `reset_audio_and_enable_dma`: Clears audio channel registers and pointers, zeroes RAM counters, writes a value to a global variable and touches DMACON to enable DMA channels.
- `reset_audio_channels`: Calls a routine that touches the audio hardware register and manipulates a global counter, then returns the value passed in D1.
- `configure_audio_dma`: Updates a global variable and writes a pointer and a constant to an audio hardware register through a loop, while also resetting a global counter and performing a decrementing loop.
- `configure_audio_dma_channels`: Clears certain global variables, updates audio channel registers, and enables DMA channels via DMACON.
- `initialize_sound_channel_state`: Computes an offset from a parameter, zeroes a word in RAM, stores several words from parameters into RAM at that offset and writes a parameter to RAM.
- `decrement_and_clamp_value`: Decrements a value in a structure if it is non-zero, otherwise copies a value and clamps the sum of two other values to a range before storing it.
- `clamp_and_copy_short`: Copies a word within a structure, adds two words, clamps the sum to a range and stores the clamped value back into the structure and another argument.
- `copy_structure_members`: If a pointer or value in a structure is non-zero, it copies several fields from that structure to a new location and potentially clears a global value.
- `return_empty_routine_3`: The routine is empty and performs no operations before returning.
- `process_string_metadata`: Performs a complex sequence of string parsing, table updates, and library-based pointer assignments, including executing a stored callback and calling a DOS library function.
- `compare_and_update_string_metadata`: Compares two null-terminated strings and, if they match, calculates a new size and populates a table with pointers to a linked list of string-related structures.
- `validate_and_update_string_table`: Compares a string from a structure with a global string and, if they match, traverses a linked list of pointers to calculate a size and populate a new table or structure.
- `execute_indirect_calls`: The routine executes several indirect library functions and indirect function calls via a table.
- `copy_string_until_null`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `convert_to_uppercase`: Checks if an input byte is a lowercase ASCII letter and subtracts a constant to convert it to uppercase if it is.
- `convert_to_lowercase`: Checks if an input byte is an uppercase ASCII letter and adds a constant to convert it to lowercase if it is.
- `compare_byte_strings`: Compares two byte strings and returns a value indicating if the first is larger, smaller, or equal, up to a certain length limit.
- `copy_memory_safe`: Copies a specified number of bytes from a source to a destination, using a reverse-order copy if the destination address is higher than the source address.
- `initialize_memory_and_libraries`: Clears a range of memory, updates a global pointer with a library-derived value, and executes a callback if a certain bit is set in a library-retrieved byte.
- `do_nothing`: this routine is empty and performs no operations before returning.
- `parse_string_tokens`: Parses a source string to extract space-separated or quoted tokens, storing pointers to these tokens in a list or table.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_to_buffer_2`: Calculates the length of a source string and copies it to a destination buffer, ensuring a null terminator is present.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `resolve_and_assign_window_pointer`: Performs a series of library calls and pointer lookups to assign a value or pointer to a structure member.
- `store_pointer_and_clear`: Stores a pointer into a global variable and subsequently clears that same global variable to zero.
- `update_format_string`: Stores a value into a global variable, calls a routine to parse a format string, and then clears the previously stored value.
- `increment_and_set_byte`: Increments a global integer and stores a byte into the memory location previously pointed to by that integer.
- `execute_recursive_call`: Performs a recursive call to itself and returns a 64-bit value formed by concatenating register contents.
- `parse_format_string`: Parses a string containing format specifiers and arguments to extract or format data.
- `perform_signed_division`: Reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls itself recursively without performing any other operation.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `execute_stored_callback`: Checks if a pointer stored at a certain offset is non-zero and, if so, executes the routine at that address.
- `iterate_and_execute_callbacks`: Iterates through a range, calls various library functions, and executes several indirect function pointers or callbacks if they are non-zero.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `validate_and_close_handle`: Checks if an index is within a valid range for a table and if the associated handle is non-zero, then calls a routine to close the handle and clears the handle in the table.
- `execute_indirect_close`: Calls a function pointer retrieved from an offset within a global table.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_function_2`: loads a function pointer from a table or structure and executes it.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_pointer`: Loads a function pointer from a table or structure and executes it.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_pointer_2`: Loads a function pointer from a table located via an offset from a global pointer and executes it.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `execute_indirect_library_function`: loads a function pointer from a table offset by a constant and executes it.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and calls a function at a specific offset.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function_2`: Retrieves a function pointer from a table using an offset from a library base and executes it.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_3`: Performs an indirect call to a library function using a pointer or offset derived from a global variable.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_function_4`: Executes a library routine by calculating an address from a base pointer stored in a global variable.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 1 of 3).adf:add21k

- `initialize_memory_and_libraries_2`: Zeros a block of memory and calls a library routine to retrieve a pointer, then stores it in a global variable.
- `open_system_libraries_and_setup`: Opens system libraries, stores their base addresses, and executes a setup routine for each entry in the library data.
- `do_nothing_2`: the routine is empty and performs no operations.
- `allocate_tagged_file_request`: Allocates a file request block, stamps it with a four-character tag, and opens a named file through DOS library wrappers, recording the handle and buffer.
- `parse_string_token`: Parses a string or a sequence of quoted strings, skipping whitespace and handling null-terminated or quoted-string delimiters.
- `parse_string_tokens_2`: Parses a source string to extract or copy space-delimited or quote-enclosed tokens into a destination table.
- `copy_string_to_buffer_3`: Calculates the length of a source string and copies its contents into a destination buffer until a null terminator or a maximum length is reached.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `update_structure_pointers`: Performs a series of indirect calls through a chain of pointers and, if certain conditions are met, updates a structure or table with a new pointer and a value from a table.
- `execute_conditional_function`: Checks if a pointer stored in a global variable is non-zero and, if so, executes the routine located at that address.
- `execute_cleanup_pointer`: Checks a memory location for a non-zero value and executes the function pointed to by that value.
- `release_program_resources`: Iterates through a table to execute cleanup callbacks, frees a list of entries, and calls library routines to close or free remaining resources.
- `return_immediately_2`: the routine performs no operations and immediately returns to the caller.
- `validate_and_traverse_table`: Checks if an index is within the bounds of a table and if the target entry is non-zero.
- `call_close_function`: Loads a function pointer from an offset within a table and executes it.
- `execute_indirect_function_3`: loads a function pointer from a table or memory location and executes it.
- `execute_indirect_function_4`: loads a function pointer from a table or structure and calls it.
- `execute_indirect_function_5`: loads a function pointer from a table or structure and executes it.
- `execute_indirect_function_6`: loads a function pointer from a table or a global structure and executes it.
- `execute_indirect_function_7`: loads a function pointer from a table or structure and executes it.
- `execute_indirect_function_pointer_3`: loads a function pointer from a table located at a fixed offset from a base address and executes it.
- `execute_indirect_function_pointer_4`: Loads a function pointer from a table located via a global pointer and executes it.
- `execute_indirect_call`: Performs an indirect call to a function pointer located at an offset from a global pointer.
- `execute_indirect_function_8`: loads a function pointer from a table using an offset from a global pointer and executes it.
- `execute_indirect_function_9`: loads a function pointer from a nested table lookup and executes it.
- `execute_indirect_function_10`: loads a function pointer from a table located via a global offset and executes it.
- `execute_indirect_function_11`: loads a function pointer from a table using an offset and executes it.
- `execute_indirect_function_12`: loads a function pointer from a table located via an offset from a base address and executes it.
- `execute_offset_function`: Loads a function pointer from a different offset within a table and executes it.
- `execute_indirect_function_13`: loads a function pointer from a table located at an offset from a global pointer and executes it.
- `execute_indirect_function_14`: loads a function pointer from a table or structure and executes it.
- `execute_indirect_function_15`: loads a function pointer from a table or structure and executes it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `execute_indirect_function_16`: loads a function pointer from a table or structure and executes it.
- `execute_indirect_call_2`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `execute_indirect_call_3`: Loads a function pointer from a table using a different offset and executes it.
- `execute_indirect_function_17`: loads a function pointer from a table and executes it.

### Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 2 of 3).adf:TRSI-Intro

- `start_fade_command`: Runs a loop that calls a memory freeing function a certain number of times, clears a stored pointer, and returns.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `perform_memory_update`: Executes a loop that calls an external routine and updates a memory location by adding a value to its current contents.
- `copy_memory_block_2`: copies a block of memory from a source address to a destination address.
- `shift_and_copy_memory`: This routine performs a bitwise left-shift-and-propagate operation on a byte while simultaneously copying or moving bytes within a memory range.
- `copy_byte_to_memory`: copies a byte from a memory location pointed to by a register into a memory location pointed to by another register.

### TRSI

- `initialize_library_references`: Opens a library and calls several functions using pointers or return values from a library opening routine.
- `close_library_and_verify_status`: calls a library close routine and executes several other function pointers while checking a memory value against a constant.

### TRSI

- `free_memory_and_decrement`: calls a memory free routine and executes a routine that decrements a local variable until it reaches negative one.
- `execute_short_sequence`: a short routine consisting of eight instructions called by another routine.

### Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 2 of 3).adf:bootblock

- `manage_library_handles_3`: A routine with 14 instructions that calls a routine to close a library and check the system superstate.
- `manage_library_handles`: Closes a library and checks the system superstate.
- `initialize_boot_environment`: Executes a sequence of instructions that includes opening and closing libraries and enables a DMA channel via a hardware register.
- `execute_fourteen_instructions_2`: Calls a routine that executes fourteen instructions and uses two library calls.
- `execute_fourteen_instructions`: a routine consisting of fourteen instructions called by another routine.
- `execute_nine_instructions`: a routine consisting of nine instructions called by another routine.

### Operation Stealth (1990)(U. S. Gold)(De)[cr TRSi](Disk 3 of 3).adf:bootblock

- `manage_library_handles_4`: A routine with 14 instructions that calls a routine to close a library and check thesystem superstate.
- `manage_library_handles_2`: Closes and checks the status of a library while managing a DMA channel.
- `initialize_graphics_library`: This routine opens and closes a graphics library while enabling DMA or certain DMA channels via the DMA controller.
- `execute_fourteen_instructions_3`: A routine consisting of eleven instructions that calls another routine and uses two library calls.
- `modify_memory_contents`: the routine performs a sequence of instructions that modifies a value in memory.
- `modify_register_value`: the routine modifies a register or a memory location using a fixed or or a bitwise operation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: performs an infinite loop that prevents further execution.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_function_table`: Allocates memory and populates a table with the addresses of other routines.
- `initialize_jump_table_and_copy_data`: Populates a jump table by calculating relative offsets and then performs a series of block memory copies and pointer updates using a loop.
- `filter_and_copy_table_values`: Iterates through a source table, checking if a value falls within a specific range, and copies certain values to a destination table if the condition is met.
- `parse_and_center_string`: Calculates the length of a string and updates a table of string-width or padding values.
- `parse_string_and_update_table`: Parses a string to calculate a length and writes a calculated value and a pointer-derived value to a table.
- `zero_memory_with_decrement`: decrements a register and writes zero to a memory location pointed to by a register until a certain value is reached.
- `update_sprite_attribute_table`: Iterates through a table of encoded sprite or object descriptors to update or load attribute data.
- `decrement_value_if_nonzero`: If a value pointed to by a pointer is non-zero, it subtracts certain amounts from that value based on its bitmask.
- `increment_value_if_different`: If a value pointed to by a pointer differs from a register, it increments that value by certain amounts based on bitmask checks.
- `copy_memory_block_3`: Swaps two values in a global structure and copies a block of memory using an optimized loop.
- `copy_block_and_update_pointer`: copies a block of memory using a routine that decrements a value and updates a pointer and a counter in a global or external structure.
- `copy_table_data`: Copies multiple bytes from a source table to a destination table using a pointer-based loop.
- `update_pointer_value`: Stores a value from a register into a memory location pointed to by another register, then updates a second memory location with the return value of a library function.
- `execute_indirect_function_18`: Loads a function pointer from a global memory location and calls it.
- `free_memory_conditionally`: Calls a library function to free memory if certain registers contain non-zero values, then returns a value from a register.
- `call_alloc_mem`: Calls a library function using a pointer stored in a memory location.
- `execute_indirect_function`: loads a function pointer from a global memory location and calls it.
- `free_memory_if_valid`: If two registers contain non-zero values, it calls a library function to free memory.
- `modify_global_bitmask`: Checks a bit in a global variable and, if set, performs a bitwise rotation and mask update on a global variable.
- `update_global_counter`: Checks a bit in a global variable and increments a value at a global address if a certain condition is met.
- `initialize_table_values`: Resets several values in a table or structure to zero or a fixed constant.
- `setup_table_sequence`: Decrements a counter and, if zero, swaps two 32-bit values and resets certain flags; otherwise, it writes a sequence of values to a table while waiting for a hardware flag or bit check.
- `iteratively_update_pointer_offsets`: Decrements a counter in a global variable while updating a set of pointers and associated integer or or pointer-based values with an offset and a constant, provided a certain condition is met.
- `swap_and_store_pointers`: Swaps two pointers in a global variable and then copies a sequence of four 32-bit values from a source pointer to a destination pointer.
- `store_long_word_split`: Takes a long word from a source pointer and stores it as two separate words into a destination buffer, advancing the destination pointer by four bytes per iteration for a fixed number of iterations.
- `process_string_buffers`: Iterates through a null-terminated string and applies a bitwise OR mask to a series of memory locations.
- `apply_bit_mask`: Performs a bitwise OR or a bit-shifted bitwise OR on a series of memory locations using a source and a destination pointer.
- `find_string_length`: Calculates the length of a null-terminated string by iterating until a null byte is found.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
