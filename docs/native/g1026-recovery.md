# Game 1026 (abyss (1991)(free spirit software)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01026). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901708 top-level bytes in 22 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 319 of 319 named (0 author symbols), 319 of 319 described (79 by rule, 59 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2585 of 2585 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13078 of 13078 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 408353 |
| code | 42618 |
| data | 71369 |
| padding | 4584 |
| unused | 374784 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Abyss (1991)(Free Spirit Software)[cr SKR].adf:Run | 2 | 117 | 2 |
| Abyss (1991)(Free Spirit Software)[cr SKR].adf:Main | 3 | 11246 | 199 |
| Disk-Validator | 1 | 386 | 1 |
| SKID_ROW | 2 | 24 | 4 |
| 15 | 1 | 2 | 1 |
| Diskfont.library | 13 | 830 | 53 |
| Abyss (1991)(Free Spirit Software)[cr SKR].adf:4 | 2 | 405 | 15 |
| Abyss (1991)(Free Spirit Software)[cr SKR].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 38 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 29 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_channel_from_sequence_entry` (4): Loads the next sequence entry for a channel by copying a pointer and length from a source table and adding a stored offset. Kind: disk hardware/track sequence validation. Pass path: the routine updates internal state/pointers based on a sequence entry; a successful 'pass' is the standard execution flow where the sequence entry is valid and the hardware registers (A5) are updated with the next expected track/sector para.

## Functions


### Abyss (1991)(Free Spirit Software)[cr SKR].adf:Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen longwords from a fixed offset in a source block to a destination pointer when that pointer is non-null, then calls an indirect routine.

### Abyss (1991)(Free Spirit Software)[cr SKR].adf:Main

- `program_entry_clear_and_start`: Program entry point that clears a block of 200 longwords and saves a library base and a pointer.
- `allocate_buffer_and_init`: Calls an indirect allocator for a 1005-unit buffer and returns 100 as an error code when it fails.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a1).
- `xor_scramble_buffer_bytes`: Walks a buffer of 619 bytes and exclusive-ors each byte with the sum of its index and the buffer's first byte.
- `copy_block_into_allocated_buffer`: Allocates a 1005-unit buffer and, when that succeeds, copies 620 bytes into it, releases the buffer and calls a routine.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a2).
- `stage_block_then_set_state`: Sets a state byte to one, allocates a 1006-unit buffer and copies 620 bytes into it when allocation succeeds, then releases it.
- `store_word_then_set_colours`: Stores a word at a fixed offset, then sets sixteen colour registers to the same value of seven in each component through an indirect colour call.
- `set_sixteen_colour_entries`: Sets sixteen colour registers to the same value of seven in each component through an indirect colour call, then calls a display update routine.
- `draw_outline_shape_at_point`: Draws a fixed eight-segment outline at a given position and size.
- `draw_four_fixed_shapes`: Clears the rendering area through an indirect call, then draws four shapes at fixed positions and sizes using the outline routine.
- `draw_hall_of_fame_table`: Draws the title ABYSS HALL OF FAME, then loops over ten entries, drawing each row through a row routine.
- `draw_hall_of_fame_row`: Draws one table row: a rank label that is blank for the first nine entries, a value formatted from the nibbles of a stored word, and a seven-character name field right-aligned by its measured width.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `insert_hall_of_fame_entry`: Finds the rank where a new score belongs in a ten-entry table, shifts lower entries down, and copies a 21-byte name into the freed slot.
- `draw_hall_of_fame_rows`: Sets the pen and drawing mode, draws a header from its two arguments, and loops over ten entries drawing each through the row routine.
- `draw_rank_row_fixed_columns`: Draws a rank label that is blank for the first nine entries, then a value at column 100 and a three-character field at column 170.
- `poll_input_until_selection`: Draws a row, then repeatedly reads messages from a port through an indirect call, replying to each one, until a state byte clears.
- `read_name_entry_keys`: Clears a 20-character name buffer to spaces, then reads key codes, ignoring one group of special codes.
- `stub_return_only`: Returns immediately without doing any work.
- `drain_message_port_replies`: Repeatedly takes messages from a port through an indirect call and replies to each one until none remain.
- `wait_for_message_value`: Waits for a message on a port, pausing between checks.
- `open_display_and_load_palette`: Allocates a display structure and opens a view, loading sixteen palette entries from a stored table and clearing a region.
- `draw_moment_message_panel`: Clears the rastport, draws a panel at fixed coordinates, and writes a text message beginning MOMENT with a sixteen-character buffer.
- `wait_until_flag_set`: Repeatedly calls a routine until a state byte becomes non-zero, then returns.
- `run_save_game_screen`: Draws the title ABYSS - SAVE GAME and copies nine words from a stored table into a record.
- `draw_centred_text_line`: Measures a text string, then moves to a position that centres it across a 640-pixel width and draws the string.
- `setup_blitter_copy_registers`: Writes a fixed blitter control word, mask and modulo values, and two source pointers with a blit size of 998 to the blitter registers.
- `draw_grid_background_and_labels`: Clears the rastport and draws vertical and horizontal grid lines across the 640-pixel width in alternating pens.
- `count_frame_and_ack_interrupt`: Increments a frame counter byte and writes 32 to the interrupt request register, acknowledging the vertical-blank interrupt.
- `setup_interrupt_and_frame_loop`: Stores an interrupt handler pointer and an initial count, then runs a loop that clears a frame state and waits on a countdown and a flag.
- `draw_message_boxes_and_text`: Clears the rastport and draws three boxes at fixed positions.
- `draw_two_panel_screen_layout`: Clears the rastport and draws boxes and horizontal rules at fixed coordinates, then writes a series of text labels, centring some by measuring their width.
- `start_audio_channel_zero`: Starts audio channel zero: when a flag is clear it writes length and volume words and clears the flag, otherwise it points the channel at a stored sample address with a length of two.
- `stop_audio_channel_zero`: Clears a flag and writes to the audio and DMA control registers to stop audio channel zero.
- `start_audio_channel_one`: Starts audio channel one: when a flag is clear it writes length and volume words and clears the flag, otherwise it points the channel at a stored sample address with a length of two.
- `stop_audio_channel_one`: Clears a flag and writes to the audio and DMA control registers to stop audio channel one.
- `start_audio_channel_two`: Starts audio channel two: when a flag is clear it writes length and volume words and clears the flag, otherwise it points the channel at a stored sample address with a length of two.
- `stop_audio_channel_two`: Clears a flag and writes to the audio and DMA control registers to stop audio channel two.
- `start_audio_channel_three`: Starts audio channel three: when a flag is clear it writes length and volume words and clears the flag, otherwise it points the channel at a stored sample address with a length of two.
- `stop_audio_channel_three`: Clears a flag and writes to the audio and DMA control registers to stop audio channel three.
- `init_audio_channel_handlers`: Clears four pairs of channel flags and installs four handler pointers in per-channel table entries.
- `allocate_free_audio_channel`: Finds a free audio channel among four, marks it in use, and fills its record with a sample address, length and volume from the sample table.
- `randomise_thirteen_entry_pointers`: Fills thirteen table pointers, each computed from random selections of a table offset, then sets several state words and bytes to fixed starting values.
- `advance_timed_state_machine`: Counts a timer word down from 192 and, at fixed points of its count, adjusts a position word, sets an output byte and toggles a bit in a flag byte.
- `blit_sprite_records_with_wait`: Programs the blitter with masks, modulo values and pointers, then loops over sprite records and blits each active one.
- `update_sprite_slots_each_frame`: Loops over thirteen object slots and, for each slot whose active bit is set, updates its entry.
- `find_record_matching_two_bytes`: Searches an eight-byte record table for the entry whose two key bytes match a given pair, and returns that record's address.
- `blit_three_segment_sprite`: Programs the blitter for three sprite segments, choosing each segment's control values and pointer offsets from the segment index.
- `setup_blitter_for_sprite_part`: Writes blitter control values chosen by a flag, a mask of minus sixty-four, and the pointers and size for one sprite part.
- `program_blitter_sprite_mask`: Programs the blitter control, mask, modulo and size registers for a sprite part, taking its source address and an offset from a record.
- `step_bcd_counter_on_timer`: When a timer byte reaches zero, steps a packed-decimal counter up or down one digit at a time, with carries between the nibbles of each byte.
- `dispatch_setup_by_mode`: Dispatches on a mode argument.
- `allocate_five_buffers_or_fail`: Waits until the blitter is idle, then allocates five buffers through an indirect call and stores each returned pointer.
- `allocate_five_resource_pointers`: Allocates five resources through an indirect call and stores each returned pointer, returning 100 when the last allocation fails.
- `load_game_tables_and_buffers`: Allocates several large blocks through library vector calls, then fills a three-by-two table of pointers from random offsets and stores scaled high-word values into a record.
- `copy_five_regions_into_buffer`: Calls a routine with 50000, allocates a 1005-unit buffer and copies five memory regions of differing sizes into it, then releases the buffer.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to -(a0).
- `copy_four_regions_into_buffer`: Allocates a 1005-unit buffer, copies four memory regions of differing sizes into it, and releases it.
- `tiny_move_5`: Small routine of 1 instruction that copies d0 to d1.
- `initialise_entity_tables_from_record`: Reads a record selected by a stored index and computes a position and size from it.
- `set_copper_display_mode`: Writes the copper list address registers and the copper jump strobe, along with display words chosen from the bits of a flag byte.
- `advance_lfsr_random_state`: Advances a 32-bit linear-feedback shift register eight steps, taking feedback from bits one and three, and stores a 16-bit random value in a global word.
- `pack_eight_entries_to_buffer`: For eight entries, packs position and flag fields into two words each.
- `pack_entries_from_index`: Packs position and flag fields into two words for each entry from a starting index up to eight.
- `copy_table_entries_into_structures`: Copies thirty-two words to an output pointer, then fills two and three paired entries of a structure with high words and following words.
- `copy_pairs_request_interrupt`: Copies two and three paired entries into a structure, packs two nibble values into one byte and copies a control word.
- `copy_three_pairs_set_bplcon`: Copies three paired entries into a structure, packs two nibble values into one byte and writes a control word.
- `init_display_and_interrupts`: Clears state bytes and loads a data block into a buffer, then calls a library routine.
- `randomise_grid_entries`: Waits until a status nibble is at most nine, then fills a grid of eighteen by ten entries with values chosen by random selections, calling a routine for each.
- `swap_handler_pointer_slot`: Saves the pointer at a fixed offset of a structure and installs the handler pointer there when the argument is one.
- `random_offset_plus_nibble`: Draws two random values and returns their sum plus the low nibble of a stored word.
- `reset_control_state_once`: When a state word differs from 37, sets it to 37, calls a routine, clears one byte, toggles bit two in two flag bytes and sets another flag byte to one.
- `initialise_game_state_and_input`: Adds a stored value to a running total, sets default state words and bytes and a timer of 9000, then calls a series of setup routines.
- `copy_four_by_four_tiles`: Copies a four-by-four block of tile bytes from a source table into destination planes, taking the source offset from a stored index.
- `update_entity_position_and_velocity`: Adjusts two position bytes by table deltas chosen by an index, and adds a random offset to a position longword that it copies into a stored pair.
- `advance_double_buffer_index`: Writes three values derived from a table with an offset of 3424 into a structure, sets a counter to four, and toggles a buffer index bit.
- `wait_port_bit_timeout`: Counts a value down while a status bit of the CIA port register is set, stopping at zero.
- `wait_for_flag_then_clear`: Waits until a flag byte becomes non-zero, then clears it and returns.
- `offset_position_entries`: Adds a first argument to two stored longwords and a second argument to three other stored longwords, shifting a set of positions by those amounts.
- `update_entity_positions`: Updates multiple sets of coordinate pairs by adding or subtracting values retrieved from a table based on an index.
- `scan_rows_for_tile_entries`: Loops over two rows, checking each against a bounding range.
- `match_code_to_table_index`: Maps a nine-bit code to an index: codes 277 and 278 map to seven and nine, and other codes are searched in a thirteen-entry table.
- `read_joystick_direction_code`: Reads the joystick movement register and the fire-button status, then combines the directional bits into one of eight direction codes.
- `clamp_packed_table_range`: Clamps the packed nibble fields of a range of table words against a limit byte and stores the clamped values back into a second table.
- `advance_stage_timer`: Counts a timer down and, when it expires, reloads it and steps a stage counter between zero and ten.
- `reset_seven_table_entries`: Sets seven table words to 4095 and sets a counter to two.
- `unpack_word_run_lengths`: Decodes a run-length compressed block of sixteen-bit words.
- `set_edge_bits_in_rows`: Ors edge bit masks into two pairs of bytes for each of up to thirty-four rows, marking bits at both ends of each row.
- `reset_blits_and_timers`: Copies a saved pointer set into working slots, programs blitter registers and blit sizes for three and two entries, then clears a stage byte and sets a timer to 3000.
- `blit_three_offset_instances`: Programs the blitter for a sprite and draws it three times at offsets derived from a stored value and an index.
- `initialise_state_bytes_and_table`: Sets a block of state bytes to starting values, zeroes a state word, and writes two table entries of 272 and 288 plus two entries of 107.
- `dispatch_tile_code_update`: Reads a table code for the current position and, when the position is inside bounds, dispatches on the code's low nine bits.
- `poll_keyboard_and_advance_events`: Handles keyboard input read from the serial register, where the letter A decrements two state values.
- `advance_sequence_state_machine`: Counts a timer byte down and, when it expires or the state reaches four, switches on the current sequence state and advances it.
- `init_entity_with_random_placement`: Takes three values from an indirect library call to pick a new position and offsets, computes the matching table entry and resets the entity's state flags.
- `match_stored_cell_and_latch`: Returns 0 unless a latched state flag is set and the entity is idle.
- `advance_timer_and_pick_cell`: Advances a timer by 2000 up to a cap, derives a table offset from its low byte and selects a random cell within bit-limited ranges.
- `process_command_and_bounds_check`: Decodes a 9-bit command code from the current table entry and checks the target cell against the map bounds and the last command.
- `move_to_open_adjacent_cell`: Tries to move the current position one cell in a direction taken from offset tables.
- `tick_entity_state_machine`: Advances a per-entity countdown state machine each frame.
- `validate_target_cell_passable`: Returns 1 when the target cell holds an enabled special code or lies inside the map bounds without a blocking flag.
- `choose_and_apply_next_step`: Handles special command codes 3, 21 and 117 for an active entity.
- `run_timed_movement_cycle`: Delegates to the step routine when the activity flag is clear.
- `direction_index_between_points`: Returns zero when the two points coincide.
- `copy_table_row_to_slot`: Copies three 16-bit words from a row of a lookup table into a buffer slot chosen by the first argument, with the row offset taken from the second argument scaled by eight.
- `search_word_then_lookup_byte`: Scans a 16-bit word list for the given value, then returns a byte from a table at an offset built from the second argument and an indirect library call result.
- `reset_entity_tracking_state`: Sets a count to 36, points eight slots at a sentinel record, clears four counters and fills sixteen bytes with 255.
- `update_cell_keep_top_bits`: Replaces the low nine bits of the current cell word with a new value while keeping its top two bits.
- `swap_two_plane_tile_blocks`: Swaps a 3-by-3 block of 16-bit words between two interleaved planes, stepping by fixed strides per row and column.
- `advance_sequence_frame_timer`: Advances a per-sprite frame counter and reloads its timer when it expires.
- `shift_entity_slots_between_rows`: Copies two entity slots and their word and byte records from one row index to another, overwrites the source slots with a sentinel and calls two routines for the new row.
- `set_state_byte_by_selector`: Stores the second argument into one of four state bytes chosen by the first argument, where selectors 0, 2, 4 and 6 map to different bytes.
- `store_value_in_bucket_table`: Returns 0 if the slot already holds the value.
- `insert_entry_and_rebalance_buckets`: Decrements a pending count, inserts the value into the bucketed table, then moves entries down the bucket chain and stores a marker equal to the third argument plus 8 in the top bucket.
- `remove_entry_from_bucket_table`: Removes a value from the bucketed table when the slot matches, checking the paired plane if needed.
- `clear_buffer_and_call_library`: Zeroes a block of 200 longwords and saves the library base taken from the executive base pointer.
- `stub_empty_return`: Empty routine that only returns.
- `allocate_record_and_register`: Allocates a record whose size comes from a stored count and fills its open-state fields.
- `split_command_line_into_argv`: Copies a command line into a newly allocated buffer and splits it into words at spaces, tabs and line breaks.
- `tokenize_quoted_string`: Parses a source string into individual tokens by splitting on whitespace while respecting double-quoted substrings.
- `append_string_bounded`: Appends a zero-terminated source string to the end of a destination string, copying at most about 32 thousand bytes.
- `append_string_with_limit`: Finds the terminating zero of the destination and copies the source after it, stopping at the given count.
- `copy_string_with_zero_padding`: Copies bytes from the source until a zero byte or the count runs out, then fills the rest of the count with zero bytes.
- `open_icon_library_and_link`: Opens the library named in the global text (icon.library) and looks up a related entry through library calls.
- `perform_signed_division`: Works out whether the result is negative from the signs of the two inputs, calls the unsigned division routine and negates its quotient when the signs differ.
- `perform_signed_division_2`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `negate_result_if_first_negative`: Calls a routine, then negates the value it leaves in the second register when the first input is negative.
- `call_helper_and_pair_results`: Calls a routine and returns a 64-bit value built from the original second input and the value the routine leaves in the second register.
- `recursive_call`: The routine calls another routine that is the same as the caller, causing the routine to recurse without performing any other operation.
- `divide_unsigned_longwords`: Divides an unsigned 32-bit dividend by a divisor.
- `run_optional_hook_then_call`: Calls an optional hook routine stored in a global pointer when that pointer is set.
- `release_entries_and_buffers`: Releases the stored entry records and their table buffer in turn, runs an optional shutdown hook and frees any remaining buffers that are set.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_indexed_entry`: Closes the entry at the given index, returning -1 and recording an error code when the index is out of range or the slot is empty.
- `multiply_longwords_truncated`: Multiplies two 32-bit values using 16-bit partial products and keeps only the low 32 bits of the product.
- `call_close_3`: The routine performs an indirect call to a function located via an offset from a pointer in a table.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested structure within a global data table and executes it.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `execute_indirect_callback`: Fetches a function pointer from a nested structure within a data table and executes it.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function`: Fetches a pointer from a nested table structure and executes the function located at that address.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `dispatch_library_vector_414`: Loads a library base from a global pointer and jumps through the library function at a fixed offset, passing the arguments unchanged.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a pointer from a memory location offset from a base address and executes the function located at that destination.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: Retrieves a pointer from a memory location offset from a global variable and executes the function at that address.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_4`: Retrieves a function pointer from a nested structure within a table and executes it.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_function`: Calls a function from a library using a pointer retrieved from a table.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_function_2`: Performs an indirect call to a library function via a pointer stored in a table.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `dispatch_library_vector_198`: Loads a library base from a stored pointer and jumps through the library function at a fixed offset, passing the arguments unchanged.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_function_3`: Executes a function by accessing a table entry relative to a base address.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_function_4`: Uses a table offset to call a function from a library.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_function_5`: Calls a routine through a pointer located within a table structure.
- `call_library_function_6`: Performs an indirect jump to a library routine using a table-based address.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.

### Disk-Validator

- `scan_disk_bitmap_bits`: Program entry of the disk validator that writes two digit characters into a text line, fills a table of bitmap words with all ones and loops over bitmap bits comparing flags and updating counters.

### SKID_ROW

- `write_output_on_nonzero_result`: Runs a first routine and, when its result is nonzero, calls a second routine and writes through the DOS library's Write vector.
- `open_dos_library`: Calls the exec library's OldOpenLibrary vector to open dos.library and stores the returned library base in a global.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `get_dos_output_handle`: Calls the DOS library's Output vector through the stored DOS base and saves the returned output handle in a global.

### 15

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### Diskfont.library

- `allocate_and_tag_block`: Calls a library vector to obtain a block.
- `check_and_set_font_handle`: This library code calls a routine via a table entry; if the result is non-zero, it stores an argument into an offset within the returned handle and calls another routine via a table entry before returning a boolean.
- `open_libraries_and_init_list`: Opens two system libraries and, when both succeed, initialises an empty list header in the supplied block and returns that block.
- `release_unused_list_entries`: Walks a list of entries, unlinks every entry whose use count is below one and calls a library vector to release it.
- `release_list_header_when_idle`: When the header has no users and no pending entries, it closes two libraries, frees the header, unlinks it from its list and returns the stored value.
- `mark_header_pending_release`: Sets the top bit of a flags byte in the header and returns zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Clears the top bit of a flags byte at a fixed offset in the header and increments a short counter at another offset.
- `decrement_counter_and_release`: Decrements a short counter in the library base.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `compute_signed_difference_score`: Computes a signed score from the difference between two 16-bit values, then for seven bit positions in two byte arrays subtracts a global table entry wherever the bits differ.
- `find_highest_scoring_font_entry`: Locks the FONTS: directory and reads a table of 260-byte records from the font file.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read_2`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_2`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir_2`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg_2`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg_2`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `copy_null_terminated_string`: Copies bytes from the source to the destination one at a time, up to and including the zero terminator.
- `call_helper_at_string_end`: Advances to the zero terminator of the string and passes that end pointer and the second argument to a routine.
- `forward_to_helper_routine`: Calls one routine with no arguments and returns.
- `count_string_length`: Counts the characters before the zero terminator and returns that count.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte.
- `compare_bytes_return_sign`: Compares the byte before one pointer with the byte at another pointer and returns minus one if the first is smaller, otherwise one.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_in_string`: Scans a zero-terminated string for the given character and returns a pointer to its first occurrence, or null if it is not found.
- `find_next_matching_char`: Starts one byte after the given pointer and scans forward for the byte in the low part of the data register.
- `stub_return_zero`: Returns zero with no other effect.
- `find_last_char_occurrence`: Measures the string length and scans backwards from the end to find the last occurrence of a byte.
- `locate_path_separator`: Searches the string for a slash and, failing that, for a colon.
- `collect_font_entries_into_buffer`: Fills a caller buffer with variable-length font records, copying names from the existing list when flag bit 1 is set and scanning the font directory for more entries when flag bit 2 is set.
- `copy_list_then_scan_directory`: Copies names from a linked list of font records into the buffer, filling in each record's type and flags.
- `copy_name_then_scan_directory`: Copies name bytes into the buffer one at a time and, when the record list is exhausted and the second flag bit is set, runs the directory scan.
- `scan_font_directory_entries`: When the second flag bit is set, locks the font directory and reads 260-byte entries whose names end in .font, appending matching records to the buffer.
- `read_font_entry_records`: Reads 260-byte font descriptor records from an open directory and appends each record whose header word matches.
- `parse_font_file_metadata`: Iterates through a font file's internal directory entries to extract and store metadata such as file size and name into a destination buffer.
- `parse_font_directory_entries`: Iterates through disk directory entries to identify files with a specific extension and copies their metadata and names into a buffer.
- `release_scan_locks_and_memory`: Unlocks the directory lock, restores the current directory and frees the temporary buffer.
- `return_zero_constant`: Returns zero immediately without any other operation, so it is a fixed-result stub.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### Abyss (1991)(Free Spirit Software)[cr SKR].adf:4

- `copy_string_into_global_buffer`: Copies a counted string from the source pointer into a fixed global buffer and writes a terminator after it.
- `free_buffer_and_set_status`: When the low byte of the data register is non-negative, it calls a routine, sets a status global to 15 and frees a block through the exec library's FreeMem vector.
- `save_state_and_enable_interrupts`: Saves the previous value of the interrupt control register, stores a new vector address and writes the master-enable and interrupt-enable bits.
- `restore_saved_interrupt_state`: Writes back the interrupt-control value saved earlier and then writes the enable bits to the interrupt control register again.
- `halt_in_infinite_loop`: Calls a routine when bit 5 of the interrupt request status is set, then enters an empty endless loop.
- `open_read_and_close_file`: Opens dos.library and then a file through the DOS Open vector.
- `tiny_move_rts`: Small routine of 2 instructions that copies 1 to d6, then returns.
- `initialise_pointer_table`: Builds a table of 15 longword pointers by accumulating offsets read from a source table at a fixed memory address.
- `advance_step_and_update_channels`: Advances a step counter and, at the cycle boundary, resets it, recomputes a table pointer, waits in a short delay loop and writes output words for each flagged source.
- `apply_channel_effect_step`: Runs one effect step on a channel record.
- `add_volume_with_cap`: Adds the data-register word to a channel level field, caps the result at 64 and copies it into the channel's volume slot.
- `subtract_volume_with_floor`: Subtracts a masked parameter from a channel level field, floors the result at zero and copies it into the channel's volume slot.
- `advance_position_with_wrap`: Sets a position from the low byte of the data register shifted left by seven, plus sixteen.
- `update_channel_from_sequence_entry`: Loads the next sequence entry for a channel by copying a pointer and length from a source table and adding a stored offset.
- `halt_if_key_code_nine`: If the stored byte equals the character nine and its top bit is clear, it opens intuition.library by name and then loops forever calling a library vector.

### Abyss (1991)(Free Spirit Software)[cr SKR].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_infinite_loop`: Empty endless loop with no other work.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_infinite_loop`: The routine enters an infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_2`: Enters a loop that never terminates.
- `execute_infinite_loop_3`: Contains a continuous loop structure.
- `execute_infinite_loop_4`: This routine enters an infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_5`: Traps execution in an infinite loop.
- `execute_infinite_loop_6`: Executes a loop that repeats indefinitely.
- `stub_infinite_loop_2`: Single-instruction endless loop with no other work.
- `execute_infinite_loop_7`: The routine performs an infinite loop that prevents further execution.
- `execute_infinite_loop_8`: Performs a loop without an exit condition.
- `execute_infinite_loop_9`: Runs a loop that does not end.
- `execute_infinite_loop_10`: Enters an infinite loop sequence.
- `execute_infinite_loop_11`: This routine enters an infinite loop that prevents further execution of the calling code.
- `execute_empty_loop`: This routine consists of an infinite loop that performs no operations.
- `execute_infinite_loop_12`: The routine enters an infinite loop that prevents further instruction execution.
- `execute_infinite_loop_13`: The routine enters an unconditional loop that prevents further execution of the calling code.
- `execute_infinite_loop_14`: The routine performs an endless loop that prevents further execution.
- `execute_infinite_loop_15`: The routine enters an infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_16`: The routine performs an infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_17`: This routine enters an infinite loop that prevents further execution.
- `execute_infinite_loop_18`: The routine enters an endless loop that prevents further execution.
- `execute_infinite_loop_19`: The routine enters an endless loop that prevents further execution of the current thread.
- `execute_infinite_loop_20`: The routine enters an unconditional infinite loop that prevents further execution.
- `execute_infinite_loop_21`: The routine enters an infinite loop that prevents further execution.
- `nop_padding`: This routine is empty and performs no operations.
- `execute_infinite_loop_22`: The routine enters an endless loop that prevents further instruction execution.
- `execute_infinite_loop_23`: The routine enters an endless loop that prevents further execution of subsequent code.
- `enter_infinite_loop`: This routine enters an infinite loop that prevents further execution of the current thread.
- `execute_infinite_loop_24`: The routine enters an unconditional infinite loop that prevents further code execution.
- `execute_infinite_loop_25`: The routine enters an unconditional infinite loop.
- `enter_infinite_loop_2`: The routine executes an infinite loop that prevents further instruction execution.
- `execute_infinite_loop_26`: The routine enters an unconditional loop that prevents further code execution.
- `execute_infinite_loop_27`: The routine enters an infinite loop that never exits.
- `perform_endless_loop`: This routine consists of a single instruction that causes the processor to loop indefinitely.
- `enter_trap_loop`: The routine executes an instruction that results in a continuous loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_constant`: Returns minus one with no other work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `do_nothing`: This routine is empty and performs no operations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_infinite_loop_3`: Two-instruction endless loop with no other work.
- `stub_bad_instruction_data`: Decoding reached bad instruction bytes, so the listing stops here.
- `null_routine`: This routine is empty and contains no executable instructions.
- `infinite_loop`: The routine enters an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `record_undecompiled_routine`: Pseudo-code could not be produced for this 12-instruction routine, so its behaviour is not recovered.
