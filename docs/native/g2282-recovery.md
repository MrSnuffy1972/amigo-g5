# Game 2282 (star wars - return of the jedi (1988)(domark)[cr qtx][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02282). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914224 top-level bytes in 36 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 1966 raw-word lines |
| Functions named and described | pass, 455 of 455 named (0 author symbols), 455 of 455 described (17 by rule, 169 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1871 of 1919 (97.50%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11749 of 11932 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 343866 |
| code | 38202 |
| data | 93832 |
| padding | 8244 |
| unused | 430080 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:run | 2 | 117 | 2 |
| ex | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:jedi | 28 | 7178 | 327 |
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:add21k | 2 | 26 | 1 |
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:wait | 2 | 117 | 2 |
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:nofastmem | 9 | 306 | 14 |
| Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:bootblock | 1 | 37 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 867 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 985 | 67 |
| logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf | 1 | 1235 | 7 |
| logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf | 1 | 43 | 2 |
| logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf | 1 | 787 | 2 |

## Cases handled

- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 605 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- star wars - return of the jedi (1988)(domark)[cr qtx][a2]/Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf: 1120 executed PCs whose RAM was overwritten before any snapshot match its bytes 287776-290480: recovered as a unit run at an address
- star wars - return of the jedi (1988)(domark)[cr qtx][a2]/Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf: 18 executed PCs whose RAM was overwritten before any snapshot match its bytes 466080-466400: recovered as a unit run at an address
- star wars - return of the jedi (1988)(domark)[cr qtx][a2]/Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf: 92 executed PCs whose RAM was overwritten before any snapshot match its bytes 288208-290352: recovered as a unit run at an address
- star wars - return of the jedi (1988)(domark)[cr qtx][a2]/Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf: 3 logged loads cover 0 bytes of the image
- star wars - return of the jedi (1988)(domark)[cr qtx][a2]/Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: Copies sixteen bytes from a source offset into a destination buffer and then calls another routine.

### ex

- `process_linked_list_and_expand_buffer`: Traverses a linked list to find the maximum value in a specific field and then expands a data buffer by writing a header and copying sixteen bytes of data from a source to a destination.
- `copy_data_block_2`: Copies sixteen bytes of data from a memory location offset from a pointer to a destination address.

### type

- `process_linked_list_and_update_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a data buffer by writing a header and copying sixteen bytes of data from a source location.
- `copy_memory_block`: Copies sixteen bytes of data from a memory location offset from a pointer to a destination address.

### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:jedi

- `apply_bit_rotated_additions`: Iterates through a nested list of data entries to perform bitwise rotations and additions on values stored in a memory buffer.
- `update_buffer_with_bit_manipulated_values`: Iterates through a list of data entries to perform bitwise rotations and additions to values stored in a memory buffer.
- `decrement_and_return`: Decrements a register value in a loop until it reaches zero.
- `copy_longwords_then_zero_fill`: Copies 32-bit values from a source region into a destination region until the destination reaches a limit, zero-fills the remainder, saves the register values into fixed globals and returns the stacked value.
- `execute_complex_logic`: A large routine that performs multiple operations including setting global variables, busy-waiting, and executing various subroutines.
- `increment_global_counter_2`: Increments a global variable by one.
- `execute_unnamed_logic`: Performs a series of operations involving setting and checking a global variable.
- `execute_complex_sequence`: Performs a long sequence of operations including setting multiple global variables, busy-waiting on a global, and executing a large memory block copy.
- `call_three_subroutines_2`: Sets a global variable to zero multiple times, sets it to three, and then calls three other routines in sequence.
- `wait_until_global_reaches_four`: Spins until a shared global reaches at least four, then resets it to zero and returns.
- `wait_for_status_bit`: Decrements a value in a loop while waiting for the high bit of a global variable to clear.
- `wait_status_bit_clear`: Waits while the high bit of a status global is set, then returns.
- `initialize_global_state`: Sets a global variable to zero or a specific constant and enters a loop that checks for non-zero values.
- `execute_conditional_loop`: Checks a global variable and either enters a loop that decrements a counter or a loop that increments a counter.
- `increment_and_loop`: Increments a global variable in a loop until it reaches a specific negative threshold.
- `recurse_and_loop`: Recursively calls itself and then enters a loop that executes a sequence of other routines a variable number of times.
- `execute_recursive_string_traversal_2`: Recursively calls itself multiple times before decrementing a counter in a loop that traverses a sequence of calls based on a value starting at eighty.
- `update_global_flag`: Sets a global variable to a specific value and enters a loop that performs conditional checks and subroutine calls.
- `match_table_entry`: Performs a loop that compares values and modifies a global variable through several conditional branches.
- `check_condition_and_return`: Compares register values against constants and returns if a specific condition is met.
- `process_state_loop`: Executes a complex loop that modifies a global variable and calls various subroutines based on its value.
- `decrement_and_validate_global_counter`: Decrements a global variable through a series of conditional checks and branches.
- `process_global_loop_variant`: Executes a complex loop involving conditional branches and modifications to a global variable.
- `process_complex_loop`: Enters a loop that modifies a global variable and performs multiple conditional calls and branches based on that variable's value.
- `process_global_loop`: Enters a complex loop that modifies a global variable and performs multiple conditional calls and branches based on that variable's value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `advance_counter_table_lookup`: Increments a shared counter and replaces it with a byte from a small table indexed by bits of the counter.
- `iterate_string_from_global`: Adds the character zero to a global variable if it is zero, then iterates through a null-terminated string starting at the address of that global variable.
- `increment_and_iterate_string`: Increments a global byte by the character zero and then iterates through a null-terminated string.
- `iterate_fixed_string`: Iterates through a null-terminated string located at a fixed memory address.
- `iterate_offset_string`: Performs two initial routine calls and then iterates through a null-terminated string at a fixed memory address.
- `dispatch_and_set_constant`: Checks a global variable for specific byte values to determine which routine to call, then sets a global variable to all ones.
- `set_global_minus_one_2`: Checks a byte for a specific value and then sets a global variable to all ones.
- `execute_recursive_string_traversal`: Calls itself twice before traversing a null-terminated string and calling itself for every non-zero byte encountered.
- `spin_on_non_zero`: Enters an infinite loop if a global variable is not zero.
- `spin_forever_after_wait`: Waits until a shared flag reads zero and then spins in an endless loop without returning.
- `call_with_saved_global_value`: Writes its input into a shared global and makes two calls, keeping the global's value from after the first call and restoring it at the end.
- `copy_memory_block_2`: Copies a block of data from a source pointer to a destination pointer using a loop.
- `set_global_to_max`: Sets a global variable to a specific maximum value.
- `call_sequence_with_branch`: Performs a conditional branch if a global variable matches a tab character, followed by a sequence of eleven routine calls.
- `make_eleven_calls`: Makes eleven calls to other routines in sequence and returns.
- `make_seven_calls`: Makes seven calls to other routines in sequence and returns.
- `make_long_call_sequence`: Makes a long fixed sequence of calls to other routines and then returns.
- `copy_block_after_ten_calls`: Makes ten calls, then copies a 32000-byte block of memory from one buffer to another in 4000 double-longword steps.
- `save_global_around_two_calls`: Makes one call, saves a shared global, makes two calls, then restores the saved global value.
- `iterate_string_pointer`: Iterates through a null-terminated string using a pointer.
- `nop_padding_routine`: This routine is empty and performs no operations.
- `decrement_local_variable`: Decrements a local variable in a loop until it returns to its original value.
- `clear_and_copy_strided_block`: Clears an 8000-longword block of memory, makes one call, then makes a run of calls.
- `clear_block_and_make_calls`: Clears an 8000-longword block of memory, makes one call and then a long series of calls.
- `iterate_null_terminated_string`: Traverses a null-terminated string by checking each byte and calling another routine for every non-zero character encountered.
- `return_nothing`: This routine is empty and performs no operations before returning.
- `copy_self_block_then_calls`: Makes two calls, runs a 10752-longword copy loop whose decompiled source and destination are the same global so that, as shown, it changes nothing, then makes eleven more calls.
- `fill_buffer_pattern`: Fills a memory buffer with a repeating sequence of four longwords.
- `composite_masked_plane_rows`: Combines source and destination over a block of rows: for each pair of bytes the destination becomes the source OR the destination bits selected by a fourth mask plane.
- `poll_until_zero_twice`: Calls a routine repeatedly until it returns zero, then repeats that polling once more before returning.
- `execute_iterative_calls`: Calls another routine repeatedly until it returns a non-zero value, then repeats the process once more.
- `copy_four_buffer_longwords`: Copies a run of longwords into four buffers spaced 10752 bytes apart.
- `deinterleave_four_planes`: Splits each group of four source bytes across four destination buffers spaced 10752 bytes apart.
- `copy_planes_between_strides`: Makes two calls, then copies 192 rows of ten-longword groups across four planes, reading them 8000 bytes apart and writing them 10752 bytes apart.
- `blit_planar_tile_data`: Iterates through a lookup table to copy longword groups from a source buffer into four separate plane offsets within a destination buffer.
- `copy_tiles_to_buffers`: Iterates through a lookup table to copy sixteen-longword groups from a source buffer into four plane offsets within a destination buffer.
- `copy_tiles_from_fixed_table`: Uses one fixed lookup table, then for each nonzero entry copies a sixteen-longword group from a source buffer across four plane offsets.
- `clear_rows_of_longwords`: Zeroes 192 rows of 56 longwords each, advancing a buffer pointer continuously through memory.
- `execute_iterative_calls_2`: Makes two calls, then repeatedly calls another routine until it returns zero, repeating this entire process twice.
- `swap_plane_bytes_via_table`: Exchanges bytes between two regions across four planes, passing each byte through a small lookup table before it is stored.
- `copy_planar_block_rows`: Copies 108 rows of 24 bytes in each of four planes from a fixed source to a destination, reading the planes 8000 bytes apart and writing them 10752 bytes apart.
- `build_bit_reverse_table`: Fills a 256-byte table so that each entry holds the bit-reversed value of its index.
- `dispatch_table_actions`: Updates a global variable from a table, clears specific bits in a memory range, and then executes routines from a jump table based on values read from a buffer.
- `dispatch_five_table_actions`: For each of five bytes from a table selected by a shared index, calls the routine in a jump table that matches the byte's high bits, skipping bytes whose low nibble is zero.
- `execute_state_machine_opcode`: Reads a byte from an offset determined by a global state variable and updates that variable based on the byte's high nibble.
- `process_state_opcode_machine`: Updates a global state variable by reading a byte from an offset determined by the current state and evaluating its high nibble as an opcode.
- `update_state_via_opcode`: Updates a global state variable by reading a byte from an offset determined by the current state and evaluating its high nibble as an opcode.
- `handle_input_and_records`: Processes an input value to either update a global variable via a table lookup or scan a table to copy eighteen words from a fixed source to a found entry.
- `dispatch_input_actions`: Handles different input values by updating global variables, modifying memory, and copying word blocks from a fixed source.
- `manage_state_and_records`: Updates a global variable and performs conditional logic to either modify a global variable or scan a table to copy eighteen words from a fixed source to a found entry.
- `update_state_and_copy_2`: Updates a global variable based on input and copies a sequence of words from a fixed source to a destination found via a table search.
- `set_global_flag_to_one`: Sets a global variable to one.
- `set_global_flag`: Sets a global variable to one.
- `call_adjust_state_then_call`: Makes one call, adds sixty-four to a shared state, sets the state to three and makes a second call.
- `call_add_offset_then_call`: Makes one call, adds 192 to a global variable, sets that variable to two, and makes a second call.
- `find_and_fill_record`: Scans a table for the first entry with a clear low flag bit and copies eighteen words from a fixed source to that entry.
- `nop_noop_sequence`: This routine is empty and performs no functional operations.
- `set_state_one_checked`: Sets a shared state to one, then runs a conditional that cannot succeed in the decompiled logic, so the state stays at one.
- `reset_state_to_two`: If a shared state is zero it makes one call, then sets the state to two.
- `set_state_one_after_call`: If a shared state is nonzero it sets it to one and to 256 and makes one call, then sets the state to one.
- `load_state_from_table_entry`: Loads a shared state from a table entry selected by the current state value, with the index scaled by four.
- `copy_global_to_free_record`: Scans a table of nineteen-word records for the first record with a clear low flag bit and copies a global variable into it.
- `initialize_and_fill_record_3`: Sets a global variable to zero, performs several operations, and then copies eighteen words from a fixed source to the first available record in a table.
- `initialize_and_fill_record`: Clears a block of memory, performs several operations that modify a global variable, and if a specific value is met, copies eighteen words from a fixed location into the first available record in a table.
- `return_immediately`: An empty routine that performs no operations and returns.
- `update_table_index`: Modifies a global variable through bitwise operations and uses it to fetch a new value from a table.
- `cycle_table_index_2`: Modifies a global variable using a bitwise operation and updates it with a value from a table.
- `cycle_table_index`: Increments a global variable within a specific range and updates it with a value from a table.
- `cycle_table_index_and_recurse`: Increments a global variable within a three-step cycle and updates it with a value from a table.
- `cycle_three_entries`: Increments a global variable within a three-entry cycle and loads a value from a table based on that index.
- `cycle_three_entries_always`: Steps a three-entry cycle, loads the selected entry and makes one call.
- `find_and_fill_free_record`: Iterates through a table to find the first entry where the low bit of the first word is clear, then copies eighteen words from a fixed memory location into that entry.
- `copy_data_to_free_record`: Scans a table for the first record with a clear low flag bit and copies eighteen words from a fixed memory location into it.
- `copy_fixed_data_to_free_record`: Scans a table of nineteen-word records for the first record with a clear low flag bit and copies data from a fixed memory location into it.
- `process_and_fill_record`: Performs several operations on a global variable and, if a specific value is met, copies eighteen words from a fixed source to the first available record in a table.
- `cycle_and_fill_record`: Checks a global variable against a range, updates it using a table, and copies eighteen words from a fixed source to the first available record in a table.
- `cycle_and_copy_data_2`: Increments a global variable in a three-entry cycle, loads a value from a table, and copies eighteen words from a fixed memory location into a free record.
- `cycle_and_copy_data`: Increments a global variable in a three-entry cycle, loads a value from a table, and copies eighteen words from a fixed memory location into the first available record in a table.
- `update_state_and_copy_data`: Increments a state variable within a three-entry cycle, fetches a value from a table, and copies data from a fixed memory location into the first free record found in a table.
- `initialize_free_record`: Locates the first available entry in a table by searching for a record with a cleared low bit and a non-negative sentinel.
- `execute_fill_sequence`: Performs a series of operations on a global variable and then copies eighteen words from a fixed source to the first available record in a table.
- `initialize_and_fill_record_2`: Performs several operations that modify a global variable and, if a specific value is met, copies eighteen words from a fixed memory location into a free record.
- `copy_input_to_record_2`: Sets a global variable to a sequence of values, calls another routine, and then copies eighteen words from a fixed memory location into a free record.
- `copy_input_to_record`: Copies a value from a register into a global variable and then calls another routine.
- `copy_input_to_free_record`: Copies a value from a register into a global variable, calls another routine, and then copies data from a fixed memory location into the first free record found in a table.
- `toggle_global_and_copy`: Toggles a global variable between zero and one, updates several other global variables, and performs a memory copy from a table to a destination pointed to by a pointer.
- `update_state_and_copy`: Toggles a global variable between zero and one and performs several updates to other global variables before calling another routine.
- `copy_record_from_table`: Finds the first available record in a table by checking a flag bit and then copies eighteen words from a source table into that record.
- `dispatch_state_actions`: Updates a global variable and conditionally calls other routines or writes values to two specific memory locations based on that variable's value.
- `call_when_input_is_three`: Checks if the input value in a register is equal to three and calls another routine if it is.
- `branch_on_state_2`: Checks a global variable and sets it to various specific values before making multiple conditional calls.
- `branch_on_state`: Checks a global variable and a register to set a global variable to various values before calling other routines.
- `set_global_and_call`: Sets a global variable to a specific value and calls another routine, then conditionally resets that variable to zero and calls another routine.
- `branch_on_registers`: Checks the values of two input registers and sets a global variable to one of several specific values before conditionally calling another routine.
- `set_negative_global_and_call`: Sets a global variable to a negative value and calls another routine, then conditionally resets that variable to zero and calls another routine.
- `conditional_set_global_and_call`: Checks a register and either sets a global variable to a negative value or sets it to a positive value and performs a conditional second call.
- `set_forty_then_wrap_call`: Sets a global variable to forty and makes a call, then resets the variable to zero and makes a second call if the value exceeds four.
- `call_add_packed_bcd_and_dispatch`: Calls a routine that adds a packed decimal value to a global variable and then dispatches several calls.
- `forward_single_call`: A two-instruction wrapper that makes one call to another unnamed routine and then returns.
- `execute_empty_call`: This routine is empty and performs no operations before returning to its caller.
- `call_dispatch_routine`: This routine acts as a wrapper that calls another routine and then returns.
- `call_packed_bcd_addition_routine`: This routine acts as a wrapper that calls a procedure to add a packed BCD value to a global variable and then executes a dispatch sequence.
- `store_global_and_call_twice`: Stores a fixed constant into a global variable and then calls an unnamed routine twice before returning.
- `invoke_two_subroutines`: Calls two unnamed routines in sequence and returns.
- `mask_value_or_delegate`: Sets the bits selected by a fixed mask in the input register value.
- `call_three_subroutines`: Calls three unnamed routines in sequence and then returns.
- `step_counter_and_copy_bitplanes`: Increments a global counter up to a fixed limit, then copies 16-row blocks of source data into four bitplanes spaced a full screen apart.
- `copy_rows_to_bitplanes`: Copies 16-row blocks of fixed source data into four bitplanes spaced a full screen apart, using the current global counter value to set how many rows are written.
- `increment_global_counter`: Adds one to a global variable and returns.
- `add_bcd_and_dispatch`: Adds three to a global value using packed decimal adjustment, then calls unnamed routines six times in total.
- `call_six_subroutines`: Calls six unnamed routines in sequence and then returns.
- `call_six_subroutines_2`: Executes six calls to another routine in sequence.
- `call_five_subroutines`: Calls five unnamed routines in sequence and then returns.
- `call_four_subroutines`: Calls four unnamed routines in sequence and then returns.
- `call_three_subroutines_3`: Executes three calls to another routine in sequence.
- `call_two_subroutines`: Calls two unnamed routines in sequence and then returns.
- `forward_single_call_2`: A short wrapper that makes one call to another unnamed routine and then returns.
- `add_packed_bcd_and_dispatch`: Adds a three-byte packed decimal value into a global variable with carry handling, then calls unnamed routines seven times.
- `execute_six_sequential_calls`: Performs six consecutive calls to the same subroutine.
- `execute_sequential_calls_3`: Performs six consecutive calls to another routine.
- `execute_sequential_calls_2`: Performs five consecutive calls to other routines.
- `execute_sequential_calls`: Executes four consecutive calls to other routines.
- `call_subroutines_sequentially`: Executes three consecutive calls to other routines.
- `copy_row_block_wrapper`: This routine acts as a wrapper that invokes two subroutines to copy twelve rows of four-bitplane words from a source offset to a destination pointer.
- `call_row_copy_routine`: This routine serves as a wrapper that calls a procedure to copy twelve rows of four-bitplane words from a source offset to a destination pointer.
- `repeat_paired_calls_loop`: After a guard comparison, repeatedly calls an unnamed routine twice per pass until a global value test fails.
- `repeat_paired_calls_loop_2`: Repeatedly calls an unnamed routine twice per pass, looping while a global value test holds.
- `copy_rows_from_offset_source`: Copies twelve rows of four bitplane words from a source block selected by the input offset into the destination pointer, stepping forty bytes per row.
- `step_grid_and_call_routine`: Advances a cycling index and looks up a value in a four-entry table.
- `align_and_walk_position_grid`: Aligns the input horizontal position down to a multiple of eight and calls an unnamed routine.
- `walk_grid_from_column`: Advances a cycling index and reads a four-entry table, then walks the input x position in steps of sixteen and y in steps of eight, calling an unnamed routine at each stop.
- `walk_grid_positions_xy`: Walks x positions from the input register in steps of sixteen and y positions from a second register in steps of eight, storing each pair in scratch words and calling an unnamed routine at each stop.
- `stub_return_only`: A stub that returns immediately without doing any work.
- `dispatch_matching_table_records`: Scans a table of nineteen-word records from a fixed start.
- `scan_records_by_countdown`: Calls an unnamed routine and then scans the nineteen-word record table for active entries whose index field matches a countdown value passed on the stack.
- `scan_records_for_each_count`: For each value counting down from the input value, scans the nineteen-word record table and calls an unnamed routine for every active entry whose index matches.
- `step_field_and_call`: Saves a 16-bit field of the structure, then ten times calls an unnamed routine and advances the field by sixteen.
- `draw_shifted_masked_sprite`: Clips a sprite's position against the screen edges and draws it into four bitplanes.
- `compute_and_draw_sprite_struct`: Reads a sprite structure, computes its clipped screen position and stores the result back into the structure.
- `draw_sprite_struct_at_position`: Reads a sprite structure's stored clipped position and draws it into four bitplanes at a word-aligned offset.
- `draw_shifted_sprite_unmasked`: Clips a sprite given its position and size in registers, then draws it into four bitplanes with a sub-word bit shift.
- `draw_unmasked_sprite_rows`: Clips a sprite passed in registers and copies its rows into four bitplanes, shifting each word by the sub-word offset.
- `draw_shifted_sprite_skipping_rows`: Skips a number of leading sprite rows taken from a register, then draws the rest into four bitplanes with a sub-word bit shift.
- `draw_masked_sprite_skipping_rows`: Skips a number of leading sprite rows taken from a register, then draws the rest into four bitplanes at a word-aligned offset.
- `draw_shifted_sprite_row_clip`: Checks the visible row range of a sprite and skips the clipped top rows, then draws the visible rows into four bitplanes with a sub-word bit shift and no transparency mask.
- `dispatch_sprite_shift_draw`: Clips a sprite and then selects one of several shifted or aligned drawing loops, based on the sub-word shift and the mask flag.
- `draw_shifted_sprite_rows`: Draws a fixed number of sprite rows into four bitplanes with a sub-word bit shift taken from a register.
- `copy_masked_sprite_rows`: Copies rows of sprite data into four bitplanes without any shift.
- `draw_masked_sprite_column`: Draws sprite rows into four bitplanes column by column.
- `draw_byte_masked_sprite_rows`: Draws sprite rows byte by byte into four bitplanes.
- `draw_shifted_masked_sprite_columns`: Draws sprite columns into four bitplanes with a bit shift taken from a register.
- `draw_complement_shift_sprite`: Draws sprite rows into four bitplanes with a bit shift computed as the complement of the sub-word offset.
- `copy_sprite_rows_stride`: Copies two columns of sprite words into four bitplanes.
- `copy_sprite_columns_stride`: Copies sprite columns into four bitplanes.
- `draw_masked_sprite_rows_clear`: Draws sprite rows into four bitplanes column by column.
- `draw_masked_sprite_bytes`: Draws masked sprite rows byte by byte into four bitplanes.
- `draw_shifted_sprite_mask_fill`: Draws sprite rows into four bitplanes with a bit shift taken from a register.
- `draw_complement_shift_fill`: Draws sprite rows into four bitplanes with a bit shift computed from the complement of the input offset.
- `call_two_then_library_vector`: Calls two unnamed routines and then calls a library function through a vector loaded from a global base pointer at offset minus 36.
- `call_via_offset_pointer`: Calls a routine using a function pointer stored at an offset from a global variable.
- `call_six_then_library_vector`: Calls six unnamed routines and then calls a library function through a vector loaded from a global base pointer at offset minus 36.
- `decode_marker_run_length`: Unpacks a run-length encoded word stream into a destination buffer.
- `call_five_then_library_vector`: Calls five unnamed routines and then calls a library function through a vector loaded from a global base pointer at offset minus 36.
- `call_three_then_library_vector`: Calls three unnamed routines and then calls a library function through a vector loaded from a global base pointer at offset minus 36.
- `poll_library_functions`: Repeatedly calls a routine until a condition is met, then executes a function via a pointer stored in a global variable.
- `set_flag_and_decode_runs`: Sets a global value to one, then decodes a run-length encoded word stream into the destination.
- `set_flag_and_decode_runs_2`: Sets a global value to two, then decodes a run-length encoded word stream into the destination.
- `set_flag_and_decode_runs_3`: Sets a global value to three, then decodes a run-length encoded word stream into the destination.
- `decode_runs_after_setup_calls`: Calls three unnamed routines, then decodes a run-length encoded word stream into the destination.
- `fill_then_decode_runs`: Writes a repeated word into the destination for a counted length, then decodes a run-length encoded word stream.
- `write_word_then_decode_runs`: Writes one word into the destination and then decodes a run-length encoded word stream.
- `null_operation_stub`: This routine is empty and performs no actions.
- `init_audio_channels_silent`: Sets all four audio channels to the same sample pointer, a sixteen-word length, a period of 400 and zero volume, then disables the four audio interrupt bits in the interrupt enable register.
- `wait_until_flag_cleared`: Sets a global flag to one, calls an unnamed routine, then busy-waits until the flag reads zero.
- `disable_audio_interrupt_bits`: Writes the interrupt enable register with the four audio interrupt bits set and the enable bit clear, which disables them.
- `disable_audio_interrupts_if_idle`: If a global state value is zero, sets it through a short sequence and disables the four audio interrupt bits.
- `return_from_subroutine`: This is an empty routine that performs no operations before returning.
- `update_audio_state`: updates a global variable multiple times and writes a value to a specific memory location.
- `modify_audio_parameters`: updates a global variable with different values and writes a value to a specific memory location.
- `disable_audio_interrupt_bits_2`: Sets a global state value, writes the interrupt enable register to disable the four audio interrupt bits, then sets the state value again.
- `store_value_then_call_five`: Stores the input register value into a global variable, then calls five unnamed routines in sequence.
- `setup_exec_memory_or_halt`: Clears a block of low memory up to a global variable, finds the current task and marks it, then opens a system library and allocates memory through it.
- `install_interrupt_server_loop`: Calls the system library function that adds an interrupt server through the library base at a fixed offset, then never returns.
- `init_custom_display_and_dma`: Initialises the custom display and sound chips at startup.
- `build_bitplane_pointer_table`: Stores four bitplane pointers, each one screen-size apart, into a small table.
- `poll_library_functions_2`: repeatedly polls a library function until ready, then calls two different library functions.
- `poll_library_until_ready`: Calls a library function repeatedly until it returns a non-zero result, storing that result in a global variable.
- `store_input_value_global`: Stores the input register value into a global variable and returns.
- `store_library_result_or_halt`: Calls a library function through a base pointer at offset minus 30 and stores its non-zero result in a global variable.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_fallback_then_halt`: Calls a routine through a global function pointer and then loops forever, so it never returns.
- `cycle_index_and_call_three`: Advances a global index, wrapping it to zero when it reaches one, and calls through a global function pointer.
- `clear_global_variable`: Writes zero to the same global variable four times and returns.
- `compute_status_flags_global`: Builds status flag bits in a global variable through a chain of comparisons, returning early when the variable is already non-zero.
- `compute_status_flags_global_2`: Builds status flag bits in a global variable through a chain of comparisons, with the same flag steps as its neighbour but without the early return.
- `compute_status_bits_register`: Builds status flag bits in a register from comparisons of a global variable, then stores the result if it is zero.
- `compute_status_bits_partial`: Applies the status flag steps to a global variable when it is zero, then calls one unnamed routine.
- `compute_status_bits_global`: Builds status flag bits in a global variable through a chain of comparisons.
- `compute_status_and_clamp`: Builds status bits from a global value and stores them back.
- `compute_status_from_input_clamp`: Builds status bits from an input value and a global, stores the result, and then steps the global toward zero by ten with clamping at zero.
- `step_status_toward_zero`: Builds status bits from an input value and a global, stores the result in the global, and steps it toward zero by ten twice with clamping at zero.
- `subtract_ten_clamped_global`: Stores the input value less ten into a global variable, clamping at zero, then steps it toward zero by ten again.
- `store_input_minus_ten`: Stores the input value less ten into a global variable, clamping at zero.
- `decode_joystick_direction_bits`: Clears a global variable and then tests its bits to build a direction value, setting the high bit at the end.
- `pack_joystick_bits_to_global`: Packs bits from an input value and the global status byte into a combined direction value and stores it in the global variable.
- `pack_button_state_bits`: Packs one input bit into bit zero of a button state value and sets the high bit when the global variable's high bit is clear.
- `compute_port_movement_delta`: Computes a movement delta from bit tests on a fixed mask and the global variable's previous value, then updates the global with a clamped subtraction.
- `compute_button_delta_clamped`: Computes a button-state delta from fixed-mask bit tests and updates the global variable with a clamped subtraction.
- `compute_mouse_delta_clamped`: Computes a mouse movement delta from the global variable and updates it with a clamped subtraction.
- `store_max_then_subtract`: Stores the larger of an input value and the global variable into the global, then subtracts a second input value and stores the result only when it does not wrap.
- `store_larger_value_global`: Stores the larger of an input value and the global variable into the global variable and returns.
- `set_global_minus_one`: Sets a global variable to all ones, which reads as minus one, and returns.
- `wait_until_global_zero`: Busy-waits until a global variable reads zero, then returns.
- `update_nibble_flag_bits`: Returns early when the low nibble of the input already matches the global variable.
- `store_nibble_flag_bits`: Stores the input value in the global variable, then sets or clears bit six for each of the four low input bits.
- `update_bit_flags_by_mask`: Clears the bit in the global variable selected by the low three bits of a second input, then sets or clears that bit according to several input bit tests.
- `update_bit_flags_partial`: Clears the bit selected by the low three bits of a second input in the global variable, then sets or clears it according to bits two and three of the first input.
- `update_single_bit_flag`: Clears the bit selected by the low three bits of a second input in the global variable, then sets or clears it according to bit three of the first input.
- `clear_selected_flag_bit`: Clears the bit in the global variable selected by the low three bits of the input register.
- `clear_pixel_in_bitplanes`: Computes a byte address from a row and column pair, then clears one pixel bit in four bitplanes spaced a screen apart.
- `clear_bit_in_bitplanes`: Clears one bit, selected by the low three bits of a register, in the same byte of four bitplanes spaced a screen apart.
- `fill_bitplane_with_pattern`: Calls four unnamed routines, then fills 200 rows of the buffer working backwards from its end with a repeating four-longword pattern read from fixed memory locations.
- `fill_rows_with_pattern`: Fills 200 rows of the buffer working backwards from its end with a repeating four-longword pattern read from fixed memory locations.
- `call_single_routine`: calls one runtime routine and returns.
- `copy_eight_longwords`: Copies eight longwords from the source pointer into the destination pointer held in the global variable, then returns.
- `copy_eight_longwords_global`: Copies eight longwords from one pointer to another, with both pointers taken from the global variable, stepping both forward together.
- `forward_single_call_3`: A short wrapper that makes one call to another unnamed routine and then returns.
- `call_five_then_wait_blitter`: Calls five unnamed routines, then busy-waits until the blitter reports it is no longer busy, then calls one more unnamed routine.
- `setup_blitter_copy_job`: Waits until the blitter is idle, then programs its source and destination pointers, modulo values, first and last word masks, shift and control registers, and the blit size.
- `forbid_then_library_call`: Disables multitasking through the system library's forbid function, then calls another library function at offset minus 132 and returns.
- `permit_then_library_call`: Re-enables multitasking through the system library's permit function, then calls another library function at offset minus 138 and returns.
- `clear_bitplane_column_blocks`: Calls one unnamed routine, then zeroes two blocks of longword columns in four bitplanes spaced a screen apart.
- `clear_bitplane_rows_block`: Zeroes twenty-four rows of a single word column in four bitplanes spaced a screen apart, starting at a fixed offset into the buffer.
- `set_global_to_four`: Writes a short sequence of values to a global variable, ending with four.
- `set_global_to_eight`: Writes a short sequence of values to a global variable, ending with eight.
- `set_global_to_minus_eight`: Writes zero and then the value minus eight to a global variable, then returns.
- `compute_row_offset_lookup`: Takes an input value modulo 192, derives a remainder and padding adjustment, selects one of two entries in a fixed lookup table, and stores a scaled byte offset in the global variable.
- `advance_row_offset_lookup`: Doubles a global value and resets it to a fixed pattern when its top bit is set.
- `zero_global_variable`: Writes zero to a global variable and returns.
- `compute_row_offset_from_global`: Takes the global variable modulo 192, derives a remainder and padding adjustment, selects one of two lookup entries, and stores a scaled byte offset back in the global variable.
- `compute_table_offset_from_registers`: Looks up an entry in a fixed table using one register value, then stores a combined offset built from the other two input registers and a fixed scale factor of fifty-six.
- `call_twice_if_flag_set`: Calls an unnamed routine when a global variable is non-zero, then repeats the same test and call once more.
- `wait_blitter_then_finish_b`: calls five runtime routines, waits until the blitter busy flag clears, then calls one more routine.
- `setup_blitter_fill_job`: Waits until the blitter is idle, then programs its pointers, masks and modulo values.
- `compute_screen_pixel_offset`: Computes a linear pixel offset from an x and a y input, using a row length of 320 pixels, and stores the result in the global variable.
- `copy_table_bytes_down_column`: Copies eight bytes from a fixed table, chosen by the input value, down a column of the buffer with a stride of forty bytes.
- `advance_and_scale_offset`: Calls two unnamed routines, adds one to the global value, then scales it by 322 and stores the result in the global variable.
- `scale_global_value`: increments a global variable and then scales it using a series of arithmetic operations.
- `reduce_value_into_band`: Reduces the input value into a 32-wide band by subtracting 32, 64 or 96 according to the range it falls in.
- `subtract_band_offset_signed`: Reduces the input value by 32, 64 or 96 according to its range, without passing small values through unchanged.
- `subtract_sixty_four`: Returns the input value minus 64.
- `subtract_thirty_two`: Returns the input value minus 32.
- `copy_eight_rows_bitplanes`: Copies eight rows of four bitplanes from a source pointer to a destination pointer, each row forty bytes apart, stepping both pointers forward together.
- `overlay_masked_rows_bitplanes`: Overlays eight rows of source data onto four bitplanes.
- `set_colour_register_entry`: Writes a 16-bit colour value into the hardware palette register selected by the index input.
- `clear_low_memory_block`: Clears a block of low memory word by word from address zero, then writes a marker word just after the cleared range.
- `fill_first_free_record`: Scans a table of nineteen-word records from the start for the first record whose low flag bit is clear, stopping at a negative sentinel.
- `fill_next_free_record`: Scans nineteen-word records forward from the given record pointer for one whose low flag bit is clear, stopping at a negative sentinel.
- `tiny_movea_rts`: Small routine of 2 instructions that copies 0 to a0, then returns.
- `invoke_flagged_record_handlers`: Walks the nineteen-word record table from the start until a negative sentinel.
- `return_void`: The routine is empty and performs no operations before returning.
- `tiny_move_rts`: Small routine of 2 instructions that copies 0 to (a0), then returns.
- `create_clamped_table_record`: Builds a nineteen-word record from a source template.
- `store_clamped_record_slot`: Clamps an input value to at most sixteen and stores it in a record field, then copies the nineteen-word record from the source into the first free slot of the table.
- `write_enables_after_call`: Sets a global state value, calls an unnamed routine, then writes two values to the interrupt enable and DMA control registers at fixed offsets from a base pointer.
- `write_enables_after_call_2`: Calls an unnamed routine, then writes two values to the interrupt enable and DMA control registers at fixed offsets from a base pointer.
- `call_then_write_enable_registers`: Calls an unnamed routine, then writes two values to the interrupt enable and DMA control registers at fixed offsets from a base pointer.
- `start_audio_channel_playback`: Starts playback on one of the four audio channels, selected by an input index.
- `call_if_flag_clear`: Calls an unnamed routine only when a global variable is zero, then returns the input register pair unchanged.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning to the caller.
- `return_register_pair_2`: Checks a global variable and returns the values of two registers.
- `perform_no_operation`: The routine is empty and performs no actions before returning.
- `call_and_return_high_longword`: Calls two unnamed routines and keeps the upper half of the second result.
- `return_register_pair`: Calls another routine and returns the original values of the input register pair unchanged.
- `call_and_return_registers`: Calls an unnamed routine and then returns the input register pair unchanged.
- `search_sorted_threshold_table`: Walks a table of three-byte threshold entries, comparing a global value against each one and calling an unnamed routine on each step past the first entry.
- `format_padded_text_field`: Fills a field of twenty-four bytes with spaces, writes a sign character chosen by comparing a global value with nine, then calls an unnamed routine to format the rest of the field.
- `search_threshold_table_strided`: Walks a table of threshold entries in fixed steps, comparing a global value against each entry and calling an unnamed routine on each step past the first.
- `search_threshold_table`: Walks a table of three-byte threshold entries, comparing a global value against each one and calling another routine on each step past the threshold.
- `clear_global_variable_2`: Sets a global variable to zero.
- `format_padded_text_field_2`: Fills a field of twenty-three spaces after a three-byte header, writes a sign character chosen by comparing a global value with nine, then calls an unnamed routine to format the rest of the field.
- `write_minus_then_call_six`: Writes a minus character into a text field, stores a pointer just past it in a global variable, then calls six unnamed routines.
- `write_decimal_digit`: Writes one ASCII digit.
- `write_zero_if_padded`: Writes an ASCII zero character when a width flag is non-zero, and leaves the position unchanged otherwise.
- `copy_text_field_with_header`: Copies a three-byte header into the destination, then copies twenty-six further bytes from the source pointer into the field after it.
- `draw_dotted_text_leader`: If a text pointer is set, writes an underscore and thirteen periods, fills dots and digits across up to fifteen positions while calling an unnamed routine, then blanks out the underscores and dots and calls two routines.
- `draw_dotted_text_leader_2`: Writes an underscore and thirteen periods, fills dots and digits across up to fifteen positions while calling an unnamed routine, then blanks out the underscores and dots and calls two routines.
- `write_char_with_cursor`: Writes a character into a text buffer at the given index, advances a position counter and places an underscore cursor after it.
- `blank_leader_markers`: Replaces underscore and dot characters with spaces across fifteen buffer positions, calls two unnamed routines, then resets the position counter to zero.
- `store_library_result_then_call`: Calls a library function at offset minus 30 and, if it returns a non-zero result, stores that result, calls an unnamed routine and calls another library function at offset minus 36.
- `nop_padding_routine_2`: An empty routine that performs no operations and returns immediately.
- `initialize_library_pointers`: Loads a library base from a global variable and calls two functions at specific offsets from that base.
- `scan_region_for_nonzero_entry`: Repeatedly scans a fixed byte region for non-zero entries, tracking the position of the last one found in a global pointer.
- `lookup_region_entry_value`: Scans a fixed byte region for non-zero entries and returns a byte taken from a fixed offset within the entry found, clamped to be non-negative.
- `scan_region_entries_status`: Scans a fixed byte region for non-zero entries, updating a global pointer as it goes.
- `scan_region_entries_restart`: Scans the fixed byte region for non-zero entries, with a restart flag that resets the scan position.
- `match_string_in_table`: Compares a stored key against ten fixed-width entries in a table, one character at a time.
- `match_table_key_string`: Walks a table of fixed-width entries, comparing each stored string with a key one character at a time.
- `search_table_for_key`: Steps through fixed-width table entries, comparing each stored string with the key.

### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:add21k

- `initialize_libraries_and_cleanup`: Entry routine that opens two system libraries through the exec library, sets a flag byte in a structure reached from the opened library, then calls library functions for raster and display cleanup and closes both libraries.

### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:wait

- `find_list_max_write_header`: Walks a linked list to find the largest value stored in a field, starting from a fixed minimum.
- `copy_sixteen_longs_then_callback`: Copies sixteen 32-bit words from a fixed offset in the structure at A2 into a caller-supplied buffer when that buffer pointer is non-null, then calls through the function pointer held in A5.

### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:nofastmem

- `entry_allocate_command_control_block`: Program entry point that allocates a control block and returns error 20 if the allocation fails, otherwise it records the raw command-line pointer and length in it and looks up the current task and library bases through exec and dos calls.
- `install_alloc_vector_or_error`: Installs a replacement memory allocation vector through the system library.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `forward_saved_vector_one`: Jump-through stub that calls the routine address held in one global slot and returns to its caller.
- `forward_saved_vector_two`: Jump-through stub that calls the routine address held in a second global slot and returns to its caller.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.

### Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_state_byte_driven_loop`: Outer loop that resets a state byte and runs nested loops of routine calls, leaving the nested loop and running a final group of routines when the state byte reaches eight.
- `init_status_words_return_flags`: Initialises a block of global status words, setting a counter to 499 and clearing several fields through routine calls, then returns the result of one more routine ORed with 132.
- `wait_until_flag_reaches_four`: Busy-waits until a global counter reaches four, then resets that counter to zero.
- `wait_status_bit_with_timeout`: Counts a global down from 3500 in inner loops of 600 checks, leaving early once bit seven of that global is set, and then busy-waits until that bit clears.
- `run_display_state_machine`: Runs a setup routine, then loops over a state variable calling a status routine each pass, returning when that routine reports completion or the state reaches its terminal value.
- `run_eighty_pass_status_loop`: Runs a setup routine and, when the state byte is non-zero, a loop of eighty routine passes that returns minus one; otherwise it steps a counter and returns the state value.
- `run_state_machine_from_two`: Sets a state byte to two, calls a setup routine, then loops calling routines and returning when a status routine reports completion or the state reaches its terminal value.
- `step_state_machine_until_complete`: Repeatedly steps a state byte, calling routines each pass, and returns when a routine reports completion or the state reaches its terminal value.
- `lookup_cycling_table_value`: Increments a counter, uses bits of it to index a 32-entry byte table, and stores the table entry back, setting bit seven when the entry exceeds one.
- `print_selected_text_string`: Chooses one of two null-terminated strings by a flag, calls a setup routine, then sends the chosen string to the text output routine one character at a time, storing the character zero in the flag when it was zero.
- `print_fixed_message_string`: Sends a fixed null-terminated string to the text output routine one character at a time.
- `print_message_after_setup_call`: Calls a setup routine, then sends a fixed null-terminated string to the text output routine one character at a time.
- `finish_mode_dependent_setup`: Calls a setup routine, then for a mode byte of three or five calls a matching routine, runs a finish routine, and sets the status word to minus one.
- `print_message_after_prep_call`: Calls a preparation routine, then sends a second fixed null-terminated string to the text output routine one character at a time.
- `wait_for_state_then_halt`: Busy-waits until a state global becomes zero, then loops forever.
- `copy_32000_bytes_between_buffers`: Calls one unnamed routine, copies a 32000-byte block between two buffers in pairs of 32-bit words, then calls two more unnamed routines.
- `run_mode_dependent_call_sequence`: If the mode byte equals nine it calls two unnamed routines, otherwise one, then makes ten further calls in order.
- `run_six_call_sequence`: Calls one unnamed routine, then a second, then five further unnamed routines in sequence, with no data handling of its own.
- `invoke_long_call_sequence`: Makes a long run of calls to unnamed routines, including one call to a fixed runtime address, and returns without touching data.
- `copy_32000_bytes_after_calls`: Makes ten calls to unnamed routines, then copies a 32000-byte block between two buffers in pairs of 32-bit words.
- `busy_wait_full_short_range`: Calls one unnamed routine, spins a 16-bit counter through its full range as a delay, then calls two further unnamed routines.
- `clear_area_walk_two_tables`: Zeroes a 32000-byte area, makes a series of calls to unnamed routines including alternating pairs, then walks two constant tables over 28 outer passes of eight inner passes.
- `clear_32000_bytes_call_pairs`: Zeroes a 32000-byte area, makes fourteen calls to one unnamed routine, then eight alternating pairs of calls to two others, and restores the saved buffer pointer.
- `print_string_from_argument`: Sends the null-terminated string addressed by the argument register to the text output routine one character at a time, returning zero.
- `build_bit_reversed_byte_table`: Fills a 256-byte table so that each entry holds its index with the eight bits in reverse order.
- `update_pointer_from_table`: Updates a global variable by using its current value as an index into a table to fetch a new longword value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_saved_pointer_two_calls`: Calls two unnamed routines, then calls through the function pointer stored in a global slot.
- `call_saved_pointer_unreached_twin`: Calls two unnamed routines, then calls through the function pointer stored in a global slot; the same dispatch as its sibling and not reached in the reference run.
- `call_saved_pointer_six_calls`: Makes six calls to unnamed routines and then calls through the function pointer stored in a global slot.
- `unpack_stream_to_global_buffer`: Copies 16-bit words from a source stream to a destination buffer, and when a word carries a marker byte of 178 it expands the following word into a run of repeated words, stopping when a run record with count eight is reached.
- `call_saved_pointer_five_calls`: Makes five calls to unnamed routines and then calls through the function pointer stored in a global slot.
- `call_saved_pointer_three_calls`: Makes three calls to unnamed routines and then calls through the function pointer stored in a global slot.
- `unpack_stream_to_fixed_buffer`: Sets a mode value of one, makes setup calls, then unpacks the marker-based word stream into a fixed destination buffer.
- `unpack_word_stream_mode_two`: Sets a mode value of two, makes setup calls, then unpacks the marker-based word stream into the destination buffer held in a global.
- `unpack_stream_to_argument_buffer`: Makes three calls to unnamed routines, then unpacks the marker-based word stream from the current source pointer into the destination passed in the address register.
- `reset_four_audio_channels`: Points all four Paula audio channels at one sample, sets each length to sixteen words, period to 400 and volume to zero, then disables the audio interrupt bits.
- `set_audio_volumes_from_table`: When its enable flag is non-zero it makes four calls to unnamed routines, then sets the volumes of audio channels zero to two from three table bytes, each doubled.
- `dispatch_through_state_jump_table`: Moves a pending state byte into the active slot, clears the pending value, and calls the routine selected by the active slot from a jump table.
- `clear_channel_record_with_global`: Clears six byte fields of a channel record, sets a 16-bit field to fifteen, and clears a global variable.
- `select_level_then_advance`: Sets a level value to 32 or 63 depending on whether an input word is below 128, makes calls, and advances the level by one or two based on a table entry.
- `advance_level_with_conditional_calls`: When an input word's low bit is clear it makes four calls, then makes one call and advances the level by one, plus one more if the next level matches a table entry.
- `advance_level_or_reset`: Advances a level counter by one, first replacing it with the argument byte when the next value matches a table entry.
- `test_mask_bit_then_call`: Tests one bit of a mask table entry, selected by the low four bits of a counter, against the argument byte, and returns the result of a call when that bit is set or zero otherwise.
- `clear_channel_record_fields`: Clears six byte fields of a channel record and sets a 16-bit field to fifteen.
- `advance_counter_wrap_at_argument`: Increments a counter and resets it to zero when it reaches the byte passed in the D1 register.
- `store_table_word_in_record`: Makes two calls to unnamed routines, then stores a 16-bit constant looked up from a table by the byte in D0 into a record field.
- `store_table_word_at_offset`: Looks up a 16-bit constant from a table by the byte in D0 and stores it in the record at the offset in D6 plus ten.
- `start_channel_a_note`: When the sample byte is non-zero, stores an octave value and a period from a 48-entry table into the first voice record, sets its volume to fifteen, and steps the volume envelope down one level.
- `start_channel_b_note`: When the sample byte is non-zero, stores an octave value and a period from a 48-entry table into the second voice record, sets its volume to fifteen, and steps the volume envelope down one level.
- `start_channel_c_note`: When the sample byte is non-zero, stores an octave value and a period from a 48-entry table into the third voice record, sets its volume to fifteen, and steps the volume envelope down one level.
- `step_envelope_counter_at_offset`: Increments a byte counter in a record at an offset, and when it reaches its limit resets it and steps the envelope level down from a table.
- `store_argument_and_call_five`: Stores the longword passed in the address register into a global and then makes five calls to unnamed routines.
- `open_library_and_allocate_memory`: Clears a block of global words, opens an exec library through a vector, allocates memory, and on success sets three memory pointers from offsets into the block; on any failure it calls the failure vector and spins forever.
- `install_interrupt_server_and_hang`: Registers an interrupt server through an exec library vector and then loops forever without returning.
- `initialise_display_and_copper_registers`: Points the copper at a list, sets the display window, data fetch, bitplane modulo and playfield control registers, waits through a long delay loop, then enables the copper interrupt and the master interrupt.
- `set_sample_pointers_and_scale`: Stores four sample addresses spaced 8000 bytes apart into a channel table, copies sixteen words from the source table doubled into a second table, and stores the argument byte offset by the ASCII comma in a global.
- `invoke_four_routines`: Makes four calls to unnamed routines and returns.
- `retry_library_open_with_delay`: Repeatedly calls a library function with the value 1005 until it returns non-zero, and on each failure calls a second library function, adds 256 to a global and spins through a million-count delay before retrying.
- `call_library_or_halt`: Calls a library function and stores a non-zero result in a global, and on a zero result calls the failure vector and loops forever.
- `forward_library_call_minus_42`: Forwards to a library entry at a fixed negative offset of 42 from a global library base.
- `forward_library_call_minus_48`: Forwards to a library entry at a fixed negative offset of 48 from a global library base; not reached in the reference run.
- `forward_library_call_minus_36`: Forwards to a library entry at a fixed negative offset of 36 from a global library base.
- `cycle_handler_and_reset_flag`: Advances a selector that wraps from one to zero, calls the routine address it points to, makes three calls to unnamed routines, then sets the selector to one.
- `copy_conditional_block`: Checks if a global variable equals a specific value and, if so, calls another routine and copies a fixed number of words from a source address to a destination address.
- `service_interrupt_bits_with_counter`: Checks four interrupt request bits, each only when its interrupt enable bit is also set; it counts occurrences per bit, and after three it clears that request, disables that enable bit and sets a state value, with the last bit also resetting the counter to zero.
- `decode_keyboard_scancode_into_table`: Decodes the keyboard serial byte by rotating and inverting it, and when the key code is below 232 it stores the code in a key-state table if the key is pressed or zero if it is released.
- `clear_byte_at_indexed_offset`: Writes a zero byte at the address held in the address register plus the offset in the D0 register and returns the state with the high bits cleared.
- `pack_direction_status_bits`: Returns early when the global is non-zero; otherwise it maps comparison results onto single bit values that combine into a packed direction code, and adds ten to a negative value.
- `decode_joystick_port_one_directions`: Builds a four-direction code from the port-one joystick movement bits, adding the value 128 when a status bit is clear.
- `read_mouse_counter_deltas`: Reads the mouse port counters, computes horizontal and vertical movement deltas against the previously stored counter values, and stores each delta in the global value along with the right and middle button states.
- `reset_global_to_minus_one`: Sets the global word to minus one and returns.
- `wait_until_global_clears`: Busy-waits until a global byte becomes zero.
- `update_nibble_flag_bits_2`: Returns early if the low nibble of the argument matches the global; otherwise it stores that nibble and sets or clears bit 64 of the global from each of its four bits in turn.
- `set_flag_bits_from_nibble`: Stores the argument word in the global, then sets or clears bit 64 of it according to each of its four low bits.
- `apply_three_flag_bits`: Clears the bit selected by the low three bits of the D1 argument in a global byte, then sets or clears that bit according to the values two, four and eight in the D0 argument.
- `apply_two_flag_bits`: Clears the bit selected by the low three bits of the D1 argument in a global byte, then sets or clears it according to the values four and eight in the D0 argument.
- `apply_single_flag_bit`: Clears the bit selected by the D1 argument in a global byte, then sets or clears it according to the value eight in the D0 argument.
- `clear_selected_bit_in_byte`: Clears the bit selected by the low three bits of the D1 argument in a global byte and returns.
- `fill_region_pattern_after_calls`: Makes four calls to unnamed routines, then writes a four-longword pattern from a constant table repeatedly through a 32000-byte region, working backwards from its end.
- `fill_region_with_pattern_backwards`: Writes a four-longword pattern from a constant table repeatedly through a 32000-byte region, working backwards from its end.
- `invoke_single_routine`: Calls one unnamed routine and returns.
- `copy_eight_longs_in_place`: Copies eight 32-bit words starting at a global pointer; the source and destination are both set from that same global, so the copy leaves memory unchanged.
- `wait_blitter_idle_after_calls`: Makes five calls to unnamed routines, waits while the blitter busy flag is set, then makes one more call.
- `setup_blitter_copy_rectangle`: Waits for the blitter to go idle, sets its source and destination pointers, first and last word masks, modulo values and control registers, then starts a blit sized from the width and height arguments.
- `forbid_task_switching_and_call`: Disables task switching through an exec library vector, then calls a second library entry and returns both results as one value.
- `permit_task_switching_and_call`: Re-enables task switching through an exec library vector, then calls a second library entry and returns both results as one value.
- `clear_status_table_with_sentinel`: Clears 532 words starting at a fixed address, then writes the value 32768 into the word after the cleared block.
- `write_dma_regs_if_idle`: Returns early when a global flag is set; otherwise it makes a call and writes two argument words into the interrupt-enable and DMA-control registers.
- `set_flag_write_dma_words`: Sets a global flag, makes one call, writes two argument words to the interrupt-enable and DMA-control registers, and clears the flag.
- `call_write_dma_words_flagged`: Makes one call, then with a global flag set writes two argument words to the interrupt-enable and DMA-control registers and clears the flag.
- `call_then_write_dma_control`: Makes one call, then writes the argument word to the DMA-control register.
- `start_audio_channel_playback_2`: Selects an audio channel from the low two bits of the D2 argument, sets its control bit, loads its sample address, length, period and volume from the arguments, waits through a delay loop, sets the final period and clears the channel's pending interrupt request.
- `call_library_forward_if_nonzero`: Calls a library function; when it returns non-zero it stores the result in a global, calls one unnamed routine, then forwards to a library entry at a fixed negative offset of 36.

### logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf

- `empty_routine`: This routine is empty and contains no executable instructions.
- `empty_routine_2`: The routine contains no executable instructions.
- `process_bitplane_data`: Iterates through a list to perform bitwise operations and additions on data stored at a destination pointed to by a register.
- `spin_count_down_to_zero`: Counts the D0 register down to zero as a busy delay and returns.
- `push_constant_frame_and_call`: Saves a block of constant words from fixed addresses into a stack frame, then calls one unnamed routine.
- `copy_words_then_save_registers`: Copies 32-bit words from a source until the destination passes a limit, stores a zero terminator, then saves the data and address registers and the user stack pointer into global cells.
- `save_registers_into_globals`: Stores zero at the destination, then saves the data and address registers, the user stack pointer and the status flags into global cells.

### logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf

- `save_registers_and_call_routine`: Saves the data and address registers and the status flags into global cells, then calls one unnamed routine.
- `save_registers_to_globals`: Stores the contents of several registers and the user stack pointer into a series of global variables.

### logged disk load from Star Wars - Return of the Jedi (1988)(Domark)[cr QTX][a2].adf

- `null_routine`: The routine is empty and contains no executable instructions.
- `empty_routine_3`: This routine contains no executable instructions and performs no operations.
