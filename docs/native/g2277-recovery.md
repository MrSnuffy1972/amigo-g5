# Game 2277 (star wars (1987)(domark)[cr bs1 - tkt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02277). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901248 top-level bytes in 37 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 461 of 461 named (0 author symbols), 461 of 461 described (53 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 87 of 87 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 17276 of 17324 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 236030 |
| code | 58548 |
| data | 78663 |
| padding | 7303 |
| unused | 520704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Star Wars (1987)(Domark)[cr BS1 - TKT].adf:binddrivers | 4 | 549 | 26 |
| Run | 2 | 117 | 2 |
| Stack | 2 | 117 | 2 |
| List | 2 | 117 | 2 |
| WB2 | 3 | 22 | 1 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Star Wars (1987)(Domark)[cr BS1 - TKT].adf:KENT | 1 | 283 | 11 |
| Star Wars (1987)(Domark)[cr BS1 - TKT].adf:starwars | 3 | 15404 | 411 |
| Star Wars (1987)(Domark)[cr BS1 - TKT].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Star Wars (1987)(Domark)[cr BS1 - TKT].adf:binddrivers

- `set_up_display_and_dma`: Entry routine that opens the graphics library through the exec vectors, copies eight colour words into the custom colour registers, and builds a copper list and bitplane pointer table.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `advance_bitplane_offsets_and_store`: Adds a signed step to a stored 32-bit value and reverses the step.
- `start_blitter_and_wait`: Sets the blitter control, modulo and destination pointer registers from the current buffer pointer, starts a fixed-size blit, and busy-waits until the blitter reports it is finished.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `plot_and_advance_points`: Loops over 64 point records, setting one bit in the bitplane at each point's position.
- `blit_rows_from_offset_table`: Waits for the blitter, then queues a series of blits for a grid of rows, taking each row's offset from a lookup table.
- `blit_tile_grid_from_table`: Draws a grid of 32 rows by 2 cells, issuing one blitter operation per cell after waiting for the blitter.
- `advance_bouncing_positions`: Updates two coordinates by their velocities and reverses each direction at fixed limits.
- `fill_lookup_table_entries`: Fills up to 32 eight-byte table entries, each holding an incrementing byte and a word taken from a second table.
- `wait_blitter_then_write_control`: Calls a subroutine, saves the status value it returns, and busy-waits until the blitter busy flag clears.
- `init_audio_channel_table`: Sets up four audio channel records that point at the audio volume registers and their handler routines.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `advance_audio_tick_and_step`: Counts a six-step tick.
- `update_audio_channel`: Increments a counter and performs conditional updates to a global variable based on several status flags.
- `populate_audio_channel_table`: Iterates through a data structure to initialize a table of pointers.
- `populate_audio_channel_table_2`: Iterates through a data structure to calculate and store memory addresses into a table.
- `populate_audio_channel_table_3`: Iterates through a data structure to calculate and store memory addresses into a table.
- `update_audio_sample_buffer`: Increments a counter and, upon reaching a threshold, calculates a new memory address for an audio buffer based on a table lookup and a multiplier.
- `update_audio_channel_pitch`: Updates one audio channel record's pitch value by a step taken from a parameter byte.
- `start_audio_channel_sample`: Starts playback on an audio channel: it copies the sample pointer from a table, reads the sample length and volume from parameter tables, and writes them to the channel's pointer and length registers.
- `dispatch_pending_interrupt_requests`: Checks the interrupt request and enable registers and, when the relevant bit is set, tests each request bit in turn.
- `dispatch_interrupt_handlers`: Checks hardware status bits and calls a routine from a table based on which bit is set.
- `dispatch_interrupt_bit_handlers`: Tests two request bits in the supplied value.
- `call_interrupt_handler_vector`: Tests bit 16 of the supplied flags and, when it is set, calls the handler through library vector 136.
- `return_saved_value_stub`: Returns the value it was passed on the stack without doing any other work.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_fifteen_longs_then_call`: Copies fifteen longwords from a fixed table into the buffer passed in, if that buffer pointer is non-null, then calls the routine whose address is held in a register.

### Stack

- `run_cli_command_setup`: Entry point of a command-line utility.
- `copy_stack_data`: Copies a block of sixteen bytes from a source address to a destination address on the stack before calling another routine.

### List

- `initialize_dynamic_data_structure`: Traverses a linked list to find a maximum value and then populates a structured memory block with a sequence of incrementing values.
- `copy_stack_data_2`: Copies sixteen words from a source pointer to a destination pointer located on the stack.

### WB2

- `open_libraries_and_set_flag`: Opens the graphics and intuition libraries through exec vectors, sets a flag byte inside the structure obtained from one of them, then calls two library vectors and returns zero.

### Port-Handler

- `initialise_handler_message_port`: Initialises a handler's control block for a message port.

### Disk-Validator

- `validate_disk_block_table`: Program entry for a disk validation utility.

### Ram-Handler

- `copy_memory_then_jump`: Calls the exec copy-memory vector to move a block of memory, then jumps to the address it has loaded.

### Star Wars (1987)(Domark)[cr BS1 - TKT].adf:KENT

- `dispatch_command_line_arguments`: Program entry that checks the argument count.
- `open_dos_library`: Opens a library through the exec vector and, on success, stores its base in a global.
- `write_string_if_nonempty`: Measures the length of the null-terminated string at the address in a register.
- `lock_examine_and_compare_file`: Locks a named item through dos, examines it, and unlocks it.
- `lock_file_and_read_block`: Obtains a lock on a named item, then opens and examines a second item and allocates a buffer for it.
- `free_memory_or_return`: Returns the stored value if a global flag is zero.
- `allocate_and_copy_table_block`: Stores two library vector values in globals, allocates a block of memory through the system, and copies a 96-byte table from a fixed address into it.
- `call_two_subroutines`: Four-instruction routine whose pseudo-code could not be produced, so only its listed facts are known.
- `return_low_byte_passthrough`: Returns the low byte of the data register it was given, without changing it.
- `return_without_action`: Performs no work and returns straight to its caller.
- `store_dos_name_bytes`: Writes the four ASCII characters of the text dos.

### Star Wars (1987)(Domark)[cr BS1 - TKT].adf:starwars

- `start_program_with_memory_check`: Program entry that requests memory from the system.
- `clear_return_register`: Clears a register to zero and returns.
- `store_flagged_seven_bit_value`: Returns zero.
- `swap_buffers_and_update_planes`: Swaps two stored buffer pointers when a pending flag is set and counts a frame counter down.
- `decode_joystick_movement_deltas`: Decodes the direction bits of a joystick data word into horizontal and vertical movement values.
- `read_joystick_fire_and_movement`: Reads the fire line from the CIA port register and the joystick data word.
- `decode_joystick_and_reset_fire`: Sets the fire-state byte to all ones, decodes the joystick data word into movement deltas, and stores them in the joystick state.
- `accumulate_joystick_position_deltas`: Decodes direction bits from a joystick data word into horizontal and vertical steps and adds them to stored position values.
- `accumulate_joystick_x_clamped`: Uses the joystick data word already in a register to derive horizontal and vertical steps, adds them to the stored position, and clamps the horizontal value to the range 0 to 319.
- `accumulate_joystick_steps_clamped`: Adds the joystick steps to the stored position, clamps the horizontal value to the range 0 to 319, and clamps the vertical value to the range 0 to 199.
- `clamp_position_and_countdown`: Clamps the vertical position to the range 0 to 199 and holds the horizontal position at zero or above.
- `countdown_timer_and_reload_record`: Clamps a stored value to be nonnegative and, when a flag is zero, counts a timer down.
- `countdown_timer_and_set_background`: Counts a stored timer down and, when it reaches one, resets it and copies several stored values into an object record.
- `read_and_clear_pending_word`: Returns a stored 16-bit value and resets the stored value to zero when it was nonzero.
- `request_handle_into_stack_buffer`: Requests a handle through a system vector.
- `fill_stack_buffer_via_handle`: Sets up a stack buffer, writes a fixed value into its end, calls two system vectors with a handle and the buffer, and returns the value it stored there.
- `request_resource_1005_and_release`: Requests resource number 1005 through a system vector.
- `call_indirect_pointer`: Calls a routine through a pointer stored at an offset from a register.
- `request_resource_1005_alt_release`: Requests resource number 1005 and, when it is obtained, passes it and two parameters to a different system vector before releasing it.
- `execute_indirect_pointer`: Fetches a pointer from a memory location calculated by adding an offset to a value in a register and then calls the routine at that address.
- `request_resource_1005_or_1006`: Requests resource number 1005 and, if that fails, requests resource number 1006 instead.
- `request_resource_1006_and_release`: Requests resource number 1006 using a value from its caller, passes the resource and two stored parameters to a system vector, then releases the resource.
- `draw_stepped_offset_pairs`: Calls a setup vector, then loops over a count, stepping through a table index.
- `draw_mirrored_offset_pairs`: Repeatedly calls two vectors with a coordinate and its mirror image around a stored centre value, advancing a stepped counter on each pass.
- `draw_mirrored_offset_pairs_loop`: Repeatedly calls two vectors with a coordinate and its mirror image around a stored centre value, updating the step values on each pass until the loop ends.
- `blit_rectangle_rows`: Works out the screen address and row width of a rectangle from stored offsets and two inputs, enables the blitter DMA bit, and blits each row, waiting for the blitter between passes.
- `fill_object_record_fields`: Fills an object record.
- `fill_object_records_per_plane`: Repeatedly fills object records and adds 8000 to a bitplane address on each pass, resetting a count value for each row.
- `advance_seed_return_high_bits`: Seeds a stored value from a word the first time it is used, then calls a system vector to advance it and returns the stored value shifted right by eight bits.
- `update_dma_control_bits_2`: Writes to the DMA control register to change channel enable bits.
- `update_dma_control_bits`: Writes to the DMA control register to change channel enable bits.
- `wait_until_flag_clear`: Busy-waits until a stored word is zero, then returns.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `run_pending_callbacks_then_flag`: Calls each of two subroutines when its stored pointer or count is nonzero, then sets a stored flag word to all ones.
- `call_two_routines_in_sequence`: Calls two subroutines one after the other and returns.
- `blit_rectangle_from_coordinates`: Computes the source and destination addresses and row width of a rectangle from two coordinate pairs, sets the blitter DMA bit, and blits the rectangle in several passes, waiting for the blitter on each.
- `load_blitter_registers`: Waits until the blitter is idle, then loads the blitter pointer, modulo and control registers from stored values.
- `load_blitter_registers_loop`: Repeatedly loads the blitter pointer, modulo and control registers from stored values, one set per pass.
- `fill_fixed_length_region`: Calls a routine with a bitplane address offset from a stored base, a stored value, zero and a fixed length of 45.
- `fill_row_span_from_coordinates`: Calls a routine with the bitplane address of a row, a stored value, zero, and the width between two stored coordinates.
- `fill_row_span_alt_order`: Calls a routine with a stored value, the bitplane address of a row, zero, and the width between two stored coordinates.
- `request_with_flags_or_fail`: Calls a system vector with the given value and a fixed flags value.
- `return_stored_longword`: Returns a longword stored at a fixed offset from its base register.
- `return_stored_longword_2`: This routine returns a longword value stored at a fixed offset from a base register.
- `call_vector_with_two_params`: Calls one system vector, passing on the two parameters it was given.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `configure_interrupts_and_dma`: Calls two subroutines, writes an interrupt-enable value and a DMA control value to the hardware, makes one vector call with a fixed argument, and writes a final control value.
- `store_vector_result_slot`: Calls a system vector with the value two and stores the returned value in a stored slot.
- `request_two_resources_and_setup`: Requests two named resources through a system vector and stores the returned handles.
- `apply_scaled_table_entries`: Loops over six 22-byte table entries and, for each non-null entry, calls a system vector with the entry and its value multiplied by ten.
- `release_stored_handles`: Calls a system vector with a stored handle, runs one subroutine, then releases two more stored handles through system vectors.
- `copy_bytes_with_overlap`: Copies a block of bytes from source to destination.
- `copy_byte_run`: Copies a run of bytes forward from source to destination, one byte at a time, for the count plus one bytes.
- `copy_words_with_overlap`: Copies a block of 16-bit words from source to destination.
- `copy_word_run`: Copies a run of 16-bit words forward from source to destination, one word at a time, for the count plus one.
- `return_absolute_value_word`: Returns the absolute value of a signed 16-bit input, negating it when it is negative.
- `return_absolute_value_long`: Returns the absolute value of a signed longword, negating it when it is negative.
- `multiply_and_divide_words`: Multiplies two signed 16-bit values, divides the 32-bit product by a signed 16-bit divisor, and returns the remainder in the high word and the quotient in the low word.
- `return_sum_of_squares`: Returns the sum of the squares of two signed 16-bit values packed into its input.
- `return_product_of_words`: Returns the product of two signed 16-bit values packed into its input.
- `compute_and_store_word_pairs`: Calls the same subroutine on pairs of stored source values and stores each result into a sequence of stored word fields.
- `shift_stored_word_pair`: Calls three subroutines, then stores the new result in one stored word field and moves the previous value of another field into a second field.
- `compute_and_store_result_pair`: Calls subroutines to compute a value and stores it in the first word of the pair at the address given, with the second argument in the next word.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `normalise_angle_with_offset`: Adds a fixed offset of 360 to an angle-like value and wraps it into a circular range using a divisor of 1440, mirroring values past 720 or 360.
- `normalise_angle_into_range`: Wraps an angle-like value into a circular range using a divisor of 1440, mirroring values past 720 or 360.
- `compute_line_step_increments`: Computes two per-step increments from the difference between two stored coordinates, scaled by 512 and divided by a value offset by 600 from the input.
- `compute_scaled_position_pair`: Computes two stored coordinates by adding offsets to stored values, shifting the sums left by seven bits, and dividing by a value offset by 600.
- `add_packed_decimal_value`: Adds a four-byte packed decimal value into a stored packed decimal number, adjusting each byte for decimal carry.
- `store_and_add_packed_decimal`: Stores the argument in a stored location, then adds four packed decimal bytes from one stored field into another, adjusting each byte for decimal carry.
- `increment_packed_decimal_byte`: Adds one to a stored packed decimal byte with decimal adjustment, then calls a subroutine.
- `compute_scaled_table_entries`: Loops over 50 table entries, scaling each value by 512 and dividing it by a stored offset of 600 plus a parameter, and writes the results to an output array.
- `copy_pair_after_call`: Calls a subroutine, then copies two stored word fields into the two words at the address held in a register.
- `update_object_positions_table`: Loops over a table of object records, adding scaled velocity values to their coordinate words.
- `process_object_entries_with_callbacks`: Walks a table of object entries, updating coordinate fields and calling three subroutines for each entry.
- `setup_object_state_from_arguments`: Stores its two arguments in stored state, then calls four subroutines to set up the related object values.
- `call_one_subroutine`: Pseudo-code could not be produced, so only the listed facts are known.
- `transform_point_table`: Loops over stored data, combining input and stored values with scaling and writing the results to output arrays.
- `copy_adjusted_word_table`: Copies entries between two stored word tables, adjusting each value with a parameter, and calls one subroutine.
- `store_word_parameter`: Stores the low word of its argument into a stored word field.
- `select_stored_offsets_from_flags`: Stores four constant offsets.
- `store_and_scale_coordinate_pair`: Stores two coordinate arguments in stored fields and computes a third field from one of them, scaled by two and offset by a stored value.
- `compute_scaled_coordinate_field`: Computes a stored coordinate field from a difference between two stored values, scaled and offset by stored values.
- `project_coordinate_with_divisor`: Computes a coordinate from the difference between a stored value and an argument, multiplied by a second argument and divided by a third.
- `project_coordinate_for_mode_two`: Computes a coordinate from the difference between a stored value and an argument, multiplied by a second argument and divided by a third, when bit 2 of the mode value is set.
- `project_coordinate_for_mode_eight`: Computes a coordinate from the difference between a stored value and an argument, multiplied by a second argument and divided by a third, when bit 8 of the mode value is set.
- `return_empty_routine_2`: This routine performs no operations and immediately returns to its caller.
- `trace_line_bits_in_bitplane`: Walks coordinate steps along a line, computing byte addresses and bit positions in the bitplane and setting bits as it goes.
- `trace_line_segment_bits`: Walks coordinate steps along a segment, computing byte addresses and bit positions in the bitplane, and calls three subroutines along the way.
- `plot_bitplane_row_span`: Computes a bitplane address from a start point and a row count, then calls four subroutines for pixel handling along the row span.
- `set_pixel_bit_run`: Computes the bitplane byte address and bit position for a coordinate pair, then sets the bit and steps along the run.
- `set_bit_in_four_planes`: Sets one pixel bit in each of four bitplanes spaced 8000 bytes apart, for a run of rows advancing by a stored stride.
- `set_pixel_bits_in_row`: Computes the byte address and bit position for a coordinate, then sets bits along a horizontal run within the bitplane.
- `set_four_plane_bits_loop`: Loops over a run of rows, setting one pixel bit in each of four bitplanes spaced 8000 bytes apart at each row's address.
- `compute_bitplane_pixel_bits`: Computes a bitplane byte address and bit position from a coordinate pair, then combines the bit values into the bitplane bytes.
- `set_bits_in_memory`: Sets specific bits in four different memory locations at regular intervals using a loop.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_pixel_bits_for_point`: Computes the bitplane byte address and bit position for a coordinate pair and sets the bit at that point.
- `set_four_plane_bits_grid`: Sets one bit in each of four planes across a grid of rows and columns, stepping by stored strides.
- `store_coordinate_pair_and_call`: Stores the two words of its argument into stored coordinate fields and calls one subroutine.
- `store_coordinate_pair_return_value`: Stores the two words of its argument into stored coordinate fields, calls one subroutine, and returns a value.
- `plot_point_if_in_bounds`: Checks whether a point lies inside a rectangle of stored bounds.
- `set_four_plane_bit_single`: Sets one pixel bit in each of four bitplanes spaced 8000 bytes apart, at the address passed in a register.
- `clear_rectangle_in_bitplane`: Clears a rectangular area of longwords in the bitplane, sized from stored bounds and a row stride.
- `draw_fixed_offset_sequence`: Makes a long sequence of calls to a drawing vector with fixed coordinate offsets derived from a parameter and a stored value, and calls one subroutine.
- `update_changed_entries`: Calls a setup vector, then loops over 24 entries, comparing each stored word with a reference value and updating the entry when they differ.
- `draw_selection_menu_screen`: Draws the selection menu by placing text labels for the difficulty, bonus and wave options at fixed positions through several drawing calls.
- `draw_countdown_digit`: Converts a stored timer value into a digit by dividing it by 50 and subtracting from nine, then draws the countdown text and that digit with several drawing calls.
- `run_title_screen`: Clears state, sets up the display, draws the title and the prompt text that asks the player to pull the trigger to start, and then waits for input.
- `wait_for_input_or_timeout`: Loops calling system and drawing routines until a flag is set or a frame counter reaches 500, then returns the final state.
- `initialise_scroll_table`: Sets a stored value to 160, calls a system routine with a fixed value, and prepares a table of entries used by the introduction.
- `draw_credits_screen`: Draws the credits screen, placing the title, publisher and author text lines at fixed positions relative to a stored offset.
- `draw_story_text_screen`: Draws the opening story text, placing its lines at fixed positions relative to a stored offset with several drawing calls.
- `run_unrecovered_routine_block`: A 47-instruction routine called from two places.
- `update_state_counters_on_param`: Updates counters held in stored state when a parameter is greater than ten and has its low two bits set, and writes table values into the state.
- `run_unrecovered_routine_block_2`: A 30-instruction routine called from one place.
- `process_stored_value_table`: A large 685-instruction routine called from one place that works through a long sequence of stored values and table entries.
- `poll_until_result_or_limit`: Calls setup routines, then loops up to 100 times calling a status routine and returns its value as soon as it is nonzero.
- `select_mode_from_stored_values`: Calls one routine, then another if its value is nonzero, and returns a small mode code chosen by comparing stored values with its parameter.
- `generate_seventy_entry_table`: Calls a setup vector, then loops over 70 entries, computing position values from stored offsets and writing them into a table.
- `fill_table_from_vector_reads`: Loops over 70 entries, calling a system vector twice per entry and storing the high byte of each result into a table.
- `draw_help_text_screen`: Draws the instruction screen, placing numbered help lines about shields, fireballs and aiming at fixed positions, with the layout chosen by its parameter.
- `draw_scoring_table_screen`: Draws the scoring table, placing enemy names and point values as text lines at positions relative to a stored offset.
- `run_display_setup_sequence`: Initialises a display state, reads a table value, and computes a result through a short sequence of system calls.
- `execute_complex_loop`: Performs a series of indirect calls and table updates within a loop that runs up to seven hundred sixty-eight times.
- `run_display_setup_sequence_2`: Initialises a display state, reads a table value, and computes a result through a short sequence of system calls, using a different starting value.
- `store_colour_table_entry`: Stores the second word of its argument into a 16-entry word table, at the index given by the low four bits of the first word.
- `wait_until_counter_reaches`: Resets a stored counter to zero, then loops until it reaches the value passed in.
- `compute_integer_square_root`: Computes the integer square root of its argument by iterating an estimate until the estimate stops changing.
- `nop_padding`: This routine is empty and performs no operations.
- `store_default_bounds_values`: Stores fixed word values into three stored fields, setting default bounds and a size of 15.
- `store_default_viewport_bounds`: Stores six fixed word values into stored position and bound fields, establishing default bounds.
- `decrement_entries_with_wrap`: Calls a system vector, then loops over 31 stored words, subtracting a step from each and wrapping values that pass a limit.
- `scroll_entries_with_wrap`: Loops over stored entries, subtracting a step from each and wrapping values below minus 500 by adding 14500.
- `scroll_entries_with_wrap_2`: Loops over 31 stored entries, subtracting a step from each and wrapping values that pass a limit.
- `scroll_entries_draw_if_visible`: Loops over stored entries, subtracting a step and wrapping values past a limit, then draws an entry when a comparison holds.
- `scroll_entries_by_parameter`: Calls a setup vector, then loops over 31 stored entries, subtracting the parameter from each and wrapping values that pass the limit.
- `update_entries_wrap_and_mask`: Loops over entries, subtracting a step and wrapping values below minus 500 by adding 14500 while negating two companion fields.
- `scroll_mirrored_entries`: Loops over stored entries, subtracting a step with wrap and negating companion fields when it wraps, and calls a drawing routine for each entry.
- `scroll_mirrored_entries_draw`: Loops over stored entries, subtracting a step with wrap and negating companion fields when it wraps, calling a drawing routine for each entry.
- `wait_for_counter_then_finish`: Resets a stored counter, calls several routines with the parameter, and waits until the counter is positive before calling a final routine.
- `call_indirect_pointer_2`: The routine loads a pointer from a memory location and calls the address contained within it.
- `pop_entries_above_limit`: Walks a stored list backwards, calling a routine and shrinking a stored count for each entry whose key is at or above the parameter.
- `scan_entries_within_range`: Scans stored entries from the current position, calling a routine for each entry that passes a range test.
- `insert_entry_in_sorted_list`: Scans a stored list for the first key above the parameter, increments the stored count, and calls a routine to insert the new entry at that position.
- `copy_entries_to_output`: Loops over the stored entries, calling a routine on each one and writing its first word into an output array.
- `update_entry_positions_by_step`: Calls a routine, then loops over the stored entries, subtracting a step from each key, adjusting a companion field and adding a velocity field to another.
- `compute_scaled_entry_values`: A large 250-instruction routine that computes scaled values from stored entry fields and a multiplier, writing the results for each entry.
- `select_level_parameters`: Sets several stored parameters from a stored counter using thresholds, with a ramp that stops at a fixed maximum, then calls a routine.
- `run_unrecovered_routine_block_3`: A 10-instruction routine called from one place that calls one other routine.
- `call_vector_for_word_run`: Calls a system vector with each word of a run of input words, advancing through the run until its count passes minus one, and returns the position after the run.
- `call_vector_with_two_params_2`: Calls two system vectors, passing on the two parameters it was given.
- `call_eleven_subroutines`: Calls the same subroutine eleven times in a row and returns.
- `initialise_stored_state_words`: Clears four stored state words, sets a starting value of 16000, and calls several routines to set up the state.
- `run_timed_frame_loop`: Repeats a block of library calls and a sub-routine call while a counter stays under a limit and a flag is clear, adding one to a tally each pass and reducing a stored value by a fixed step when it exceeds a threshold.
- `reset_flag_and_wait_idle`: Clears a status word, then polls a routine until it returns zero and the status word is zero again, and finally passes a stored parameter to another routine.
- `draw_scaled_zigzag_lines`: Draws a fixed set of connected line segments whose horizontal reach scales with the parameter, then a stepped pattern of paired moves and draws across the scaled width.
- `display_shield_status_text`: Reads the stored shield value and chooses between two message strings, one for a depleted shield and one for an active one, by calling the text routine with the matching entry.
- `apply_shield_value_change`: When a mode flag equals one, adds the signed parameter to the stored shield value and refreshes its display.
- `add_offset_to_shield_value`: Saves the current shield value into one stored slot, sets a second slot to ten, adds a value from the caller's argument block to the shield value, and calls a routine to refresh it.
- `handle_pause_command`: Handles three command codes: 64 pauses the game by waiting for input to go idle, showing the Game Paused message, waiting for input again and then restoring the saved state; 80 and 81 set and clear a flag byte, and 82 inverts another byte.
- `draw_animated_indicator_frame`: Draws a small marker in one of two states, switching its shape each time state three is entered, and decrements two countdown values while redrawing each countdown at an offset.
- `update_marker_and_countdowns`: Switches the marker shape when state three is active, decrements two countdown values and redraws each countdown at an offset from the marker position, then draws the fixed outline.
- `draw_marker_at_stored_position`: Draws the marker outline and its two countdown displays at a position held in a register, with the same paired moves and draws as the other marker routines.
- `draw_marker_outline_and_countdown`: Draws the first countdown display at a fixed offset from the marker, decrements the second countdown and draws it when it is nonzero, then draws the marker outline.
- `draw_marker_outline_and_pass`: Draws the marker outline at the stored position with a countdown display, passes a nonzero routine result on and clears the state word.
- `update_velocity_with_sign_flag`: Computes a scaled horizontal offset from stored positions, reads three values from a library lookup and blends two velocity values toward new targets, clamping them to fixed ranges.
- `update_velocity_from_offsets`: Computes a scaled horizontal offset from stored positions, reads three values from a library lookup and blends two velocity values toward new targets, clamping them to fixed ranges.
- `clamp_and_blend_velocity_values`: Clamps the first velocity to a positive limit and the second to a fixed range, then blends both toward new targets.
- `blend_velocities_and_flag`: Caps the second velocity value at a positive limit, blends both velocity values toward new targets, and when a pending flag is set clears it, counts down a value and passes control to a routine.
- `set_countdown_to_seven`: Sets one stored countdown value to seven and calls a routine with zero, which redraws the countdown display.
- `print_number_digits`: Prints a value one digit at a time by repeatedly taking the lowest group of four bits and shifting the value down, inserting a separator position after every third digit.
- `tick_countdown_then_redraw`: Decrements a countdown and, when it reaches zero, runs two routines, swaps a stored pointer to a second value and draws the same block at fixed coordinates once with each pointer.
- `draw_panel_and_reset_timer`: Draws a fixed panel by calling library routines at set positions, prints two stored values at offsets, sets a countdown to ten, and draws a frame using two stored coordinates before a final routine.
- `draw_count_marker_and_frame`: Draws a marker at a fixed offset, repeating the drawing step a second time when a stored counter is ten or more, then draws a frame using two stored coordinates.
- `draw_element_by_parameter_count`: Draws a fixed element at one offset, drawing it a second time when the parameter is ten or more, then draws a frame using two stored coordinates and a fixed size.
- `draw_five_text_items`: Computes two offsets from stored positions, then prints five text items at fixed offsets from them, each passing its own buffer to the same text routine.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `fill_angle_lookup_tables`: Fills two sets of table entries by calling a pair of library lookups on a stored input value and a stepped copy of it.
- `draw_tapered_vertical_lines`: Calls one routine repeatedly with a fixed second argument of 250 while stepping the first argument down through seven ranges.
- `draw_shape_by_selector`: Selects one of ten drawing patterns from a numeric code and, for each, stores a start point and issues a short series of paired move and draw calls at small offsets.
- `draw_text_string_glyphs`: Draws a null-terminated string one character at a time.
- `select_random_entry_and_print`: Clears a flag and, when a counter exceeds two, prints a fixed message first.
- `handle_text_entry_key`: Handles one keypress for a text entry field: a printable character is stored at the current position, a space is stored for code zero, and code one ends the entry while codes two and three add a space or delete the last character.
- `set_velocity_and_clear_pending`: Calculates a scaled offset from stored positions, looks up one value from a library table using an index that depends on a sign flag, and stores it.
- `draw_high_score_table`: Draws the high score screen with ten rows of ranks, initials and scores taken from stored records, highlighting the row matched by the parameter.
- `insert_random_score_into_table`: Sums nine values drawn from a library random source and places the total into a sorted ten-entry score table, shifting lower entries down.
- `rank_score_record_into_table`: Accumulates nine random values into the caller's score record, then inserts the total into the sorted ten-entry score table at its rank and shifts lower entries down.
- `insert_score_at_rank`: Walks the sorted ten-entry score table from the top and, at the first entry the stored score is at least as high as, shifts the lower entries down and writes the score into that position.
- `open_device_and_send_commands`: Opens a message port and allocates an I/O request block, then opens a device into that block and sends two commands with a flag set on the first.
- `call_library_function`: The routine clears a memory location and then calls a function from a library.
- `start_disk_track_read`: Configures the disk controller for a track read by writing the sync value and the buffer pointer, starting the transfer and polling a bounded number of times for completion.
- `reset_level_state_and_brief`: Zeroes the per-level state variables, sets a timer and a stored value from the current stage (capped after six stages), and sets the spawn counter.
- `update_starfield_particles`: Moves each of thirty-one particles by reducing its depth value, wrapping it to a far depth when it passes a lower bound, and shifting its horizontal and vertical positions by stored velocities with wrap-around.
- `read_object_position_values`: Reads two word values through a library call, adjusting a stored pointer before the second read, then scales each value by a shift and stores both into the object record along with a third value read from the first call.
- `spawn_random_object_from_pool`: Spawns an object into the next slot of a fixed ten-entry table when the active count is below the stored limit, giving it a position from random values and a fixed timer from a library lookup.
- `update_object_pool_each_frame`: Ages five timer slots, then steps every active object in the pool: it applies stored velocities, counts down per-object timers and resets expired values.
- `launch_object_toward_target`: Scans the object pool for ready entries and, for those that pass a random chance test, computes a launch velocity that closes on a stored target position by dividing by a range value.
- `run_fixed_library_sequence`: Runs a fixed sequence of library calls that alternate between two routines, taking no parameters.
- `draw_eight_point_outline`: Draws an eight-segment outline using coordinate fields from the object record, moving to the first point and drawing through successive points.
- `draw_object_wireframe`: Draws a figure from the object record's enable flags, drawing one or two point rows at offsets that depend on a state field, then draws four groups of point pairs followed by a final set of five.
- `resolve_object_hits_in_range`: Walks the object pool and, for each object in range that passes a random test, reduces its counter and sets the shot flag.
- `remove_record_and_adjust_count`: Scans a table of six-word records for one whose first value is at or below a threshold and wins a random test.
- `update_scroll_and_flag_timer`: Computes a lookup value from a stored offset and a sign flag, derives a scaled offset from stored positions, and reduces two stored totals by scaled amounts unless a level counter is under 100, in which case it sets a constant.
- `draw_blinking_indicator_bar`: When a flag is set and a counter is below 320, advances the counter and draws a short bar at a fixed offset.
- `draw_hud_gauge_readouts`: Draws a status display: while a level counter is below 111 it draws one gauge at a value derived from that counter.
- `draw_all_pool_objects`: Draws every slot of the object pool in order by passing each slot's stored value and a fixed value of 300 to a routine.
- `run_update_loop_until_end`: Repeats a block of updates while a stored count is non-negative and no end flag is set, clearing a timing flag and waiting for it to be set on each pass.
- `scale_value_by_library_lookup`: Passes a value to one library lookup, then passes a fixed scale of 10000 to a second library call and returns the product shifted right by thirteen bits.
- `forward_call_without_arguments`: Calls one routine with no arguments and returns, passing nothing on to its caller.
- `append_word_pair_to_buffer`: Stores two 16-bit values at the current position of a shared output buffer and advances the buffer pointer by eight bytes.
- `record_sample_pairs_into_buffer`: Fills an output buffer with pairs of values returned by a routine, taking one pair per step of forty-eight until a limit of 1488 is reached, then stores a further set of pairs.
- `init_starfield_particle_table`: Fills thirty-one three-word particle records: each gets a depth value that steps upward from a starting value, and two random values shifted down into range as its screen position.
- `repeat_calls_then_busy_wait`: Calls one routine ten times with the same global argument, then spins in an empty counting loop of two million steps as a delay.
- `load_shape_files_into_slots`: Clears six 22-byte slot records and loads one shape per slot by passing each record to a loader routine with a fixed argument.
- `load_resource_into_record`: Looks up a named resource through a library call, stores the returned handle and a word value into the caller's record, then uses the returned value and the handle in a second library call to finish the load.
- `load_assets_and_parse_scores`: Loads the named sound and shape files through a loader routine, clears an eleven-entry table, and reads a formatted score string into packed digit values by replacing blank characters with zeros and combining each pair of digits.
- `init_game_state_defaults`: Zeroes a block of state variables and sets a series of timing and limit values through a library call, including the starting speeds and counters.
- `show_return_key_prompt`: Shows a prompt asking the user to press the Return key, with a list heading.
- `reset_game_state_to_defaults`: Sets dozens of stored state variables to starting values, including several limits, speeds and counters, and clears a 31-entry table.
- `init_score_and_wave_display`: Resets a status flag and two state words, then draws the score and wave labels through the text routine at fixed positions and draws a fixed rectangle before a final library call.
- `wait_for_input_then_release`: Polls a routine until it returns a nonzero result, then loops over a second input check and calls a library cleanup routine once the input completes.
- `load_bitmap_resource_record`: Opens a resource identified by a fixed ID, reads its ten-byte header, computes the data size from its width and height fields, allocates memory for it and reads the data in.
- `load_masked_bitmap_resource`: Loads a bitmap resource the same way as its neighbour, then fills the new buffer with a full-ones pattern row by row before storing the sizes and buffer pointers into the caller's record.
- `blit_clipped_sprite_to_bitplane`: Clips a sprite against a screen rectangle, sets up the blitter's source, mask and destination channels with their modulo values, and starts a masked copy.
- `load_trumpet_sound_and_periods`: Stores a table of twelve sound period values, loads the named trumpet sound resource through a loader, and builds an offset table of sample positions from the header.
- `parse_chunk_offsets_from_resource`: Reads a block from a resource at a fixed offset, then steps through its nested chunk headers by each chunk's length.
- `advance_music_playback_step`: Advances music playback on each tick: when its countdown reaches zero it reads two bytes from the sequence and stops at the end marker.
- `play_channel0_note_from_table`: Looks up a pitch period and a sample pointer for a note index from tables, then writes the channel address, length, period and volume registers.
- `advance_channel0_sequence_step`: Advances the channel sequencer on each tick: when its countdown ends it reads two bytes, stops at the end marker, and for note bytes looks up a value from a table and passes it to the channel-trigger routine.
- `play_channel1_note_and_timing`: Looks up a period for a note index, writes the second channel's sample address, period and volume and enables its DMA. It then sets the channel countdown from a table, doubling or dividing it according to flag bits in the next byte.
- `set_channel1_short_sample`: When the countdown is below three, points the second channel at a sample base offset by 120 and sets its length to a fixed short value.
- `start_music_playback`: Starts music playback by setting the sequence pointers, silencing the first two audio channels, clearing the playback counters and setting the playing flag.
- `stop_music_and_disable_channels`: Disables the first two audio channels in the DMA control register and clears the music playing flag.
- `set_music_playing_flag`: Sets the music playing flag word to one.
- `get_music_playing_flag`: Returns the music playing flag word, which callers use to check whether music is still active.
- `map_event_code_to_result`: Returns the library result for a set of specific event codes, and for any other code returns the input minus nineteen.
- `setup_tower_level_state`: Resets the level counters and scrolling values, loads a starting value through a library call and places the tower records in the table.
- `update_tower_entities_tick`: Moves each tower record by a scaled offset, reduces the vertical positions, and on a random check adds a fixed change to the vertical velocity of a record.
- `wrap_and_spawn_tower_entities`: Advances a tick counter and draws a random value, then moves fifty-one two-word entries by a scaled amount, wrapping their horizontal positions into a fixed range.
- `generate_tower_placements`: Places up to five entries across the level at increasing horizontal positions in fixed steps, each chosen through library lookups.
- `draw_tower_entity_by_type`: Draws one tower record according to its type field: type two draws a fixed set of point and line calls placed from the record's coordinate fields, and other types draw their own outline or move the record first.
- `spawn_tower_entries_randomly`: Walks the placed entries and, for each one not yet spawned with a type below three, rolls a random value and spawns an object when it is under 500, marking the entry as used.
- `check_tower_entries_in_range`: Sets a mode flag from a stored value, then for each placed entry compares its horizontal and vertical positions with a stored range and updates or draws it when it lies inside that range.
- `remove_entry_and_start_timer`: Sets a flag from a stored counter, then walks the placed entries and, for each entry within a limit that passes two random checks, removes it by compacting the table.
- `update_vertical_motion_state`: Updates a vertical velocity value while a counter is below 516: when a timer is zero it eases the velocity toward a target derived from a speed value, otherwise it adds or subtracts a stored step.
- `set_speed_and_random_steer`: Computes a scaled speed from two stored positions, then picks random values for a second speed and a steering value through a library lookup.
- `draw_flash_bursts_by_timer`: When a timer is not positive, draws one burst at a fixed horizontal offset chosen by a status bit.
- `draw_placed_tower_entries`: Draws the placed tower entries from their table, then, when the entry type is above two and the counter is below 500 with no flag set, draws two extra items.
- `run_tower_height_loop`: While a stored height is above 5000, adjusts a stored value and steps through fifty entries, updating a speed value and redrawing each entry.
- `compute_two_depth_projections`: Divides two stored coordinates by two distance terms built from a stored depth value plus the fixed offsets 470 and 730, and stores the four scaled results.
- `compute_spread_projections`: Divides a parameter by two distance terms built from a stored depth value plus the fixed offsets 470 and 730.
- `compute_near_far_projections`: Divides three stored coordinates by two distance terms built from a stored depth value plus the fixed offsets 450 and 750, and stores the six scaled results.
- `compute_wide_spread_projections`: Divides a parameter by two distance terms built from a stored depth value plus fixed offsets of 750 and 450.
- `draw_stage_hint_messages`: Chooses a message row from the current stage and draws its text lines, covering the points for using the force, the exhaust port messages, the catwalk warning and the fireball hint.
- `init_tower_arrays_and_state`: Zeroes the level counters, sets the starting counter to 500 and a stored value to 129, then fills eleven entries of two parallel tables with starting positions.
- `scroll_tower_track_entries`: Computes three projection positions from a stored offset scaled by 512, then moves eleven pairs of entries down by a stored speed.
- `draw_tower_column_rows`: Draws a vertical strip of sixteen cells for each row in a range, where a flag selects either a fixed span from a start row or a span ending near it.
- `draw_level_bonus_screen`: Draws the end-of-level bonus screen: it builds a short text line from the shield count, draws a fixed line pattern across the screen, and prints the messages for remaining shields, shield strength, starting bonus points and the Death Star result.
- `draw_masked_row_with_flags`: Draws a row of cells when a bit flag in the parameter is set, using a sequence of library calls that read and write the sprite registers.
- `draw_tower_object_by_code`: Draws a shape selected by a type code: code twelve draws a fixed pattern of line calls with a register-driven value, code eight draws a line from stored coordinate fields, and other codes draw nothing.
- `draw_tower_bit_pattern`: Draws cells selected by the bit groups of a flag word, stepping the horizontal position by a fixed spacing of 26624 on each pass.
- `set_tower_display_value`: Copies four stored display values into their slots, then passes a value to a library routine chosen by stage: a value from a table for the first eight stages and a fixed value of 18400 after that.
- `fire_tower_shot_when_ready`: When a fire input bit group is set and the cooldown is clear, loops up to five times over a random chance and a position check.
- `draw_stage_tower_layers`: Draws the tower layers for the current stage when its table value is positive, using paired moves and draws at fixed offsets and a set of marker shapes.
- `drain_sorted_tower_entries`: Repeatedly calls one routine while the head entry's key is below the given value and the list pointer is still inside the table.
- `spawn_tower_entries_by_flags`: When a cooldown is clear and a flag mask is set on the current entry, it tries each set flag bit against a random chance.
- `draw_tower_marker_by_stage`: Draws a marker at a stage-dependent horizontal offset, choosing between a position 2000 units out and one 1600 units out based on the stage and a flag.
- `draw_moving_marker_pair`: Draws a pair of marker outlines whose vertical position moves by a fixed step each pass, with the number of repeats taken from a state word.
- `draw_fixed_coordinate_figure`: Stores a long fixed set of coordinate words on the stack, then draws the resulting figure as a sequence of point and line calls.
- `animate_flight_path_sequence`: Steps a value from minus 720 up to zero, changing the step size at fixed points, and on each step draws a frame with a random vertical value and passes the current value to a routine.
- `run_draw_loop_until_flag`: Repeats a draw-and-wait pass, clearing a timing flag and waiting for it to be set on each pass, until a stored flag is set or a second flag is nonzero.
- `wait_until_flags_set`: Repeats a short step while a pair of flags stays clear, calling a routine that waits for a state change on each pass.
- `return_state_block_address`: Returns the address of a fixed block of storage held relative to the program base.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `startup_open_dos_library`: Clears a block of 848 longwords of storage, saves the system library base, and checks a status bit in it.
- `stub_empty_return`: Contains only a return instruction, so calling it does nothing.
- `create_tagged_dos_record`: Allocates a small record, marks it with a four-character tag, and records the current directory, input and output handles through library wrappers.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `write_counted_text_line`: Builds a text line from a counted string held in a stored structure and a fixed separator, writes it through the output wrapper, and then writes the caller's value after it.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a given count or until a null byte is copied, then fills the rest of the count with zeros.
- `append_string_to_destination`: Moves to the end of the destination string and appends the source text one byte at a time, stopping at the terminating null or after 32766 bytes, and then terminates the result.
- `append_string_with_limit`: Moves to the end of the destination string, then copies the source text across up to a counted limit, stopping at the source's null byte.
- `copy_string_to_buffer`: Copies bytes from a source string to a destination buffer, including the terminating zero byte, and returns the original destination pointer.
- `multiply_32_bit_halves`: Combines the 16-bit halves of two 32-bit inputs by cross multiplication, returning the summed cross terms as the high 32 bits of the result, with the second input as the low 32 bits.
- `open_icon_library_resource`: Opens the icon library through the library lookup chain, reads a resource with a fixed ID from it, and stores that resource's pointer and a second value in the caller's record.
- `format_string_to_buffer`: Stores a destination pointer, runs a formatting routine with a character output callback and the caller's arguments, then writes a null terminator at the end of the output.
- `format_to_buffer_return_word`: Formats into a stored destination the same way as its neighbour, writes a null terminator at the end, and returns the formatter's result as a 16-bit value.
- `write_char_to_format_buffer`: Writes one byte to the current position of the formatting destination, advances that position by one, and returns the byte written.
- `forward_format_to_formatter`: Passes its arguments and the character output callback to the formatting routine without any other change.
- `call_two_routines_undecompiled`: The decompiled body is unavailable, so this description rests on the listing alone: the 48-instruction routine calls two other routines and is called by one.
- `format_text_string_with_output`: Walks a format string, sending plain characters to an output routine and expanding each percent specification by reading its argument from a list, with space padding as the default.
- `perform_signed_division`: Reads two signed inputs and sets a sign flag from their signs, calls another routine, and negates that result when the flag is set.
- `negate_result_if_negative`: Calls one routine and negates its result when the first input is negative, then returns the result together with the second input.
- `call_and_return_concatenated`: Calls one routine and returns a 64-bit value built from the register value left by that call together with the original second register value.
- `recursive_call`: The routine calls itself and then returns.
- `divide_32_bit_quotient_remainder`: Divides a 32-bit value by a 32-bit divisor, using a bit-by-bit shift-and-subtract loop when the divisor's high word is set and a 16-bit division otherwise.
- `compute_string_length`: Walks a zero-terminated byte string from its start to the terminator and returns the number of bytes before it.
- `call_subroutine`: The routine calls another routine and then returns.
- `call_three_helper_routines`: The decompiled body is unavailable, so this description rests on the listing alone: the 21-instruction routine calls three other routines and is reached both by a direct call and by a taken address.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `call_two_unnamed_routines`: The decompiled body is unavailable, so this description rests on the listing alone: the 15-instruction routine calls two other routines and is reached by a direct call and by a taken address.
- `write_byte_and_advance_pointer`: Writes one byte from a fixed offset in a data block to the current position of a pointer held in a register, advances that pointer by one, and returns the byte.
- `process_fixed_range_in_steps`: Calls one routine for each slot in a fixed address range, stepping forward by twenty-two bytes each pass until the end of the range is reached.
- `clear_record_resources_by_flags`: Returns minus one for a null record.
- `clear_record_resources_direct`: Performs the same flag-driven resource release as its neighbour on a record already held in a register, with no null check.
- `flush_or_refill_stream_buffer`: Operates on a buffered stream record: it returns minus one when an error bit is set, and a request value of minus one clears the buffer and returns zero.
- `reset_stream_buffer_pointers`: Clears the stream record and its buffer pointers when the request value is minus one.
- `put_byte_into_stream_buffer`: Stores one byte from the caller's frame into a scratch location and passes it to a routine for a single-byte write.
- `send_single_byte_to_stream`: Sends one byte from the caller's frame through a routine routine when the buffer is in single-byte mode, and a failed send sets an error bit and returns minus one.
- `start_stream_buffer_with_byte`: Sets the stream's start and end pointers from its buffer and size fields, marks the stream as active, and writes one byte from the caller's frame at the start, advancing the pointer.
- `allocate_free_record_slot`: Scans a table of twenty-two-byte records from its start and returns the first record whose flag byte is clear, after zeroing its three longword fields.
- `allocate_record_from_pointer`: Scans forward from the pointer held in a register through twenty-two-byte records for the first free one, clears its three longword fields and returns it.
- `clear_three_word_record`: Zeroes the three longword fields of the record addressed by a register and returns nothing.
- `init_stream_buffer_record`: Initialises a stream record: when an allocation check returns zero it sets a size of one and points the buffer at an area inside the record.
- `init_stream_buffer_at_address`: Sets a size of 1024 and marks the buffer as allocated, then points the buffer at the address held in the caller's stack slot.
- `free_linked_record_list`: Walks a singly linked list of records from a global head pointer, releasing each record through a routine with its stored size plus eight.
- `free_record_list_from_register`: Walks a linked list of records starting at the pointer held in a register, releasing each record through a routine with its size plus eight.
- `allocate_and_link_record`: Allocates a block sized to the parameter plus eight bytes through a library call.
- `link_record_into_list`: Pushes a block at the address held in a register onto the global linked list, storing its size from the caller's frame just after the link.
- `forward_low_word_argument`: Passes the low 16-bit word of its argument to another routine and then returns.
- `unlink_and_free_record`: Searches the global linked list for the record whose header matches the given pointer, returning minus one when none matches.
- `unlink_record_from_cursor`: Walks the global linked list from a cursor held in a register looking for the record whose header matches a value in the caller's frame.
- `unlink_record_from_list`: Removes the record at a register address from the global linked list, joining its predecessor to its successor or updating the head when it is first.
- `pop_head_record_from_list`: Moves the global list head forward to the next record of the record in a register, then calls a library routine and returns zero.
- `check_indexed_entry_present`: Checks that an index is within the table count and that its entry is present, setting an error code of two and returning minus one when it is out of range or empty.
- `check_entry_at_frame_index`: Looks up the table entry at the index in the caller's frame and passes it to a routine, returning one when that routine reports success and zero otherwise.
- `stub_returns_zero`: Returns zero without doing anything else.
- `delete_file_with_error_report`: Calls a file deletion wrapper with its argument.
- `return_zero_without_work`: Returns zero without doing anything else, like the stub before it.
- `call_two_routines_by_pointer`: The decompiled body is unavailable, so this description rests on the listing alone: the 19-instruction routine calls two other routines, is called by five, and has its address taken.
- `read_write_with_error_code`: Checks the access bits of a file record: when they are clear it sets error code five and returns minus one.
- `write_to_handle_with_error`: Passes a handle and two stored arguments to a write routine.
- `stub_does_nothing`: Contains only a return and does nothing when called.
- `check_break_signal_flag`: Checks the break signal bit through a library call with a mask for that bit.
- `call_routine_then_return_zero`: Makes one call to another routine and returns zero regardless of its result.
- `write_fixed_value_to_output`: Gets the console output handle, writes through it with a fixed value and a length of four, and finishes with a library call.
- `call_optional_hook_routine`: Calls a hook routine through a stored pointer when that pointer is set, then calls another routine and returns.
- `shutdown_and_release_resources`: Releases the table of records by calling a routine for each entry and then once for the whole block.
- `release_entries_under_supervisor`: Runs a loop over the table entries calling a routine for each, releases the whole block, then runs two optional cleanup routines when their stored pointers are set.
- `release_or_return_slot_value`: When an alternate pointer is clear, releases a stored block through a routine when its pointer is set; otherwise it calls a routine with the alternate pointer.
- `return_slot_after_library_calls`: Calls two library routines, passing a stored pointer to the second, and returns a value stored at a fixed slot in the caller's frame.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_indexed_file_entry`: Validates a file-table index and returns minus one, setting error code two, when the index is out of range or the entry is empty.
- `close_file_record_handle`: Closes the handle stored in a file record unless the record's flag bit marks it as already closed, then clears the handle and returns zero.
- `call_close_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_examine_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_unlock_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_entry_from_table`: Calls a library routine selected by a fixed offset from a stored library base.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_cleared_handle_record`: Reads a value through a library call and allocates a cleared 34-byte record through the system memory allocator.
- `allocate_cleared_record_from_frame`: Allocates a cleared 34-byte record through the system memory allocator, stores values from the caller's frame into it and sets a type code of four.
- `fill_existing_record_fields`: Fills an existing record addressed by a register with values from the caller's frame and sets its type code to four, then calls a routine to record the handle.
- `call_routine_with_record_end`: Passes the address of a record's end field at a fixed offset to a routine and then returns.
- `free_cleared_handle_record`: Releases a 34-byte record that was allocated earlier.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `forward_argument_with_constant`: Takes one argument and calls another routine with that argument together with the fixed constant 48, then returns nothing.
- `forward_pointer_argument`: Passes its single argument unchanged to another routine and returns nothing.
- `allocate_entry_with_fields`: Returns zero when the first argument is zero.
- `reset_entry_and_call_library`: If the argument is non-zero, sets its status byte to 255 and two longword fields to minus one, then calls a library routine with the block and a word field from it.
- `call_library_function_stub`: Loads a library base from a global variable and calls the library function at offset minus 198 from it, passing no arguments of its own.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_function_stub_2`: Loads a library base from a global variable and calls the library function at offset minus 294 from it, passing no arguments of its own.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function_stub_3`: Loads a library base from a global variable and calls the library function at offset minus 132 from it, passing no arguments of its own.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_stub_4`: Loads a library base from a global variable and calls the library function at offset minus 210 from it, passing no arguments of its own.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `initialise_self_referencing_header`: Stores its own address plus four in the first longword, clears the second longword and stores its own address in the third longword.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_stub_5`: Loads a library base from a global variable and calls the library function at offset minus 552 from it, passing no arguments of its own.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `allocate_block_and_init_hardware`: Allocates a memory block through the operating system and, if that fails, loops forever writing a colour value to the video hardware.

### Star Wars (1987)(Domark)[cr BS1 - TKT].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_stub`: Returns the value minus one and does nothing else.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_return_stub`: Does nothing and returns immediately.
