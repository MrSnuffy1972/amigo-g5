# Game 1971 (captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01971). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (920240 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 26 raw-word lines |
| Functions named and described | pass, 416 of 416 named (0 author symbols), 416 of 416 described (20 by rule, 33 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1787 of 1787 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8748 of 8748 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 605232 |
| code | 25862 |
| data | 127584 |
| padding | 2842 |
| unused | 158720 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| endcli | 2 | 117 | 2 |
| Liberation | 5 | 227 | 3 |
| delirium | 2 | 267 | 8 |
| PlayASP | 4 | 10 | 1 |
| Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf:HardDisk_Install | 3 | 10 | 1 |
| math.library | 1 | 807 | 27 |
| tridee.library | 1 | 2181 | 93 |
| vector.library | 1 | 3569 | 184 |
| Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 516 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 655 | 38 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 162 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 173 | 15 |
| logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf | 1 | 11 | 2 |
| logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf | 1 | 11 | 2 |
| logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf | 1 | 11 | 2 |

## Cases handled

- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a2]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf:StartUp.asp: high-entropy data file (7.84 bits/byte), classified as packed data
- runtime segment an address: 406 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 375 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 50 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a2]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf: 9 executed PCs whose RAM was overwritten before any snapshot match its bytes 230976-231072: recovered as a unit run at an address
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a2]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf: 9 executed PCs whose RAM was overwritten before any snapshot match its bytes 230976-231072: recovered as a unit run at an address
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a2]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf: 9 executed PCs whose RAM was overwritten before any snapshot match its bytes 230976-231072: recovered as a unit run at an address
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a2]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf: 3 logged loads cover 0 bytes of the image
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a2]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf:c/PlayASP: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a2]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf:HardDisk_Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `call_library_then_halt` (runtime segment an address): Calls an operating-system library routine through a stored pointer twice, then enters an endless loop so control never returns. Kind: loader/sabotage path. Pass path: the code following the call (which in this case is an infinite loop/halt).

## Functions


### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_handler`: Copies sixteen 32-bit words from a fixed offset of a base block into a caller-supplied buffer, but only when the buffer pointer is non-null.

### Liberation

- `dispatch_resident_entry_handler`: Follows a chain of scaled offset links from a fixed table location until a zero link is reached, then calls the routine pointer stored in the second slot of the final entry.
- `scan_and_unpack_tagged_blocks`: Scans a sequence of blocks for a four-character tag, copies each tagged block's payload into place, applies a second pass of tagged records, and zero-fills the remainder.
- `decode_bitstream_literals_and_copies`: Reads a packed bit stream one bit at a time through a carry-shifting register and alternately copies literal bytes or repeated runs from earlier output.

### delirium

- `load_hunk_file_sections`: Parses AmigaDOS executable hunks, allocating memory for each code, data or BSS section through the system allocator and copying its contents in.
- `count_down_delay_loop`: Decrements a 16-bit register until it wraps past zero, producing a short busy-wait delay.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_backward_run_into_buffer`: Fills a buffer from its end toward its start, alternating between literal values read from the input and back-reference copies from bytes already written.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bits_from_reverse_stream`: Shifts a requested number of bits out of a 32-bit bit buffer into an accumulator, refilling the buffer from the preceding memory word whenever it empties.
- `perform_obfuscated_math`: Performs a negation and addition on a byte through a pointer and modifies a byte in another memory location.

### PlayASP

- `traverse_and_call_list_entry`: Traverses a table of pointers until a null entry is found and then calls a function from the resulting pointer.

### Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf:HardDisk_Install

- `traverse_linked_list_and_execute`: Follows a chain of pointers by dereferencing values until a null pointer is encountered.

### math.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `store_two_fields_in_library`: Stores two longword arguments into fixed fields of a structure addressed through a register, at offsets 36 and 40 bytes.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_counter_then_conditional_call`: Decrements a 16-bit usage counter in a library structure.
- `free_or_defer_library_block`: When the counter at offset 32 is zero, it makes two system library calls through the library base and returns the value stored at offset 36.
- `free_block_and_return_value`: Reads a longword stored at offset 36 of a structure, passes the structure to two system library calls that release it, and returns the saved value.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `lookup_word_from_table`: Masks the input to its low nine bits, doubles that to a byte offset, and returns the 16-bit word at that offset in a table held in library data.
- `scale_and_offset_point_pair`: Scales the two 16-bit coordinates in a structure by a ratio built from its scale and divisor fields, then adds a fixed offset field to each.
- `swap_words_and_store`: Rotates a 32-bit value by sixteen bits, exchanging its upper and lower words, and stores the result at the target address.
- `scale_point_by_divisors`: Multiplies each 16-bit coordinate by a scale field, divides by a sum that includes a shared offset field when that sum is non-zero, then adds an offset field to each.
- `stub_return_immediately`: A routine that only returns to its caller and performs no work.
- `stub_return_immediately_2`: The decompiled body is only a return, although the decoded span covers many instructions, so nothing observable happens here.
- `execute_library_function`: Library code that performs a specific mathematical operation.
- `split_value_by_hundred`: Calls another routine with a sentinel value of minus one, then divides its result by 100, packing the quotient into the upper word and the remainder into the lower word of the returned value.
- `advance_linear_congruential_seed`: Reseeds a 16-bit state variable from its argument when that argument is non-negative, then advances it with a linear congruential step that multiplies by 1509 and adds 41.
- `build_rotation_matrix_from_angles`: Looks up sine and cosine values in a table for three angles, then combines them in fixed-point arithmetic to fill a nine-element 3 by 3 rotation matrix in the caller's buffer.
- `build_alternate_rotation_matrix`: Builds a nine-element 3 by 3 matrix from three angles using the same table lookups as its neighbour, but combines the terms in a different order.
- `stub_return_immediately_3`: Returns its caller without storing or computing anything, so the routine has no effect.
- `multiply_3x3_fixed_point_matrix`: Multiplies two 3 by 3 matrices of 16-bit fixed-point values, shifting each dot product right by 14 bits to keep the scale, and writes the nine results to the output buffer.
- `compute_integer_square_root`: Computes the integer square root of its 32-bit argument by binary search over 16-bit candidates.
- `refine_square_root_bracket`: Narrows a binary search bracket for a square root by repeatedly comparing the square of the midpoint with the target value, until the bracket is one step wide.
- `return_constant_one`: Returns the constant one together with an unused upper word, with no computation.
- `interpolate_in_sorted_table`: Finds the magnitude of a 16-bit input in a sorted word table by binary search, then linearly interpolates between the two neighbouring entries, negating the result when the input was negative.
- `interpolate_in_passed_table`: Performs the same sorted-table search and linear interpolation as its neighbour, but over a table whose address the caller supplies, and negates the result for negative input.
- `compute_interpolation_fraction`: Computes where an input lies between two table entries by dividing the scaled difference by the span between those entries, negating the result when the sign flag is set.
- `compute_clamped_triangle_ratio`: Forms two edge vectors relative to a base point, takes their cross product and dots it with a third vector, then divides by a scale value and clamps the result between zero and a limit stored in the structure.

### tridee.library

