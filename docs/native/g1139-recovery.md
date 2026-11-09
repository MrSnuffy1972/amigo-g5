# Game 1139 (captive ii - liberation (1992)(mindscape)(disk 1 of 5)[cr hlm]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01139). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905216 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 333 of 333 named (0 author symbols), 333 of 333 described (19 by rule, 308 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 648 of 648 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7769 of 7769 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 604015 |
| code | 23428 |
| data | 116829 |
| padding | 2736 |
| unused | 158208 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| endcli | 2 | 117 | 2 |
| Liberation | 5 | 10 | 1 |
| PlayASP | 4 | 10 | 1 |
| Captive II - Liberation (1992)(Mindscape)(Disk 1 of 5)[cr HLM].adf:HLM.EXE | 2 | 150 | 6 |
| Captive II - Liberation (1992)(Mindscape)(Disk 1 of 5)[cr HLM].adf:HardDisk_Install | 3 | 10 | 1 |
| math.library | 1 | 807 | 27 |
| tridee.library | 1 | 2181 | 93 |
| vector.library | 1 | 3569 | 184 |
| Captive II - Liberation (1992)(Mindscape)(Disk 1 of 5)[cr HLM].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 897 | 17 |

## Cases handled

- captive ii - liberation (1992)(mindscape)(disk 1 of 5)[cr hlm]/Captive II - Liberation (1992)(Mindscape)(Disk 1 of 5)[cr HLM].adf:StartUp.asp: high-entropy data file (7.84 bits/byte), classified as packed data
- runtime segment an address: 503 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- captive ii - liberation (1992)(mindscape)(disk 1 of 5)[cr hlm]/Captive II - Liberation (1992)(Mindscape)(Disk 1 of 5)[cr HLM].adf:c/PlayASP: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- captive ii - liberation (1992)(mindscape)(disk 1 of 5)[cr hlm]/Captive II - Liberation (1992)(Mindscape)(Disk 1 of 5)[cr HLM].adf:HLM.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- captive ii - liberation (1992)(mindscape)(disk 1 of 5)[cr hlm]/Captive II - Liberation (1992)(Mindscape)(Disk 1 of 5)[cr HLM].adf:HardDisk_Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 to d0 and then branches to another location.

### Liberation

- `traverse_and_jump_table`: Follows a chain of pointers by reading a value, multiplying it by four, and repeating until a zero is found, then performs an indirect jump using an offset of four from the resulting address.

### PlayASP

- `traverse_pointer_table_and_call`: Follows a chain of pointers starting from a global variable by multiplying each retrieved value by four until a null pointer is encountered, then performs an indirect jump to an address stored at an offset of four bytes from that null pointer.

### Captive II - Liberation (1992)(Mindscape)(Disk 1 of 5)[cr HLM].adf:HLM.EXE

- `expand_compressed_data_block`: Reads a header from a data stream to determine the number of expansion steps and then enters a loop that performs conditional table lookups and bitwise operations to write expanded bytes into a destination buffer.
- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `lookup_masked_table_word`: Looks up a word in a table at a fixed base, indexed by twice a register value, and returns it masked with a second register word.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 163 into d0.
- `apply_bit_masks`: Sets bits 1, 3, and 5 of a byte in d0 and then branches to another location.

### Captive II - Liberation (1992)(Mindscape)(Disk 1 of 5)[cr HLM].adf:HardDisk_Install

- `traverse_pointer_chain_and_call`: Follows a chain of pointers starting from a global variable by multiplying each retrieved value by four until a null pointer is encountered, then calls a routine at an offset of four from the final pointer.

### math.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `store_two_fields_in_library`: Stores two longword arguments into fixed fields of a structure addressed through a register, at offsets 36 and 40 bytes.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_counter_then_conditional_call`: Decrements a 16-bit usage counter in a library structure.
- `set_structure_bit`: It is library code in libs/math.library.
- `free_block_and_return_value`: Reads a longword stored at offset 36 of a structure, passes the structure to two system library calls that release it, and returns the saved value.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `lookup_word_from_table`: Masks the input to its low nine bits, doubles that to a byte offset, and returns the 16-bit word at that offset in a table held in library data.
- `scale_and_offset_point_pair`: Scales the two 16-bit coordinates in a structure by a ratio built from its scale and divisor fields, then adds a fixed offset field to each.
- `swap_words_and_store`: Rotates a 32-bit value by sixteen bits, exchanging its upper and lower words, and stores the result at the target address.
- `scale_point_by_divisors`: Multiplies each 16-bit coordinate by a scale field, divides by a sum that includes a shared offset field when that sum is non-zero, then adds an offset field to each.
- `perform_complex_math_on_stack`: Uses a table to fetch two 16-bit values based on an index from a stack offset, performs several multiplications and subtractions, and stores the results back into two different stack offsets.
- `execute_multiple_math_blocks`: Performs four distinct sequences of table lookups, multiplications, and subtractions using values from various stack offsets, updating those offsets with the results.
- `execute_library_function`: Performs three sequences of mathematical operations involving table lookups and multiplications, using negated values from stack offsets to calculate and store results.
- `split_value_by_hundred`: It is library code in libs/math.library.
- `advance_linear_congruential_seed`: Reseeds a 16-bit state variable from its argument when that argument is non-negative, then advances it with a linear congruential step that multiplies by 1509 and adds 41.
- `build_rotation_matrix_from_angles`: Looks up sine and cosine values in a table for three angles, then combines them in fixed-point arithmetic to fill a nine-element 3 by 3 rotation matrix in the caller's buffer.
- `build_alternate_rotation_matrix`: Builds a nine-element 3 by 3 matrix from three angles using the same table lookups as its neighbour, but combines the terms in a different order.
- `accumulate_scaled_products`: Iterates through a series of 16-bit values from a buffer, multiplying them by corresponding values from a second buffer and accumulating the results into a 32-bit sum that is scaled and stored.
- `multiply_3x3_fixed_point_matrix`: Multiplies two 3 by 3 matrices of 16-bit fixed-point values, shifting each dot product right by 14 bits to keep the scale, and writes the nine results to the output buffer.
- `calculate_integer_square_root`: Calculates the integer square root of a 32-bit value in d0 using a binary search loop.
- `compare_and_return_word`: It is library code in libs/math.library.
- `set_d0_to_one`: Sets register d0 to one and restores registers d1 through d4 from the stack before returning.
- `interpolate_table_value`: Performs linear interpolation by finding a value within a table using a binary search and calculating the fractional position.
- `move_and_branch_2`: It is library code in libs/math.library.
- `compute_interpolation_fraction`: Computes where an input lies between two table entries by dividing the scaled difference by the span between those entries, negating the result when the sign flag is set.
- `compute_clamped_triangle_ratio`: Forms two edge vectors relative to a base point, takes their cross product and dots it with a third vector, then divides by a scale value and clamps the result between zero and a limit stored in the structure.

### tridee.library

