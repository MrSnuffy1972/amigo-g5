# Game 2258 (worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02258). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901840 top-level bytes in 61 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 255 of 255 named (0 author symbols), 255 of 255 described (35 by rule, 70 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 754 of 756 (99.74%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3519 of 3520 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 503259 |
| code | 10736 |
| data | 303957 |
| padding | 12720 |
| unused | 71168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:Run | 2 | 117 | 2 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:INSTALL | 3 | 27 | 2 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:OCEAN | 4 | 205 | 7 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:RunBack | 3 | 163 | 2 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:TEAM17 | 4 | 27 | 2 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:FLOPPYWORM | 3 | 234 | 23 |
| mathtrans.library | 18 | 830 | 97 |
| diskfont.library | 15 | 1040 | 76 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:HOOKTRO | 2 | 235 | 8 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:WORMS | 2 | 267 | 8 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:HDWORM | 3 | 217 | 23 |
| Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 71 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 67 | 1 |

## Cases handled

- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott][a]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:TWSTUFF/TW6: CrM2: Crunch-Mania LZH-mode-packed, unpacked with ancient (41486 -> 61440 bytes)
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 103 64 KB pages each with >= 29491 distinct PCs executed, 1687552 of its 1700032 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 103 64 KB pages each with >= 29491 distinct PCs executed, 1687552 of its 1700032 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EAE) A7=an address (A7)=an address, slid to exec.library -an address): 586 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 58 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 54 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott][a]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:INSTALL: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott][a]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:OCEAN: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott][a]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:TEAM17: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)(disk 1 of 3)[cr ott][a]/Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:WORMS: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If a buffer pointer is supplied, copies sixteen 32-bit values from a fixed table in the data area into it.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:INSTALL

- `initialize_program_entry`: Runs a series of setup calls, including a call through a stored function pointer and a second call through a global table pointer.
- `decrement_to_minus_one_2`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:OCEAN

- `execute_program_entry`: Performs a sequence of calls including a decrementing loop and a memory free operation.
- `decrement_to_minus_one_3`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `decode_compressed_bitstream_blocks`: Decodes compressed data in blocks by reading bits, copying literal bytes or back-referenced runs into an output buffer, and then updating a table of word counts.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values`: Iteratively calculates and stores new values into a table using a pointer.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:RunBack

- `run_command_in_background`: Parses the command line for an optional stack size switch and the command to run, then opens the DOS library and starts the command as a background task with redirected input and output.
- `measure_string_and_call_library`: Scans a null-terminated string to find its length, then calls two library functions in sequence with it.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:TEAM17

- `execute_program_entry_2`: Executes a series of subroutine calls and clears a value at a memory location.
- `decrement_to_minus_one`: Decrements a 16-bit register value by one repeatedly until it reaches minus one, then returns.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:FLOPPYWORM

- `run_startup_library_calls`: Writes zero through a pointer and calls a library function three times, storing each returned word into a global location.
- `empty_stub_routine_2`: The routine consists of three 68000 instructions that form a no-op sequence and then return immediately, so it performs no observable operation.
- `store_four_results_to_globals`: Calls another routine four times and saves each returned value into a different slot of a data block addressed from a base register.
- `call_function_pointer`: Calls a function pointer stored at a fixed offset from the data base.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `poll_function_pointer`: Repeatedly calls a function pointer stored in the data base until it returns nonzero.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_single_routine_and_return`: Forwards control to one other routine and returns, with no processing of its own.
- `copy_byte_buffer`: Copies a block of bytes from a source to a destination, moving eight bytes at a time and then the remaining bytes one at a time, and writes a zero terminator after the last byte.
- `clear_globals_and_call`: Clears one global longword, calls another routine, clears a second global longword, and returns the address of this routine.
- `repeat_call_until_flag_set`: Repeatedly calls one routine until the zero flag condition left by that call is set.
- `allocate_and_link_list_node`: Allocates a memory block through a stored function pointer and, if allocation succeeds, links it at the head of a list and returns the address past its link fields.
- `remove_list_node_or_allocate`: Searches a list for a given node and, if found, unlinks it and passes it to a release routine.
- `find_list_tail`: Follows the chain of linked pointers until it reaches a null link, then stores that last node address in a global variable.
- `save_restore_global_value`: Saves the current value of a global longword, calls two other routines, and then restores the saved value.
- `save_restore_global_d0`: The routine saves the value of a global variable to a local register, calls two other routines, and then restores the global variable to its original value.
- `allocate_and_link_node_2`: Uses a memory allocation routine to create a new block and inserts it into a list structure.
- `remove_list_node_or_allocate_2`: Searches a list for a given node and, if found, unlinks it and passes it to a release routine.
- `find_and_unlink_list_node`: Walks a linked list looking for a node whose next link matches a given node.
- `allocate_and_push_list_node`: Allocates a memory block through a stored function pointer and, if it succeeds, pushes it onto the head of a list.
- `free_all_list_nodes`: Detaches the list head and releases it, then releases each following node in the chain through a release routine, using the exec FreeMem call.
- `tiny_ori`: Small routine of 1 instruction that merges 5 into d0.

