# Game 1973 (captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01973). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (920240 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 26 raw-word lines |
| Functions named and described | pass, 416 of 416 named (0 author symbols), 416 of 416 described (20 by rule, 51 by the local model) |
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
| Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf:HardDisk_Install | 3 | 10 | 1 |
| math.library | 1 | 807 | 27 |
| tridee.library | 1 | 2181 | 93 |
| vector.library | 1 | 3569 | 184 |
| Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 516 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 655 | 38 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 162 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 173 | 15 |
| logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf | 1 | 11 | 2 |
| logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf | 1 | 11 | 2 |
| logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf | 1 | 11 | 2 |

## Cases handled

- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf:StartUp.asp: high-entropy data file (7.84 bits/byte), classified as packed data
- runtime segment an address: 406 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 375 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 50 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf: 9 executed PCs whose RAM was overwritten before any snapshot match its bytes 230976-231072: recovered as a unit run at an address
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf: 9 executed PCs whose RAM was overwritten before any snapshot match its bytes 230976-231072: recovered as a unit run at an address
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf: 9 executed PCs whose RAM was overwritten before any snapshot match its bytes 230976-231072: recovered as a unit run at an address
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf: 3 logged loads cover 0 bytes of the image
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf:c/PlayASP: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- captive ii - liberation (1992)(mindscape)[cr dlm](disk 1 of 5)[a]/Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf:HardDisk_Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `spin_forever_after_two_calls` (runtime segment an address): Makes two calls through a pointer in the segment, one of them into an operating system library, and then loops forever with no exit. Kind: anti-tamper/crack detection trap. Pass path: not determined.

## Functions


### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_header_block_and_call`: When the destination pointer argument is nonnull, copies sixteen 32-bit entries from a fixed block in the state into it.

### Liberation

- `program_entry_chain_dispatch`: Program entry point that walks a chain of table entries, scaling the stored index by four at each step, until it reaches a zero entry.
- `decode_packed_block_and_free`: Runs a bit-stream decoding loop over a packed block, shifting a bit buffer through the carry flag, then frees a memory block and clears the processor cache.
- `decode_bitstream_codes_to_output`: Reads bits one at a time from a packed input through a shifted bit buffer, decoding each code into a run of output bytes.

### delirium

- `load_amigados_hunks_and_relocate`: Program entry for the loaded executable that parses AmigaDOS hunk records.
- `delay_loop_countdown`: Busy-waits by decrementing a 16-bit count from its starting value until it wraps past zero, giving a timed delay with no other work.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_backward_literal_runs`: Decodes runs from a packed stream by reading control bits.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bits_from_stream_word`: Shifts bits out of a multi-word bit buffer one count at a time, fetching the next 32-bit word from the input when the buffer empties.
- `stub_bad_instruction_data`: Only a few bytes decode before the instructions become invalid, so this is data rather than a real routine.

### PlayASP

- `walk_chain_and_dispatch`: Follows a chain of pointer-linked records from a fixed table address, scaling each link value, until a zero link is reached.

### Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf:HardDisk_Install

- `walk_installer_chain_and_dispatch`: Performs the same chain walk as the playback entry, starting from a different fixed table address in the installer, and then calls the handler in the final record.

### math.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `store_two_longs_in_record`: Library code that writes its two longword arguments into the record addressed by the first data register, at byte offsets 40 and 36.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_countdown_and_call`: Decrements a 16-bit countdown held in the record and, when it reaches zero and a flag bit in the record is set, calls another library routine and returns its result.
- `run_callbacks_or_set_pending`: When the countdown field in the record is zero, it reads a value from the record and invokes two library function pointers, returning that value.
- `remove_node_and_free_memory`: Reads the value at offset 36 of the record, then calls the Amiga exec library functions FreeMem and Remove, which unlink the node from its list and release its memory.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `lookup_table_word_by_index`: Returns the 16-bit word from a library data table at an index formed by the low nine bits of the input value, doubled to a byte offset.
- `scale_offset_point_in_place`: Rescales the x and y values in the record by a factor taken from the fifth field, divided by the sum of the third and fourth fields.
- `swap_word_halves_store`: Swaps the upper and lower 16-bit halves of the data register value and stores the result at the address held in the second address register.
- `scale_point_with_zero_guard`: Scales the x and y values in the record by factors from the third and fourth fields, dividing each by a per-axis sum and skipping the division when that sum is zero.
- `stub_return_immediately`: It is library code in libs/math.library that only returns to its caller and performs no work.
- `stub_return_immediately_2`: The decompiled body is only a return, although the decoded span covers many instructions, so nothing observable happens here.
- `return_from_subroutine_2`: This routine performs a standard subroutine return sequence.
- `split_value_into_hundreds_pair`: Calls another library routine with a fixed argument of minus one, then packs its returned value as a quotient and remainder by one hundred, the quotient in the upper half and the remainder in the lower half.
- `advance_linear_congruential_state`: Optionally loads a 16-bit value into a library global when its argument is non-negative, then advances that global with a multiply-and-add step using fixed constants.
- `build_rotation_matrix_from_angles`: Builds a 3x3 rotation-style matrix in fixed-point form from three angle values, using sine and cosine lookups from the library table.
- `build_rotation_matrix_alternate_order`: Builds a nine-element fixed-point matrix from three table-driven angle values, but multiplies the factors in a different order and sign pattern from its neighbour.
- `stub_return_immediately_3`: Returns its caller without storing or computing anything, so the routine has no effect.
- `multiply_fixed_point_matrices`: Multiplies two 3x3 matrices of 16-bit fixed-point values and stores the result as nine 16-bit values in the output record.
- `integer_square_root_bisection`: Computes the integer square root of its 32-bit input by bisecting over 16-bit candidates and keeps the largest candidate whose square does not exceed the input.
- `refine_square_root_bounds`: Continues an integer square-root bisection, narrowing a pair of 16-bit bounds until they are adjacent, and returns the lower bound in the low word.
- `return_constant_one`: Returns a fixed value of one in the high word, passing the other argument through unchanged.
- `interpolate_inverse_table_lookup`: Takes the absolute value of its input and binary-searches a sorted 16-bit table for it, then linearly interpolates between the two neighbouring entries to give a fractional index.
- `bisect_table_and_interpolate`: Binary-searches a sorted 16-bit table, passed by address, for the input value and interpolates between the bracketing entries.
- `interpolate_between_table_entries`: Computes the fractional position of the input value between two table entries using a 16-bit fixed-point division.
- `compute_clamped_face_normal_dot`: Forms difference vectors from three points, takes their cross product and dots it with a fourth vector, then scales the result by values stored in the record.

### tridee.library

