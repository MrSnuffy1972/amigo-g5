# Game 1596 (pga tour golf (1991)(electronic arts)[cr agl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01596). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (958496 top-level bytes in 43 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 319 of 319 named (0 author symbols), 319 of 319 described (61 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2672 of 2672 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 287738 bytes, 14 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7552 of 7758 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 451918 |
| code | 27368 |
| data | 465217 |
| padding | 8873 |
| unused | 5120 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:Golf | 7 | 233 | 7 |
| Disk-Validator | 1 | 386 | 1 |
| PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3688 | 144 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 129 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 481 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1839 | 136 |
| PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:TR1.EGA (raw code file, load address unknown) | 1 | 0 | 0 |
| PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:FXLARGE (raw code file, load address unknown) | 1 | 261 | 3 |
| PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:GR3.EGA (raw code file, load address unknown) | 1 | 0 | 0 |
| PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:FLYMUS (raw code file, load address unknown) | 1 | 246 | 4 |
| PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:gmusic (raw code file, load address unknown) | 1 | 246 | 4 |
| PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:GR2.EGA (raw code file, load address unknown) | 1 | 0 | 0 |
| PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:FXSMALL (raw code file, load address unknown) | 1 | 239 | 3 |

## Cases handled

- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:AVENEL.PAK: high-entropy data file (7.66 bits/byte), classified as packed data
- runtime segment an address: 1614 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 49 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 35 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 745 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:TR1.EGA: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:FXLARGE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:GR3.EGA: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:FLYMUS: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:gmusic: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:GR2.EGA: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:FXSMALL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf: blocks 986-986 (bytes 504832-505344) outside the filesystem hold data, not code (opcode density 0.031, entropy 5.79)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf: blocks 1012-1013 (bytes 518144-519168) outside the filesystem hold data, not code (opcode density 0.023, entropy 5.68)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf: blocks 1015-1015 (bytes 519680-520192) outside the filesystem hold data, not code (opcode density 0.016, entropy 5.68)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf: blocks 1021-1021 (bytes 522752-523264) outside the filesystem hold data, not code (opcode density 0.027, entropy 5.25)
- pga tour golf (1991)(electronic arts)[cr agl]/PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:Golf: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 7 never-loaded region(s), 287738 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `reset_dma_registers` (gmusic (raw code file, load address unknown)): Sets a global variable to a specific value and clears a hardware register or global variable. Kind: loader/setup routine. Pass path: sub_0028 (initialization/setup path).

## Functions


### PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:Golf

- `run_callbacks_then_free_memory`: Program entry that calls several unnamed routines, invokes a function pointer taken from a global table in a loop, clears a longword in memory, and then frees memory through the Amiga exec library.
- `decrement_word_until_minus_one`: Decrements a 16-bit register variable in a loop until it reaches minus one, then returns.
- `decode_and_accumulate_values`: Runs three passes that call a function pointer and an unnamed decoding routine to read values, then adds each value into a memory array at an index built from those reads.
- `merge_zero_into_d0`: Two-instruction stub that calls a routine merging zero into the D0 register.
- `unpack_stream_backwards`: Decodes a bit-packed stream read from its end toward its start, using a bit buffer and length values from global data, and writes the decoded bytes downward from a destination pointer.
- `copy_decoded_byte_groups`: Copies bytes in groups of four, reading backwards from the source and writing forward into the destination.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Disk-Validator

- `scan_bit_range_and_count`: Writes a two-digit count as characters into a text buffer, fills an index table with minus-one entries, and loops over a bit range comparing bits from two table words while counting the positions that differ.

### PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `lookup_table_word_by_index`: Doubles the 16-bit input to form a byte offset, reads a 16-bit value from a fixed table in the program's data, and returns it packed with the shifted index.
- `set_flag_and_write_marker`: Stores one in a 16-bit table entry indexed by the parameter and writes the byte value 94 into a pointer slot derived from the same parameter plus sixteen.
- `clear_flag_and_reset_marker`: Stores zero in a 16-bit table entry indexed by the parameter and writes the byte value 32 into the paired pointer slot.
- `toggle_flag_and_branch`: Flips the low bit of a byte in a table entry indexed by the parameter, calls an unnamed routine, and then calls one of two other unnamed routines depending on whether that result is zero.
- `call_single_routine_and_return`: Makes one call to an unnamed routine and returns.
- `run_six_iteration_update_loop`: After two tests through unnamed routines, passes a fixed table address and a count of twelve to a routine, then loops six times calling one of two routines according to a result.
- `conditionally_invoke_table_update`: Checks a result from an unnamed routine and then calls a routine with a fixed table address and a count of twelve, making an extra call with the same address first when a second test succeeds.
- `call_fixed_table_update`: Calls one unnamed routine with a fixed table address, a second table address and the count twelve, then returns.
- `set_or_clear_bitplane_pixel`: Sets or clears one bit in each of four bitplanes spaced 8000 bytes apart, using a mask from a small table and the bits of a global value to decide whether each plane is set or cleared.
- `wait_while_status_bit_set`: Loops while a status bit in a byte two bytes past the pointer is set, then returns.
- `program_blitter_row_loop`: Writes the blitter's mask, modulo, pointer, control and size registers, then loops over rows calling unnamed routines to start each blit.
- `draw_rect_if_ready`: Returns zero when a state flag is set or the two stored counters differ.
- `draw_rect_if_counters_match`: Returns whether two stored counters in a record match.
- `draw_rect_at_offset`: Multiplies a width field by a second field to form a byte offset, makes two unnamed calls around a draw call using two base addresses plus that offset, and returns one.
- `call_routine_over_index_range`: Loops over an inclusive range of indices and calls one unnamed routine for each index.
- `call_routine_over_row_span`: Makes calls for a widening set of offsets from the start values up to a count, then calls an unnamed routine twice for each index in the start-to-end span.
- `iterate_rectangle_rows_calls`: Walks rows between two fields of a record and calls unnamed routines with values derived from the record's fields on each row, then repeats two calls for every index in a second range.
- `call_routine_twice_per_index`: Loops over an index up to a limit read from the stack frame and calls an unnamed routine twice for each index.
- `call_routine_for_offset_pairs`: Loops from one to a count, calling an unnamed routine with the start value minus the index, with the end value plus the index, and twice with the start value.
- `reset_pointer_if_enabled`: When a global flag is nonzero, calls an unnamed routine with a fixed address and the value stored at the pointer, then stores a different fixed address into that pointer.
- `query_and_store_value`: Calls several unnamed routines to set up a small record and tests two results.
- `call_two_routines_in_order`: Calls two unnamed routines one after the other and returns.
- `call_routine_then_second`: Calls an unnamed routine and then a second routine located elsewhere in the program's code, then returns.
- `lookup_checked_and_compare`: Looks up a value through an unnamed routine using a fixed selector, rejects the result when an optional check fails, then returns one if a computed value equals the third argument and zero otherwise.
- `lookup_handle_and_compare`: Looks up a handle with a second fixed selector, computes a value from it with another unnamed routine, and returns one when that value equals the third argument and zero otherwise.
- `call_frame_arg_return_one`: Calls an unnamed routine with an argument read from the frame, calls a second unnamed routine, and returns one.
- `call_frame_arg_return_zero`: Calls an unnamed routine and passes a short value from the frame to a second routine, then returns zero.
- `call_frame_arg_ok_result`: Calls two unnamed routines, passing a value read from the frame to the second, and returns one.
- `lookup_and_report_found`: Looks up a handle with a fixed selector through an unnamed routine, calls one of two other routines depending on whether the handle is zero, and returns whether a handle was found.
- `copy_bytes_forward`: Copies a block of bytes from the source to the destination one byte at a time, for the number of bytes given as the third argument.
- `toggle_state_against_parameter`: Compares a global state value with the parameter.
- `reset_state_if_matched`: Compares a global state value with a longword read from the table base.
- `advance_state_if_matched`: Compares a global state value with a longword read from the table base.
- `set_flag_from_state_match`: Compares a global value with a longword read from the table base and sets the global to one when they are equal and zero otherwise.
- `clear_state_return_two`: Sets a global state value to zero and returns two.
- `set_flag_if_zero`: Stores one in a global flag when the argument is zero and zero otherwise.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `swap_adjacent_bytes`: Swaps each adjacent pair of bytes in a buffer, stepping two bytes at a time for the count given as the second argument.
- `reverse_bytes_in_quads`: Reverses the byte order of each four-byte group in a buffer, for the number of groups given as the second argument.
- `toggle_flag_after_call`: Calls an unnamed routine and then inverts the low bit of a global flag.
- `nop_padding_sequence`: This routine is empty and consists of three instructions that perform no operation.
- `reset_global_and_run_sequence`: Sets a global value to zero, then makes a fixed sequence of nine calls to unnamed routines, one of which is a second routine located elsewhere in the program's code.
- `call_routine_around_fixed_address`: Calls an unnamed routine, then another routine with a fixed address argument, then the first routine again.
- `call_routine_around_second_address`: Calls an unnamed routine, then another routine with a different fixed address argument, then the first routine again.
- `call_routine_with_fixed_address`: Calls one unnamed routine passing a fixed address.
- `call_routine_with_other_address`: Calls one unnamed routine passing a different fixed address.
- `set_global_with_routine_call`: Calls an unnamed routine with the current global value and the low word of the argument, stores that word in the global, and calls another routine with the word repeated into both halves.
- `call_when_global_in_range`: Checks whether a global value equals a value from the first argument and falls within a range bounded by the two arguments, and calls an unnamed routine with that value when the test passes.
- `call_routine_for_two_spans`: Makes two calls to an unnamed routine, each passing the global value together with the low and high words of one of the two arguments.
- `call_routine_for_span_pair`: Makes two calls to an unnamed routine, each passing the global value together with words taken from the two arguments.
- `classify_character_code`: Classifies a character using a flag table and maps certain punctuation characters to numeric codes, updating a global flag for some of them and returning a code or zero.
- `repeat_call_and_flush_flag`: Calls an unnamed routine to get a count, calls another routine that many times with the argument, then calls a routine and clears a global flag when that flag is set.
- `call_with_global_then_store`: Calls an unnamed routine with the current global value and the low word of the argument, then stores that word into the global.
- `emit_three_calls_with_global`: Calls an unnamed routine to get a length, then makes three calls to another unnamed routine with the global value, a zero, the high word of the first argument plus six, and the length.
- `emit_calls_with_parity_offset`: Gets a length from an unnamed routine, chooses an offset of zero or four from its low bit, and makes three calls to another routine with the global value and offset coordinates.
- `forward_call_to_routine`: Passes both arguments to an unnamed routine and returns.
- `emit_call_with_parity_shift`: Gets a length from an unnamed routine, shifts the first argument by a multiple of that length with an offset of zero or two depending on its low bit, and passes the result to another unnamed routine.
- `forward_call_either_branch`: Passes two arguments to an unnamed routine whichever branch of a global flag test is taken, so both branches make the same call.
- `forward_frame_values_call`: Calls an unnamed routine with a word and a longword read from the frame, then returns.
- `forward_call_both_branches`: Passes two arguments to an unnamed routine on both branches of a flag test, so the flag has no effect on the call.
- `draw_clipped_nibble_pixels`: Clips a rectangle to a 320 by 200 screen, then draws each packed byte as two pixels, high nibble then low nibble, by calling an unnamed routine for each pixel.
- `draw_nibble_pixels_in_rect`: Walks a rectangle's rows and columns reading packed bytes and passes each high and low nibble to an unnamed routine.
- `draw_nibble_pixels_unclipped`: Walks rows and columns of a rectangle, passing the high and low nibble of each packed byte to an unnamed routine without bounds clipping.
- `advance_packed_pointer_by_rows`: Advances the source pointer across the rows of a rectangle, then draws nibble pixels for the remaining cells through an unnamed routine.
- `draw_nibble_pixels_skewed`: Walks a packed source along a skewed grid, checking bounds on each cell, and draws a nibble-derived pixel through an unnamed routine when the cell lies inside the rectangle.
- `draw_nonzero_nibble_pixels`: Walks a packed source over a rectangle and calls an unnamed routine for each nonzero nibble inside the bounds, taking the high nibble first and then the low nibble.
- `draw_nibble_pixels_offset`: Walks a packed source over a skewed rectangle and passes each nonzero high and low nibble that lies inside the bounds to an unnamed routine.
- `draw_nibble_grid_then_store`: Walks a grid of packed bytes, passing each high and low nibble to an unnamed routine, then stores a frame value into the global.
- `draw_nibble_rows_then_store`: Walks rows of packed bytes, passing each high and low nibble to an unnamed routine, and stores a frame value into the global once all rows are done.
- `draw_nibble_pairs_until_done`: Passes the high and low nibbles of each packed byte to an unnamed routine, advancing across columns and rows until the row counter reaches its limit, then stores a frame value in the global.
- `build_record_and_submit`: Builds a record on the stack from fields of the argument, fills in fixed bytes and longwords, and submits it through unnamed routines, but only after a check on a field or a routine result.
- `build_stack_record_and_call`: Fills a stack record with a word returned by an unnamed routine, fields copied from the argument and fixed bytes, then passes the record to another routine.
- `call_routine_with_record_fields`: Passes the stack record, several fields from the argument, the global value and the constant 192 to an unnamed routine.
- `fill_four_values_from_routine`: Calls an unnamed routine four times using the same word field of the argument and stores each returned longword into successive fields of the argument.
- `flush_four_pending_entries`: For each of four stored entries that is nonzero, calls an unnamed routine with the entry and two coordinate words, then clears that entry.
- `call_routine_for_twelve_rows`: Calls an unnamed routine twelve times, passing a base address that advances by forty bytes each time.
- `update_row_record_from_table`: Looks up a value through an unnamed routine and a table, computes a row offset from two argument fields for forty-byte rows, and writes fields into the row record, with the global flag choosing the write path.
- `copy_record_compute_byte_width`: Copies position and size fields from a source record into the current record, computes a width in bytes by dividing the x value by eight and rounding up when there is a remainder, and makes two unnamed calls.
- `set_default_rectangle_globals`: Stores eight constant values into the global variable in sequence: 320, 8, 0, 0, 240, 55, 0 and 0.
- `copy_globals_to_record_slot`: Copies several global fields into a forty-byte record slot selected by the parameter, then calls an unnamed routine.
- `store_record_into_slot`: Copies fields from a record passed in the frame into a forty-byte slot selected by an index, then calls two unnamed routines with that slot.
- `process_slot_if_active`: Calls an unnamed routine with the forty-byte slot address selected by the parameter, but only when a longword field of that slot is nonzero.
- `process_slot_then_call`: Calls an unnamed routine with the forty-byte slot address when a longword field is nonzero, then makes one further call to another unnamed routine.
- `fill_scaled_index_table`: Makes two unnamed calls, then fills a table of two hundred words with the global value times each index, and finally sets the global to one.
- `claim_sprite_channel_two`: Attempts to claim sprite channel two through an unnamed routine.
- `release_sprite_channel_two`: Releases sprite channel two: if the global flag is set it calls a routine with the channel number, then clears the flag.
- `claim_sprite_channel_three`: Attempts to claim sprite channel three through an unnamed routine.
- `release_sprite_channel_three`: Releases sprite channel three: if the global flag is set it calls a routine with the channel number, then clears the flag.
- `claim_sprite_channel_four`: Attempts to claim sprite channel four through an unnamed routine.
- `release_sprite_channel_four`: Releases sprite channel four: if the global flag is set it calls a routine with the channel number, then clears the flag.
- `call_routine_with_global_value`: Passes the global value to an unnamed routine and returns.
- `run_conditional_calls_on_flag`: Makes unnamed calls depending on whether the global value is nonzero: one call always, one more call and two further calls only when it is nonzero, and a final call every time.
- `decode_sixteen_nibble_words`: Loops sixteen times over a table of words, splitting each word into three four-bit fields and passing them with the index and the global value to an unnamed routine.
- `copy_two_words_from_global`: Copies two words from fixed offsets in a record referenced by the global value into the two destination pointers.
- `classify_record_code`: Looks up a record through an unnamed routine and sets the global to a code when particular flag bits are set: one or two for two specific stored values, and the stored value itself when it is below a limit.
- `open_system_libraries`: Sets up global values and calls unnamed routines to open the diskfont, dos, graphics and intuition libraries, checking each result for zero.
- `setup_double_buffered_bitplanes`: Stores two buffer addresses in the global table, calls an unnamed routine with depth four, width 320 and height 200 for each buffer, and fills four bitplane pointers spaced 8000 bytes apart.
- `swap_display_buffers`: Stores the current buffer pointer into a field, flips the buffer index between two values, and calls unnamed routines to make the other buffer current.
- `copy_fields_from_buffer_one`: Sets the global to one, then copies a pointer chosen from the global table into several record fields and loads the global from the fields of the chosen pointer.
- `copy_fields_from_buffer_zero`: Sets the global to zero, then copies a pointer chosen from the global table into several record fields and loads the global from the fields of the chosen pointer.
- `report_graphics_init_failure`: Calls unnamed routines selected by bits of a global value and always makes a final call with a fixed address.
- `release_resources_by_flag_bits`: Clears and handles flag bits in a global value, calling unnamed routines for each set bit and for each nonzero stored pointer, to release resources in turn.
- `store_constant_global_value`: Stores a fixed constant in the global variable.
- `copy_record_words_set_a`: Copies six consecutive words from a twelve-byte table entry in the first table area, selected by the parameter, into the global variable one at a time.
- `copy_record_words_set_b`: Copies six consecutive words from a twelve-byte table entry in the second table area, selected by the parameter, into the global variable one at a time.
- `set_global_sequence_fifteen`: Stores zero, then fifteen, then four into the global variable, leaving four as its final value.
- `set_global_sequence_fourteen`: Stores zero, then fourteen, then four into the global variable, leaving four as its final value.
- `call_by_flag_then_finish`: Calls one of two unnamed routines depending on whether the global value is zero, then makes one more call to an unnamed routine.
- `return_global_value`: Returns the 16-bit value of the global variable.
- `select_resource_address_and_clear`: Picks one of two fixed addresses according to whether an unnamed routine returns one, then clears the global.
- `run_pending_global_calls`: Makes up to two calls to an unnamed routine, each only when the global value is nonzero, and clears the global after each call.
- `allocate_or_report_memory_error`: Requests a block through an unnamed routine and, when none is returned, reports a memory error with the block size.
- `call_global_function_pointer`: Calls the routine whose address is stored in the global variable and returns whatever value is left in the result registers.
- `fill_masked_span_four_planes`: Fills a horizontal span across four bitplanes, applying start and end word masks from small tables and writing the middle words in a loop with a pattern taken from the global value.
- `store_dma_control_constant`: Stores a fixed constant in the global variable.
- `stub_no_decompiled_code`: The decompiler timed out, so no pseudo-code exists for this routine.
- `copy_defaults_and_set_size`: Copies seventy-two longwords from a fixed table into the global area, then stores the values 319, 199, 320 and 200 as size fields and calls an unnamed routine.
- `store_selected_default_pointer`: Chooses one of two fixed table addresses depending on whether the global is zero, then stores the longword found there into two fixed memory locations and calls an unnamed routine.
- `copy_default_longword_to_pointers`: Copies a longword from a fixed address into the two destinations passed to it.
- `write_checker_pattern_words`: Writes pairs of 16-bit checkerboard patterns (21845 and 43690) or the all-ones value 65535 into the global, selected by bits 0, 1, 16 and 17 of the argument.
- `call_when_global_in_span`: Checks whether a global value falls within a span bounded by two arguments and calls an unnamed routine with the high word of the first argument when the test passes, with a second branch for the outside-span case.
- `build_linked_index_lists`: Walks index chains stored in a table starting from the global value, copying chain entries into two stack arrays of twenty words each.
- `copy_index_chains_into_buffer`: Calls an unnamed routine with two frame bytes, then walks the same index chains as its neighbouring routine and copies each chain's entries into a buffer.
- `flatten_chain_entries`: Follows index chains from the global value, writing each visited entry into a buffer, then processes the collected entries in a second loop.
- `accumulate_chain_values`: Follows index chains again, writing each entry and a computed value into two buffers, and loops until the chain is finished.
- `accumulate_table_counts`: Adds a longword from a table into a second table entry for each index in the buffer, and writes the high word of each sum into an output buffer.
- `sort_buffer_words_ascending`: Compares each buffer word with its neighbour and swaps them when they are out of order, repeating across the buffer as a single sorting pass.
- `shift_buffer_entries_down`: Shifts entries of a word buffer down by one position in a loop, adjusts the counters that describe the buffer, and makes two unnamed calls.
- `step_index_chains_with_calls`: Loops over the index chains from the global value, making unnamed calls for each step, and returns once the index passes the global value.
- `clear_chain_table_and_reset`: Zeros a table of one hundred longwords starting at a fixed address, then sets the global to one and back to zero.
- `clear_final_quarter_words`: Increments the argument, computes a pointer into a table from it, and zeroes the final quarter of the words, from the count minus a quarter up to the count.
- `fill_words_to_limit`: Stores a fixed word value into consecutive words from the current pointer until the counter reaches the limit held in the frame.
- `execute_sequence_of_routines`: calls a series of unnamed routines, performs table fills, and manages sprite channel claims and global flag toggles.
- `call_three_unnamed_routines`: The decompiler timed out, so no pseudo-code exists for this routine.
- `compute_even_byte_count`: Divides the argument by eight to get a byte count, adds one when any of bits 16 to 18 are set, and rounds the result up to an even number.
- `allocate_plane_record`: Allocates a block sized from a stored field, using four or five times that field depending on a flag, and stores the block pointer and derived offsets in the record.
- `free_plane_record`: Frees the block held in a record through an unnamed routine, passing four or five times the argument depending on a flag, then clears the record's pointer and offset fields.
- `process_all_table_entries`: Loops over one hundred and eighty forty-byte table entries, passing each entry's pointer and two of its words to an unnamed routine.
- `process_entry_range`: Processes the forty-byte table entries whose indices lie between two values read from an index table, calling an unnamed routine for each one.
- `process_indexed_entry`: Looks up a forty-byte table entry through an index table using the argument, then passes its pointer and two of its words to an unnamed routine.
- `clamp_minimum_and_call`: Clamps the first argument to a minimum of sixteen, passes it with the second to an unnamed routine, calls another routine when that returns zero, and returns the result.
- `unpack_nibble_rows`: Unpacks rows of nibble-packed data from the source into the destination, rounding the word count up when flag bits are set.
- `unpack_nibble_rows_three_buffers`: Unpacks rows of nibble-packed data using three pointer buffers, rounding the word count up when flag bits are set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_external_entry_point`: Calls a routine at a fixed address outside this segment and returns.
- `call_external_entry_wrapper`: Calls one unnamed routine and returns.
- `run_boot_setup_sequence`: Sets up a global value, calls unnamed routines, runs a loop of nineteen calls, then calls the boot entry routine, which finds the dos library module at startup, and sets the global to one.
- `delegate_to_unnamed_routine`: Makes one call to an unnamed routine and returns.
- `setup_eight_row_sequence`: Sets the global to fifty-two, calls an unnamed routine with fixed arguments, makes eight calls to another unnamed routine across a table, then calls a fixed routine when the global is zero.
- `set_table_entry_to_one`: Stores one in a word table entry chosen by the global value.
- `set_table_entry_to_two`: Stores two in a word table entry chosen by the global value.
- `call_sequence_with_zero_args`: Makes four calls to unnamed routines, passing zero arguments to the second call.
- `run_init_sequence_return_byte`: Calls a series of unnamed and fixed routines, toggling the global between one and zero around them, then returns a byte combined with 244 by a bitwise OR. The listing warns of overlapping instructions, so part of it may be misdecoded.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_counter_with_parity_check`: Increments a global counter and returns early when the new value is even and a second test on the counter passes.
- `round_high_words_three_times`: Repeats three times: shifts the global right by sixteen bits and adds one when the resulting word's top bit is set.
- `stub_self_assignment`: Assigns the global to itself three times, so it has no effect.
- `check_value_limits_and_reset`: Tests a global value through an unnamed routine against two limits.
- `clear_global_value`: Sets the global variable to zero and returns.
- `get_global_word_value`: Returns the 16-bit global value.
- `update_twelve_byte_entry`: When the global value is nonzero, makes an unnamed call, then selects a twelve-byte entry through another unnamed routine and writes its fields differently depending on whether the global equals one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_one_unnamed_routine`: The decompiler timed out, so no pseudo-code is available.
- `call_routine_with_stack_frame`: Calls an unnamed routine, passing the argument and a pointer to the caller's stack frame.
- `call_two_unnamed_routines`: The decompiler timed out, so no pseudo-code is available.
- `format_signed_number_text`: Converts a value read through a pointer into characters written backwards from a buffer using a digit table and a sign flag, then returns the pointer to the first character.
- `write_digits_from_table`: Writes the digits of a value backwards into a buffer by repeatedly taking a remainder through an unnamed routine and looking up each digit in a character table, tracking a sign flag.
- `format_string_with_arguments`: Works through a format string, copying ordinary characters through an output routine supplied as an argument and expanding percent conversions from the argument list.
- `chain_result_to_output`: checks a global variable against a constant, executes a conditional function using the result, and passes the returned value to a write wrapper followed by an output wrapper.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `copy_zero_terminated_string`: Copies a zero-terminated string from the source to the destination byte by byte, including the terminator, and returns the destination pointer.
- `measure_zero_terminated_string`: Counts the bytes of a zero-terminated string and returns the count, excluding the terminator.
- `convert_char_to_uppercase`: Converts a lowercase letter to uppercase by subtracting 32 and returns other characters unchanged.
- `convert_char_to_lowercase`: Converts an uppercase letter to lowercase by adding 32 and returns other characters unchanged.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `execute_indirect_call`: performs an indirect jump to an unrecovered address.
- `call_two_routines_undecoded`: The decompiler timed out, so no pseudo-code exists.
- `handle_file_seek_error`: attempts a seek operation and, if it fails, calls a routine to seek to the start of a file and handles an error by storing a value in a global variable.
- `rewind_dos_file`: Seeks to the start of a DOS file through the seek wrapper, passing zero as both offset and origin, and returns.
- `manage_file_table_entries`: iterates through a table to find an empty entry, performing various file operations including opening, locking, and seeking, while updating table data.
- `perform_read_operation`: calls a read wrapper, checks a signal bit, and handles potential input/output errors.
- `open_dos_library_or_alert`: Zeros a large table of longwords, stores the register value and a global pointer, then opens the dos library with exec's OldOpenLibrary and reports a failure through exec's Alert.
- `stub_immediate_return`: Stub with an empty body that returns immediately.
- `main`: allocates memory for a global structure and parses command line or workbench arguments to set the current directory, redirecting standard input and output streams.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `prepare_text_buffer_from_record`: Chooses a source string from a record or a global pointer, computes a size from it and the argument, allocates a buffer through an unnamed routine and copies the text into it.
- `append_zero_terminated_string`: Appends a zero-terminated string from the source to the end of the destination, copying up to a limit of 32766 characters and stopping at the terminator.
- `append_string_limited_count`: Appends characters from the source to the end of the destination, up to the count given as the third argument, then writes the terminator.
- `copy_string_padded_to_count`: Copies up to the given number of characters from the source, stopping at the terminator, then fills the rest of the destination with zero bytes.
- `multiply_longwords_to_64bit`: Multiplies two 32-bit values taken from the input registers and returns the 64-bit product, built from 16-bit partial products.
- `lookup_window_icon_record`: Opens icon.library through a library vector call, looks up the window's icon entry, and stores its pointer and a value into the first argument record.
- `signed_divide_longwords`: Performs a signed division by calling an unnamed unsigned routine and negating the result when the operand signs differ.
- `signed_remainder_adjust`: Calls an unnamed unsigned routine and negates its result when the first operand is negative, then returns the result with the second operand.
- `forward_unsigned_routine_result`: Calls an unnamed routine and returns the value left in the result registers unchanged.
- `call_routine_discard_result`: Calls an unnamed routine and returns, leaving no result.
- `divide_longword_unsigned`: Performs an unsigned division of a 32-bit dividend by a 32-bit divisor.
- `divide_longword_restoring_loop`: Performs an unsigned restoring division of the two register operands using a sixteen-step bit loop, leaving the quotient and remainder in the registers.
- `forward_call_then_return`: Calls an unnamed routine and returns, with no other work.
- `call_two_routines_timeout`: The decompiler timed out, so no pseudo-code exists.
- `inspect_timed_out_routine`: The decompiler timed out, so no body is available for this 22-instruction routine.
- `visit_record_table_entries`: Steps through a table of records in increments of 22 bytes, from a start offset to an end offset computed from the base register.
- `release_flagged_record`: Returns minus one when given a null record pointer.
- `release_record_without_check`: Performs the same release sequence as the flagged-record routine but without a null check on its pointer register.
- `put_byte_to_stream`: Writes one byte into a buffered stream record, starting buffering when needed; unbuffered writes go to an unnamed routine.
- `find_free_record_slot`: Scans a table of 22-byte records from a fixed start to a fixed end and returns the first record whose status byte is clear.
- `find_free_record_from_cursor`: Starts at the record pointer held in a register and advances in 22-byte steps until it finds a record with a clear status byte.
- `clear_record_header`: Zeroes the three leading 32-bit fields of the record whose pointer is in a register.
- `setup_stream_buffer`: Initializes a stream record after an unnamed routine returns a buffer pointer.
- `release_all_list_nodes`: Walks a singly linked list whose head is stored in a global variable and passes each node, together with its recorded size plus eight, to an unnamed routine.
- `free_linked_memory_list`: Walks a singly linked list in a register, passing each node and its recorded size plus eight to an unnamed routine until the next pointer is null.
- `allocate_and_link_list_node`: Requests a block through a library call sized from its argument plus eight.
- `forward_low_word_argument`: Passes the low word of its single 32-bit argument to one unnamed routine and then returns.
- `unlink_and_release_list_node`: Searches a singly linked list whose head is in a global variable for the node matching its argument, unlinks it, and passes the node and its recorded size plus eight to an unnamed routine.
- `remove_matching_list_node`: Walks a linked list held in registers and finds the node that matches a pointer stored in the caller's record.
- `unlink_list_node`: Unlinks a node by pointing its predecessor, or the global list head when there is no predecessor, at the node's next pointer.
- `pop_list_head_node`: Replaces the global list head with the next pointer of the node held in a register.
- `check_handle_is_interactive`: Checks that its index is below a stored count and that the matching table slot is in use.
- `run_and_record_io_error`: Calls an unnamed routine with its argument and returns zero when that call succeeds.
- `return_zero_stub`: Does nothing except return zero, using only two instructions with no calls or memory writes.
- `write_data_and_fetch_error`: The decompiler timed out, so the body is unavailable.
- `check_signal_bit_result`: Calls a library function with arguments zero and 4096 and tests bit 4096 of its result.
- `run_optional_hook_then_call`: If a global function pointer is set, it calls the function that pointer addresses.
- `release_program_resources_on_exit`: Closes every entry of a file-handle table and frees its storage, then calls a registered exit callback and releases several stored blocks.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `forward_to_library_close`: Loads a library base from a global variable and jumps to the library function at offset minus 36, which the facts identify as the close wrapper.
- `forward_to_library_delete`: Loads a library base from a global variable and jumps to the library function at offset minus 72, which the facts identify as the file-deletion wrapper.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `forward_to_library_lock`: Loads a library base from a global variable and jumps to the library function at offset minus 84, which the facts identify as the lock wrapper.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `forward_to_library_open`: Loads a library base from a global variable and jumps to the library function at offset minus 30, which the facts identify as the open wrapper.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `forward_to_library_seek`: Loads a library base from a global variable and jumps to the library function at offset minus 66, which the facts identify as the seek wrapper.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_parent_dir`: Wrapper that calls the dos library function ParentDir and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `forward_to_library_unlock`: Loads a library base from a global variable and jumps to the library function at offset minus 90, which the facts identify as the unlock wrapper.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `forward_to_library_write`: Loads a library base from a global variable and jumps to the library function at offset minus 48, which the facts identify as the write wrapper.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `forward_to_library_offset_414`: Loads a library base from a global variable and jumps to the library function at offset minus 414 of that base.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `close_handle_table_slot`: Checks that a table index is below a stored count and that its slot is in use.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `forward_to_library_offset_198`: Loads a library base from a global variable and jumps to the library function at offset minus 198 of that base.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `forward_to_library_offset_294`: Loads a library base from a global variable and jumps to the library function at offset minus 294 of that base.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `forward_to_library_offset_210`: Loads a library base from a global variable and jumps to the library function at offset minus 210 of that base.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `forward_to_library_offset_372`: Loads a library base from a global variable and jumps to the library function at offset minus 372 of that base.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `forward_to_library_offset_552`: Loads a library base from a global variable and jumps to the library function at offset minus 552 of that base.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `forward_to_library_offset_300`: Loads a library base from a global variable and jumps to the library function at offset minus 300 of that base.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `forward_to_library_offset_216`: Loads a library base from a global variable and jumps to the library function at offset minus 216 of that base.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_426`: Library stub: loads a library base from a global variable and calls the library function at offset minus 426 of that base, returning its result to the caller.
- `execute_library_call`: performs an indirect call through a pointer retrieved from a library base and an offset.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `forward_to_library_offset_378`: Loads a library base from a global variable and jumps to the library function at offset minus 378 of that base.
- `execute_indirect_function_call`: Calculates an address by traversing two levels of pointers starting from an offset of a base address and executes the resulting function.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `forward_to_library_offset_390`: Loads a library base from a global variable and jumps to the library function at offset minus 390 of that base.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_routine_sequence`: Makes a fixed sequence of calls to unnamed routines and to two fixed-location routines, then returns.

### PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:FXLARGE (raw code file, load address unknown)

- `configure_dma_transfer_table_2`: Iterates through a table-driven structure to calculate and write source or destination addresses and transfer counts into DMA controller registers.
- `configure_dma_channel_transfer`: Configures a DMA channel or a DMA-related table or structure by calculating a source address and a certain number of-cycles or a certain amount of-data.
- `update_dma_transfer_table`: Calculates a destination address and transfer count based on a table, then writes a pointer or value to a memory location and modifies the DMA controller register.

### PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:FLYMUS (raw code file, load address unknown)

- `update_dma_transfer_table_2`: Iterates through a table of control structures to recalculate DMA transfer parameters, including source pointers and transfer sizes, while updating hardware DMA control registers and certain global or table-based pointers.
- `initialize_dma_transfer_data`: Iterates through a table to calculate and store new pointer or value or size information into a global structure, and then populates a secondary table with pointers to other routines.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `reset_dma_control_registers`: Clears a global variable and sets a global variable to a specific value while interacting with DMA and certain custom hardware registers.

### PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:gmusic (raw code file, load address unknown)

- `process_dma_instruction_table`: Iterates through a table to parse custom instruction opcodes, updating DMA transfer pointers, sizes, and custom hardware registers or global pointers based on the instruction type.
- `setup_dma_table_entries`: Populates a global table with calculated pointers and size-related values derived from a source table, and then initializes a separate table with routine addresses.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `reset_dma_registers`: Sets a global variable to a specific value and clears a hardware register or global variable.

### PGA Tour Golf (1991)(Electronic Arts)[cr AGL].adf:FXSMALL (raw code file, load address unknown)

- `configure_dma_transfer_table`: Iterates through a table to configure DMA channel transfer parameters, including source or destination addresses and or transfer sizes, based on a set of encoded instructions and control flags.
- `configure_dma_transfer_table_3`: Iterates through a table-driven structure to calculate and write source or destination addresses and transfer counts into DMA controller registers.
- `configure_dma_transfer_parameters`: Calculates a memory offset using a value from a register and writes a routine pointer, a value, and a certain byte to a specific memory location or table entry, and modifies the DMA controller register.