### mathtrans.library

- `stub_unimplemented_library_entry`: Only an unimplemented-instruction marker is decoded here, so the routine body is not recovered.
- `store_address_to_global`: Saves the address passed in one address register into a global variable.
- `unlink_and_release_node`: If a counter in the record is zero, unlinks the record from its list and releases it through the exec library FreeMem call, then returns a stored result.
- `set_record_flag_bit_eight`: Sets the bit with value eight in a flag byte of the record it is given, then returns zero.
- `increment_use_count_and_init`: If a record's flag bit with value eight is clear, increments a use counter in the record.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call`: Decrements a 16-bit counter in a record.
- `test_bit_status`: Tests the state of a specific bit within a word or longword.
- `convert_value_bit_by_bit`: Converts a value held in a data register by repeatedly shifting it and comparing it against a running accumulator, building the result one bit at a time over up to 23 steps.
- `adjust_bitfield_value`: Sets one bit in a working value, selected by the second input, then repeatedly doubles and subtracts to form a running difference.
- `call_library_and_adjust_flags`: Calls one library routine through a fixed offset and returns its result with bit value eight cleared and bit value two set.
- `call_jump_table_entries`: Calls a sequence of function pointers at fixed offsets from a register base, alternating between two entries with a third entry in the middle, then returns.
- `handle_library_status_sequence`: Calls library routines at fixed offsets in an order chosen by two status checks, then returns a result with some bits changed.
- `dispatch_library_status_branches`: Calls library routines at fixed offsets in an order chosen by a sign value and two status checks, then returns a result with some bits cleared.
- `branch_library_calls_on_sign`: Calls a library routine, then makes further library calls depending on a count and a sign value, and returns a result with some bits cleared.
- `branch_library_calls_on_count`: Makes library calls chosen by a count and a sign value, then returns a result with some bits cleared.
- `call_library_by_sign_flag`: Calls a library routine, then makes either two calls, one call or two other calls depending on a sign value and a flag, and returns a result with some bits cleared.
- `call_library_then_mask_result`: Calls one library routine through a fixed offset, then calls another routine and returns its result with some bits cleared.
- `call_library_twice_then_mask`: Calls the same library routine through a fixed offset twice, then calls another routine and returns its result with some bits cleared.
- `mask_return_value`: Calls a function through a fixed offset twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `build_value_by_shift_subtract`: Clears the sign bit of an input value and handles a large-magnitude case with a call.
- `build_value_by_shift_subtract_2`: Builds a result by repeated shift and subtract steps, up to 24 of them.
- `execute_loop_with_branching`: Runs a shift, add and subtract loop over data registers, returning once a counter runs out.
- `compute_cordic_trig_value`: Reduces an angle by fixed multiples of pi, then runs an iterative 23-step loop of shifts, additions and table-driven subtractions to produce a trigonometric-style value.
- `compute_cordic_trig_value_2`: Returns early when the argument is outside a range, otherwise runs the same iterative 23-step loop of shifts and table subtractions to compute a trigonometric-style value, then calls other routines to store it.
- `run_cordic_iteration_loop`: Runs the 23-step shift and table-subtract loop used by the trigonometric code, then calls another routine to finish.
- `compute_cordic_trig_value_3`: Returns early for small arguments, otherwise runs the iterative table loop to compute a trigonometric-style value, and calls other routines to store the result, depending on a sign argument.
- `run_cordic_iteration_loop_2`: Runs the iterative table loop with a sign-dependent step, then calls another routine to store the result.
- `run_cordic_iteration_loop_3`: Runs the iterative shift and table loop, then branches on a stored argument to call a routine and returns after a few final calls.
- `store_cordic_result_value`: Runs a shift and table-subtract loop over one value, then calls one routine and branches to a final store path depending on a stored argument.
- `run_cordic_iteration_loop_4`: Runs the iterative shift and table loop on an input value, then branches on a stored argument, storing the result through a routine call.
- `accumulate_table_steps_into_result`: Loops over a table of words, adding or subtracting shifted values into two accumulators on each step.
- `empty_stub_routine`: Contains no operations and returns immediately.
- `call_single_routine_and_return_2`: Calls one other routine and returns, with no other work.
- `call_two_routines_and_return`: Calls two other routines in sequence and returns, with no other work.
- `normalize_integer_to_float`: Converts a signed integer into a float-like form by moving it into the high word and normalising it by doubling while adjusting an exponent.
- `normalize_fixed_point_value_2`: This library code performs normalization on a value by adjusting its sign, swapping words, and repeatedly doubling the value while decrementing a counter.
- `call_routine_then_two_more`: Calls one routine, then calls two more in sequence, because a flag that stays clear guards them.
- `call_routine_then_three_more`: Calls one routine, then calls three more in sequence, because a flag that stays clear guards them.
- `call_routines_with_incremented_byte`: When an argument byte is nonzero and adding one to it does not overflow, it calls one routine, then calls two routines passing the incremented byte, followed by one more call.
- `return_from_subroutine`: Performs a standard return from a subroutine to the calling function.
- `empty_stub_2`: This library code is an empty routine.
- `return_from_subroutine_4`: This library routine performs a sequence of instructions that results in an immediate return to the caller.
- `empty_stub_3`: This library code contains five instructions and performs no observable work; it ends immediately.
- `empty_routine`: This library code consists of four 68000 instructions and performs no observable work, immediately returning to its caller.
- `process_complex_math_logic`: This library routine performs a series of conditional checks and calls other routines based on the value of a register.
- `call_four_routines_in_sequence`: Calls four other routines in sequence and returns with no other work.
- `add_or_subtract_table_entries`: Walks a table of values for a counted number of steps, adding each entry to an accumulator while the accumulator is negative and subtracting it otherwise.
- `subtract_table_from_accumulator`: Adds the first table entry to an accumulator, then walks the following entries, subtracting each one while the accumulator stays non-negative, until a counter runs out.
- `encode_float_with_exponent`: Encodes a nonzero value into a float-like form using a shift-and-subtract loop of up to 22 steps, then adds an exponent by repeated doubling and calls routines to store the parts.
- `return_immediately`: This library code routine performs no operations and returns immediately.
- `encode_float_with_exponent_2`: Runs the shift-and-subtract encoding loop of up to 22 steps on a value, then adds an exponent by repeated doubling and calls routines to store the result.
- `run_shift_subtract_with_exit`: Runs a shift and subtract loop that branches on a counter and calls other routines, then adds an exponent by doubling and returns.
- `call_other_routine_thrice`: Calls another routine three times in succession and returns.
- `store_float_parts_by_exponent`: Handles the sign and zero cases by calling routines, then for nonzero values in range computes an exponent and shifts the mantissa, calling routines to store the parts.
- `store_float_parts_by_exponent_2`: Calls two routines first, then returns for zero; otherwise it computes an exponent from the low bits, shifts the mantissa, and calls routines to store the parts.
- `empty_library_stub`: The library code contains no instructions and performs no operation.
- `return_to_caller`: Performs a sequence of instructions and returns to the calling routine.
- `return_from_subroutine_5`: This routine performs a sequence of instructions before returning to its caller.
- `execute_unnamed_routine`: This library code calls another routine.
- `execute_subroutines`: This library routine calls two other routines and then returns.
- `execute_unnamed_routine_2`: This library code performs a call to another routine.
- `execute_unnamed_routine_3`: This library code invokes a routine via a pointer and then returns.
- `invoke_callback_and_return`: Calls the routine whose address is held in an address register and returns the data register value it had on entry, unchanged.
- `call_indirect_return_d0`: Calls the routine whose address is in an address register, then returns the data register value it had on entry.
- `call_routine_and_keep_d0`: Calls one routine and then returns the data register value it had on entry.
- `library_callback_wrapper`: This library code calls a routine via an address register and returns the original value of a data register.
- `dispatch_library_status`: As library code, it calls a routine to check library status and returns the result.
- `build_shifted_value`: As library code, it calls a routine to build a value using shifts and subtractions and returns the original data register value.
- `run_cordic_iteration`: As library code, it calls a routine to perform a sign-dependent iterative table loop and returns the original data register value.
- `execute_cordic_loop`: As library code, it calls a routine to run a 23-step trigonometric shift and subtraction loop and returns the original data register value.
- `compute_trigonometric_value`: As library code, it calls a routine to compute a trigonometric value via an iterative loop and returns the original data register value.
- `store_register_via_pointer`: Calls one routine, then stores the data register value through the address held in an address register.
- `call_sequential_routines`: As library code, it calls one routine followed by three additional routines and returns the original data register value.
- `call_multiple_routines`: As library code, it calls one routine followed by two additional routines and returns the original data register value.
- `call_incremented_routines`: As library code, it calls a routine and then two more routines using an incremented byte value and returns the original data register value.
- `execute_indirect_callback_2`: Executes a routine via a pointer stored in a data register and returns the original value of that register.
- `encode_floating_point`: As library code, it calls a routine to encode a value into a floating-point format and returns the original data register value.
- `call_routine_thrice`: As library code, it calls another routine three times in succession and returns the original data register value.
- `call_indirect_and_return`: This library code calls a routine via a pointer in an address register and returns the value held in the data register.
- `invoke_callback_and_return_2`: This library routine calls another routine via a pointer and returns the original data register value unchanged.
- `pass_data_through_callback`: Invokes a callback routine using an address held in an address register and returns the original value of a data register unchanged.
- `call_library_vector_keep_d0`: Calls a library function through a fixed offset from a stored library base, then returns the data register value it had on entry.
- `call_indirect_with_offset_1`: This library code calls a routine at an address calculated by adding a negative offset to a library base pointer.
- `call_library_vector_keep_pair`: Calls a library function through a fixed offset from a stored library base, then returns the two data register values it had on entry as one pair.
- `invoke_callback_and_return_concat_2`: This library routine invokes another routine via a pointer stored in a global variable and returns the concatenation of two input values.
- `execute_indirect_function`: Retrieves a function pointer from a global variable, applies a negative offset to it, and executes the resulting address.
- `execute_indirect_function_2`: Retrieves a function pointer from a global variable, applies a negative offset to it, and executes the resulting address.
- `execute_indirect_callback`: Fetches a routine pointer from a global variable with a fixed offset and executes it.
- `invoke_callback_and_return_3`: This library code calls a routine via a pointer stored in a global variable and returns the concatenated results in two registers.
- `invoke_callback_and_return_4`: This library code executes a routine through a pointer located at a specific offset in a global variable and returns the concatenated results.
- `invoke_callback_and_return_concat`: This library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: Calls a routine at an address formed by adding an offset held in an address register to a stored library base, then returns the data register value it had on entry.
- `normalize_fixed_point_value`: Normalises a 32-bit value: a negative value is negated, a small value has its words swapped, and then it is doubled repeatedly while an exponent counts down until the top bit is set.

