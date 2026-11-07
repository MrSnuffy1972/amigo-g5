# Game 2279 (star wars (1987)(domark)[cr df]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02279). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 36 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 427 of 427 named (0 author symbols), 427 of 427 described (50 by rule, 21 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 105 of 105 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16688 of 16688 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 190970 |
| code | 55128 |
| data | 109245 |
| padding | 7665 |
| unused | 538112 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Stack | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| List | 2 | 117 | 2 |
| WB2 | 3 | 22 | 1 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Star Wars (1987)(Domark)[cr DF].adf:starwars | 3 | 15403 | 411 |
| Star Wars (1987)(Domark)[cr DF].adf:fighter | 2 | 83 | 3 |
| Star Wars (1987)(Domark)[cr DF].adf:bootblock | 1 | 10 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: When its pointer argument is non-zero, copies sixteen long words from a fixed offset in a buffer into the memory it points to.

### Stack

- `scan_list_for_maximum`: Walks a linked list of entries, keeping the largest value found in a field and starting from 149.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a destination pointed to by a stack value, then executes a function pointer.

### type

- `traverse_list_and_update_buffer`: Iterates through a linked list to find the maximum value in a specific field, then updates a buffer with this value and a sequence of incrementing longwords.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### List

- `traverse_list_and_update_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then updates a buffer with this value and a sequence of incrementing words.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### WB2

- `open_libraries_and_set_flag`: Calls a library routine twice to obtain two pointers, sets a byte in a structure reached through them to one, then makes two further library calls and returns zero.

### Port-Handler

- `parse_executable_hunk_records`: Fills a load-state structure from tables of section values, then loops over record type words.

### Disk-Validator

- `count_differing_bit_transitions`: Clears a block of words to minus one and walks a range of bit positions taken from stored limits.

### Ram-Handler

- `call_library_then_indirect_jump`: Calls a library routine through a global base offset, then makes an indirect jump through a target table that could not be recovered and is therefore treated as a call.

### Star Wars (1987)(Domark)[cr DF].adf:starwars

- `setup_disk_read_transfer`: Pulses a CIA port bit with timed delays and then writes disk control words and a disk pointer to the hardware registers.
- `forward_to_nested_routine`: Contains a single call to another routine with no arguments and then returns.
- `store_masked_field_if_flagged`: If a type byte at offset four equals one and a flag bit at offset seven is set, stores the word at offset six masked to seven bits into the output word.
- `swap_buffers_and_set_bitplanes`: When a pending flag is set it swaps two buffer pointers, clears the flag, decrements a counter and writes a scaled value into a table.
- `decode_joystick_movement_deltas`: Sets a status byte to 255, then either decodes direction bits from a 16-bit counter word into movement deltas of minus five, five, minus seven or seven, or computes deltas from stored values when a flag is set.
- `track_fire_button_and_deltas`: Tracks a fire-button bit read from a global input byte, setting or clearing a state byte and resetting a stored word to minus one when the button is released.
- `update_player_position_and_trigger_events`: Calculates movement deltas from joystick input, updates and clamps the player's horizontal and vertical coordinates, and checks countdown timers to trigger specific game state transitions or object updates.
- `apply_joystick_deltas_with_clamp`: Decodes direction bits from a 16-bit word into deltas, adds them to stored horizontal and vertical positions, and clamps horizontal values to 0 to 319 and vertical values to 0 to 199.
- `apply_direction_deltas_with_clamp`: Decodes direction bits from a register word into deltas, adds them to stored horizontal and vertical positions, and clamps the results to 0 to 319 horizontally and 0 to 199 vertically.
- `advance_position_with_timed_trigger`: Adds direction deltas to stored horizontal and vertical positions and clamps them to the screen.
- `clamp_positions_and_countdown`: Clamps stored horizontal and vertical positions to their screen ranges, then when a countdown word reaches one it resets that word and copies stored values into an object structure.
- `start_timed_copy_on_countdown`: Clamps the vertical position to a minimum of zero, then when a countdown word reaches one it copies stored values into an object structure, sets a flag word with its top bit set and runs a delay loop of 100 steps.
- `countdown_copy_set_background`: When a countdown word reaches one it copies stored values into an object structure, runs a delay loop of 100 steps and writes a stored value to the background colour register.
- `take_pending_value_and_clear`: Returns a stored word and resets it to zero when it is non-zero, so each pending value is consumed once.
- `exchange_marker_message_with_port`: Waits on a port or signal through a library call, and when that call returns a non-zero handle it stores a fixed marker value in a message, passes it through a library routine and releases it through a second library call.
- `send_marker_message_and_free`: Stores a fixed marker value in a message structure, passes the message through a library routine and then releases it through a second library call.
- `dispatch_resource_or_fallback`: Looks up a resource by a fixed numeric identifier through a library jump table.
- `call_entry_without_args`: Makes one call through the library jump table to an entry point that takes no arguments, then returns.
- `dispatch_resource_second_entry`: Performs the same resource lookup as its neighbouring routine, but passes the found handle with two parameters to a different library entry point before releasing it.
- `call_entry_no_arguments`: Makes a single no-argument call through the library jump table and returns.
- `lookup_resource_with_secondary_id`: Looks up a resource by a fixed identifier and, if it is absent, looks up a second fixed identifier instead.
- `dispatch_record_resource_handle`: Looks up a resource using an identifier paired with a field read from a caller-supplied record, and stores the handle back into that record.
- `plot_symmetric_offset_pairs`: Makes one no-argument library call, then steps a counter and, for each step, calls two alternating library entry points twice each with a symmetric pair of offsets around a centre value.
- `plot_symmetric_pairs_stepped`: Advances an accumulator by a stepped count held in a record and, for each step, calls two library entry points with mirrored offsets around a centre value.
- `plot_symmetric_pairs_until_negative`: Performs the same mirrored-offset calls as its sibling routine but with the exit test placed at the end of the loop, so the loop stops once its counter goes negative.
- `blit_masked_plane_rows`: Waits for the blitter to become idle, then for each of four bitplanes writes a fill word of all ones or all zeros chosen from a bit mask, together with source, modulo and size registers.
- `blit_plane_rows_with_mask`: Writes custom-chip blitter registers for each bitplane in turn, selecting an all-ones or all-zeros fill word from a bit mask and waiting on blitter-busy between planes.
- `blit_plane_rows_early_exit`: Performs the same per-plane blitter register writes as its sibling routine but leaves the loop as soon as the mask shifts to zero, rather than after a fixed number of planes.
- `advance_seeded_counter_value`: On first use it initialises a stored 32-bit value from a 16-bit data slot, then adds the result of a no-argument library call plus one to that value.
- `write_dma_control`: Writes a value to the DMA control register.
- `write_dma_control_register`: The decompiler produced no pseudo-code, so this description rests only on the listed facts: the routine touches the DMA control register, and writes to it enable DMA channels when bit 15 is set.
- `wait_until_flag_cleared`: Spins in a loop reading a 16-bit flag from the base-relative data area and returns once the flag reads zero.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `set_flag_after_optional_calls`: Calls one library entry point when a first status word is nonzero, calls it again when a second status word is nonzero, then sets the 16-bit flag at the same offset that the wait routine polls to all ones.
- `call_routine_twice`: Calls one other routine twice in a row and returns.
- `blit_rectangle_across_five_planes`: Computes a word width and a row height from two corner coordinate pairs, enables the blitter in the DMA control register, then loops five times, once per bitplane.
- `program_blitter_pointers_per_plane`: Loops a counted number of times, waiting on blitter-busy before each pass.
- `program_blitter_pointers_then_wait`: Copies two pointer values into blitter pointer registers, sets a modulo register and advances the source pointers by one plane size, then waits on blitter-busy before the next pass.
- `call_routine_fixed_block_rows`: Passes a pointer offset from a base address, a stored value, zero and a fixed row count of 45 to a second routine.
- `call_routine_on_row_span`: Computes a byte offset from a row index multiplied by a 40-byte row stride, adds it to a base pointer, and passes that pointer with a stored value, zero and the difference between two stored counts to a second routine.
- `call_routine_row_span_swapped`: Makes the same row-offset calculation as its sibling routine but passes the stored value first and the computed pointer second.
- `request_block_with_fallback`: Requests a memory block from a library entry point using a fixed attribute word, and calls a no-argument fallback routine if the request returns nothing.
- `return_stored_long_value`: Returns a 32-bit value read from a fixed slot in the base-relative data area.
- `stub_empty_return`: Contains only a return instruction, so it performs no work and is a stub.
- `forward_two_args_to_library`: Passes its two parameters unchanged to one library entry point and returns.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `set_interrupt_and_dma_controls`: Calls two no-argument routines around the setup, writes control words that set bits in the interrupt-enable and DMA control registers, and then clears low DMA channel bits.
- `store_result_of_library_call`: Calls one library entry point with a fixed argument of two and stores the returned value in a base-relative data slot.
- `open_libraries_and_input_device`: Opens two named system libraries and calls a no-argument fallback routine if either fails to open.
- `release_pointers_with_fixed_sizes`: Walks a table of records and, for each non-null pointer, calls a library entry point with that pointer and a scaled value.
- `close_pointers_after_routine`: Calls one library entry with a stored pointer, calls another routine, then passes two further stored pointers to a second library entry point.
- `copy_bytes_overlap_safe`: Copies a counted block of bytes from source to destination.
- `copy_bytes_forward_counted`: Copies bytes one at a time from source to destination, decrementing a counter until it reaches minus one.
- `copy_words_overlap_safe`: Copies a counted block of 16-bit words from source to destination.
- `copy_words_forward_counted`: Copies 16-bit words one at a time from source to destination, decrementing a counter until it reaches minus one.
- `return_absolute_value`: Returns the magnitude of a signed 16-bit value by negating it when it is negative and returning it unchanged otherwise.
- `return_absolute_long_value`: Returns the magnitude of a signed 32-bit value by negating it when it is negative and returning it unchanged otherwise.
- `multiply_halves_then_divide`: Multiplies the two 16-bit halves of its first argument, then divides that product by the low 16 bits of its second argument.
- `sum_squares_of_halves`: Returns the sum of the squares of the two 16-bit halves of its argument.
- `multiply_argument_halves`: Returns the product of the two 16-bit halves of its argument.
- `fetch_six_values_into_fields`: Calls a routine in three pairs, each pair using a different stored input, and writes the six 16-bit results into consecutive slots of the base-relative data area.
- `shift_stored_word_pair`: Makes three calls, then stores the value returned by a further call into one slot and the previously held slot value into a second slot, moving a pair of stored words along.
- `store_pair_after_three_calls`: Makes three calls, two of them with a stored input, then writes the returned low word and the first argument into the two words of the caller's record.
- `stub_no_operation_return`: Contains only a return, so it performs no work.
- `lookup_quadrant_table_value`: Reduces its argument into a half-turn range of a 360-step cycle using quadrant reflection, looks up a 16-bit value in a table in the base-relative data area, and negates it when a mode flag is clear.
- `lookup_quadrant_table_unshifted`: Performs the same quadrant reflection and table lookup as its sibling routine, but uses the argument directly without the 360-unit offset.
- `project_four_coordinates_by_depth`: Adds 600 to a depth value, then computes four quotients of coordinate differences and sums, each shifted left by nine bits before division by the depth.
- `project_pair_by_depth`: Computes two quotients, each a shifted coordinate sum divided by a depth value offset by 600, and stores them in two base-relative slots.
- `add_packed_decimal_digits`: Adds the bytes of a second value to the corresponding bytes of a four-byte packed-decimal buffer, applying decimal adjustment after each addition so the buffer stays in binary-coded decimal.
- `accumulate_packed_decimal_maximum`: Stores its argument, adds four bytes of packed-decimal digits from one buffer into another with decimal adjustment, keeps the larger of the accumulated value and a running maximum, and then calls a second routine.
- `increment_packed_decimal_byte`: Adds one to a packed-decimal byte in the base-relative data area with decimal adjustment, then calls another routine.
- `project_fixed_length_table`: Projects each entry of a fixed-length table by shifting its coordinate left by nine bits and dividing by a depth value offset by 600.
- `store_projected_pair_to_buffer`: Makes one call and then writes two stored 16-bit values into the buffer that the caller points to.
- `apply_scaled_offsets_to_table`: Adds scaled multiples of a stored scalar to three interleaved word fields of a table in three passes.
- `copy_offset_word_pairs`: Makes calls to prepare three copy passes, each moving word pairs from one table to another while adding or subtracting a fixed offset of 30 and stepping through the destination.
- `project_points_with_depth_guard`: Sets point counts from stored values, adjusting them when a flag is set, and emits three words per row through a helper when a motion flag is clear.
- `relay_single_routine_call`: The decompiler timed out, so no pseudo-code exists.
- `transform_and_project_vertex_rows`: Repeatedly combines coordinate triples with two scalar multipliers using sums and differences, then divides each resulting pair by a depth value offset by 600.
- `copy_table_with_derived_entries`: Copies a table entry by entry into an output buffer, calling a helper routine for each value, then fills several derived entries by copying earlier values at fixed positions.
- `store_argument_word`: Stores the low 16-bit word of its argument in a base-relative data slot.
- `set_flag_dependent_slot_values`: Sets four word slots to default values, then overrides each one when the matching flag bit of the argument is set, and copies the results into several further slots.
- `store_doubled_bound_differences`: Stores two input words, then computes a value from doubled differences between those inputs and four stored bounds and writes it to a slot.
- `store_bound_differences_only`: Computes the same doubled bound differences as its sibling routine and writes the result to a slot, without storing the input words.
- `clip_line_edge_intersection`: Selects rectangle edges using flag bits and computes where a line crosses each one by multiplying a stored difference and dividing by a slope value.
- `find_line_edge_crossing`: Computes where a line crosses an edge chosen by flag bits, using the same multiply-then-divide approach as its sibling routine.
- `find_line_side_crossing`: Computes a crossing of a line with one of two sides chosen by flag bits, and returns it when it lies within the stored range.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `draw_line_into_bitplanes`: Draws a line between two points in a four-plane bitmap with a 40-byte row stride and an error accumulator.
- `draw_line_from_stored_endpoints`: Reads two endpoints through helper routines and draws a line in four bitplanes using the same error-accumulator stepping as its sibling, returning early when the endpoints are rejected.
- `draw_line_between_registers`: Draws a line in the four-plane bitmap between endpoints passed in registers, using the same error-accumulator stepping as its siblings with a fixed row stride and single-bit writes to each plane.
- `draw_vertical_run_in_planes`: Sets one pixel bit in each of four bitplanes for a counted run of rows, stepping down the bitmap by a stored row stride.
- `draw_vertical_run_from_pointer`: Sets the same pixel bit in each of four bitplanes down a counted run of rows, starting from a pointer passed in an address register rather than computed from coordinates.
- `draw_steep_line_segment`: Draws a line segment in four bitplanes that moves one row per step, using an error accumulator to decide when to advance to the next byte column.
- `draw_steep_line_from_pointer`: Draws the same one-row-per-step line as its sibling routine, starting from a pointer passed in an address register and using the stored row stride.
- `fill_horizontal_run_in_planes`: Sets a horizontal run of pixels in four bitplanes, writing the leading partial byte bit by bit, then whole bytes with per-plane colour bytes, then the trailing bits.
- `set_bit_in_memory`: Sets a specific bit within a series of memory locations at regular intervals using a bitmask derived from a counter.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `draw_shallow_line_segment`: Draws a line segment in four bitplanes that advances horizontally by an error accumulator, setting pixel bits along each row and stepping down by a row stride when the accumulator crosses.
- `draw_shallow_line_from_pointer`: Draws the same shallow line as its sibling routine, starting from a pointer passed in an address register and using the stored row stride.
- `store_point_and_call_routine`: Stores the two 16-bit halves of its argument into a pair of base-relative slots, then calls another routine.
- `store_point_pair_and_return`: Stores the two halves of its argument into the same pair of slots as its sibling routine and calls one routine.
- `plot_point_if_within_bounds`: Checks a point against four stored bounds and, when it lies inside them, offsets its vertical position and sets the matching pixel bit in each of four bitplanes.
- `set_pixel_bit_at_pointer`: Sets one pixel bit in each of four bitplanes at a byte pointer passed in an address register, using a bit index taken from a register.
- `clear_block_longwords_backwards`: Clears a block of bitplane memory by storing zero 32-bit words in an unrolled loop, working backwards from a row pointer computed from a stored row span.
- `emit_scaled_table_values`: Divides a 32-bit field from a stored record by its argument and passes the quotient to library entry points.
- `project_and_emit_point_table`: Projects a table of points by dividing scaled coordinates by a depth value offset by 600, and computes paired offsets around each projected value.
- `draw_fixed_text_labels`: Makes a series of drawing calls at fixed screen positions with string pointers, around one library entry point call, then clears two state words to zero.
- `update_countdown_display_and_mode`: Writes one digit derived from a stored timer into a text buffer and draws it beside the countdown label.
- `run_prompt_selection_loop`: Draws the prompt text asking the player to wait for information or pull the trigger, together with two positioned labels.
- `wait_for_state_or_timeout`: Repeatedly calls a set of routines while a state word is zero and a counter stays below 500, then clears a stored value.
- `build_128_entry_offset_table`: Fills a 128-entry table of point triples by deriving each entry from a stored source table with shifts and fixed offsets of 100 and 2000, using two conversion calls with a constant of minus 140 per entry.
- `draw_credits_text_block`: Draws four credit lines at offsets from a stored horizontal position and then copies a fixed-size block of bitplane data to the display.
- `draw_story_text_block`: Draws eight lines of narrative text at increasing vertical offsets from a stored position and then copies a fixed-size block to the display.
- `decode_unavailable_routine_body`: The decompiler timed out, so no pseudo-code exists.
- `update_128_entry_offset_table`: Adds scaled values, derived from two conversion calls on a stored counter, to a 128-entry table of point offsets and copies stored values into a second buffer.
- `inspect_unavailable_routine_body`: The decompiler timed out, so no pseudo-code exists.
- `draw_vertex_pairs_with_offset`: Draws a fixed sequence of vertex pairs read from a stored table, calling two drawing entry points per vertex, with the second taking a stored offset added to each value.
- `run_timed_frame_sequence`: Runs three counted sequences of frames, each frame calling helper routines, busy-waiting until a timer word advances, and drawing with an adjusted vertical value.
- `select_stage_result_code`: Calls a routine and, when its result is nonzero, passes that result to another routine.
- `update_record_table_with_respawn`: Advances a table of records by a per-record velocity and draws each one, respawning a record with fresh values from a helper call when it passes a bound.
- `initialise_record_table_randomly`: Fills a table of seventy three-field records with values from a helper routine, storing the high bytes of two draws and a count from one to eight derived from a third draw.
- `draw_briefing_text_for_index`: Selects one of about fifteen text blocks from an index argument and draws it at a position offset from a stored column with a fixed row offset, then restores the previous buffer pointer.
- `draw_scoring_legend_table`: Draws a scoring legend with labelled rows for targets such as tie fighters, laser towers, trench turrets and the exhaust port, pairing each label with a point value such as 1,000 or 50,000.
- `run_timed_sweep_sequence`: Runs a sequence of up to 768 frames, each waiting for a timer tick and drawing with an offset that steps down every five frames once a threshold is passed.
- `run_timed_sweep_variant`: Performs the same frame sequence as its sibling routine, with the same counter, timer wait and offset steps, and calls a few further routines between frames.
- `run_timed_downward_sweep`: Runs a downward frame sequence from a starting counter of 110 until it reaches minus 360, drawing each frame and stepping an offset down every five frames once the counter is below minus 300.
- `store_indexed_table_entry`: Stores the high 16-bit word of its argument in a 16-entry word table, with the entry selected by the low four bits of the argument's low word.
- `wait_until_timer_reaches`: Resets a 16-bit timer word to zero and spins until it reaches the value passed in the argument.
- `compute_integer_square_root`: Iterates Newton's method toward the integer square root of its argument, starting from the argument divided by 200 plus two, and returns once the estimate stops changing.
- `empty_routine`: This routine is empty and performs no operations.
- `set_default_bounds_small`: Sets three base-relative words to fixed defaults of 15, minus 15 and 15, values used as small extents by the point and line routines.
- `set_default_viewport_bounds`: Sets six base-relative words to fixed default extents and bounds: a pair of horizontal limits at minus 160 and 159, a width-like value of 160, a height-like value of 115, and a pair of vertical limits at minus 84 and 84.
- `advance_record_positions_with_wrap`: Moves a table of thirty records by a speed argument and wraps each position by a fixed span when it passes a limit, choosing the upper or lower limit from the sign of the speed.
- `advance_records_wrap_below_limit`: Steps a counted run of records by a speed, wrapping any position that falls below minus 500 by a fixed span, and calls drawing helpers for each record.
- `advance_records_wrap_above_limit`: Steps thirty records by a speed and wraps any position above 14000 back by a fixed span, then calls drawing helpers for each record and draws it again when a stored comparison value says so.
- `advance_counted_records_wrap_above`: Steps a counted run of records by a speed, wraps any position above 14000 by a fixed span, and calls drawing helpers for each record.
- `advance_records_reflect_on_wrap`: Advances thirty records by a speed and, when a position wraps, negates the record's two velocity fields.
- `advance_records_twelve_bit_wrap`: Steps a counted run of records by a speed, reflecting the velocity fields when a position falls below minus 500.
- `advance_records_reflect_above_limit`: Steps thirty records by a speed and, when a position exceeds 14000, wraps it back by a fixed span and negates the record's two velocity fields.
- `advance_counted_records_reflect`: Steps a counted run of records by a speed, wrapping and reflecting any record whose position exceeds 14000, and draws each record through a helper.
- `draw_frame_with_timer_wait`: Resets a timer word, makes a sequence of drawing calls that use the argument twice, then waits until the timer advances before making one final call.
- `call_routine_and_return`: Makes one call to a library entry point with no arguments and returns.
- `pop_stack_entries_while_matching`: Walks backwards through a stack of six-byte entries while each entry's first word is at or above the argument and the pointer stays within the stack bounds.
- `claim_matching_entry_slot`: Walks an array of six-word entries, skipping those whose key is at or below the argument, until one passes a two-field test.
- `insert_entry_in_sorted_list`: Finds the first six-byte entry whose key exceeds the argument within a counted range, increments a stored count, and calls a helper with that entry and the following one to shift the list.
- `emit_entry_records_with_reciprocal`: Writes one output triple for each active list entry, copying two stored words and a value equal to 25600 divided by the entry's key plus 600.
- `update_entry_list_and_prune`: Advances a list of six-word entries by subtracting a stored step from each key and adding velocity fields, wrapping the lateral coordinate within a twelve-bit range.
- `draw_six_entry_symmetric_pattern`: Scales a table of six entries by a stored factor and, for each entry, plots three symmetric point pairs around a centre using the two alternating drawing routines.
- `set_parameters_from_counter`: Derives three stored parameters from a counter, each switching to a fixed value once the counter reaches a threshold.
- `review_unavailable_routine_body`: The decompiler timed out, so no pseudo-code exists.
- `plot_word_sequence_from_table`: Calls one drawing entry point for each word of a table, stepping through the table two bytes at a time for a counted number of words, and returns the pointer just past the table.
- `forward_pair_to_drawing_calls`: Passes a word argument to one drawing entry point and a long argument to another, then returns.
- `call_routine_eleven_times`: Makes eleven consecutive calls to one no-argument routine and returns.
- `run_timed_update_until_limit`: Clears several state words and loops while a helper reports no event, a flag stays clear and a timer word is below 1500.
- `run_timed_update_counting`: Repeats a timed loop that draws, waits for a tick, reduces a local step value by 250 once it exceeds 1000 and increments a counter.
- `drain_helper_then_final_call`: Clears a flag word, calls a helper repeatedly until it reports no pending work and the flag reads zero, then makes one final call with a stored value.
- `draw_zigzag_point_pairs`: Draws a zigzag pattern of point pairs at fixed offsets by calling two drawing entry points for each pair.
- `draw_shield_status_indicator`: Draws the shield status by choosing one of three constants from threshold tests on a stored level, drawing one of two labels, and reducing a counter above one while drawing a marker, then copies a fixed-size block to the display.
- `adjust_shield_counter_by_delta`: When a state word equals one it adds the argument to a stored counter and makes a helper call.
- `start_counter_from_record_delta`: Copies a stored counter into a state word, sets another state word to ten, adds a value from the caller's record to the counter, and makes a helper call.
- `handle_pause_and_flag_commands`: Dispatches on a command code: code 64 pauses by waiting for a helper to report nothing pending, drawing the 'Game Paused' label, waiting for a resume input and restoring the display pointer, while codes 80 and 81 set and clear a flag byte and code 82 toggles a byte.
- `draw_status_markers_and_glyphs`: Draws status markers chosen by a mode word: a pair of lines in one mode, and a blinking glyph in another.
- `draw_blinking_glyphs_and_timers`: Draws the blinking glyph and the two timed markers, then a fixed point-pair glyph.
- `draw_offset_glyph_and_timers`: Draws a fixed point-pair glyph at an offset from its argument, then the two timed markers and a second glyph.
- `draw_timed_markers_and_glyph`: Draws one timed marker that blinks while its counter is nonzero, then a fixed point-pair glyph.
- `draw_paired_point_glyphs`: Draws two fixed point-pair glyphs at offsets on either side of a stored position, stores a helper result in a record field, calls another helper when it is nonzero, and clears the mode word.
- `update_motion_offsets_and_flags`: Computes display offsets from stored values and helper results, and clamps two velocity fields to plus or minus 270 and plus or minus 320 when a control flag is clear.
- `update_motion_offsets_from_locals`: Performs the same offset and velocity update as its sibling routine, computing the scaled difference from local values first and applying the same clamps when the control flag is clear.
- `damp_offsets_with_clamp_upper`: Clamps one velocity field to an upper limit of 270 and a second to plus or minus 320, updates two offsets as weighted averages toward their velocities, and handles the pending flag like its siblings.
- `damp_offsets_clamp_lateral`: Clamps a lateral velocity field to an upper limit of 320, updates two offsets as weighted averages toward their velocities, and handles the pending flag like its siblings.
- `start_countdown_and_call_helper`: Sets one countdown counter to seven and calls a helper with a zero argument.
- `draw_number_digits_with_separator`: Draws a number one four-bit digit at a time from the low end, placing a separator pair of points after every fourth digit.
- `tick_flash_countdown_when_zero`: Decrements a countdown word and, when it reaches zero, draws a pair of points using the alternate display buffer before making one further call.
- `reset_flash_countdown_and_draw`: Makes setup calls, sets the flash countdown to ten, and then copies a block sized 15 by 29 to the display.
- `draw_small_block_flagged`: Draws a fixed point, calls a helper once or twice depending on whether a stored byte is below ten, and copies a block sized four by eleven to the display.
- `draw_block_with_count_threshold`: Makes setup calls, draws a fixed glyph once or twice depending on whether the argument is below ten, and copies a block sized 17 by 24 to the display.
- `layout_five_item_slots`: Computes two scale values from differences of stored coordinates, then draws five items at positions offset from a computed centre, each using its own record buffer.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `fill_rotation_table_entries`: Advances two stored angle values, then fills a table of six pairs of lookup results by calling two lookup routines with the angle, stepping the angle by 288 per entry.
- `draw_decreasing_gap_column`: Calls one drawing routine repeatedly with a fixed second argument of 250, while the first argument counts down from 1300 to zero in steps that shrink from 46 to 4 as it goes.
- `draw_selected_point_pattern`: Selects one of ten small point patterns from a case number and draws it at the argument position by writing an origin pair to base-relative slots and calling a drawing entry point with each pattern's offsets.
- `draw_text_string_glyphs`: Draws a null-terminated string character by character, advancing by a stored spacing value on each space.
- `handle_initials_entry_key`: Scans up to thirty key-table entries for the selected character using input-state reads, stores the chosen character in the entry buffer and draws it.
- `append_character_to_entry_buffer`: Adds the selected character to a three-character entry buffer, with code '1' ending the entry, code '2' appending a space, code '3' removing the last character, and any other code appended while there is room.
- `draw_status_label_with_value`: Computes a display value from a stored value and a flag byte, sets a scaled offset, and draws a labelled value through a helper.
- `run_high_score_table_screen`: Draws the high-score table with ten ranked rows, each showing a formatted rank, a three-character name and a score, and highlights the row matching the argument.
- `insert_score_into_top_ten`: Sums nine values from a helper into a total, then inserts the total into a ten-entry table of eight-byte records in descending order.
- `insert_total_into_top_ten`: Accumulates helper values into a total while shifting a stored value right by four bits on each pass.
- `insert_value_into_sorted_ten`: Scans a ten-entry table of 32-bit values for the first entry the argument is at least as large as, shifts the entries below it down, and inserts the argument there before calling a helper.
- `setup_timer_io_request`: Creates a message port and an I/O request block, opens a device through a library entry point and sends two requests through the same block.
- `clear_flag_and_call_library`: Clears a global word and then calls a routine with a fixed code value.
- `scan_disk_track_sync`: Configures disk controller registers for a track read, waits on a ready flag with a bounded count, and scans the buffer for the standard track sync word.
- `reset_round_state_hints`: Resets a set of round state words and computes a stored value from a level counter, multiplying it by 15 up to a cap of 90 and adding 80.
- `advance_records_with_angle_offsets`: Moves thirty records by a fixed step, wrapping keys that fall below minus 500, shifts their two coordinates by negated stored offsets with twelve-bit wrapping, and draws each record through a helper.
- `compute_record_offsets_from_angles`: Looks up two values from a table at an angle taken from a record field offset by 360, then stores them as quarter-scaled words in that record's fields.
- `spawn_record_when_slot_free`: When the active count is below a stored limit and a stored counter exceeds 100, fills the next free record with positions and velocities drawn from helper values, marks a slot as occupied, sets default fields and a special flag, and records a timer value of 300.
- `update_active_record_table`: Advances each active record by its velocity and countdown fields, sometimes re-picking velocities from helper values, and draws records that remain alive.
- `assign_targets_to_active_records`: Scans active records and, when a helper check and a bounds test both pass, sets a record's position and velocity from stored coordinates divided by a stored scale.
- `run_alternating_library_pair_calls`: Makes thirteen calls that alternate between two library entry points, each taking no arguments, and returns the result registers unchanged.
- `draw_record_outline_points`: Draws the outline of a record by plotting points taken from fixed field offsets of the record, alternating two drawing entry points and finishing with two extra points.
- `draw_record_segment_pattern`: Draws a record's segments with one or two drawing calls at offsets chosen by a state field.
- `detect_record_hits_and_update`: Scans active records for one that falls within stored bounds on helper-sampled tests, and on a hit sets a flag and adjusts its velocity and countdown fields.
- `remove_random_record_from_list`: Scans a list of six-word records for one whose first field is at or above minus 401 and whose second field is non-negative, and passes helper checks.
- `update_totals_and_countdown_flags`: Once a level counter reaches 100, reduces two 32-bit totals by scaled stored values, and otherwise sets a fixed value of 200.
- `draw_advancing_marker_bar`: When a flag byte is set and an advancing counter remains below 320, draws a short bar at fixed positions whose vertical extent depends on a status bit, using the block copy routine.
- `draw_totals_readout`: Draws a readout of two 32-bit totals.
- `draw_each_record_timed_value`: Calls one routine for each active record with the record's field and a fixed value of 300.
- `run_frame_loop_until_flag`: Runs frames while a stored counter stays non-negative and a flag is clear, drawing each frame and waiting for a timer tick.
- `scale_lookup_by_ten_thousand`: Passes its argument to a helper, looks up a value using a fixed argument of 10000, and returns that value shifted right by thirteen bits.
- `forward_call_to_one_routine`: Calls one other routine and returns.
- `append_pair_to_output_buffer`: Writes two 16-bit values, each stored as a 32-bit integer, into the output buffer whose pointer is held in a base-relative slot, then advances that pointer by eight bytes.
- `build_helper_value_buffers`: Fills an output buffer with pairs of helper values in a loop, then fills a second region with helper values offset by 7500 and minus 10000.
- `initialise_record_ramp_table`: Initialises thirty records whose first field steps from minus 259 by 483 per record, and fills their other two fields with helper values shifted right by twelve bits.
- `delay_after_ten_calls`: Makes ten calls to one routine with the same global argument, then spins through a loop of two million iterations before returning.
- `initialise_shape_record_table`: Zeroes the first field of six records and registers six shape entries with a loader routine.
- `load_named_shape_resource`: Loads a named resource through a library entry point, stores the returned handle and a parameter in the caller's record, allocates a buffer for it, and calls a further library entry point to finish loading.
- `load_sound_and_shape_assets`: Loads sound and shape resources by name through a loader routine, clears a table of eleven entries and formats a numeric value into a text buffer, then packs pairs of its digits into bytes.
- `allocate_game_buffers_table`: Clears a set of buffer pointers, allocates buffers of fixed sizes through a library allocator, and fills a sixteen-entry word table with fixed constants.
- `show_return_key_prompt`: Draws the 'Return key' prompt from a list buffer at a fixed position using three positioned draw calls and one library call, then makes a final library call.
- `reset_game_state_variables`: Sets a large group of game state words, counters and flags to starting values, and clears a 31-entry table.
- `init_score_wave_display_state`: Sets one byte and two words of state to fixed starting values, then makes a fixed sequence of subroutine calls, including one with two constant coordinates and a final call with a constant position and size.
- `wait_status_then_finish`: Repeatedly polls a status routine.
- `load_bitmap_file_into_struct`: Opens a named file in old-file mode, reads a 10-byte header for the dimensions, allocates a buffer sized from them, reads the pixel data into it and closes the file.
- `load_planar_image_deinterleaved`: Opens a named file, reads a 10-byte header and allocates a buffer, then reads the pixel data and splits each group of four source words into four separate plane rows.
- `blit_clipped_sprite_five_passes`: Clips a sprite rectangle against stored screen bounds and adjusts the source pointers and shift for any clipped left or top edge.
- `init_shape_tables_from_file`: Loads the file named shapes/trumpet, then fills a five-entry table of row offsets and half-widths from its header.
- `index_first_and_third_chunks`: Uses a file routine with a fixed identifier to load a buffer, then steps through three consecutive chunk headers.
- `advance_two_voice_sequencer`: Runs once per tick and decrements two countdown counters.
- `advance_channel_zero_voice`: Steps one voice on the first audio channel.
- `advance_second_voice_countdown`: Decrements the countdown for the second voice.
- `advance_channel_one_voice`: Steps one voice on the second audio channel.
- `update_audio_registers`: Modifies a global variable based on a value from a memory offset and interacts with audio hardware registers.
- `init_voice_state_and_streams`: Copies two saved stream pointers into the voice state, then clears four state and countdown words.
- `clear_active_flag_state`: Writes the command value 3 to a shared output word and clears a one-word flag to zero.
- `set_active_flag_state`: Sets a one-word flag in the shared state to 1 and returns.
- `read_active_flag_state`: Returns the current value of a one-word flag held in the shared state.
- `dispatch_code_to_handler`: Takes a code value and returns the result of a shared handler routine for codes 0, 2, 4, 6, 8, 12 and 19.
- `init_tower_level_state`: Resets a set of score and level state words and fills a 26-entry table of positions with values returned by a routine.
- `update_and_cull_sprite_list`: Advances each six-word record in a list: it subtracts a scroll offset from the first word, decrements a timer, adds a velocity-based offset to the third word and nudges the fourth word by an amount returned by a routine.
- `advance_scrolling_object_frame`: Advances a frame counter and updates 51 scrolling values by subtracting a speed and wrapping paired offsets within plus or minus 2048.
- `spawn_records_from_table_groups`: Fills a list of 14-word spawn records from up to five groups of table entries, placing each entry at a column offset set by a stored scroll position and a check based on a routine result.
- `draw_entity_by_type`: Dispatches on a type value, calling a different sequence of drawing routines for types 1, 2, 3 and 5, and a single routine for other values.
- `spawn_offset_shot_records`: Scans the spawn records and, for each one of type below three that is not already marked, takes a routine check below 500 before trying three offset positions.
- `check_records_for_player_hits`: Sets a flag from a stored counter, then walks the spawn records and tests each against the player's bounding box.
- `cull_offscreen_spawn_records`: Sets a difficulty-related flag, then scans spawn records from the front and stops at the first one that is not yet due.
- `update_velocity_and_sample_offsets`: Adjusts a vertical velocity word from stored state and routine lookups: it damps the value toward a target, or steps it up and then down, depending on two stored counters.
- `setup_wave_parameters`: Computes a scaled difference between two stored positions and draws several values from a routine in fixed ranges, with the second range shrinking as a stored counter rises above 500.
- `draw_overlay_items_for_frame`: Increments a frame counter and draws one or two fixed-size items at computed positions.
- `draw_frame_with_optional_extras`: Passes the address of a record block to two routines, then runs a per-frame update.
- `run_scroll_transition_loop`: Returns zero for the first stage.
- `divide_coordinates_by_base_offsets`: Divides two stored 32-bit coordinate values by a stored base value plus 470 and then by the base plus 730, storing each quotient as a 16-bit word.
- `compute_bounds_quotients_for_param`: Divides a parameter offset by a value 30720 below and 30720 above it, using divisors built from a stored base plus 470 and plus 730.
- `divide_values_by_base_offsets_two`: Divides three stored 32-bit values by a stored base plus 450, and the same three values by the base plus 750, storing six 16-bit quotients.
- `compute_wide_bounds_quotients`: Stores three 16-bit quotients: a parameter offset by 46080 below and above, each divided by the base plus 750, and the parameter itself divided by the base plus 450.
- `draw_message_text_screen`: Draws a set of labelled text lines at fixed offsets from a base position and selects one message from an index capped at four.
- `init_scroll_position_tables`: Clears several state words and points two base pointers at the scrolling position buffers.
- `update_scrolling_position_lists`: Recomputes four scale values from stored state, then scrolls two position lists by a speed.
- `draw_block_rows_in_range`: Chooses a row range from a parameter, then for each row draws one item and sixteen more at coordinates read from a table, using the row value as the vertical position.
- `run_level_end_bonus_sequence`: Builds a stage-number string, draws a grid of positions and pauses briefly, then runs three passes of twenty frames each.
- `check_bounds_and_register_hit`: Tests a parameter's mask bits against two sets of stored bounds.
- `draw_sprite_pattern_by_code`: Draws a set of primitive items using stored coordinate values.
- `draw_masked_grid_pairs`: Computes a base coordinate from a stored value, then for each of four nibbles of a mask it draws pairs of items at offsets from that base and advances the base by a fixed step.
- `save_positions_and_set_limit`: Copies four stored position words into a backup area and calls a routine with a stored value.
- `evaluate_trigger_bitfields`: Clears a countdown when it is active and updates a flag from a stored counter.
- `draw_scrolling_sprite_column`: Draws paired items at coordinates from stored state, then updates a vertical offset and a scale value.
- `advance_records_before_threshold`: Repeatedly calls a routine and advances a record pointer by six bytes and a stored index by twelve bytes while the head record's key is below the parameter and the pointer stays inside the record list.
- `spawn_and_update_enemy_shots`: When a mask and a counter allow it, spawns up to two records at fixed offsets from a stored value, gated by routine checks.
- `draw_periodic_indicator_sprites`: Draws an indicator at a position chosen by a stored level counter, using a frame counter that cycles from zero to ten and only draws once it passes five.
- `draw_playfield_and_hit_records`: Draws a fixed group of items at stored coordinates, then, for each active record in range, tests for a collision with a routine check and calls a routine with a constant on a hit.
- `transform_points_and_draw_lines`: Stores a set of coordinate pairs offset from stored values, then passes each pair through two lookup routines and a combining step to scale it.
- `sweep_scroll_value_across_frames`: Loops a value from -720 up to zero in steps that change at -360 and -120.
- `run_frames_until_flag_set`: Runs frames in a loop, clearing a counter each time and waiting for a frame routine to set it, until a status flag or a state flag becomes non-zero.
- `loop_until_status_flags`: Repeatedly calls drawing and timing routines, choosing between two routines based on a counter and a one-time marker, until either of two status words becomes non-zero.
- `get_shared_table_address`: Returns the address of a fixed table inside the shared data area, computed from the base pointer by a constant offset.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `init_dos_library_or_alert`: Clears 849 longwords of shared state and saves the stack base.
- `return_immediately`: This routine performs no operations and returns immediately to the caller.
- `allocate_and_open_file_entry`: Allocates an entry for the indexed file slot and fills its fields and a name tag.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `split_text_into_argument_array`: Copies a text buffer and splits it into tokens separated by spaces, tabs or line breaks.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies a null-terminated string into a destination of a given length, stopping at the terminator, then fills the rest of the destination with zero bytes.
- `append_string_bounded`: Finds the terminating null of the destination, then appends the source string up to a limit of 32766 characters, writing a terminator if the copy was cut short.
- `append_string_with_limit`: Finds the end of the destination string, then appends the source string until its terminator or until the count in the third argument runs out.
- `copy_string_to_buffer`: Copies a null-terminated string from the source into the destination, including the terminator, and returns the original destination pointer.
- `multiply_32_bit_low_product`: Multiplies two 32-bit values from registers using 16-bit halves and returns the low 32 bits of the product in the upper half of the result.
- `open_named_library_into_struct`: Opens the library named by a global string through an exec routine, then looks up a second named library through it.
- `format_into_buffer_terminated`: Saves a destination pointer in shared state, calls a formatting routine with the second argument and the caller's stack arguments, then writes a zero terminator at the destination and returns the result.
- `format_into_buffer_word_result`: Does the same job as the terminated formatting wrapper, saving the destination pointer, formatting, and writing a zero terminator, but returns the formatter's result as a 16-bit value.
- `write_byte_to_output_buffer`: Writes the low byte of its argument to the current position of the shared output pointer, advances that pointer by one byte and returns the byte.
- `format_arguments_to_output`: Passes its argument and the caller's stack arguments to a shared formatting routine that uses the output callback, discarding the result.
- `execute_subroutines`: A routine consisting of forty-eight instructions that calls other routines and returns their results.
- `emit_printf_formatted_text`: Walks a format string, sending plain characters to an output callback and expanding each percent conversion.
- `negate_result_by_input_signs`: Takes two signed inputs and records whether their signs differ.
- `negate_result_if_first_negative`: Calls a routine and negates its second returned value when the first input is negative.
- `call_and_pass_through_second`: Calls a routine and returns its second register result together with the unchanged second input register.
- `forward_call_no_arguments`: Calls one routine with no arguments and returns.
- `divide_32_bit_quotient_remainder`: Divides two 32-bit values and returns the quotient and remainder together.
- `compute_string_length`: Walks a zero-terminated byte string until its terminator and returns the number of bytes before it.
- `forward_call_single_routine`: Calls one routine with no arguments and returns.
- `write_byte_to_buffered_stream_2`: Writes a single byte to a buffered stream by calling a routine that performs buffered byte writing operations.
- `stub_empty_return_callback`: Does nothing and returns immediately.
- `write_buffered_byte`: A routine consisting of fifteen instructions that calls other routines to perform byte writing operations.
- `write_byte_and_advance_pointer`: Reads a byte from a fixed offset within a base block, writes it to the destination pointer held in a register, advances that pointer by one byte and returns the byte.
- `call_routine_over_address_range`: Calls a routine for each address from a base-relative start up to a fixed limit, stepping 22 bytes each time.
- `flush_and_reset_stream_buffer`: Returns minus one for a null stream.
- `flush_stream_buffer_registers`: Operates on a stream block passed in a register: it flushes pending data when a flag is set, releases buffers if flagged, then clears the stream's pointer, count and flag fields and returns the flush result.
- `write_byte_to_buffered_stream`: Writes one byte to a buffered stream.
- `put_byte_via_register_stream`: Writes one byte to a buffered stream whose block is passed in a register.
- `write_byte_direct_or_buffered`: Writes one byte to a stream.
- `write_byte_with_stream_mode`: Writes one byte to a stream: it allocates a buffer if none exists, and when the stream's size field equals one it passes the byte through a write routine.
- `store_byte_in_stream_buffer`: Stores one byte at the stream's current buffer position, advances that position, sets the stream's buffered flag and returns the stored byte.
- `allocate_free_pool_entry`: Scans a fixed pool of 22-byte entries for one whose flag byte is clear, zeroes its first three words and returns it.
- `allocate_pool_entry_from_register`: Searches the same kind of 22-byte entry pool, starting from a pointer held in a register.
- `clear_pool_entry_words`: Zeroes the first three longwords of the entry whose address is held in a register.
- `init_stream_buffer_descriptor`: Tries to obtain a buffer.
- `init_shared_buffer_descriptor`: Sets the buffer size to 1024 and marks the buffer as allocated, pointing it at a shared buffer address held in shared state.
- `free_all_pool_entries`: Walks a linked list starting at a head pointer in shared state, releasing each node through a release routine with its size plus eight.
- `free_entries_list_from_register`: Walks a linked list whose head is held in a register, releasing each node through a release routine with its size plus eight, then clears the shared head pointer and returns zero.
- `allocate_tracked_memory_block`: Allocates a block of the requested size plus eight bytes through an allocation routine.
- `link_block_at_list_head`: Links a block held in a register at the head of a shared list, storing the previous head in its first word and its size in its second word.
- `forward_low_word_argument`: Passes the low 16 bits of its argument to a routine and returns.
- `unlink_and_free_tracked_block`: Searches the shared list for the block whose header matches the parameter.
- `unlink_block_by_register_key`: Searches a list whose head is held in a register for the block that matches a structure pointer.
- `unlink_node_from_list`: Removes a node from a list by writing its successor into the previous node, or into the shared head when the node was first.
- `pop_head_and_release`: Sets the shared list head to the successor of the node held in a register, calls a release routine and returns zero.
- `check_table_entry_and_call`: Checks that an index is in range and that its table entry is set.
- `test_table_entry_result`: Looks up a table entry from an index argument, passes it to a routine and returns true when that routine returns non-zero.
- `return_zero`: Returns the integer value zero.
- `delete_file_store_error`: Calls a delete routine with its argument.
- `return_zero_2`: The routine is empty and returns zero to the caller.
- `check_file_mode`: Tests a bit in a value returned by another routine and returns a negative value with an error code if certain mode bits are not set.
- `check_file_mode_then_call`: Returns minus one and records error code 5 when the handle's mode bits are clear.
- `write_record_store_error`: Calls a routine with the handle and two fields from the caller's data.
- `stub_empty_return_handler`: Does nothing and returns; it is an empty handler referenced by address.
- `poll_mask_bit_then_call`: Calls a routine with a fixed mask and tests bit 12 of its result.
- `execute_fixed_message`: Calls another routine to obtain a pointer and then writes the resulting data to an output stream.
- `write_fixed_message_to_output`: Obtains a pointer from a routine using a constant and a length of four, writes the result to the output stream through a routine, then calls a library routine.
- `call_optional_handler_then_routine`: Calls a stored handler when its pointer is set, then calls one routine and returns.
- `release_resources_and_return_status`: Frees a stored array after calling a routine for each of its entries, runs a stored cleanup handler and releases up to three further stored blocks.
- `release_resources_under_supervisor`: Loops over a table, releases the stored array and block, and runs a cleanup handler.
- `release_stored_list_blocks`: When a stored list pointer is set, releases its block through a release routine with a size computed from the stored count, and releases the secondary block.
- `call_cleanup_then_return_slot`: Calls one routine, passes a stored value to a second routine, and returns the value held in its saved stack slot.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `release_indexed_table_entry`: Checks that an index is in range and that its table entry is set, otherwise recording error code 2 and returning minus one.
- `release_entry_and_clear`: Calls a routine on the stored entry unless its flag bit is set, then clears the entry and returns zero.
- `call_stored_handler_slot`: Calls the handler stored at a fixed negative offset from a base pointer held in shared state.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_second_stored_handler`: Calls the handler stored at another fixed negative offset from the same base pointer.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock_handler_slot`: Calls a DOS-library function through a handler slot at a fixed negative offset from a base pointer.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_handler_slot`: Calls a DOS-library function through a handler slot at a fixed negative offset from a base pointer.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read_handler_slot`: Calls a DOS-library function through a handler slot at a fixed negative offset from a base pointer.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_unlock_handler_slot`: Calls a DOS-library function through a handler slot at a fixed negative offset from a base pointer.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_handler_slot`: Calls a DOS-library function through a handler slot at a fixed negative offset from a base pointer.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_supervisor_library_slot`: Calls a library routine through a handler slot at a fixed negative offset from a base pointer.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_init_record_entry`: Looks up a free index, returning zero if none exists, and allocates a cleared 34-byte record.
- `allocate_and_init_record_from_params`: Allocates a cleared 34-byte record through the allocator and fills it from the caller's fields: a value, a flag byte, a type code of 4 and a zero byte.
- `fill_record_entry_fields`: Fills an existing 34-byte record with a value, a flag byte, a type code of 4 and a zero byte, stores a value from a routine, then calls a routine on the record or on its trailing field.
- `process_record_trailing_field`: Passes the address of a field 20 bytes into a record held in a register to a routine, then returns.
- `release_record_entry`: Releases a 34-byte record.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_routine_with_constant_48`: Calls a routine with its argument and the constant 48, then returns.
- `release_record_block`: Calls another routine passing a pointer as an argument to free a block of memory.
- `allocate_type5_record`: If the given pointer is non-zero, allocates a cleared block through the allocator and stores a type code of 5, a word from the second argument and the pointer in it.
- `release_record_with_size_field`: If the record pointer is set, marks its type byte and two trailing fields as invalid and frees the block using the size stored in its word field, returning the free result.
- `call_library_slot_198`: Calls a library function through a handler slot 198 bytes below a base pointer held in shared state.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_slot_294`: Calls a library function through a handler slot 294 bytes below a base pointer held in shared state.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_slot_132`: Calls a library function through a handler slot 132 bytes below a base pointer held in shared state.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_slot_210`: Calls a library function through a handler slot 210 bytes below a base pointer held in shared state.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `initialize_list_head_pointers`: Initialises a list head in the block at its argument: the first word points four bytes past the head, the second word is cleared and the third word points back to the head itself.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_slot_552`: Calls a library function through a handler slot 552 bytes below a base pointer held in shared state.
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
- `initialize_shared_state_and_buffer`: Clears a large block of shared memory and initializes several global variables and pointers.

### Star Wars (1987)(Domark)[cr DF].adf:fighter

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Star Wars (1987)(Domark)[cr DF].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
