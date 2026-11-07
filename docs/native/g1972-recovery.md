# Game 1972 (captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01972). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (920240 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 26 raw-word lines |
| Functions named and described | pass, 416 of 416 named (0 author symbols), 416 of 416 described (20 by rule, 34 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1787 of 1787 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8748 of 8748 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 605232 |
| code | 25862 |
| data | 128125 |
| padding | 3325 |
| unused | 157696 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| endcli | 2 | 117 | 2 |
| Liberation | 5 | 227 | 3 |
| delirium | 2 | 267 | 8 |
| PlayASP | 4 | 10 | 1 |
| Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf:HardDisk_Install | 3 | 10 | 1 |
| math.library | 1 | 807 | 27 |
| tridee.library | 1 | 2181 | 93 |
| vector.library | 1 | 3569 | 184 |
| Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 516 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 655 | 38 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 162 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 173 | 15 |
| logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf | 1 | 11 | 2 |
| logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf | 1 | 11 | 2 |
| logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf | 1 | 11 | 2 |

## Cases handled

- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a3]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf:StartUp.asp: high-entropy data file (7.84 bits/byte), classified as packed data
- runtime segment an address: 406 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 375 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 50 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a3]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf: 9 executed PCs whose RAM was overwritten before any snapshot match its bytes 230976-231072: recovered as a unit run at an address
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a3]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf: 9 executed PCs whose RAM was overwritten before any snapshot match its bytes 230976-231072: recovered as a unit run at an address
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a3]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf: 9 executed PCs whose RAM was overwritten before any snapshot match its bytes 230976-231072: recovered as a unit run at an address
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a3]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf: 3 logged loads cover 0 bytes of the image
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a3]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf:c/PlayASP: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a3]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf:HardDisk_Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `call_library_twice_then_halt` (runtime segment an address): Calls a library routine through an indirect pointer twice, then enters an endless loop with no exit. Kind: loader/protection trap. Pass path: the code following the infinite loop (which is unreachable in this snippet).

## Functions


### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: When a destination pointer is supplied, copies sixteen 32-bit values from a fixed offset in the base block into it.

### Liberation

- `follow_link_chain_and_call`: Program entry point.
- `scan_hunks_and_clear_cache`: Scans a sequence of blocks for a four-byte marker.
- `unpack_bitstream_data`: Decodes a bit-packed compressed stream.

### delirium

- `load_executable_hunks`: Parses the hunks of an AmigaDOS executable by reading each hunk type code.
- `count_down_register_loop`: Repeatedly decrements a 16-bit count until it passes zero, so it performs a fixed busy-wait with no other effect.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_run_length_bytes_backwards`: Decodes run-length coded bytes into a buffer, filling from a top pointer downwards.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bits_through_buffer`: Shifts a bit buffer across words, loading a new word when the buffer runs out.
- `modify_byte_and_flag`: Performs a negation and addition on a byte through a pointer and modifies a bit in another memory location.

### PlayASP

- `traverse_linked_list`: Walks a linked list by following pointers until a null value is reached, then calls a function from a table.

### Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf:HardDisk_Install

- `traverse_linked_list_alt`: Follows a chain of pointers through a list and executes a routine found at an offset within the final element.

### math.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `store_pointer_pair_in_struct`: Library code: writes two 32-bit values from the address registers into a structure addressed by the data register, at offsets 36 and 40.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_counter_and_call`: Library code: decrements a 16-bit counter in the library's block.
- `finish_or_mark_pending_request`: Library code: when a counter is zero, reads a value from the request block and makes two library calls through the library base.
- `release_request_block_memory`: Library code: reads a value from the request block, then calls the system Remove and FreeMem routines through the library base to unlink and free it, returning the value.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `lookup_word_table_entry`: Library code: returns the 16-bit entry from a 512-entry table at the index given by the low nine bits of the data register.
- `transform_point_with_scale_offset`: Library code: scales a two-word point by a ratio taken from the structure's fields, dividing by the sum of two other fields, then adds offset fields and stores the result in place.
- `swap_word_halves_store`: Library code: swaps the upper and lower 16-bit halves of a register value and stores the result at the address held in the address register.
- `scale_point_by_divided_fields`: Library code: multiplies each of two stored coordinates by a scale field, divides by the sum of a shared field and another field, then adds offset fields.
- `return_from_subroutine`: The routine performs a series of operations before executing a return instruction.
- `return_from_subroutine_2`: This routine performs a series of operations before returning to its caller.
- `return_from_subroutine_3`: The routine performs a series of operations before returning to its caller.
- `split_result_hundreds_remainder`: Library code: calls a subroutine with an all-ones argument, then returns its value divided by 100 in the upper word and the remainder in the lower word.
- `update_linear_congruential_state`: Library code: updates a 16-bit state value with a multiply-and-add step.
- `build_rotation_matrix_from_angles`: Library code: looks up sine and cosine style values from a 512-entry table for three angle arguments, then writes a three-by-three matrix of 16-bit values built from products of those values.
- `build_rotation_matrix`: Reads sine and cosine style entries from a fixed table for three angle inputs and fills a nine-entry fixed-point 3x3 rotation matrix into the output buffer.
- `return_unchanged_registers`: Performs no computation and returns the caller's register values unchanged, so it behaves as an empty stub in the math library.
- `multiply_matrices_3x3_fixed`: Multiplies two fixed-point 3x3 matrices by accumulating each row-column dot product, shifting the sum right by 14 bits, and writing nine result entries; all-zero rows are stored as zero.
- `compute_integer_square_root`: Computes the integer square root of a 32-bit value by a binary search over 16-bit candidates, returning zero for a zero input and one for inputs up to three.
- `refine_square_root_search`: Narrows a 16-bit candidate root by bisection, testing each midpoint's square against the target until the two bounds are adjacent, and returns the lower bound.
- `return_constant_one`: Returns the constant one with no other work, so the caller receives a fixed result without any computation.
- `interpolate_global_table_value`: Takes the absolute value of a 16-bit input, bisects a fixed 128-entry sorted table to find its bracketing entries, linearly interpolates between them, and restores the sign.
- `interpolate_passed_table_value`: Bisects a sorted 16-bit table passed by pointer to find the entries bracketing an input, interpolates the fraction between them, and restores the sign taken from the stacked argument.
- `interpolate_between_table_entries`: Computes the fractional position of an input between two table entries by dividing the scaled difference by the table span, negating the result when the sign flag is set.
- `compute_clamped_triple_product`: Builds two edge vectors from three stacked point coordinates, takes their cross product, dots it with a third vector, scales by values from a structure, and clamps the result to that structure's limit.

### tridee.library

