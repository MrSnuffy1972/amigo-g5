# Game 1697 (blues brothers, the (1991)(titus)[cr company][t +5 legend]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01697). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 296 of 296 named (0 author symbols), 296 of 296 described (4 by rule, 59 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1809 of 1809 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12717 of 16167 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 89343 |
| code | 61516 |
| data | 705709 |
| padding | 8 |
| unused | 44544 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf:bootblock | 1 | 31 | 1 |
| logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf boot-block load | 1 | 92 | 2 |
| logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf boot-block load | 1 | 447 | 14 |
| logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf boot-block load | 1 | 15597 | 279 |

## Cases handled

- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: custom-format (non-DOS) disk, 1606 non-empty blocks: matched against track-loaded segments
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: the boot block reads 18432 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: the boot block reads 47616 bytes from disk offset 805888 to an address (found statically); recovered as a unit
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: the boot block reads 98304 bytes from disk offset 707584 to an address (found statically); recovered as a unit
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: 3 logged loads cover 164352 bytes of the image
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 2-4 (bytes 1024-2560) outside the filesystem hold data, not code (opcode density 0.018, entropy 6.73)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 6-7 (bytes 3072-4096) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.77)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 9-12 (bytes 4608-6656) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.77)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 16-531 (bytes 8192-272384) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.41)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 538-786 (bytes 275456-402944) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.88)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 790-870 (bytes 404480-445952) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.80)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 874-902 (bytes 447488-462336) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.01)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 904-904 (bytes 462848-463360) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.59)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 906-966 (bytes 463872-495104) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.95)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 968-968 (bytes 495616-496128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.18)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 970-970 (bytes 496640-497152) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.38)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 972-1158 (bytes 497664-593408) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.67)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 1160-1162 (bytes 593920-595456) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.94)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 1164-1164 (bytes 595968-596480) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.30)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 1166-1232 (bytes 596992-631296) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.86)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 1234-1238 (bytes 631808-634368) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.77)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 1240-1268 (bytes 634880-649728) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.18)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 1270-1270 (bytes 650240-650752) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.55)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 1272-1272 (bytes 651264-651776) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.57)
- blues brothers, the (1991)(titus)[cr company][t +5 legend]/Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf: blocks 1274-1381 (bytes 652288-707584) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.43)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_shared_byte_to_247` (Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf boot-block load +an address to an address (98304 bytes)): Stores the value 247 into a shared variable. Kind: memory integrity/checksum trap. Pass path: sub_E164.
- `load_disk_sector_block` (Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf boot-block load +an address to an address (98304 bytes)): Converts a track and sector pair into a byte offset in the disk image, using 512-byte sectors and 5632-byte tracks. Kind: track/sector validation and custom block loader. Pass path: The loop continues to the next sector/block via the `dbra` instruction in the caller, provided the `iVar6` counter reaches zero and the function returns successfully..
- `step_counter_toward_target` (Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf boot-block load +an address to an address (98304 bytes)): Masks a shared byte and steps it one unit at a time toward a target derived from the argument, calling a helper at each step. Kind: checksum/integrity trap. Pass path: the loop completes and the bitwise operations on pbVar2 are applied to the data/state.

## Functions


### Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf:bootblock

- `read_disk_blocks_into_memory`: Issues a sequence of read requests through the exec library's DoIO call, setting each request's command, length, byte offset and destination, and then hands control to the loaded code.

### logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf boot-block load

- `start_loaded_program_entry`: Entry point of the loaded program: it sets the system interrupt enable bits, makes a system trap call, initialises a few pointers and a counter, then loops waiting for the beam to reach a line until the counter reaches zero.
- `initialise_display_copper_list`: Clears a memory block, builds a copper list of wait and move entries covering 200 lines, and sets up the blitter, sprite and display registers through helper calls.
- `clear_block_and_wait_frames`: Clears a block of 456 words, sets two stored addresses and a count, then loops waiting for the beam to reach a line, calling helpers on each pass.
- `run_step_sequence_until_done`: Advances a step counter by two per pass until it reaches 348, calling helpers each pass.
- `run_frame_step_loop_entry`: Entry into the frame step loop: it sets a state byte, selects the next source pointer from a table (using a default when the entry is the end marker), and runs the loop that clears a block and waits for frames once the state reaches two.
- `step_back_frame_sequence`: Steps a counter back by two and, when it reaches zero, clears the state and runs one helper.
- `reset_frame_sequence_state`: Clears the state byte, runs a helper, selects the next source pointer, and then runs the wait-and-finish loop that returns once the state reaches two.
- `advance_colour_step_frame`: Counts down a frame counter and, when it reaches zero, moves a step index forward by two, wrapping after 456 entries.
- `run_frame_wait_loop_fragment`: Fragment of the frame loop that selects the next source pointer from a table.
- `remap_row_colours_to_copper`: Waits for a raster line, advances the source row pointer by 160 bytes, and for each of 160 bytes searches a small palette list for the matching value.
- `remap_row_colours_without_wait`: Advances the source row pointer and, for each of 160 bytes, searches a small palette list for the matching value, writing the match into a copper list entry with a fixed value of 4 beside it.
- `write_indexed_list_block_entries`: When the input value is not the end marker 640, writes two groups of words into a list record, each group taken from pointer tables at that index, together with fixed control words and an offset computed from a parameter.
- `fade_palette_towards_targets`: Counts down a frame counter, then moves each of a list of colour entries one step toward its target in each of the three 4-bit components.
- `copy_system_view_pointers`: Forbids task switching while it reads view pointers from the graphics library, copies eight pointer and word entries into the copper block, sets the wait count and frame counter, then waits for frames until the counter reaches zero.

### logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +5 Legend].adf boot-block load

- `decode_bitplane_block_by_mode`: Decodes an image block according to its mode byte.
- `interleave_three_bitplane_rows`: Interleaves three bitplane source rows, whose count comes from the block header, into word-aligned output.
- `reset_six_record_defaults`: Sets one field in each of six eight-byte records to 16, writing it twice, and then sets two stored pointers to the same default value.
- `dispatch_display_setup_by_mode`: Dispatches on a mode byte.
- `build_image_entry_tables`: For each entry in a table of image pointers, copies three plane blocks of the decoded size into the output buffer, then interleaves them into the entry's final word-aligned layout.
- `index_image_group_pointers`: Loads a resource through a helper, then for each group in its header records the group's start address in a pointer table, advancing by a size computed from the group's width, height and a mode flag.
- `index_image_groups_single_mode`: Loads a resource through a helper, then records the start address of each group in a pointer table, advancing by a size computed from its width and height.
- `build_256_entry_address_table`: Fills a table of 256 entries with addresses, each 26 bytes after the previous one, starting from a fixed base address.
- `encode_group_into_plane_rows`: Re-encodes an image group row by row, writing a count word and packed row data into the output, and recording each row's start address in a pointer table.
- `merge_four_bitplanes_into_words`: For each row of a group, ORs four bitplane source words at a fixed stride into one output word, and writes a zero word after each row.
- `index_fixed_stride_records`: Loads a resource through a helper, then stores pointers to a counted list of records in a pointer table, stepping 32 bytes per record.
- `fill_pointer_table_by_stride`: Fills consecutive entries of a pointer table with an address that advances 32 bytes per entry, until the index reaches its limit.
- `load_fixed_resource_block`: Calls a helper with one fixed resource address and returns.
- `call_other_routine`: Calls another routine with a specific constant passed as an argument.
- `lookup_table_entry_value`: Returns the value held in the entry at the given index of a pointer table, read from a word at a fixed offset behind the entry's address.
- `step_through_ten_entries`: Loops over ten entries in ascending or descending order depending on a flag, calling helpers for each one.
- `copy_four_plane_rows`: Copies 192 rows of 20 words from a source area into four areas spaced at fixed intervals, starting at an offset given by the parameter.
- `copy_plane_rows_from_state`: Calls the row copy routine with the parameter and two stored values.
- `locate_iff_body_and_cmap`: Searches an IFF file's chunk list for the BODY and CMAP chunks through a chunk-finder helper, stores the data address of each, and returns one when both are found and zero otherwise.
- `unpack_packbits_row`: Decodes PackBits-style run-length data: a negative header byte repeats the next byte, and a non-negative header copies that many plus one literal bytes, until the requested output length is reached.
- `unpack_four_plane_line_blocks`: For each of 192 lines it unpacks four 40-byte bitplane rows spaced at a fixed plane interval, then moves the destination back to the start of the line.
- `unpack_sixteen_rgb_colours`: Converts 16 three-byte colour entries into 12-bit colour words, placing the high nibble of the first byte in the red field, of the second in the green field and of the third in the blue field.
- `compare_zero_terminated_text`: Compares a zero-terminated string with a second string character by character.
- `clear_and_combine_flags`: Clears ten stored values and calls a helper, then combines the stored values with repeated OR operations into a result.
- `build_threshold_plane_masks`: For each threshold level from 15 down to zero, builds a 16-entry mask table by comparing the three colour nibbles of each of 16 colour words against the level, and passes the table to a helper.
- `build_fading_colour_masks`: Copies 16 colour words into a work table, then for each of 16 levels reduces each word's three colour nibbles by one step and passes the resulting table to a helper.
- `load_startup_data_files`: Initialises a 256-entry table and calls about thirty helpers at startup, including the loads of the data files named 0.BIN and DEMO0.BIN.
- `show_inter_lbm_picture`: Loads the INTER.LBM picture through a helper, runs a short sequence of display calls with a fixed number of steps, and then returns.
- `draw_two_forty_step_lines`: Draws 40 steps in each of two directions around the given position, calling a helper for each step and either a delay or a flush routine depending on a flag.
- `fill_alternating_mask_rows`: Fills two 18-word tables with alternating blocks of all-ones and zero words, repeating the blocks across each table.
- `reduce_value_modulo_ten`: Reduces the low byte of its second argument modulo ten, then passes the result to two helper routines.
- `rearrange_bytes_into_plane_lanes`: Copies bytes from a source block into a destination block at fixed lane offsets, placing four byte lanes spaced 432 bytes apart so each source byte lands in its own plane position.
- `gather_bytes_from_plane_lanes`: Copies bytes from one of two source tables, chosen by a parameter, into the destination at lane offsets spaced 432 bytes apart.
- `copy_plane_lanes_between_buffers`: Copies the same lane pattern between two buffers passed in as parameters, with 432 bytes between lanes.
- `copy_lanes_from_indexed_source`: Picks one of several stored buffers using a low-nibble index and a flag, then copies its lane pattern into the matching destination slot.
- `copy_lanes_into_indexed_slot`: Copies a fixed source buffer's lane pattern into the destination slot selected by the low nibble of the parameter.
- `set_initial_position_values`: Sets two stored values to 160 and 1, the initial state of the position pair.
- `reset_sentinel_values`: Sets one stored value to zero and another to the largest 16-bit value, all ones.
- `load_blues_picture_and_wait`: Loads the BLUES.LBM picture through a helper, copies a stored state word, then loops calling a helper until its counter is set and finishes the load.
- `run_choice_menu_loop`: Loads the CHOIX.LBM picture, then loops: on each pass it advances an index through a list of entries and draws them through helper calls, until a selection is made.
- `compute_plane_byte_address`: Returns an address made from a row value multiplied by 128 plus a column value offset by a stored base.
- `draw_tile_grid_block`: Walks a block of 12 rows at a 640-byte stride, drawing 20 bytes per row through a helper for each address, then stores 20 and 12 as its size values.
- `copy_four_plane_word_group`: Copies one eight-word group from a source table into four plane areas spaced at fixed intervals in the destination.
- `scroll_column_block_step`: Moves the stored scroll offsets by 16 units in one step, wraps a counter after 38 steps back to 20, and redraws the affected columns through helper routines.
- `redraw_scroll_edge_blocks`: Redraws the edge blocks of a scrolling display by calling a helper with block offsets derived from the scroll position, choosing among four cases.
- `encode_flag_run_length`: Packs six nonzero tests into a flag byte, then extends the current run while the flags match the previous entry, up to 255, and otherwise starts a new run record.
- `append_flag_run_entry`: Packs six nonzero tests into a flag byte and extends the current run when it matches the previous flags, up to 255; otherwise it starts a new run record.
- `update_flag_run`: Updates a byte at a pointer by setting specific bits based on a global variable, or advances a table pointer and resets a byte if a condition is met.
- `start_new_flag_run_record`: Starts a new run record in the output by writing a zero count and the saved flag byte, then advances past both.
- `decode_flag_run_step`: Steps through a flag run-length stream: when the current run is exhausted it reads the next run's count and value, otherwise it decrements the count, then splits the value into six flag bits.
- `advance_match_counter_step`: Checks a stored value against a table entry.
- `update_record_position_table`: Loops over 41 records, setting each record's horizontal and vertical position from a stored origin plus fixed offsets, clearing its status bytes, and calling 30 helpers to refresh the related display data.
- `step_record_value_sequence`: Reads the table entry for the record's current index.
- `load_record_table_pair`: Copies two bytes from a table, at offsets one and two past the record's step, into two fields of the record.
- `reset_record_step_state`: Sets a record's step to one and clears its counter and two flag bytes.
- `set_record_timing_and_direction`: Sets the record's delay to 60 and its speed to plus or minus 60 depending on a direction byte, toggles a bit, and sets a counter to one.
- `convert_value_to_digit_tiles`: Splits a stored value into decimal parts by repeated subtraction of ten and eight, looks up a tile value in a table for each part, and sets the result fields through helper calls.
- `dispatch_record_draw_by_type`: Positions a helper at the record's coordinates, then calls a per-type routine chosen from a table by a stored byte, passing the record.
- `read_map_cell_value`: Returns the low byte of a map cell's value.
- `read_map_cell_below_value`: Looks up the map cell one row below the record's position using the same tile lookup, and returns the low byte of a word from its table entry.
- `update_record_animation_frame`: Looks up the record's current frame entry, falling back to the previous frame's entry when a flag is set, and copies its value into the record.
- `apply_record_table_command`: Reads the command entry for the record.
- `place_map_tile_and_redraw`: Writes a tile value into the map at the given position, then redraws it on screen when it lies inside the visible window, wrapping the offsets across the 40-column map width.
- `lookup_tile_palette_index`: Returns an index for the record's position, built from the tile's table entry as a base value, plus the low nibble of a stored word, plus two.
- `lookup_cell_attribute_byte`: Returns one byte from the table entry of the map cell at the given position, read at a fixed offset of 22.
- `redraw_visible_map_region`: Redraws the visible window of the map by advancing a set of tile counters, then copying tile data for 11 rows of 19 cells into the screen buffer using each cell's table entry.
- `copy_scrolled_map_rows`: For each entry in a list of row pointers, copies a 16-cell block of tile data from the source buffer to the screen buffer at the entry's offset, wrapping across the scroll width.
- `apply_pair_helper_four_times`: Calls one helper four times with parameter pairs: each value with itself first, then the two values together twice.
- `dispatch_record_update_by_type`: Clears a flag on the record, runs the update routine for its type, then draws it.
- `draw_record_by_type`: Draws a record through a helper, choosing a path by its type byte and a flag: types 100 or below 3 are drawn one unit further along, and other types use the table entries for their index.
- `process_record_logic`: Checks a value at a pointer and branches to different subroutines, potentially passing data retrieved from a table using an index and an offset.
- `dispatch_record_update`: Evaluates a value from a pointer to decide whether to call one of two different routines or a third routine with parameters derived from a table and an offset.
- `advance_record_timer_and_reset`: Advances the record's frame from a table, and for certain state codes counts a timer up to 30 before resetting the record to its idle state.
- `select_record_animation_table`: Chooses which animation table the record uses from its flags, its speed magnitude and its state, then advances the record's frame through that table.
- `select_animation_table_variant`: Selects the record's animation table from its state and speed, with a variant that uses a different table for state 4 and for the one-direction case, then advances the frame.
- `advance_speed_based_animation`: Selects an animation table by the record's speed magnitude, using a second table at eleven or more, applies a special table for two states, and advances the frame index.
- `advance_frame_by_speed_threshold`: Selects a table by speed magnitude and the record's state, then advances the record's frame index and copies the entry's value into the record.
- `advance_frame_from_state_table`: Selects a frame table from the record's flags and advances the frame index one step, wrapping to one at the table's count, then copies the frame value into the record.
- `advance_frame_index_wrapping`: Advances the record's frame index through the table at offset 52, wrapping to one after the last entry, and copies the entry's value into the record.
- `advance_frame_with_fixed_delay`: Advances the record's frame through the table at offset 32 and sets its delay field to 48.
- `advance_frame_by_mode_argument`: Chooses a frame table from a mode argument, zero or two or eight, then advances the record's frame index one step with wrap-around.
- `reset_frame_to_first_entry`: Sets the record's frame index to one and copies the table entry at that index into the record's value.
- `select_animation_table_by_flags`: Chooses the animation table from the record's flags, state and direction, sets its delay to 48 or 24, and advances the frame index through that table.
- `update_record_from_neighbour_cells`: Reads the map cells around the record through helper lookups, then updates the record's state flags and delay from what it finds, advancing its frame.
- `advance_frame_and_maybe_activate`: Sets the record's mode to two, advances its frame through a table at offset 92, and when the record is idle and the argument reaches eleven it switches the record to an active state with new timing values.
- `advance_frame_mode_from_state`: Sets the record's mode from whether its state is two, advances its frame through the table at offset 92, and applies the same activation check as the other frame routines.
- `advance_frame_mode_one_activation`: Sets the record's mode to one, advances its frame through the table at offset 92, and activates the record when it is idle and the argument reaches eleven.
- `select_mode_and_frame_table`: Sets the record's mode and frame table from its state, using a helper lookup to switch to another table when the cell reads thirteen, then advances the frame.
- `advance_frame_mode_two`: Sets the record's mode to two, advances its frame through the table at offset 100, and activates the record when it is idle and the argument reaches eleven.
- `advance_frame_by_cell_code`: Sets the record's mode from its state, selects a table at offset 96 or 100, switching to offset 104 when the cell lookup returns thirteen, then advances the frame.
- `advance_frame_with_cell_check`: Sets the record's mode to one, selects the table at offset 96 and switches to offset 104 when the cell lookup returns thirteen, then advances the frame index.
- `activate_record_at_offset_eight`: Sets the frame to its first entry and, when the record is idle and the argument reaches eleven, activates it and moves it eight units in its direction.
- `spawn_offset_record`: Creates a companion record at the given record's position shifted by eight units in each direction, copies its position fields, sets its state to one, and stores a helper-provided address.
- `forward_to_frame_helper`: Calls a helper routine and returns with no other work.
- `process_record_state_transitions`: Compares the record's state with its companion's state and calls helpers for any transition, then selects a frame table from the state and sets the record's mode to idle.
- `advance_frame_with_flag_step`: Steps the record's frame index forward by the low bit of a global flag, wrapping to one at the table end, and copies the entry's value into the record.
- `select_frame_table_for_state`: Chooses the record's frame table from its state and its type-seven flag, using a fixed table for state seven and ordered tables for the other states.
- `advance_frame_by_flag_step`: Advances the record's frame through the table at offset four, stepping by the low bit of a global flag.
- `select_frame_table_by_type`: Chooses a frame table from the record's type and state, with the default being the table at offset 44.
- `advance_frame_by_state_code`: Chooses a frame table from the record's state code, then advances the frame index by the low bit of a global flag.
- `advance_frame_table_at_44`: Advances the record's frame index through the table at offset 44, stepping by the low bit of a global flag.
- `advance_frame_table_at_32`: Advances the record's frame index through the table at offset 32, stepping by the low bit of a global flag.
- `advance_frame_table_at_zero`: Advances the record's frame index through its first table, stepping by the low bit of a global flag.
- `advance_frame_by_mode_parameter`: Selects a frame table from a parameter value, using the table at offset 32 for two and the default table at offset 44 otherwise, then advances the frame by the flag step.
- `advance_frame_preselected_table`: Advances the record's frame through the table already selected for it, stepping by the low bit of a global flag.
- `advance_frame_first_table`: Loads the record's first frame table and advances the frame index by the low bit of a global flag.
- `store_byte_indirect`: Stores a byte into a memory location determined by an offset and a pointer.
- `advance_record_animation_substep`: Advances a record's animation with a sub-step counter.
- `dispatch_update_helper`: Checks a value within a structure and calls one of many different routines based on that value.
- `forward_to_update_helper`: Calls one update helper and returns.
- `check_bounds_set_velocity`: Marks the record off-screen when its horizontal position is outside 1 to 320 or its vertical position is outside the playfield.
- `set_leftward_velocity_flag`: Sets bit two of the record's flag byte, gives it a horizontal velocity of minus 128, clears two position fields, sets its vertical step to 128, and returns zero.
- `reset_record_to_idle`: Sets the record's state to idle, clears its flag and timing fields, and returns one.
- `snap_record_or_idle`: For record kinds 11 and 12 it sets two offsets to minus three and copies the record's position shifted down by four bits into its tile fields.
- `clear_record_to_idle`: Sets the record's state to idle and clears its two flag bytes.
- `advance_record_state_machine`: Runs a small state machine on the record's counter.
- `spawn_directional_record`: Creates a new record beside the given one, positioned by its direction byte, sets its vertical and horizontal position fields, and marks it with state two.
- `flag_companion_when_active`: When the selected record is not idle, sets a flag byte on its companion record to three and clears this record's two flag bytes.
- `advance_record_step_case`: Advances the record's step and runs a case-specific setup for steps zero through three, such as switching the companion record to state two and copying its position.
- `toggle_record_state_and_delay`: Toggles the record's state bit when a lookup returns one, then sets its frame to the state plus one and its delay to 40.
- `update_record_motion_and_frame`: Updates a record's frame and motion when its flags allow it.
- `update_record_motion_variant`: Performs the same frame and motion update as the main routine, with a different branch for records whose state is a special value, using helper lookups to check the surroundings.
- `tick_record_speed_counter`: Accumulates a speed counter.
- `tick_global_speed_counter`: Accumulates the global speed counter and, each time it passes 100, raises the level counter as in the per-record version and calls a helper.
- `decrement_record_counter`: When the record's timer flag is clear, decrements its counter and calls a helper while the counter is positive, then sets the timer to 75 unless the mode is 16.
- `draw_record_status_slots`: Draws five status slots for the record: slots below the record's count get one helper call, and the remaining slots get another.
- `step_record_toward_target`: Moves the record toward a target by a step looked up from a distance table of six-byte entries up to twelve, adjusting its position fields accordingly.
- `clamp_record_horizontal_speed`: Sets the record's horizontal speed from a stored factor, capped so it never exceeds the record's velocity, with the sign taken from the velocity.
- `update_record_position_from_map`: Updates the record's screen coordinates from its velocity, then looks up the map cell at the new position and stores the cell's offset, with the vertical position adjusted by the map row.
- `update_record_flag_state`: Updates a record's flag byte from its status bits: it sets the flag when the record is unflagged and its companion is clear, clears it when a helper reports a match, and clears it for kinds 11 and 12.
- `update_object_position_by_state`: Reads a state value from a helper for the object, then moves the stored fixed-point position by one step and recomputes the integer position from it.
- `update_actor_motion_and_facing`: Applies the horizontal velocity to the object's position, clamps it to a limit, and turns the facing direction when the object reaches a boundary.
- `clamp_object_position_to_bounds`: Clamps an object's position to a minimum value when it is too low and resets its speed.
- `dispatch_object_update_by_state`: Calls the state-update helper for an object only when it is not in its special mode and its flag is clear, then calls the position update helper based on the state value.
- `scan_entity_pairs_for_collision`: Loops over 41 slots of 110-byte entity records and checks each active entry against the current object's bounding box.
- `snap_to_overlapping_surface`: Tests whether an object overlaps a surface record, and if so copies the surface's position and velocity and places the object just below it.
- `compute_dominant_axis_delta`: Computes the horizontal and vertical distances from a reference point to a target position.
- `reset_object_at_spawn_match`: Scans a table of 30 two-word keys for the one matching the object's identity, then places the object at the paired coordinates.
- `reset_object_vertical_when_above`: When the object's position is below a reference value and its flag is clear, it sets the object to a start state at the stored row.
- `snap_object_vertical_by_state`: Moves the object to the stored row when it is above the reference value, or to the next row down when its speed is negative, and clears its motion flags.
- `advance_frame_counter_and_snap`: Snaps the object to a row as in the neighbouring routine, then increments a shared counter and wraps it to 1 when it reaches 400.
- `advance_counter_with_indirect_object`: Performs the same row snap and counter increment as its neighbour, but reads the object through an indirect pointer rather than a fixed shared variable.
- `clamp_global_flag_to_one`: Sets a shared flag to 1 when it is nonzero.
- `step_until_value_changes`: Adds a per-object offset to a value, then samples a lookup repeatedly until the result differs from the first sample.
- `pass_offset_to_helper`: Adds an object field to the argument and passes the sum to a single output helper.
- `copy_sixteen_words_to_table`: Copies sixteen consecutive words from the source block into a table in shared memory, writing each one at a fixed stride.
- `store_indexed_table_word`: Stores the second word of its argument into a table slot selected by the first word, scaled by four.
- `store_shared_color_word`: Stores its word argument into a shared variable.
- `fill_sixteen_rows_from_source`: Copies one 36-byte-stride source entry into sixteen consecutive 40-byte rows, writing the same first value to each row and a second value from the source to the paired slot.
- `set_copper_list_and_jump`: Writes a new copper list start address and triggers a copper jump so the display list restarts at that address.
- `restart_copper_list_at_address`: Sets a shared flag to zero and writes a new copper list start address, then triggers a copper jump so the display list restarts there.
- `restart_copper_list_with_flag`: Sets a shared flag to one and writes a different copper list start address before triggering a copper jump.
- `set_shared_pointer_sequence`: Assigns three fixed address constants to a shared variable in turn, ending on the first one.
- `setup_blitter_masks_and_wait`: Loads full-word masks and data values into the blitter registers, sets the DMA control word to enable the blitter, and polls the DMA status until the blitter has finished.
- `run_callbacks_from_linked_list`: Walks a singly linked list from a shared head pointer and passes each node's word and two long fields to a handler.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `draw_clipped_map_cells`: Clips a rectangle to the visible window and copies map cells into the display buffer.
- `copy_record_into_shared_slot`: Copies an eleven-word record into a shared slot and advances the slot pointer by a fixed size.
- `update_audio_input_and_sprites`: Runs once per frame to set the four audio channel volumes and the first sprite pointer, then reads the fire-button and disk status bits from the CIA port to choose the next state.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use.
- `initialise_word_if_zero`: Stores a fixed default word into a shared variable only when that variable is currently zero.
- `wait_until_shared_flag_clear`: Busy-waits in a loop until a shared variable becomes zero, then returns.
- `copy_fixed_block_at_offset`: Calls a block copy helper with the same source and destination offset taken from a shared base, and a fixed count of 40 units.
- `call_helper_with_table_entry`: Looks up a four-byte-stride table entry using the argument as the index, then passes that entry and a shared base value to a helper.
- `initialise_blit_descriptor_fields`: Writes two argument values into a blit descriptor, clears its offset fields, sets its mask fields to all ones, and stores a standard blitter control word.
- `copy_tile_into_four_planes`: Copies 64 words from a source block into the four bitplanes of the display buffer, with the planes spaced 7680 bytes apart.
- `draw_tile_at_packed_position`: Unpacks a packed coordinate pair into row and column offsets, adds them to a shared base, and passes them with a scaled size value to a drawing helper.
- `draw_across_four_planes`: Builds a parameter block from two offsets, a width and a height, then calls a helper four times.
- `initialise_blit_descriptor_zero`: Writes a shared base value and zero offset fields into a blit descriptor, sets its mask fields to all ones, and stores a standard blitter control word.
- `clear_six_1024_word_buffers`: Zeroes six consecutive blocks of 1024 words each and stores the start address of each block in a pointer table.
- `set_shared_constant_word`: Stores one constant word into a shared variable whatever its previous value, through two branches with identical results.
- `set_shared_word_32`: Stores the value 32 into a shared variable in both branches, so the result does not depend on the prior value.
- `clear_buffer_block_by_selector`: Clears 24 longwords in one of two buffers, chosen by whether the argument is nonzero, stepping through the block in rows.
- `clip_region_and_zero_planes`: Checks that the requested column and row lie within fixed limits, and calls a fallback helper when they do not.
- `clip_and_draw_sprite_rows`: Computes the clipped horizontal and vertical extent of a sprite from its position and size, then draws its rows into the display buffer.
- `draw_masked_tile_row`: Draws one row of a tile using a path chosen by the alignment value.
- `clear_masked_bits_in_planes`: Clears the bits of one mask from words in one buffer and the complementary mask from words in a second buffer, across three word positions.
- `clear_masked_bits_with_constant`: Works like its neighbour and clears masked bits across buffer words, but first combines the argument with a fixed constant before inverting it to form the mask.
- `stub_infinite_loop_only`: Contains only an infinite loop that never exits, with no calls or memory access.
- `pop_byte_from_ring_buffer`: Takes the next byte from a small ring buffer, advancing the read index while it is behind the write index, and clears that slot.
- `check_keyboard_interrupt_condition`: Tests a shared status word for a keyboard interrupt condition and, when it is set, flags it and calls a handler.
- `record_key_transition_in_queue`: Writes a key press or release value into a small ring buffer when its low bit is set.
- `update_fire_button_state`: Updates a shared button-state byte from its previous value and sets the idle state when no button bits are present.
- `poll_mouse_state_and_deltas`: Reads mouse movement and the right and middle button state from the hardware registers, and updates shared delta values and a button flag.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_two_port_status_flags`: Sets or clears a direction flag bit for each of two input ports from bits of a shared status byte, calling a helper after each update.
- `set_bit_flags_from_word`: Sets or clears one bit in four flag bytes of a record, chosen by a bit index argument and bits of the word argument.
- `clear_buffer_set_dma_control`: Zeroes a 128-byte buffer, then stores a few constants into shared variables, including a control word, a value of 247 and a buffer pointer.
- `clip_and_draw_horizontal_span`: Clips a line segment against the screen edges, using a horizontal limit of 319 pixels, and draws it through helper calls.
- `fill_table_until_flag_clears`: Writes a computed offset into successive table entries, waiting on a shared flag between writes and advancing the source pointer in steps.
- `run_sequence_and_return_zero`: Clears a shared variable and calls a fixed sequence of helpers.
- `initialize_global_sequence`: Sets a series of values in a global variable and executes a loop that calls another routine multiple times.
- `copy_name_uppercase_skipping_dots`: Fills a ten-byte field with spaces, then copies a source string into it.
- `search_entry_table_for_name`: Scans 32-byte table entries, skipping entries with a special leading byte, for one whose first eleven bytes match a stored name.
- `copy_block_with_chunk_stepping`: Copies up to 1024 bytes from a fixed source into a destination.
- `compute_scaled_index_offset`: Computes a word-sized offset from a shared base and the argument, scaling the index by two.
- `divide_word_by_shared_value`: Divides a 16-bit argument by a shared divisor and forms the quotient and remainder, which are passed to helper routines.
- `set_shared_byte_to_243`: Stores the value 243 into a shared variable.
- `set_shared_byte_to_247`: Stores the value 247 into a shared variable.
- `load_disk_sector_block`: Converts a track and sector pair into a byte offset in the disk image, using 512-byte sectors and 5632-byte tracks.
- `decode_mfm_sector_data`: Scans a raw track buffer for the disk sync pattern, then decodes each sector's odd and even MFM bit streams into bytes.
- `combine_mfm_odd_even_bits`: Combines the odd and even bit streams held in a pair of longwords using a mask, and returns the decoded value.
- `stub_return_stack_value`: Returns an eight-byte value read from the stack, with no other work.
- `return_null_value`: The routine is empty and returns a null value to the caller.
- `step_counter_toward_target`: Masks a shared byte and steps it one unit at a time toward a target derived from the argument, calling a helper at each step.
- `pulse_control_byte_with_delay`: Sets and clears bits of a control byte in a fixed sequence, then runs a delay loop of 3584 iterations.
- `set_control_bits_delay`: Clears one bit and sets another in a control byte, then runs the same delay loop of 3584 iterations.
- `reset_global_and_call_helper`: Clears a shared variable and calls a helper routine.
- `select_and_set_audio_sample`: Waits for a shared flag to clear, then, when the requested entry differs from the current one, looks up its eight-byte record.
- `initialize_audio_channels`: Performs a series of loops and assignments to populate a table with values from other memory locations and interacts with audio hardware registers.
- `load_audio_channel_records`: Fills the four audio channel records with a sample pointer, length, period and volume taken from a table.
- `compare_audio_period_to_target`: Masks the low twelve bits of a stored value into a record field and clears a flag.
- `apply_audio_effect_step`: Reads the effect code from a channel's control word and updates the period or volume field by one step.
- `initialise_audio_dma_once`: When a shared flag is clear and a second condition holds, it calls two helpers, sets the audio pointers and the audio control registers, and marks the flag set.
- `poll_interrupt_request_flag`: Checks a shared bit and, when it is set, updates a pair of shared flags and calls a helper.
- `set_record_field_and_call`: Stores the constant 128 into a field of a record, calls a helper, and returns the argument unchanged.
- `repeat_helper_six_times`: Calls a helper six times in a loop and returns, with no other work.
- `stub_infinite_loop_on_field`: Loops forever on a comparison of a record byte with itself, which is always true, so the routine never returns.
- `reset_audio_period_and_state`: Clears a shared variable, sets the period register of the first audio channel to 126, and sets two shared flags.
- `apply_operand_sign_to_result`: Works out the sign of the result from the signs of two inputs, calls a helper for the magnitude, and negates the result when the sign is negative.
- `divide_32_by_32_unsigned`: Performs unsigned division with a 16-step shift-and-subtract loop, returning the quotient and the remainder.
- `divide_with_swapped_divisor`: Swaps the two halves of the divisor, then runs the same 16-step shift-and-subtract division, returning the quotient and remainder.
- `multiply_32_by_32_bits`: Multiplies two 32-bit values from the registers using 16-bit partial products, and returns the result as a 64-bit value.
- `stub_infinite_loop_only_2`: Contains only an infinite loop with no effective work, so control never returns to the caller.
- `stub_infinite_loop_only_3`: Contains only an infinite loop, so control never returns to the caller.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `stub_bad_instruction_data`: The bytes at this address do not decode to a valid instruction, so there is no routine to describe.
- `empty_routine`: This routine contains no executable instructions and performs no operations.
- `empty_routine_2`: This routine is empty and contains no executable instructions.
- `stub_bad_instruction_data_2`: The listing counts many instructions here, but the bytes do not decode to a valid instruction from the start, so this is data rather than a routine.
- `encode_run_length_data`: This routine processes data to implement run-length encoding by identifying sequences of identical bytes.
- `null_routine`: This routine is empty and contains no valid instructions.
- `null_operation_placeholder`: The routine is empty and contains no executable instructions.
- `return_from_subroutine`: The routine is empty and contains no valid instructions.
- `clear_status_flags`: The routine is empty and performs no operations.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `return_from_mouse_poll`: This routine is empty.
- `calculate_mouse_delta_offset`: This routine is empty.
- `empty_routine_3`: The routine contains no valid instructions and performs no operations.
- `return_from_subroutine_7`: The routine performs a single instruction to return control to the calling function.
- `abort_execution`: The routine contains no valid instructions and terminates the current execution flow.
- `calculate_pixel_offset`: Calculates a memory offset based on sprite coordinates to facilitate clipping and drawing.
- `write_word_to_memory`: The routine writes a 16-bit word from a register into a memory location specified by a pointer.
- `return_from_subroutine_2`: The routine consists of a single instruction that exits the current function and returns control to the caller.
- `return_from_subroutine_3`: The routine performs a single return instruction to exit the current function.
- `empty_routine_4`: This routine contains no valid executable instructions and performs no operations.
- `empty_routine_5`: The routine contains no valid instructions and is empty.
- `update_global_timing_state`: This routine is empty due to corrupted instruction data at the entry point.
- `clamp_value_to_one`: Ensures a global variable is constrained to a value of zero or one.
- `return_from_subroutine_4`: The routine consists of a single return instruction that exits the current function and returns control to the caller.
- `update_record_position_table_2`: The routine is empty and contains no valid executable instructions.
- `empty_routine_6`: This routine contains no executable instructions.
- `empty_routine_7`: The routine contains no valid executable instructions and performs no operations.
- `empty_routine_8`: The routine contains no executable instructions.
- `extract_digit_from_value`: Calculates a single decimal digit from a numeric value using division and modulo operations.
- `process_bitplane_data`: This routine performs bitwise operations to manipulate or extract data from multiple bitplanes.
- `halt_execution`: The routine contains no valid instructions and terminates the control flow immediately.
- `wait_for_vblank`: The routine is empty and performs no operations.
- `return_from_interleave`: The routine is empty and serves only to return from the calling procedure.
- `return_from_interleave_2`: The routine is empty and performs no operations before returning to the caller.
- `return_from_subroutine_5`: The routine consists of a single instruction that returns control to the caller.
- `extract_bitplane_pixel`: Extracts a single bit from a source buffer using a bitmask and shifts it to prepare for bitplane composition.
- `extract_bitplane_pixel_2`: This routine is called during bitplane decoding to isolate and retrieve a single bit from a specific bitplane data stream.
- `update_timer_state`: The routine is empty due to corrupted instruction data at the entry point.
- `empty_routine_11`: This routine contains no valid instructions.
- `return_from_subroutine_6`: The routine is empty and consists only of a single instruction that terminates execution or returns control.
- `empty_routine_9`: The routine contains no valid executable instructions.
- `extract_bitplane_data`: The routine performs bitwise operations to isolate specific bits from a source value.
- `validate_memory_bounds`: The routine performs a comparison between a register and a memory address to check if a value falls within a specific range.
- `abort_execution_2`: The routine performs no operations and terminates the execution flow immediately.
- `return_empty_routine`: The routine contains no executable instructions and performs no operations.
- `empty_routine_10`: This routine contains no valid instructions and performs no operations.
- `nop_instruction`: The routine contains a single instruction that does not perform any operation or change the state of the processor.
- `execute_invalid_instruction`: The routine consists of a single byte of data that does not form a valid instruction.
- `clear_memory_block`: This routine is empty.
- `nop_placeholder`: This routine is empty and consists of a single instruction that does not perform any operations.
- `halt_execution_2`: The routine contains no valid executable instructions and terminates the control flow immediately.
- `return_from_audio_initialization`: This routine is empty and serves only to return control to the caller after the audio DMA setup is complete.
- `fetch_indirect_map_attribute`: Retrieves a byte from a memory location indexed by a value found in a table.
