# Game 337 (jail break (1990)(odyssey)[cr pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00337). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905968 top-level bytes in 31 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 180 of 180 named (0 author symbols), 180 of 180 described (6 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1184 of 1184 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 16900 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5357 of 5357 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 631022 |
| code | 22758 |
| data | 244783 |
| padding | 7405 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Jail Break (1990)(Odyssey)[cr PDX].adf:paradox | 2 | 207 | 8 |
| Jail Break (1990)(Odyssey)[cr PDX].adf:jailbreak | 4 | 4401 | 154 |
| Jail Break (1990)(Odyssey)[cr PDX].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 225 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 514 | 6 |
| jailregback.map (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 5 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 209 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 372 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- jail break (1990)(odyssey)[cr pdx]/Jail Break (1990)(Odyssey)[cr PDX].adf:Map/jailregback.map: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 16900 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Jail Break (1990)(Odyssey)[cr PDX].adf:paradox

- `load_hunk_sections_at_entry`: Program entry that calls AllocMem, FindTask and FreeMem and steps through hunk-style records, comparing their type codes with the loader's hunk values and copying section data.
- `stub_empty_return_routine`: Contains only a return instruction and has no visible behaviour.
- `countdown_delay_loop`: Decrements the data register in a loop until it reaches zero, acting as a timing delay.
- `tiny_ori`: Small routine of 1 instruction that merges 164 into d0.
- `unpack_backward_run_length`: Decodes a packed byte stream by writing output backwards from an end pointer, copying runs of earlier bytes and handling repeat counts of three, four and seven.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `step_byte_with_space_wrap`: Decrements a byte in the data register a counted number of times, replacing a wrapped zero value with a space character.
- `null_routine`: This routine is empty and contains no valid instructions.

### Jail Break (1990)(Odyssey)[cr PDX].adf:jailbreak

- `run_entry_setup_sequence`: Program entry that looks up the running task and calls a sequence of library and helper routines to set up state.
- `wait_for_vblank_then_update`: Spins until the vertical blank request bit in the interrupt request register is set, clears it, then calls seven helper routines in order.
- `wait_vblank_run_four_steps`: Repeatedly waits for the vertical blank request bit, clears it and calls four helper routines, looping while the word in the data register stays zero.
- `dispatch_by_mode_until_clear`: Repeatedly picks one of two helper groups by a mode word, each group calling three helpers, and loops while a second word stays nonzero.
- `call_four_until_flag_set`: Calls four helper routines repeatedly until a shared word equals minus one.
- `call_seven_then_optional_helper`: Calls seven helpers repeatedly while a shared word is nonzero, then calls one more helper, with an extra call when a mode word equals two.
- `initialise_mode_and_helpers`: Clears a mode word, calls thirteen helper routines, stores 254 in a shared word, then calls six more helpers.
- `initialise_tile_graphics_state`: Calls helpers to prepare graphics and state, sets a group of position, size and timer words to fixed starting values, and copies configuration words into table slots.
- `reset_level_state_and_flags`: Sets a wait value and clears several state flags and counters, makes fifteen helper calls and stores 254 in a shared word.
- `poll_input_and_decrement_counter`: Decrements a countdown word and resets a state pair when it runs out, then calls helpers and polls a helper result until it is zero.
- `scan_entries_for_state_two`: Makes one helper call, then walks thirteen fixed-size records and calls a helper for each record whose state word equals two.
- `scan_entries_for_state_one`: Walks thirteen fixed-size records and, for each record whose state word equals one, calls two helpers in order.
- `decrement_counters_set_flag_bits`: Visits three counters from the last to the first: a zero counter has its single-bit mask stored into a shared word, while a nonzero counter is decremented by one.
- `reset_state_when_counter_expires`: When a signed counter is nonnegative it decrements the counter, and when it reaches zero it clears one state word, sets another to all ones and sets the counter to minus one.
- `expire_timer_and_clear_pair`: Decrements a timer word while it is nonzero, and when it reaches zero clears two state words and makes a helper call.
- `reload_counter_on_zero_flag`: When bit four of the input word is set, it decrements a counter, and when the counter reaches zero it reloads it to twenty and calls a helper.
- `update_position_from_mouse_delta`: Reads the mouse movement counter, computes signed horizontal and vertical deltas from the previous reading, and either moves a group of coordinates with clamping or accumulates the deltas with saturation and writes derived outputs.
- `set_positions_from_flags`: When a timer word is zero it calls a helper, derives two position words from base coordinates and sets the timer to seventy-four.
- `dispatch_when_state_two`: When the mode word equals two, it calls one helper for each of three nonzero flag bytes, then calls two more helpers.
- `clear_expired_timer_flags`: Clears three flag bytes whose timer words are negative or above 209 when a status bit is clear, and calls a helper once all three flags are zero.
- `initialise_level_positions_and_timers`: Sets a group of position words to 208 and clears a set of flag bytes, then runs a series of helper calls including a loop that repeats while a helper returns nonzero.
- `run_setup_helper_sequence`: Runs seven helper calls, repeats one helper call until it returns zero, then makes four more helper calls.
- `decrement_and_clear_flag_word`: Decrements a word at offset 6 of the structure pointed to by the address register, calls a helper, and clears a shared flag word when a set of comparisons on two status words holds.
- `update_record_flags_by_position`: When a shared status bit is clear, it walks ten-word records until a terminator, marking each record as inside or outside a rectangle and calling helpers for each change.
- `copy_sprite_fields_and_constants`: Copies a group of position and size words from the structure into fixed RAM words, choosing between two field pairs by a flag bit.
- `dispatch_on_mode_value`: Calls one helper when the mode word equals one and a different helper otherwise.
- `dispatch_command_selector`: When a status bit is set, it switches on a command selector from one to sixteen, setting mode and level words or calling helpers for each case, then clears the selector.
- `dispatch_state_byte_handlers`: When a status bit is set, it dispatches on a state byte from one to seven, running a short sequence of helper calls for each state, then clears the state byte.
- `program_blitter_block_copies`: When a mode word equals two, it programs the blitter registers and copies a block five times at a fixed offset, repeating the setup with a second offset when bit thirteen of the input word is set.
- `advance_timer_and_counters`: Decrements a timer word and, when it expires, reloads it to two and steps up to three counters, wrapping each one past a threshold.
- `step_movement_with_collision_checks`: Steps two coordinates in a loop by a stored velocity, checking tile and object codes at each step.
- `check_flag_and_reset_counter`: When a flag is set and its counter, advanced by two, passes 189, it either calls helpers and adds ten to a word at offset 8 when a position difference lies within a signed range, or clears the flag once the counter exceeds 209.
- `update_timers_for_two_entries`: Decrements a position timer for one entry by a stored step and calls a helper, clearing its flag and resetting the timer when it goes negative.
- `clear_sixteen_state_words`: Writes zero to sixteen words of a fixed state block, advancing eight bytes between writes.
- `step_entries_with_tile_bounce`: When a mode word equals two and a flag is set, it walks a zero-terminated list of four-word entries, moving each vertically by its velocity.
- `advance_counter_run_six_calls`: When the word at offset 10 is at most the word at offset 8, it increments the word at offset 6, calls a helper and adds one thousand to offset 10.
- `call_helper_twelve_times`: Calls a helper routine twelve times in total, arranged as two loops of five calls followed by two single calls.
- `write_four_byte_glyph_cell`: When a count field is nonzero, it computes a cell address from two index fields and writes four fixed bytes there, adjusting three of them with values from the structure.
- `sync_changed_word_field`: When the word at offset 4 differs from the word at offset 14, it copies the first into the second and calls two helpers.
- `configure_blitter_source_rows`: When a count field is nonzero, it writes the blitter modulo, mask and size registers from the structure, then loops over a short count, waiting for the blitter to go idle before advancing its source pointer.
- `copy_five_blitter_rows`: Computes two destination addresses from the input position and programs the blitter registers, then starts five copies whose source pointer advances by thirty-two bytes and destination by the plane step each time.
- `copy_tile_rows_via_blitter`: Programs the blitter for a copy at a position computed from the input registers, then runs five blitter starts, advancing the source by thirty-two bytes and each destination by the plane step.
- `update_tile_buffer_cells`: When a mode word equals two and a countdown byte expires, it walks a tile buffer of twenty-six rows by thirteen bytes.
- `handle_mode_input_actions`: When a status bit is clear, it branches on a mode word: mode two checks two pairs of flag bytes and calls a helper when a counter is set, while mode one calls helpers depending on whether a position lies within a rectangle.
- `check_button_and_position_range`: When the mode word equals one, the right-button port bit is clear, and two position words fall inside fixed ranges, it calls one of two helpers depending on a flag.
- `wait_fire_after_cell_index`: Computes a cell index from two position words and stores it in a shared word, calls two helpers, then polls the fire-button port until its bit is set.
- `compute_offset_then_wait_input`: Computes a shared offset from a multiplier and two position words plus a constant of 260, calls a helper, then polls the fire-button port until its bit is set.
- `update_map_nibble_cell`: Computes a map cell address from two position words and a stored plane index, then clears or merges the low or high nibble of that byte according to a mode value.
- `delay_when_mode_thirty_one`: When a mode word equals thirty-one, it runs a short delay loop of thirteen iterations, and otherwise it returns at once.
- `update_map_tile_byte`: Computes a map byte address from two position words and a plane index, and when the stored byte differs from a value derived from a state word, it writes the new value and calls a helper.
- `clear_map_cell_and_notify`: Computes a map byte address from position and plane values, and when the byte is nonzero it zeroes it and calls a helper.
- `clear_map_cell_with_refresh`: Computes a map byte address from the same position and plane values, and when the byte is nonzero it zeroes it and calls two helpers.
- `call_two_helpers_in_order`: Calls two helper routines in sequence and returns.
- `call_two_helpers_then_return`: Calls two helper routines in order and returns with no other work.
- `fill_grid_block_sprites`: Walks a grid of cells, writing a fixed block of parameters into the output registers for each cell and calling a helper that supplies the next cell position.
- `draw_grid_tiles_with_helper`: Walks a small grid of tile positions, writing fixed tile parameters for each cell and calling a helper that supplies the next position.
- `initialise_record_fields_and_call`: Stores a base address and an offset computed from an index, sets a set of fixed fields and a frame size, then calls a helper.
- `call_three_helpers_in_order`: Calls three helper routines one after another with no other work.
- `initialise_record_fields_alt`: Performs the same record setup as its neighbour with a different frame size, then calls a helper.
- `call_four_helpers_by_value`: Calls a helper four times whichever way a word at the address register compares with two, so both branches make the same four calls.
- `call_two_helpers_when_idle`: When a global mode word is zero it calls two helpers, otherwise it returns unchanged.
- `call_two_helpers_when_clear`: When a global flag word is zero it calls two helpers, otherwise it returns unchanged.
- `call_eight_helpers_when_zero`: When a global mode word equals zero it calls a helper eight times in a row, otherwise it returns unchanged.
- `store_inputs_into_record_block`: Copies a value from one global into a record, writes fixed extent words, stores the incoming register values into the record and calls a helper.
- `call_four_helpers_in_order`: Calls four helper routines in sequence and returns.
- `call_single_helper_then_return`: Calls one helper routine and returns with no other operation.
- `call_single_helper_second`: Calls one helper routine and returns with no other operation, identical to its neighbours.
- `store_inputs_and_constant_block`: Stores fixed zero and extent words in a record, copies the two address-register inputs into two record fields, and calls a helper.
- `clear_record_and_store_inputs`: Clears five record words, sets extent words to fixed values, stores the two address-register inputs into the record, and calls a helper.
- `call_init_player_state`: The routine calls another routine and then returns.
- `call_init_player_state_2`: This routine calls a procedure that initializes the player state and then returns.
- `install_interrupt_handler_and_check`: Stores a constant and a handler address into shared globals, calls a library function through a table entry, and when the result is nonzero it calls two more library functions that use the interrupt-enable register.
- `install_interrupt_with_retry`: Stores a constant and a handler address into shared globals, calls a library function, and when it returns nonzero calls two more library functions, repeating that pair once more if a second call also returns nonzero.
- `allocate_block_and_store_handler`: Calls AllocMem through a library table entry, then stores a handler address and a constant into shared globals.
- `clear_two_memory_regions`: Clears two memory regions byte by byte with countdown loops, writing zero to every byte in each region.
- `advance_counter_below_limit`: Increments a counter while the next value stays under one hundred and then calls four helpers.
- `decrement_counter_with_fallback`: Decrements a counter and clamps it at zero.
- `increment_limited_level_counter`: Increments a counter while the next value stays below nine and calls a helper.
- `decrement_counter_and_branch`: Decrements a counter and clamps it at zero, calling one helper when it underflowed and another helper otherwise.
- `call_helper_twice`: Sets a short counter to one and calls a helper twice in a loop, then returns.
- `clear_flag_bit_over_records`: Clears bit one in a byte field of each record in a chain, calling a helper after each one.
- `clear_flag_bit_records_alt`: Performs the same chain walk as its neighbour, clearing bit one and calling a helper for each record.
- `dispatch_state_update_by_step`: Dispatches on a step counter from one to thirteen, updating position, flag and timer words for each step and calling helpers for sprite and state setup.
- `clear_tile_buffer_and_wait`: Clears a buffer of 338 bytes through a pointer, calling two helpers and writing a constant for each cell with a nonzero high nibble other than fifteen.
- `store_masked_state_when_idle`: When a flag word is zero and the low nibble of the input is nonzero, it copies two words from the pointer and stores a masked word and a small value derived from the input.
- `process_tile_interaction`: Looks up a tile code at the position given by the address register and, for codes in a fixed range, updates a counter, a state word and tile values while calling helpers.
- `reflect_velocity_on_edges`: Negates a pair of velocity words when a masked offset from the second pointer's position indicates an edge hit, using the sign of each velocity to decide.
- `check_overlap_and_reflect`: Walks a zero-terminated table of entries and tests whether each overlaps a bounding box.
- `update_counter_pair_by_index`: Calls a helper, then selects one of four cases by an index.
- `set_direction_by_state_index`: When a flag bit is set it clears a related flag and resets timing words.
- `set_tile_collision_flags`: Chooses tile collision flag offsets from a base value according to the current tile type and a secondary tile value.
- `scroll_offsets_until_done`: Adds sixteen to a shared scroll offset, then repeatedly moves four shared position words by one step each and calls helpers until the second helper returns zero.
- `animate_cells_until_done`: Repeatedly steps four shared words in opposing directions and calls helpers until one helper returns zero, then subtracts sixteen from a shared offset.
- `initialise_state_and_helpers`: Calls helpers to clear buffers and wait, sets a group of position words to 208 and flag bytes to zero, repeats one helper call until it returns zero, then sets timers and a counter and makes several more helper calls.
- `call_helper_grid_loop`: Calls a helper once for each cell of a thirteen-by-thirteen nested loop, then returns the input registers unchanged.
- `call_helper_thirteen_times`: Calls a helper thirteen times in a countdown loop, then returns the input registers unchanged.
- `call_helper_grid_loop_alt`: Performs the same thirteen-by-thirteen nested helper calls as its neighbour and returns the input registers unchanged.
- `copy_tile_row_bytes`: Copies 338 bytes from a source row, located by a stored index and a fixed row size, to a destination pointer read from the structure.
- `clear_tile_buffer_and_scan`: Clears a grid of tile bytes of twenty-six rows by thirteen cells, then scans a second buffer cell by cell.
- `decode_level_map_cells`: Walks a grid of twenty-six rows by thirteen cells over a map buffer, calling a helper for each cell whose nibbles are nonzero and accumulating the returned values into the position counters.
- `call_two_helpers_keep_result`: Calls two helper routines and returns the data register value it was given.
- `call_helper_four_times`: Calls a helper four times in a countdown loop and returns the input registers unchanged.
- `parse_control_byte_stream`: Reads a stream of signed bytes: a nonnegative byte ends a run, a sentinel value is skipped, and any other negative byte calls a helper repeatedly while its count is nonzero.
- `write_byte_to_buffer`: Stores the low byte of the data register at the address held in the second address register and returns.
- `initialise_record_block_inputs`: Stores the two register inputs into shared words, computes an address from a stored index and a table, sets fixed sprite and frame fields in the record addressed by the address register, and calls helpers.
- `start_five_blitter_copies`: Computes a source address from the inputs and programs the blitter control words, then starts five copies.
- `call_helper_two_then_return`: Calls a helper twice in a loop and returns the input registers unchanged.
- `call_helper_over_zero_string`: Walks a zero-terminated string, calling a helper for each byte and advancing the pointer to the position the helper returns.
- `copy_or_clear_tilemap_columns`: For each of five columns, it either clears eight vertically spaced bytes at the destination or copies eight bytes from a source table, according to a bit in a mask.
- `animate_sprite_position_steps`: Initialises blitter globals, derives position fields from the structure's size and offset words, then loops over a count, reading a blitter address and advancing the position fields on each pass.
- `configure_blitter_from_object`: Programs the blitter control words from the object record, then loops over its step count, waiting for the blitter to go idle and advancing source and destination pointers on each pass.
- `copy_interleaved_word_pairs`: Copies sixteen words from one source into every other word of a destination table, then copies sixteen words from a second source offset by thirty-two bytes into the remaining positions.
- `scan_for_marker_then_call`: Advances a pointer two bytes at a time until it finds the four-byte marker that spells BODY, then calls a helper and returns.
- `decode_cmap_tile_entries`: Scans forward in 16-bit steps for the ASCII marker CMAP, then unpacks thirty-two three-byte records, each into one 16-bit value built from the upper nibble of the third byte, the second byte and the first byte shifted left by four.
- `run_nested_wait_loop`: Calls the same helper in a nested loop of thirteen outer and twenty inner passes, then returns the helper's last result plus sixteen.
- `init_record_fields_from_args`: Computes a record address from the index in D2 and the base pointer using a 160-byte stride, writes the low words of the two register arguments plus a fixed set of default size and position values into it, stores the pointer argument in a global, calls the helper and returns the two register arguments.
- `or_combine_five_planes`: Walks the full table of long-words and, at each position, ORs together five values spaced at a fixed plane stride, storing each combined result sequentially in a destination array.
- `or_fold_word_columns`: Repeats 520 blocks of a sixteen-column loop that ORs five 16-bit values read at a 32-byte stride, writing one combined word per column into a destination buffer; the source advances 160 bytes per block.
- `poll_byte_until_helper_zero`: Sets a status byte to 36, then repeatedly calls the helper and adds eight to a byte at an offset from the caller's pointer until the helper returns zero.
- `countdown_byte_until_helper_zero`: Sets a status byte to 244, then repeatedly calls the helper and subtracts eight from a byte at an offset from the caller's pointer until the helper returns zero.
- `decrement_packed_nibble_counters`: Runs sixteen rounds of twenty-six helper calls, then for each of thirty-two words in a global table decrements each non-zero nibble among its three lowest nibbles and writes the word back.
- `accumulate_nonzero_grid_cells`: Scans a grid of 13 rows by 20 columns of data bytes, calling a helper for each non-zero byte, and advances two running accumulators by sixteen per cell and per row.
- `call_helper_thirteen_times_2`: Calls the same helper thirteen times in a row using a countdown counter, then returns with no value.
- `clear_thirteen_slot_words`: Zeroes one word at the start of each of thirteen sixteen-byte slots in a global table, calls the helper, and returns the two register arguments.
- `lookup_scaled_table_word`: Forms a 32-bit value from the high word of the first argument and a word at a fixed offset from a pointer, reduces it modulo ten and halves it, then indexes a 22-byte-stride table with the result.
- `write_chip_register_from_table`: Looks up a 16-bit offset in a 22-byte-stride table using the second argument, writes the first argument as a word into the custom chip register at that offset, and returns both arguments.
- `clear_four_words_set_flag`: Zeroes four consecutive words of a global array, stores fifteen into a global value, then calls the helper and returns the two register arguments.
- `copy_sprite_def_free_slot`: Looks up a 22-byte definition by the first argument and checks a global bitmask and a pointer field.
- `clear_flag_bit_in_records`: Calls the helper once, then clears bit one of the byte at offset twenty in each of thirty-three 22-byte records in a global table.
- `update_global_word_by_division`: Divides a 32-bit value built from the high word of the D2 register and a global word by a constant, XORs the quotient into that global word, then rotates the word left by nine bits and XORs in a constant.
- `xor_then_rotate_table_bytes`: Walks a 900-byte table from the last entry back to the first, XORing each byte with four and rotating it left by three bits within the byte.
- `rotate_then_xor_table_bytes`: Walks a 900-byte table from the last entry back to the first, rotating each byte left by five bits and then XORing it with four.
- `set_pointer_and_call_helpers`: Calls the helper, stores a pointer into a global at the global base plus the D7 word times five, then calls the helper twice more.
- `compare_nul_terminated_strings`: Walks two byte pointers forward together, comparing bytes until the first string reaches its terminating zero or the bytes differ, then returns.
- `handle_key_input_state`: Dispatches a key code by state.
- `poll_serial_byte_change`: Sets bit six of a global control byte, compares the current byte with the previously stored value, and when they differ and bit zero is set calls the helper.
- `setup_pointer_between_helper_calls`: Calls the helper four times, stores the address of a global as a pointer, copies that pointer into a RAM word, then sets the pointer to a fixed constant and calls the helper four more times.
- `set_flags_and_update_records`: Sets the low two bits of a global flag byte, then walks thirty-three 22-byte records and, for each record whose second long is non-zero, overwrites its first long with a value from a global.
- `load_sfx_table_entries`: Calls a library routine through a pointer and, if it returns non-zero, walks thirty-three table entries.
- `call_pending_entry_callbacks`: Walks thirty-three 22-byte records and, for each whose first long is non-zero and second long is zero, calls a routine from the library table through a pointer derived from the library base.
- `open_dos_library_handle`: Calls an exec library routine through a pointer built from the library base and stores its return value in a global handle.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `wait_for_nonzero_timer_word`: Busy-waits until the word formed from a global byte repeated twice is non-zero, copies that word into another global, then writes a constant with bit fifteen set to the global control word and returns.
- `fill_eight_split_words`: Writes eight 8-byte slots in a global table, each receiving the high and low words of a running 32-bit value taken from a global, which advances by a fixed step after each slot.
- `fill_five_split_words`: Writes five 8-byte slots in a global table, storing the high and low words of the input register in each slot, and advancing the register by a fixed large step per slot.
- `call_helper_seven_times`: Calls the same helper seven times in sequence and returns.
- `transpose_byte_columns`: Calls a helper twice, then copies 640 bytes from a source buffer into a destination in sequence, reading them with a stride of forty bytes across two passes of forty rows by eight columns.
- `run_helper_sequence_with_settings`: Runs a fixed sequence of helper calls, setting one global to six and later to five and a second global to two 16-bit constants at points in the sequence.
- `edit_five_char_text_entry`: Fills a five-character buffer with underscore placeholders, then loops reading key codes: code two clears the buffer back to placeholders, code one deletes the last character, other codes are appended up to five characters, and codes three or four end the loop.
- `close_library_and_reset_state`: Calls a helper, then calls a library close routine through a pointer built from the library base, resets a global to fifteen, restores it from a saved pointer, sets it to a fixed 16-bit value, and calls a helper that starts the copper list.
- `load_entries_until_zero`: Walks a table of three-word entries, calling a library function through a pointer for each to store its result in the entry, and stops when the call returns zero.
- `walk_entries_until_negative`: Walks a table of three-word entries, skipping zero entries, returning when it reaches a negative entry, and calling a library function through a pointer for each positive one.
- `stub_interrupt_handler_undecoded`: The decompiler produced no pseudo-code for this four-instruction routine, so its behaviour is not recovered.
- `handle_vblank_interrupt`: The routine checks a bit in a custom hardware register and, if set, writes a value to a custom hardware register and calls another routine before returning.

### Jail Break (1990)(Odyssey)[cr PDX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_display_and_copy_tables`: Saves two control words, disables interrupts and DMA by writing a value with bit fifteen clear, zeroes a large block of memory, copies two 600-long tables into destination buffers, copies a table of words into a list, calls two helpers, writes two custom registers and waits for a busy flag before restoring the saved words.
- `call_helpers_store_record_field`: Calls one helper when bit two of a global flag is set, then calls five helpers in sequence and stores the constant 112 into a record field at offset 156.
- `tiny_ori_2`: Small routine of 1 instruction that merges 64 into d0.
- `tiny_and`: Small routine of 1 instruction that masks d0 with d0.
- `advance_scan_and_divide`: Decrements a countdown word and, when it reaches zero, resets it to eight and advances a scan index past entries equal to minus one.
- `advance_scan_and_setup_blitter`: Decrements a countdown word and, when it reaches zero, resets it to eight and advances a scan index past entries equal to minus one.
- `draw_blit_grid_rotating_mask`: Waits for the busy flag to clear, advances a column offset in a table, then fills a grid of twenty-two rows by eight columns, writing blitter register values while a mask rotates left by two bits each cell.
- `build_bounce_entry_list`: Updates a step value and bounces it between two limits, using the sum of two globals as the test value.
- `copy_words_until_zero_terminator`: Copies 16-bit words from one global buffer to another, stopping after the last word that precedes a zero word, then stores one global into another global.
- `build_indexed_entry_table`: Builds 105 eight-byte entries, each holding a fixed byte, a byte from a global, a zero word and a long read from a small table indexed by the next source byte.
- `fill_cycling_offset_entries`: Fills 105 eight-byte entries with a running code byte, a byte made from a source byte plus a cycling value from zero to three, a count byte and a zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_audio_channel_mode`: Dispatches on the low two bits of its first argument.
- `reset_record_fields_from_source`: Clears several word and byte fields of a record, copies a long from the data pointer into one field, stores a fixed address constant in another, sets a byte to 63 and sets a flag byte to one.
- `scramble_table_and_run_counters`: Fills a 16-word table from a rotate, add and XOR sequence, decrements a countdown byte that wraps when it goes negative, and calls four helpers.
- `advance_sequence_channel_state`: Advances one sequence channel record: it reads table values and a byte stream, updates an accumulator and step counters, and calls a handler from a jump table when a command byte is present.
- `store_nibble_field_return_mask`: Stores the low four bits of the second byte argument into a record field at offset forty-seven and returns the first argument ORed with 126.
- `store_low_nibble_field`: Stores the low four bits of the second byte argument into a record field at offset forty-seven and returns nothing.