- `return_minus_one_status`: Returns the value minus one and nothing else; it is the program entry stub of the tridee library and reports a failure status to any caller.
- `store_two_fields_in_record`: Writes two register values into fixed offsets 36 and 40 of the record addressed by the first register, returning nothing.
- `increment_counter_and_clear_flag_2`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_use_count_and_release`: Decrements the 16-bit use counter in the record and, when the counter reaches zero and a pending-release bit is set, calls the routine that releases the record.
- `release_node_when_unused`: If the record's use counter is zero, removes the node from its exec list and frees its memory; otherwise it sets a pending-release bit in the record so the release happens later.
- `remove_and_free_node`: Removes a node from its exec list and frees its memory through exec calls, then returns the pointer value stored in the record.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `increment_library_count`: Increments a global counter and calls a function pointer based on a bit flag in a provided address.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `decrement_library_count`: Decrements a global counter and calls a function pointer if the counter reaches zero and a specific condition is met.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `initialize_entry_table`: This library routine allocates a memory block for an entry table and initializes its header and field values based on a record's size and content.
- `allocate_entry_table_block`: Allocates a memory block for an entry table through exec's AllocMem using a size passed in, then initializes the header fields from the input record and returns the block, or the original argument if allocation fails.
- `initialize_entry_table_fields`: Stores the base pointer and derives the entry count from the record's size value, dividing by 52 bytes per entry, then writes the count and scaled offsets into the structure header.
- `forward_stack_argument_after_call`: Calls another routine and then returns the value passed on the stack, forwarding that argument unchanged.
- `release_memory_blocks`: Clears a field in a structure and calls different function pointers based on a bit flag to manage memory or resources.
- `free_two_exec_blocks`: Frees two memory blocks through exec's FreeMem, choosing between the direct call and an alternate entry point by a flag bit in the caller's state.
- `free_exec_memory_block`: Calls exec's FreeMem on the block it is given and then calls a further routine to finish the release.
- `allocate_and_initialize_entry`: This library routine allocates a memory block based on a count and initializes several fields within that block using values derived from the input arguments.
- `allocate_entry_table_from_count`: Allocates a table block through exec's AllocMem for the given count and writes the header fields that point into it, returning the block or the original argument when allocation fails.
- `return_stack_argument`: Returns the value passed on the stack without modification.
- `reset_record_bounds_fields`: Copies a stored 32-bit value into the record, clears several fields to zero, and sets four 16-bit bound fields to the signed maximum and minimum values.
- `project_points_with_depth`: Walks a list of three-coordinate points, adds the record's translation terms, and projects each point by dividing scaled x and y by a depth-derived term, storing the results in the entries that follow.
- `compute_dot_product_three_components`: Computes a dot product of three 16-bit inputs with the record's three weight fields, skipping any input that is zero, and packs the sum into fixed-point form by shifting.
- `project_point_xy_perspective`: Adjusts the x and y words of a point by dividing each scaled coordinate by a depth term built from the point's z word and record fields, then adding the record offsets.
- `copy_object_state_from_record`: Copies header values and eight words from the input record into the target object, then uses the record's flag bits to return a status code and to copy saved position fields into the object.
- `save_state_by_flag_bits`: Returns bit two of the record flags when it is set; otherwise, when the low flag bits are set and bit four is set, it copies five state values into a saved area of the object and returns bit four.
- `save_state_when_flagged`: Returns bit four of the record flags, copying five state values into a saved area when the low flag bits and bit four are set, and returns zero otherwise.
- `return_immediately`: Returns at once with no work and no effect.
- `return_void`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `store_packed_argument_words`: Stores the two halves of the first 32-bit argument and the low half of the second argument into three 16-bit fields of the record at offsets 108 to 112.
- `load_object_from_descriptor`: Copies the flags, transform and bounds fields from an input descriptor into the object, resets two bounds words, runs follow-up routines chosen by flag bits, and writes two saved values back to the descriptor.
- `interpolate_node_list_when_flagged`: When a flag bit in the object is set, walks a linked list of nodes and, for each node with the matching flags and a valid index, computes a fractional value between two bounds and adds it to the node's byte field.
- `interpolate_node_list_values`: Walks a linked list of nodes and, for each node with matching flags and a valid index, computes a fractional value between two bounds and adds it to the node's byte field.
- `stub_return_without_effect`: Contains only a return, so it does nothing when called.
- `accumulate_node_bounds_flagged`: When the object's flag bit eight is set, walks a linked list of nodes and, for each node with a non-negative result, widens the object's four bound fields to include that node's extents and records the node pointer in the list slot.
- `accumulate_node_bounds_in_list`: Walks a linked list of nodes and, for each node with a non-negative result, widens the object's four bound fields to include the node's extents and stores the node pointer in its list slot.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a4, then returns.
- `interpolate_node_byte_by_flags`: Interpolates a byte value for one node between its index bounds, applying limits chosen by the object's flag bits, and stores the result in the node's byte field.
- `scale_node_byte_when_flagged`: When a flag bit in the object is set, computes a scaled byte from the input value and the node's bounds, adds the node's base byte, clamps negative results to zero, and stores it in the node's byte field.
- `stub_exit_without_action`: Contains only a return and performs no action.
- `stub_return_no_action`: Contains only a return and performs no action, so it has no effect on the caller.
- `store_and_scale_node_byte`: Stores the input byte in the node when a flag bit is clear, then computes a clamped scaled byte from the input and the node's bounds and writes it into the node's byte field.
- `compute_node_byte_from_input`: Computes a clamped byte from an input byte minus a scaled term built from the input word and the node's bounds, then stores it in the node's byte field plus the node's base byte.
- `compute_node_byte_from_word`: Computes a clamped byte from an input word and the node's bounds, adds the node's base byte, and stores the result in the node's byte field.
- `return_void_2`: This library routine is empty and performs no operations before returning.
- `compute_scaled_node_byte`: Computes a byte from an input word scaled by a bounds-derived ratio, adds the node's base byte, clamps negative results to zero, and stores the result in the node's byte field.
- `compute_node_extent_bounds`: Finds the minimum and maximum 16-bit field across the node's entry list, shifts both by a scale taken from the node's flags, stores them in the node, and widens the object's bound fields to include them.
- `compute_entry_extent_bounds`: Finds the minimum and maximum 16-bit field across the entry list, applies the node's scale shift, stores the extents in the node, and widens the object's bound fields to include them.
- `clip_edge_against_bounds`: Tests the entries of a list against the object's bound limits and, where an entry crosses them, takes a new entry from a pool, copies its coordinates and flags, and links it into the list.
- `clip_point_against_bounds`: Tests a point argument against the entry list's bound limits and, when it lies outside, takes pooled entries, copies the entry coordinates, and links the new entries into the list.
- `test_point_inside_radius`: Tests a stored 2D vector against a squared radius; when the threshold test passes it swaps two stored vectors and returns the squared length, otherwise it rotates the vector and returns a fixed-point quotient.
- `divide_rotated_fixed_point`: Rotates a 2D vector by a fixed-point angle pair and computes a fixed-point quotient by long division with bit-count scaling, returning minus one when the divisor is zero.
- `divide_fixed_product_by_difference`: Computes a long-division fixed-point quotient of two scaled 16-bit values by a difference, returning the quotient with its shift count, or minus one when the difference is zero.
- `shift_and_divide_fixed`: Shifts a 32-bit value by a count and divides it by a 16-bit divisor using long division, returning the quotient scaled by that shift count.
- `compute_squared_length_ratio`: Rotates nothing but compares squared component lengths of the object's stored vectors, checks the bound limits, clears a status bit, and returns a fixed-point ratio of the two squared lengths, or minus one when the bound test fails.
- `return_squared_length_ratio`: Computes a fixed-point ratio of two squared lengths taken from the stored vector and the input vector, returning the low 16 bits of the quotient.
- `compute_scaled_squared_ratio`: Builds scaled squared components from the stacked arguments and the record, then returns the fixed-point quotient of their products divided by their sum.
- `compute_fractional_difference`: Computes the scaled difference of two 16-bit values, divides it by the first value, and returns the low 16 bits of the quotient.
- `rotate_point_and_clip`: Rotates a 2D point by a fixed-point angle pair, stores the rotated components in the record, applies the bound test and flag handling, and returns the stored result or minus one.
- `return_minus_one_fallback`: Returns minus one together with the record's stored word, signalling failure to the caller.
- `compute_squared_distance_3d`: Subtracts two stored three-component points component by component and returns the sum of their squared differences, made non-negative.
- `store_point_difference_sum`: Writes the input word and a zero word into the output buffer, computes the difference and sum of two stored coordinate pairs against the record's offset values, and clears a status bit in the record.
- `compute_entry_min_max_bounds`: Gathers the minimum and maximum x and y words over the stored entries, storing them in the record's bound fields and clearing a status bit, or returning minus one when the bound test fails.
- `stub_return_minus_one`: Returns minus one with the second argument's stored word and performs no other work.
- `clip_polygon_against_bounds`: Clips a polygon's vertex list against the object's bound limits, taking new vertices from a pool where edges cross the limits, copying their coordinates, and linking them into the output list.
- `clip_vertices_to_threshold`: Walks a vertex list against a threshold value and, where vertices cross it, takes new vertices from a pool, copies their coordinates, and links them into the output list; it returns minus one when the pool is exhausted.
- `clip_vertices_against_plane`: Clips a vertex list against a threshold value, creating pooled vertices where edges cross it, and writes the surviving vertex indices back into the list.
- `clip_and_measure_squared_length`: Clips a vertex list against a threshold and returns the squared length of the first resulting segment, or minus one when the vertex pool is exhausted.
- `return_minus_one_word`: Returns minus one together with a stored word from the object, reporting failure without other work.
- `clip_edge_pair_to_bounds`: Tests a list of entries against the object's bound limits and, where entries cross them, creates pooled entries, copies their coordinates, and links them into the list, returning a stored result word.
- `clip_segment_to_range`: Clips a segment's entries against the range given by the argument and the object's bound limits, creating pooled entries where they cross and linking them into the output list, then clears a status bit.
- `clip_list_to_limits`: Walks a list of entries against two limit values, creates pooled entries for those that cross, copies their coordinates, and links them into the output list.
- `project_point_into_record`: Stores the input word and offset-adjusted coordinates into the record, computes two projected bound pairs by subtracting and adding the reference coordinates, and clears a status bit.
- `compute_entry_bounds_and_test`: Computes the minimum and maximum words over the stored entries into the record's bound fields, then returns minus one when a cross-product sign test on the end entries holds, otherwise clears a status bit.
- `clip_polygon_to_threshold`: Clips a polygon's entries against a threshold on the stored coordinates, takes pooled entries where edges cross it, copies their coordinates, and links the new entries into the output polygon.
- `clip_list_against_threshold`: Walks a list of entries against a threshold, taking pooled entries where edges cross it, copying their coordinates, and linking the new entries into the output list.
- `clip_vertex_list_to_pool`: Clips a vertex list against a threshold, drawing new vertices from a pool wherever the list crosses it, and links the results into the output list.
- `call_through_stub`: Calls one other routine and returns, with no other work of its own.
- `sort_entries_by_key`: Repeatedly reorders a singly linked list of entries by their 16-bit key word, swapping neighbours whose keys are out of order until the list is sorted.
- `test_overlapping_entry_pairs`: Walks a list of entries, comparing each entry's bounding extents with the next; for overlapping pairs it computes midpoint values and sign-tests them against the object's stored data.
- `find_overlapping_entry`: Searches the entry list for the first entry whose bounding extents overlap the given entry, then computes the midpoint values of the matching pair.
- `split_polygons_by_plane`: Computes a polygon's plane normal and offset from the cross product of its edge vectors, then classifies the vertices of the following polygons by the sign of their distance to that plane.
- `classify_polygons_by_plane`: Classifies the polygons in a list by the sign of their vertices' distance to a stored plane, using the plane's normal and offset.
- `select_polygons_by_plane_side`: Walks the polygon list, computes each polygon's centroid, and collects the polygons that lie on the matching side of the stored plane into the output list.
- `classify_vertices_by_edge_sides`: Computes the 2D edge vectors of two polygon edges, then loops over the following vertices and classifies each one by the sign of its cross product against those edges, copying coordinates into the work area as it goes.
- `set_color00_from_edge_sign`: Computes 2D cross products of polygon edge vectors and writes the background colour register with one of three fixed values (15, 240 or 3840) depending on the sign combination found.
- `insert_polygon_into_sorted_list`: Inserts a polygon into a linked list at the position given by the caller, relinks its neighbours, and writes the background colour register with one of several fixed values according to the edge signs.
- `link_entry_into_list`: Links an entry into a list at the position given by the caller, then loops over the remaining entries, copying each entry's coordinates into the work area.
- `swap_list_links_and_copy`: Swaps two list links, then copies each following entry's coordinates into the work area, adding 100 to the middle value when the entry has fewer than three vertices.
- `return_from_subroutine_4`: This library routine is empty and performs no operations before returning to the caller.
- `interpolate_crossing_coordinate`: Orders two endpoints by their first word, then linearly interpolates the second word at the position where the first word crosses the given value, returning the interpolated word.
- `return_from_subroutine_5`: This routine is empty and performs no operations before returning.

### vector.library

- `return_minus_one_status_2`: Returns minus one as its result and performs no other work; it is the program entry stub of the vector library and reports a failure status when started directly.
- `store_vector_components`: This library routine stores two 32-bit values into a structure at specific offsets.
- `increment_use_count_clear_flag`: Increments the 16-bit use counter at record offset 32 and clears the pending-release flag bit at offset 34.
- `decrement_and_check_resource`: This library routine decrements a short integer at an offset within a structure and calls another routine if the resulting value is zero and a specific bit is set in a status byte.
- `release_vector_resource`: This library routine checks a status field in a structure and either executes two function pointers or modifies a bit flag before returning a value.
- `release_resource`: This library routine calls two function pointers and returns a value previously stored in a structure.
- `return_zero`: This library routine returns zero.
- `update_vector_structure`: Calculates new coordinate values based on input registers and bitwise flags, then writes these values into a structure pointed to by a retrieved pointer.
- `allocate_dimensioned_record`: Calculates a required memory size based on input dimensions and flags, then allocates a memory block and populates it with specific geometric or structural metadata.
- `allocate_record_with_dimensions`: Allocates a record through exec's AllocMem and fills its size and dimension fields from the arguments, allocating a second block when the flag bits request it.
- `allocate_record_and_link_block`: Allocates a block through exec's AllocMem, links it into the record's pointer fields, and sets dimension and offset fields from the arguments, returning zero when allocation fails.
- `pass_through_function`: This library routine simply calls another routine and returns the value it produces.
- `fill_row_pointer_table`: Fills a table of pointers with addresses separated by a stride computed from two size values, writing one entry per row.
- `fill_cumulative_offset_table`: Writes a running total of a step value into each 16-bit entry of a table, starting from zero, for the count stored in the record.
- `fill_arithmetic_word_sequence`: Fills a 16-bit array with an arithmetic sequence that starts at the first value and adds a fixed step for each entry, for the count given in the argument.
- `execute_conditional_function_2`: This library routine checks a bit flag in a structure and executes one of two function pointers based on that flag.
- `execute_conditional_function`: This library routine retrieves a pointer from a global variable and executes a function at an offset from that pointer, potentially executing a second function based on a bit test of a byte at a different offset.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `walk_flagged_entry_list`: Walks a linked list of entries, skipping entries with a status bit set, processes the entries whose flag masks match, and returns a combined status value.
- `gather_indexed_entry_coordinates`: Walks a table of indices for an entry, reads the 16-bit coordinates each index points to from an indirection table, and copies them into the work area until a negative index ends the list.
- `dispatch_handler_and_walk_list`: Calls a handler selected from a jump table by an index, adds its result to a running total in the object, then walks a linked list of entries and marks each as processed.
- `process_deferred_entry_list`: Takes the object's deferred entry list, walks each entry's linked list applying flag-based processing, and returns a stored total together with a stored value.
- `stub_conditional_call_fragment`: A short fragment of conditional calls decoded from a gap between routines; it has no clear standalone behaviour beyond those calls.
- `stub_conditional_call_chain`: Calls a chain of routines chosen by a flag bit in the object and returns one when the chain completes.
- `call_if_not_zero_flag`: Calls one routine and, when the preceding comparison leaves the zero condition clear, calls a second routine.
- `call_if_zero_flag`: This library routine executes another routine and then conditionally executes it a second time if the zero flag is not set.
- `call_optional_routine_return_one`: Calls a routine and, when the preceding comparison leaves the zero condition clear, calls another routine and returns one.
- `copy_entry_coordinates_to_buffer`: Copies the 32-bit coordinate word of each entry in the object's list into the buffer, reading through the indexed entry table or directly according to a flag in the object.
- `copy_indexed_coordinate_longs`: Copies the 32-bit coordinate value at offset 12 from each pointed-to entry into the output array, for the count given in the argument.
- `wait_then_set_pixel_bits`: Waits for the blitter to finish, then walks coordinate pairs and, for each point inside the object's bounds, sets the matching bit in a 16-bit mask word.
- `set_pixel_bits_in_bounds`: Walks coordinate pairs and, for each point inside the object's bounds, sets the matching bit in a 16-bit mask word, without waiting for the blitter.
- `blend_masked_plane_words`: Writes words row by row into a bitplane buffer, combining each word with a mask from a stored table through an AND and an OR with the stored pattern word.
- `stub_gap_fragment_bad_data`: Decoded from a gap between routines, so this body is not a reliable routine; it clears a field, makes two calls and then branches on a count.
- `stub_gap_fragment_unreached`: Decoded from an unreached gap between routines, so the body is not a reliable routine; it stores a word in the object, makes two calls and then selects one of two paths by a size check.
- `step_fixed_point_line_slope`: Computes a fixed-point slope from two 16-bit endpoints by dividing the scaled difference by a height value, with sign handling, and steps along the edge; decoded from an unreached gap, so it may be incomplete.
- `clip_span_to_bounds`: Clamps a span's start and end coordinates to the object's bound limits and skips rows that fall outside the bound range; decoded from an unreached gap, so the body may be incomplete.
- `clip_span_pair_to_bounds`: Adjusts a pair of span coordinates by offsets from the object and clamps them to the object's bound limits, decoded from an unreached gap.
- `test_span_against_bounds`: Tests a span's extent against the object's bound limits and returns zero when the span lies outside them; decoded from an unreached gap.
- `step_span_accumulators`: Advances several accumulator words along a slope by repeatedly adding step values and decrementing a count on each pass; decoded from an unreached gap.
- `walk_span_edges_by_slope`: Steps two edge accumulators along a slope, checks each step against the object's stored limits, and calls routines to emit span records; decoded from an unreached gap, so it may be incomplete.
- `advance_span_edge_pair`: Advances a pair of span edges by one step, updating their accumulator words and writing edge coordinates into the object; decoded from an unreached gap.
- `test_span_bounds_when_flagged`: When the object's flag word is nonzero, returns zero for spans lying outside the bound limits, otherwise calls a further routine; decoded from an unreached gap.
- `return_zero_constant`: This library routine is an empty routine that returns zero.
- `return_minus_one_when_flagged`: Returns minus one when the object's flag word and the record's word are both negative, otherwise returns zero.
- `clamp_bounds_to_window`: Clamps four stored bound words of the record to the object's window limits, keeping the tighter value of each pair and storing the results in the object.
- `classify_box_overlap`: Compares the record's bounding box with the object's bound limits, returning minus one when the box lies entirely outside them, and otherwise continuing with edge clipping steps.
- `clip_span_list_to_range`: Walks a list of span pairs, clipping each pair's start and end against the stored range and writing the surviving spans into the output.
- `clip_span_list_to_limit`: Walks a list of span pairs, clipping each pair against a limit taken from the object and writing the surviving spans into the output.
- `return_zero_2`: This library routine returns zero to the caller.
- `emit_span_when_inside`: Increments the object's span counter and, when the record's box lies inside the object's bounds, emits a span entry carrying its extents.
- `copy_coordinates_and_test_box`: Copies coordinate pairs from the entries into the buffer, then tests the record's bounding box against the object's bounds and stores the result.
- `clip_coordinate_pairs_to_limit`: Swaps the two halves of each coordinate pair, clips the pairs against a limit, and writes the surviving pairs into the output buffer.
- `clip_coordinate_pairs_to_range`: Clips coordinate pairs against a range taken from the arguments, writing the surviving pairs and their swapped halves into the output buffer.
- `store_argument_in_object_field`: Stores its stack argument as a word in the object's field at offset 36.
- `compute_entry_extent_range`: Walks the object's entry list and finds the minimum and maximum of each entry's 32-bit pair, returning the extremes packed together.
- `fill_span_with_fixed_step`: Fills a buffer with values that step by a fixed-point increment computed from the span width, for the count held in the object.
- `write_span_pattern_words`: After clipping a span against the object's bounds, writes span pattern words through register-style stores, looping over the span's rows.
- `fill_span_rows_with_step`: Fills buffer rows with values that step by a fixed-point increment derived from the span width, for each row count held in the object.
- `wait_then_step_edge_pair`: Waits for the blitter to finish, then steps two edge accumulators and stores their coordinate and step words for each row in a list.
- `blit_span_with_masks`: Waits for the blitter, programs its first-word mask and the source and destination modulo registers, then starts a blit for each row of the span.
- `blit_rows_with_modulo`: Waits for the blitter, programs its A and B modulo registers and a size-related control register, then blits rows whose count depends on the operands.
- `blit_block_with_size_register`: Programs the blitter's A and B data, control, pointer and size registers for each block of the copy, then starts the blit.
- `prepare_span_fill_parameters`: Computes a span width from the input pair, compares the object's extents against a threshold of 48, and stores the resulting setup words for the span fill in the object.
- `prepare_span_fill_with_offset`: Computes a span width from the input pair, compares the object's extents against a threshold of 48, and stores the setup words for the span fill, offset by one line.
- `wait_then_prepare_span_fill`: Waits for the blitter to finish, then computes and stores the span fill parameters from the input pair against the object's extents.
- `write_span_pixels_with_pattern`: Walks an input coordinate list and writes span words for each entry, applying a pattern table selected by the object's flag and a mask word for each row.
- `combine_words_with_mask`: Combines each destination word with a mask by an AND, ORs in a value, and stores the result, looping over the row counts held in the registers.
- `write_span_pixels_by_table`: Walks coordinate pairs from the input and writes span words for each one, using per-row table entries from the object.
- `mask_and_combine_row_words`: Clears masked bits of each destination word, ORs in a value, and steps through the rows of the span until the row count runs out.
- `compute_row_span_offsets`: Computes a row offset from the input value and the object's stride, and selects a source pattern according to the object's flag bits before the span rows are written.
- `write_row_spans_from_list`: Walks a list of span coordinates and writes the span words for each row, choosing the write path from a flag in the object.
- `write_span_words_by_nibble`: Splits span coordinates into 16-pixel word groups and 4-bit remainders, reads a table entry for each group, and writes the combined pixel words.
- `merge_masked_words_into_plane`: For each word in a list, clears the destination bits selected by a mask, then sets them from the source word, stepping through the rows.
- `merge_masked_words_row`: Clears the destination bits selected by a mask and sets them from the source word for each word in a row, stepping through the rows of the span.
- `write_span_pixels_from_list`: Walks a list of span coordinates and writes the span pixel words for each entry, applying the object's pattern table.
- `set_masked_row_words`: For each row, clears the destination bits selected by a mask, ORs in a value, and steps through the destination until the row count runs out.
- `blit_rectangle_fill`: Waits for the blitter, computes width and height from the object's bounds, and programs its first-word mask, control, modulo, destination pointer and size registers to blit the rectangle.
- `wait_and_copy_rows`: Polls the blitter-busy flag, then copies coordinate row words from the input into the blitter's data registers, looping once per row.
- `program_blitter_line_draw`: Programs the blitter's line-mode control words and pointers for each line segment in the object's span list, waiting for the blitter between segments.
- `blit_line_segments_loop`: Waits for the blitter between repeated line segments, steps the coordinate counters by 64, and programs the blitter registers for each segment.
- `set_blitter_registers_from_bounds`: Polls the blitter-busy flag twice, then stores the coordinate, modulo and step words into the blitter register block.
- `step_blit_rows`: Loops over rows, stepping the source counter by 64 each row, and writes row words through the object's table, stopping when the row count is exhausted.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `compute_pixel_word_address`: Computes a byte address and bit offset for a pixel from the object's stride or row table and the coordinate, returning both packed into one result.
- `compute_row_byte_address`: Computes a row byte address from the input row and the object's stride, plus a column byte offset derived from the object's bounds.
- `check_table_byte_flag`: Reads a byte from a table at the given index and returns without changing any state; the check has no effect on the caller.
- `return_from_subroutine_6`: This routine executes a single return instruction to exit the current function and return control to the caller.
- `return_from_subroutine_7`: performs a standard subroutine return to the caller.
- `decode_bitplane_pixel_row`: Reads the colour source word for each pixel, decodes its low nibble through a stored table, and emits the bitplane words for the span, branching on a status byte.
- `rotate_and_store_words`: Rotates each source word by a bit count taken from the state, stores the rotated word, and accumulates the OR of all rotated words.
- `select_checker_pattern_masks`: Clears the object's status word, derives a difference flag from the colour bits, and selects alternating 32-bit checker masks according to the record's flag bit.
- `compute_pattern_masks_from_bytes`: Derives the pattern and mask words from two stored bytes, selecting alternating 32-bit masks, and clears the object's status word.
- `expand_bits_to_mask_words`: Expands each bit of a 16-bit input into a full word chosen from two stored values, writing one output word per bit.
- `expand_byte_bits_to_words`: Expands each bit of a byte into a word chosen from two stored values, writing one output word per bit.
- `select_gap_table_entry`: Decoded from an unreached gap; reads an entry from a table pointed to by the object, choosing the index by the sign of a stored word.
- `load_image_header_fields`: Reads header fields from an image block (width, a rounded row-word count, a depth byte and padding) and stores them in the object; decoded from an unreached gap.
- `scale_dimensions_by_fixed_factor`: Scales two dimension arguments by a fixed-point factor from the record, shifting right by 6 bits and substituting the unscaled value when the result is zero; decoded from an unreached gap.
- `store_sampling_parameters`: Stores its argument words into the object and derives a count of half the width or one less, chosen by a flag bit in the record; decoded from an unreached gap.
- `store_clip_parameters_from_flags`: Optionally calls a routine for the coordinates, then stores flag-selected parameter words into the object; decoded from an unreached gap.
- `scale_dimensions_by_record_flags`: Scales the input size values by a factor derived from the record's flag bits, substituting the unscaled value when the result is zero; decoded from an unreached gap.
- `scale_pair_by_ratio`: Computes a proportional value from the arguments by a multiply and divide, storing it in one of two words selected by flag bits; decoded from an unreached gap.
- `scale_offset_by_ratio_signed`: Scales a value by a ratio and adjusts the sign and offset of the result for negative inputs; decoded from an unreached gap.
- `return_zero_constant_2`: An empty routine that returns zero to the caller.
- `reset_object_state_fields`: Clears four state fields in the object and walks a table of entries from the record, setting up per-entry state; decoded from an unreached gap.
- `compact_duplicate_table_entries`: Expands a table of 16-bit offsets by adding a base value to each non-zero entry, then scans the 32-bit list of indices and, for every non-negative value, marks later matching entries as removed (all-ones) while decrementing a live count and advancing a running offset by a step value.
- `mark_repeated_table_entries`: Walks a list of 32-bit indices and, for each non-negative value, marks every later entry equal to it as removed by writing an all-ones value and decrementing the remaining count.
- `filter_matching_table_entries`: Scans a list of 32-bit indices and removes entries equal to a given key with an all-ones value and decrementing the remaining count, storing a fresh value into a parallel output table for each kept group.
- `clip_rectangle_against_bounds`: Tests a rectangle against a bounds record of four 16-bit limits and returns zero when it lies outside them or would be clipped to nothing.
- `return_zero_without_effect`: Contains only a return of zero, with no reads, writes or calls, so it is an empty stub in the library.
- `compute_scaled_bitmap_row_steps`: Copies origin and size fields from the source record into the state block, derives fixed-point step values from the width and height ratios, and fills a table of row offsets.
- `derive_second_bitmap_row_offsets`: Repeats the row step and offset setup for a second state block, negating the step when the flip flags are set.
- `return_last_bitmap_row_offset`: Computes the row step from a divisor passed in a register, builds the row offset table with the same flip handling as its neighbours, and packs bitmap bits into words row by row.
- `pack_rows_from_stride_offset`: Derives a starting offset as the base value plus the row index times the stride stored in the source record, then builds the row offset table and packs bitmap bits into 16-bit words per row.
- `accumulate_steps_then_pack_rows`: Adds a step value into an accumulator once per count, then writes the row offsets and packs bitmap bits into 16-bit words row by row.
- `pack_masked_bitmap_rows`: Writes a row of words with each bit cleared by a mask derived from the shift value, then runs the per-row bit-packing loop over the bitmap.
- `pack_bitmap_rows_for_blit`: Computes the word count and row offsets for a blit from the stored step values, calls the library's blitter wait helper, then packs bitmap bits into 16-bit words row by row.
- `pack_bitmap_rows_into_words`: Loops over the rows, reading source bits through an offset table and packing them into 16-bit words, then advances the source pointer by a step held in a register.
- `advance_and_pack_bitmap_rows`: Advances the source pointer by a row step and packs bitmap bits into 16-bit words for each row, looping until the remaining row count reaches zero.
- `composite_bitplane_rows_with_masks`: Walks a row index table and either copies source rows directly, zero-fills rows marked empty, ORs overlapping rows, or applies AND-NOT masks, depending on the mode flags stored in the state block.
- `merge_bitplane_rows_by_table`: ORs a list of source rows into a buffer, then composites destination rows from a row index table: rows marked zero are cleared, rows matching the current index are OR-ed in, and other rows get a masked merge.
- `overlay_bitplane_rows_by_table`: Walks the row index table and writes each destination row from source rows, clearing rows marked zero, OR-ing rows that match the current index, and applying a masked merge for the rest.
- `clear_and_merge_bitplane_rows`: Clears destination rows with an AND-NOT of the source, then walks the row index table: zero entries clear the row, matching entries OR the source row in, and other entries apply a masked merge.
- `merge_long_bitplane_rows_table`: Applies the row index table to a set of destination rows using 32-bit words: zero entries clear the row with an AND-NOT, matching entries OR the source row in, and other entries apply a masked merge.
- `overlay_long_bitplane_rows`: OR-es a list of source rows into a buffer in 32-bit words, then writes destination rows from the row index table: zero entries clear the row, matching entries OR the source in, and other entries apply a masked merge.
- `clear_and_merge_long_rows`: Clears destination rows in 32-bit words with an AND-NOT of the source, then walks the row index table: zero entries clear the row, matching entries OR the source row in, and other entries apply a masked merge.
- `copy_or_zero_bitplane_rows`: Copies each source row into the destination according to an index table and zero-fills rows whose index is zero.
- `clear_and_copy_word_rows`: Clears a run of destination rows, then for each table entry with a non-zero index copies a row of 16-bit words from the source bitplane.
- `clear_and_copy_long_rows`: Writes 32-bit rows from the source bitplane for each non-zero table entry and zero-fills the row for entries that are zero.
- `copy_or_clear_word_rows`: Walks the row index table and either zero-fills the destination row when the index is zero or copies a row of 16-bit words from the source bitplane.
- `setup_blit_edge_masks`: Derives the row and column offsets from the source record and the state block, builds a per-row address table, and computes the first and last word masks for a clipped rectangle.
- `setup_blit_offsets_from_registers`: Performs the same offset and edge-mask setup as its neighbour, but takes the origin and width from register inputs rather than the source record.
- `program_blitter_rows_after_wait`: Builds the per-row address table and the edge masks from register inputs, then either writes blitter control words, waiting for the blitter to finish between operations through the library's wait helper, or runs the per-row masked word loop.
- `blit_shifted_masked_bitmap`: Blits a bitmap into destination rows at a bit shift: for each row it clears the destination bits covered by a shifted mask word, then ORs in the shifted data word.
- `blit_shifted_rows_with_merge`: Repeats the shifted masked blit over a rectangle, choosing per row between clearing destination bits with a mask and OR-ing in a shifted data word.
- `blit_shifted_rows_with_clip`: Performs the shifted masked blit row by row with row and column counters that stop the loop when either runs out.
- `blit_shifted_rows_by_or`: Blits shifted source words into destination rows over a rectangle by OR-ing them in, clearing destination bits first only on rows whose source pointer differs from the marker.
- `blit_shifted_or_with_clip`: Performs a row-by-row shifted OR blit bounded by row and column counters.
- `blit_right_shifted_rows_merge`: Blits source words shifted right by a count derived from the negated shift value, clearing destination bits with an AND-NOT mask and then OR-ing the shifted words in on each row.
- `blit_right_shifted_masked_bitmap`: Blits a bitmap into destination rows at a right bit shift: for each row it clears the covered destination bits with a shifted mask word, then ORs in the shifted data word.
- `merge_right_shifted_rows`: Walks a rectangle of rows applying a right-shifted mask and data word to each destination row: it clears the covered bits with AND-NOT and ORs in the shifted source words.
- `blit_right_shifted_or_clip`: Performs a right-shifted row blit bounded by row and column counters, ORing shifted source words into the destination.
- `blit_right_shifted_by_or`: Blits right-shifted source words into destination rows by OR-ing them in over the rectangle, clearing bits first only on rows whose source pointer differs from the marker.
- `blit_right_shifted_rows_clipped`: Performs a right-shifted row blit bounded by row and column counters, ORing shifted words into the destination.
- `copy_table_rows_with_masks`: Copies or merges source rows into destination rows according to an index table: when the mode flag is set, rows are written directly and zero-filled for empty entries; otherwise destination words are cleared with an AND-NOT mask and the source words are ORed in, with a masked merge for other indices.
- `blit_masked_rows_with_clip`: Blits rows into the destination with a mask: for each word it clears the covered bits with an AND-NOT and ORs in the source word.
- `blit_unshifted_rows_by_or`: Blits unshifted source words into destination rows by OR-ing them over the rectangle, clearing destination bits first on rows whose source pointer differs from the marker.
- `blit_unshifted_rows_or_clip`: Performs an unshifted row blit bounded by row and column counters, clearing the covered bits and ORing source words in on rows whose pointer differs from the marker.
- `overlay_unshifted_rows_by_table`: ORs source rows into the destination first, then walks the row index table: empty entries clear the row with an AND-NOT, matching entries OR the source in, and other entries apply a masked merge.
- `overlay_unshifted_rows_with_clip`: Performs the table-driven row merge bounded by row and column counters: empty entries clear the destination row, matching entries OR the source words in, and others apply a masked merge.
- `copy_rows_zero_empty_entries`: Copies rows of 16-bit words from the source pointers into the destination, zero-filling rows whose table entry is empty or out of range.
- `zero_or_copy_rows_clip`: Writes zero words for rows that fall outside the clip bounds and copies source words for the rest, looping over the row table.
- `fill_zero_rows_then_copy`: Zero-fills a block of destination rows, then for each table entry copies a row of 16-bit words from the source pointer.
- `copy_rows_or_zero_clip`: Copies source rows of 16-bit words into the destination for each table entry, writing a zero word for entries that are empty or outside the clip bounds.
- `merge_byte_offset_bitplane_rows`: Merges byte-addressed bitplane rows using the row index table, with the starting byte offset adjusted when the doubled shift value carries or is negative.
- `overlay_byte_rows_with_clip`: Performs a byte-wise table merge bounded by row and column counters: source byte rows are OR-ed into the destination, empty entries clear the row, and matching or other entries apply the merge rules.
- `clear_then_merge_byte_rows`: Clears a block of byte-wide destination rows with an AND-NOT of the mask, then walks the row index table: matching entries OR the source row in, and other entries apply a masked merge.
- `merge_byte_rows_with_clip`: Merges byte-wide rows bounded by row and column counters: source bytes are OR-ed into destination rows for matching entries, and masked merges are applied when the source pointer differs from the marker.
- `overlay_byte_rows_by_table`: ORs source byte rows into the destination first, then walks the row index table: empty entries clear the row with an AND-NOT, matching entries OR the source in, and other entries apply a masked merge.
- `overlay_byte_rows_clip_merge`: Performs the table-driven byte-row overlay after an OR pass, bounded by row and column counters: empty entries clear the row, and other entries merge source bytes under a mask.
- `program_blitter_masks_and_modulos`: Sets the blitter's first and last word masks, the source and destination modulos and the shift control register for a clipped rectangle, adjusting pointers and row counts to match.
- `start_blitter_row_pointers`: For each row, writes the source A, B, C and D pointer registers of the blitter with the row addresses and selects the mask combination for the row.
- `feed_blitter_source_words`: Writes source words into the blitter's A-data register for each row, taking the row pointer from the table and selecting the mask word for the row.
- `start_blitter_rows_with_size`: Writes the A-data word, the control register and the blit size register for each row, starting one blitter operation per row.
- `copy_request_rows_via_blitter`: Reads a request record from the state block, clears its flag bytes, and copies the rows it describes.
- `start_blitter_copy_rows`: Programs the blitter's B and D modulo registers, pointers and size for a set of rows with a fixed copy control word.
- `record_and_copy_rows`: Stores the row parameters into a request block and then copies the rows word by word from source to destination.
- `program_blitter_copy_rows`: Writes the blitter's B and D modulo registers, pointers and size for each row of a copy, using a fixed control word.
- `dispatch_command_by_opcode`: Clears a flag bit in the record when the state byte is set, then reads an opcode byte and calls the matching handler from a jump table.
- `load_sprite_bounds_from_record`: Loads four 16-bit bounds and a size pair from the referenced record into the state block.
- `copy_bounds_and_size_fields`: Copies four 16-bit bounds from the linked bounds record into the state block, then copies the size and offset fields from the source record.
- `call_three_helpers_return_zero`: Calls three helper routines of the library in sequence and returns zero to the caller.
- `call_helpers_if_count_positive`: Calls two helper routines, then calls a third only when a doubled count field minus one is non-negative.
- `run_flag_selected_helper_sequence`: Tests one bit of a flag byte in the state block and runs one of two helper call sequences.
- `call_helper_then_conditional_helper`: Calls a helper and, depending on the zero flag it leaves behind, calls a second helper and returns one.
- `initialize_flags_from_record`: Clears three state fields, copies a 32-bit value from the source record, and sets the high bit of each byte in two flag words when a flag in the record is set and the value is non-zero.
- `call_three_helpers_zero_result`: Calls three helper routines in sequence and returns zero.
- `call_helpers_when_count_nonnegative`: Calls two helper routines, then a third only when a doubled count field minus one is non-negative.
- `copy_counted_long_table`: Reads a count word from the source record and stores it in the state block, then copies count plus one 32-bit entries from the record into a table in the state block.
- `copy_counted_long_pairs`: Reads a count word from the source record and stores it in the state block, then copies 32-bit entries into the state table two at a time until the count runs out.
- `compute_table_min_max_bounds`: Scans the count-prefixed table of 16-bit pairs in the state block, tracking the lowest and highest value in each of the two columns.
- `scan_table_min_max_bounds`: Scans a table of 16-bit pairs for as many entries as a register count gives, tracking the minimum and maximum of each column.
- `walk_indexed_entry_list`: Reads a mode byte from the record: in mode one it follows a chain of 16-bit offsets until a minus-one terminator and keeps the last entry.
- `store_flags_and_return_word`: Stores a sentinel value and copies a 16-bit flag word from the record into the state block.
- `return_void_3`: This library routine is an empty routine that returns without performing any operations.

### Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_twice_then_halt`: Calls a library routine through an indirect pointer twice, then enters an endless loop with no exit.
- `return_minus_one_constant`: Returns the constant minus one with no other work, so the caller receives a value with all bits set.
- `poll_button_bit_until_clear`: When bit six of a polled byte is set, it stores a pointer, sets two words to all-ones, and calls helper routines.
- `open_graphics_library_and_setup`: Forbids multitasking, opens the graphics library through the system library, makes a series of library calls on the returned base, and stores the results with the interrupt and DMA register readings in the state block.
- `setup_copper_list_and_interrupts`: Clears all interrupt request and enable bits and the DMA channels, fills a table of eight entries with addresses computed from a base offset, and points the copper and audio registers at the new values.
- `fill_arithmetic_sequence_table`: Fills a run of table entries with a sequence in which each value is the previous value plus a step.
- `restore_system_state_and_close`: Restores the saved copper pointer, interrupt, DMA and audio register values, closes the library, re-enables multitasking, and calls further library routines to finish.
- `clear_interrupt_request_after_calls`: Sets a state byte to all-ones when the fire-button bit of the polled input byte is clear, calls five helper routines, then writes the interrupt request register to acknowledge one interrupt.
- `tick_counter_and_call_helper`: Decrements a countdown word in the program data when it is non-zero, otherwise calls a helper.
- `render_text_character_stream`: Reads characters from a string: control codes end the line, a newline advances the line position by a fixed step, and printable characters look up a width in a table.
- `set_text_position_fields`: Stores a position value and a fixed attribute into the text state fields, calling a helper to compute the position before and after the stores.
- `store_helper_result_in_field`: Calls a helper routine and stores its result into a state field at a fixed offset.
- `store_pointer_and_clear_attribute`: Stores the incoming pointer in the state block, calls a helper, and then clears a 16-bit attribute field.
- `measure_text_width_from_table`: Walks a string until a control code, subtracting each character's width from a table indexed by character code minus the space code, starting from a fixed width.
- `compute_centered_line_offset`: Counts the newline characters in a string up to its terminator and returns a vertical start position that centres the block, computed as a fixed base minus nine units per line, halved.
- `clear_object_data_structure`: Checks if a global index is non-zero, then calculates a memory offset to zero out a large block of data within a structure.
- `build_copper_line_list`: Returns unless the index lies between thirty-one and one hundred and eleven.
- `toggle_and_verify_three_reads`: When a state counter is within range, it toggles a flag and, once the flag is set, reads a helper value three times, storing each result and recording whether the readings differ.
- `combine_nibble_range_values`: Walks two 16-bit inputs one nibble at a time, comparing each pair of nibbles and computing a clamped value that moves toward the second input.
- `pack_status_bits_from_counters`: Advances one of two counters, reads a value through helper calls, and packs the high and low bytes and carry bits of that value into a status word and a companion byte in the program data.
- `pack_status_pair_into_buffer`: Packs a value and a counter from register inputs into two words at the target pointer: the first holds the shifted value, and the second combines the sum's high byte with carry flags and the low nibble of the source.
- `initialize_object_state_fields`: Writes initial pointer and size fields into two state records, clears their offset fields, and sets a control word and a table pointer.
- `decode_timed_text_stream`: Counts down a delay, and when it expires reads the next byte of a text stream: a zero byte ends the stream, a high-bit byte selects a fixed delay, a space pauses briefly, and other bytes look up a length-prefixed entry in a table and return its value.
- `decode_text_stream_next_value`: Reads the next byte of a text stream and returns a value: a space resets the state and returns the space code, a printable byte looks up a length-prefixed entry in a table and returns its value, and a zero or control byte sets the end state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_dos_library_and_call`: Clears a data word, opens a library through the system library, calls a routine on the returned base, then calls another library routine and returns zero.
- `wait_frames_around_workbench_calls`: Waits for up to one hundred display frames while polling a library result, then runs a sequence of library calls including the workbench close and open wrappers.
- `derive_memory_tier_from_flags`: Derives a tier value from two flag bits and an availability query, then queries available memory again.
- `append_selected_string_with_space`: Copies a zero-terminated string chosen by a state index from a table into the destination, then appends a space and a terminator.
- `open_math_tridee_vector_libraries`: Opens three shared libraries, math, tridee and vector, through the open-library wrapper, and stores each returned base in its own data word.
- `call_three_routines_in_sequence`: Calls three routines in sequence and returns, with no other work.
- `close_library_wrapper`: Checks a register and calls a function pointer stored at an offset from a global variable.
- `call_open_library_2`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `install_handler_and_run_sequence`: Stores a handler pointer and a display offset in the state block, then calls a library routine on a base.
- `clear_state_and_run_sequence`: Clears three state fields, calls a library routine on a base, and calls a second library routine when its result is non-zero.
- `reset_state_then_run_sequence`: Clears three state fields and runs the same library call sequence as its neighbour, calling the follow-up routines only when the earlier results are non-zero.
- `reset_fields_then_run_sequence`: Clears three state fields and runs the same conditional library call sequence as its neighbours, with a final call that takes the argument minus two.
- `return_minus_one_data`: The decompiled body returns a value whose high half is minus one and whose low half is a data word.
- `dispatch_by_opcode_word_check`: Compares a 16-bit word in the record with a fixed instruction encoding and calls through one of two stored code pointers depending on the result.
- `expand_tilde_escapes_into_buffer`: Copies a string into a buffer while expanding tilde escape sequences: a tilde followed by n ends the copy after a fixed string, and other tilde codes insert a fixed name followed by a colon.
- `retry_lock_then_call`: Calls a lock-check routine repeatedly until it reports success, then finds the length of the name string, calls a routine with that length and a fixed argument of minus two, clears two state fields, and makes a final cleanup call.
- `open_dos_graphics_intuition_libraries`: Opens the dos, graphics and intuition libraries through the system library, storing each returned base in its own state field after one earlier library setup call.
- `close_three_library_bases`: Closes each stored library base that is non-zero, in order, and returns the result of the last close attempted.
- `call_close_work_bench`: Wrapper that calls the intuition library function CloseWorkBench and returns its result to the caller.
- `call_open_work_bench`: Wrapper that calls the intuition library function OpenWorkBench and returns its result to the caller.
- `adjust_window_bounds_after_check`: Range-checks an index against a stored limit and traps when it is out of range.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `recalculate_window_bounds_on_check`: Calls a library check and, when it succeeds, recalculates the window bounds from the old bounds and the size values, halving the sizes.
- `close_window_and_screen_resources`: Releases the window and screen held in the state block: when the first handle is set it clears the pointer and closes the window, then when the second handle is set it closes the screen.
- `open_screen_window_palette`: Clamps a requested width to the range two hundred to two hundred fifty-six, opens a screen, loads its colour table and opens a window on it, then brings the screen to the front.
- `set_pointer_shape_from_base`: Calls a pointer-setting routine of the intuition library through a stored base, with no arguments, and returns the registers unchanged.
- `replace_library_vector_entry`: Saves the current library vector entry and replaces it with a new one, calling the system's cache-clear and checksum routines around the change.
- `restore_library_vector_entry`: Restores the saved library vector entry and its companion word, calling the cache-clear and checksum routines around the restore while multitasking is forbidden.
- `return_minus_eleven_constant`: Returns the constant minus eleven with no other work.
- `return_immediately_as_hook`: Does nothing and returns.
- `return_immediately_as_callback`: Does nothing and returns.
- `walk_digit_fields_and_dispatch`: Calls a stored routine and then walks a string of decimal digits, stopping at a terminator or colon.
- `signal_task_after_callback`: Calls an optional callback stored in the state block when it is set, then sends a signal through the exec library.
- `signal_wait_frames_then_callback`: Sends a signal through the exec library, waits for display frames while a counter is non-zero, then calls an optional callback stored in the state block when it is set.
- `allocate_signal_and_add_task`: Allocates a signal and a memory block for a task control structure, fills in its name, signal masks and list pointers, and adds it as a task through the exec library.
- `remove_task_and_free_resources`: Forbids multitasking, removes the task and frees its memory block and signal when they are set, then permits multitasking again.
- `wait_signal_then_init_task`: Waits on the signal of one of two bit numbers, records the selected pointer, sets a flag byte and calls a library routine that starts the task.
- `init_task_from_pointer_argument`: Sets a flag byte in the record given by a pointer, calls the library routine that starts the task, and runs the same conditional setup helpers as its neighbour when that call succeeds.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_shared_task_setup_sequence`: Runs the shared task setup sequence with its arguments taken from registers, using the exec library's task lookup and an interrupt server removal stub along the way.
- `write_in_chunks_of_eighty`: Calls a library routine for a value and, when that value is non-zero, passes a total of one hundred seventy-nine units to a second library routine in chunks of at most eighty.
- `skip_leading_spaces`: Advances a string pointer past space characters, stopping at the first other character or when the length counter runs out.
- `split_command_line_into_words`: Splits a command line into words: it terminates each word with a zero byte at a space or newline, treats double-quoted sections as single words, and stores each word's start address in a table.
- `initialize_command_state_from_library`: Stores the results of successive library calls into state fields, sets a flag byte once the third value is available, and calls a follow-up helper when the last value is non-zero.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `open_library_by_name_argument`: Calls the system's open-library routine with the arguments already in place and returns its result through the registers.
- `copy_two_words_from_record`: Copies two 16-bit fields from the record at a stored pointer into two state words.
- `run_helpers_then_close_library`: Calls three helper routines, then closes the library whose base is stored in the state block when it is set, returning that close result, or zero when no base is stored.
- `close_library_base_in_a1`: Closes the library whose base is in the second address register when that base is non-zero, and returns nothing.
- `call_add_int_server`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `halt_in_endless_loop`: Contains only an endless loop with no exit, so control never returns from this point.
- `increment_counter_clear_flag_bit`: Increments a 16-bit counter in the object and clears bit three of the flag byte beside it.
- `advance_counter_clear_flag_bit`: Repeats the same step as its neighbour: it advances a 16-bit counter and clears bit three of the flag byte beside it.
- `init_task_run_helpers`: Stores a pointer in the state block, sets a flag byte in the record and calls a library routine that starts the task.
- `send_chunked_total_to_library`: Gets a value from a library call and, when it is non-zero, passes a total of one hundred seventy-nine units to a second library routine in chunks of at most eighty.
- `skip_spaces_in_argument_string`: Advances a string pointer past space characters using a length counter, stopping at the first other character or when the counter runs out.
- `split_argument_line_into_words`: Splits a command line into words as its counterpart does: zero bytes at spaces and newlines, quoted sections kept whole, and word start addresses stored in a table.
- `initialize_state_from_library_results`: Stores successive library results into state fields, sets a flag byte once the third value is available, and calls follow-up helpers when the last value is non-zero.
- `open_library_via_jump_pointer`: Calls the system's open-library routine through a stored pointer, with arguments already in place, and returns its result.
- `open_library_via_stored_vector`: Calls the system's open-library routine through a stored vector, with arguments already in place, and returns its result.
- `copy_two_words_linked_record`: Copies two 16-bit fields from the record referenced by a stored pointer into two state words.
- `run_helpers_close_library`: Calls three helper routines, then closes the library whose base is stored in the state block when it is set, returning that close result, or zero when none is stored.
- `close_second_library_base`: Closes the library whose base is in the second address register when that base is non-zero, returning nothing.
- `add_interrupt_server_via_vector`: Calls the exec library's add-interrupt-server routine through a stored vector, with arguments already in place.
- `remove_interrupt_server_via_vector`: Calls the exec library's remove-interrupt-server routine through a stored vector, with arguments already in place.

### logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf

- `tiny_ori_2`: Small routine of 1 instruction that merges 29 into d0.
- `walk_chain_then_call_entry`: Follows a linked table, scaling each link, until it reaches a zero link, then calls the routine stored four bytes into the final entry.

### logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf

- `follow_link_chain_to_handler`: Follows a linked table of scaled links until it reaches a zero link, then calls the routine stored four bytes into the final entry.
- `walk_chain_then_library_call`: Follows a linked table of scaled links until a zero link, then calls the routine four bytes into the final entry, which is a library stub that calls a library function at a fixed offset.

### logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a3].adf

- `follow_second_link_chain`: Follows a linked table of scaled links until a zero link, then calls the routine stored four bytes into the final entry.
- `walk_chain_library_call_alt`: Follows a linked table of scaled links until a zero link, then calls the routine four bytes into the final entry, which is a library stub that calls a library function at a fixed offset.
