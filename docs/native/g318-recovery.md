# Game 318 (Ultima5 v1.2 0785): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00318). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2572159 top-level bytes in 84 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 18 raw-word lines |
| Functions named and described | pass, 158 of 158 named (0 author symbols), 158 of 158 described (11 by rule, 16 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1320 of 1320 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 460325 bytes, 7 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2981 of 3049 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 428097 |
| code | 11540 |
| data | 2132460 |
| padding | 62 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 150 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1048 | 68 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 375 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 924 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 56 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |
| ibm.ch (raw code file, load address unknown) | 1 | 0 | 0 |
| loader (raw code file, load address unknown) | 1 | 4 | 1 |
| look2.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| runes.ch (raw code file, load address unknown) | 1 | 0 | 0 |
| ruptile. (raw code file, load address unknown) | 1 | 0 | 0 |
| shoppe.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| signs.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| u5fx (raw code file, load address unknown) | 1 | 0 | 0 |
| ultimav (raw code file, load address unknown) | 1 | 463 | 6 |

## Cases handled

- Ultima5_v1.2_0785/Ultima5_v1.2_0785.hdf: hard-disk image (AmigaDOS OFS, 2880 blocks): its filesystem parses and all 76 files in it are byte-identical to the extracted tree Ultima5_v1.2_0785_hdf, which is recovered file by file; no block outside the filesystem holds data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/create.16: high-entropy data file (7.96 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/dng1.16: high-entropy data file (7.96 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/dng2.16: high-entropy data file (7.97 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/dng3.16: high-entropy data file (7.97 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/end1.16: high-entropy data file (7.93 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/end2.16: high-entropy data file (7.95 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/items.16: high-entropy data file (7.85 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/startsc.16: high-entropy data file (7.91 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/story1.16: high-entropy data file (7.96 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/story2.16: high-entropy data file (7.95 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/story3.16: high-entropy data file (7.94 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/story4.16: high-entropy data file (7.92 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/story5.16: high-entropy data file (7.91 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/story6.16: high-entropy data file (7.95 bits/byte), classified as packed data
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/ultima.16: high-entropy data file (7.95 bits/byte), classified as packed data
- runtime segment an address: 590 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 332 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 276 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/ibm.ch: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/loader: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/look2.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/runes.ch: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/ruptile.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/shoppe.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/signs.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/u5fx: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Ultima5_v1.2_0785/Ultima5_v1.2_0785_hdf/Ultima5/UltimaVData/ultimav: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 9 never-loaded region(s), 460325 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `install_vectors_and_enable_chips`: Calls two function pointers taken from an argument structure, then writes jump instructions that point at other routines into fixed low-memory vectors.
- `patch_code_locations_then_init`: Overwrites several code locations with return, no-operation, jump and call instructions that point at other routines.
- `spin_if_bit_set`: Checks one bit of a CIA interrupt-control byte and loops forever if it is set.
- `advance_cyclic_counter_table`: Advances a small cyclic counter held in a shared byte, first storing the old value in a table entry and clearing the counter when that old value is even.
- `assign_constant_value`: Stores the value eight into a global variable.
- `write_fixed_text_and_terminate`: When a stored length is zero, sets it to five and writes the five-character text Angel into a buffer.
- `poll_prompt_until_accepted`: Repeatedly calls a fixed routine with a fixed address argument.
- `execute_polling_loop`: Repeatedly calls another routine to check a value and performs conditional calls based on the result.
- `call_handler_then_reset_state`: Calls a function pointer taken from an argument structure and stores its result.
- `dispatch_then_reset_state`: Calls a function pointer from an argument structure when a shared value is negative, otherwise calls the following routine.
- `forward_to_stored_handler`: Calls the function pointer at a fixed offset in a stored structure and returns the two data register values it was given.
- `call_stored_handler_reset_state`: Calls a function pointer at a fixed offset in a stored structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_state_and_mark_init`: Sets a group of state variables to constants and sets a one-time flag when it is still zero.
- `clear_byte_block_from_count`: Zeroes a block of bytes starting at a fixed address, with the block length taken from a shared value.
- `fill_byte_block_with_one`: Sets a block of bytes starting at a fixed address to one, with the block length taken from a shared value.
- `count_zero_and_nonzero_bytes`: Counts the zero and nonzero bytes in a block that starts at a fixed address and is sized by a shared value.
- `count_until_table_entry_nonzero`: Increments a counter in the argument structure on each pass until the byte at an index, taken from a shared value and used against a fixed table, becomes nonzero.
- `clear_long_word_columns`: Calls the following routine, then zeroes ten 32-bit words in each of four columns.
- `copy_long_word_columns`: Calls two routines to obtain addresses, then copies ten 32-bit words in each of four columns from the first area to the second, using the same fixed strides.
- `delay_busy_loop`: Runs a nested counting loop of fixed size to waste time.
- `set_value_doubled_when_flagged`: Stores a shared value into a 16-bit field of the structure, doubling it when the structure's first flag word is nonzero.
- `step_field_count_nonzero_results`: Increments or decrements a 16-bit field according to a flag word.
- `run_timed_passes_until_flag`: Runs several counted passes that each call a few routines, using half and full values of a shared count.
- `mark_byte_values_in_table`: Calls a routine, then runs seven groups.
- `clear_byte_values_in_table`: Walks a byte list until a 255 terminator in each of six groups, setting the table entry for every listed value to zero, and calls a routine after each group.
- `store_shared_value_plus_one`: Stores one more than a shared value into a 32-bit field of the structure.
- `call_and_store_three_words`: Calls a routine three times and stores the same input word into three different 16-bit fields of the structure, one after each call.
- `store_doubled_value_plus_one`: Stores an input word minus one into a 16-bit field, then stores a shared value and twice that value plus one into 32-bit fields of the structure.
- `call_two_routines_in_sequence`: Calls two routines in sequence and returns, with no other work.
- `store_shared_block_address`: Stores the address of a shared block into a pointer field of the structure.
- `run_counted_toggle_loop`: Runs a loop counted down from an input word, toggling a state byte on each pass and calling several routines.
- `reset_counter_and_pointer`: Clears one field, increments a counter in the structure, and stores the address of a shared block into a pointer field.
- `blit_shifted_record_to_planes`: Looks up an image record in a table by index and writes its rows into four bitplanes.
- `clear_flag_and_call_two`: Clears a shared byte and then calls two routines in sequence.
- `call_routine_span_times`: Calls a routine once, then calls it again once for each step of a span computed from two input words.
- `set_flag_and_call_two`: Sets a shared byte to one and then calls two routines in sequence.
- `combine_table_masks_by_position`: Calls a routine to obtain a position value, then looks up 16-bit mask entries in a fixed table using the low nibbles of two inputs.
- `store_shared_value_word`: Stores a shared value into a 16-bit field of the structure and returns its two input registers unchanged.
- `stub_return_inputs_unchanged`: Routine that does no work and returns its two input register values unchanged.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use.
- `draw_bit_pattern_across_planes`: Writes a pattern into four bitplanes, choosing for each plane whether to set, clear or toggle bits according to a shared mask and a draw-mode flag.
- `store_range_then_call_two`: Stores two input values into shared words, then calls two routines when a flag is clear or when a comparison against a shared value falls within a range.
- `store_parameter_to_shared_word`: Stores its input word into a shared 16-bit variable and returns.
- `set_pixel_bits`: Sets or clears bits across four bitplanes based on a bit position and checks input values against a threshold.
- `set_pixel_bit_across_planes`: Calls a routine to obtain a bit position, then sets or clears that bit in one byte of each of four bitplanes according to successive low bits of a shared value.
- `store_words_then_call_conditionally`: Stores two input words into shared variables and calls routines in sequence, calling one more only when a register argument is zero.
- `step_line_with_error_term`: Takes absolute differences between two point pairs and steps along the longer axis with a Bresenham-style error term, calling a routine at each step and once more at the end.
- `step_line_variant_negated_x`: A variant of the line stepper that starts from the negated first difference, with the same error-term loop and per-step call.
- `step_line_variant_negated_y`: Another variant of the line stepper, starting from a negated second difference, with the same error-term loop and per-step call.
- `step_error_term_until_done`: Advances a Bresenham-style error term in a loop, calling a routine after each step, and returns once the step counter reaches zero.
- `finish_line_error_steps`: Advances a Bresenham-style error term through the remaining line steps, calling a routine between steps and once more at the end.
- `step_line_negated`: Performs a line drawing algorithm using a second difference error term and a loop to step through coordinates.
- `test_coordinates_inside_range`: When a flag in the structure is set, orders two coordinate pairs into ranges and tests a shared value against them.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 1 into d4, then returns.
- `blit_masked_block_four_planes`: Draws a rectangle of words into four bitplanes, combining each word with a background source through a mask.
- `clear_block_by_longwords`: Zeroes 32000 bytes starting at a shared pointer, written as groups of four 32-bit words.
- `copy_shared_block_in_place`: Copies 32000 bytes of a shared block in groups of four 32-bit words.
- `forward_call_and_return`: Calls the routine that follows it and then returns.
- `copy_bitplane_rows`: Copies rows of data across four bitplanes from a source to a destination.
- `copy_bitplane_rows_four_planes`: Copies twenty bytes from each of 160 rows, in all four bitplanes, from a source area to a destination area with a row stride of forty bytes.
- `return_from_subroutine`: This routine performs a standard return from a subroutine by popping the return address from the stack.
- `halt_in_infinite_loop`: Loops forever with no exit, so the program never gets past this routine once it is reached.
- `store_shifted_then_wait`: Stores its input word shifted left by four bits into a shared word.
- `check_small_inputs_call_external`: When a shared flag is set, checks two input values against eight.
- `set_shared_pointer_by_flag`: Stores the address of a shared variable into a pointer slot, and replaces it with a fixed address when a flag is zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d4.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `update_global_pointer`: Sets a global variable to its own address or a specific constant address based on a flag.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 126 into d4, then merges 0 into d0.
- `modify_global_address`: Updates a global variable with an address based on the state of a flag.
- `set_conditional_pointer`: Checks if a global byte is zero and, if so, assigns a specific constant address to a global pointer; otherwise, it assigns the address of the global pointer to itself.
- `set_shared_pointer_by_flag_2`: Stores the address of a shared variable into a pointer slot and replaces it with a fixed address when a flag is zero.
- `copy_table_twice_set_bytes`: Copies twelve words from a table chosen by a shared index into a block, writing each word twice at a four-byte stride.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_two_shared_flags_on`: Sets two shared flag bytes to one and returns.
- `clear_shared_flag_byte`: Sets one shared flag byte to zero and returns.
- `mask_byte_value`: Returns a byte with its lower two bits set to one.
- `update_value_when_changed`: When the input differs from a stored value, stores the input and derives another shared value as the input times eight plus 140.
- `update_bounded_position_return_code`: Updates shared position values, clamping them to fixed limits, and returns a numeric code.
- `update_bits_and_counters`: Sets a bit in a global variable based on a register value, increments a value at a memory location, and performs a bitwise OR on a byte through a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `walk_packed_step_table`: Walks a table of packed step bytes, adding signed x and y deltas taken from each byte to a position.
- `poll_until_nonzero_count`: Calls a routine up to the count given in its argument, calling a second routine after each miss.
- `initialise_tables_and_setup_paths`: Fills a 256-entry identity table, sets several shared values to constants, and runs a long sequence of setup calls with fixed data addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `examine_timed_out_segment_entry`: The decompiler timed out, so no pseudo-code is available and the behaviour is not recovered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `forward_call_with_zero_flag`: Passes its four arguments on to the following routine with a fifth argument of zero, then returns.
- `forward_call_with_zero_fourth`: Passes its three arguments on to the following routine with a fourth argument of zero, then returns.
- `print_word_wrapped_text`: Prints a null-terminated string one character at a time, breaking lines at spaces so that each line fits a width taken from a shared table.
- `convert_lowercase_to_uppercase`: Converts a lowercase ASCII letter to uppercase by subtracting 32.
- `convert_uppercase_to_lowercase`: Converts an uppercase ASCII letter to lowercase by adding 32.
- `read_byte_field_offset`: Returns the byte at a fixed offset in the structure pointed to by the address register.
- `clear_shared_flag_then_call`: Clears a shared variable and then calls one fixed routine.
- `layout_seven_centered_rows`: Calls setup routines, then for seven entries calls a routine with a horizontal position centred on a width of 320 using a table value.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `wait_until_nonzero_then_store`: Repeatedly calls a fixed routine with an address until it returns a nonzero value, calling a second routine between tries.
- `wait_until_nonzero_then_pass`: Repeatedly calls a fixed routine with an address until it returns a nonzero value, calling a second routine between tries.
- `pass_shared_value_return_field`: Passes a shared variable to a fixed routine and returns a 32-bit field read from the structure pointed to by the address register.
- `wait_until_nonzero_then_pass_2`: Repeatedly calls a fixed routine with an address until it returns a nonzero value, calling a second routine between tries.
- `loop_until_nonzero`: Repeatedly calls another routine until it returns a non-zero value, then calls a third routine with a global variable and returns the result.
- `call_subroutine_with_global`: Calls another routine using a constant and a global variable before returning a value from a memory location.
- `wait_then_call_with_result`: Calls a routine with its first argument, then waits until a fixed routine returns a nonzero value, calling a second routine between tries.
- `pass_two_fields_return_one`: Passes two 32-bit fields of the structure pointed to by the address register to a fixed routine and returns 1.
- `forward_call_with_fixed_block`: Calls the following routine with a fixed block address, zeros for three arguments and its own first argument, then returns.
- `call_chain_with_shared_value`: Calls a routine with its argument, takes a value from a second routine and converts it through a third.
- `call_chain_return_result`: Calls a routine with its argument, then runs a chain of further calls using the returned value and a shared variable.
- `store_call_result_in_slot`: Calls a routine with its first argument, takes the returned value from a second routine and stores it in a 32-bit slot of a shared table selected by the second argument.
- `call_two_routines_with_argument`: Calls a routine with its argument and then calls a second routine, returning no value.
- `store_argument_in_shared_word`: Stores its 16-bit argument into a shared variable and returns.
- `wait_then_store_with_default`: Calls a routine, then waits until a fixed routine returns a nonzero value.
- `wait_then_store_default_pair`: Calls a routine, then waits until a fixed routine returns a nonzero value, calling a second routine between tries.
- `default_zero_field_then_call`: Sets a 32-bit field of the structure to 65535 when it is zero, then passes that field, a shared variable and three structure fields to a fixed routine.
- `return_flag_from_wait_result`: Calls a routine with its argument, then returns 1 when a fixed routine returns a nonzero value and 0 otherwise.
- `wait_convert_store_shared`: Calls a fixed routine with a data address, then waits until another fixed routine returns a nonzero value, calling a third routine between tries.
- `convert_field_store_shared`: Converts a 32-bit field of the structure through a fixed routine, stores the result in a shared variable, then calls another routine and passes the shared variable to a fixed routine.
- `nop_placeholder`: This routine is empty and performs no operations.
- `examine_timed_out_five_calls`: No pseudo-code was produced because the decompiler timed out, so the behaviour is not recovered.
- `examine_timed_out_single_call`: No pseudo-code was produced because the decompiler timed out, so the behaviour is not recovered.
- `examine_timed_out_short_entry`: No pseudo-code was produced because the decompiler timed out, so the behaviour is not recovered.
- `reset_state_by_switch`: Sets a shared state variable to 10 for four of its values, and calls a routine for two of those values as well.
- `call_routine_fixed_arguments`: Calls a routine with a fixed data address and shared values along with two constants, then returns.
- `init_five_slot_block_defaults`: Calls three fixed routines with a fixed data address and then clears two bytes and sets a third byte to 15 in each of five eight-byte slots.
- `call_fixed_routine_with_address`: Calls a fixed routine with a fixed data address, then returns.
- `call_fixed_routine_once`: Calls one fixed routine with no arguments and returns.
- `repeat_call_sequence_forever`: Repeats forever a fixed sequence of calls: one with zero, a hundred calls with a counter, and one with 289 as its value.
- `repeat_call_sequence_forever_2`: Repeats forever a fixed sequence of calls: one with 289, one with zero, and a hundred calls with a counter.
- `repeat_routine_calls_countdown`: Counts from the given value down to zero, calling one fixed unnamed routine on each step, and then calls that same routine once more.
- `store_word_in_global`: Stores its 16-bit argument into a shared global variable and returns.
- `store_word_in_global_2`: Stores its 16-bit argument into a shared global variable and returns.
- `write_four_bytes_table_entry`: Multiplies its first argument by eight to select an eight-byte table entry, then stores four byte arguments into the first four bytes of that entry in order.
- `store_two_bytes_indexed_entry`: Uses the shared global variable, scaled by eight, as an index into an eight-byte-per-entry table and stores two byte arguments into the first two bytes of that entry.
- `set_low_nibble_table_entry`: Uses the shared global variable, scaled by eight, to select a table entry, then replaces the low four bits of its third byte with the low nibble of its argument while keeping the high four bits.
- `read_first_byte_table_entry`: Returns the first byte of the table entry selected by the shared global variable, scaled by eight.
- `read_second_byte_table_entry`: Returns the second byte of the table entry selected by the shared global variable, scaled by eight.
- `assemble_file_path_name`: Builds a path string into a buffer from up to four optional parts: the first part is followed by a colon when the buffer does not already end in one, the second is preceded by a separator character from a shared global variable unless the buffer already ends in a backslash, slash or colon, the third is appended as is, and the fourth is introduced by a period.
- `append_string_to_buffer`: Scans the destination string to its terminating zero and then copies the source string, including its terminator, starting at that point.
- `divide_signed_32bit_values`: Divides a signed 32-bit dividend by a signed 32-bit divisor.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend by a 32-bit divisor.
- `multiply_32bit_words`: Multiplies two 32-bit values by forming 16-bit partial products: the two cross products of the high and low words are summed and shifted into the high half, and the low-word product is added.
- `copy_string_return_terminator`: Copies a null-terminated source string into a destination buffer, including the terminating zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_self_address_then_trap`: Stores the address of this routine into a shared global variable, then executes trap instruction zero before returning.
- `stub_return_immediately`: Contains only a return and does nothing.
- `configure_display_and_copper`: Writes display window, data fetch and bitplane control hardware registers and the interrupt enable register, and references the text ultimav.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `manage_interrupts_and_sprites`: Accesses the interrupt enable and request registers in both their read and write forms, and touches the eight sprite pointer registers.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_immediately_2`: Contains only a return and does nothing.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `empty_stub_rts`: An empty stub consisting solely of an rts instruction.

### loader (raw code file, load address unknown)

- `start_loader_code_region`: Describes a four-instruction block at the start of the loader's raw code file.

### ultimav (raw code file, load address unknown)

- `configure_display_and_ports`: Writes the bitplane modulo, display window, data fetch and bitplane control registers and the interrupt enable register, and touches the controller port register that carries the fire buttons and disk status.
- `start_copper_list_dma`: Writes the copper list pointer register, strobes the copper jump register so the list starts at once, and writes the DMA control register.
- `configure_four_audio_channels`: Writes the location and volume registers of all four audio channels and the DMA control register, and touches the controller port register that carries the fire buttons and disk status.
- `program_audio_sample_channels`: Writes the sample location and length registers of all four audio channels and the DMA control register, and touches the first custom-chip register.
- `process_undescribed_routine_block`: No pseudo-code is available for this 65-instruction routine, and no hardware registers or referenced text are recorded for it.
- `invoke_two_unnamed_routines`: Calls two unnamed routines and is itself called from two unnamed routines.
