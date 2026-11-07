# Game 2276 (star wars (1987)(domark)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02276). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 37 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 425 of 425 named (0 author symbols), 425 of 425 described (50 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 32 of 32 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16583 of 16583 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 191316 |
| code | 54974 |
| data | 101335 |
| padding | 7703 |
| unused | 545792 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Stack | 2 | 117 | 2 |
| List | 2 | 117 | 2 |
| WB2 | 3 | 22 | 1 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Star Wars (1987)(Domark).adf:MFC | 2 | 57 | 1 |
| Star Wars (1987)(Domark).adf:starfix | 1 | 37 | 2 |
| Star Wars (1987)(Domark).adf:starwars | 3 | 15404 | 411 |
| Star Wars (1987)(Domark).adf:bootblock | 1 | 10 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_and_call`: If the pointer passed on the stack is not null, copies sixteen longwords from a fixed offset in a block addressed by an address register into the buffer it points to.

### Stack

- `write_max_value_header`: Scans a linked list of entries for the largest value in one field, starting from 149.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a stack address and then calls another routine.

### List

- `traverse_list_and_update_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a data structure by writing a header and copying blocks of memory.
- `copy_memory_block`: Copies sixteen bytes from a source address to a destination address via a loop.

### WB2

- `open_graphics_intuition_libraries`: Opens the graphics and intuition libraries through library calls, using the library names referenced in the file.

### Port-Handler

- `dispatch_hunk_type_records`: Initialises a block descriptor from a table of values and clears 31 longwords of a buffer.

### Disk-Validator

- `check_bitmap_bit_consistency`: Writes two text characters that show a number into a display buffer, then scans a range of bitmap longwords.

### Ram-Handler

- `copy_memory_via_library`: Calls a library function through a global base at a fixed negative offset, then makes an indirect call whose target the decompiler could not recover.

### Star Wars (1987)(Domark).adf:MFC

- `unpack_backward_rle_stream`: Clears a counter by counting down a 16-bit loop, then decodes a packed byte stream backwards into a destination buffer, expanding two marker bytes into runs of repeated bytes.

### Star Wars (1987)(Domark).adf:starfix

- `busy_loop_then_trap`: Runs a fixed-count busy loop that adds a constant to a global variable, then stores the address of the routine itself into that global and issues trap number zero.
- `swap_library_field_and_call`: Looks up a library base through a library call, saves a pointer field of the structure it returns and replaces that field with the address of a global.

### Star Wars (1987)(Domark).adf:starwars

