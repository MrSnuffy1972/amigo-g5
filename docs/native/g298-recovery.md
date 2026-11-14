# Game 298 (bubble ghost (1988)(accolade)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00298). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (923856 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 156 of 156 named (0 author symbols), 156 of 156 described (32 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2939 of 2939 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 32000 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5790 of 5790 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 411740 |
| code | 21322 |
| data | 91723 |
| padding | 6367 |
| unused | 392704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bubble Ghost (1988)(Accolade).adf:BG | 19 | 230 | 5 |
| Bubble Ghost (1988)(Accolade).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3275 | 48 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2275 | 102 |
| Bubble Ghost (1988)(Accolade).adf:ghost.pre (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 1683 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1027 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bubble ghost (1988)(accolade)/Bubble Ghost (1988)(Accolade).adf:BG: Imploder (xfd) stream found by ancient scan (18628 -> 49500 bytes) but only 3 of 15 sampled windows are in reference RAM: not used
- bubble ghost (1988)(accolade)/Bubble Ghost (1988)(Accolade).adf:ghost.pre: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubble ghost (1988)(accolade)/Bubble Ghost (1988)(Accolade).adf:BG: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 32000 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Bubble Ghost (1988)(Accolade).adf:BG

- `free_memory_loop_clear`: Calls an exec library routine that frees memory, repeated three times in a loop, after an earlier library call.
- `decrement_until_minus_one`: Decrements a 16-bit register variable in a loop until it reaches negative one, then returns.
- `decode_bitstream_into_longword_table`: Runs fifteen passes over a bit-stream decoder.
- `decode_backward_bitstream_copy`: Reads length and distance codes from a bit stream that is consumed from the end of the input, and copies back-references and literal bytes into the output.
- `decode_bitstream_quad_copy`: Decodes a backward bit-stream block, writing four bytes per step when copying and tracking the remaining block size.

### Bubble Ghost (1988)(Accolade).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `search_table_and_store_entry`: Walks a table of routine pointers until it finds a zero entry, returning a failure value if the search runs past the end.
- `setup_tables_and_dma_control`: A large setup routine that calls helpers with fixed coordinate and size arguments and fills two tables of entries.
- `build_alternating_row_tables`: Runs two passes that fill table entries for a 320 by 200 area, alternating between two marker values on each step.
- `run_four_entry_helper_calls`: Calls a helper to obtain a value and, if it is nonzero, calls a helper four times with a fixed size for each table entry.
- `call_helpers_with_fixed_arguments`: Calls helpers with fixed numeric arguments in sequence and returns the result of the final call.
- `call_helpers_with_parameter_block`: Calls a helper with a block of eleven fixed numeric arguments, then makes a second call with another block and returns its result.
- `write_constant_table_of_values`: Writes a long list of fixed numeric values into global variables, in groups separated by a marker value, and then returns 121.
- `call_helpers_with_index_offsets`: Makes helper calls with fixed numeric arguments and with a computed index offset from a base table, then returns 318 after a final helper call.
- `loop_helper_calls_with_index`: Loops over a count argument, computing an index from a stored value and the remaining count, and calls two helpers with that index on each pass.
- `format_values_into_text_buffers`: Converts three numeric values into fixed-width text buffers of six, six and two characters, plus a one-character buffer.
- `format_integer_as_decimal_text`: Converts an integer into decimal characters by repeated division, pads the result with zeros to a fixed width and terminates it.
- `call_helper_pair_with_arguments`: Makes four helper calls, two of them with the same pair of parameter values, and returns the result of the final call.
- `walk_text_string_with_helpers`: Walks a zero-terminated text string from a fixed address.
- `run_counter_loop_until_limit`: Calls helpers repeatedly while a global flag stays zero, incrementing a counter and stopping once it passes fourteen.
- `sort_paired_longword_table`: Walks a table of four entries across five passes, swapping each pair of longwords with its partner whenever the first is smaller.
- `draw_five_row_value_table`: Clears a field and draws five rows.
- `call_helpers_on_nonzero_result`: Calls a helper with a fixed argument pair and, if the result is nonzero, makes two more helper calls with a fixed size before returning the last result.
- `call_helpers_on_nonzero_result_2`: Calls a helper with a fixed argument pair and, if the result is nonzero, makes two more helper calls with a fixed size and no return value.
- `call_helpers_on_nonzero_result_3`: Calls a helper with a fixed argument pair and, if the result is nonzero, makes two more helper calls with a fixed size before returning the last result.
- `update_clamped_state_from_byte`: Clamps two position values to fixed ranges and branches on a byte read from a pointer, adjusting a state table and a cursor index.
- `check_distance_and_update_slots`: Switches on a small value from one to eight and sets fixed state fields in range-checked cases.
- `scan_seven_entries_and_flag`: Advances a cycle counter modulo six when it is nonzero, then tests seven table-derived positions.
- `call_seven_helpers_in_sequence`: Makes seven helper calls in sequence and returns the result of the last one.
- `delay_fixed_count_loop`: Counts up to 4000 in an empty loop to waste time, then returns the input value.
- `call_two_fixed_blocks`: Makes two helper calls, each with a computed base index, a fixed 32 by 32 size and fixed argument values, then returns the result of the second call.
- `call_helpers_clear_four_slots`: Makes two pairs of helper calls with fixed 32 by 32 sized arguments, then writes a shared value into four slots indexed by a computed index and returns that index.
- `call_two_table_block_draws`: Makes two helper calls with fixed 32 by 32 sizes and fixed argument values, using two values read from a table indexed by a global index.
- `step_entry_timers_and_draw`: Loops over ten entries.
- `draw_five_by_ten_grid`: Loops over a five by ten grid.
- `draw_grid_cells_by_row`: Loops over grid cells in rows of ten and columns of five, calling a helper with a coordinate and fixed 32 by 32 sizes for each cell.
- `scale_two_results_by_32`: Calls helpers and stores two of their results, each multiplied by 32, into globals.
- `draw_stepped_strip_rows`: Calls helpers with fixed sizes, then runs a loop from 162 down to 2 in steps of two, drawing 320 by 2 strips at successive positions on each pass.
- `convert_flag_store_offset`: Converts a global value into a zero-or-one flag, then calls a helper and stores its result plus a fixed offset into a global.
- `set_field_and_test_word`: Sets a field to 22 and calls two helpers.
- `test_bit_in_table`: Finds a byte in a table at a computed offset and returns one bit of it, chosen by a shift count taken from a register.
- `test_bit_in_table_2`: Returns one bit from a byte in a table at a computed offset, selected by a shift count taken from a register.
- `allocate_and_init_record`: Allocates a record through a helper and stores its pointer in an output parameter.
- `init_record_fields_and_finish`: Calls a helper to fill a record from its arguments, stores two argument values into the record and calls a finishing routine.
- `set_table_entry_and_call`: Looks up a pointer in a table by index and, if it is set, writes a fixed value and two parameter values into its fields before calling a helper.
- `release_record_fields`: Calls a helper on a record, then releases up to two nested pointer fields and the record itself through helper calls.
- `read_fixed_records_and_sum`: Reads a four-byte value and a twenty-byte block through a sequence of helper calls, then returns the sum of two stored values.
- `read_record_block_return_word`: Reads a sequence of fixed-size blocks through helper calls, copies a block into the caller's buffer and returns a 16-bit value read from the record.
- `read_bytes_until_flag_set`: Repeatedly reads a counted run of single bytes through helper calls until a status bit is set or a read fails.
- `obtain_handle_from_table_entry`: Looks up an entry in a table by index and calls a helper to obtain a handle, storing it.
- `open_handle_if_slot_empty`: When a table entry is set and its handle slot is empty, calls a helper with the entry's values and the parameters, and stores the returned handle in that slot.
- `release_handle_slot`: If a handle slot is set, calls a helper to release the handle and clears the slot.
- `call_helper_for_indices`: Calls one helper seven times, with the index values zero through six, and returns seven.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_task_and_clean_up`: Looks up the current task through the exec library, copies the argument bytes into a stack buffer ending with a space, and clears a block of 43 longwords of globals.
- `release_resources_and_return`: Runs an optional cleanup handler, calls library routines including a forbid step and a reply-message step, and conditionally releases the resources recorded earlier.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `open_dos_library_or_cleanup`: Opens the dos library through an exec library call.
- `find_or_allocate_list_node`: Walks a linked list from a global head, stopping at a node whose field is zero.
- `parse_file_mode_string`: Parses a string to determine file access modes and flags, then opens a handle and populates a structure with the resulting handle and configuration.
- `read_stream_bytes_into_buffer`: Fills a buffer with rows of bytes read from a stream structure, refilling the stream through a helper when its count runs out.
- `open_slot_with_mode_flags`: Finds a free slot in a table of handles, derives open flags from the input value and opens a handle through helpers.
- `call_helper_with_masked_flags`: Calls a helper with a flag value built from bit fifteen of its second argument plus a fixed value, and with its third argument stripped of bit fifteen.
- `close_stream_and_flush`: Flushes pending output when a flag is set, releases a buffer when one is present, then closes the handle through a helper.
- `flush_pending_stream_writes`: Walks a linked list of streams, and for each stream that is open for writing with pending bytes it writes the pending bytes through a helper.
- `call_helper_with_field_eight`: Calls one helper with the value of a field at a fixed offset from the base-register structure, then returns.
- `read_next_stream_byte`: Returns the next byte from a buffered stream, refilling the buffer through a helper when it is empty.
- `call_helper_or_set_error`: Optionally calls a helper when a flag is set, clears the flag, then calls a helper and returns its result.
- `invoke_library_function_pointer`: Calls one library function pointer stored at a fixed negative offset from a saved base value, then returns.
- `invoke_library_function_pointer_2`: Calls one library function pointer at a fixed negative offset from a saved base value, then returns.
- `open_file_or_set_error`: Calls a helper with a sentinel value and releases the result if it is nonzero.
- `call_indirect_function`: Executes a function through a pointer stored at a specific offset from a global base address.
- `execute_indirect_function`: Loads a function pointer from a global table and executes it.
- `process_masked_input`: Checks a global value, then performs conditional logic involving multiple indirect function calls and bitwise operations to return a value or an error.
- `execute_indirect_function_2`: Loads a function pointer from a location within a global table and executes it.
- `close_record_and_report_status`: Looks up a record through a helper and returns minus one if it is missing.
- `compare_c_strings`: Compares two null-terminated strings byte by byte.
- `release_all_entries_then_call`: Walks a table of entries and, for each one that is set and not marked, calls a helper with its stored value.
- `write_stream_byte_newline_mode`: Writes one byte to a buffered stream.
- `call_helper_after_lookup_check`: Looks up an entry through a helper and returns minus one if it is missing.
- `call_helper_then_return_zero`: Optionally calls a helper when a flag is set, calls a helper with its argument, then returns zero.
- `execute_indirect_function_3`: Loads a function pointer from a location within a global table and executes it.
- `calculate_string_length`: Walks a null-terminated string to find its end and returns the number of characters before the terminator.
- `fill_bytes_with_value`: Writes a byte value into a run of memory for a given count and returns the start address.
- `allocate_buffer_if_missing`: Returns zero when a buffer is already present and not marked for reset.
- `seek_then_call_helper`: Looks up a stream through a helper and returns minus one if it is missing.
- `call_helper_record_failure`: Calls a helper with three arguments after clearing an error flag.
- `call_library_vector_42`: Loads a library base from a global variable and calls the library function at offset minus 42 from that base, discarding any result.
- `forward_single_argument`: Passes its one 32-bit argument unchanged to another routine and returns nothing of its own.
- `allocate_block_from_free_list`: Walks a linked list of free blocks and carves out a block of the requested size, rounded up to whole longwords, splitting off the leftover space when it is large enough.
- `call_with_error_status`: Clears a status flag, optionally runs a routine first when a pending flag is set, then calls another routine with its three arguments.
- `call_library_vector_48`: Loads a library base from a global variable and calls the library function at offset minus 48 from that base, discarding any result.
- `call_then_check_status`: Calls a routine with the first argument and returns minus one if that yields zero.
- `allocate_linked_memory_block`: Requests a block 12 bytes larger than the caller's size, stores that total size in the block header, and links the block onto a global list.
- `map_zero_to_minus_one`: Forwards its argument to another routine and passes a nonzero result straight back.
- `return_constant_minus_one`: Returns the value minus one with no other work, serving as a fixed return value for its caller.
- `reset_and_allocate`: Clears several global variables, performs arithmetic operations on values from global variables, allocates memory using a calculated size, and calls another routine with the allocated block and size when allocation succeeds.
- `save_and_call`: Saves two registers to the stack, calls another routine while passing the value from a register as the first and second arguments, and then restores the saved registers before returning.
- `process_and_clear_node_list`: Walks a global linked list node by node, passing each node and its third longword to another routine.
- `append_node_to_list`: Appends a node to the tail of a doubly linked list whose header holds the head and tail pointers, setting the node's forward link to empty and its back link to the old tail.
- `call_library_vector_210`: Loads a library base from a global variable and calls the library function at offset minus 210 from that base, discarding any result.
- `call_library_vector_198`: Loads a library base from a global variable and calls the library function at offset minus 198 from that base, discarding any result.
- `combine_result_by_mode`: Calls another routine with a value picked from a global table by the mode argument, and records error code 22 when that call fails.
- `call_library_vector_66`: Loads a library base from a global variable and calls the library function at offset minus 66 from that base, discarding any result.
- `find_table_entry_by_key`: Searches an array of eight-byte entries, up to a global count, for one whose first word is nonzero and whose key field matches the argument, and returns its address.
- `multiply_32bit_words`: Multiplies two 32-bit values taken from the data registers using 16-bit partial products, adding the two cross terms to the low-word product.
- `divide_signed_32bit_values`: Divides two signed 32-bit values, routing negative operands through other routines and negating the result when needed.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend by a 32-bit divisor after shifting both to normalize the divisor, returning the quotient in the high longword and the remainder in the low longword of a 64-bit result.
- `call_with_repeated_second_argument`: Passes its two arguments to another routine, repeating the second argument as the third, and returns nothing.
- `free_block_into_list`: Returns a block of the given size to an address-sorted free list, merging it with an adjacent free block before or after it when they touch.
- `read_masked_input_bits`: Calls another routine with a mask of 12288 and returns the masked bits when any are set.
- `forward_call_without_arguments`: Calls another routine with no arguments and returns nothing.
- `call_library_vector_306`: Loads a library base from a global variable and calls the library function at offset minus 306 from that base, discarding any result.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `setup_pointer_chain`: Initialises a small list header in the block it is given: the first longword points 4 bytes past the block start, the second longword is cleared, and the third longword points back to the block.
- `allocate_and_register_record`: Obtains an identifier, then allocates a record and fills its fields from the two arguments and the identifier.
- `teardown_registered_record`: Tears down a record created earlier: if its pointer field is set it first calls a routine on the record, then marks its status byte as 255 and sets a longword to minus one.
- `call_with_fixed_code_48`: Passes its argument to another routine together with the fixed value 48 and returns nothing.
- `forward_parameter`: Pushes a register value onto the stack, retrieves a value from the stack at a specific offset, pushes that value onto the stack again, calls another routine with the second value, and then restores the register from the stack before returning.
- `allocate_typed_record`: When the first argument is nonzero, allocates a record of the size given by the second argument, then stores a type byte of 5, the second argument as a short, and the first argument in it.
- `initialize_structure`: Initializes fields of a structure pointed to by a parameter, sets specific byte and longword values, and calls a library function with data from the structure.
- `call_library_vector_222_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Loads a library base from a global variable and calls the library function at offset minus 294 from that base, discarding any result.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_30`: Loads a library base from a global variable and calls the library function at offset minus 30 from that base, discarding the result.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Loads a library base from a global variable and calls the library function at offset minus 198 from that base, discarding the result.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Loads a library base from a global variable and calls the library function at offset minus 210 from that base, discarding the result.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_222`: Loads a library base from a global variable and calls the library function at offset minus 222 from that base, discarding the result.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_360`: Loads a library base from a global variable and calls the library function at offset minus 360 from that base, discarding the result.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
