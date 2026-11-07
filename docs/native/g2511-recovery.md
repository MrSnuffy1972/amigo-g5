# Game 2511 (Cybercon3 v1.7 0243): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02511). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (855538 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 180 of 180 named (0 author symbols), 180 of 180 described (7 by rule, 172 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1916 of 1916 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1024 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3032 of 3032 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 261940 |
| code | 8850 |
| data | 584746 |
| padding | 2 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 153 | 11 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2162 | 127 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 711 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| Disk.1 bytes 0-1024 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Cybercon3_v1.7_0243/Cybercon3_v1.7_0243_hdf/Cybercon3/Disk.1: raw disk image (292352 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Cybercon3_v1.7_0243/Cybercon3_v1.7_0243.hdf: hard-disk image (AmigaDOS OFS, 896 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree Cybercon3_v1.7_0243_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 6 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1312 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 474 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Cybercon3_v1.7_0243/Cybercon3_v1.7_0243_hdf/Cybercon3/Disk.1: bytes 0-1024 look like code no run loaded: recovered position-independently (load address unknown)
- Cybercon3_v1.7_0243/Cybercon3_v1.7_0243_hdf/Cybercon3/Disk.1: 7 logged loads cover 260096 bytes of the image
- Cybercon3_v1.7_0243/Cybercon3_v1.7_0243_hdf/Cybercon3/Disk.1: chunks 152-277 (bytes 38912-71168) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.98)
- 1 never-loaded region(s), 1024 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `store_constant_to_global` (runtime segment an address): Stores a constant byte into a global variable. Kind: checksum/integrity trap. Pass path: btst.b #an address, d0 (where d0 bit 3 is 0).
- `set_control_port_all_ones` (runtime segment an address): Writes an all-ones value to the CIAB port register, with no other work. Kind: hardware/port state check. Pass path: the branch at an address (after the beq.b).
- `update_audio_hardware_parameters` (runtime segment an address): Uses a table lookup to calculate an offset and writes masked values to specific memory locations, while interacting with various audio and DMA hardware registers. Kind: disk hardware/track parameter check. Pass path: the code continues execution after the rts instruction in sub_1B12.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `clear_memory_and_initialize_hardware`: Clears a large block of memory, resets specific hardware registers, and copies a data block to a new location.
- `decompress_and_clear_memory`: Decompresses a data stream and, depending on the result of a function call, executes one of several indirect calls or clears a block of memory starting at a specific address.
- `nop_placeholder_routine`: This routine is empty and performs no operations before returning.
- `store_data_to_offset`: stores a 32-bit value and a 16-bit value into a structure pointed to by a register at specific offsets.
- `store_word_to_offset`: Copies a word from a stack location to a memory location offset from a pointer.
- `wait_for_bit_set`: polls a byte at an offset from a pointer until a specific bit is set.
- `return_constant_value`: returns a constant 32-bit integer value.
- `call_indirect_function`: calls a function through a pointer located at an offset within a table in memory and returns its result.
- `decompress_data_stream`: Decompresses a data stream by reading length headers and copying bytes or blocks of data from a source table to a destination.
- `empty_routine`: This routine is empty and contains no executable instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reorder_data_tables`: performs a complex nested loop to copy and rearrange data from a source pointer into several destination tables.
- `loop_forever`: contains an infinite loop that never exits.
- `wait_for_nonzero_flag_byte`: spins in a loop while a global byte remains zero.
- `update_color_via_nested_loops`: Executes nested loops to decrement two registers and calls a routine that modifies a hardware color register.
- `enter_infinite_loop`: An empty routine that enters an infinite loop and never returns.
- `loop_forever_no_exit`: contains only an unconditional loop with no calls or work, so it never returns.
- `write_packed_data_to_hardware`: Uses nested loops to pack three nibbles into a 16-bit result and writes the result and its double to a memory location associated with hardware registers.
- `decrement_and_store_eight`: Decrements a counter through a loop and stores the value eight into a global variable.
- `scale_vector_components`: Multiplies three consecutive values in a table by eight and then performs a second pass multiplying them by seven.
- `write_scaled_words_to_color`: Decrements a global variable and then writes a series of doubled words from a code pointer into the hardware color register.
- `add_three_word_vector`: Adds a three-word source vector to a three-word destination, then stores a helper result into an extra word.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `pack_three_nibbles`: Combines three register values into one 16-bit result by shifting them into place in four-bit steps, then returns it.
- `modify_hardware_registers`: Performs bitwise operations on a byte, increments a value, and returns either zero or all ones based on a bit test.
- `return_all_ones`: Returns a value consisting of all bits set to one.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `copy_memory_with_xor`: Calculates a destination address by performing an XOR operation on a pointer and then copies two thousand words from that source to a new location.
- `copy_memory_block`: Copies a block of memory of two thousand words from a source address to a destination address.
- `return_immediately`: This routine consists of a single instruction and returns to its caller without performing any operations.
- `store_constant_to_global`: Stores a constant byte into a global variable.
- `set_control_port_all_ones`: Writes an all-ones value to the CIAB port register, with no other work.
- `update_coordinate_tables`: Performs arithmetic on values from two memory locations and stores the results back into those locations.
- `initialize_table_and_bounds`: Fills a table with a repeating sequence of values and then calculates the bounding box of a linked list of four-word structures.
- `fill_memory_with_pattern`: Stores four initial values into a table and then uses a loop to fill subsequent memory locations with a repeating pattern of two values.
- `merge_sorted_interval_lists`: Merges two lists of sorted intervals by calculating intersections and combining overlapping or adjacent ranges.
- `insert_into_linked_list`: Traverses a linked list until the end is reached and then inserts a new entry at the final pointer location.
- `tiny_movea_rts`: Small routine of 2 instructions that copies a2 to a1, then returns.
- `compute_rectangle_list_bounds`: Starts from one rectangle's bounds and widens them to cover each entry of its linked list, storing the result in two globals.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_table_bounds`: Performs range checks on a global variable and, if valid, stores calculated boundary values into a table before potentially calling another routine twice.
- `execute_empty_calls`: An unreached routine that calls another routine twice and returns zero.
- `execute_recursive_calls_2`: Performs two consecutive calls to another routine and returns zero if a 16-bit register is less than one.
- `execute_recursive_calls`: This routine performs two consecutive calls to another routine before returning zero.
- `tiny_addq_add`: Small routine of 2 instructions that adds 4 to d3, then adds d3 to d7.
- `decrement_word_register`: Decrements a 16-bit word in a data register by one in a loop until the value reaches negative one.
- `populate_nested_pointer_array`: Performs a nested loop to calculate a 32-bit value and stores it through a pointer that is decremented in each iteration.
- `write_complex_word_do_while`: Uses a do-while loop to execute a nested calculation and store a 32-bit word through a pointer that is decremented in each iteration.
- `write_complex_word_to_pointer`: Performs a nested loop to calculate a value and stores it as a 32-bit word through a pointer that is decremented in each iteration.
- `populate_word_table_incrementing`: Iterates through a nested loop structure to perform arithmetic operations and store the resulting 32-bit values into a table using an incrementing pointer.
- `populate_word_table_do_while`: Uses a do-while loop to iterate through a nested structure, performing arithmetic and storing 32-bit results into a table via an incrementing pointer.
- `populate_word_table_via_nested_loop`: Iterates through a nested loop structure to perform arithmetic operations on register values and store the resulting 16-bit words into a memory buffer via a pointer.
- `interpolate_values_in_loop`: Calculates a combined value from two inputs and stores it into two different memory locations within a loop.
- `interpolate_and_store_values`: Calculates a value based on the difference and sum of two numbers and stores the result into two different memory locations via a loop.
- `interpolate_coordinate_table`: Performs linear interpolation between points in a table to calculate intermediate values.
- `return_immediately_2`: The routine is empty and returns immediately.
- `call_via_conditional_offset`: Compares a value at a pointer to a global variable and calls a routine using either a specific offset or a value from a table.
- `execute_indirect_call_2`: Checks the sign of a byte at a pointer location and performs an indirect call using an offset derived from the pointer's contents.
- `call_based_on_distance`: Calculates the sum of squares of three differences and performs an indirect call using an offset if the result is within a specified limit.
- `call_via_distance_check`: Calculates the sum of squares of three values and calls a routine using an offset from a pointer if the sum is within a certain range.
- `scale_vector_components_2`: Checks if three values are within a specific range and repeatedly doubles them while decrementing a counter.
- `call_via_indirect_pointer`: Calls a routine using an offset retrieved from a pointer.
- `sum_and_call_indirect`: Sums two values into a global variable and calls a routine using a third value as an offset.
- `call_via_indexed_offset`: Selects a pointer based on a byte check and calls a routine using an offset derived from a value and an index.
- `call_indirect_offset_2`: Performs an indirect call to a routine using an offset stored in a pointer.
- `store_and_call_offset`: Stores two parameters into global variables, modifies one via subtraction from a constant, and performs an indirect call using a pointer.
- `add_and_call_offset`: Adds three values to three global variables, modifies one via subtraction from a constant, and performs an indirect call using a pointer.
- `negate_and_call_offset`: Iterates through two tables to negate sets of three values and then performs an indirect call using an offset from a table.
- `add_and_call_offset_2`: Iterates through two tables to add three constant values to sets of three values and then performs an indirect call using an offset from a table.
- `test_and_store_boolean`: Iterates through a table to perform a containment test and stores the result as a boolean byte in another table before calling an indirect routine.
- `check_sign_and_store`: Iterates through a table to check if a value is negative and stores the inverted result as a byte in another table before calling an indirect routine.
- `process_and_store_boolean`: Iterates through a table to perform a calculation and stores the inverted boolean result of a sign check into a byte table before calling an indirect routine.
- `double_indirect_call`: Performs two consecutive indirect calls using pointers and offsets.
- `call_indirect_via_offset`: Performs an indirect call to a routine located at an offset stored in a pointer.
- `call_indirect_via_offset_2`: Executes an indirect call using an offset retrieved from a pointer.
- `call_indirect_via_table_offset`: Calls a routine by adding a short offset found at a specific position within a table to a base address.
- `store_data_and_call_indirect`: Stores three words of data into a memory location calculated from a pointer and an offset, then performs an indirect call using another offset from that pointer.
- `call_indirect_via_table_offset_2`: Calculates a target address by adding a short value from a table to a base address and calls it.
- `call_indirect_via_table_offset_3`: Performs an indirect call to a routine using an offset located two bytes into a table.
- `call_indirect_via_table_offset_4`: Uses a short offset from a table to perform an indirect call to another routine.
- `call_indirect_via_offset_3`: Performs an indirect call to a routine using an offset provided by a pointer.
- `execute_indirect_call`: Loads an offset from a table and calls the routine located at that address.
- `call_indirect_via_table_offset_5`: Executes an indirect call to a routine located at an offset found within a table.
- `call_indirect_via_nested_offset`: Performs an indirect call to a routine using an offset retrieved from a location determined by adding a pointer value to itself.
- `call_indirect_table_entries`: Loads two offsets from a table pointed to by a register and calls the resulting routines.
- `store_and_call_indirect`: Stores a word from a pointer into a global variable and then calls a routine using an offset from that same pointer.
- `call_indirect_offset`: Calls a routine using an offset stored at the location pointed to by a register.
- `branch_on_byte_value`: Checks a byte in a table and performs one of two different indirect calls based on whether that byte is negative.
- `store_and_call_indirect_2`: Stores a byte into a global variable and then calls a routine using an offset from a pointer.
- `set_byte_and_call`: Stores a byte of value 255 into a memory location at an offset from a pointer and then calls another routine.
- `clear_byte_and_call`: Stores a byte of value 0 into a memory location at an offset from a pointer and then calls another routine.
- `shift_and_store_loop`: Iterates through a list of values, right-shifting them by a variable amount and storing them in a table before calling another routine.
- `perform_fixed_point_math`: Performs complex arithmetic operations involving shifts and divisions on data within a table and stores the results.
- `return_immediately_stub`: The body is a bare return with no work, so no behaviour is recovered; the code is a stub that only returns.
- `perform_indirect_call`: This routine performs an indirect call to another routine using an address stored at a specific offset within a pointer.
- `perform_indirect_call_2`: Checks a value at a pointer offset and performs an indirect call using a calculated address.
- `copy_and_analyze_index_table`: Copies a sequence of values from a source table into a temporary buffer while checking a secondary flag table.
- `call_via_pointer`: Calls a routine using an offset provided by a pointer.
- `store_and_call_via_pointer`: Stores a value from the stack into a global variable and then calls a routine using an offset from a pointer.
- `call_via_offset`: Calls a routine using an offset from a pointer.
- `conditional_call_via_offset`: Checks a value at an offset from a pointer, potentially calls another routine, and then calls a routine using an offset from a pointer.
- `expand_rectangle_bounds`: Computes the bounding box of a linked list of rectangles and stores the result in two global variables before performing an indirect call.
- `merge_interval_lists`: Merges two sorted lists of intervals and performs an indirect call via an offset.
- `insert_linked_list_entry`: Inserts a new entry into a linked list and performs an indirect call via an offset.
- `update_global_via_pointer`: Performs an indirect call and conditionally stores a global variable or zero into a memory location at a specific offset.
- `call_indirect_via_offset_4`: Performs an indirect call to a routine using an address calculated from a pointer and an offset.
- `clear_and_call_indirect`: Sets a 16-bit value at an offset from a pointer to zero and then performs an indirect call.
- `load_record_and_call`: Loads several short values from a record into global variables and then calls a routine via a pointer found within a nested table structure.
- `execute_complex_arithmetic`: Performs a loop of bitwise shifts, multiplications, and subtractions on data registers.
- `perform_complex_arithmetic_loop`: Executes a loop performing various bitwise shifts, multiplications, and subtractions on register values.
- `calculate_rotation_matrix_from_memory`: Fetches three sets of values from memory using offsets derived from a pointer and calculates a nine-entry three-by-three matrix stored in a table.
- `build_matrix_from_angle_registers`: Computes the same nine-entry three-by-three matrix as its neighbour, but takes the three angle fields directly from data registers and writes the matrix to the output record.
- `empty_routine_2`: This routine contains no functional instructions and performs no operations before returning.
- `copy_five_words_between_slots`: Copies five 16-bit words from one set of fixed memory locations to five adjacent fixed locations, one word each.
- `multiply_matrix_vector`: Calculates three values by performing a matrix-vector multiplication using values from several memory locations.
- `compute_matrix_products`: Computes three products through a matrix-vector multiplication process and stores them in memory.
- `multiply_matrix_by_vector`: Performs a matrix-vector multiplication by calculating three products using values from two different memory locations and storing the results in a table.
- `multiply_matrix_record_by_vector`: Multiplies a three-by-three matrix held in a record, addressed by one register, by the three-word vector at another register address.
- `swap_stored_triplet_buffers`: Updates a three-word state from fields of a record, after an optional call that adds scaled offsets, then exchanges the current and previous three-word buffers held in fixed memory.
- `compute_transformation_matrix`: Calculates a 4x4 transformation matrix by performing scaled vector additions and bitwise shifts.
- `compute_difference_with_sign`: Loads a 16-bit value from a table indexed by the first parameter word and returns it with its sign word, plus the difference between that value and a second table entry.
- `perform_containment_test`: Calculates a dot product and performs a containment test using three coordinate sets from a table, returning minus one if the test fails.
- `compute_plane_dot_from_registers`: Performs a containment test and scaled dot product using three coordinate values from data registers, returning minus one if the test fails.
- `calculate_scaled_ratio_3`: Computes a scaled ratio based on the comparison of two input values using multiple conditional branches and arithmetic operations.
- `calculate_scaled_ratio_2`: Computes a scaled ratio based on the comparison of two input values.
- `calculate_scaled_ratio`: Compares two values and returns either a constant, a zero, or a value derived from a complex scaled division and multiplication sequence.
- `return_constant`: Returns the constant 8192 in d0 to the caller without any other effect.
- `compute_scaled_product`: Divides an input value by the high word of a second input, then uses the low word of the quotient to index a scaled multiplication with a constant offset, producing a 32-bit result formed from the low word duplicated into high and low parts.
- `walk_list_and_dispatch`: Walks a linked list and calls one of two routines for each node based on a value at a specific offset within the node.
- `insert_into_linked_list_2`: Inserts a global variable into a doubly linked list by updating the pointers of the new node and its neighbors.
- `unlink_list_entry_and_mark`: Removes an entry from a doubly linked list, sets its flag word to minus one and marks it with a fixed sentinel.
- `traverse_linked_list`: Iterates through a linked list by following pointers at specific offsets until a termination condition is met.
- `unlink_and_invalidate_node`: Sets the node's word at byte offset 82 to minus one, then unlinks the node from its doubly linked list by joining its predecessor and successor.
- `unlink_and_invalidate_node_2`: Sets a word at an offset of eighty-two bytes within a node to minus one, then unlinks the node from a doubly linked list by updating the predecessor's pointer to the successor.
- `unlink_node_from_chain`: When the byte at offset 106 is positive, sets that byte to minus one and unlinks the node from its successor chain at offset 102, updating the neighbouring link.
- `push_node_onto_chain_head`: When the node's field at offset 106 is not positive, links the node in front of the current global head and makes it the new head.
- `initialize_data_structures`: Clears a block of memory, initializes several global variables with specific constants, and builds a linked list of nodes.
- `compute_status_global_flag`: Writes a series of flag tests into a global variable, but the final value depends only on whether one field equals a fixed value and another field equals 15 or 11: minus one if so, otherwise zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_table`: Performs arithmetic on a global variable and a pointer to update two related memory locations.
- `initialize_and_process_structure`: Initializes several fields in a structure, performs a loop that decrements a global value, and checks specific bitmasks in memory.
- `increment_global_offset_value`: Increments a 16-bit value at a specific offset within a global structure and calls another routine.
- `process_complex_loop`: Initializes a global variable, enters a loop that calls two routines, and performs memory operations based on a specific constant check.
- `execute_looping_calls`: Calls another routine ten times within a loop.
- `write_structured_data`: Calculates an offset based on a register, stores a global variable into multiple memory locations, and copies data from two different tables.
- `initialize_structure_fields`: Writes a calculated value to an offset in a global structure, copies a value from a memory address, and sets several other fields to specific constants.
- `check_and_modify_byte`: Checks if a global variable is non-zero and, if so, calls another routine before performing a bitwise OR on a byte at an offset from a register.
- `loop_until_null`: Repeatedly calls another routine as long as the value pointed to by a register is not zero.
- `execute_looping_routines`: Repeatedly calls two different routines within a loop until a specific condition is met via a null check.
- `modify_byte_at_offset`: Performs a bitwise OR on a byte at a specific offset from a register.
- `execute_unnamed_subroutines`: An unreached routine that calls two unnamed subroutines.
- `call_unnamed_routines`: This routine executes two unnamed subroutines.
- `execute_unnamed_routine`: An unreached routine that calls another routine.
- `return_from_subroutine_2`: This routine consists of a single instruction that returns control to a caller.
- `execute_unnamed_subroutines_2`: This routine calls two unnamed subroutines.
- `loop_and_modify_global`: Stores a value in a global variable and enters a loop that calls another routine until the global value reaches zero, followed by arithmetic and further calls.
- `set_global_memory_sequence`: Calculates a value based on a global variable and stores it into a series of consecutive memory locations.
- `process_memory_sequences`: Calls two different routines, one of which calculates a value based on a global variable and stores it in consecutive memory locations, while the other iterates through data to store concatenated results in a table.
- `process_data_segments`: Iterates through data using a loop that calls another routine, performing comparisons and storing concatenated results into a table.
- `clear_global_variables`: Sets several consecutive global memory locations to zero.
- `and_three_helper_words_repeated`: Repeats 512 times: obtains three words from helper routines and stores their bitwise AND at the output pointer.
- `clear_memory_block`: Iterates through a memory block starting at a specific address, setting groups of four-byte words to zero.
- `add_to_global`: Adds the value of a register to a global variable.
- `update_global_with_sequence`: Sets a global variable to various values and calls several routines to update it based on conditional checks.
- `call_helper_then_store_routine_address`: Calls a helper routine and then stores the address of a routine in a global variable.
- `call_if_nonzero`: Calls another routine if a global variable is non-zero.
- `update_audio_hardware_parameters_2`: Updates audio hardware parameters by performing table lookups to calculate offsets and writing masked values to specific memory locations.
- `initialize_audio_parameters`: Initializes a series of memory locations and tables with specific values, including a loop that performs bitwise operations to populate a buffer.
- `update_audio_register`: Uses a value to index into a table and writes masked data to a memory location based on the retrieved value.
- `tiny_ori`: Small routine of 1 instruction that merges 58 into d6.
- `update_table_entry`: Calculates an offset using a table and a register, then writes masked values to a destination based on specific bit conditions.
- `update_table_entry_2`: Uses a value from a register to index into a table and writes masked data to a calculated memory location.
- `update_table_entry_3`: Performs a table lookup to determine a memory offset and updates two memory locations with masked register values.
- `update_audio_hardware_parameters`: Uses a table lookup to calculate an offset and writes masked values to specific memory locations, while interacting with various audio and DMA hardware registers.
- `update_global_sequence`: Sets a global variable to zero, calls several routines, and performs arithmetic on a byte through a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_3`: This routine is empty and performs no operations before returning to the caller.
- `nop_placeholder`: This routine is empty and performs no operations.
- `execute_no_operation`: The routine consists of a single instruction that performs no action and returns immediately.
- `perform_no_operation`: The routine is empty and performs no actions before returning.
- `return_from_subroutine_4`: The routine is empty and simply returns.
- `return_no_operation`: The routine consists of a single instruction that immediately returns to its caller, performing no work and touching no registers or hardware.