- `return_negative_one`: Sets register d0 to negative one and returns.
- `store_two_fields_in_library_2`: Stores two longword arguments into fixed fields of a structure addressed through a register, at offsets 36 and 40 bytes.
- `increment_counter_and_clear_flag_2`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_counter_then_conditional_call_2`: Decrements a 16-bit usage counter in a library structure.
- `set_structure_bit_alt`: It is library code in libs/tridee.library.
- `free_block_and_return_value_2`: Reads a longword stored at offset 36 of a structure, passes the structure to two system library calls that release it, and returns the saved value.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_and_call_library`: Increments a byte at a relative address, and if the result is not zero, calls a routine via a pointer in a6.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `decrement_and_close_library`: Decrements a byte at a relative address and, if it reaches zero, clears a longword and calls a routine via a pointer in a6.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `calculate_buffer_size`: As library code, it checks a bit in a structure at offset 10 of a0, and if not set, calculates a new value by multiplying a word from a0 by 52, adding a shifted version of that word, and adding 164, then calls a library routine if bit 0 of a value at offset -1 of a6 is clear.
- `recalculate_buffer_layout`: Allocates memory and updates several fields in a library structure, including size, count, and offset values based on an input structure.
- `recalculate_buffer_layout_fields`: Recalculates the size and layout fields of an existing buffer from a size value, deriving a count by dividing by 52 and storing a shifted size and an end offset.
- `call_and_restore_registers`: It is library code in libs/tridee.library.
- `clear_buffer_field`: As library code, it checks if the word at offset 148 of a0 is non-zero, and if so, clears the word at offset 148 of a0 and calls a library routine if bit 0 of a value at offset -1 of a6 is clear.
- `free_memory_and_call_handler`: Calls a routine using a pointer from a structure, then retrieves a pointer from the stack to perform further operations and calls another routine.
- `free_memory_then_call`: It is library code in libs/tridee.library.
- `allocate_and_initialize_structure_2`: Calculates a size based on an input value and calls a library routine to allocate memory, then initializes several fields in the new structure.
- `allocate_and_initialize_structure`: It is library code in libs/tridee.library.
- `clear_d0_and_restore`: Sets register d0 to zero and restores registers d1 through d7 and a0 through a6 from the stack before returning.
- `reset_bounds_and_clear`: Copies a longword between two fields, clears two state fields, and sets four 16-bit bounding fields to the largest and smallest signed 16-bit values.
- `project_point_list_to_screen`: Adds a translation to each point in a list, then perspective-projects the x and y coordinates by scaling them and dividing by depth plus an offset, skipping the division when the divisor is zero.
- `compute_weighted_sum_fixed`: Computes a weighted sum of three 16-bit inputs using three stored coefficients, skipping any zero input, then repacks the result into a fixed-point pair.
- `scale_point_by_divisors_2`: Multiplies each 16-bit coordinate by a scale field, divides by a sum including an offset field when that sum is non-zero, then adds an offset field to each.
- `copy_and_process_structure`: Copies several fields from a structure pointed to by a1 into a stack frame, performs a loop to copy eight 16-bit words from a table at the start of the structure to the stack, and if bit 0 of a 16-bit word at offset 42 is set, pushes three 16-bit words from the structure onto the stack and calls another routine.
- `push_structure_fields_to_stack`: Checks if bits 0 and 1 are both set in a 16-bit word at offset 42 of the structure pointed to by a1; if so, it pushes three 16-bit words from offsets 26, 28, and 30 of the structure onto the stack, pushes two 32-bit pointers onto the stack, and pushes a 32-bit value from a table onto the stack before returning.
- `copy_structure_fields`: As library code, it checks bits 0 and 1 of a word at offset 42 of a1, and if both are set, copies five words from a structure pointed to by a1 to a structure pointed to by a0 starting at offset 68.
- `stub_return_immediately_2`: Returns without doing any work.
- `push_three_words_to_stack`: Pushes three 16-bit words from offsets 108, 110, and 112 of a structure pointed to by a6 onto the stack, then pushes a pointer to a global variable and a constant value before returning.
- `store_three_words_in_state`: Stores three 16-bit values taken from its two arguments into fixed fields of a state structure at offsets 108, 110 and 112.
- `load_record_into_state`: Copies several fields from an input record into the state structure, sets two 16-bit limits to the largest and smallest signed values, and runs flag-dependent processing.
- `check_structure_bit_and_branch`: It is library code in libs/tridee.library.
- `update_entity_list`: Iterates through a list of pointers, conditionally adding an offset, updating a field from a structure, and performing a calculation involving multiplication and division to update a byte at offset 15 of each entry.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `validate_and_fetch_list_pointer`: Saves the current stack pointer to a memory location, loads two offsets from the base address in a6 into a4 and a5, and checks bit 3 of a 16-bit word at offset 128 of the base address; if bit 3 is set, it loads a longword from the address at offset 90 of the base address into d0 and otherwise branches to another routine or returns.
- `update_bounding_box_from_list`: Iterates through a linked list of structures, optionally applying an offset to each pointer and updating a field within the structure from a value in a control block.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a4, then returns.
- `process_table_index`: As library code, it checks several bits and a word at offset 12 of a1, and if conditions are met, updates a byte at offset 14 of a1, calculates a value using a table at offset 104 of a6, and branches to another routine.
- `load_multiple_structures_2`: Checks bit 7 of a byte at offset 129 of a6; if not set, it pushes d0 and several values from a1 onto the stack, then loads three sets of three words from a table pointed to by a0 using offsets from a2 and pushes them onto the stack.
- `load_multiple_structures`: As library code, it repeatedly loads three words from offsets of structures pointed to by a2 into the stack, then loads a pointer from offset 100 of a6 and pushes its contents to the stack.
- `store_scaled_byte`: Library code that stores an input byte into a record field, optionally scaling the value by a ratio before writing.
- `calculate_clamped_delta_2`: As library code, it checks a bit in a structure at offset 20 of a pointer in a1; if the bit is clear, it stores d0 into offset 5 of that structure and calculates a value using a series of arithmetic operations involving a global structure in a6, then branches to another routine.
- `calculate_clamped_delta`: Calculates a value based on the sum of two 16-bit words, subtracts a scaled value derived from a global structure, and updates a byte at offset 15 of a structure with the result, ensuring it is not negative.
- `copy_and_branch`: As library code, it copies the word in d0 to d1 and branches to another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_clamped_byte`: Calculates a clamped delta by subtracting a scaled value derived from a global structure from a 16-bit word, then adds the result to a byte at offset 13 of a structure and stores it at offset 15.
- `find_value_range_2`: As library code, it clears a longword at offset 16 of a structure in a1 and, if a bit at offset 20 is clear, iterates through a list of values pointed to by a table in a2 to find a minimum and maximum value, which are then scaled and stored in offsets 16 and 18 of the structure in a1 and potentially updated in a global structure in a6.
- `find_value_range`: Library code that iterates through a list of records to find the minimum and maximum values, scales them, and updates a global range.
- `initialize_bounding_box_bounds`: Sets a flag at offset 32 of the structure in a1 and, if a specific bit in that flag is not set, initializes a bounding box with minimum and maximum values from registers d1 and d2, or resets them to zero and 32767 respectively.
- `calculate_vertex_bounding_box`: Iterates through a list of vertex indices to find the minimum and maximum coordinate values, updating a bounding box structure.
- `process_vector_scaling_and_rotation`: Calculates scaling factors and performs a 2D rotation and magnitude comparison using values from a structure pointed to by a4.
- `calculate_complex_product`: Library code that performs multiple signed multiplications and shifts using registers d2 through d5, then calculates a value based on the bit position of d7 and returns it via the stack.
- `calculate_scaled_product`: As library code, it multiplies a 16-bit value in d7 by a 16-bit value in d2, shifts the result left by a word from the stack, subtracts d0 from d7, and enters a loop that performs bit shifts and increments a value on the stack until a condition is met or it branches to another routine.
- `divide_shifted_by_word`: Performs a division of a 32-bit value by a 16-bit divisor using a bit-shifting loop and returns the quotient scaled by a shift amount.
- `swap_structure_fields_conditionally`: Clears a 16-bit word at offset 44 of a structure in a1 and moves a 16-bit value from d0 to offset 34.
- `compute_squared_ratio_fixed`: Returns a 16-bit ratio of two squared magnitudes, each built from stored record fields and scaled by two stored coefficients.
- `compute_weighted_squared_ratio`: Computes a ratio of two squared quantities, weighting the denominator by squared coefficients passed as arguments, and returns it in the low word of the result.
- `compute_scaled_difference_ratio`: Computes a 16-bit result by multiplying the difference of two products by a third word and dividing by a fourth word, then returns.
- `calculate_bounding_box_area`: Calculates the area of a bounding box by multiplying width and height derived from coordinate differences, and updates the area and dimensions in a structure at a1.
- `adjust_and_branch`: As library code, it adds 8 to a4, unlinks a4, and branches to another routine.
- `compute_squared_edge_length`: Calculates the squared length of the difference between two stored vertices in three dimensions, using their indices from the structure, and returns its absolute value.
- `update_bounding_box_2`: Calculates coordinate bounds by reading from a table and updating a structure with the resulting minimum and maximum values.
- `update_bounding_box`: Calculates the minimum and maximum coordinate bounds from a list of points stored in a table and updates the bounding box values in a structure.
- `return_minus_one_fallback`: Returns minus one together with a longword from the second argument, with no other work.
- `update_vertex_list_from_source`: Checks several flags and bounds in the structures passed in a1 and a6, then iterates through a list of vertex indices to copy vertex data from a source list to a destination list.
- `filter_and_copy_vertices`: Iterates through a list of vertex indices, checking a depth value at offset 10 of each vertex structure against a threshold; if the depth is valid, it increments a counter and copies the 16-byte vertex data to a destination list using an index from a secondary table.
- `reorder_transformed_vertex_list`: Processes a list of vertices by transforming their coordinates via a scaling routine and then reordering them into a new list based on a threshold comparison.
- `update_vertex_list_and_reorder`: Compares a value from a table with a threshold in a structure, then enters a loop that calls scale_point_by_divisors_2 to transform coordinates, updates a list of vertex data, and performs a block memory move to reorder elements in a buffer.
- `branch_to_offset`: Copies the value from a2 to a7 and then branches to an offset within another routine.
- `update_bounding_box_bounds`: Checks a flag in a structure at offset 20 of a1 and, if a specific bit pattern is met, compares a value at offset 10 of a pointer in a1 with a value in d0; if the condition is met, it modifies a flag at offset 20 of a1 and returns a longword value from a global variable, otherwise it proceeds to iterate through a list of pointers to calculate minimum and maximum coordinate values.
- `calculate_bounding_box_and_update_buffer`: Iterates through a list of vertex indices to find the minimum and maximum coordinate values, updating a bounding box structure, and if certain flags or capacity conditions are met, copies vertex data from a source to a destination buffer.
- `compute_squared_edge_length_2`: Calculates the sum of the squares of the differences between three pairs of 16-bit values retrieved from two structures, returns the absolute value of the result in d0, and otherwise returns.
- `calculate_coordinate_offsets`: Library code that stores a word from a register into a structure, clears a word at offset 2, calculates a value by subtracting a word at offset 49 from a word at offset 10 of a table, and updates four word values in a structure using arithmetic involving a second table.
- `update_bounding_box_and_check_area`: Library code that iterates through a list of points to find minimum and maximum 16-bit values for two coordinates, stores these in a structure, and then performs a cross-multiplication comparison between two sets of coordinate differences.
- `update_vertex_buffer_and_reorder`: Checks several flags and capacity constraints in a structure, then copies vertex data from a source to a destination buffer while applying a bitmask to a field at offset 20.
- `filter_and_copy_vertices_2`: Checks a depth value at offset 10 of a vertex structure against a threshold provided in a stack parameter; if the depth is positive, it increments a counter, updates an index in a table, and copies 16 bytes of vertex data into a destination buffer.
- `transform_and_reorder_vertices`: Transforms a list of vertices by scaling their coordinates and then reorders them into a new list based on a depth threshold.
- `call_routine_then_return`: Calls one other routine and returns, with no other work of its own.
- `sort_entry_list_by_key`: Sorts a linked list of entries in place by a 16-bit key stored in each entry, swapping adjacent out-of-order nodes and repeating until a pass makes no swap.
- `process_overlapping_rectangles`: Iterates through a linked list of rectangular structures to find overlaps, then performs geometric calculations involving coordinate averaging and cross-product-based area comparisons using data from the structure's members.
- `check_and_update_bounding_box`: Checks if two rectangular structures overlap by comparing their dimensions and then updates a target structure with either an expanded bounding box or a collection of specific rectangle coordinates.
- `calculate_expanded_bounding_box`: Calculates an expanded bounding box by adding 100 to a width value, averaging the sums of various dimension pairs, and adding 50 to one of the resulting dimensions before storing the results into a structure.
- `calculate_weighted_area_difference_2`: Calculates a weighted difference between dimensions of two rectangle structures, performing a bitwise XOR comparison against a value in a structure, then optionally averages coordinates from a table based on a flag and a secondary index.
- `calculate_weighted_area_difference`: Calculates a weighted difference between dimensions of two rectangle structures, performing a bitwise XOR comparison against a value in a structure, and if a condition is met, iterates through a list to update a destination structure with width and height differences.
- `calculate_rectangle_intersection_area`: Retrieves dimensions from two rectangle structures and stores their width and height differences into a destination structure.
- `compare_and_swap_rectangle_dimensions`: Calculates the area of two rectangles by multiplying their width and height values retrieved from structures, compares these areas, and swaps the dimension values in the target structure if the comparison fails.
- `validate_and_swap_pointers`: Calculates the area of a rectangle using values from a structure in a6 and compares two such areas; if the comparison fails, it modifies the color register and returns, otherwise it exchanges the contents of registers a1 and a3 and potentially performs longword moves to swap pointers.
- `swap_pointer_elements`: Library code that exchanges the contents of registers a1 and a3, and if a1 does not equal a2, performs a series of longword moves to swap pointers between the locations pointed to by a1, a3, and a0.
- `shuffle_pointer_data`: Moves a longword from a pointer in a1 to d0, stores a1 at the address in a2, stores a3 at the address in a0, moves the longword from a3 to the address in a1, moves d0 to the address in a3, exchanges a3 and a1, and branches to another routine.
- `unlk_a6_and_return`: Unlocks a structure pointed to by a6 and returns.
- `compare_and_swap_registers`: Compares d0 and d1, and if d0 is less than d1, it exchanges the values of d0 and d1.
- `stub_return_immediately`: Performs several arithmetic and swap operations on registers d0 through d5 but does not branch to an earlier instruction or call another routine.

### vector.library

- `return_negative_one_2`: Returns minus one in register d0 as library code.
- `store_two_fields_in_library_3`: Stores two longword arguments into fixed fields of a structure addressed through a register, at offsets 36 and 40 bytes.
- `increment_counter_and_clear_flag_3`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_counter_then_conditional_call_3`: Library code that decrements a 16-bit counter in a library structure; if the counter reaches zero and bit 3 of a byte at offset 34 is set, it calls another routine and returns its result, otherwise it returns zero.
- `modify_structure_flags`: Saves d2, a5, and a6 to the stack, copies a6 to a5, loads a6 from an offset of 40 in the structure pointed to by a5, tests a word at offset 32 of that structure, and if it is zero, branches to another routine; otherwise, it sets bit 3 at offset 34 of the structure in a5 and branches to another routine.
- `free_structure_and_return_pointer`: Reads a longword from offset 36 of a structure, passes the structure to two system library calls to remove it from a list and free its memory, and returns the original longword in d0.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `jump_to_next_instruction`: Performs a short relative branch to the instruction located at the next immediate address.
- `calculate_and_update_vector_structure`: Calculates a value based on several arithmetic operations involving input registers and a bitmask, then writes the result and several other parameters into a structure pointed to by a pointer obtained from a call.
- `initialize_vector_structure_2`: Allocates memory for a new structure and populates it with several parameters, including a pointer, a linked list, and calculated values stored at various offsets.
- `initialize_vector_structure`: As library code, it calls another routine to allocate memory, then populates a structure in a2 with a pointer, a linked list, and several calculated 16-bit values derived from multiplying registers and offsets, and finally calls two other routines before returning.
- `transfer_pointer_and_call`: Library code that moves the value from register a2 to a0, calls another routine, clears register d6, and then branches to another location.
- `fill_row_pointer_table`: Fills a table of pointers with consecutive addresses separated by a stride, computed as the product of two 16-bit values, with one entry per row.
- `load_vector_parameters`: This library routine loads two 16-bit values from offsets 22 and 34 of a structure in a2 into d2 and d7, then branches to another routine.
- `fill_arithmetic_word_sequence`: Fills a 16-bit array with an arithmetic sequence starting from the first argument and increasing by the second argument per element, for a count given by a third argument.
- `execute_library_callback`: This library routine retrieves a pointer from a structure in a0, loads a value from that pointer, and if bit 0 of the byte at offset -1 of a6 is set, calls another routine using a value from the stack.
- `call_free_mem_wrapper`: Calls a routine to free memory, then checks a bit in a value at an offset from a pointer in a1, and if that bit is not set, pushes three onto the stack and calls another routine.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `process_vector_linked_list`: Traverses a linked list of structures using a pointer in a1, performing bitwise checks on a status word at offset 5 of each structure.
- `traverse_linked_list_and_execute`: Traverses a linked list of structures using indices and pointers, updating a value in a structure at offset 922 of a6 with each step.
- `update_vector_state`: Calls another routine, updates a 16-bit value in a structure at offset 330 of a6 by adding a value from d0, and modifies bit 12 and bit 8 of a 16-bit word at offset 20 of a4.
- `clear_library_pointer`: This library routine moves a pointer from offset 894 of a6 into a4, clears the value at offset 894 of a6, and then restores registers from the stack before returning.
- `branch_to_sequence`: Returns immediately by performing a short branch to another routine.
- `test_and_execute_conditional_sequence`: Tests bit 3 of a byte at offset 962 of the structure in a6, and if set, executes a sequence of four subroutines with conditional branching based on the zero flag.
- `execute_conditional_subroutines`: As library code, it calls one subroutine, then if the zero flag is not set, calls another subroutine before returning 1 in d0.
- `call_conditional_subroutines`: Calls a subroutine, and if the zero flag is not set, calls another subroutine before returning 1 in d0 as library code.
- `validate_and_execute_interpolation`: Calls a routine to update the interpolation state, and if the zero flag is not set, calls another routine to calculate the interpolation step size and returns one, otherwise it returns.
- `copy_pointer_list_values_2`: This library routine checks a value at offset 8 of a6; if non-negative, it copies 32-bit values from a list into a destination buffer, either by using an offset from a pointer or by reading directly from a list of pointers, based on a bit in a value at offset 32 of a4.
- `copy_pointer_list_values`: Copies a counted run of 32-bit values, each read from a fixed offset within a referenced record, into the destination buffer.
- `execute_sequential_subroutines`: Calls three different subroutines and loads the address of a value from an offset of a6 into a1 before the calls.
- `clip_and_prepare_point_data`: Library code that saves registers, loads two 16-bit values from a table, compares them against bounds in a structure, and if they are within range, calculates a bitmask and a word offset for bitplane access.
- `plot_clipped_points_to_bitplane`: This library routine calculates a memory offset using a multiplication, then loops through a list of points to clear and set bits in a destination buffer using bitmasks and shifts derived from coordinate values.
- `calculate_vector_scaling_and_bounds_2`: Calculates scaling factors and coordinate bounds using fixed-point division based on values in a structure, then performs a loop to update a.
- `calculate_vector_scaling_and_bounds`: Calculates scaling factors and coordinate bounds using fixed-point division based on values in a structure, then performs a loop to update a series of 16-bit values in a destination buffer if the calculated coordinates fall within a specified range.
- `calculate_and_clamp_vector_bounds`: Performs fixed-point division on vector components to determine scaling factors, checks if the resulting coordinates fall within bounds defined in a structure, and if valid, calculates a series of offset values to be stored in a table and calls another routine.
- `clamp_vector_coordinates_2`: Calculates a 2D coordinate offset by adding a 16-bit value to a 16-bit input and checking if the result falls within a range defined by offsets 30 and 34 of a structure in a5.
- `clamp_vector_coordinates`: Subtracts a 16-bit offset from a structure in a5 and writes the result to a destination in a0, then performs range checks against values at offsets 30, 32, 34, and 36 of the structure in a5 to clamp the resulting coordinates, writing a specific constant to a1 if the values fall outside the allowed bounds.
- `transform_and_clamp_vector_coordinates`: Calculates a coordinate offset by subtracting values from a structure in a5, writes the results to a destination in a1, and performs a loop that applies clamping logic to transform vector data within specified bounds.
- `accumulate_and_transfer_registers_2`: Library code that pushes register d7 onto the stack, adds d7 and two instances of d3 to the stack, subtracts d5 from the stack, and then branches to an earlier instruction or a later instruction based on the result.
- `update_vector_transform_loop`: Iterates through a list of vectors, performing coordinate transformations and clamping results within specified bounds stored in a structure pointed to by a6, then updates the vector data via pointers in a0 and a1.
- `accumulate_and_transfer_registers`: Adds the contents of register d6 to d4, increments d0 by one, and performs multiple additions to accumulate the values of d4 and d6 into d7 before branching to another routine.
- `validate_span_index`: As library code, it checks a word at offset 932 of a structure in a6; if non-zero, it performs several comparisons using values from a5 and d2 to determine whether to call another subroutine and return 1, or return 0.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `test_both_flags_negative`: Returns minus one only when two signed 16-bit state fields are both negative, and zero otherwise.
- `clamp_bounds_to_clip_window`: Clamps the four bounding fields of a record to the clip window, taking the larger of each minimum and the smaller of each maximum, and stores the results in the working state.
- `interpolate_vector_data_range`: Checks if two rectangular coordinate ranges overlap; if they do, it performs linear interpolation on a list of 16-bit values to generate intermediate values within the specified bounds and stores the results in a destination buffer.
- `interpolate_vector_data_list`: Performs linear interpolation on a series of 16-bit values within a list, calculating intermediate values based on a target threshold and storing the results in a destination buffer.
- `interpolate_vector_data`: Performs linear interpolation on a series of 16-bit values within a list, calculating intermediate values based on a target threshold and storing the results into a destination buffer.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `update_vector_interpolation_state`: Increments a counter in a structure and checks it against range bounds; if the counter is within a specific window, it performs a loop to copy 32-bit values from a source list into a destination list, using either direct offsets or an index table based on a flag.
- `interpolate_vector_list_elements`: Copies 16-bit values from a source list into a destination list using a loop, then performs linear interpolation between elements based on a threshold value and stores the results in a new list.
- `interpolate_vector_list`: Iterates through a list of 16-bit values and performs linear interpolation between elements based on a threshold value, storing the resulting 16-bit values into a destination buffer.
- `interpolate_vector_elements`: Iterates through a list of 16-bit values, performing linear interpolation between elements based on a threshold value, and stores the resulting interpolated 32-bit words into a destination buffer.
- `pop_stack_to_offset`: As library code, it pops a 16-bit word from the stack and stores it at an offset of 36 from the pointer in a5, then returns.
- `interpolate_vertex_span_values`: Iterates through a list of vertex pairs, calculates a linear step value based on the difference between coordinates, and fills a destination array with interpolated 32-bit values while tracking the minimum and maximum 16-bit values encountered.
- `fill_span_rows_with_interpolation`: Fills table rows with values interpolated linearly across each span between consecutive vertices, updating running minimum and maximum extents and returning them packed together.
- `calculate_interpolation_step_size`: Checks bounds in a structure, retrieves two 16-bit values from a list, calculates their absolute difference, and performs a division using a shifted value to determine a step size or scaling factor.
- `prepare_interpolation_parameters`: As library code, it calculates a value by dividing a shifted d1 by d7 plus 1, performs a swap and arithmetic on d3, subtracts a value at offset 38 of a5 from d0, swaps d0, and sets d0 to 8000 before branching to another routine.
- `fill_span_rows_with_interpolation_2`: Fills table rows along each edge span with fixed-point interpolated values using a loop, then calls two subroutines and performs a second loop before returning as library code.
- `calculate_blitter_span_offsets`: Iterates through a table of span data to calculate horizontal offsets and lengths, then writes these values to the blitter start and end registers and calls update_blitter_span_registers.
- `update_blitter_span_registers`: Iterates through a table of span data to calculate horizontal offsets and lengths, then writes these values to the blitter's start, end, and mode registers.
- `write_blitter_span_registers`: Waits for the blitter, then writes a set of span words to the blitter registers for each row, taken from a pair of tables.
- `test_multiple_bits`: As library code, it tests bit 0 of the longword at the address in a1, bit 0 of the longword at the offset 0 of the structure in a5, bit 0 of the word at the address in a1, and bit 0 of the longword at the offset 0 of the structure in a5, incrementing both a1 and a5 after each test.
- `preserve_and_call_subroutine`: Pushes the value of d0 onto the stack, calls a subroutine, and then restores the original value to d0.
- `calculate_span_offset_and_fetch_data`: Calculates a word-based offset from a value in d0 and a structure in a5, then uses the result to fetch a word from a table pointed to by a pointer at offset 12 of the a5 structure.
- `apply_masked_span_fill_2`: Iterates through a sequence of word pairs to perform bitwise masking and logical operations on a destination memory range.
- `write_masked_span_words`: Writes longwords along a horizontal span by combining each destination longword with a mask and a value.
- `apply_masked_span_fill`: Iterates through a sequence of word pairs to perform bitwise masking and logical operations on a destination memory range.
- `write_masked_span_words_2`: Writes masked fill words along a horizontal span into a destination, combining each word with a mask and a value and advancing through row tables.
- `update_offset_and_branch`: Decrements a 16-bit value from an offset of a5, stores the result at an offset of a6, and if a longword at another offset of a5 is non-negative, it loads a word from a pointer in a1 into d0 using an offset calculated from d0.
- `merge_masked_bitplane_data`: Calculates a memory offset using a multiplication of a value from a structure and then enters a loop that performs bitwise masking and logical merging of two source words into a destination buffer, using a lookup table to handle nibble-based bit manipulation.
- `merge_masked_bitplane_words_2`: Performs a word-aligned bitwise merge of source data into a destination buffer using two bitmasks.
- `merge_masked_bitplane_words`: Performs a word-aligned bitwise merge of source data into a destination buffer using two bitmasks.
- `blit_masked_bitplane_spans`: Performs bitwise masking and merging of word-aligned data into a destination buffer.
- `apply_masked_bitplane_span`: Performs bitwise masking and logical operations to write a sequence of words into a bitplane buffer, using a lookup table to handle nibble-based bit manipulation and bitwise OR/AND operations to merge source data with existing bitplane content.
- `write_masked_span_words_3`: Writes masked words for a row span across rows of a bitplane, combining each source word with a mask and advancing through the row offset table.
- `calculate_blitter_stride_and_sequence`: Calculates destination stride parameters and bitplane counts based on source dimensions and a structure in a5, then programs the blitter with a sequence of calculated addresses and values.
- `calculate_blitter_stride_parameters_2`: Iterates through a list of source addresses and bitplane data to calculate destination addresses and stride values, then programs a series of parameters into a structure.
- `calculate_blitter_stride_parameters`: Iterates through a list of source addresses and bitplane data to calculate destination addresses and stride values, then programs a series of parameters into a structure.
- `program_blitter_sequence`: Iterates through a list of source addresses and bitplane data to calculate destination addresses and stride values, then programs a series of blitter operations by writing to a structure in a4.
- `program_blitter_row_runs`: Calculates bitplane offsets and stride values for a series of blitter operations, then iterates through a loop to program the blitter's destination address, area, and size registers using values from a table and a structure in a4.
- `emit_blitter_row_run`: Iterates over a run count, programming the blitter's pointer and size registers for each row and advancing the offset by the stored stride, then returns to its caller once the run is exhausted.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `compute_row_address_offset_2`: As library code, it multiplies the low word of a value in d0 by a 16-bit value at offset 22 of the structure in a5, then adds the result of another routine.
- `compute_row_address_offset`: As library code, it multiplies the low word of a value in d0 by a 16-bit value at offset 22 of the structure in a5, then adds the result of a calculation involving a 16-bit value at offset 366 of the structure in a6 and a 16-bit value at offset 38 of the structure in a5 divided by eight, returning the 32-bit result in d0.
- `perform_arithmetic_operations_2`: As library code, it extends d4 to a longword, divides d3 plus one by d4, swaps the words in d2, multiplies a 16-bit value at offset 22 of the structure in a5, then returns immediately or calls another routine.
- `perform_arithmetic_operations`: As library code, it extends d4 to a longword, divides d3 plus one by d4, swaps the words in d2, multiplies a 16-bit value at offset 22 of the structure in a5 by d3, subtracts a 16-bit value at offset 222 of the structure in a6 from d5, and finally multiplies d3 by d1 after decrementing d3.
- `return_immediately`: As library code, it subtracts a 16-bit value at offset 22 of the structure in a5 from a 16-bit value at offset 22 of the structure in a5, clears d1, sets d2 to zero, and returns.
- `rotate_and_duplicate_source_words`: Reads a byte from a pointer in a structure to determine rotation counts and dimensions, then loops through source words to rotate them and store each result twice in a destination buffer.
- `rotate_source_rows_with_mask`: As library code, it processes a set of rows by rotating each 16-bit source word by a shift count held in d2, storing each rotated word twice, zero-filling the remaining columns, and writing the inverted OR of the row's bits as the mask word.
- `mask_structure_bytes`: As library code, it reads two bytes from a structure in a6 at offsets 958 and 959, masks the lower 7 bits of each byte, and branches to another routine.
- `expand_bits_into_word_tables`: Reads two bytes from a structure in a6 and expands their individual bits into two separate tables of 32-bit words.
- `expand_byte_bits_into_tables_2`: As library code, it clears the word in d0 and branches to another routine.
- `expand_byte_bits_into_tables`: As library code, it loops over a fixed number of columns, taking one bit at a time from a byte and storing either zero or a pattern word into each of two tables, finishing by storing the last pattern pair into two fixed slots.
- `resolve_vector_pointer_and_scale`: Retrieves a pointer from a list based on an index and size stored in a structure, then calculates scaled dimensions and coordinate bounds for a vector and writes the results to a destination structure in a6.
- `update_vector_scaling_and_bounds`: Calculates scaled dimensions and coordinate bounds for a vector by performing arithmetic on values from a pointer in a1 and writes the results to a structure in a6, then branches to another routine based on a value at offset 8 of the structure in a6.
- `calculate_scaled_dimensions_2`: As library code, it performs 16-bit multiplications and logical shifts on values from d0 and d1 based on flags in a structure at a4, and if certain conditions are met, it moves values from a6 to registers and calls another routine.
- `update_vector_bounds_and_metadata`: Calculates scaled dimensions based on a bit flag in a structure, stores several 16-bit values into a target structure in a6, and calls two other routines before returning a status in d0.
- `test_vector_flags`: As library code, it tests bit 3 of a byte at offset 324 of a structure in a6 and branches; if bit 3 is set, it calls another routine, otherwise it tests bit 4 of the same byte, clears d6, moves d1 to d3 and d0 to d2, restores d4 and d5 from a table, moves a byte from offset 22 of a4 to offset 326 of a6, and branches.
- `update_scaled_vector_bounds`: Calculates new dimensions and coordinate bounds based on scaling flags in a structure at a4, then writes the resulting width, height, and offset values into a structure at a6.
- `perform_scaled_division`: As library code, it tests bit 0 of d6; if set, it multiplies d3 by d2, divides the result by d0, and then branches to another location; otherwise it branches to another location.
- `calculate_scaled_dimensions`: As library code, it tests bit 1 of d6 and returns if not set; otherwise it multiplies d3 by d0, divides the result by d1, performs arithmetic on values from offsets 10 and 12 of a6 based on the sign of the results, and returns 1 if a bitwise comparison with a value from offset 22 of a4 fails, otherwise it returns 0.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `process_vector_table_entries`: Clears a small buffer in a structure, calculates a scaling factor, and iterates through a table of entries.
- `process_table_entries`: As library code, it loops through a table in a5, adding the value in a0 to each non-zero word and storing the result in a4, then processes a second table in a1 by calling mark_duplicate_row_entries and updating values in a structure in a6 based on comparisons and sign flags.
- `transfer_register_values`: As library code, it moves a long word from a pointer in a4 to a memory location at a5, moves a word from d4 to a5, moves a word from d0 to d1, sets a0 to a1, sets a3 to a5, and branches to another routine.
- `mark_duplicate_row_entries`: As library code, it scans the remainder of a table for entries equal to the current value, storing the running slot into each matching position and marking those entries reused.
- `check_bounds_and_set_offsets`: As library code, it checks if a coordinate pair falls within a specified range defined in a structure in a5, and if so, calculates and stores several signed offsets into a structure in a6 before returning 1 in d0.
- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `calculate_scaled_vector_parameters`: Updates a structure in a6 by copying values from a structure in a5, performs division and bitwise operations to calculate scaled dimensions, and populates a buffer with values derived from these calculations using a loop.
- `scale_and_fill_vector_buffer`: Calculates a scaled value by dividing a word by a divisor and multiplying by 512, then performs a loop that populates a buffer with scaled values and bitwise-shifted remainders.
- `update_scaled_vector_buffer`: Calculates a scaled value by dividing a word by a value and multiplying by 512, then updates a buffer using a loop that performs bitwise shifts and masks to store results into a destination table.
- `calculate_scaled_offset_and_update_buffer`: Calculates a scaled value by multiplying a 16-bit word by a constant and adding it to a 32-bit result, then updates a buffer using a loop that performs bitwise operations and shifts.
- `accumulate_and_branch`: Adds the contents of register d2 to register d0, performs a decrement and branch loop on register d7, loads a 16-bit value from an offset of 184 in a6 into register d5, and branches to another routine.
- `extract_bitstream_descriptors`: Iterates through a loop to extract bit-length descriptors and scaling factors from a bitstream, storing the results into a destination table pointed to by a register.
- `unpack_bitstream_segments_2`: Calculates scaling factors and bit-length descriptors based on a value in a register, then iterates through a table of segments to extract bits from a source buffer and accumulate them into a destination buffer using bitwise shifts and masking.
- `unpack_bitstream_segments`: Iterates through a series of segments to extract bits from a source buffer based on bit-length descriptors and accumulates them into a destination buffer.
- `swap_and_loop`: As library code, it swaps d4, loops using d5, and branches to another routine.
- `copy_segmented_word_data_2`: Checks bit 1 of a byte at offset 292 of the structure in a6; if set, it performs a word-based copy from a source buffer to a destination buffer using a count and offset provided in the structure, otherwise it returns.
- `apply_segment_masking_operations`: Performs bitwise OR operations on destination memory using source words from a table, then iterates through a segment table to either merge source words into the destination or apply a bitwise AND-NOT mask to specific segments based on their width and position.
- `apply_mask_to_row_segments`: As library code, it walks the row segment table, ORing source words into the destination for full-width segments and applying an AND-NOT with the mask for partial segments.
- `clear_then_merge_row_segments`: As library code, it clears the destination bits covered by the mask, then walks the segment table merging source words into each row.
- `process_segment_table`: As library code, it uses nested loops to iterate through a table and a counter from a global variable, performing bitwise operations between a source buffer and a destination buffer based on values loaded from a0 and a4.
- `or_source_into_long_segments`: As library code, it ORs source longs into the row buffer, then walks the segment table using 32-bit words.
- `apply_mask_to_long_segments`: As library code, it walks the long-word segment table for each row, applying an AND-NOT with the mask to empty and partial segments and ORing source longs into full-width segments, and finishes when every row is processed.
- `copy_segmented_word_data`: Checks a flag at offset 293 of the structure in a6; if set, it iterates through a table to copy words from a source buffer to a destination offset, otherwise it iterates through a table to zero out memory segments and then copies word data from a source buffer to a destination.
- `clear_then_copy_row_segments`: Zeroes each row first, then copies source words into the destination for every segment in the table, stopping when the row count runs out.
- `copy_segment_data`: As library code, it uses nested loops to iterate through a table and a counter from a global variable, loading a word from a0 and either branching to another routine or copying a long word from a pointer at a1 plus an offset into a destination buffer at a5.
- `zero_row_then_copy_words`: Zeroes one row of words, then copies source words into the row for each segment in the table, zero-filling any span with no source, until the segment count runs out.
- `validate_and_fetch_offset`: Checks several status flags and bounds in a structure passed in a5, then calculates an index and retrieves a word from a table pointed to by a field in that structure.
- `calculate_and_populate_address_table`: Calculates a memory offset based on several input parameters and uses a loop to populate a table with calculated addresses or pointers.
- `apply_masked_bit_shifted_words_2`: Performs a bitwise merge of two source word streams into a destination buffer by shifting each word by a specified amount and combining them.
- `merge_bit_shifted_words`: Performs a bitwise merge of two source word streams into a destination buffer by shifting each word by a specified amount and combining them using logical AND and OR operations.
- `mask_shifted_word`: As library code, it performs a loop where it masks a word from a source pointer by shifting a value in register d3 by the amount in d4, inverting it, and performing a bitwise AND with a destination pointer, otherwise it returns.
- `mask_and_shift_words`: Performs a bitwise AND operation on words in a destination buffer using a bitmask generated by shifting a source word by a specified amount.
- `or_shifted_word_to_destination`: Masks a word with a bitmask, fetches a word from a table, shifts it left by a value in d4, and performs an OR operation into a destination pointed to by a6.
- `ors_shifted_rows_pointer_check`: Ors shifted source words into the destination like its neighbour, with a stacked row-count argument.
- `apply_masked_bit_shifted_words`: Calculates a bit-shift amount from a register and iterates through two source tables to perform bitwise AND-NOT and OR operations on a destination buffer.
- `merge_bit_shifted_words_2`: Performs a bitwise OR and AND operation on a destination buffer using two source tables, where the source words are swapped and bit-shifted by a variable amount.
- `mask_swapped_word`: As library code, it performs a loop where it takes a word from a source pointer, swaps its bytes, performs a logical shift right by the amount in d4, inverts the result, and performs a bitwise AND with a destination pointer, otherwise it returns.
- `apply_bitmask_to_shifted_words`: Performs a bitwise AND operation on a destination buffer using words from a source table that have been swapped and bit-shifted.
- `or_swapped_shifted_word_to_destination`: Fetches a word from a table, swaps its bytes, shifts it left by a value in d4, and performs an OR operation into a destination pointed to by a6.
- `or_swapped_shifted_rows_checked`: ORs swapped and left-shifted words from a source table into a destination, using a loop count for the primary row.
- `process_masked_word_blocks`: Performs nested loops to manipulate memory blocks by either copying words, clearing words, or applying bitwise NOT and OR operations between a source and destination using a mask.
- `or_rows_with_mask`: Performs a nested loop to mask words from a source table using a bitwise NOT and AND operation before ORing them into a destination, then increments the pointers by specified offsets.
- `branch_with_mask`: As library code, it copies a word from register d0 to d2 and then branches to another location.
- `or_rows_with_single_mask`: Performs a nested loop to mask words from a source table using a bitwise NOT and AND operation before writing them to a destination, then increments the pointers by specified offsets.
- `store_word_and_branch`: Stores a word from d0 into d2 and then branches to another routine.
- `or_word_rows_stacked`: Ors source words into destination rows like its neighbour, with a stacked row-count argument.
- `setup_word_copy`: As library code, it calculates two destination pointers using values from a table and an offset, decrements a counter in d3, and if the counter is not negative and the destination pointer has not reached a limit, it saves several registers and branches to another routine; otherwise it returns.
- `copy_rows_to_destination`: Performs a nested loop to copy words from a source table to a destination, then increments the pointers by specified offsets.
- `transfer_word_and_branch`: Saves registers d1 through d3 and a3 onto the stack, copies a 16-bit value from register d0 into register d2, and performs a short branch to another routine.
- `zero_then_copy_rows_stacked`: Writes a zero word into the first column of each row, then copies source words into the remaining columns of rows whose source pointer differs from the reference.
- `apply_masked_bit_operations`: Performs nested loops to modify memory by applying bitwise NOT and AND or bitwise OR operations between source tables and a destination buffer.
- `or_bytes_with_mask`: Performs a nested loop to mask bytes from a source table using a bitwise NOT and AND operation before ORing them into a destination, then increments the pointers by specified offsets.
- `branch_with_mask_short`: As library code, it copies a word from register d0 to d2 and then performs a short branch to another location.
- `or_bytes_with_single_mask`: Performs a nested loop to mask bytes from a source table using a bitwise NOT and AND operation before writing them to a destination, then increments the pointers by specified offsets.
- `transfer_word_and_branch_2`: Saves registers to the stack, copies a 16-bit value from register d0 into register d2, and performs a short branch to another routine.
- `or_byte_rows_stacked_checked`: ORs source bytes into destination rows with a stacked row-count argument.
- `calculate_blitter_area_parameters`: Calculates and writes blitter control registers, including area size and mode, by performing arithmetic on values from a structure in a6 and updating hardware registers for destination, source, and area dimensions.
- `setup_blitter_parameters`: Calls a routine to wait for the blitter, sets the blitter destination and source pointers via hardware registers, and calculates a control word based on comparisons between pointers.
- `set_blit_control_word`: Adds a constant to register d5 and writes the result to the blitter control register, then branches to another location.
- `issue_blit_size_per_row`: Writes the blitter control word and the A-data word for each row, choosing among three control words according to whether the row's source pointer matches the reference.
- `copy_segments_with_nested_loops`: As library code, it extracts parameters from a table, calls another routine, and then uses nested loops to copy words from a source to a destination, adjusting pointers by specified offsets.
- `start_blit_rows_fixed_control`: As library code, it sets the blitter's control register and size for each of a counted set of rows, programming the B and D pointers and modulo for each row, and waits for the blitter to be idle before starting.
- `copy_3d_block_data`: Packs a control structure into a destination buffer using values from a structure in a6, then performs a nested loop to copy word data from a source buffer to a destination buffer based on dimensions stored in the packed structure.
- `start_blit_rows_from_stack`: As library code, it packs a control word from two stored values, then loops over a row count writing the blitter's control register, pointers and size for each row, and waits for the blitter to be idle beforehand.
- `dispatch_byte_command_handlers`: Checks a bit in a structure at offset 961; if set, it clears bits in a word at offset 16 of a structure in a0.
- `update_rectangle_fields`: As library code, it checks a byte and a long at specific offsets in a structure pointed to by a0; if certain conditions are met, it copies four words from a pointer in a0 to a structure in a5, otherwise it clears several fields in a5 and sets a value at offset 36 and offset 34 using values from the pointer.
- `load_rectangle_block_fields`: Copies four rectangle words from an attached block into the state block, reads the row count and width from the record, and multiplies the width by the row count when a flag bit is set.
- `branch_to_initializations`: Performs a short branch to another routine.
- `execute_conditional_routine_sequence`: Calls two routines and then calls a third routine if the word at offset 8 of the structure in a6, when multiplied by 2 and decremented by 1, is greater than or equal to 0.
- `branch_on_bit_test`: Performs a relative branch to another routine based on the state of bit 3 in a byte at offset 962 of the structure pointed to by a6.
- `branch_to_interpolation_logic`: Performs a short branch to another routine that manages interpolation state and conditionally calls a second routine to calculate interpolation steps based on the zero flag.
- `init_record_flags`: Clears several state fields, copies a pointer from the record into the state block, and sets two flag bytes when that pointer is nonzero and a record bit is set.
- `execute_library_initializations`: As library code, it calls three different routines and then returns zero.
- `run_routines_size_check`: As library code, it calls two routines, then calls a third routine if a value derived from a word at offset 8 of the structure in a6 is non-negative, and finally returns zero.
- `copy_long_table_into_state`: Reads a count word from a table pointer in the record and stores it in the state block, then copies that many 32-bit entries from the table into the state block, stopping once the count runs out.
- `copy_long_pairs_into_state`: Stores a count word from a table pointer in the record into the state block, then copies the table's 32-bit entries into it two at a time.
- `load_word_to_register`: Loads a word from an offset of 8 in the structure pointed to by a6 into d7.
- `find_coordinate_min_max_counted`: Scans a list of coordinate pairs whose count is held in a register, tracking the running minimum and maximum for each channel.
- `update_structure_pointers`: Clears a word at offset 942 of a structure in a6, then reads a byte from a0 and a pointer from a0 to update offsets 922 and 942 of the structure in a6.
- `modify_structure_and_branch`: As library code, it writes a word to an offset of a6, performs bitwise operations on a word from a0, writes a word to another offset of a6, and then either branches to another routine or adds 4 to a7 and branches to another location.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### Captive II - Liberation (1992)(Mindscape)(Disk 1 of 5)[cr HLM].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_hardware_and_audio`: Sets the bit 15 of the DMA control register, performs a series of library calls to initialize hardware and buffers, updates a counter in a loop, and eventually stops music playback and closes a library.
- `search_and_copy_data`: Iterates through a source buffer to find a matching byte from a target list, then copies seven 32-bit words from the target list into a destination buffer at an offset of 276 from a pointer.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d4.
- `initialize_hardware_and_buffers`: Processes a list of bytes to determine a count, then clears a series of 32-bit words in a buffer, sets a bit in the CIAA_PRA register, and clears several bytes and a word in a global structure.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_status_byte`: Increments a byte at an offset of 36 in a structure pointed to by a6, compares a byte from a table to a value, and either clears the byte at offset 37 of that structure or calls another routine twice depending on the comparison and a test of a byte at offset 38.
- `update_audio_channels_and_play_sound`: Updates the four Amiga audio channel pointers by calling a routine for each channel and then calculates a value based on a byte from a table to update a 16-bit value in a global variable.
- `process_input_command_and_update_state`: Parses a command byte from a structure in a6 to determine an action, updates various fields in that structure and a target structure in a5 by fetching data from a table, and performs hardware register updates including DMA and video pointer modifications.
- `dispatch_nibble_command`: Extracts a nibble from a byte at offset 2 of a structure and branches to different subroutines based on its value.
- `update_structure_value`: Copies a 16-bit word from offset 16 of a structure in a6 to offset 6 of a structure in a5.
- `search_table_for_value`: Searches a table for a value matching a field in a structure, updates the structure with the found value, and sets a flag based on a comparison with another field.
- `update_scaled_value_2`: Calculates a value by fetching a byte from a table using an index derived from a byte at offset 27 of a structure, or by applying a bit-shift.
- `update_scaled_value`: Calculates a value by fetching a byte from a table using an index derived from a byte at offset 27 of a structure, or by applying a bit-shift and sign-extension to that byte, then scales this result by a factor derived from offset 26, adds it to a word at offset 16, and stores the result at offset 6 of another structure; finally, it updates the byte at offset 27 using a value derived from offset 26.
- `dispatch_input_command_data`: Reads a command byte from a structure and branches to specific subroutines based on its lower and upper nibbles.
- `extract_nibble_to_structure`: Extracts the lower nibble from a byte at offset 3 of a structure and stores the result in a byte at offset 18 of the same structure before returning.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `modify_and_test_byte`: Sets the carry bit in a byte through a pointer and then tests a bit at an offset of 762 from that same pointer.