- `return_constant_minus_one`: Returns a constant of minus one as a 32-bit result.
- `store_two_fields_in_library`: Stores two longword arguments into fixed fields of a structure addressed through a register, at offsets 24 and 28 bytes.
- `increment_counter_and_clear_flag_2`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_counter_then_conditional_call`: As library code, it decrements a 16-bit usage counter in a structure and calls a follow-up routine if the counter reaches zero and a specific flag bit is set.
- `release_structure_block`: As library code, it checks a field in a structure and, if zero, releases the structure using two library calls and returns a saved value.
- `free_block_and_return_value`: Reads a longword stored at offset 20 of a structure, passes the structure to two system library calls that release it, and returns the saved value.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `open_math_library_once`: Increments a usage count and, on first use, opens the math.library shared library through the exec library call and stores its base for later use.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `decrement_and_close_library`: Decrements a global variable and, if it reaches zero, calls a function through a pointer based on a bit test.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `allocate_and_init_entry_buffer`: Allocates a record sized from an entry count in its input, plus a fixed header, then sets its size, stride and pointer fields for either a plain or an extended parameter layout.
- `allocmem_and_init_entry_buffer`: Allocates a record through the exec AllocMem call and fills in the same size, stride and pointer fields from the parameter block, depending on a flag in that block.
- `reinit_entry_buffer_fields`: Recomputes the entry count, stride and pointer fields of an existing record from a size value in the parameter block, without allocating anything.
- `call_then_return_argument`: Calls another library routine for its side effect, then returns the value that was passed in as its argument.
- `clear_pending_and_call_library`: If a pending-state field in the record is set, it clears that field and calls a library function with a small code, then makes a second library call selected by a flag bit.
- `free_memory_via_exec_sequence`: Makes exec library calls in a fixed sequence, choosing between two call forms by a flag bit, and then calls another routine.
- `free_memory_then_call_next`: Calls the exec memory-release function (FreeMem) through the library base, then calls another routine and returns.
- `allocate_entry_buffer_by_count`: Allocates a record sized from the entry count in the low word of its input at 84 bytes per entry, then sets its size, stride and pointer fields.
- `allocate_entry_buffer_from_size`: Allocates a record through the exec AllocMem call and sets its size, stride and pointer fields from the size argument.
- `return_passed_argument`: Returns the value passed on the stack without changing it.
- `reset_record_bounds_sentinels`: Resets a record's position fields to zero and sets its four bound fields to the largest and smallest 16-bit values, so that later updates shrink or grow them to the real extent.
- `transform_point_list_with_divide`: Walks a list of points, adds each point to the record's offset vector and stores the translated point back.
- `compute_weighted_component_sum`: Multiplies three input components by three record fields, skipping any zero component, and sums the products.
- `scale_point_by_divisors`: Multiplies each 16-bit coordinate by a scale field, divides by a sum including an offset field when that sum is non-zero, then adds an offset field to each.
- `copy_record_state_and_flags`: Copies a fixed block of fields from the source record into the destination record, then returns a result word built from the source flag bits.
- `copy_saved_fields_when_flagged`: If the bit-two flag is set in the source record, it returns that bit at once.
- `copy_saved_fields_on_flag`: When either of the two low flag bits is set in the source record, it copies five saved fields into the destination and returns the bit-four value.
- `stub_return_immediately_6`: As library code, it returns without doing any work.
- `stub_return_immediately_4`: It is library code in libs/tridee.library that is empty.
- `store_split_words_into_record`: Stores three 16-bit parts of its two arguments into fixed fields of the record, writing the low and high words of the first argument and the low word of the second.
- `load_object_settings_and_dispatch`: Copies a block of settings from the input record into the object, initialises two limit fields to extreme values, and calls helper routines selected by flag bits in the object.
- `interpolate_node_byte_fields`: Walks a linked list of nodes and, for nodes with the expected flag bits set, moves a byte field toward a target by a proportional step derived from a range in the record divided by a span.
- `interpolate_node_fields`: Iterates through a linked list of nodes, updating specific byte and word fields based on flags and calculated scaling values.
- `return_empty`: This library routine is empty.
- `extend_bounds_from_node_list`: Walks a linked list of nodes and, for each node that passes a test, widens the record's four bound fields to include that node's extents.
- `extend_bounds_and_relink_nodes`: Walks a linked list of nodes, clearing each link as it goes, and widens the record's four bound fields to include each node that passes the test.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a4, then returns.
- `update_byte_field_with_projection`: Computes a byte field from a projection term subtracted from a record value, clamps negative results to zero, and writes the sum into the record.
- `update_byte_field_when_flagged`: When a flag bit in the library state is set, it computes a clamped projection-derived byte and stores it in the record.
- `update_byte_field_with_rounding_2`: As library code, it calls another routine to update a byte field with rounding.
- `update_byte_field`: This library routine calls another routine to update a byte field with rounding.
- `update_byte_field_with_rounding`: Stores a low byte from the input into the record unless a flag bit is set, then computes a clamped projection-derived byte with a rounding offset of two and writes it to the record.
- `store_clamped_projection_byte`: Subtracts a projection-derived term from the input byte, clamps negative results to zero, and stores the sum with the record's base byte.
- `store_clamped_projection_byte_word`: Subtracts a projection-derived term from the low byte of the input word, clamps negative results to zero, and stores the sum with the record's base byte.
- `return_from_subroutine_3`: This library routine is empty and performs no operations before returning.
- `calculate_clamped_projection`: This library routine subtracts a projection-derived term from an input byte, clamps the result to zero if negative, and stores the sum into a record.
- `compute_list_min_max_bounds`: Finds the smallest and largest 16-bit values among the list entries referenced by the record, shifts them by a flag-selected amount, and stores them in the record.
- `compute_list_bounds_flat`: Finds the minimum and maximum 16-bit values in a list of entries, shifts them by a flag-selected amount, and widens the library bound fields.
- `clip_and_emit_polygon_vertices`: Clips an edge list against bounds and emits the resulting vertex records into an output buffer, updating its count.
- `clip_emit_vertices_with_offset`: Applies an offset to a point, clips it against bounds and emits the resulting vertex records into an output buffer, updating its count.
- `project_point_with_fixed_division`: Scales two point components by a divisor and tests whether the squared length falls within a bound, swapping two stored vectors when it does.
- `divide_fixed_point_long_division`: Rotates two components with a sign-adjusted multiply, then performs a bit-by-bit long division of a squared value by a remainder term to produce a fixed-point quotient.
- `divide_difference_fixed_point`: Computes a fixed-point quotient of a scaled value divided by the difference between two inputs, using a bit-by-bit long division.
- `divide_shifted_value_by_divisor`: Divides a shifted 32-bit value by a 16-bit divisor with a bit-by-bit long division, returning the quotient scaled by the shift count.
- `compute_gated_squared_ratio`: Computes a ratio of products of two squared components to a weighted sum of the squares, with fixed-point shifts.
- `compute_squared_ratio_word`: Computes the ratio of a product of two squared components to a weighted sum of the squares, returning the low word of the result.
- `compute_scaled_squared_ratio`: Divides a product of two squared terms by a sum built from the stored and input squares, using fixed-point scaling.
- `compute_scaled_difference_ratio`: Computes the difference between two 16-bit values, scaled by a third value and divided by the first value, and returns the low word.
- `project_edge_and_cull_test`: Rotates a 2D offset pair from sign-normalised input components and stores the results in the record.
- `return_sentinel_with_record_value`: Returns minus one in the high longword together with a longword read from the record in the low word.
- `compute_squared_edge_length`: Computes the squared length of the difference between two vertices in the coordinate table and returns its absolute value.
- `store_vertex_and_bounds_terms`: Stores an input value and a zero into the record, computes a coordinate difference from the vertex table, and derives paired sum and difference terms for two axes.
- `compute_vertex_list_extents`: When a visibility gate in the record passes, scans a list of vertices for the smallest and largest x and y values and stores the four extents in the record.
- `return_minus_one_with_argument`: Returns minus one in the high longword together with the low longword of its second argument.
- `clip_and_emit_edge_vertices`: Clips an edge against bounds and emits the resulting vertex records into the output buffer, updating its count.
- `clip_polygon_vertex_loop`: Loops over an index list of vertices, clipping each edge against bounds and emitting vertex records into the output buffer.
- `clip_vertex_loop_with_gate`: Runs the same clipping loop over a vertex index list, entered through a gate that checks the first vertex before emitting records.
- `clip_loop_return_squared_length`: Clips a polygon edge loop against bounds and emits the clipped vertex records into the output buffer.
- `return_minus_one_with_value`: This library routine returns a longword containing negative one in the high word and a value from a structure offset in the low word.
- `clip_point_list_into_buffer`: Clips a list of points against bounds, substituting open limits when the first coordinate is non-negative, and emits vertex records into the output buffer.
- `clip_point_list_bounded`: Clips a list of points against bounds given as arguments and emits vertex records into the output buffer.
- `compute_squared_vertex_pair_distance`: Computes the squared distance between the two vertex pointers stored in the record, using their three coordinates, and returns its absolute value.
- `calculate_vertex_transform`: As library code, it performs arithmetic on vertex coordinates and stores the results in a destination structure.
- `compute_extents_with_gate`: Computes the smallest and largest x and y values over a vertex list and stores them in the record.
- `clip_and_emit_segment_vertices`: Clips a segment against bounds and emits the resulting vertex records into the output buffer, updating its count.
- `clip_vertex_list_second_pass`: Clips a vertex list against bounds, emitting records into the output buffer, and when a sign flag is negative it runs a second pass over the emitted entries.
- `clip_vertex_list_with_fallback`: Clips a vertex list through an entry gate that tests the first vertex, emitting records into the output buffer, and then runs a second pass on flagged input.
- `call_helper_and_return`: Calls another library routine and then returns.
- `sort_node_list_by_key`: Sorts a linked list of nodes in place by the 16-bit key stored in each node, repeatedly swapping adjacent out-of-order nodes until a pass makes no change.
- `split_polygons_against_plane`: Walks a list of polygon nodes and tests pairs whose bounds overlap.
- `classify_polygon_against_plane`: Finds a candidate node with overlapping bounds, builds a plane from up to three of its vertices, and checks which side the next node's vertices lie on.
- `compute_polygon_plane_side_test`: Builds a plane from a polygon's vertices and tests the vertices of the following nodes against it, collecting those on one side.
- `collect_polygons_on_plane_side`: Builds a plane from a polygon and collects nodes whose vertices fall on one side of it, emitting the matching vertices.
- `accumulate_polygon_edge_sign_mask`: Tests whether a polygon node lies across a plane and whether successive edge cross products change sign, accumulating the results into a bit mask.
- `check_polygon_edge_signs_loop`: Computes edge cross products for a polygon and checks for sign changes between consecutive edges, recording the outcome in a mask, then advances through the node list.
- `check_polygon_edge_signs_second`: Repeats the polygon edge sign test over a node list, setting mask bits for sign changes and moving to the next polygon when the list ends.
- `relink_polygon_nodes_by_sign`: Runs the edge sign test on candidate polygon nodes and relinks them in the list according to the result.
- `relink_and_test_polygon_plane`: Relinks a polygon node in the list, builds a plane from its vertices, and classifies the following nodes against that plane.
- `swap_nodes_and_test_plane`: Swaps the current node with its neighbour in the list, then builds a plane test over the following nodes and collects the ones that match.
- `return_from_subroutine_4`: This library routine performs a single return instruction to exit the current execution flow.
- `interpolate_between_ordered_pair`: Orders two 32-bit inputs so the smaller comes first, then linearly interpolates the 16-bit value in the second word between their coordinates.
- `stub_return_immediately_5`: A routine consisting of a sequence of instructions that performs no observable operation before returning.

### vector.library

- `return_negative_one`: This library routine returns negative one.
- `store_two_fields_in_structure`: This library routine stores two longword arguments into a structure at specific offsets.
- `increment_countdown_clear_flag`: Increments the 16-bit countdown held in the record and clears a pending flag bit in the record.
- `decrement_usage_counter_and_check_flag`: Decrements a 16-bit usage counter within a library structure.
- `free_memory_block`: As library code, it checks a status field and, if zero, releases a block via two library calls and returns a saved value.
- `free_block_and_return_value_2`: This library routine reads a longword from a structure, passes the structure to two system library calls to release it, and returns the saved value.
- `return_zero_value`: Returns zero.
- `allocate_and_init_vector_object`: Allocates an object record whose size is derived from the input dimensions and flag bits, using a library allocation call, and initialises its fields from the arguments.
- `initialize_vector_object`: Allocates a memory block and populates it with coordinate, velocity, and flag data derived from input arguments.
- `allocmem_and_init_vector_object`: Allocates an object record through the exec AllocMem call, fills in its size, position and flag fields from the arguments, and when a flag is clear allocates and fills a second block.
- `allocmem_fill_vector_object`: Allocates an object record through the AllocMem call and fills its size, position and flag fields from the arguments, returning the record.
- `call_other_routine`: Calls another routine and returns its result.
- `fill_pointer_table_with_stride`: Writes a sequence of addresses into a pointer table, each one a fixed stride further on than the last, for the number of entries given in the argument.
- `fill_array_with_arithmetic_ramp`: Writes a running sequence of 16-bit values into an array, starting at zero and adding a step taken from the record for each entry.
- `fill_array_with_arithmetic_sequence`: Writes an arithmetic sequence of 16-bit values into an array, starting from the first argument and adding a step for each entry, for the count given.
- `conditional_library_calls_on_pending`: If a pending field in the record is nonzero, it makes one library call; it then makes a second library call, choosing between two call forms by a flag bit.
- `free_mem_then_exec_call`: Calls the exec FreeMem function through the library base, then makes a second exec call selected by a flag bit.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `run_node_handlers_and_sum`: Walks a linked list of nodes, skipping nodes already marked, and calls a handler chosen by each node's type bits, summing the returned values.
- `iterate_node_handlers_total`: Walks a node list in the same way, invoking the per-node handler and accumulating a 16-bit total in the record.
- `dispatch_node_list_accumulate`: Walks a node list, calling the handler for each node through a type-indexed table and adding the result into the running total.
- `process_node_list_total`: Walks a node list, computing a handler result for each node that is not marked and adding it to the record's running total.
- `call_library_pair_by_flag`: Calls one of two library routines depending on a flag bit, then makes further conditional calls on the same bit.
- `execute_conditional_library_calls`: Checks a bit in a byte, then executes a sequence of library calls based on the result of that check and subsequent conditions.
- `call_optional_helper_return_one`: Calls one library routine, then calls a second one only when a condition flag is set, and returns one in either case.
- `call_conditional_helper_then_one`: Calls one library routine and then a second one only when the condition flag is set, returning the constant one.
- `call_helper_flag_return_result`: Calls one library routine and returns its result, unless a condition flag is set, in which case it calls a second routine and returns one.
- `copy_entry_values_to_array`: When a pending field in the record is negative, it copies a count of 32-bit values from the record's entries into the output array, choosing between two address layouts by a flag bit.
- `copy_pointed_values_to_array`: Copies a count of 32-bit values from a table of record pointers into the output array, reading each entry at a fixed offset within the record it points to.
- `write_clipped_point_bitplanes`: Waits for the blitter, then for each point inside the record's clip rectangle sets or clears a single bit in each bitplane, using a mask derived from the point and the record's source data.
- `write_clipped_point_bits_direct`: Performs the same clipped per-point bit write across the bitplanes without first waiting for the blitter, and returns one.
- `write_clipped_bit_runs`: Writes a run of bits into each bitplane at successive positions taken from a list, skipping points outside the clip rectangle.
- `rasterize_edge_span_endpoints`: Steps an integer edge walk between the record's endpoints and, for each step inside the clip rectangle, stores clipped span endpoints into a table.
- `rasterize_edge_endpoints_flagged`: Performs the same edge walk as its neighbour, first storing an input flag word in the record and then recording clipped span endpoints.
- `rasterize_edge_endpoints_full`: Steps a line-style edge walk over the record's endpoints, recording clipped span endpoints into the table and using sentinels for out-of-range rows.
- `step_edge_span_pair_endpoints`: Steps a Bresenham-style edge walk between two endpoints and stores clipped left and right span endpoints at each step, using sentinels for rows outside the clip rectangle.
- `step_edge_span_pair_swapped`: Steps the same edge walk with the axes swapped, storing clipped endpoints for each stepped row and returning one when the walk completes.
- `step_span_pair_alt_order`: Runs the edge walk again with the endpoint-recording order changed, storing clipped endpoints and sentinels, and returns a value depending on the clip test at the end.
- `step_span_pair_branch_a`: Steps an edge walk between two endpoints using one decision-variable branch and stores clipped span endpoints for each row, with sentinels outside the clip rectangle.
- `step_span_pair_branch_b`: Runs the same edge walk with a different decision ordering, storing clipped span endpoints per row and returning zero or one from the clip test.
- `step_span_pair_branch_c`: Runs the edge walk with another decision ordering, storing clipped span endpoints and sentinels per row until the walk completes.
- `check_span_range_and_return`: When a flag word in the library state is set, it tests a span range against the clip limits, returning zero if the span lies outside them and otherwise calling a further routine.
- `return_zero`: This library routine is empty and simply returns a zero value to the caller.
- `return_minus_one_when_negative`: Returns minus one only when two negative flag words in the library state and record are both set, and returns zero otherwise.
- `clamp_record_bounds_to_clip`: Clamps the record's four bound fields against the clip limits and stores the results in the library state, so later stages work with the clipped box.
- `clip_polygon_against_rectangle`: Clips a polygon's vertex list against the horizontal and vertical bounds in turn, interpolating new vertices where edges cross and writing the survivors into the output buffer.
- `clip_vertex_list_against_edge`: Clips a list of vertex values against one boundary, interpolating a crossing vertex where an edge crosses it, and writes the resulting list to the output buffer.
- `clip_vertex_list_against_bound`: Clips a vertex list against a single bound, interpolating crossing vertices and writing the survivors to the output buffer.
- `return_zero_2`: This library routine performs no operations and returns a value of zero to the caller.
- `clip_vertices_into_rectangle_bounds`: Copies the record's vertex data into the clip buffer when the bound test passes, then clips the vertex list against the rectangle's edges, interpolating crossing points.
- `copy_and_clip_vertex_pairs`: Copies pairs of 32-bit vertex values from a table of record pointers into a buffer and clips them against the bounds.
- `clip_vertex_pairs_against_bound`: Clips a vertex list against a bound, writing the surviving and crossing vertices to the output buffer and storing a flag word in the record.
- `clip_vertex_list_lower_bound`: Clips a vertex list against a lower bound taken from the record, interpolating crossing vertices and writing survivors to the output buffer.
- `store_word_return_zero`: Stores its 16-bit argument into a field of the record and returns zero.
- `fill_edge_span_table_values`: Walks a list of vertex pairs, tracks the overall minimum and maximum of their x values, and fills a per-row table with values interpolated along each edge.
- `interpolate_span_table_rows`: Walks vertex pairs and fills the per-row table with values interpolated along each span, tracking the running minimum and maximum.
- `build_span_rows_from_edges`: Walks edge pairs in the record and, when a visibility flag passes, computes interpolated values for each row and writes packed span words to the output buffer.
- `fill_interpolated_row_table`: Walks vertex pairs and fills a per-row table with values interpolated along each edge, storing a final row value after each span.
- `fill_interpolated_rows_wait_blitter`: Waits for the blitter, then fills per-row tables with values interpolated along each edge pair, writing a final row value after each span.
- `program_blitter_for_edge_spans`: Waits for the blitter, sets its mask and modulo registers, then for each edge pair computes step parameters and programs the blitter for each span.
- `program_blitter_edge_rows`: Waits for the blitter and loops over edge pairs, programming the blitter modulo registers and writing a span start word for each row of each edge.
- `start_blitter_row_operations`: Waits for the blitter, then for each row of an edge pair writes the source and data words and starts a blit sized to that row, looping until the edge list ends.
- `draw_masked_span_rows`: Draws horizontal spans into the bitplane buffer for each edge row, applying start and end masks from lookup tables and filling whole words between them.
- `draw_masked_span_rows_alt`: Draws masked horizontal spans like its neighbour, taking its row source from a different record field.
- `draw_masked_span_rows_blitter`: Waits for the blitter, then draws masked horizontal spans for each edge row into the bitplane buffer, using start and end masks and filling the words between them.
- `draw_masked_span_rows_wide`: Draws masked horizontal spans for each edge row into the bitplane buffer, choosing between two fill paths by a flag and using start and end masks from lookup tables.
- `draw_masked_spans_from_tables`: Writes masked span words for each row from precomputed mask pairs into the bitplane buffer, advancing through the row pointer tables until the row count is exhausted.
- `draw_masked_spans_pointer_rows`: Walks edge rows and writes masked span words into the bitplane buffer using row pointer tables and two masks, updating the table pointers at the end of each row.
- `write_masked_span_words`: Writes masked span words into the bitplane buffer for each row of an edge list, combining start and end masks with the row pointer table.
- `draw_masked_span_rows_flagged`: Draws masked horizontal spans for each edge row into the bitplane buffer, using one fill path when a flag byte is set and another otherwise, with masks from lookup tables.
- `draw_masked_span_rows_paired`: Draws masked spans per edge row into the bitplane buffer, with a flag choosing between paired-row fills and a single-row fill.
- `draw_masked_span_rows_stepped`: Draws masked span rows from the edge list using a width-dependent fill of single, paired or run words, with mask pairs from lookup tables.
- `fill_masked_rows_by_width`: Fills masked span rows into the bitplane buffer, choosing a single-word or run fill from the span width, and advances through the edge list until it ends.
- `fill_masked_span_rows_runs`: Fills masked span rows with a run of identical words between the start and end masks, choosing single or multi-word paths from the span width.
- `fill_masked_span_rows_blocks`: Fills masked span rows into the bitplane buffer from the edge list, using start and end masks and filling the words between them, and advances the row table pointers until the list ends.
- `fill_masked_span_rows_single`: Fills a masked span row with a single mask pair and a run of words, advancing through the edge list and row pointers until the list ends.
- `blit_span_rows_with_masks`: Waits for the blitter, programs its masks, offsets and size for the span rows, then blits each row using the pointer table.
- `program_blitter_span_rows`: Waits for the blitter and programs its masks and span parameters from the edge list, writing per-row blitter values and then the final control words.
- `setup_blitter_span_rows_loop`: Programs the blitter for each span row of the edge list, waiting for the blitter between rows and writing row offsets and sizes.
- `blit_row_loop_with_flags`: Writes per-row blitter source and destination words from the row table, waiting for the blitter between rows, and finishes by writing the blitter control words.
- `write_blitter_row_table`: Waits for the blitter and writes the row table words and blitter control values for each row, advancing the row offsets.
- `write_span_row_words`: Writes row words and offsets from the table into the blitter registers for each span row, looping over the row count and returning when done.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `compute_span_row_offset`: Computes a byte offset for the current span row from the record's position and a row multiplier or lookup table, returning it packed with a column value.
- `compute_span_start_offset`: Computes the start offset of a span from its column and the record's row stride, returning it packed with a second value.
- `check_table_byte_flag`: Reads a byte from a library table at an index taken from a data register, then returns on either branch without changing state.
- `stub_immediate_return`: Returns straight away with no operations; the decompiled body contains nothing beyond the return, so the routine has no observable effect.
- `stub_return_only`: A bare return in the vector library with no memory access, library calls or hardware register use.
- `unpack_vector_record_words`: Reads a packed vector record from the object state, takes a bit shift from one of its fields and rotates 16-bit words by that amount into two work buffers.
- `write_shifted_rows_with_mask`: Copies source words shifted by a bit count into two consecutive output words per row, pads the rest of each row with zero words and writes the inverted OR of the row as a mask word.
- `fill_alternating_bit_planes`: Fills two long-word arrays with alternating one-bit and zero-bit patterns derived from two object state bytes, with a parity flag choosing the phase.
- `build_bitplane_mask_pairs`: Walks a count of bits from two object state bytes and writes each bit combination as a pair of long words into two arrays.
- `select_long_pairs_by_bits`: Walks a bit word and stores one of two long-word pairs per bit into two arrays, then copies the last pair into the object state and sets a state byte to a marker value.
- `select_long_pairs_by_byte`: Walks the bits of a byte and stores one of two long-word pairs per bit into two arrays, then copies the last pair into the object state and sets a state byte to a marker value.
- `load_scaled_bitmap_header`: Finds the bitmap record for the current object, stores its plane size, row padding and flags in the object state, and scales the width and height when a scale factor applies.
- `prepare_scaled_bitmap_placement`: Reads a bitmap header from a record, stores its size and flags in the object state and scales the width and height.
- `store_scaled_placement_rect`: Fails when the scaled size is zero.
- `store_centred_placement_rect`: Stores the placement rectangle and a centred origin for the object into its state, then calls two library routines to draw it.
- `draw_placement_if_visible`: Optionally recomputes the placement origin through a library call, then unless a clip flag is set stores the placement fields and calls two library routines to draw the object.
- `compute_clipped_centred_placement`: Computes the placement of an object from its scale and centre offsets and fails when the scaled size is zero.
- `compute_ratio_scaled_placement`: Derives a size from the ratio of two dimensions, adjusts the origin for negative extents and stores the placement fields in the object state.
- `compute_scaled_placement_rect`: Derives the width from a ratio of two dimensions when a flag is set, adjusts the origin for negative extents and stores the placement fields in the object state, then calls two library routines to draw.
- `stub_return_zero`: A routine that only returns zero, with no work, no memory access and no library calls.
- `build_index_table_with_flags`: Clears four state longs and a counter, then fills a table from a record's 16-bit offsets, either copied directly or relocated against a base address depending on a flag.
- `relocate_index_table_and_dedupe`: Adds a base address to each nonzero 16-bit offset in one table, then scans a second table and removes entries that repeat an earlier value, storing the surviving count and stride in the object state.
- `collect_unique_index_entries`: Scans a table of index values and, for each value, marks later equal entries as removed while recording the surviving values into a second table.
- `compact_unique_index_list`: Scans a second index table, recording each value once and marking repeats as removed, then stores the surviving count and stride into the object state.
- `clip_vertical_placement_rect`: Tests whether a placement rectangle overlaps the clip bounds of a record and fails when it lies outside.
- `return_zero_3`: This library routine is empty and simply returns zero to the caller.
- `prepare_scaled_row_descriptors`: Computes the source step, horizontal scale and bit shift for each row of a scaled bitmap from the object state, and stores the per-row descriptors in its work table.
- `compute_scaled_bitmap_steps`: Derives scale steps from two dimensions by division, combines them with the object origin and clip offsets, and stores the start position and fraction in the object state.
- `compute_scaled_steps_with_flip`: Derives scale steps and origin offsets by division like its neighbours, but flips the vertical offset when a record flag is set before storing them in the object state.
- `walk_scaled_bitmap_rows`: Walks the rows of a bitmap through its row table, stepping through scaled source positions and storing per-row start offsets in the object state.
- `write_row_bit_offset_table`: Accumulates a scaled step across a count of columns, then writes each row's byte offset and bit position into a table of word pairs.
- `write_row_offset_mask_table`: Accumulates a scaled step like its neighbours and writes each row's byte offset together with a masked bit position into a table of word pairs.
- `setup_blitter_row_geometry`: Splits a width into a bit shift and a count of 16-bit words, stores these and related row values in the object state, waits for the blitter to be free and then calls two library routines.
- `pack_source_bits_into_words`: Reads source bytes through an offset table, extracts selected bits with a shift and packs them into 16-bit words that are written to the output row table.
- `advance_bitmap_row_pointer`: Steps through columns and rows, advancing a source pointer by the row stride and calling a library routine per row group until the column counter ends.
- `combine_bitplane_rows_into_buffer`: Walks a table of row offsets and combines bitplane data from the source into the work buffer, copying or ORing according to the object flags.
- `or_rows_into_mask_buffers`: ORs source rows into the output buffers through tables of row offsets, with a separate pass when the object flags are clear.
- `clear_mask_rows_from_buffer`: Clears bits in the output rows by ANDing them with inverted source words, walking a table of row offsets and skipping zero entries.
- `clear_or_rows_by_index`: Walks a table of row offsets, ORing source words into output rows when the entry matches the current column and clearing bits from the output otherwise.
- `or_rows_with_index_table`: Walks an index table and ORs or masks long-word rows into the output buffer, where a zero entry ends the list and a matching index selects the source row.
- `clear_mask_long_rows`: Clears bits in long-word output rows by ANDing them with inverted source long words, walking a table of row offsets.
- `clear_or_long_rows_indexed`: Walks a table of row offsets over long-word rows, ORing source longs into output rows on a match and clearing bits from the output otherwise.
- `copy_or_zero_long_rows`: Fills each output row with either a copied source row of long words or zeros, chosen by a row index table and the object's count fields.
- `copy_or_zero_word_rows`: Writes each output row as zero words or as a copied source row of words, chosen by a row index table.
- `fill_zero_then_copy_words`: Fills output rows with zero words where the index table marks them empty and copies word rows from the source otherwise, using the object's row counts.
- `zero_long_then_copy_rows`: Zeroes a run of long words, then writes each output row as zeros or a copied source row of words according to a row index table.
- `blit_shifted_rows_with_mask`: Copies a block of source rows into the output buffer at a bit offset, combining each row with its mask and handling the leading and trailing partial words.
- `blit_shifted_rows_inverse_mask`: Variant of the shifted row copier that makes a second mask pass, combining source rows into the output with an inverted mask on the leading word.
- `blit_rows_using_hardware_blitter`: Waits for the blitter to be idle, sets up row sizes and bit shifts, and dispatches to one of several row copy routines according to the object's flags.
- `clear_or_shifted_long_rows`: Clears the destination bits for each shifted source word, ORs the shifted source into long-word rows and advances along the row until its count is used up.
- `clear_or_shifted_word_rows`: Clears the destination bits for each shifted source word, ORs the shifted source into word rows and advances along the row until its count is used up.
- `clear_or_split_long_rows`: Clears and then ORs shifted source words into long-word output rows, splitting each shift across the high and low halves of a long where needed.
- `or_shifted_word_rows`: ORs a shifted source row into word output rows, carrying the bits that fall off each shift into the next word, and moves on to the next row group.
- `or_shifted_long_rows`: ORs a shifted source row into long-word output rows, carrying the bits that fall off each shift into the next word, and moves on to the next row group.
- `blit_masked_row_group`: Copies masked row groups into the output with a bit shift taken from the object state, clipping rows against the row count and advancing the source and destination pointers.
- `clear_or_right_shifted_longs`: Clears and then ORs right-shifted source words into long-word output rows, handling the carry from the high word of each row.
- `clear_or_right_shifted_words`: Clears and then ORs right-shifted source words into word output rows, handling the carry from the high word of each row.
- `clear_right_shifted_long_rows`: Clears bits of long-word output rows wherever a right-shifted source word has bits set, handling the row remainder.
- `or_right_shifted_word_rows`: ORs right-shifted source words into word output rows, advancing through each row and the row count of the object.
- `or_right_shifted_long_rows`: ORs right-shifted source words into long-word output rows, advancing through each row and the row count of the object.
- `zero_rows_then_clear_masked`: Zeroes output rows that the index table marks empty and clears bits from the remaining rows where a mask applies, using the object's row counts.
- `clear_or_word_rows_strided`: For each pair of word rows, clears destination bits matching a mask row and ORs the source row in, advancing by a stride and repeating across the row count.
- `clear_masked_word_rows`: Clears destination words wherever the mask row has bits set, walking rows with a stride and a count taken from the object's row fields.
- `clear_masked_word_column_rows`: Clears destination words in a column of rows wherever the mask has bits set, advancing by a row stride and a count taken from the object's row fields.
- `or_word_rows_with_stride`: Walks word rows and ORs source words into the destination, advancing by a row stride until the object's row count is used up.
- `or_word_rows_by_index`: Walks a row index table and ORs source words into matching word output rows, advancing by a stride across the row count.
- `zero_word_rows_by_index`: Writes zero words into output rows wherever the index table marks them empty, advancing by a stride across the object's row count.
- `copy_word_rows_by_index`: Copies source words into output word rows along an index table, advancing by separate strides for rows and columns.
- `zero_word_columns_by_stride`: Zeroes word columns of output rows across the object's row count, stepping by a row stride and handling trailing rows from the index table.
- `zero_word_rows_by_stride`: Clears word rows across the object's row count, stepping by a row stride.
- `shift_byte_rows_with_carry`: Shifts rows of byte data one bit at a time, carrying the bits that fall off each row into the next and applying the mask sign bit.
- `clear_or_byte_rows_strided`: For each pair of byte rows, clears destination bits matching a mask row and ORs the source row in, advancing by a stride across the row count.
- `clear_masked_byte_rows`: Clears destination bytes wherever the mask row has bits set, walking rows with a stride taken from the object's row fields.
- `clear_masked_byte_column_rows`: Clears destination bytes in a column of rows wherever the mask has bits set, advancing by a row stride.
- `or_byte_rows_with_stride`: ORs source bytes into destination byte rows, advancing by a row stride until the object's row count is used up.
- `or_byte_column_rows`: ORs source bytes into a column of destination byte rows, advancing by a row stride.
- `setup_blitter_shifted_copy`: Waits for the blitter, sets its first and last word masks, shift and modulo registers from the row geometry, then starts a blit of the row group through a library routine.
- `blit_rows_with_pointer_setup`: Waits for the blitter and loads its source, mask and destination pointers for each row, skipping rows the index table marks empty.
- `feed_blitter_data_words`: Writes source words to the blitter's data register row by row, waiting for the blitter between groups and advancing through the index table.
- `start_blitter_rows_fill`: Sets the blitter's control register and size for each row group, feeds fill words and starts the blit, then repeats for the remaining rows.
- `dispatch_blitter_row_request`: Reads a pending request record for the object, clears its two flag bytes and, when a request is due, waits for the blitter and loads row, mask and source values before calling a library routine.
- `blit_constant_row_fill`: Waits for the blitter, sets its control and size registers for a fixed word count, and writes a repeating row value through its B and D channels for each row.
- `blit_bitplane_rows_with_shift`: Waits for the blitter, computes a shift and word count from the object's row fields, then programs the blitter registers and feeds row words in a loop.
- `start_blitter_fixed_width_fill`: Waits for the blitter, sets its control and size registers for a fixed width, and starts a row fill through its D channel.
- `dispatch_record_type_handler`: Reads a type byte from a record and calls the matching handler from a jump table, then processes the handler's result.
- `load_object_bounds_from_record`: Copies four bounds values from a record into the object state and clears the object's offset fields, or reads the same values from a header when no bounds record is present.
- `load_bounds_and_header_fields`: Copies four bounds values from a bounds record and reads a header's size and flags into the object state, adjusting the flags when the height is negative.
- `call_three_steps_return_zero`: Calls three library routines in turn and returns zero.
- `call_steps_if_count_positive`: Calls two library routines, then a third only when a count in the object state is positive, and returns zero.
- `call_pair_by_flag_result`: Tests a flag byte in the object state and calls a sequence of library routines, returning one when the second group ran and zero otherwise.
- `call_conditionally_return_flag`: Calls one library routine and, when its result is nonzero, calls a second one and returns one.
- `init_object_flags_from_record`: Clears three state words, copies a count from the record and sets the high flag bits on the object when the record requests them.
- `run_three_steps_return_zero`: Calls three different library routines in sequence and then returns zero.
- `run_routines_size_check`: Calls two subroutines, then calls a third when a size field derived from the record is non-negative.
- `copy_long_table_into_state`: Reads a count from a record and copies that many long words from it into the object state table.
- `copy_long_pairs_into_state`: Reads a count from a record and copies that many pairs of long words from it into the object state table.
- `find_min_max_word_values`: Scans a table of 16-bit values, keeping the lowest and highest seen so far in two register pairs for the caller.
- `find_min_max_bounds`: Scans a table of 16-bit values using a count from the caller and records the smallest and largest in the object state.
- `parse_vector_record_header`: Reads a record's type and data pointer into the object state, then walks its entries according to the type and stores each entry's flag word.
- `init_record_pointer_state`: Marks the object's record pointer as unset, copies a flag word from the record into the state and returns a masked form of those flags.
- `return_immediately`: Returns immediately without performing any operations.

### Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spin_forever_after_two_calls`: Makes two calls through a pointer in the segment, one of them into an operating system library, and then loops forever with no exit.
- `stub_return_minus_one`: Returns the all-ones value and does nothing else, so callers receive a fixed failure code.
- `wait_for_input_flag_clear`: Waits for a shared input flag to be set and then cleared, calling six routines around the wait.
- `open_graphics_and_save_state`: Forbids multitasking, opens the graphics library by name, saves its base and the system's hardware state, and returns failure when the library cannot be opened.
- `install_copper_list_and_interrupts`: Disables interrupts and DMA, fills a table of addresses into a copper list, points the copper at that list and re-enables the chosen interrupts and DMA channels.
- `fill_arithmetic_sequence_table`: Fills a table of 32-bit entries with an arithmetic sequence, storing each value's high and low halves as separate words.
- `restore_system_state_and_close`: Restores the saved copper pointer and display registers, clears the audio volumes and DMA settings, closes the opened library and re-enables multitasking.
- `clear_input_status_and_finish`: Clears a status byte when the fire-button flag is not set, calls five routines and then writes an interrupt request.
- `tick_countdown_then_update`: Decrements a countdown held in the segment and calls one routine when it reaches zero.
- `interpret_text_command_stream`: Reads command bytes from a text stream one at a time, using each byte's sign to choose between storing a value and calling a routine, until the end marker is reached.
- `store_text_layout_fields`: Stores the results of two routine calls and a fixed table value into the object state, setting up the layout fields for a text line.
- `store_routine_result_field`: Calls one routine and stores its result in a field of the object state.
- `store_pointer_and_reset_field`: Stores a pointer in the object state, calls a routine and clears a counter field.
- `measure_text_width`: Sums the width of each character in a string from a width table, starting from a maximum width, and returns half of the remainder.
- `compute_text_offset_from_lines`: Counts line breaks in a text string and returns a vertical offset that moves up by a fixed step for each line.
- `clear_text_buffer_row`: Clears one row of a text buffer by writing zero longs across it, with the row chosen by a counter in the segment.
- `advance_scrolling_text_row`: Adjusts a scrolling position from a counter in the segment, stepping through a table of characters, and returns early when the counter is out of range.
- `sample_three_values_and_compare`: Toggles a flag in the object state and, when it is set, calls a routine four times, stores each result and notes whether the results differ.
- `combine_nibble_values`: Combines two 16-bit values across four nibble positions, choosing between them with clamping at each step, and returns the combined result.
- `read_flagged_table_byte`: Checks a flag in the object state and either returns a table byte or updates a counter and calls routines to read the next value.
- `pack_two_byte_record`: Packs two input bytes and computed flag bits into a pair of words at the given address, shifting one value right and the other left.
- `init_default_object_fields`: Writes default values into two object records, including two pointers and fixed sizes, and stores a table address in the object state.
- `read_stream_on_timer_tick`: Counts down a timer in the object state and, when it expires, toggles a flag and reads the next byte from a stream.
- `parse_text_control_byte`: Reads the next byte of a text stream.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_use_close_library`: Clears a global, opens a library through the system table, makes one library call on it and closes the library again.
- `run_startup_screen_loop`: Waits for the display's vertical blank, closes and reopens the Workbench screen through wrappers, and loops a fixed number of times making graphics and intuition calls before returning zero.
- `check_free_memory_level`: Calls the memory-available routine and sets a level code by comparing the result with two thresholds, storing the code for the caller.
- `copy_name_string_to_buffer`: Copies a null-terminated string selected by an index from the segment's table into a buffer, appends a space and a terminator, and marks the entry when the index is above one.
- `open_required_libraries`: Opens the math, tridee and vector libraries in turn and stores each library base in the segment.
- `call_three_routines_in_order`: Calls three routines in order and returns.
- `close_library_if_open`: Closes the library through the system table when the given base is nonzero.
- `call_open_library_2`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `run_loader_callback_setup`: Installs a loader callback and buffer address in the segment state, runs a loader step and then performs library calls that depend on each result.
- `reset_loader_state_then_run`: Clears three loader state fields, runs a loader step and performs library calls that depend on the results, as its neighbouring loader entries do.
- `reset_intro_loader_state`: Clears loader state fields and runs the intro load steps, calling library routines that depend on each result.
- `clear_loader_fields_and_load`: Clears three loader state fields and runs a loader step, calling library routines that depend on each result, the same sequence as the entries around it.
- `return_segment_or_failure`: Returns the stored value in the low word with an all-ones upper word, so callers receive a failure marker alongside it.
- `dispatch_by_opcode_marker`: Compares a marker word in the caller's record with a fixed opcode pattern and calls one of two stored routines accordingly.
- `expand_tilde_escape_text`: Copies a text string, expanding tilde escape codes into fixed text, and stops at an escape for the end of a line or at the end of the string.
- `wait_for_disk_then_scan`: Repeats a DOS call until its result is valid, then scans the name string to find its length.
- `open_dos_graphics_intuition`: Opens the DOS, graphics and intuition libraries in turn and stores each library base in the segment, with one further routine call in between.
- `close_stored_library_bases`: Closes each stored library base that is set, calling one routine first when the first base is present, and returns the result of the last close.
- `call_close_work_bench`: Wrapper that calls the intuition library function CloseWorkBench and returns its result to the caller.
- `call_open_work_bench`: Wrapper that calls the intuition library function OpenWorkBench and returns its result to the caller.
- `mark_bitmap_row_and_clip`: Sets the high bits in a row byte of a bitmap, then calls a routine and, when it succeeds, recomputes clamped clip values from the stored rectangle.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `recompute_clip_rectangle`: When a routine reports success, recalculates the clipped rectangle bounds from the stored values, halves the width offset and updates the stored edges.
- `close_window_and_screen`: Clears the pointer image from the open window, closes the window and screen when they are open, and returns the status of the last close.
- `open_screen_window_with_palette`: Derives screen dimensions from stored values with clamping, loads a 4-bit colour palette, opens a screen and a window on it, and brings the screen to the front.
- `set_pointer_image`: Calls the pointer-setting routine of the intuition library with the stored pointer data and returns its arguments unchanged.
- `patch_library_vector_and_checksum`: Saves a library vector and a stored value, rewrites the library's table entry, then forbids multitasking while it clears the cache and recomputes the library checksum before permitting it again.
- `restore_library_vector_and_checksum`: Forbids multitasking, restores the saved library table entries and flag bits, clears the cache and recomputes the library checksum, then permits multitasking again.
- `return_constant_value`: Returns a constant value to the caller.
- `nop_placeholder`: This routine is empty and performs no operations.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `wait_for_frame_then_call`: When the stored display pointer matches the saved value, calls a stored hook routine repeatedly, waiting for the frame between calls, until a flag ends the loop.
- `signal_completion_callback`: Runs an optional callback stored in the segment and then sends a signal through the system's exec library.
- `wait_frames_then_callback`: Sends a signal, waits for frames while a counter is nonzero, calling a routine on each pass, and then runs an optional callback.
- `start_disk_watch_task`: Reads a disk-status value, allocates a signal and memory for a task, and adds the disk watch task to the system.
- `stop_task_and_free_resources`: Removes the task, frees its memory and signal, and releases the related resources when they were allocated.
- `wait_for_signal_bits`: Waits on the system until one of two signal bits is set and returns the value associated with whichever fired, updating a byte pointer as it goes.
- `register_handler_entry`: Marks an entry in a record with flag bits, stores a handler pointer and a value in the segment, then calls a library routine and records whether it succeeded.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `remove_interrupt_server_if_added`: Looks up the current task and, when an interrupt server was registered, removes it through the library wrapper and clears the flags it set.
- `write_text_in_eighty_chunks`: When the setup call succeeds, writes a fixed run of 179 characters through a stored output routine in chunks of at most eighty.
- `skip_leading_spaces`: Advances through a text buffer until a non-space character is found or the buffer length runs out.
- `scan_text_line_fields`: Walks a text buffer one line at a time, stopping at a newline or a matching delimiter, and calls a routine for each field found.
- `store_three_resource_handles`: Calls routines to obtain three values and stores them in the segment state, marking one flag entry and clearing another when the values are valid.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `open_library_by_name`: Passes its arguments to the operating system's OpenLibrary routine through the system table and returns the library base it obtains.
- `copy_size_words_from_header`: Copies two 16-bit size values from a header into two state fields of the segment.
- `close_after_three_steps`: Calls three routines, then closes the library handle through the system table when it is set, and returns the result of that close.
- `close_library_if_given`: Closes the library passed in when it is nonzero.
- `call_add_int_server`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spin_forever_halt`: Loops forever with no exit and no other work.
- `advance_counter_clear_flag`: Increments a 16-bit counter in the object state and clears one flag bit in the byte next to it.
- `advance_counter_clear_flag_2`: Increments a 16-bit counter in the object state and clears one flag bit in the byte next to it, the same as its neighbouring entry.
- `remove_interrupt_server_if_registered`: Looks up the current task and, when an interrupt server was registered, removes it through the library wrapper and clears the flags it set.
- `write_chunked_block_to_handle`: Obtains a handle through a library call and, when the handle is valid, writes a total of 179 bytes through a stored routine in chunks of at most 80 bytes.
- `skip_leading_spaces_2`: Advances a text pointer past space characters, stopping at the end of the counted input length.
- `split_command_line_tokens`: Splits a counted command line into tokens, with spaces and line-feeds as separators and double quotes grouping text.
- `store_optional_resource_pointers`: Calls a subroutine to get a pointer and stores it in a state field only when it is nonzero.
- `call_indirect_via_table`: Calls a routine through a pointer stored at an offset within a global variable.
- `call_indirect_function`: Fetches a function pointer from a global table and executes it.
- `copy_two_words_from_subrecord`: Copies two 16-bit fields from a sub-record pointed to by the state block into fixed slots in the state block.
- `call_conditional_function`: Calls a routine through a pointer in a global variable if a specific field in a state block is non-zero.
- `call_library_function_conditionally`: Calls a routine through a pointer in a global variable if a provided value is non-zero.
- `add_interrupt_server_wrapper`: Passes its arguments to the operating system's AddIntServer routine through a library base in the segment.
- `remove_interrupt_server_wrapper`: Passes its arguments to the operating system's RemIntServer routine through a library base in the segment.

### logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf

- `tiny_ori_2`: Small routine of 1 instruction that merges 29 into d0.
- `follow_chain_then_call_entry`: Follows a chain of pointers until it reaches a zero link, then calls the routine stored at the end of the chain.

### logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf

- `follow_chain_then_call_entry_2`: Enters the same pointer-chain dispatch as its neighbour, following links until a zero link and then calling the routine stored there.
- `follow_chain_then_call_entry_3`: Follows a chain of pointers until it reaches a zero link, then calls the routine stored at the end of the chain.

### logged disk load from Captive II - Liberation (1992)(Mindscape)[cr DLM](Disk 1 of 5)[a].adf

- `traverse_pointer_table`: Traverses a table of pointers by multiplying values by four until a null pointer is found, then calls the routine at the resulting address.
- `traverse_pointer_table_2`: Follows a chain of pointers stored in memory by dereferencing values multiplied by four until a null pointer is encountered, then calls the function at the next offset.
