# Game 820 (faery tale adventure, the (1986)(microillusions)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00820). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902096 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 445 of 445 named (0 author symbols), 445 of 445 described (168 by rule, 270 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2408 of 2408 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14526 of 14558 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 123418 |
| code | 49530 |
| data | 684507 |
| padding | 4193 |
| unused | 40448 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| Faery Tale Adventure, The (1986)(MicroIllusions).adf:FMAIN | 3 | 13906 | 353 |
| 9 | 1 | 51 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions).adf:echo | 2 | 117 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 88 | 86 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EF8) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 59 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 149-159 (bytes 76288-81920) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.69)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.11)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.57)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 896-917 (bytes 458752-470016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 920-955 (bytes 471040-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 960-1171 (bytes 491520-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 1312-1370 (bytes 671744-701952) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.24)
- faery tale adventure, the (1986)(microillusions)/Faery Tale Adventure, The (1986)(MicroIllusions).adf: blocks 1376-1501 (bytes 704512-769024) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialise_game_screens` (FMAIN): A large setup routine that initializes display buffers, populates lookup tables, and configures screen or windowing parameters. Kind: track/disk check. Pass path: iVar7 == 0.

## Functions


### Disk-Validator

- `format_string_and_initialize_buffer`: Converts a numeric value into a string of characters and writes it to a buffer, then initializes a memory range with a specific bitmask or constant value.

### Faery Tale Adventure, The (1986)(MicroIllusions).adf:FMAIN

- `initialize_memory_block`: Calculates a base address from a global pointer, initializes a new pointer, and clears a block of memory by zeroing out a sequence of longwords.
- `initialize_input_and_resources`: Checks for an input device and, if successful, configures a pointer to a structure, initializes a table with data from a memory buffer, and sets flags in a global status byte.
- `release_flagged_memory_blocks`: Tests bits in two flag bytes and, if a bit is set, calls a library function to free a memory block of a fixed size using a stored pointer.
- `build_six_entry_offset_table`: Initializes a 920-byte structure and clears a 16-bit field, then iterates through six relative offsets to populate a table with a pointer to the next entry and an associated value.
- `search_and_patch_string`: Searches a string or a table of string-related metadata for a matching pattern and, if found, overwrites a character or a sequence of characters in the string buffer.
- `initialise_game_screens`: A large setup routine that initializes display buffers, populates lookup tables, and configures screen or windowing parameters.
- `update_position_and_calculate_delta`: Updates a stored position with new coordinates, calculates and stores the difference (delta) between the new and previous position, and optionally updates a value in a table based on the new coordinate's bit-masked or shifted value.
- `update_view_mode_and_bounds`: Determines a new display mode or offset by searching through a table of range-based entries and updates a stored index or pointer if a change is detected.
- `update_stored_index_on_change`: Reads a byte from a record and passes it to a library call, then compares a lookup-table value with a stored value.
- `compute_offset_coordinate_pair`: Calls two routines to obtain values, then computes two coordinates from a stored position plus those values and a constant, storing the results in two fields.
- `update_entity_position_and_attributes`: Calculates a new coordinate or bounding box for an entity in a table and updates its associated properties, such as sprite or tile-based appearance and certain state flags, based on a lookup or a conditional check.
- `reset_record_if_idle`: Finds a record through a lookup call and, when its counter is not positive and its state is neither 14 nor 15, resets its counter and two fields and sets its state to 14.
- `set_mode_and_offset`: Sets a mode byte to six or five depending on an argument, and if the argument or a certain value has changed, updates a pointer, calls three routines, and computes an offset from a lookup table plus a constant.
- `initialise_record_defaults`: Sets a series of default constants and timing thresholds into state fields, then populates lookup tables and resets certain memory-clearing loops or pointers.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `set_scroll_positions_from_arg`: Calls two routines and derives several position fields from its argument and a returned value, storing a fixed offset minus the argument and a second offset minus the returned value.
- `classify_position_against_bounds`: Compares two stored 16-bit coordinates against fixed bounds and sets a result variable, potentially updating a value in a table or calling a routine with table-derived arguments.
- `update_position_flags_and_call`: Checks if a state and a counter are within certain bounds, masks and validates coordinates, and updates a global variable with a derived value.
- `swap_buffer_pointers`: Swaps two stored buffer pointers, copies two longwords from the new buffer into state fields, and calls several routines with addresses inside the state block.
- `initialize_input_device`: Checks for an input device, retrieves a pointer, and if valid, configures a structure with a pointer and a constant before calling a routine to check for a specific condition.
- `configure_input_request_and_send`: Sets certain fields of an I/O request structure, including a value of ten and a pointer, then makes four library calls on it.
- `collect_flagged_table_entries`: Walks a table of 18-byte records up to a stored count, and for each record with bit two set in its flag byte copies a value into an output buffer.
- `map_value_to_state_index`: Reads a byte from a buffer and remaps it based on a mode or value, then calls several routines to update a state or write a value to a table.
- `populate_lookup_tables`: Iterates through a source buffer to extract and store byte values into several lookup tables.
- `iterate_display_setup_loop`: Increments a local counter while calling another routine and a display routine, stopping if a specific return value or a maximum count is reached.
- `invoke_display_setup`: Calls a routine that sets a mode byte and an offset using a value from a global variable and a constant.
- `set_initial_state_and_call`: Checks a bit in a global variable and, if not set, stores a passed parameter into a global variable, resets another global variable to zero, and calls another routine.
- `load_bytes_from_lookup_table`: Fills two seven-entry byte tables from a lookup routine, and fills a further table from entries starting at a certain offset, while checking for a newline character.
- `loop_until_counter_reaches_ten`: Repeatedly calls another routine while a value stored in a global variable is less than ten, then returns.
- `update_attribute_cache_and_render`: Compares values from a data table against a cached state and calls a rendering routine to update or redraw specific attributes if a mismatch is detected.
- `call_scaled_entry_if_enabled`: Checks if a bit in a flag byte is set, then calls another routine using a table entry, a scaled argument, and a second parameter.
- `initialise_three_buffers_and_pointer`: Calls a routine to clear or initialize three separate buffers, then loads a pointer from a table using a stored index, and finally calls a routine on a fourth buffer.
- `store_stack_value`: Stores a value from the stack into a memory location.
- `update_entity_state_and_bounds`: Iterates through a linked list of entity structures to update or clamp their position coordinates and certain attribute or animation flags.
- `update_entity_animation_and_position`: Iterates through a linked list of entity structures to update their screen coordinates with boundary clamping and update animation frame or sprite index based on certain conditional flags or pointer-based lookups.
- `update_entity_position_and_bounds`: Updates a pointer-based structure or list by calculating new coordinate or attribute values based on a range check and a mathematical offset.
- `pop_byte_from_ring_buffer`: Compares a read index to a write index in a ring buffer, and if they differ, retrieves a byte from a table and increments the read index with wrap-around.
- `advance_pseudo_random_state`: Updates a 16-bit value using a linear congruential generator method and returns a value derived from the new state.
- `mask_random_with_parameter`: Calls a pseudo-random updater and returns the result masked with a provided parameter.
- `get_random_bit`: Calls a pseudo-random updater and returns the least significant bit of the result.
- `mask_random_result`: calls a routine that updates a pseudo-random state and returns the lower two bits of the resulting value.
- `mask_random_with_seven`: Calls a pseudo-random updater and returns the result masked with a constant value of seven.
- `mask_random_result_2`: calls a routine that updates a pseudo-random state and returns the lower six bits of the resulting value.
- `return_random_value`: Calls a pseudo-random updater and returns the resulting value.
- `combine_random_modulo_parameter`: Calls a pseudo-random updater and returns a 32-bit value containing a parameter and the remainder of the random value modulo that parameter.
- `call_library_after_random`: Calls a library routine through a pointer stored in a state field and returns the input registers.
- `load_address_and_branch`: Loads an address into a register from a pointer and then branches to another location.
- `fill_memory_with_ascii_digit`: Calculates an ASCII digit from a register and stores it into a memory range using a descending loop.
- `fill_memory_with_space`: Fills a range of memory with a constant byte value using a descending loop.
- `nested_callback_loop`: Executes a triple-nested loop that calls a series of function pointers, with additional conditional calls if a counter exceeds a certain value.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `step_through_text_with_library`: Iterates through a string, calling a library routine to process or skip text or a library routine to handle a specific control character.
- `load_registers_from_stack`: Loads two longwords from the stack into registers.
- `position_bit_mask_from_registers`: Calculates a bit mask based on register bits and compares scaled coordinates against a stored offset or table entry, returning a byte mask.
- `decrement_stack_counter_by_four`: Calls a library routine through a pointer stored in a state block, then decrements a counter field by four.
- `subtract_block_size_from_counter`: Calls a library routine through a pointer stored in a state block, then subtracts a stored size plus four from a counter field.
- `run_fixed_call_sequence`: Calls a routine with a value from a pointer and then executes a fixed sequence of no-argument calls to the same routine.
- `copy_block_to_multiple_buffers_2`: Calls a routine that copies a block of data into multiple destination buffers.
- `copy_block_to_multiple_buffers`: Copies 16-bit words from a source pointer into five destination buffers using a loop that processes pairs of source words per iteration.
- `do_nothing_routine`: the routine is empty and performs no operations before returning.
- `forward_stored_value_once`: Calls a routine using a value retrieved from a pointer and returns the original input registers.
- `run_stored_value_call_sequence`: Starts with a call using a value from a pointer, followed by a long sequence of no-argument calls to the same routine before returning.
- `call_copy_block_to_multiple_buffers`: Calls a routine that copies a block of data into multiple destination buffers and then returns.
- `update_block_offsets_from_state`: Calculates new coordinate offsets by subtracting stored state values from input arguments and applying a wrapping or clamping logic, then stores these results into a state block.
- `walk_word_records_updating_fields`: Walks a set of 16-bit word records reached through pointers in a state block, updating several of their fields and calling another routine.
- `write_tile_words_at_row`: Computes two destination addresses from arguments using a 64-byte row stride and two base pointers, then writes words to them.
- `compute_wrapped_tile_index`: Converts two screen positions to tile coordinates by shifting them and subtracting stored origin values, wrapping each within its range, and returns a combined index.
- `scan_tile_table_at_position`: Uses stored origins to convert arguments to tile offsets, then scans a word table and updates its entries.
- `expand_run_length_bytes`: When a flag byte is nonzero, reads a byte-coded sequence where negative values indicate run lengths, expanding it into an output pointer.
- `scale_value_by_table`: Conditionally scales a short parameter by a value from a table indexed by a second parameter and adds the result to a first parameter, masking the result to fifteen bits.
- `scale_value_with_sign`: The routine conditionally scales a short parameter by a value from a table indexed by the second parameter, adds it to the first parameter, preserves the sign bit of the first parameter, and masks the result to fifteen bits.
- `test_and_modify_bit`: Loads a longword from a pointer and checks a specific bit, then conditionally sets the highest bit of that longword.
- `mask_lower_bits`: Returns a value with the highest bit cleared.
- `update_coordinate_bounds`: Calculates the distance between a parameter and a global variable, then updates that global variable or sets it to a new value based on certain range checks.
- `update_offset_conditionally_2`: Compares a register to a constant and, if it is not greater or equal, decrements a value at a memory location relative to a pointer before branching.
- `decrement_conditional_offset`: Compares a word in a register to a constant and, if it is not greater or equal, decrements a value at a memory location relative to a pointer.
- `update_offsets_conditionally`: Compares a word to a constant and, if it is not greater or equal, decrements a value at a memory location relative to a pointer; it also adds one to a stored vertical offset if a certain condition is met.
- `update_offset_conditionally`: Compares a word to a constant and, if it is not greater or equal, decrements a value at a memory location relative to a pointer.
- `decrement_offset_on_condition`: Compares a word to a constant and, if it is not greater or equal, decrements a byte at a location relative to a pointer.
- `increment_vertical_offset_above_ten`: Adds one to a stored vertical offset when the difference is above ten, and otherwise leaves it unchanged.
- `handle_error_message`: Checks if a parameter is not equal to two and, if so, calls three other routines with constant arguments and a global variable pointer while referencing error text.
- `select_index_with_range_checks`: Indexes a table with a short parameter, applies a series of range checks to choose a small index, then returns a combined value formed from that index and an input register.
- `modify_registers_and_test_bit`: Loads a word from a pointer, increments a byte, and tests a bit in a register.
- `update_index_from_joystick`: Checks a joystick port and, if certain conditions are met, resets a stored index to zero and sets it to nine, or otherwise updates a stored index based on certain conditions.
- `check_joystick_and_reset_index`: Checks the state of a joystick port and, if certain conditions are met, resets a stored index to zero and sets it to nine, or otherwise updates a value from a table and calls another routine.
- `check_index_threshold`: Loads a value from a pointer, performs arithmetic, and branches based on whether the result is below a certain threshold.
- `reset_index_to_nine`: Clears a stored index to zero, then sets it to nine and calls one routine with nine when the previous value was not nine.
- `sample_pixel_from_map_2`: The routine calls another routine twice under conditional tests on the returned short value and may return zero, otherwise it returns the value from the first call.
- `compute_morphed_bitmap`: Takes two pointer arguments and three short arguments and performs a nested loop that writes the bitwise NOT of the AND of five source words into a destination word for each iteration.
- `run_blitter_copy_for_planes`: Waits for the blitter with a library call, then loops over four plane pointers, programming the blitter source, destination, modulo and size registers for each one.
- `set_blitter_regs`: Writes a constant and two parameters to a set of blitter registers and returns zero.
- `blit_all_planes_with_wait`: Loops over four bitplane pointers, waiting for the blitter through a library call before programming its source, destination, modulo, control and size registers for each plane.
- `blit_single_plane_with_offset`: Checks the blitter busy bit and waits through a library call when needed, then programs the bliter pointer, modulo and size registers using a stored offset.
- `blit_planes_from_argument_offset`: Loops over four bitplane pointers, waiting on the blitter when it is busy, and programs its registers for each plane using an offset supplied by the argument.
- `blit_four_planes_by_table`: Loops over four bitplane pointers held in a table, waiting on the blitter when busy, and programs its registers for each plane from stored values.
- `execute_indirect_call`: Loads an address from a table and calls the routine at that address.
- `execute_indirect_function`: loads a function pointer from a memory location relative to a register and executes it.
- `set_bit_and_test_register`: Performs a bitwise OR with a constant on a byte-sized register, tests a bit in a long-word register, and adds a zero-value byte to a register.
- `calculate_coordinate_offset`: Calculates a coordinate offset using a table, checks certain conditions, and writes a value to a memory location based on the result of another routine.
- `calculate_masked_offset_2`: Negates a word in a register and performs conditional checks on a register before branching.
- `calculate_masked_offset`: Performs several conditional checks and arithmetic operations, calls a routine via a pointer, and stores a masked value into a memory location.
- `add_masked_difference`: Calls another routine, checks a bit in a register, and if a certain bit is set, adds a byte from one register to another.
- `store_byte_difference`: The routine stores the masked difference of two uninitialized bytes into a byte at an offset from a pointer argument and conditionally stores a constant byte to another offset from the same pointer.
- `update_state_with_branches_3`: Checks a value in a table and either decrements a value at a pointer or branches to another routine with a new parameter.
- `update_state_with_branches_2`: Retrieves a pointer from a routine, checks values in a table, decrements a value at a pointer, and executes a series of conditional branches that call other routines with new arguments.
- `update_state_with_branches`: Executes a routine to get a pointer, modifies a value at that pointer, and then executes a series of conditional branches that call other routines with different arguments or pointers.
- `execute_conditional_pointer`: Checks if a register equals a specific constant, then pushes a value to the stack, calls a routine using a pointer, and calls another routine using a table entry and a scaled argument.
- `execute_conditional_function`: Checks if a register is non-zero, then calls a function pointer with a constant argument, updates a pointer, and calls another routine.
- `validate_and_update_state`: Executes a function pointer, then checks a byte in a table and conditionally executes further functions or updates a value if a certain condition is met.
- `initialize_registers_and_branch`: Saves registers to the stack, clears certain registers, and performs an unconditional branch to another location.
- `compare_byte_after_call`: Calls a routine and then checks if a byte at a memory location, offset by a register, matches a constant.
- `increment_counter_on_match`: Loads a pointer, calls another routine, and if a byte at a certain location matches a constant, increments a register.
- `process_battle_outcomes`: Executes a loop that calls a subroutine, checks a byte at a memory offset, and conditionally calls other subroutines or increments a register based on certain memory or register values.
- `report_battle_outcome`: Compares a value at a memory location to a constant and, if it is less than that constant, calls a routine to report a battle outcome if a certain register is non-zero.
- `report_battle_outcome_text`: Chooses between certain battle outcome messages from flags and a stored counter, and calls routines to show the chosen text.
- `validate_map_movement`: Checks certain map or coordinate values and, if they meet certain criteria, iterates through a loop to check if a movement or coordinate change is valid.
- `clear_register_and_branch`: Clears a register and branches to another location.
- `validate_coordinate_differences`: Iterates through a loop, calling other routines to check if certain values at a table offset do not match specific constants, and calculates if the difference between certain values falls within a certain range.
- `increment_and_compare_pointer`: Increments a register, loads a value from a pointer, and branches if the loaded value is less than the register.
- `initialize_counter_and_branch`: Creates a stack frame, saves registers to the stack, clears a word in a global block, and initializes a register before branching.
- `count_slots_of_type_one`: Loops over slots, and for each slot of type one, checks a field of the associated record and calls a routine to update a count or a value in a global variable.
- `compare_address_offsets`: Calculates the absolute difference between values retrieved from a table via a library routine and performs conditional branches based on those differences.
- `branch_on_pointer_comparison`: Performs arithmetic on a register and branches if the result is less than or equal to a certain value or if a register matches a certain state.
- `apply_offset_and_call_routines`: Adds a longword to another, then calls two different routines sequentially before branching.
- `validate_and_store_coordinates`: Calls a routine using parameters or data from a table, and if the return value is zero, stores the returned values back into a table.
- `store_scaled_offset_to_table`: Multiplies a word from a pointer, calculates a new address, and stores a register value into that address, repeating this for a second register and a different address.
- `copy_entry_fields_from_table`: Reads four bytes for an index from a table of four-byte entries, copies them into a second table, and calls routines to apply them.
- `configure_mode_dependent_values`: Sets a series of state fields depending on a mode value, with a different set of constants for each mode, and calls routines with the arguments.
- `fill_table_with_random_values`: Fills word table entries with values from the random generator masked to twelve bits, then calls a routine to finish the table.
- `compare_and_dispatch_table_entry`: Compares a value loaded from a table against a limit and, if not equal, calls a routine using a pointer or value pushed onto the stack.
- `dispatch_table_index_and_execute`: Loads a value from a table using an index, increments that index masked to a certain range, and branches to a routine or executes conditional logic based on that value.
- `branch_to_table_address`: Decrements a register, compares it to a constant, and if a condition is met, shifts the register and jumps to an address loaded from a table.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `enqueue_event_if_not_full`: Advances a write index through a 32-entry ring and, unless the ring is full, stores the argument at the new position.
- `increment_pointer_and_test`: Increments a pointer or register and checks if a certain byte is zero, branching or continuing based on the result.
- `setup_text_window_layout`: Makes a series of routine calls with fixed positional constants, including 400 and 44, and library calls through graphics vectors.
- `print_counted_string`: Measures a zero-terminated string and calls another routine with the string and its length.
- `setup_input_buffer`: Creates a stack frame, saves registers, loads a pointer from a global, sets a destination buffer address, and clears a word in a stack-relative location before branching.
- `process_input_string_2`: Parses a string or a formatted string starting with a percent sign, copying characters into a destination buffer and handling carriage returns.
- `parse_input_string`: Parses a string or a formatted string starting with a percent sign, copying characters into a destination buffer and handling carriage returns or null terminators.
- `branch_to_target`: Performs a short branch to a specific address.
- `parse_input_string_with_format`: copies characters from a source string or a formatted string starting with a percent sign into a destination buffer while checking for a carriage return or a null terminator.
- `update_pointer_and_branch`: Increments a pointer, clears a byte, and branches or calls another routine based on a comparison or a zero-check.
- `do_nothing`: the routine is empty and performs no operations.
- `load_address_and_branch_2`: Loads an address from a global variable into a register and branches to another routine.
- `load_pointer_and_branch`: Loads an address from a global offset into a register, moves a value from the stack into a data register, and branches to a different location in another routine.
- `parse_input_string_2`: Parses a string or a sequence of strings, handling special escape characters and delimiters, and writes the resulting characters into a global buffer.
- `print_search_result_message`: Uses a pointer-based index to access a table and calls a routine to print a message containing a string, a constant, and a provided argument.
- `print_found_message`: Uses a pointer-based index to access a table and calls a routine to print a message containing a string and a provided argument.
- `print_table_entry_text`: Looks up a text entry in a table by a stored index and passes it to another routine for display.
- `initialise_text_block_and_open`: Initialises a text block pointer and a value from a linked structure, then calls routines to clear or set up a display or text object.
- `configure_block_and_flag`: Sets a pointer to a block, sets a word value, calls two routines, and then sets a byte value to a constant.
- `loop_down_by_fives_drawing`: Loops from 100 down in steps of five, calling one routine with the step value and a buffer pointer, and calls another routine on each pass.
- `loop_up_by_fives_drawing`: Loops from zero up to 100 in steps of five, calling one routine with the step value and a buffer pointer, and calls another routine on each pass.
- `clear_fields_then_call`: Clears two 16-bit fields of a structure reached through a global pointer and then calls another routine.
- `run_indexed_call_sequence`: Stores a value from a state field and then executes a fixed sequence of calls with small index arguments, including one based on a count.
- `update_table_from_randoms`: Reads random values, doubles one of them and stores it into a table entry at an index, then continues with further table updates and calls.
- `process_56_byte_record_entry`: Selects a 56-byte record by index and, when its state field is two, makes a library call through the graphics vector.
- `copy_fourteen_longs_to_table`: Copies a block of fourteen 32-bit values from a source pointer into a fixed table in the state block.
- `lock_device_path`: Performs a routine that calls a library function to request a lock on a device path.
- `dispatch_entry_from_table`: Initialises a state block, clears one 16-bit field, then calls a routine with values from an 18-byte table entry selected by the argument.
- `call_unless_state_two_done`: When a record's state is two and a routine returns zero, it returns early; otherwise it calls another routine.
- `load_song_index_entries`: Loads a song index by reading a list of entries and calculating or storing offset-based pointers into a table.
- `update_display_and_input`: Checks a global flag to decide whether to trigger a drawing routine, a window or block-fill routine, and two calls to a routine that writes data to a memory buffer, followed by a recursive check of the routine's own return value.
- `loop_over_twenty_two_slots`: Loops twenty-two times, reading values from a global structure and calling routines that perform drawing or indexing operations based on those values.
- `test_space_key_pressed`: Calls one routine to read a key or input code, stores whether it equals 32, the space character, in a flag, and returns that flag.
- `copy_memory_block`: Checks if a pointer is outside a certain range and, if so, copies a specified number of bytes from a source pointer to a destination pointer using a library call to allocate or locate memory.
- `initialise_game_display_state`: Calculates the signed area or orientation of a bounding box or polygon using cross-product-like logic and updates a global state structure.
- `initialize_pointer_table`: Sets a certain memory location to two, calls a routine with a pointer and a table entry, and then conditionally updates a global variable if a certain value exceeds a threshold.
- `copy_record_position_into_slot`: Copies the position of a 22-byte record into a slot, adding ten to its vertical coordinate, and stores a flag byte from the argument.
- `update_conditional_byte_2`: Checks a byte at a calculated address and, if it matches a certain condition, replaces a byte in a structure with a new value or updates a pointer.
- `update_conditional_byte`: Checks a byte at a memory location calculated from a table or pointer and replaces it with a new value if it matches a specific constant, otherwise writes an undefined byte to an adjacent offset.
- `store_byte_to_offset`: Stores a byte from a register into memory at a fixed offset from a register.
- `update_string_display_coordinates_2`: Updates a list of screen-space or table-space coordinates for string or object rendering.
- `update_string_display_coordinates`: Iterates through a list of string or object coordinates, calculating or updating screen-space or table-space positions based on certain flags.
- `validate_and_update_object_bounds`: Checks if an object's or string's coordinates fall within a specific screen or table-defined boundary, then updates a lookup table with a new object reference or a modified state block if certain conditions or type-checks (such as checking for specific type flags) are met.
- `branch_to_target_3`: Performs a single relative branch to a fixed address.
- `resolve_object_reference`: Validates a pointer or index against a range and updates a lookup table with a new object or string reference.
- `parse_command_string`: Iterates through a string buffer to extract command parameters and update a state-tracking structure.
- `parse_string_and_update_state`: Iterates through a string buffer, updating a pointer and checking for specific control characters or boundary conditions.
- `parse_and_update_object_data`: Iterates through a data buffer to extract or update object properties, calculating new pointer offsets and updating a global state or table based on certain character-based boundary or type checks.
- `validate_pointer_bounds`: Loads a pointer from a data buffer and checks if a byte at an offset equals a certain value, branching if not.
- `update_string_pointer_and_validate_bounds`: Increments a pointer through a data buffer while checking for specific character or coordinate-based boundary conditions.
- `branch_to_target_2`: Performs a short branch to a fixed address.
- `process_input_string`: Reads characters from a library source, stores them in a buffer, and compares a string from a table against a buffer, clearing a table entry if a mismatch occurs.
- `refresh_library_state_pair`: Calls library routines to refresh the state of two linked library bases, issuing three vector calls on each.
- `call_and_release_nonzero_handle`: Calls a routine with two arguments, stores its result in a global, and if that result is non-zero, passes it to another routine before returning the stored result.
- `lock_df0_path_and_verify_type`: Requests a lock on the device path 'df0:' through a library call, stores the handle, and if the handle is non-zero, checks a type field and potentially calls a routine before returning the handle.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `initialize_stack_frame_and_branch`: Creates a new stack frame, clears a word in that frame, and performs an unconditional short branch.
- `clear_byte_on_condition`: Checks a byte at a certain offset, and if it is not zero, clears that byte and certain registers before returning.
- `process_list_and_compare`: Calls another routine, adjusts the stack pointer, and decrements a pointer or counter before checking if a value is less than a constant; if the condition is met, it clears a register and branches away.
- `update_ui_element_state`: Updates a character or pixel buffer or window graphics based on a conditional check of a global flag, then iterates through a table to apply a series of updates to a set of pointers or buffer offsets.
- `update_window_graphics_buffer`: Updates a graphics buffer or window pixel data by calculating a new character or pixel value and then iterates through a table to apply a series of drawing or copying operations.
- `branch_to_subroutine`: Pushes a long address onto the stack, calls a subroutine, adjusts the stack pointer, and then calls another subroutine before branching or checking a register.
- `branch_to_target_address`: Performs a short branch to a fixed location in memory.
- `update_buffer_value`: Loads a word from a pointer, extends it, adds a constant, and stores a byte back to a global offset, then conditionally calls a routine using a pointer and a value from the stack.
- `initialize_game_state_and_buffers`: Initializes several memory buffers and a global status variable, then executes a loop that calls a routine ten times using a table of pointers and calculated sizes.
- `update_status_and_call_function`: Stores a longword from a register into a global variable, then checks if that value is zero before calling a function pointer and potentially adjusting the stack.
- `reset_status_fields`: If a certain flag is clear, this routine zeroes several status fields in a global block and resets a counter to a fixed value.
- `setup_stack_and_branch`: Pushes a long address onto the stack, calls a subroutine, adjusts the stack pointer, and calls another subroutine before performing a short branch.
- `reset_status_fields_and_redraw`: If a certain flag is clear, it zeroes several status fields in a global block, calls several routines, and sets a counter to a fixed value before calling a library vector with a screen-state base.
- `execute_indirect_library_call`: Sets up a stack frame, pushes arguments, and calls a library routine, then executes an indirect function pointer from a table.
- `check_library_call_with_stack_args`: Calls a library routine using two arguments taken from the caller's stack frame, stores the signed result, and if the result is negative, calls another routine and stores its result in a global.
- `set_bounding_box_around_point`: Writes four words into a twelve-byte record at the given index: the point's coordinates offset by 250 left and right, and by 200 above and below.
- `reset_display_and_advance_counter`: Executes several routines with various arguments, updates a counter, modifies a byte in a memory block, and increments a range of bytes in a buffer by a constant.
- `update_display_buffer_loop_2`: Updates a range of memory by copying values from a table, setting certain word values, and calling routines to clear or set display or scroll positions.
- `clear_display_buffer_range`: Calculates a bit-shifted value, clears a range of memory, and checks a register before writing certain words to a table.
- `update_display_buffer_loop`: Executes a loop that calls library routines to move or clear memory, updates certain global or table-based values with shifted or cleared data, and modifies a buffer based on a counter.
- `choose_code_by_flag_byte`: Returns eight, or ten when the table byte at the given index is non-zero.
- `set_mode_dependent_coordinates_2`: Checks a byte in a global variable and, depending on the result, loads a fixed value into a register before branching to another routine.
- `set_mode_dependent_coordinates`: Checks a mode-related value and either calls a routine with calculated coordinates or calls it with fixed coordinates.
- `draw_sprite_at_fixed_position`: Calls a routine with the fixed coordinates of 100, 100, and 100 and a pointer to a record, then returns.
- `process_entity_data_and_branch`: Loads two pointers from a structure and passes them as arguments to another routine.
- `update_entity_state_and_bounds_2`: Updates a byte in an entity structure using a register and checks or modifies coordinate-based distance or bounding box constraints.
- `process_coordinate_logic`: Calculates absolute differences between coordinates and updates a byte at a pointer if certain conditions or register values are met.
- `calculate_coordinate_differences`: Calculates the absolute difference between two sets of coordinates and writes the result to a new memory location, then calls another routine.
- `push_values_to_stack_and_call`: Pushes two words and a longword onto the stack and calls a routine.
- `execute_conditional_branch`: Compares a register to a constant and, if not equal, checks a register and potentially calls another routine before writing constant bytes to a memory location.
- `handle_conditional_logic_2`: Checks a register against several constant values and either calls another routine or writes a constant byte to a memory location offset from a pointer.
- `handle_conditional_logic`: Checks a register against several constant values and either calls other routines or writes a constant byte to a memory location offset from a pointer.
- `set_conditional_flag`: Checks register values and, if certain conditions are met, calls routines using pointers or table offsets and writes a constant byte to a memory location.
- `compare_and_branch_pointer`: Compares a register to a constant and branches if not equal, then loads a signed word from a pointer and compares it to a register before branching.
- `recursively_execute_function_pointer`: If a register is non-zero, it calls a function pointer with a constant argument and then calls another function pointer using an offset from a pointer-based or table-based address.
- `set_sprite_flag`: Checks if a register equals ten and another register is non-zero, then stores a constant byte into a byte at an offset from a pointer.
- `branch_to_global_address`: Loads a word from a pointer, subtracts it from a global variable, and if the result is non-negative, clears a global variable and calls a routine using a pointer.
- `execute_empty_stub`: Calls another routine using a global variable as an argument and returns.
- `validate_and_expand_string_data`: Checks a sequence of pointers or values against a target value and, if certain conditions or certain byte-patterns match, uses a run-length-encoded expansion routine to copy or write data to a calculated memory range.
- `process_script_state`: Clears a byte at offset one of a structure, adds a 16-bit value from offset two to a 32-bit field at offset sixteen, and calls another routine four times if the first byte is zero.
- `parse_script_instruction`: Iterates through a buffer of instructions, decoding custom opcodes to update a pointer, modify a global or local variable, or update a timer or coordinate.
- `decrement_counter_and_update_state`: Writes constant words to two offsets of a structure pointed to by the first argument, then reads a byte from an offset of the second argument.
- `start_audio_channel_one_playback`: Stores channel-one state bytes, then writes the sample pointer, length, and period from its three arguments, along with a volume of 64, to the channel-one registers, and enables the channel-one interrupt and DMA.
- `silence_audio_con_one`: Clears the low bit of a channel-one state byte, then sets the channel-one volume register to zero and the period register to two.
- `store_four_channel_values`: Stores four 32-bit arguments into four state slots in a global block, one argument per slot.
- `initialise_four_audio_channels`: Stores two copies of each of four 32-bit arguments into a global block, zeroes the volume registers of all four audio channels, and copies four words from a table into state.
- `mute_all_audio_channels`: Zeroes the volume registers of all four audio channels, clears the DMA enable bits for channels zero to three, and sets a status byte to a terminal value.
- `install_vblank_music_server`: Fills in an interrupt server structure and an audio-four entry, adds them through the system interrupt-server and interrupt-vector library calls, and sets a timing value.
- `remove_vblank_music_server`: Calls a routine to stop the music handler, removes an interrupt server through the system library, and writes a value to a global variable before restoring an interrupt vector through the library.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_indexed_block`: Calculates an address from a global pointer, clears a specific number of longwords in a block, and then stores a register value and a global variable into that block.
- `execute_empty_routine`: the routine contains no executable instructions and performs no operations.
- `parse_text_fields_and_load_segment`: Parses a text buffer to identify a specific segment identifier, then loads a named segment and builds a pointer table containing split quoted and unquoted string fields.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `initialize_d0_and_branch`: Loads a maximum signed word value into a data register and performs an unconditional short branch to another routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `perform_signed_division`: Determines the sign of two inputs, calls another routine, and optionally negates the result before returning a 32-bit value containing the result and the second input.
- `prepare_signed_division`: Saves registers, checks the sign of registers, negates values if negative, and calls a division routine.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original contents of a register with the value left in that register by the call.
- `execute_recursive_call`: calls another routine and then returns, resulting in an infinite recursive loop.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers, employing a bit-shifting algorithm or standard modulo arithmetic depending on the divisor size, and returns the result in a concatenated 64-bit value.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `run_optional_hook_then_finish`: Calls a stored hook routine when its pointer is set, then calls a second routine and returns.
- `release_loaded_resources`: Executes a loop or twenty times, then checks and calls several routine pointers or library functions based on whether certain global pointers or handles are non-zero.
- `get_library_function_result`: Checks if an index is within a valid range and if a pointer in a table is non-zero, then calls a library function using a pointer from that table and updates a global status variable.
- `call_library_function`: Loads a library base from a global variable and calls a function at a specific offset.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `execute_indirect_library_function`: loads a function pointer from a table or structure and executes it.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `execute_indirect_function_2`: loads a function pointer from a table or offset and executes it.
- `execute_indirect_library_function_2`: Loads a function pointer from a table located at an offset from a global pointer and executes it.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_2`: Loads a library base from a global or external pointer and calls a function at a specific offset.
- `execute_indirect_function_3`: loads a function pointer from a table located via a global offset and executes it.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `execute_indirect_function_4`: loads a function pointer from a table located via a global offset and executes it.
- `execute_indirect_library_function_3`: loads a function pointer from a table using an offset from a global pointer and executes it.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: loads a function pointer from a table or structure and executes it.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `update_struct_fields`: Calls several library functions and writes several bytes and a word to a structure or memory block, then calls another routine recursively or otherwise.
- `delete_signal_message_port`: If a certain field in a block is non-zero, it recursively calls itself, then invalidates a type byte, clears a stored value, and calls a library function.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_with_constant`: Calls a library routine with a provided argument and a constant value.
- `pass_argument_to_library_call`: Forwards a single argument to a library function through a library vector.
- `update_memory_block_data`: If a pointer is non-null, it calls a library function and, if successful, writes a constant, a short, and a pointer into the returned memory block.
- `test_pointer_and_restore_stack`: Checks if a pointer or value loaded from a structure is non-zero and restores the stack pointer if the check fails.
- `initialize_structure_fields`: Writes certain values to a structure pointed to by a register, copies a word from a structure offset onto the stack, and calls another routine.
- `execute_library_function`: Calls a library function using a pointer retrieved from a global base and an offset.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_library_function_4`: Loads a function pointer from a table located at an offset from a global pointer and executes it.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_library_function_5`: loads a library base from a global variable and executes a function at a fixed offset relative to that base via an indirect pointer.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `execute_indirect_library_function_6`: loads a function pointer from a table offset relative to a base address and executes it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_258`: Library stub: loads a library base from a global variable and calls the library function at offset minus 258 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_468_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_150_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_144_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `recalculate_table_size_and_copy_data`: Iterates through a linked list or table to find a maximum value, then expands a destination buffer with an incrementing sequence and copies a block of data from a source pointer to a destination pointer.

### Faery Tale Adventure, The (1986)(MicroIllusions).adf:echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_register_to_register`: Copies a longword from one register to another before branching.

### Faery Tale Adventure, The (1986)(MicroIllusions).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: The routine executes an infinite loop that repeats a single instruction indefinitely.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_38`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_41`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_42`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_43`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_44`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_45`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_46`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_47`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_48`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_49`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_50`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