- `setup_disk_dma`: Performs hardware timing loops, manipulates a status register, and configures DMA parameters and buffer pointers for a disk transfer.
- `forward_control_to_routine`: Makes a single call to one unnamed routine and then returns, with no arguments or other work.
- `copy_masked_flag_field`: When a byte field of the structure equals 1 and the high bit of another byte is set, writes the structure's word field, masked to its low seven bits, through the output pointer.
- `update_display_frame_state`: Swaps a pair of display buffer pointers when a flag is set and derives x and y deltas from a potentiometer-style input register.
- `update_slot_timers_with_delta`: Sets a latch byte to 255, then adds byte deltas taken from a word of a record to two position values, clamped to 0 to 319 and 0 to 199.
- `update_slot_timers_on_fire`: Sets the latch byte to 255 or clears it according to one bit of a global input word, then derives deltas from bit tests on a record field.
- `update_slot_timers_latched`: Sets a latch byte to 255, derives deltas from bit tests on a record field and clamps two position values to fixed ranges.
- `update_slot_timers_plain`: Derives deltas from bit tests on a record field without touching the latch byte, clamps two position values to fixed ranges and counts down four timers.
- `update_slot_timers_from_flags`: Derives position deltas from bit tests on an input word held in a register, clamps two position values to fixed ranges and counts down four timers.
- `update_slot_timers_on_bit`: Adjusts the vertical delta when one bit of an input register is set, clamps two position values and counts down four timers.
- `clamp_y_then_update_timers`: Clamps the horizontal position to zero when negative and clamps the vertical position to the range zero to 199, then counts down four timers.
- `clamp_low_y_update_timers`: Clamps the vertical position to zero when negative, then counts down four timers.
- `update_slot_timers_only`: Counts down four timers without changing any position values.
- `read_and_clear_word_flag`: Returns the word stored at a fixed global offset and resets that word to zero when it was non-zero.
- `open_handle_read_buffer_field`: Asks a library routine for a handle using a fixed argument of minus two.
- `read_library_buffer_field`: Points a stack buffer at a fixed location, stores a constant in it, passes it with a handle to a library routine and then the handle to a second library routine.
- `call_with_handle_or_fallback`: Requests a handle from a library routine using the code 1005 and the first argument.
- `call_fallback_library_stub`: Calls one library function at a fixed offset with no arguments and returns.
- `call_with_handle_alternate_call`: Requests a handle using code 1005 from the first argument.
- `call_indirect_address`: Calls a routine through a pointer stored at a specific offset from a base address.
- `call_handle_by_code_pair`: Requests a handle using code 1005, and when that returns nothing requests one using code 1006 instead.
- `call_handle_for_code_1006`: Requests a handle using code 1006 from the first argument, passes it with two stored arguments to a library function and then to a second library routine that takes only the handle.
- `emit_mirrored_position_pairs`: Calls one library routine to start, then steps a counter down to zero.
- `emit_stepped_mirror_pairs`: Issues pairs of library calls at positions mirrored about a value taken from a frame.
- `emit_mirror_pairs_loop`: Issues pairs of library calls at mirrored positions for each counter value, then moves the counter down, looping while it is non-negative.
- `program_blitter_for_planes`: Computes a byte address and a width from two pairs of coordinates, then for each of four bitplanes waits until the blitter is free and writes its size, modulo and address registers.
- `write_plane_state_per_bit`: Shifts a bit mask and, for each bit, stores zero or minus one in a block depending on the bit, along with a row address, a column value and a stride.
- `write_plane_rows_per_bit`: Loops over a bit mask, storing to a block a word that is zero or minus one depending on the bit, together with a row address, a column value and a stride.
- `advance_seeded_counter`: On first use, seeds a longword from a 16-bit global counter in its upper and lower halves.
- `access_dma_register`: Calls another routine that accesses the DMA control register.
- `access_dma_control_register`: The decompiler timed out, so this description rests only on the listed facts: the routine accesses the DMA control register and is reached through a pointer in data and from unnamed routines.
- `wait_until_flag_clear`: Spins while a word in a global slot is non-zero and returns once it clears.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `dispatch_pending_words_then_flag`: Calls one unnamed routine if a first pending word is non-zero and another if a second pending word is non-zero, then sets a global word to minus one.
- `call_two_routines_in_order`: Calls two unnamed routines one after the other and returns.
- `blit_five_plane_rows`: Computes a byte offset and two plane addresses from coordinate pairs, then runs five times: it waits until the blitter is free, writes two address registers and a control word, advances both addresses by 8000 and writes width and modulo values.
- `blit_plane_pairs_by_count`: Repeats a count held in a register: it waits until the blitter is free, copies two longwords and a word from a record into blitter registers, then advances two stored addresses by 8000 and decrements the count.
- `blit_plane_pairs_until_zero`: Copies two longwords and a word from a record into blitter registers, advances two stored addresses by 8000 and repeats after waiting for the blitter, stopping when the count reaches zero.
- `call_routine_with_fixed_arguments`: Calls one unnamed routine with a base address computed from a global slot plus 3720, a value from another slot, zero and the constant 45.
- `call_routine_with_row_arguments`: Calls one unnamed routine with an address derived from a global slot plus a row offset of two word slots multiplied by 40, a second slot value, zero and the difference of two word slots.
- `call_routine_with_region_args`: Calls one unnamed routine with a value from a global slot, an address derived from another global slot plus a row offset of two word slots multiplied by 40, zero, and the difference of two word slots.
- `request_block_or_fallback`: Asks a library routine for a block using the argument and the fixed code 65539.
- `read_longword_from_slot`: Returns the longword stored at a fixed offset of minus four from a base held in an address register.
- `stub_return_immediately`: The pseudo-code is a bare return with no operations, so the routine does nothing.
- `forward_two_args_to_library`: Passes its two arguments unchanged to a library function reached through a global base at a fixed negative offset.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `setup_interrupt_and_dma_state`: Calls two unnamed routines, a library function, and then two further unnamed routines with one library call between them.
- `store_library_result_slot`: Calls a library function with the constant 2 and stores its return value in a global slot.
- `setup_input_device_request`: Requests two values from a library routine using fixed codes and calls a fallback when either is zero.
- `pass_stored_blocks_with_sizes`: Walks a six-entry table, multiplying each entry by ten, and passes every non-null stored pointer with its size to one library function.
- `pass_stored_handles_to_library`: Passes a stored value to a library function, calls an unnamed routine, then passes two stored handles to a second library function.
- `copy_bytes_overlap_safe`: Copies a count of bytes between two buffers, walking backward when the source precedes the destination so overlapping regions copy correctly, and forward otherwise.
- `copy_bytes_forward`: Copies bytes one at a time from a source to a destination for a count held in a data register, advancing both pointers forward until the count reaches minus one.
- `copy_words_overlap_safe`: Copies a count of 16-bit words between two buffers, walking backward when the source precedes the destination so overlapping regions copy correctly, and forward otherwise.
- `copy_words_forward`: Copies 16-bit words one at a time from a source to a destination for a count held in a data register, advancing both pointers forward until the count reaches minus one.
- `compute_absolute_value`: Returns the absolute value of a signed 16-bit argument by negating it when it is below zero and returning it unchanged otherwise.
- `compute_absolute_longword`: Returns the absolute value of a signed longword argument, negating it when it is below zero and returning it unchanged otherwise.
- `multiply_then_divide_packed`: Multiplies the two 16-bit halves of the first argument and divides the product by the low 16-bit half of the second.
- `sum_of_word_squares`: Returns the sum of the squares of the two 16-bit halves of its argument, computed as signed values.
- `multiply_two_words`: Returns the product of the two 16-bit halves of its argument, computed as signed values.
- `sample_three_slot_pairs`: Calls one unnamed routine twice for each of three global slots and stores each returned word in a neighbouring global word, six results in total.
- `shift_word_history_after_calls`: Calls an unnamed routine three times, then saves a word from one global slot, calls another routine and stores its returned word in a second global slot while moving the saved word into the first.
- `sample_into_pair_buffer`: Calls an unnamed routine twice with the same global slot, then calls a third routine, and stores the returned word and the first argument into the two words of the buffer passed in.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `lookup_quarter_wave_table`: Adds 360 to an angle argument, reduces it modulo 1440 and folds the result into a quarter-wave using symmetry.
- `lookup_folded_table_value`: Reduces an angle argument modulo 1440 and folds it into a quarter-wave using symmetry.
- `scale_four_offsets_by_divisor`: Adds 600 to a divisor argument, then stores four values, each a sum or difference of global offsets and arguments shifted left by nine and divided by that divisor.
- `scale_two_offsets_by_divisor`: Stores two values, each a global offset plus an argument shifted into a high word and divided by a divisor formed from another argument plus 600.
- `add_packed_bcd_digits`: Adds a four-byte argument into a four-byte packed BCD number, one byte at a time from the lowest, adjusting each byte back to decimal after the addition.
- `add_bcd_counter_track_max`: Stores its argument in a global longword, adds four bytes of that value into a packed BCD counter with decimal adjustment, and raises a stored maximum to the counter when the counter is larger.
- `increment_bcd_byte_then_call`: Stores one in a global byte, adds it into a packed BCD byte with decimal adjustment, then calls an unnamed routine.
- `project_points_perspective`: Projects fifty three-dimensional points from a table onto the screen with a perspective divide by depth plus 600, adding a centre offset passed in as a parameter, and writes the resulting 2D coordinates into an output array.
- `store_two_words_from_globals`: Calls one other routine, then stores two 16-bit words taken from a global data area at the destination address held in an address register.
- `offset_records_by_scaled_value`: Adds scaled multiples of one value to three fields of each record in a 16-bit table over three passes, and sets three flag bytes by testing a computed value against minus 500.
- `emit_stepped_coordinate_pairs`: Walks a pair of 16-bit tables, writing coordinate pairs offset by plus or minus 30 for each step, and continues while a called routine returns a nonzero value.
- `project_sixty_records_perspective`: Sets loop counts from a mode argument, optionally copies three-word entries through called routines, then either clears three flag bytes when a global value is below minus 469 or projects a table of sixty records to 2D coordinates by dividing by depth plus 600.
- `call_one_routine_undecoded`: The decompiler timed out and produced no pseudo-code, so the body cannot be read; from the listing alone it has 22 instructions, is reached through a handler pointer, and makes one call to another routine.
- `transform_and_project_vertex_groups`: Transforms groups of 3D vertices by adding and subtracting a scaled global value from their coordinates, then projects each vertex to 2D by dividing by depth plus 600, repeating over the list of groups until its counter expires.
- `copy_records_and_duplicate_entries`: Copies 16-bit coordinate records from a table into an output buffer, pairing each with a value from a called routine, then duplicates six earlier entries and, when the argument is nonzero, fills further entries with a global value offset by 300.
- `store_argument_low_word`: Stores the low 16-bit word of its argument into a fixed global variable and returns.
- `select_four_flag_offsets`: Tests four bits of its argument and, for each set bit, replaces a default constant with an alternative value; it writes the resulting values into several global locations and sets four flag bytes to match.
- `store_inputs_boundary_difference`: Saves two 16-bit inputs into global variables, then stores a value derived from their differences to four boundary globals, each difference doubled, into another global.
- `compute_boundary_difference_value`: Computes a value from the differences between two inputs and four boundary globals, each doubled, and stores it in a global variable.
- `clip_point_first_edges`: Tests the edge flag bits of a point and, for the flagged edges, computes the crossing coordinate by a scaled division; it returns the crossing when the result lies within the edge's boundary range and otherwise returns the original value.
- `clip_point_remaining_edges`: Tests the edge-two flag and then the four and eight edge flags of a point, computing a crossing coordinate by scaled division for each flagged edge and returning it when it lies within the boundary range, otherwise returning the original value.
- `clip_point_last_edge_pair`: Tests the four and eight edge flags of a point and, for the flagged edge, computes the crossing coordinate by scaled division, returning the crossing when it lies within the boundary range and otherwise the original value.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `draw_clipped_line_bitplanes`: Clips a line segment's endpoints with called routines, then draws it into four bitplanes spaced 8000 bytes apart, setting one pixel bit per step and filling horizontal spans with per-plane colour bytes.
- `draw_line_stored_endpoints`: Takes its endpoints from stored globals, clips them with called routines, and draws the resulting line into four bitplanes, setting pixel bits and filling horizontal spans with per-plane colour bytes.
- `draw_line_bitplanes_direct`: Draws a line between two supplied endpoints into four bitplanes spaced 8000 bytes apart, setting pixel bits along the major axis and filling horizontal spans with per-plane colour bytes, without calling clipping routines.
- `fill_pixel_column_bitplanes`: Sets one pixel bit in each of four bitplanes spaced 8000 bytes apart for a vertical run of rows, stepping through the bitplane buffer by a row pitch taken from the arguments.
- `set_pixel_column_from_pointer`: Sets one pixel bit in each of four bitplanes spaced 8000 bytes apart for a vertical run of rows, starting at a buffer address passed in an address register and stepping by a row pitch.
- `draw_slanted_line_bitplanes`: Draws a sloped line into four bitplanes spaced 8000 bytes apart, stepping along the major axis and advancing the row whenever a minor-axis error term is used up, setting one pixel bit per step.
- `draw_slanted_line_at_pointer`: Draws a sloped line into four bitplanes spaced 8000 bytes apart, starting from a buffer address passed in an address register and advancing rows with an error term that tracks the minor axis.
- `fill_horizontal_span_bitplanes`: Sets pixel bits for a horizontal run across four bitplanes spaced 8000 bytes apart, writing partial bits at the ends and filling whole bytes in between with per-plane colour bytes taken from globals.
- `fill_horizontal_span_at_pointer`: Sets pixel bits for a horizontal run across four bitplanes spaced 8000 bytes apart, starting at a buffer address passed in an address register, filling whole bytes with per-plane colour bytes and partial bits at the ends.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `draw_sloped_span_bitplanes`: Draws a sloped line into four bitplanes spaced 8000 bytes apart by plotting short runs of pixels on each row, advancing one row at a time and shifting the run across columns according to an error term.
- `draw_sloped_span_at_pointer`: Draws a sloped line into four bitplanes spaced 8000 bytes apart by plotting short runs of pixels row by row, starting from a buffer address passed in an address register and using an error term to step across columns.
- `store_pair_then_call_routine`: Stores the two 16-bit halves of its argument into two global variables and then calls another routine.
- `store_pair_return_registers`: Stores the two 16-bit halves of its argument into two global variables, calls another routine, and returns the incoming data registers unchanged.
- `plot_pixel_if_in_bounds`: Plots one pixel into four bitplanes spaced 8000 bytes apart, but only when the point lies inside a rectangle given by four limit globals; the row address is computed at 40 bytes per row.
- `set_pixel_bit_four_planes`: Sets one pixel bit, selected by a bit number from a data register, at a buffer address in four bitplanes spaced 8000 bytes apart.
- `clear_display_longwords_backward`: Zeroes 32-bit words in display memory by writing backward from a row address computed from two globals, using an unrolled sequence inside a nested loop; the decompiled listing is truncated, so only the start of the clearing sequence is evident.
- `call_table_quotient_sequence`: Runs a sequence of indirect calls through a global jump table, each passing the quotient of a table entry divided by a value from an earlier call, with fixed loops of thirty, fifteen and nineteen steps.
- `project_strip_quads_and_plot`: Projects up to twenty-four 3D points by perspective division with depth plus 600, expands each into a strip of four coordinate pairs in a buffer, then calls another routine for each buffered point that lies within the horizontal bounds.
- `print_menu_labels_fixed_positions`: Calls a sequence of routines with fixed coordinate pairs, then indirect routines; the referenced text includes select a death star, easy, medium, hard, wave 1, wave 3, wave 5, bonus and no bonus, so it lays out labels at fixed positions.
- `update_countdown_select_mode`: Converts a timer global into a countdown digit character, draws the countdown text at fixed positions, then polls an input value to choose one of three modes, storing the choice and resetting a flag; if no choice is made it draws a fixed sequence of symbols instead.
- `run_prompt_state_loop`: Draws two positioned text items, then runs a three-state loop until the state reaches its end value, drawing the prompt text that tells the player to pull the trigger to start; the loop is preceded and followed by display-pointer swaps.
- `wait_frames_until_state_change`: Runs up to 500 frame iterations while a state variable stays zero, calling update routines on each pass; afterwards it returns minus one if the state is still zero, otherwise a flag showing whether the state is not four.
- `build_point_table_from_lookup`: Builds a table of 128 points by passing each source value through two indirect lookup calls, shifting the results right, adding offsets of 100 and 2000, and storing the pairs in an output buffer, then calls another routine.
- `draw_credit_text_lines`: Draws the publisher credit, the Amiga version credit, the title text and one further element at fixed positions, using the referenced text lines as the displayed strings.
- `draw_intro_crawl_lines`: Draws eight lines of the opening text at successive vertical offsets from a base row, using the referenced lines about the Empire, the rebel planet and Obi-Wan Kenobi, then draws one further element at a computed position.
- `stub_timeout_forty_seven`: The decompiler timed out and produced no pseudo-code, so the body cannot be read; the listing shows 47 instructions and two callers, and nothing more can be said of its behaviour.
- `offset_point_table_lookup`: Derives two offsets from a fixed-point lookup, adds them to each of 128 entries in a point table, and writes a pair of global words into an output buffer for each entry; it also increments a global when its argument exceeds ten and has its low two bits set.
- `stub_timeout_thirty_instructions`: The decompiler timed out and produced no pseudo-code, so the body cannot be read; the listing shows 30 instructions and a single caller, and nothing more can be said of its behaviour.
- `draw_shape_vertex_pairs`: Passes many coordinate pairs from a shape table to indirect drawing routines, offsetting the second value of each pair by a global, so that the table is drawn as an outline; it is preceded by calls that set up the drawing state.
- `run_timed_scroll_sequence`: Runs three timed sequences of frames, each waiting for the frame counter to advance while calling update routines; one pass steps a value from 100 down to minus 200, another from minus 50 up to 200, and a third from 100 down past minus 370, returning early with a nonzero state code if one is raised.
- `classify_phase_from_counter`: Calls a routine with its argument, then returns a phase code: 1 when a counter is below 20 and the mode is not one, 0 or 3 depending on a flag when the counter is below 300 or the mode is two, and 2 otherwise.
- `update_particle_records_respawn`: Updates seventy three-word records by adding a velocity to each position, respawning a record with random values when it crosses a limit, and calls a drawing routine for each record; the argument selects whether the vertical or horizontal limits are tested.
- `randomize_seventy_particle_records`: Fills seventy three-word records with random values, storing the high byte of two random numbers as positions and a value from one to eight as a speed.
- `draw_briefing_text_by_mode`: Selects a display position from a mode number between zero and sixteen, then draws the matching briefing text line through a text routine; the referenced text covers the X-Wing shields, deflector strength, Trench Turrets, the Death Star and the flight instructions.
- `draw_scoring_table_labels`: Draws a scoring table of target names, including Tie Fighters, Laser Towers, Laser Bunkers, Trench Turrets, Exhaust Port and Darth Vader Ship, with their point values such as 1,000, 2,000, 25,000 and 50,000, at fixed row offsets, then draws one final element.
- `run_scene_frames_stepping`: Runs up to 768 frames of a scene, stepping a position parameter by 256 every five frames and calling update routines each frame; it stops early and returns a nonzero code if an input routine reports one.
- `run_scene_frames_stepping_copy`: Runs up to 768 frames of a scene with the same stepping loop as its sibling routine, stepping a position parameter by 256 every five frames and returning early with a nonzero code when an input routine reports one.
- `run_scene_frames_descending`: Runs a scene for frames while a position counter descends from 110 to just above minus 360, stepping a second parameter, calling update routines each frame, and returning early with a nonzero code when an input routine reports one.
- `store_word_in_slot_table`: Stores the second 16-bit word of its argument into one of sixteen word slots, the slot being chosen by the low four bits of the first word.
- `wait_until_frame_count`: Resets a frame counter global to zero, then loops until the counter reaches the value passed in the argument.
- `compute_integer_square_root`: Computes the integer square root of its 16-bit argument by Newton-style iteration, starting from the argument divided by 200 plus 2 and refining the estimate by division until it stops changing.
- `nop_padding`: This routine is empty and performs no operations.
- `set_default_small_bounds`: Sets three global values to 15, minus 15 and 15, which act as small limit values for later drawing or clipping.
- `set_default_screen_bounds`: Sets six global values to fixed limits: 160 and 115 as dimensions, horizontal bounds from minus 160 to 159, and vertical bounds from minus 84 to 84, which define the drawing area.
- `advance_depth_records_direction`: Moves thirty-one three-word records along depth by subtracting a speed argument, adding 14500 to any record that drops below minus 500 or whose depth passes the range in the other direction, and draws each record through called routines when a depth test passes.
- `advance_depth_records_counted`: Moves a counted run of three-word records along depth by a speed value, wrapping any record that leaves the lower depth range by adding 14500, and draws each record through called routines when a depth test passes.
- `advance_depth_records_positive`: Moves thirty-one three-word records along depth by subtracting a speed value, wrapping each record that exceeds 14000 by subtracting 14500, and draws each one through called routines when a depth test passes.
- `advance_depth_records_counted_wrap`: Moves a counted run of three-word records along depth by subtracting a speed value, wrapping any record above 14000 by subtracting 14500, and draws each record through called routines when a depth test passes.
- `advance_records_with_rotation`: Moves thirty-one three-word records along depth, flipping the sign of their two velocity words when they wrap; when the argument is positive it also adds a scaled global offset to the two position words, wrapping them within a 4096 range, and draws each record.
- `advance_rotating_records_counted`: Moves a counted run of three-word records along depth by subtracting a speed value, flipping the sign of their velocity words when they wrap, adding two offsets and masking the position words within a 4096 range, then draws each record.
- `advance_depth_records_mirrored`: Moves thirty-one three-word records along depth by subtracting a speed value, wrapping any record above 14000 by subtracting 14500 and flipping the sign of its two velocity words, then draws each record through a called routine.
- `advance_counted_records_mirrored`: Moves a counted run of three-word records along depth by subtracting a speed value, wrapping any record above 14000 by subtracting 14500 and flipping the sign of its two velocity words, then draws each record through a called routine.
- `sync_frame_and_draw_pair`: Resets the frame counter, calls two routines with its argument and then two further routines, and waits until at least one frame has passed before returning.
- `call_single_table_routine`: Calls one routine through a jump table and returns, with no other work.
- `remove_expired_list_entries`: Walks backward through a list of six-byte entries, calling a routine for each entry whose key is at least the argument and reducing a count by six, then stores the new list end.
- `spawn_list_entry_when_ready`: Advances through a list of entries while their keys stay above a threshold; when a free entry and a random test allow, it initialises the entry from a fixed table, sets a flag and returns minus one, otherwise it returns zero.
- `insert_sorted_list_entry`: Finds the insertion point for a key in a sorted list of six-byte entries, increments the list count, moves the later entries with a block-move routine, and returns the address of the new slot.
- `build_entry_projection_buffer`: Converts each entry of a depth-sorted list into a three-word output record holding two global values and a perspective scale of 25600 divided by depth plus 600, and draws each record when the argument is nonzero.
- `update_particle_list_expire`: Moves each entry of a particle list by subtracting a speed, applying two velocity words to its coordinates with wrapping within a 4096 range, then removes entries that have passed the depth or vertical limits by calling a removal routine.
- `draw_six_point_ring_shape`: Derives three scaled offsets from a rotation-style table and draws a six-point ring of segments around a centre point, drawing each segment through called plotting routines; a negative argument selects a variant that sets a colour for each segment.
- `update_difficulty_parameters`: Derives three parameters from a level counter, capping them at fixed values once the counter reaches seven, then calls one routine and increments the counter up to 99.
- `stub_timeout_ten_instructions`: The decompiler timed out and produced no pseudo-code, so the body cannot be read; the listing shows 10 instructions, a pointer reference and a single caller, and nothing more can be said of its behaviour.
- `send_word_sequence_to_routine`: Passes each 16-bit word of a table to a called routine in turn, advancing the table pointer and stopping after the count supplied in a data register, then returns the next table address.
- `call_two_routines_with_arguments`: Passes its two arguments to two separate called routines, one each, and returns.
- `call_eleven_routines_in_sequence`: Calls one routine eleven times in succession, with no other work, and returns.
- `run_timed_main_loop`: Clears position and speed globals, then runs up to 1500 frames, drawing each frame through called routines and reducing a speed value by 250 while it exceeds 1000; it then waits for pending items to clear and finishes with a final call.
- `run_frame_loop_until_flag`: Repeats frame updates through called routines, reducing a speed value by 250 while it exceeds 1000, until a status routine returns nonzero, a flag is set or 1500 frames pass; it then clears the flag and waits for pending items to clear.
- `wait_for_pending_items_clear`: Clears a flag, repeatedly calls a routine until it returns zero and the flag is zero, then makes one final call with a global argument.
- `draw_zigzag_marks_scaled`: Draws a zigzag pattern of short segments whose length scales with its argument, moving between fixed offsets from a centre point through called plotting routines.
- `draw_shield_status_display`: Draws the shield status by choosing one of three bar settings from the shield value, drawing either the shield text or the shield gone text, decrementing a blink counter, and then drawing the remaining status element.
- `adjust_shield_value_state`: When the state is one, adds its argument to a shield value; a negative argument instead resets a set of counters and returns if the shield value is depleted, otherwise it loads a countdown from the shield value and sets a display step of ten.
- `start_shield_countdown`: Copies the shield value into a countdown state, sets a fixed step of ten, adds a value from a caller-supplied record to the shield value, and calls one routine.
- `handle_pause_command_code`: Handles a command code: 64 pauses the game by clearing pending input, saving a state byte, drawing the Game Paused text and waiting for input before restoring; 80 and 81 set and clear a flag, and 82 toggles a byte.
- `draw_status_overlay_shapes`: Draws status overlay symbols, choosing between two line shapes by a state value and toggling a small shape pattern on each frame, then draws two decrementing counter markers and a fixed outline at positions relative to a base row.
- `draw_status_overlay_store_result`: Draws the status overlay: in state three it toggles a small shape pattern, decrements two counter markers, draws a fixed outline, then stores a called routine's result in a record and calls another routine when it is nonzero.
- `draw_fixed_shape_overlay_store`: Draws a small fixed shape at a position passed in a data register, decrements two counter markers, draws a fixed outline, then stores a called routine's result and calls another routine when it is nonzero.
- `draw_counter_outline_overlay`: Draws one fixed symbol at a fixed position, decrements a counter marker, draws a fixed outline, then stores a called routine's result and calls another routine when it is nonzero.
- `draw_outline_store_result`: Draws a fixed symbol and a fixed outline at positions relative to a base row, then stores a called routine's result in a record and calls another routine when it is nonzero.
- `update_damped_motion_values`: Derives a display offset from a counter and a direction flag, then moves two motion values toward target inputs with damping, clamping them to fixed limits; when a flag is set it resets a countdown timer, toggles a byte and calls a routine.
- `update_damped_motion_setup`: Stores two derived values in a record, computes a scaled difference, sets three lookup values, damps two motion values with clamping, and when a flag is set resets a timer, toggles a byte and calls a routine.
- `damp_motion_values_clamped`: Clamps one motion value to at most 270, doubles and clamps another to within plus or minus 320, damps both toward target values, and when a flag is set resets a countdown and toggles a byte.
- `damp_motion_upper_clamp`: Clamps a motion value to at most 320, damps two motion values toward target values, and when a flag is set resets a countdown and toggles a byte.
- `reset_countdown_and_call`: Sets a countdown timer to seven and calls one routine with a zero argument.
- `draw_packed_digit_string`: Draws up to eight digits taken from successive four-bit groups of a 32-bit value, calling a digit routine for each one and placing a separator before every fourth digit.
- `tick_timer_flash_message`: Decrements a countdown timer and, when it reaches zero, draws a message at a fixed position twice with a display buffer swap in between, then calls another routine.
- `draw_timed_message_block`: Draws two fixed text items on a fixed row, starts a countdown of ten, then draws a further fixed-size block at a fixed position through a buffer copy routine.
- `draw_count_symbols_block`: Draws one or two copies of a symbol depending on a count byte, then draws a fixed-size block at a position in the display area.
- `draw_two_digit_count_symbols`: Draws one or two copies of a symbol depending on whether its argument is at least ten, then draws a fixed-size block at a fixed position.
- `draw_five_offset_markers`: Derives a scale and a centre offset from two position values, then draws five markers at offset positions taken from tables, placing each through a routine that draws it at the given coordinates.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `build_angle_lookup_tables`: Advances a base angle by 120 and fills several 16-bit tables from two indirect lookup calls: six entries at successive offsets of 120 plus a few fixed entries, with the second table taking the other lookup's results.
- `plot_points_decreasing_steps`: Plots points at a fixed height of 250 across the horizontal range, with the horizontal step shrinking through the sequence 46, 36, 25, 18, 11, 7 and 4, calling a plot routine for each point.
- `draw_glyph_by_code`: Draws a small glyph chosen by a code from zero to nine, plotting a fixed sequence of points relative to a position and updating two cursor values for each stroke.
- `draw_vector_text_string`: Draws a zero-terminated string one character at a time from a vector glyph table, advancing the cursor by a width for each space and plotting the strokes of each glyph through called routines.
- `pick_random_entry_and_draw`: Chooses a random entry from a table of three-byte records within up to thirty attempts, then draws it as a text string or a glyph depending on its code, using a buffer when a flag bit is set; it also draws a fixed entry when a counter exceeds two.
- `process_initials_key_input`: Handles one key code for an initials entry field of up to three characters: a letter or space is appended while there is room, code 3 removes the last character, code 2 adds a space, and other codes are rejected; it returns minus one while a frame counter is below 1501 and zero otherwise.
- `draw_counter_text_item`: Derives a horizontal position from a counter, draws a text item through a lookup call, then clears a flag and calls one routine when the state flag was set.
- `draw_high_score_table`: Draws a table of ten rows with rank numbers, initials and scores in formatted fields, highlighting the row of a new entry and drawing messages from the referenced text; when a new entry is pending it reads initials through a key-input routine, otherwise it waits for about 500 frames.
- `insert_random_total_into_table`: Sums nine values from a random-number routine, then inserts the total into a ten-entry sorted table by shifting lower entries down, and calls one routine after placing it.
- `insert_shifted_total_into_table`: Accumulates values from a random-number routine while shifting a running value right by four bits on each pass, then inserts the total into the same ten-entry sorted table and calls one routine.
- `insert_into_sorted_table_tail`: Scans down a ten-entry table for the first entry the total is not below, shifts the remaining entries down with a block-move routine, stores the total in the freed slot, and calls one routine.
- `open_system_libraries_setup`: Opens system libraries and creates device and timer request structures through library vector calls, setting their fields and issuing requests; it calls an error routine whenever an open or allocation returns zero.
- `clear_flag_and_call_library`: Clears a global variable to zero and calls one routine with a fixed 32-bit constant, which is a library vector call at a fixed offset from a loaded library base.
- `initialize_data_structure`: Modifies bits in global variables and populates a table with several constant values and a pointer.
- `reset_level_state_draw_hints`: Resets scoring, timer and motion globals for a new round, sets a speed-dependent value from the level counter, and when a flag is set draws the Shoot Fireballs and Shoot Tie-Fighters hint text at fixed positions.
- `advance_records_by_rotation_offsets`: Moves thirty-one three-word records along depth by subtracting a fixed step, flipping velocity signs when a record wraps, rotating its two position words by offsets masked to a 4096 range, and draws each record through a called routine.
- `randomize_object_position_fields`: Fills three position fields of an object record with values from two lookup calls, each shifted right by two bits, after applying a fixed offset to a size field.
- `spawn_random_object_slot`: When the object count is below its limit and a timer threshold is met, claims the next record, fills its position and velocity from random values, sets its lifetime and flags, and draws a lookup entry for that lifetime.
- `update_active_object_list`: Updates each active object in a list: counts down its timers, moves it by its velocity, applies random speed and shot adjustments when timers expire, and removes it from the list when it passes the lower depth limit, then updates the list-level state.
- `launch_object_projectiles`: For each active object inside the target bounds, when a random roll passes, claims a free projectile record and sets its position, a direction flag and velocity components scaled by the object's distance.
- `call_alternating_pair_sequence`: Makes a fixed sequence of thirteen calls that alternate between two routines, with no other work, and returns the incoming data registers unchanged.
- `draw_record_outline_shape`: Draws a fixed outline shape by stepping through points taken from an object record's coordinate fields, plotting them with two routines, and then draws a short second stroke.
- `draw_object_sprite_strokes`: Draws an object's sprite from flags in its record: one or two strokes offset from a base position, then, when a further flag is set, four blocks of five values and five more entries through called routines.
- `check_object_hits_and_draw`: Checks each active object against target bounds and a random chance; on a hit it reduces the object count, plays a hit effect through a call and adjusts its timers, then updates the list pointers and draws each object.
- `remove_random_object_in_range`: Scans the object list for the first record within the depth and screen bounds that passes two random tests, removes it by moving the rest of the list down, decrements the count and calls a routine.
- `update_hud_counters_and_flags`: Derives display values from two status globals and sets two lookup values; once the level counter reaches 100 it subtracts scaled rates from two running counters, and when a flag is set it toggles a byte, resets a countdown and calls a routine.
- `draw_timed_marker_blink`: While a flag is set and a frame counter is below 320, draws a small marker whose vertical position depends on a status bit, then advances the counter.
- `draw_score_readout_values`: Draws a score or status readout: below level 111 it draws a value from a lookup with a decreasing offset, and otherwise it converts two running totals into lookup values and draws the resulting figure at a fixed position.
- `draw_active_object_markers`: Draws one marker for each active object at a fixed level of 300, using each object's stored value for position and stepping through the object records.
- `run_round_until_end`: Runs frames of a round while a state flag is zero and a timer stays non-negative, waiting for each frame to pass; when the state flag is set it performs an end-of-round action, clears a pending flag, and returns the state flag value.
- `scale_lookup_by_shift`: Passes its argument to one lookup call, takes a second lookup of the constant 10000, shifts that result right by 13 bits, and returns the value.
- `call_single_routine_passthrough`: Calls one routine and returns with no other work.
- `append_pair_to_buffer`: Appends its two 16-bit halves as two longword entries to a buffer, advancing the buffer pointer by eight bytes.
- `build_geometry_table_entries`: Fills a table of longword pairs from lookup calls: thirty-one pairs stepped by 48, ten further entries, then sixteen pairs stepped by 96 that receive offsets of 7500 and minus 10000, followed by twenty-one routine calls with fixed constants.
- `initialise_random_record_table_spread`: Initialises thirty-one three-word records with a horizontal position starting at minus 259 and stepping by 483 per record, and random vertical and depth values each shifted right by 12 bits.
- `delay_after_ten_calls`: Calls one routine ten times with the same global argument, then spins in a countdown loop of two million iterations before returning.
- `clear_and_load_shape_records`: Clears the first longword of six records, then calls a loader six times with the addresses of those records; the referenced file names are laser, nose and star shape files.
- `load_resource_into_record`: Looks up a resource by identifier through a library call, stores the returned handle and a word argument in a record, obtains the resource size, and calls another routine to load it.
- `load_sound_assets_pack_digits`: Loads the sound and music resources by name, including the Death Star, exhaust, hit, laser and missile sounds and the music file, clears an eleven-entry table, and converts an eight-character number into four packed-decimal bytes.
- `allocate_buffers_init_tables`: Clears a set of global pointers, allocates buffers of 32000, 4000, 1000 and 800 bytes through a memory routine, and fills a sixteen-word table with fixed values that include pairs of repeated nibbles.
- `read_line_with_return_prompt`: Clears a 31-entry input buffer, reads up to eighty characters through a library call, draws the prompt lines that mention the Return key and a list, and returns.
- `reset_game_state_globals`: Sets a large set of game globals to defaults, including speed and position values, a wave value of 17 and a timer of eight, clears a 31-entry table, then calls routines to reset the display area.
- `draw_score_wave_headers`: Sets game flags and resets two counters, then draws the score and wave headers through called routines, clearing a status area and drawing two further labels at a fixed position.
- `wait_for_input_event_loop`: Loops reading input events through called routines until one of them returns zero, then calls a finishing routine and returns, polling a second set of inputs while it waits.
- `load_image_record_from_file`: Opens a resource by identifier, reads a ten-byte header to derive width, height and data size, allocates a buffer, reads the image data into it, closes the resource, and fills a record with the size, offsets and buffer pointers.
- `load_sprite_planes_with_masks`: Loads a sprite resource and rearranges its rows into interleaved bitplane buffers, writing an all-ones mask word before each row and filling the mask and plane buffers for every sprite row.
- `blit_clipped_masked_sprite`: Clips a sprite against the screen bounds, then programs the blitter control and size registers for a masked copy of its bitplanes with the computed modulo values, waiting for the blitter to be free before each start.
- `load_sound_sample_tables`: Loads the trumpet sound resource, builds five sample pointer and length tables, fills a table with twelve note period values and twelve random values, and fills a 120-entry table with scaled lookup results.
- `locate_sample_chunks_in_file`: Reads a chunked resource and walks its chunk headers, skipping odd-length padding, to record the data pointers and sizes of two sample chunks.
- `advance_music_channel_tick`: Runs one music tick for two channels: when a channel's counter expires it reads the next two sequence bytes, sets the period and volume from the note table and starts a sample with a duration from a lookup table, otherwise it decrements the counter.
- `play_music_note_channel_zero`: Handles one note event for audio channel zero: for a non-rest note it looks up the sample address, length and period, programs the channel's address, length, period and volume registers, pauses briefly, and then advances the channel timing and reads the next sequence bytes.
- `advance_music_sequence_tick`: Advances the music sequence timer: when it reaches zero it reads the next two sequence bytes and sets the following event duration from a lookup table, otherwise it decrements the timer and writes a short follow-up value to the channel registers.
- `play_music_note_channel_one`: Handles one note event for audio channel one: for a non-rest note it looks up a period in the note table and programs the channel's length, period and volume registers, then computes the note duration from a lookup table.
- `update_music_state`: Checks a value in a table and, if it is less than three, updates a global variable with a value from another memory location.
- `initialise_music_playback`: Resets both music sequence pointers to their starts, clears the timer and duration globals, enables the audio DMA channels and interrupts, and marks playback as active.
- `stop_music_playback`: Writes a value of three to an output global and clears the playback-active flag, which stops music output.
- `set_playback_active_flag`: Sets the playback-active flag to one, so that the music routines treat playback as running.
- `read_playback_active_flag`: Returns the current value of the playback-active flag.
- `map_code_to_routine_result`: Takes a code and calls one routine for the codes 0, 2, 4, 6, 8, 12 and 19, returning that routine's result; for any other code it returns the code minus 19 without calling anything.
- `initialise_round_state_and_layout`: Resets round state, derives scoring values from a level counter, fills 26 records with random positions, loads a level layout from a table, counts marker entries, and for later levels draws the tower-clearing bonus text and status shapes at fixed positions.
- `update_and_compact_entry_list`: Moves each entry in a list by a speed and an acceleration, adds a random jitter to some entries, then removes entries that fall below a threshold by compacting the list.
- `scroll_entry_ring_respawn`: Scrolls a ring of records by a random step, appends a fresh record when the leading record passes a threshold, rotates a five-entry group between two sets, and periodically refreshes a pattern table from a stored list.
- `generate_row_entries_both_sides`: Generates entries for five rows by scanning from each side of the screen and placing an entry wherever a random test passes, storing its position, type and a value derived from its depth.
- `draw_typed_object_pieces`: Draws an object according to its type code through a sequence of plotting calls, then for each of four sub-parts within depth range projects a point and draws that part at the computed scale.
- `spawn_marked_entries_from_list`: For each entry of a type below three that is not already marked, when a random test passes, marks it and creates a new entry at an offset position within the list's range.
- `check_entry_hits_and_draw`: Checks the entries of a list against a target position and type, and on a hit marks the entry, plays a sound effect through a call and sets a flag; it then draws the entries at their positions.
- `claim_nearby_entry_when_clear`: Walks the list of records and removes the first one ahead of a threshold that passes two random tests, starting a timer; when a flag is clear it also scans for a secondary condition on the remaining records.
- `update_scroll_offset_and_lookup`: Adjusts a scroll offset that eases toward a target or accelerates while a flag is active, then looks up two values at that offset and stores them as scaled offsets.
- `update_scroll_velocity_damped`: Sets motion globals from a level counter, damps a velocity value toward a target, and when a flag is set toggles a byte, resets a countdown and calls a routine.
- `draw_blinking_indicator`: Draws a blinking indicator at a fixed position: when a timer is not running it draws a symbol on alternate frames, otherwise it chooses one of two symbols from a status bit, and it advances the frame counter.
- `draw_frame_with_optional_overlay`: Draws the background and foreground layers through called routines, then draws two overlay items when the level is above two, a counter is below 500 and a flag is clear, and finishes with a final call.
- `run_wave_intro_sequence`: Returns zero on the first level; on later levels it runs a sweep of frames while a position value exceeds 5000, easing a scroll value and drawing each frame, then waits for a frame condition, draws two items, and returns the flag value.
- `project_two_depth_offsets`: Divides two stored 32-bit values by two depth-dependent denominators offset by 470 and 730 units, and stores the four 16-bit quotients in globals.
- `project_offset_pair_by_depth`: Divides the argument offset by 30720 above and below, using two depth-dependent denominators offset by 470 and 730 units, and stores the four quotients in globals.
- `project_triple_values_by_depth`: Divides three stored 32-bit values by two depth-dependent denominators offset by 450 and 750 units, storing the six quotients in globals.
- `project_offset_triple_wide`: Divides the argument by denominators derived from a depth value to compute two quotients offset by plus and minus 46080 and a third quotient, storing the three results in globals.
- `draw_tactical_hint_text`: Draws hint text lines chosen by the level, including Use the Force, Exhaust Port Ahead, Exhaust Port Missed and Avoid Catwalks, then draws a block of cells that cycles through three marker values.
- `initialise_column_positions`: Resets scroll and count state, points two list pointers at their buffers, and fills eleven column slots with increasing positions stepped by 1900, each paired with a second value 950 further on.
- `scroll_column_list_tick`: Scrolls eleven column positions and a set of records, recycling the first column off the left edge by shifting the list and appending a new column 1900 further along, updates two projection values from a rotation global, and compacts records that have left the screen.
- `draw_column_tile_pattern`: Draws a vertical column of tiles between two rows derived from a mode value, plotting sixteen-entry tile patterns through lookup and plot calls for each row.
- `show_bonus_score_screen`: Draws the end-of-wave bonus screen with the referenced lines about the Death Star being destroyed, starting bonus points, shield bonus and shield strength, builds a bonus line from the shield count, flashes the display over several timed frames, then clears it.
- `test_shot_hit_region`: Tests whether a shot lies within a target box for the flagged regions, and on a hit clears the active flag, toggles a status byte, calls a hit routine and starts a sound effect.
- `draw_shape_by_code`: Draws one of three marker shapes chosen by a code, plotting a fixed sequence of points around the stored marker coordinates: code twelve draws a longer pattern, code eight a short four-point box, and other codes a standard box.
- `draw_masked_nibble_markers`: Walks the four-bit groups of a mask and draws a marker pair for each group with its low bits set, moving down the column by a fixed step per group and returning the last drawn value.
- `set_status_colour_and_copy`: Sets a colour value from a global, copies four display coordinate globals into saved slots, and sets a status colour from the wave counter, using a fixed value once the counter exceeds nine.
- `check_trigger_target_hits`: Checks the target flags of the current entry against a range test and a random chance; when a flag bit matches and the position is inside the shot window, it sets a state value, plays a sound through a call and marks the entry as hit, then updates the active flag.
- `draw_moving_target_sprites`: Draws moving target sprites and their trailing strokes at computed positions, advancing their timers, playing a sound effect and spawning a new marker when a random test passes.
- `process_entries_below_threshold`: Repeatedly calls one routine for each list entry whose key is below the argument, advancing the list pointers past each one.
- `spawn_lane_entries_check_hits`: Spawns new list entries in left and right lanes when random tests pass and the state allows, rebuilds the table of entry positions, then checks each entry for a hit within a window and, on a hit, clears the flag and plays a sound effect.
- `draw_periodic_marker_sprites`: Draws marker sprites at fixed positions chosen by a status flag and level counters, using a timing counter so the markers appear on a rhythm, with a third marker drawn when a flag is set.
- `draw_trench_frame_entries`: Draws the trench frame: plots two rows of marker strokes, draws lane markers from a counter, then walks the list of six entries drawing hits and each entry in range through called routines, and rebuilds the list pointers.
- `draw_projected_wireframe_frame`: Projects a wireframe of fifty-seven vertex pairs from stored rotation values into screen coordinates, scaling each vertex by depth, then plots the wireframe segments through called routines.
- `animate_sweep_steps`: Steps an animation parameter from minus 720 up to zero, advancing by forty, twenty or twelve as it passes thresholds, and in each step sets a value from a random routine, draws a frame and waits for a frame to pass.
- `run_frames_until_flag`: Runs frames, each drawing through called routines and waiting for a frame, until a hit flag or a status flag is set; it then clears the sequence state and sets the end flag when no further hit is pending.
- `run_display_loop_until_flag`: Repeats a display and wait cycle, alternating between two display routines according to a level counter, until an input or status flag is set; it then clears a pending flag and returns the status value.
- `compute_base_relative_address`: Returns an address formed by adding a fixed negative offset to the A4 base register, giving a pointer to a slot in the global data area.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `clear_table_and_open_libraries`: Zeroes a long block of longwords, records the exec library base in a global, and checks a flag bit in a library structure before calling a library vector.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use; it does no work.
- `allocate_and_fill_file_record`: Allocates a block sized from a stored index multiplied by six, sets its header fields and writes a four-character tag into it.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `split_command_line_arguments`: Splits a text buffer into whitespace-separated tokens, keeping spaces inside double quotes and treating a doubled quote as one quote character.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source string into a destination buffer until a zero byte is copied or the count runs out, then writes zero bytes over the rest of the count.
- `append_string_capped`: Moves to the end of the destination string and copies the source after it, stopping at a zero byte or after a fixed maximum length.
- `append_string_with_count`: Scans to the end of the destination string, then copies the source onto it until a zero byte or until the count argument runs out.
- `copy_string_to_buffer`: Copies bytes from a source string to a destination buffer until a zero byte is copied, including the terminator.
- `multiply_low_32_bit_product`: Multiplies two 32-bit values by summing the cross products of their 16-bit halves, so only the low 32 bits of the product are computed.
- `load_icon_library_for_window`: Opens a library by a global name and, if that succeeds, uses a value from the caller's structure to open a second resource.
- `format_into_buffer`: Stores a destination pointer in a global, runs a formatting routine that emits characters through the byte-writing callback, then writes a terminating zero at the destination.
- `format_into_buffer_short`: Stores a destination pointer in a global, runs a formatting routine that emits characters through the byte-writing callback, then writes a terminating zero at the destination.
- `write_byte_advance_cursor`: Writes the low byte of its argument to the location held in a global cursor pointer, advances that cursor by one, and returns the byte.
- `forward_format_to_callback`: Passes a format string and its stack arguments to the formatter, using the byte-writing callback as the output routine.
- `invoke_two_subroutines`: Pseudo-code is unavailable because decompilation timed out.
- `format_string_to_output`: Walks a format string, sending literal characters through an output callback and handling conversion specifiers with flags, width, precision and length modifiers.
- `apply_operand_sign_to_result`: Sets a sign flag from the signs of its two operands, calls another routine, and negates that routine's result when the flag is set.
- `negate_when_first_negative`: Calls another routine, then negates the value it leaves in the D1 register when the first operand is negative.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value whose upper half is the value that routine left in the D1 register and whose lower half is the D1 value from before the call.
- `forward_call_and_return`: Calls one routine and then returns, with no other work in the body.
- `perform_32bit_division`: Divides the 32-bit dividend in D0 by the divisor in D1 and returns the quotient in the upper half and the remainder in the lower half of a 64-bit result.
- `compute_string_length`: Walks a zero-terminated byte string to its terminator and returns the number of bytes before it.
- `trampoline_call_and_return`: Calls one other routine and returns, passing its own stack arguments along.
- `invoke_three_subroutines`: Pseudo-code is unavailable because decompilation timed out.
- `stub_return_only_2`: Pseudo-code is only a return, so the body does no work.
- `sequence_two_subroutine_calls`: Pseudo-code is unavailable because decompilation timed out.
- `write_global_byte_advance_cursor`: Stores a byte from a fixed global location into the address held in a cursor register, advances that cursor by one, and returns the byte.
- `call_routine_for_each_record`: Walks an array of 22-byte records from a start address up to a fixed end address, calling another routine with each record's address.
- `release_record_resources`: Returns a minus-one value for a null pointer.
- `release_record_without_check`: Applies the same flag-driven calls and field clearing as the null-checked cleanup routine to the record in address register A2, without a null check.
- `put_byte_into_buffer`: Writes one byte into a buffered output record and returns it, or returns a minus-one failure value when the record's error flag is set.
- `put_global_byte_into_buffer`: Writes a byte taken from a fixed global location into a buffered output record and returns the stored value.
- `write_global_byte_size_check`: Writes a byte from a fixed global location into an output record.
- `write_global_byte_to_record`: Writes a byte from a fixed global location into the record's buffer, or passes it to another routine when the size field is one.
- `write_global_byte_fast_path`: Resets the record's pointer from its base field, sets its end pointer from the size field and marks the buffer active.
- `find_free_record_slot`: Scans an array of 22-byte records for the first one whose flag byte is clear, zeroes its three words and returns its address.
- `find_free_record_from_pointer`: Starts from the record in address register A2 and checks one 22-byte record at a time for a clear flag byte.
- `clear_record_header_words`: Zeroes three consecutive longwords starting at the address held in A2.
- `initialize_record_buffer`: Calls another routine to obtain a buffer pointer.
- `initialize_record_from_global`: Sets a large size field in the record, sets a flag bit, and stores a pointer taken from a global slot into the record.
- `process_all_list_records`: Walks a singly linked list of records from a global head pointer, calling another routine on each record with a size taken from the record's second longword plus eight.
- `process_record_list_from_pointer`: Walks a singly linked list of records starting at the address held in A2, calling another routine on each record with a size taken from its second longword plus eight.
- `allocate_record_and_link`: Requests a block of the argument plus eight bytes through a library call.
- `link_record_into_list`: Pushes the record at the address in A2 onto the front of a global linked list, stores a value from a fixed slot in the A5 base in its second longword, and returns the address past the two-longword header.
- `forward_low_word_argument`: Forwards the low 16-bit word of its 32-bit argument to another routine and then returns.
- `unlink_record_from_list`: Searches the global linked list for the record whose data address is the argument minus eight.
- `unlink_record_matching_global`: Searches a linked list starting at the address in A2 for the record whose data address matches a fixed global slot.
- `unlink_record_from_predecessor`: Removes the record at A2 from a linked list by pointing its predecessor at the record's next pointer, or updates the global head when the record is first.
- `remove_list_head_record`: Sets the global list head to the next pointer of the record in A2, calls another routine, and returns zero.
- `check_table_entry_active`: Checks a record index against a bound held in a global and returns minus one, with an error code of two stored in a global, when the index is negative, too large, or points at an empty table entry.
- `check_interactive_status`: Calls another routine to determine if a specific condition is met based on a value retrieved from a table.
- `stub_return_zero`: Pseudo-code only returns zero, with no calls or memory access in the body.
- `handle_file_deletion`: calls a routine to delete a file and, if unsuccessful, calls a routine to report an I/O error and stores a value in a global variable.
- `return_zero`: returns the value zero to the caller.
- `invoke_two_unnamed_routines`: Pseudo-code is unavailable because decompilation timed out.
- `dos_transfer_with_error_check`: Returns minus one and sets an error code of five when the record's access bits are clear.
- `perform_dos_transfer`: Attempts a data transfer and, if it fails, calls an error routine and stores the result in a global variable.
- `stub_return_only_3`: Pseudo-code is only a return, so the body does no work.
- `check_signal_bit_flag`: Calls a library routine with a zero value and a single-bit mask, then tests that bit in the result.
- `call_then_return_zero`: Calls one routine and returns zero.
- `execute_output_and_callback`: Performs an output operation and then calls a function through a pointer stored in a table.
- `run_hook_then_call`: Calls a hook through a global pointer when that pointer is set, then calls another routine and returns.
- `release_table_and_resources`: Calls another routine on each six-byte table entry, releases the table block, and invokes an optional hook.
- `release_table_entries_supervised`: Calls another routine on each six-byte table entry, releases the table block, invokes an optional hook and releases three further stored pointers.
- `finish_buffer_release_sequence`: When the secondary pointer is clear and the primary pointer is set, calls another routine with the stored pointer and a size from the stored count, then again with the second stored pointer and a size of the count plus one, times four.
- `call_library_then_return_value`: Calls one library routine with no argument, then calls a second library routine with a stored pointer, and returns a longword taken from the A5 base.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `clear_table_entry_by_index`: Validates a six-byte table index against a count held in a global.
- `clear_pointer_entry`: Checks a bit in a word at a pointer location, calls another routine if the bit is not set, and then zeroes the pointer.
- `call_close_handler`: Calls a function through a pointer retrieved from a table offset.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `execute_indirect_function_pointer`: Fetches a function pointer from a nested memory structure and executes it.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested table structure and executes it.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_pointer_3`: Retrieves a pointer from a nested memory structure and executes the function located at that address.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `execute_indirect_function_pointer_4`: Retrieves a pointer from a nested table structure and executes the function located at that address.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `execute_indirect_function_pointer_5`: Retrieves a function pointer from a nested table structure and executes it.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_pointer_6`: Retrieves a pointer from a nested structure within a data table and executes the function located at that address.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `forward_library_vector_call`: Jumps through a library vector at a fixed offset from a library base stored in a global slot.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_record_with_fields`: Calls a routine with a minus-one argument and returns zero when the result is minus one.
- `allocate_record_from_globals`: Allocates a 34-byte block through a library call and fills its fields from fixed global slots, storing the result of a second routine in it.
- `fill_record_from_globals`: Fills the record in address register A2 from fixed global slots, using the same field layout as the allocation routine, and stores the result of a second routine in it.
- `pass_record_field_address`: Calls another routine with the address of a field fourteen bytes into the record in address register A2, then returns.
- `release_record_block`: Clears the record's marker byte and two longword sentinels, calls another routine on the record's byte field when its pointer field is set, and then frees the record block through a library call.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_with_constant_48`: Calls another routine with the input parameter and the fixed value 48, then returns.
- `forward_pointer_argument`: Passes its pointer argument to another routine and returns without other work.
- `allocate_tagged_block`: Returns zero when the first argument is zero.
- `free_tagged_block`: Returns zero for a null pointer.
- `forward_library_vector_198`: Jumps through the library vector at a fixed offset from a library base stored in a global slot.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `forward_library_vector_294`: Jumps through the library vector at a fixed offset from a library base stored in a global slot.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `forward_library_vector_132`: Jumps through the library vector at a fixed offset from a library base stored in a global slot.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `forward_library_vector_210`: Jumps through the library vector at a fixed offset from a library base stored in a global slot.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory it points to, adds four to that stored value, clears the next longword in the block and stores the original pointer value in the longword after it.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `forward_library_vector_552`: Jumps through the library vector at a fixed offset from a library base stored in a global slot.
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
- `setup_disk_track_dma`: Sets up the floppy disk controller for a track transfer: it disables interrupts around a timing loop, writes a sync word and the buffer address and length to the disk registers, and enables interrupts again.

### Star Wars (1987)(Domark).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