### diskfont.library

- `check_and_set_font_handle`: Calls a library routine to obtain a handle and, if it is nonzero, stores an address argument into a field at a fixed offset before calling a second library routine.
- `check_and_set_font_handle_2`: The routine is library code that calls a routine via a table entry using the library base.
- `open_dos_and_graphics_libraries`: Stores two register arguments in global slots, then opens the dos and graphics libraries.
- `close_library_and_free_block`: Closes a library and frees a memory block through the exec library, then returns zero.
- `release_idle_entries_and_unload`: Walks a list of font records and unlinks each one whose use count has reached zero, releasing it through a library routine.
- `release_record_or_flag_pending`: If a record has no pending uses and a zero count, it closes the libraries, frees the record and returns its handle value.
- `set_record_pending_flag`: Sets the pending flag with value 128 in a record byte and returns zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Clears the top bit of a flag byte in a record and increments a short counter stored at another fixed offset.
- `decrement_counter_and_release`: Decrements a 16-bit counter in a record.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `scan_font_directory_entries`: Copies names and attribute fields from a linked list of font entries into a caller buffer, and optionally scans a directory of font files to add more.
- `append_font_entries_to_buffer`: Continues copying font list entries into a buffer, writing names and attribute fields and checking a directory when a flag bit is set.
- `copy_font_metadata_to_buffer`: Iterates through a linked list of font records to copy their names and associated metadata into a destination buffer.
- `scan_fonts_directory_into_list`: Scans the FONTS: directory, copying each font name into a buffer and allocating list records through the exec library.
- `scan_dot_font_files`: Looks for entries whose names end in the dot-font suffix, opens each candidate through the dos library, and adds matching entries to a buffer.
- `copy_font_names_into_buffer`: Copies font name bytes into a buffer, then reads attribute records through the dos library and adds each matching entry.
- `parse_font_record_metadata`: Iterates through font attribute records to extract and store metadata such as offsets, lengths, and flags into a destination buffer.
- `read_font_records_and_release`: Reads attribute records for font entries through the dos library, releasing locks and memory as it goes.
- `return_zero_stub`: Returns zero and performs no other work.
- `compute_weighted_font_distance`: Compares two font attribute records over seven flag bit positions, subtracting a weighted table value for each bit that differs, after starting from a score based on a 16-bit field difference.
- `compute_font_match_score`: Compares two font attribute records over seven flag bit positions, subtracting a weighted table value for each bit that differs, and returns the remaining score.
- `subtract_weighted_flag_differences`: Subtracts weighted table values for each mismatched flag bit from a starting score across seven bit positions, then returns the score.
- `compute_font_match_score_2`: This library routine compares two font attribute records over seven bit positions and subtracts a weighted table value for each differing bit.
- `return_constant_value`: Returns a fixed constant and uses no arguments or registers.
- `open_font_and_register`: Opens a font through the graphics library, copies its attributes into a list entry and adds the entry to a font list.
- `load_font_entries_from_directory`: Locks the FONTS: directory, reads font headers through the dos library and loads the best-scoring candidates into a list.
- `process_font_glyph_data`: Iterates through a collection of font glyphs to find the one with the highest score based on a comparison routine.
- `select_best_scoring_font`: Repeatedly picks the candidate with the highest score, loads it into a list and clears its slot until no candidate remains.
- `update_font_metadata`: Iterates through a collection of font data structures to find the highest scoring entry and updates associated metadata.
- `release_font_working_records`: Clears font slots and releases working memory, file locks and font handles through the dos, exec and graphics libraries.
- `search_font_entries`: Iterates through a list of font data structures to find the entry with the highest value in a specific field.
- `load_font_file_header`: Opens a font file through the dos library, checks its size and name, and allocates a memory block for its header.
- `load_disk_font_table`: Builds a table for a disk font: it allocates a memory block, fills its header and copies entries from the source records, counting them.
- `free_stacked_font_block`: Reads a pointer from the caller's stack and passes it to the exec memory release routine.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem_2`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination until a zero byte is copied, including the terminator.
- `advance_string_end_and_call`: Advances to the terminating zero byte of a string, then calls another routine with that position and the second argument.
- `skip_string_terminator_and_call`: Advances a pointer to the terminating zero byte of a string, then calls another routine with that pointer.
- `measure_string_length`: Counts the bytes of a zero-terminated string before its terminator and returns the count.
- `count_null_terminated_bytes`: Reads bytes from a pointer, counting each one including the zero terminator, and returns the count.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte, returning minus one if the first is smaller, one if it is larger, and zero if they are equal.
- `compare_null_terminated_strings_3`: This library code compares two null-terminated strings and returns a value indicating their relative order.
- `compare_null_terminated_strings_2`: This library routine compares two null-terminated strings and returns a value indicating their relative order.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_character_in_string`: Scans forward through a zero-terminated string for a given byte and returns its address, or zero if the terminator is reached first.
- `find_byte_in_string`: This library code searches for a specific byte within a null-terminated string and returns a pointer to its location.
- `find_character_in_string_2`: This library routine searches for a specific byte within a null-terminated string and returns a pointer to its location.
- `find_last_character_in_string`: Scans backward from a pointer for a given byte, stopping at a lower bound, and returns the address where it is found, or zero if it is not.
- `search_string_backwards`: This library code scans a string backwards to find a specific byte, returning a pointer to the match or zero if the boundary is reached.
- `search_backward_for_character`: This library routine scans a string backwards to find a specific byte, returning a pointer to the match or zero if the boundary is reached.
- `skip_path_prefix`: Looks for a slash, or failing that a colon, in a path string and returns the position just after it, or the original pointer if neither is present.
- `increment_returned_value`: Returns its input value increased by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine`: This library routine calls another routine and then returns.
- `call_another_routine`: The routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:HOOKTRO