- `return_minus_one_status`: Returns a constant minus one with no other work.
- `store_two_fields_in_library_2`: Stores two longword arguments into fixed fields of a structure addressed through a register, at offsets 36 and 40 bytes.
- `increment_counter_and_clear_flag_2`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_counter_then_conditional_call_2`: Decrements a 16-bit usage counter in a library structure.
- `free_or_defer_library_block_2`: When the counter at offset 32 is zero, it makes two system library calls through the library base and returns the value stored at offset 36.
- `free_block_and_return_value_2`: Reads a longword stored at offset 36 of a structure, passes the structure to two system library calls that release it, and returns the saved value.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `open_library_on_first_use`: Increments a usage counter and, on first use, opens the math library through the system open call, caching the returned base pointer for later calls.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `close_library_when_unused`: Decrements a usage counter and, when it reaches zero with a cached base present, closes the library through the system close call and clears the cached base.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `allocate_sized_object_buffer`: Computes a buffer size from a count field, allocates that buffer through the library allocation call, and fills in its size and pointer fields, reporting failure through a fallback call.
- `allocate_and_init_size_block`: Allocates a block through the system allocation call and, on success, fills its size, counter and pointer fields from the parameters, then returns the block.
- `recalculate_buffer_layout_fields`: Recalculates the size and layout fields of an existing buffer from a size value, deriving a count by dividing by 52 and storing a shifted size and an end offset.
- `call_routine_return_argument`: Calls another routine with no work of its own, then returns its stack argument unchanged.
- `clear_pending_flag_and_report`: If a pending flag at offset 148 is set, it clears the flag and makes a library call tagged with a fixed code, then makes a second tagged library call before returning.
- `run_paired_library_calls`: Makes a system library call, then a second call through one of two bases chosen by a flag bit, then a tagged call.
- `free_memory_then_call`: Makes a system memory-release call through the exec base and then calls another routine.
- `allocate_indexed_record`: Allocates a record sized as 84 bytes times a counter argument, through the library allocation call or a fallback base, and stores its size, pointer and owner fields.
- `allocate_size_record_from_count`: Allocates a record through the library allocation call and, on success, fills its size, pointer and buffer fields from a 16-bit count argument.
- `return_stack_argument`: Returns its single stack argument unchanged, with no other work.
- `reset_bounds_and_clear`: Copies a longword between two fields, clears two state fields, and sets four 16-bit bounding fields to the largest and smallest signed 16-bit values.
- `project_point_list_to_screen`: Adds a translation to each point in a list, then perspective-projects the x and y coordinates by scaling them and dividing by depth plus an offset, skipping the division when the divisor is zero.
- `compute_weighted_sum_fixed`: Computes a weighted sum of three 16-bit inputs using three stored coefficients, skipping any zero input, then repacks the result into a fixed-point pair.
- `scale_point_by_divisors_2`: Multiplies each 16-bit coordinate by a scale field, divides by a sum including an offset field when that sum is non-zero, then adds an offset field to each.
- `copy_state_and_decode_flags`: Copies eight words and several longword fields from a source record into the target structure, then reads flag bits of the source to choose a return code.
- `check_flags_and_save_backup`: Tests flag bits in a source record and returns a result code, returning the second flag bit directly when it is set.
- `save_backup_when_flag_set`: Returns zero unless the source record's flag bits are set, and when the third flag bit is set it copies a saved block of longwords and a word into the structure.
- `stub_return_immediately_4`: Returns without doing any work.
- `return_immediately`: This routine is an empty function that performs an immediate return to its caller.
- `store_three_words_in_state`: Stores three 16-bit values taken from its two arguments into fixed fields of a state structure at offsets 108, 110 and 112.
- `load_record_into_state`: Copies several fields from an input record into the state structure, sets two 16-bit limits to the largest and smallest signed values, and runs flag-dependent processing.
- `update_entry_bytes_by_ratio`: When a flag bit is set, walks a linked list of entries and, for each qualifying entry, adjusts a byte field by a proportion computed from two stored limits and a divisor.
- `update_entry_bytes_all_entries`: Walks a linked list of entries and, for each qualifying entry, adjusts a byte field by a proportion computed from two stored limits and a divisor.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `accumulate_entry_bounds`: Walks a linked list of entries and, for each entry passing a test, widens four 16-bit bounding fields in the state to include that entry's extents, recording qualifying entries in an output list.
- `relink_entries_and_accumulate_bounds`: Walks a linked list of entries, widening the state's four bounding fields by each qualifying entry's extents, and rebuilds a list of the qualifying entries as it goes.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a4, then returns.
- `compute_record_byte_by_ratio`: For a record with flag bits set and a valid index, computes a byte from a divided offset and stores it in the record's byte field, clamping negative results to zero.
- `compute_byte_from_scaled_input`: When a flag bit is set, computes a byte from the input value scaled by a ratio of two stored fields, then writes it into a record byte field and clamps negative results to zero.
- `store_scaled_byte_2`: This library routine stores an input byte into a record field, optionally scaling the value by a ratio before writing.
- `store_scaled_byte`: Library code that stores an input byte into a record field, optionally scaling the value by a ratio before writing.
- `store_scaled_byte_in_record`: Stores the input low byte into a record field unless a flag bit is set, then computes a byte from the input scaled by a ratio and writes it into another record byte, clamping negative results to zero.
- `compute_byte_offset_from_scaled`: Subtracts a scaled amount from a byte input and writes the result to a record byte field, adding a base byte value and clamping negative results to zero.
- `compute_scaled_offset`: Library code that calculates a scaled value using multiple parameters from a table and a record, then adds this to a base byte and stores the result.
- `return_immediately_2`: This library routine is empty and performs no operations before returning to the caller.
- `compute_scaled_offset_alt`: Library code that performs a calculation involving scaled values and record fields to update a byte in a record.
- `compute_entry_value_range`: Scans the values of a set of entries for their minimum and maximum, shifts both by a field taken from the record's flags, stores them in the record, and widens the state's two bounding fields to include them.
- `find_value_range`: Library code that iterates through a list of records to find the minimum and maximum values, scales them, and updates a global range.
- `clip_record_into_pool_entries`: Marks the record as processed, then tests its range against an input value.
- `clip_edge_into_pool_entries`: Tests the range of an edge against a limit, and when it passes, copies vertex data into fresh entries drawn from a pool and links them into the output list, handling both endpoints.
- `rotate_or_swap_vector_pair`: Scales two coordinates by the reciprocal of a divisor, then either swaps two longword vectors and returns a squared length when a magnitude test passes, or rotates the pair and computes a fixed-point quotient by repeated division.
- `rotate_vector_and_divide_fixed`: Rotates a two-component vector by a pair of scaled coordinates, then computes a fixed-point quotient of its squared components by repeated long division with bit-count normalisation.
- `divide_with_normalized_shift`: Computes a fixed-point quotient of two 16-bit operands by normalising the divisor's bit length and running a long-division loop, returning the quotient together with its shift count.
- `divide_shifted_by_word`: Shifts a 32-bit operand and divides it by a 16-bit divisor in a long-division loop, returning the quotient scaled by a shift.
- `compute_squared_ratio_or_check`: Stores its argument into a record field.
- `compute_squared_ratio_fixed`: Returns a 16-bit ratio of two squared magnitudes, each built from stored record fields and scaled by two stored coefficients.
- `compute_weighted_squared_ratio`: Computes a ratio of two squared quantities, weighting the denominator by squared coefficients passed as arguments, and returns it in the low word of the result.
- `compute_scaled_difference_ratio`: Returns a 16-bit result computed by multiplying a difference of two words by a third word and dividing by the first word.
- `rotate_and_test_record_bounds`: Rotates a two-component vector into the record with a sign correction, then runs a bounds test over the record's vertex data.
- `return_minus_one_fallback`: Returns minus one, combined with a stored longword from the structure, with no other work.
- `compute_squared_edge_length`: Computes the squared length of the difference between two stored vertices in three dimensions, using their indices from the structure, and returns its absolute value.
- `set_record_coordinates_from_vertex`: Stores a value and a zero into a record, derives an offset from a vertex coordinate and adjusts it to avoid zero, then fills the record's coordinate fields from a pair of vertex coordinates and clears a processing flag.
- `compute_record_vertex_bounds`: When the input argument is not three, runs a bounds test that can return minus one, then computes the minimum and maximum of each vertex coordinate and stores them as the record's bounding fields.
- `return_minus_one_fallback_2`: Returns minus one together with a longword from the second argument, with no other work.
- `clip_polygon_into_pool_entries`: Clips a list of vertex indices against a limit, copying vertex data for kept or newly computed vertices into fresh pool entries and linking them into the output.
- `clip_vertex_ring_into_pool`: Walks a ring of vertex indices, emitting vertex data into pool entries for each kept or newly intersected vertex and closing the ring with a sentinel value.
- `emit_clipped_vertices_into_pool`: Walks a ring of vertex indices, emitting clipped vertex data into pool entries and linking the results into the output, with a sentinel value closing the ring.
- `compute_polygon_edge_metric`: Walks a stored vertex list to compute either a squared edge length or a clipped result, choosing between them by a record type field and allocating output entries from a pool.
- `return_minus_one_fallback_3`: Returns minus one together with a longword from the structure, with no other work.
- `clip_entry_into_pool`: Tests a record's vertex range against a clip limit, and when it straddles the limit, copies vertex data into fresh pool entries and links them into the output list.
- `clip_vertex_list_against_limit`: Walks a list of vertex coordinates against a limit, copying vertex data for kept or newly computed vertices into fresh pool entries and linking them into the output.
- `compute_squared_edge_length_2`: Computes the squared distance between two vertices referenced by the structure, summing squared differences in three coordinates, and returns its absolute value.
- `calculate_edge_bounds`: Library code that computes coordinate differences and stores the resulting values into a destination structure.
- `compute_bounds_and_cull_test`: Computes the minimum and maximum of each vertex coordinate and stores them as the record's bounding fields, then runs a cross-product test over two vertices that returns minus one when it fails.
- `clip_edge_list_into_pool`: Clips a record's vertex list against a limit and copies vertex data into fresh pool entries, linking them into the output and stopping when the pool is exhausted.
- `append_vertex_to_pool`: Iterates through a list of vertex pointers and copies their coordinate data into a pre-allocated memory pool.
- `clip_vertex_ring_into_pool_2`: Walks a ring of vertex pointers, emitting vertex data into pool entries for each kept or newly intersected vertex and closing the ring with a sentinel value.
- `call_routine_then_return`: Calls one other routine and returns, with no other work of its own.
- `sort_entry_list_by_key`: Sorts a linked list of entries in place by a 16-bit key stored in each entry, swapping adjacent out-of-order nodes and repeating until a pass makes no swap.
- `resolve_overlapping_polygon_pairs`: Walks a list of polygon entries and, for each pair whose bounding boxes overlap, computes a plane from three vertices and tests the other polygon's vertices against it, using the result to reorder or skip the pair.
- `test_polygon_against_plane_list`: Computes a plane from a polygon's three vertices, then tests the entries in a list whose bounding boxes overlap against that plane and relinks the ones that pass.
- `classify_entries_against_plane`: Computes a plane from a polygon's vertices and classifies each overlapping entry by the sign of its vertices' distances from that plane.
- `compute_polygon_plane_and_test`: Computes a polygon's plane from up to three vertices, then loops over a list of entries and handles each overlapping entry according to the sign of its vertices' distance from that plane.
- `test_entry_edges_against_plane`: For each entry, computes edge vectors between its vertices and tests the sign of each edge against a stored plane, rebuilding the ordering of overlapping entries as it goes.
- `test_polygon_edge_crossings`: Computes cross products between consecutive polygon edges and compares their signs, setting one of two status codes when the edges of two polygons cross.
- `test_edge_crossings_and_reorder`: Walks a ring of edges, computing cross products against a reference polygon and setting status codes on sign changes, then updates the entry list ordering.
- `insert_entry_in_sorted_list`: Tests a polygon against a plane and, when the test passes, inserts the entry into a list ordered by a 16-bit key at offset 18, relinking neighbouring entries.
- `relink_and_recompute_entry_plane`: Relinks two list nodes by swapping their pointers, then recomputes the bounding-plane fields of an entry from its vertices, using a fixed offset of 100 when fewer than three vertices exist.
- `relink_and_recompute_entry_plane_2`: Relinks a list node into a new position and then recomputes the bounding-plane fields of the entry from its vertices, using a fixed offset of 100 when fewer than three vertices exist.
- `return_from_subroutine`: This routine is empty and performs only a return instruction.
- `interpolate_between_ordered_values`: Orders two 16-bit values, then linearly interpolates a third word between them using their difference as the span, returning the interpolated value.
- `stub_return_immediately_5`: The decompiled body is only a return, although the decoded span covers 19 instructions, so nothing observable happens here.

### vector.library

- `return_minus_one_status_2`: Returns minus one with no other work.
- `store_two_fields_in_library_3`: Stores two longword arguments into fixed fields of a structure addressed through a register, at offsets 36 and 40 bytes.
- `increment_usage_and_clear_flag`: Increments a 16-bit usage counter in a library structure and clears the deferred flag bit at offset 34.
- `decrement_counter_then_conditional_call_3`: Decrements a 16-bit usage counter in a library structure.
- `free_or_defer_library_block_3`: When the counter at offset 32 is zero, it makes two system library calls through the library base and returns the value stored at offset 36.
- `free_block_and_return_value_3`: Reads a longword stored at offset 36 of a structure, passes the structure to two system library calls that release it, and returns the saved value.
- `return_zero_status`: Returns zero with no other work.
- `allocate_and_initialise_sized_object`: Computes the byte size of a new object from width, height and flag bits, allocates it through the library allocation call, and fills in its header fields.
- `initialize_vector_entry`: Allocates a memory block and populates it with specific fields including size, flags, and link data.
- `allocate_and_init_vector_entry`: Allocates a fixed-size entry through the library allocation call and fills its size, flag and link fields from its arguments, storing a handler pointer in its first word.
- `initialise_allocated_vector_entry`: Allocates an entry through the library allocation call and fills its link, size, handler and offset fields from the arguments.
- `call_routine_return_argument_2`: Calls two other routines with no work of its own, then returns its stack argument unchanged.
- `fill_row_pointer_table`: Fills a table of pointers with consecutive addresses separated by a stride, computed as the product of two 16-bit values, with one entry per row.
- `fill_arithmetic_word_sequence`: Fills a 16-bit array with an arithmetic sequence that starts at zero and increases by a fixed step per element, with the count and destination taken from the structure.
- `fill_arithmetic_word_sequence_2`: Fills a 16-bit array with an arithmetic sequence starting from the first argument and increasing by the second argument per element, for a count given by a third argument.
- `release_optional_block_and_entry`: Releases an optional block referenced by the entry when its pointer field is non-zero, then makes a second system release call on the entry itself.
- `free_memory_then_conditional_call`: Makes a system memory-release call, then a second release call through either the exec base or the library base depending on a flag bit.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `process_entries_via_handler_table`: Walks a linked list of entries, skipping those already marked processed, and calls a routine chosen from a table by the entry's low flag bits, accumulating the returned values and marking each entry processed.
- `traverse_entries_marking_processed`: Traverses a linked list of entries with a flag-driven inner loop, marking each entry processed, adding handler results to a running total, and calling a handler selected by a flag bit.
- `process_linked_list_entries`: Iterates through a linked list of entries, executing a routine selected from a table based on a flag bit within each entry.
- `traverse_entries_marking_processed_2`: Traverses a linked list of entries from a stored head, marking each entry processed, adding handler results to a running total, and calling a handler selected by flag bits.
- `dispatch_flagged_routine_calls`: Calls one of two other routines depending on a flag bit, then makes conditional calls to further routines and returns one.
- `execute_conditional_sequence`: This library routine executes a series of conditional calls to other routines based on the state of a bit in a memory location.
- `call_routine_return_one`: Calls another routine and returns one.
- `call_optional_routine_return_one`: Calls another routine, makes a second call when the condition flag is clear, and returns one.
- `test_and_branch_on_zero`: Executes a routine and checks the zero flag to determine whether to execute a second routine and return a value of one, or simply return the result of the first routine.
- `copy_entry_values_to_buffer`: When a marker word is negative, resets it and copies a counted list of 32-bit values into the caller's buffer, reading each value from a fixed offset within the record it references.
- `copy_pointer_list_values`: Copies a counted run of 32-bit values, each read from a fixed offset within a referenced record, into the destination buffer.
- `draw_masked_points_to_bitplanes`: Waits for the blitter, then for each point inside the entry's bounds clears a bit in each destination bitplane and sets it from a second buffer, using a shift derived from the point's x coordinate.
- `draw_masked_points_to_bitplanes_2`: For each point inside the entry's bounds, clears a bit in each destination bitplane and sets it from a second buffer, using a shift derived from the point's x coordinate.
- `plot_clipped_points_to_bitplane`: For each point within the entry's bounds, clears then sets one bit in each destination bitplane using a shift from the point's x coordinate, skipping points outside the bounds and returning one when the list ends.
- `setup_sloped_edge_steps`: Computes fixed-point slope steps for a sloped edge from its endpoint deltas, rejects edges outside the clip window, and prepares stepping values for the span drawing that follows.
- `setup_sloped_edge_steps_2`: Stores a supplied flag word, then computes fixed-point slope steps for a sloped edge from its endpoint deltas and rejects edges outside the clip window.
- `draw_sloped_edge_spans`: Computes fixed-point slope steps for a sloped edge and walks its rows through a pointer table, writing span endpoints for each row inside the clip window.
- `emit_clipped_edge_endpoints`: Computes the clipped endpoints of one edge step, clamping them to the clip window, and stores either the two endpoints or a sentinel value of minus 16384 when the step lies fully outside.
- `walk_clipped_edge_endpoints`: Steps along an edge, storing clipped endpoint pairs for each row inside the clip window, and returns zero or one depending on whether any step was visible.
- `clip_line_span_endpoints`: Calculates and clips the horizontal start and end coordinates of a line segment against a bounding box.
- `walk_clipped_edge_endpoints_2`: Steps along a sloped edge with an error term, storing endpoint fields into the output structure for each visible row inside the clip window.
- `calculate_clipping_span_endpoints`: Computes clipped horizontal span endpoints for a line segment by comparing calculated coordinates against a bounding box.
- `clip_span_endpoints`: Clips the calculated horizontal span endpoints against a bounding box and clamps them to specified minimum and maximum limits.
- `check_span_inside_clip`: Checks whether a span lies inside the clip window and, when clipping is active, calls another routine.
- `return_zero`: This library routine returns a zero value to the caller.
- `test_both_flags_negative`: Returns minus one only when two signed 16-bit state fields are both negative, and zero otherwise.
- `clamp_bounds_to_clip_window`: Clamps the four bounding fields of a record to the clip window, taking the larger of each minimum and the smaller of each maximum, and stores the results in the working state.
- `clip_polygon_to_window`: Clips a polygon's vertex list against the edges of the clip window, computing intersection points where edges cross and storing surviving vertices in a work buffer.
- `clip_vertices_against_horizontal_edge`: Clips a list of vertices against one boundary line, computing the intersection of each crossing edge by a linear division and writing the resulting vertex pairs to an output list.
- `clip_vertices_against_boundary_line`: Clips a list of vertices against a second boundary line, computing each crossing intersection by a linear division and writing the resulting vertex pairs to an output list.
- `return_zero_2`: This library routine returns a value of zero to the caller.
- `collect_visible_entry_vertices`: Checks an entry's bounds against the clip window and, when it overlaps, copies its vertex data into the working buffer, advancing a counter as it goes.
- `copy_vertices_and_test_clip`: Copies the paired vertex values of a polygon into a buffer, then tests the entry's bounds against the clip window.
- `clip_vertex_pairs_against_boundary`: Clips paired vertex values against a boundary line, computing each crossing point by a linear division and writing the resulting pairs to an output buffer.
- `clip_vertex_pairs_against_boundary_2`: Clips paired vertex values against a second boundary line, computing each crossing point by a linear division and writing the resulting pairs to an output buffer.
- `store_word_and_return_zero`: Stores a 16-bit argument into a state field and returns zero.
- `fill_edge_rows_into_table`: For each consecutive pair of vertices, computes a fixed-point horizontal step and fills table rows between the two endpoints, tracking the minimum and maximum extents.
- `fill_span_rows_with_interpolation`: Fills table rows with values interpolated linearly across each span between consecutive vertices, updating running minimum and maximum extents and returning them packed together.
- `rasterize_polygon_edges_into_table`: Walks the edges of a polygon and steps each edge row by row with fixed-point increments, writing per-row results into a table and tracking the extents.
- `fill_span_rows_with_interpolation_2`: Fills table rows along each edge span with fixed-point interpolated values, stepping across the rows of each span.
- `fill_span_rows_with_interpolation_3`: Waits for the blitter, then fills table rows along each edge span with fixed-point interpolated values.
- `rasterize_polygon_edges_into_table_2`: Waits for the blitter, then walks polygon edges row by row and writes fixed-point per-row values into a table.
- `setup_blitter_line_draw`: Waits for the blitter, then writes a sequence of control and step words to the blitter registers, with octant bits chosen from the slope.
- `write_blitter_span_registers`: Waits for the blitter, then writes a set of span words to the blitter registers for each row, taken from a pair of tables.
- `fill_polygon_spans_with_blitter`: Walks the rows of a polygon's span table, computing each row's start address and mask, and writes blitter-driven fill words for rows that fit the working width.
- `generate_blitter_mask_rows`: Calculates and writes bitmask data to memory by performing bitwise operations on source values and a lookup table.
- `fill_polygon_spans_with_blitter_2`: Waits for the blitter, then walks the rows of a polygon's span table, computing each row's start address and mask, and writes fill words for rows that fit the working width.
- `fill_bitplane_span_words`: Fills horizontal spans of a bitplane by looking up start and end word masks in a table indexed by the low five bits of each x coordinate, then combining them with the destination words.
- `write_masked_span_words`: Writes fill words along a horizontal span, combining each destination longword with a mask and a value, and advancing through row tables and the bitplane count.
- `apply_masked_bitplane_span`: Iterates through a series of bitplane spans to perform bitwise masking and logical operations.
- `write_masked_span_words_2`: Writes masked fill words along a horizontal span into the destination, combining each word with a mask and a value and advancing through row tables.
- `fill_bitplane_span_words_2`: Computes start and end positions for a row span, then writes masked fill words across the bitplane using tables indexed by the low four bits of each coordinate.
- `blit_masked_bitplane_span_words`: Writes a sequence of words into a bitplane by applying bitwise masks to handle span edges.
- `write_masked_span_words_3`: Writes masked words across rows of a bitplane, combining each source word with a mask derived from the span edges and advancing through the row table.
- `fill_bitplane_rows_with_masks`: Writes a pair of masked edge words per row, filling the interior words with a solid value, and advances through the row offset table.
- `blit_masked_bitplane_spans`: Performs bitwise masking and merging of word-aligned data into a destination buffer.
- `fill_bitplane_span_words_3`: Writes masked fill words for a row span across the bitplane, using start and end masks taken from tables indexed by the low four bits of each coordinate.
- `write_masked_span_words_4`: Writes masked words for a row span across rows of a bitplane, combining each source word with a mask and advancing through the row offset table.
- `start_blitter_span_fill`: Waits for the blitter, then programs its size and control registers for a polygon row region computed from the record's bounds, returning zero when the region is empty and one after starting the operation.
- `write_blitter_edge_registers`: Spins while the blitter is busy, then for each edge computes step and offset values from its endpoints and writes them to the blitter registers.
- `draw_blitter_polygon_edges`: Walks a list of paired coordinate records, computing each edge's horizontal and vertical steps and an error accumulator, and programs the blitter's line registers for every edge, waiting for the blitter to go idle before each write.
- `emit_blitter_span_rows`: Loops over a per-row table of stored offsets, writing each row's destination pointer and size into the blitter registers and waiting for the blitter to be idle before every write.
- `emit_blitter_offset_rows`: Calls a subroutine to compute a starting offset, then programs the blitter's destination pointer and size registers for each row in a group, stepping the source offset by the stored row increment until the row counter runs out.
- `emit_blitter_row_run`: Iterates over a run count, programming the blitter's pointer and size registers for each row and advancing the offset by the stored stride, then returns to its caller once the run is exhausted.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `compute_bitplane_byte_offset`: Computes the byte offset of a position within a bitplane.
- `compute_row_address_offset`: Multiplies the low word of its input by the stored row stride and adds the column byte offset from a horizontal coordinate difference divided by eight, returning the combined 32-bit address offset.
- `stub_flag_table_check`: Reads a byte from a global table at an index and tests it for zero and for a sign bit.
- `stub_immediate_return`: Returns straight away with no operations; the decompiled body contains nothing beyond the return, so the routine has no observable effect.
- `return_immediately_3`: This library routine is empty and performs no operations before returning.
- `build_shifted_mask_rows`: Builds blitter source and mask rows.
- `rotate_source_rows_with_mask`: Processes a set of rows by rotating each 16-bit source word by a shift count held in a register, storing each rotated word twice, zero-filling the remaining columns, and writing the inverted OR of the row's bits as the mask word.
- `expand_alternating_bit_masks`: Expands the bits of two flag bytes into per-row 32-bit words using alternating one-and-zero bit patterns, one table per byte.
- `expand_flagged_pattern_masks`: Expands the bits of two flag bytes into per-row 32-bit words using alternating bit patterns.
- `expand_bit_pattern_columns`: Loops over a column count, shifting a bit register and storing one of two pattern words into each of two tables depending on the low bit.
- `expand_byte_bits_into_tables`: Loops over a fixed number of columns, taking one bit at a time from a byte and storing either zero or a pattern word into each of two tables.
- `setup_bitmap_header_fields`: Reads a bitmap record and stores its key fields: the plane-data pointer, a flag byte, a bit depth, the row width in bytes after rounding up to whole 16-pixel words, and the padding modulo.
- `load_relative_bitmap_header`: Locates a bitmap record through a relative pointer and stores its plane-data pointer, flag byte, bit depth, row width in bytes after rounding to whole 16-pixel words, and padding modulo.
- `store_scaled_draw_rectangle`: Scales the requested width and height by a stored factor, returning failure when a scaled dimension becomes zero.
- `store_centered_draw_rectangle`: Saves a rectangle's corner and size into the state block without scaling, computes a centred origin from half the width and a row-count offset, calls two subroutines, and returns success unless a bail-out flag is set.
- `store_flagged_draw_rectangle`: Optionally calls a subroutine to fetch coordinates when one state flag is set.
- `compute_scaled_blit_rectangle`: Derives an output rectangle from the requested width and height: it keeps them, scales one side to preserve the aspect ratio of the other, or uses the source extents.
- `store_flip_normalized_rectangle`: Scales one dimension by a ratio of two inputs when a flag bit selects it, converts negative width or height to positive values while recording a flip bit and shifting the origin, and returns early when the flip bits do not match the caller's mask.
- `store_ratio_flipped_rectangle`: Scales the height from the width ratio when a flag bit is set, converts negative sizes to positive while recording flip bits and moving the origin, and returns early when the flip bits do not match the caller's mask.
- `stub_return_zero`: Returns zero with no operations; the decompiled body is a bare return of a zero value, so the routine has no observable effect beyond that result.
- `build_deduplicated_row_offsets`: Fills an offset table from a source record, either adding a base pointer to each relative entry or copying entries directly when a flag bit is set.
- `assign_unique_row_slots`: Adds a base value to each nonzero entry of a source table into a destination table.
- `mark_matching_row_slots`: Writes the current value and slot into the table, scans the rest of the table marking later entries equal to that value as reused, then advances the running slot and moves on to the next distinct value until the table ends, storing the final slot and count.
- `mark_duplicate_row_entries`: Scans the remainder of a table for entries equal to the current value, storing the running slot into each matching position and marking those entries reused.
- `clip_rectangle_to_window`: Tests a rectangle against a clip box held in the state record and returns zero when it lies entirely outside.
- `stub_return_zero_status`: Returns zero immediately with no other operations; the decompiled body is only a return of zero.
- `compute_scanline_step_table`: Derives per-line fixed-point stepping values from the width and height ratios and stores them in the state record.
- `fill_bitplane_shift_table`: Computes horizontal and vertical step increments from the ratio of requested to source sizes, then loops over the rows writing each row's shift nibble and word offset into a table, accumulating the step as it goes.
- `fill_scanline_step_offsets`: Computes the starting address and row modulo from the clipped rectangle, then fills a per-row table of shifted word offsets and shift values from an accumulated step.
- `compute_blit_word_split`: Stores the base address and modulo for the blit, fills per-row shift words from an accumulated step, and splits the row width into whole 16-bit words plus a leftover partial word.
- `accumulate_and_fill_word_offsets`: Accumulates a horizontal step over a fixed number of columns, writes each row's shifted word offset and shift value into a table, splits the row width into whole words and a partial-word remainder, then calls a subroutine before continuing with the per-row loop.
- `fill_inverted_mask_word_offsets`: Writes per-row word offsets like its neighbours, but stores the complement of each shifted word ANDed with a stored mask so the result acts as a clipping mask.
- `pack_row_pixel_bits`: Splits the row width into whole 16-bit words and a partial remainder, then for each row gathers one bit per pixel from the source data and packs them into words stored in the row buffer.
- `pack_source_rows_into_words`: Loops over rows and columns, taking a single bit from each source byte at a computed offset and shifting it into a running 16-bit word.
- `advance_and_pack_rows`: Advances a row pointer by a stored stride using an accumulator, then packs the next row's bits into words with the same gather-and-shift loop as its neighbour.
- `copy_or_zero_fill_rows`: Walks each row of the table, copying stored words from a source buffer into the destination when a flag bit is set and writing zero words for the remaining columns.
- `merge_masked_row_segments`: ORs source words into a row buffer, then walks a segment table merging each segment into the destination.
- `apply_mask_to_row_segments`: Walks the row segment table, ORing source words into the destination for full-width segments and applying an AND-NOT with the mask for partial segments.
- `clear_then_merge_row_segments`: Clears the destination bits covered by the mask, then walks the segment table merging source words into each row.
- `merge_long_row_segments`: Walks the segment table for each row using 32-bit words, ORing source longs into the destination for full-width segments and combining them with an AND-NOT mask for partial segments.
- `or_source_into_long_segments`: ORs source longs into the row buffer, then walks the segment table using 32-bit words.
- `apply_mask_to_long_segments`: Walks the long-word segment table for each row, applying an AND-NOT with the mask to empty and partial segments and ORing source longs into full-width segments, and finishes when every row is processed.
- `copy_or_zero_word_segments`: Copies source words into each row segment from the segment table, writing zero words for empty segments and combining partial segments with a mask.
- `clear_then_copy_row_segments`: Zeroes each row first, then copies source words into the destination for every segment in the table, stopping when the row count runs out.
- `copy_or_zero_long_segments`: Walks the segment table for each row using 32-bit words, copying source longs into the destination for segments with data and writing zero longs for empty segments.
- `zero_row_then_copy_words`: Zeroes one row of words, then copies source words into the row for each segment in the table, zero-filling any span with no source, until the segment count runs out.
- `build_row_pointer_table`: Computes a starting byte offset from clipped source coordinates and the row size, then fills a table of per-row pointers relative to a base.
- `fill_row_pointer_table_masked`: Computes the same starting offset from a shared multiplier and row size, then fills the per-row pointer table.
- `run_masked_blit_pass`: Builds per-row pointer tables from the source and mask data, computes the first-word shift and starting row from the clipped rectangle, then drives the pass by waiting for the blitter and calling a sequence of row subroutines.
- `blit_shifted_masked_words`: Copies rows of source words into the destination after shifting each word by a bit count.
- `clear_shifted_mask_rows`: Clears destination bits in each row by ANDing with the complement of the shifted source words, handling the spill word at the end of each row.
- `clear_shifted_rows_pointer_check`: Clears destination bits from shifted source words like its neighbour, and additionally compares each row's source pointer against the previous one, skipping a row when the pointers match.
- `or_shifted_source_rows`: Ors shifted source words into the destination row by row, left-shifting each word by a bit count and handling the spill word at the end of each row.
- `or_shifted_rows_pointer_check`: Ors shifted source words into the destination like its neighbour, with a stacked row-count argument.
- `clear_right_shifted_mask_rows`: Clears destination bits by ANDing with the complement of source words shifted right by a count, with separate handling for the spill word.
- `clear_right_shifted_rows_stacked`: Clears destination bits from right-shifted source words row by row, with a stacked row-count argument and a shared row pointer table.
- `clear_right_shifted_complement_rows`: Clears destination bits in each row by ANDing with the complement of the source words shifted right by a count, handling the spill word at the end of each row.
- `clear_right_shifted_rows_checked`: Clears destination bits with the complement of right-shifted source words row by row, using a stacked row-count argument.
- `or_right_shifted_source_rows`: ORs right-shifted source words into the destination row by row, handling the spill word at the end of each row.
- `or_right_shifted_rows_checked`: ORs right-shifted source words into the destination like its neighbour, with a stacked row-count argument.
- `copy_or_zero_span_rows`: Walks the row pointer table and, for each row, either zero-fills the span or copies source words into it.
- `clear_then_or_word_rows`: Walks rows of word pairs, clearing the destination with the complement of one source word and then ORing in a second source word.
- `clear_complement_word_rows`: Clears destination words by ANDing them with the complement of the source words, row by row.
- `clear_complement_word_rows_stacked`: Clears destination words with the complement of the source words for each row, like its neighbour, but takes a stacked row-count argument and checks the source pointer against the reference before each row.
- `or_source_word_rows`: Ors source words into each row of the destination, row by row, walking the row pointer table.
- `or_word_rows_stacked`: Ors source words into destination rows like its neighbour, with a stacked row-count argument.
- `zero_or_copy_word_rows`: Walks the row pointer table and, for each row, zero-fills the span or copies source words into it, depending on whether the row's source pointer matches the reference.
- `copy_word_rows_zero_fill`: Copies one source word into the first word of each row, then copies or zero-fills the remaining columns according to the row's source pointer.
- `zero_rows_then_copy_matching`: Zero-fills each row of words first, then copies source words into rows whose source pointer differs from the reference.
- `zero_then_copy_rows_stacked`: Writes a zero word into the first column of each row, then copies source words into the remaining columns of rows whose source pointer differs from the reference.
- `clear_complement_byte_rows_shifted`: Clears destination bytes with the complement of source bytes row by row, after a one-bit alignment step derived from a stored shift.
- `clear_then_or_byte_rows`: Clears destination bytes with the complement of one source byte, then ORs in a second source byte, row by row.
- `clear_complement_byte_rows_checked`: Clears destination bytes with the complement of source bytes row by row.
- `clear_then_or_byte_pairs`: Clears destination bytes with the complement of one source byte, then ORs a second source byte in, row by row.
- `or_source_byte_rows`: Ors source bytes into each row of the destination, row by row.
- `or_byte_rows_stacked_checked`: ORs source bytes into destination rows with a stacked row-count argument.
- `configure_blitter_masks_and_modulos`: Sets the blitter's first-word and last-word masks from the edge bits of the clipped rectangle, along with its source and destination modulos and the shift value in the control register.
- `issue_blit_per_row`: Loops over the rows, loading the blitter's A, B, C and D pointer registers for each row.
- `feed_blitter_source_words`: Writes source words to the blitter's A-data register for each row, choosing among three control words by comparing the row's source pointer with the reference.
- `issue_blit_size_per_row`: Writes the blitter control word and the A-data word for each row, choosing among three control words according to whether the row's source pointer matches the reference.
- `emit_blit_record_rows`: Reads a blit record, clears its counters, and when its row count is nonzero either writes a control word and size for each row or calls subroutines to set the control words and pointers.
- `start_blit_rows_fixed_control`: Sets the blitter's control register and size for each of a counted set of rows, programming the B and D pointers and modulo for each row.
- `emit_flagged_blit_rows`: Packs a row's flag bits into a control word and clears a blit record counter when it is nonzero.
- `start_blit_rows_from_stack`: Packs a control word from two stored values, then loops over a row count writing the blitter's control register, pointers and size for each row.
- `dispatch_byte_command_handlers`: Reads a command byte and calls the handler chosen from a jump table by that byte.
- `load_sized_record_fields`: Copies a record's size and position fields into the state block.
- `load_rectangle_block_fields`: Copies four rectangle words from an attached block into the state block, reads the row count and width from the record, and multiplies the width by the row count when a flag bit is set.
- `call_three_routines_return_zero`: Calls three subroutines in turn and returns zero.
- `run_routines_optional_call`: Calls two subroutines, then calls a third only when a doubled and decremented size field from the record is non-negative.
- `run_flagged_routine_sequence`: Checks a flag bit in the state block and runs one of two subroutine sequences.
- `call_routine_return_flag`: Calls a subroutine and, when the condition flag it leaves is set, calls a second subroutine and returns one.
- `init_record_flags`: Clears several state fields, copies a pointer from the record into the state block, and sets two flag bytes when that pointer is nonzero and a record bit is set.
- `run_three_routine_calls`: Calls three subroutines in turn and returns zero, with no logic of its own beyond the three calls.
- `run_routines_size_check`: Calls two subroutines, then calls a third when a size field derived from the record is non-negative.
- `copy_long_table_into_state`: Reads a count word from a table pointer in the record and stores it in the state block, then copies that many 32-bit entries from the table into the state block, stopping once the count runs out.
- `copy_long_pairs_into_state`: Stores a count word from a table pointer in the record into the state block, then copies the table's 32-bit entries into it two at a time.
- `find_coordinate_min_max_pairs`: Scans a counted list of coordinate pairs, tracking the running minimum and maximum of each channel with signed 16-bit comparisons.
- `find_coordinate_min_max_counted`: Scans a list of coordinate pairs whose count is held in a register, tracking the running minimum and maximum for each channel.
- `parse_draw_command_table`: Stores a table pointer from the record and reads a mode byte.
- `decode_flag_word_from_record`: Sets a state field to a sentinel, copies the record's flag word into the state block, and returns either a stored 64-bit value or the record's flag word masked to its high bits.
- `stub_empty_return_void`: Returns immediately; the decompiled body is empty, so the routine has no observable effect.

### Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_then_halt`: Calls an operating-system library routine through a stored pointer twice, then enters an endless loop so control never returns.
- `return_failure_status`: Returns all bits set, a failure result, without performing any other work.
- `wait_for_fire_button_release`: Checks the fire-button bit of a global input byte and, when it is set, stores a pointer and a value, calls subroutines, then waits until a flag byte is zero and the fire-button bit is clear before one final subroutine call.
- `open_graphics_and_setup_view`: Forbids multitasking, opens the graphics library through the exec library, and runs a sequence of graphics calls that set up the display, storing the library base and resulting values in the state area.
- `build_copper_list_enable_dma`: Clears all interrupt and DMA enables, fills eight copper-list pointer entries from an offset table, and installs a handler pointer.
- `fill_stepped_value_table`: Fills a table of four-byte slots with a running value that grows by a fixed step, writing the high and low halves of each value into the slot, until the count register runs out.
- `restore_display_and_close_graphics`: Restores the saved display and DMA registers, clears the audio volume registers, closes the graphics library and permits multitasking again.
- `acknowledge_interrupt_after_routines`: Sets a global byte to all-ones when the fire-button bit is clear, runs five subroutines, and then writes the interrupt-request register to acknowledge the pending interrupt.
- `tick_counter_and_check_threshold`: Decrements a global countdown byte when it is nonzero, otherwise calls a subroutine.
- `advance_text_cursor_per_character`: Reads characters from a text pointer, advancing the vertical position by a fixed amount for a line-feed and the horizontal position by each printable character's width from a table.
- `set_text_line_defaults`: Stores the default horizontal and vertical positions for a text line, sets a fixed width constant, and calls subroutines to fill in the remaining fields.
- `store_text_position_from_routine`: Calls a subroutine to compute a value and stores it as the current text position field in the state block.
- `store_pointer_and_clear_field`: Stores an incoming pointer into a state field, calls a subroutine, and then clears a 16-bit state field.
- `measure_centered_text_width`: Starts from a fixed width and subtracts each character's width, taken from a table indexed by character code, until a code below the space character ends the string.
- `compute_text_block_offset`: Counts line-feed characters in a text string up to its terminator and returns a start offset that moves up by a fixed step for each line.
- `clear_line_buffer_block`: When a row counter is nonzero, zeroes a block of a fixed-stride buffer by writing zero longs backwards from the end of the selected row.
- `build_copper_wait_band`: Builds copper-list entries for a band of scanlines, writing wait instructions and register moves in pairs with a per-line vertical position.
- `sample_values_track_change`: Toggles a parity byte on each call and, on alternate calls, samples a subroutine value three times into a table, setting a change flag when the samples disagree.
- `step_packed_fields_toward_target`: Walks the packed four-bit fields of two inputs.
- `pack_sprite_control_words`: Packs horizontal and vertical start and stop values, along with their high bits, into a two-word header laid out like a hardware sprite control block.
- `write_sprite_control_header`: Packs a position pair and a height into a two-word sprite control header, setting the high bits of the start and stop fields, and writes both words to the buffer pointed to by the argument.
- `init_default_state_fields`: Stores a set of default values into the state block: two pointers to a fixed data area, two size fields, two width fields and a control word, followed by one further default pointer.
- `advance_sequence_stream_byte`: Counts down a timing field and, when it expires, toggles a phase bit and reads the next byte from a data stream, with a zero byte resetting the timing and a high-bit byte selecting alternative timing.
- `decode_stream_symbol`: Reads the next byte from a data stream, with a space code setting a timer and returning the space, and other codes searched in a fixed symbol table and mapped to a code value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_dos_library_then_close`: Clears a global flag, opens a library through the exec library, closes it again, and calls a final library routine before returning zero.
- `run_workbench_handoff_loop`: Closes the Workbench screen through a wrapper, waits for display frames in a bounded retry loop, and then runs a state loop that calls subroutines until a terminal value is reached.
- `select_memory_mode_from_availability`: Calls the exec memory-availability function and compares the returned size against two thresholds.
- `copy_indexed_string_with_separator`: Copies a null-terminated string, selected by an index from a pointer table, into the output buffer.
- `open_math_tridee_vector_libraries`: Opens the math, tridee and vector libraries through the open-library wrapper and stores each returned library base in its own global slot.
- `call_three_routines_void`: Calls three subroutines in turn and returns, with no other logic in the decompiled body.
- `call_indirect_via_d0`: Checks if a register contains a non-zero value and then calls a function pointer stored at an offset from a global variable.
- `call_open_library_2`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `install_handler_run_library_calls`: Installs a handler pointer and a buffer offset in the state block, runs a subroutine, and makes library calls through a stored library base when the result is nonzero.
- `clear_state_run_library_calls`: Zeroes three state fields and runs a subroutine.
- `clear_state_run_script_calls`: Zeroes three state fields and runs the same library-call sequence as its neighbour.
- `reset_state_then_library_calls`: Zeroes three state fields, runs a subroutine, and then makes the same conditional library calls through a stored base as its neighbours.
- `return_global_with_failure_high`: Returns a global value paired with an all-ones high word.
- `dispatch_on_marker_word`: Compares a marker word in the caller's record against a fixed code, then calls one of two stored handler pointers depending on the result.
- `expand_tilde_template_string`: Copies a template string into a buffer, expanding tilde escapes: a tilde followed by n ends the copy, and another escape inserts a stored name followed by a colon.
- `lock_and_call_loaded_segment`: Calls a library function through a stored base repeatedly until it reports success, then calls the loaded routine with the length of a string argument.
- `open_dos_graphics_intuition_libraries`: Opens the DOS, graphics and intuition libraries through the exec library, storing each returned library base in its own state slot.
- `close_stored_library_bases`: Closes three stored library bases through the exec library, skipping any that are zero, and returns the result of the last close.
- `call_close_work_bench`: Wrapper that calls the intuition library function CloseWorkBench and returns its result to the caller.
- `call_open_work_bench`: Wrapper that calls the intuition library function OpenWorkBench and returns its result to the caller.
- `mark_cell_and_update_bounds`: Checks that a column index lies within a fixed range and traps if it does not, then sets the top two bits of a byte at a computed offset.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `recompute_rect_bounds_if_flagged`: Calls a subroutine and, when it returns nonzero, recomputes four stored rectangle bounds from the current extents, clamping the width at zero and halving the results.
- `close_screen_and_window`: Clears the pointer, then closes the screen and window through the intuition library when their handles are set.
- `open_screen_window_palette`: Derives a screen height from a stored value clamped to a maximum of 256, opens a screen and window through the intuition library, loads the colour palette, and brings the screen to the front.
- `set_intuition_pointer`: Calls the intuition library's pointer-setting function through a stored library base, passing the current pointer data.
- `patch_library_vector_clear_cache`: Forbids multitasking, saves a library function vector and its checksum data, redirects a library entry to a stored replacement, clears the processor cache and permits multitasking again.
- `restore_library_vector_clear_cache`: Restores the saved library vector values into the library entry, sets a flag bit in the library record, and clears the processor cache through the exec library.
- `return_minus_eleven_status`: Returns the constant minus eleven, a fixed status value, with no other work in the decompiled body.
- `stub_no_op_handler`: Does nothing and returns; its address is referenced elsewhere, so it acts as a no-op handler entry.
- `stub_no_op_callback`: Does nothing and returns; its address is stored as a callback, so calling it has no effect.
- `scan_digit_codes_with_callback`: Checks a state word against a stored value, then calls a routine with the input text and scans its characters for digit codes zero to four until a terminator or colon.
- `signal_task_after_callback`: Calls a stored callback when one is set, then sends a signal through the exec library.
- `wait_and_drain_pending_callbacks`: Sends a signal through the exec library, repeatedly calls a subroutine while a pending counter is nonzero, and then calls a stored callback when one is set.
- `start_disk_watch_task`: Allocates a signal bit and a memory block for a task node, fills the node with the signal masks and a task name, and adds it to the system task list through the exec library.
- `release_task_resources`: Removes the watch task from the task list, frees its signal bit and memory block when they are present, and calls stored cleanup routines through the exec library.
- `wait_for_signal_then_setup`: Waits through the exec library for one of two signal bits, then marks a state byte, stores a value, and calls setup subroutines.
- `start_task_state_sequence`: Marks a state byte, stores a value and calls a library function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `find_current_task_and_setup`: Looks up the current task through the exec library, stores it with a signal value in the state, and runs the same subroutine sequence as its neighbour, which includes removing an interrupt server through a stub.
- `write_chunked_block_to_handle`: Obtains a handle through a library call and, when the handle is valid, writes a total of 179 bytes through a stored routine in chunks of at most 80 bytes.
- `skip_leading_spaces`: Advances a text pointer past space characters, stopping at the end of the counted input length.
- `split_command_line_tokens`: Splits a counted command line into tokens, with spaces and line-feeds as separators and double quotes grouping text.
- `store_optional_resource_pointers`: Calls a subroutine to get a pointer and stores it in a state field only when it is nonzero.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `open_library_via_exec`: Calls the exec library's open-library function through a stored base and returns its result.
- `copy_two_words_from_subrecord`: Copies two 16-bit fields from a sub-record pointed to by the state block into fixed slots in the state block.
- `close_handle_if_set`: Calls three subroutines, then calls a library function on a stored handle when that handle is nonzero.
- `close_library_via_pointer`: Checks if a value is non-zero and, if so, executes a function pointer stored at an offset from a global variable.
- `call_add_int_server`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `halt_in_infinite_loop`: Loops forever with no work, so control never returns from this segment entry.
- `increment_counter_clear_flag_bit`: Increments a 16-bit counter in the record and clears bit three of a flag byte in the same record.
- `advance_counter_clear_flag_bit`: Increments a 16-bit counter and clears bit three of a flag byte in the record, with the same effect as its neighbour.
- `start_task_state_sequence_alt`: Stores a value and a state byte, calls a library function, and when its result is nonzero runs a subroutine sequence chosen by a stored flag before setting a status word.
- `write_chunked_block_copy`: Obtains a handle through a library call and, when valid, writes a total of 179 bytes through a stored routine in chunks of at most 80 bytes.
- `skip_leading_spaces_alt`: Advances a text pointer past space characters, stopping at the end of the counted input length.
- `split_command_line_tokens_alt`: Splits a counted command line into tokens separated by spaces and line-feeds, with double quotes grouping text.
- `store_optional_resource_pointers_alt`: Calls subroutines to fetch pointers and stores them in state fields, storing each only when it is nonzero.
- `call_exec_stub_through_base`: Calls a function through a stored exec library base pointer and returns, with no other logic.
- `open_library_via_exec_copy`: Calls the exec library's open-library function through a stored base and returns, the same pattern as the stub before it.
- `copy_two_words_subrecord_alt`: Copies two 16-bit fields from a sub-record pointed to by the state block into fixed slots in the state block.
- `close_handle_if_set_alt`: Calls three subroutines, then calls a library function on a stored handle when that handle is nonzero, returning the function's result or zero.
- `call_library_function_via_pointer`: Checks if a value is non-zero and then executes a function through a pointer stored at an offset from a global variable.
- `add_interrupt_server_via_exec`: Calls the exec library's add-interrupt-server function through a stored base, passing its arguments through unchanged.
- `remove_interrupt_server_via_exec`: Calls the exec library's remove-interrupt-server function through a stored base, passing its arguments through unchanged.

### logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf

- `tiny_ori_2`: Small routine of 1 instruction that merges 29 into d0.
- `walk_index_chain_and_call`: Walks a chain of table entries, scaling the stored index by four at each step, until it reaches a zero entry.

### logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf

- `traverse_and_call_vector`: Traverses a table of pointers until a null entry is found and then calls the function at the next offset.
- `walk_index_chain_via_stub`: Walks a chain of table entries, scaling the stored index by four at each step, until a zero entry, then calls the routine stored just past it.

### logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a2].adf

- `walk_index_chain_copy`: Walks a chain of table entries, scaling the stored index by four at each step, until it reaches a zero entry.
- `walk_index_chain_stub_copy`: Walks a chain of table entries, scaling the stored index by four at each step, until a zero entry, then calls the routine stored just past it.