- `unpack_rnc_block_list`: Walks a linked list of memory blocks looking for one tagged as RNC-compressed, then decodes it with literal copies and back-references into newly allocated memory.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a masked global value matches the second word of a pair.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Ands a word with a mask word, and if a byte value is less than another byte it calls another routine and returns that routine's result instead of the masked word.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `build_bit_length_tables`: Reads a count and fills a byte buffer on the stack with values from a source routine.
- `process_memory_buffer`: This routine iterates through a list of pointers, performs arithmetic on values retrieved from a global pointer, and updates a buffer by adding values to existing entries.
- `return_from_subroutine_2`: This routine is empty and performs only a return instruction.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:WORMS

- `load_hunks_and_relocate`: Program entry that walks an executable image, checks each block header against the loader's block type codes, allocates memory for each block through the system library and copies its contents in.
- `spin_down_word_counter`: Counts a 16-bit register down through zero until it wraps around, with no other effect.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning.
- `decode_backward_lz_stream`: Decodes a compressed stream into a buffer working from the end towards the start, reading run-length codes from a bit reader and either copying literal bytes or repeating bytes from a distance behind the output position.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bits_from_bitstream`: Shifts a requested number of bits out of a packed bitstream one at a time, carrying them through a 32-bit working register and refilling it from memory when the source runs empty.
- `execute_trap_instruction`: This routine executes a software trap instruction and then attempts to invalidate cache lines.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:HDWORM

- `invoke_vector_twice_store_results`: Program entry that clears a byte before each of two calls through a library function vector, storing each returned value into a global slot.
- `call_three_routines_return_zero`: Calls three other routines in sequence and returns zero, adding no work of its own.
- `store_four_call_results`: Calls four other routines in turn and stores each returned value into one of four fixed slots at offsets from a base register.
- `call_vector_and_bias_result`: Calls a function through a pointer stored at a fixed offset from the global base.
- `free_memory_via_library`: Calls the system memory-release routine through a function vector held at a fixed offset from the global base, then returns.
- `stub_return_zero`: Returns zero immediately and performs no other work.
- `retry_open_library_until_ok`: Repeatedly calls the system library-open routine through its function vector until it returns a non-zero handle, with no other work in the loop.
- `close_library_via_vector`: Closes a system library through its function vector at a fixed offset from the global base, then returns.
- `preserve_global_across_calls`: Saves a global variable, calls two other routines, and then restores the original value to the global variable.
- `copy_bytes_with_zero_terminator`: Copies a byte sequence from a source pointer to a destination pointer, moving eight bytes at a time for the bulk and single bytes for the remainder.
- `clear_two_globals_and_call`: Clears two global variables to zero, calls one other routine, and returns the address of this routine.
- `repeat_call_until_zero_flag`: Calls one other routine repeatedly and keeps looping until the zero condition flag is set, then returns.
- `allocate_sized_block_and_link`: Allocates a block of the requested size through the memory-allocation vector and, on success, records the size in the block header and links the block into a list, returning the address just past that header.
- `free_listed_block_or_allocate`: Searches a linked list for the block at the given address; if it is found, the block is unlinked and returned through the memory-free vector.
- `find_list_tail_2`: Follows next-pointers through a linked list until it reaches an entry whose link is zero, then stores that last entry's address in a global slot.
- `preserve_global_across_two_calls`: Saves a global variable, calls two other routines, then writes the saved value back so those calls leave the global unchanged.
- `preserve_global_during_list_operations`: Saves a global variable to a register, calls two linked list management routines, and restores the original value to the global variable.
- `allocate_and_link_node`: Allocates a memory block and links it into a list by updating pointers.
- `manage_linked_list`: Searches a linked list for a specific node to unlink it or allocates a new node if the list is empty.
- `unlink_block_from_list`: Walks a linked list to find the entry whose link points to the given block, then removes that link and releases the block through the memory-free vector.
- `allocate_block_at_list_head`: Allocates a block through the memory-allocation vector and links it at the head of a list, storing the previous head as its first longword and clearing the second.
- `free_all_list_entries`: Releases the first list entry through the memory-free vector, then walks the chain of entries attached to it and releases each one.
- `tiny_ori_2`: Small routine of 1 instruction that merges 16 into d0.

### Worms (1995)(Ocean)(M3)(Disk 1 of 3)[cr OTT][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `zero_memory_from_own_address`: Stores a routine address in a global variable, raises trap number zero, clears that variable, then zeroes memory in longword steps from this routine's own address up to a fixed upper bound.
- `draw_text_and_setup_copper`: Draws zero-terminated text strings into bitplane memory by copying eight glyph rows per character from a font table, with a line stride of 80 bytes per row.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_minus_one`: Returns minus one as its result and performs no other work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_libraries_and_init_data`: Opens a sequence of system libraries through their function vectors, stopping early if any open returns nothing, then copies a block of data one longword at a time into a fixed buffer.
