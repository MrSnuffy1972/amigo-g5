# Game 2198 (Faery Tale Adventure, The): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02198). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902096 top-level bytes in 23 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 447 of 447 named (0 author symbols), 447 of 447 described (68 by rule, 91 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2408 of 2408 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14643 of 14675 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 123458 |
| code | 49844 |
| data | 688916 |
| padding | 4550 |
| unused | 35328 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Type | 2 | 117 | 2 |
| Wait | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Faery Tale Adventure, The.adf:FMAIN | 3 | 13906 | 353 |
| 9 | 1 | 51 | 2 |
| Faery Tale Adventure, The.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 88 | 86 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EF8) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 59 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 149-159 (bytes 76288-81920) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.69)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.11)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.57)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 896-917 (bytes 458752-470016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 920-955 (bytes 471040-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 960-1171 (bytes 491520-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 1312-1370 (bytes 671744-701952) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.24)
- Faery Tale Adventure, The/Faery Tale Adventure, The.adf: blocks 1376-1501 (bytes 704512-769024) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_main_game_screen` (FMAIN): Initialises the game screen and sprite structures, computes five buffer addresses, copies a 32-byte palette block and runs a long sequence of setup and state calls. Kind: loader/integrity check. Pass path: the branch where iVar7 == 0 (the 'if' block continues execution).

## Functions


### Type

- `find_max_and_prepare_buffer_2`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_then_call`: Copies sixteen longwords from a fixed offset in the base block into a buffer passed on the stack, when that buffer pointer is non-null, then makes one indirect call.

### Wait

- `find_max_and_prepare_buffer`: Walks a linked list of entries to find the largest value stored in a field, then when a follow-up call succeeds writes a header of incrementing words and copies a sixteen-word block into the new entry.
- `copy_memory_block`: Copies a sequence of words from a source address to a destination address using a loop.

### Disk-Validator

- `validate_disk_bitmap_blocks`: Writes a status digit into a text line, initialises a table with an all-ones marker, then walks bit positions over a block range comparing two allocation bitmaps and calls indirect routines when they disagree.

### Faery Tale Adventure, The.adf:FMAIN

- `clear_block_and_store_globals`: Zeroes 947 longwords in the global data area, stores two global pointers in fixed slots and then makes one call.
- `open_libraries_and_init_display`: Opens the graphics and layers libraries and the trackdisk device, loads a font, allocates memory and sets up the display structures.
- `cleanup_flagged_resources`: Checks bit flags in two stored status bytes and, for each set bit, passes a stored pointer with its fixed size to an indirect routine.
- `index_six_variable_length_records`: Makes an indirect call with a size of 920 and a flag of eleven, then parses six consecutive length-prefixed records from a stored buffer, saving each record's start address and length in two tables.
- `open_nearby_door_tile`: Looks for a tile of code fifteen near a given position and, if found, rewrites the surrounding map cells using a lookup table.
- `setup_main_game_screen`: Initialises the game screen and sprite structures, computes five buffer addresses, copies a 32-byte palette block and runs a long sequence of setup and state calls.
- `step_position_until_blocked`: Computes the change from the previously stored position, updates the stored position and velocity fields, and when the flag is set recomputes a sector index.
- `scan_region_table_for_event`: Scans a table of three-byte regions for the one containing the current position, derives an event code from it and runs the matching indirect handler when the code changes.
- `update_mode_from_table`: Looks up a mode code in a six-byte-per-entry table selected by a stored index.
- `compute_target_position_with_offset`: Calls two indirect routines to read values, combines the stored position with an offset of 150 through two more indirect calls and stores the results as a target position.
- `place_random_candidate_entry`: Tries up to fifteen random offsets around a stored target position, stopping at the first candidate that an indirect check accepts.
- `reset_entry_and_apply_effects`: For the entry selected by a stored index, when its timer has run out and its type is not one of two reserved codes, it resets the timer and marks the entry, then applies a stat penalty or plays a sound by type.
- `set_selection_and_reset_fields`: Sets a stored selection code to six or five depending on whether the parameter is ten.
- `reset_state_and_start_stage`: Resets stored state variables, then either makes one indirect call or sets up the next stage record and its buffers, selecting a display or sound by stage number before the routine finishes.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `set_paired_offsets_from_parameter`: Derives several paired offset values from the parameter and a value returned by an indirect routine, stores them into field pairs and calls indirect routines with the results.
- `select_entry_from_position_state`: Chooses a table index from the current position range and state flags, then passes four consecutive entries from a table to one of two indirect routines, depending on a flag and the parameter.
- `track_grid_cell_changes`: Derives a small grid cell index from two stored position words and, when that cell changes, records it and calls a follow-up routine.
- `swap_pointer_pair_and_update`: Swaps two stored pointers, copies the first two words of the new target into two fields, runs three indirect routines on those fields, writes the second word back and makes one final indirect call.
- `open_input_device_request`: Opens the input device through indirect library calls, allocating a request block and attaching a handler buffer.
- `send_device_request_and_release`: Sets the command code of a stored request block to ten and points it at a buffer, then makes three indirect calls on that block and one indirect call on the stored handle.
- `collect_flagged_table_entries`: Walks a table selected by a stored index, passes each entry whose flag bit is set to a follow-up routine, then fills any remaining of up to twelve slots with alternating constants and marks them empty.
- `place_indexed_entry_value`: Maps a stored index to a value using special cases for two indices and a lookup table, then makes a series of indirect calls that place the value at positions derived from the parameters, choosing one of two constants by the low bit of the first parameter.
- `dispatch_state_and_event`: Dispatches on a stored state word and an event code.
- `retry_poll_up_to_limit`: Clears a counter and repeatedly calls a routine with a stored coordinate pair until it returns five or the counter reaches twenty-five.
- `notify_then_dispatch_event_five`: Makes one indirect call with the constant one and a stored coordinate pair, then calls the state dispatch routine with five.
- `set_state_and_dispatch`: If a status bit is clear, stores the parameter as the current state word, clears a companion byte and calls the state dispatch routine.
- `fill_byte_tables_from_source`: Fills several byte tables in a global block by calling an indirect routine with consecutive index values.
- `wait_until_index_reaches_ten`: Repeatedly calls one routine while a stored index word is below ten, and returns once the index reaches ten.
- `push_changed_entry_fields`: Compares the selected table entry's six values with cached copies and, for the first value that differs, passes the new value through indirect calls, caches it and returns.
- `forward_indexed_entry_if_enabled`: If a status bit is set, makes an indirect call using a table entry selected by the parameter and a value from a second table, with the second argument passed through.
- `prepare_buffers_and_select_pointer`: Passes three 35-byte buffers and one 60-byte buffer to an indirect routine, then selects a pointer from a table using a stored index minus one.
- `store_argument_in_node_field`: Stores the single stack argument into the field four bytes into the structure that the address register points to.
- `update_linked_object_positions`: Walks a linked list of objects, applying each object's offset to its stored position and clamping the result to fixed bounds.
- `advance_linked_entries_clamped`: Walks a linked list of entries, computing a tile code from the stored position when a flag applies, then adds each entry's offset to its position and clamps the result to fixed bounds, calling a callback when one is set.
- `advance_linked_entries_bounded`: Walks a linked list of entries, taking queued codes from them and computing a tile code from the position when a flag applies.
- `read_ring_buffer_byte`: Returns the next byte from a 128-entry circular buffer, or zero when the read and write indices are equal, and advances the read index.
- `next_pseudo_random_value`: Advances a stored 16-bit linear congruential state by multiplying by a fixed multiplier and adding one, then returns a value built from the new state with its bits shifted and masked.
- `mask_random_with_parameter`: Calls the pseudo-random generator and returns the bitwise AND of its result with the parameter.
- `pick_random_bit`: Calls the pseudo-random generator and returns its lowest bit, which is zero or one.
- `pick_random_two_bits`: Calls the pseudo-random generator and returns its low two bits, giving a value from zero to three.
- `pick_random_three_bits`: Calls the pseudo-random generator and returns its low three bits, giving a value from zero to seven.
- `pick_random_six_bits`: Calls the pseudo-random generator and returns its low six bits, giving a value from zero to 63.
- `return_random_byte`: Calls the pseudo-random generator and returns the low byte of its result.
- `combine_random_modulo_parameter`: Calls the pseudo-random generator, takes its low 16 bits modulo the low 16 bits of the parameter, and returns that remainder in the high word together with the parameter in the low word.
- `invoke_helper_then_library_call`: Calls one helper routine, then makes an indirect call through a library base stored in a global slot and returns its input registers unchanged.
- `format_number_as_decimal_text`: Writes an unsigned number as decimal digits into a ten-byte text buffer from the right, filling unused leading positions with spaces.
- `write_decimal_digits_with_padding`: Writes the decimal digits of a number into a buffer from a passed-in start index moving leftward, and fills the remaining positions with spaces once the number is exhausted.
- `clear_buffer_with_spaces`: Fills a buffer with space characters from a passed-in index down to the start.
- `run_nested_callback_sweep`: Runs three nested loops of fixed counts that call indirect routines on each pass, adds extra calls once the outer counter passes nine, and returns its input registers unchanged.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `print_string_with_escape_codes`: Walks a zero-terminated string.
- `lookup_masked_nibble_at_position`: Derives a table position from two packed coordinate parameters and stored offsets, then returns the upper nibble of a table entry when a mask built from the parameter bits is set, otherwise zero.
- `lookup_masked_nibble_from_registers`: Derives a table position from two packed coordinate values passed in registers and stored offsets, then returns the upper nibble of a table entry when a mask built from the value bits is set, otherwise zero.
- `call_library_then_drop_four`: Makes an indirect library call through a stored base and then decrements a stored stack pointer by four bytes.
- `call_library_then_subtract_block`: Makes an indirect library call through a stored base, then subtracts a stored length plus four from a stored stack pointer.
- `call_routine_sequence_with_arg`: Makes one call with a stored argument, then a long unbroken run of calls to other routines with no arguments.
- `call_six_routines_in_order`: Makes six consecutive calls to other routines, with no arguments and no results.
- `copy_block_to_multiple_buffers`: Copies 16 iterations of word pairs from a source block, selected by a parameter, into five destination buffers, advancing the destination by eighty bytes per iteration.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `forward_stored_value_to_routine`: Passes a stored global value to one routine and returns the input registers unchanged.
- `forward_value_then_run_sequence`: Passes a stored value to one routine, then makes a long run of calls to other routines with no arguments, and returns its input registers unchanged.
- `forward_call_to_routine`: Calls one other routine with no arguments and returns, without touching any data itself.
- `process_map_cell_window`: Clamps two packed position values to fixed limits, then walks a 9-by-18 window of map cells, looking up each cell's byte in a table and passing the derived address to an indirect routine.
- `unpack_five_bitplane_rows`: Expands packed source bytes through a lookup table into five bitplane words for eight rows, writing each plane's row into five destination arrays and a second copy 40 bytes further on.
- `copy_rows_of_four_words`: Copies eight rows of four 16-bit words from a source block selected by an index into a destination position computed from the arguments, advancing by the row width each time.
- `compute_tile_pointer_for_position`: Converts two packed position values to a cell offset by subtracting stored origin offsets and wrapping the cell coordinates, then combines the offset with a looked-up cell code and returns it added to a stored base address.
- `fill_cell_window_buffer`: Fills a buffer of 19 rows by 6 columns with cell codes looked up around a position, then stores one more looked-up code at a stored position, adding 256 when a stored level counter is above seven.
- `decompress_run_length_stream`: Copies bytes from a stored source pointer into the output.
- `scale_value_by_table`: When the second parameter selects one of eight table entries, multiplies the third parameter by that entry, halves it, adds it to the first parameter and keeps the result within fifteen bits.
- `scale_value_with_sign`: Like the table scaling routine, but keeps the sign bit of the first parameter: when the selector is below eight, it adds the halved table product to the lower fifteen bits and restores the original top bit.
- `copy_bit14_into_bit15`: Copies bit 14 of the value into bit 15 and returns the result, so the top bit follows the bit below it.
- `clear_top_bit_of_register`: Returns the register value with its top bit cleared, keeping the low fifteen bits.
- `step_position_toward_target`: Compares a target position with the stored position and moves the stored x and y by one unit toward it when they differ by more than a small threshold, snapping to a fixed offset from the target when they are far apart.
- `step_position_from_register_targets`: Moves the stored x and y positions one step toward target values held in registers, snapping them to a fixed offset from the target when the distance is large.
- `step_position_by_one_unit`: Moves the stored x position one unit toward a register value when the difference exceeds twenty, and steps or snaps the stored y position toward another register value.
- `nudge_x_and_step_y`: Increments the stored x position when a register value exceeds twenty, and steps or snaps the stored y position toward a target held in registers.
- `step_y_toward_target`: Steps the stored y position one unit toward a register-held target when the difference exceeds ten, or snaps it to a fixed offset from the target when the difference is large.
- `nudge_y_by_one_unit`: Moves the stored y position by one unit in the direction of a register value when that value lies more than ten away from zero.
- `nudge_y_up_when_positive`: Increments the stored y position by one when a register value exceeds ten.
- `handle_error_message`: When the parameter is not two, makes three routine calls with fixed arguments, one of them using a global pointer, and the routine references error text.
- `classify_position_into_band`: Looks up a byte from a table by the parameter, then for certain ranges of the parameter overrides it with a small code from three to ten, and returns the result.
- `select_direction_code_from_input`: Reads joystick movement bits or stored coordinates, depending on a mode bit, and maps them to a direction code through a table, calling a handler when the code changes.
- `process_joystick_input`: Reads bits from custom chip registers to determine direction, updates a global state, and calls a handler if the direction changes.
- `read_joystick_direction_code`: Reads the joystick port movement bits from the custom chip register, maps the pair to a direction code through a table and calls a handler when the code changes.
- `update_direction_from_timer`: Subtracts twenty from a stored timer and, when the result falls outside zero to nine, resets the timer and uses neutral code nine.
- `set_neutral_direction_code`: Clears a stored timer and, if the stored direction code is not nine, sets it to nine and calls a handler with nine.
- `lookup_with_secondary_check`: Calls one lookup routine and returns its result, unless that result is a small value other than one, in which case it calls a second lookup.
- `combine_five_sources_inverted`: Loops over rows of words, writing the bitwise inverse of the AND of five source words into a destination word for each position.
- `setup_blitter_for_five_planes`: Programs the blitter hardware registers for five entries, writing control, mask, modulo and pointer values with offsets taken from a table selected by the parameter, and makes library calls before and after the register writes.
- `set_blitter_regs`: Writes a fixed control value and zero to the blitter control and modulo registers, sets the destination pointer and blit size from its two arguments, and returns zero.
- `setup_blitter_masked_planes`: Programs the blitter for five rows, writing the control registers, first and last word masks, source and destination pointers and modulo values, using a stored shift count and a table, with library calls around the writes.
- `program_single_blit_region`: Programs one blit with a fixed control value, word masks, source and destination pointers and modulo registers, using stored positions and a library call before the writes.
- `program_blit_rows_from_base`: For five rows, programs the blitter control, mask, modulo and destination registers from a stored table, with each row offset by a multiple of a stored step plus the parameter.
- `program_blit_columns_offset`: For five entries, programs the blitter control, mask, modulo and destination registers, where each entry's offset is a multiple of a stored step plus the parameter, with a library call before the loop.
- `forward_indirect_call`: Makes one indirect call through a stored function slot and returns.
- `execute_indirect_call`: Performs an indirect call to a function pointer stored in a global variable.
- `compute_direction_code_between`: Works out the signed distances between two positions and their absolute sizes, looks up a direction code from the signs and magnitudes, and stores it in a record field after an adjustment from an indirect call.
- `compute_direction_code_between_2`: Works out the signed distances between two records and their absolute sizes, looks up a direction code from the signs and magnitudes, and stores it in a record field after an adjustment from an indirect call.
- `compute_direction_code_from_registers`: Works out a direction code from signed distances held in registers, using the same sign and magnitude tests and table, and stores it in a record field.
- `compute_direction_code_from_deltas`: Works out a direction code from signed distances held in registers and a stored position, using sign and magnitude tests and a table, then stores it in a record field.
- `store_direction_from_sum`: After an indirect call, conditionally negates one value and stores the masked sum of two values into a record field, setting a second field to a fixed code unless a register holds five.
- `store_byte_difference`: Stores the masked difference of two register values into a record field, and when a register does not hold five, stores twelve into an adjacent field.
- `apply_hit_and_place_entry`: Subtracts an amount from an entry's counter field and clamps it at zero, then places a new entry at a randomised offset chosen by a mode argument.
- `apply_record_hit_and_place`: Subtracts an amount stored in a record from an entry's counter field and clamps it at zero, places a new entry at a randomised offset chosen by a mode argument, then runs a follow-up check when the entry type qualifies.
- `subtract_counter_and_place_entry`: Subtracts a per-record amount from an entry's counter, clamps it at zero, places a new entry at a randomised offset selected by a mode code, and makes a follow-up check when the entry type qualifies.
- `place_entry_by_mode`: Chooses a placement mode from a flag and a mode code, places a new entry with a random offset, then, when the entry type qualifies, runs a follow-up check and a second routine.
- `place_entry_flag_then_check`: Chooses between two placement modes from a flag, places a new entry with a random offset, then runs the same type-qualified follow-up check.
- `place_entry_fixed_then_check`: Always places a new entry with a random offset in one fixed mode, then runs the type-qualified follow-up check.
- `count_entry_types_and_notify`: Counts entries of two particular types within a range of records, then, when a stored counter is positive and below a threshold, makes calls with those counts and a global pointer.
- `process_entry_counts_2`: Iterates through a list of entries to increment counters based on byte values and then performs conditional logic and calls to other routines based on the final counter totals and global values.
- `increment_entry_counters`: Iterates through a list of entries using a function pointer to retrieve offsets, incrementing specific counters if certain byte values are found.
- `process_entry_counts`: iterates through a list of entries to increment counters based on specific byte values and calls other routines to report status if certain thresholds are met.
- `report_entry_counts_and_status`: Reports counts of two kinds of entries with text messages when a stored counter is below fifty and each count is non-zero, then runs a closing status routine when a flag is set.
- `report_fled_or_defeated_counts`: Reports counts of fleeing and defeated foes with the matching message text when a stored counter is below fifty, then runs a closing status routine when a flag is set.
- `check_proximity_to_other_entries`: Returns sixteen when a position lies within about eleven columns and nine rows of another live entry, skipping the given index and the first slot, and zero when none is close.
- `check_record_proximity_to_entries`: Returns sixteen when a record's position lies within about eleven columns and nine rows of another live entry, skipping the record's own slot and the first slot, and zero when none is close.
- `check_record_proximity_from_index`: Scans entries upward from a starting index and returns sixteen when the record lies within about eleven columns and nine rows of one of them, otherwise zero.
- `scan_for_entry_within_reach`: Scans entries for one whose position lies within about ten columns and eight rows of the record, returning sixteen when one is found and zero when none remain.
- `find_nearest_entry_index`: Scans entries from the second slot up to a stored limit and records the index whose distance value is smallest in a stored global.
- `find_nearest_entry_for_record`: Scans entries from the first slot up to a stored limit, computing a distance for each, and stores the smallest distance in the record and its index in a stored global, then returns that smallest distance.
- `compute_approximate_distance`: Takes the absolute horizontal and vertical differences between two records.
- `call_indirect_pair_2`: Executes two consecutive indirect function calls if a comparison between two values is met.
- `call_indirect_pair`: Makes two indirect calls in sequence with no arguments and returns.
- `try_move_entry_to_position`: Computes a new position for an entry from two indirect transformations of its stored coordinates, tests it with a helper routine, and stores the new position only when the test returns zero.
- `store_entry_position`: Stores two register values as the position of the entry selected by an index in a record field, and returns one.
- `submit_indexed_frame_pair`: Looks up a four-byte frame descriptor for the given index and passes it to an indirect routine with a fixed set of arguments, then does the same with a second descriptor for indices below nine.
- `build_palette_table_from_level`: Builds a 32-entry table of colour values packed as red, green and blue nibbles, using a stage-dependent base and a clamped level argument, and adds small random boosts when the mode argument is set.
- `randomize_palette_frames`: Repeats 32 times: fills palette entries one to thirty-one with random values bounded by 4095, loads them through an indirect call and then makes a second indirect call with a delay argument of one.
- `dispatch_queued_event_code`: Pops the next code from a 32-entry circular queue when one is pending and dispatches it through a switch.
- `dequeue_and_dispatch_command`: Takes the next byte from a 32-slot ring buffer and dispatches on its value.
- `dispatch_command_from_register`: Dispatches on the value held in the data register instead of reading a queue.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `enqueue_byte_if_not_full`: Appends the byte passed in to a 32-slot ring buffer when the buffer is not full, advancing the write index.
- `draw_string_with_length`: Measures the length of a zero-terminated string, then makes one library call with fixed numeric arguments, a second call with two fixed values, and finally passes the string and its length to a library routine.
- `invoke_library_setup_sequence`: Makes three library calls in order: a seven-argument call using a library base value read from a global, a two-argument call, and a call with no arguments.
- `pass_string_with_length`: Measures the length of a zero-terminated string and passes the string pointer and that length to one routine.
- `wrap_text_into_lines`: Copies a zero-terminated string into a line buffer, starting a new line at a space or carriage return or when the line passes about 37 characters.
- `resume_text_wrap_frame_loop`: Continues the text-wrapping loop with its position kept in a stack frame, copying characters into a line buffer and passing finished lines to the output routine.
- `resume_text_wrap_after_char`: Re-enters the text-wrapping loop after one character has been consumed, then continues filling lines and passing finished lines to the output routine.
- `process_text_buffer_character`: Processes a single character from a source buffer and appends it to a line buffer while managing text wrapping.
- `flush_char_then_wrap_text`: Writes one pending character into the line buffer, then continues the text-wrapping loop.
- `process_formatted_string`: Parses a source string to build a destination buffer, handling special characters such as carriage returns, null terminators, and percent-sign expansion.
- `return_immediately`: The routine is empty and performs no operations.
- `wrap_text_from_string_index`: Skips a count of zero-terminated strings in a fixed text table, then runs the text-wrapping loop from that point and passes finished lines to the output routine.
- `wrap_second_table_text`: Skips a count of zero-terminated strings in a second fixed text table, then runs the text-wrapping loop from there and passes finished lines to the output routine.
- `wrap_text_after_skipping_strings`: Skips a count of zero-terminated strings starting at a given pointer, then runs the text-wrapping loop from there and passes finished lines to the output routine.
- `print_table_entry_with_separators`: Outputs the entry selected by an indexed pointer table, then a shared global string, then the caller's argument, then the shared global string again.
- `print_table_entry_then_argument`: Outputs the entry selected by an indexed pointer table, then a shared global string, then the caller's argument.
- `print_indexed_table_entry`: Outputs the entry selected by an indexed pointer table through the output routine.
- `initialise_record_and_copy_block`: Stores a pointer to a sub-record and calls three library functions on the record, one of them copying a 32-byte block, then sets a state byte to 2.
- `select_second_state_record`: Points a state field at a second record, stores a fixed 16-bit value in a control word, calls two routines, and sets a state byte to 3.
- `sweep_values_down_by_five`: Loops 21 times from 100 down to 0 in steps of five, calling a routine with the current value and a fixed buffer address, then calling a library function with 1 on each pass.
- `sweep_values_up_by_five`: Loops from 0 up to 100 in steps of five, calling a routine with the current value and a fixed buffer address, then calling a library function with 1 on each pass.
- `clear_record_position_fields`: Zeroes two 16-bit fields of the record reached through a global pointer, then calls one routine.
- `issue_parameter_call_sequence`: Copies a 32-bit value between two globals and then makes a fixed sequence of calls passing small constants, a table-derived index, and values read from globals.
- `pack_entry_fields_within_limit`: Reads values through a library call and stores them as 16-bit and 32-bit fields in a buffer at offsets the library returns, using an entry from a three-byte table.
- `fill_slot_from_template`: Selects a 56-byte slot by index and calls a library function on it when its state is two.
- `copy_template_into_fixed_record`: Copies a 14-longword template from a global pointer into a fixed record, sets two fields to zero and nine, then passes the record to a library function.
- `call_through_and_return`: Makes one call to another routine and returns, with no other operations.
- `submit_table_entry_record`: Calls a library function on a fixed record, clears a word field in it, then passes five values from an 18-byte table entry selected by the argument to a routine.
- `call_routine_unless_slot_done`: Returns early when the current slot is in state two and a library call on it returns zero.
- `load_song_data_blocks`: Opens the file named by a global string, reads size-prefixed blocks up to 28 entries and a fixed byte limit, and records each block's offset in a table.
- `setup_display_when_flag_clear`: When a flag byte is zero, it calls a library function with a fixed value, sets up a display structure with fixed width and height values, and calls two library functions with the given arguments.
- `draw_strip_rows_from_tables`: Loops over 22 rows of tables, drawing vertical runs and strips through library calls with computed offsets and widths.
- `check_code_equals_space`: Calls a library function for a code, stores a flag recording whether the code equals the ASCII space value 32, and returns the same result.
- `copy_bytes_to_new_allocation`: If the source address is above a fixed threshold, it allocates a new buffer of the given length through a library call and copies that many bytes into it.
- `draw_quad_with_winding_flags`: Computes a quadrilateral from two sets of four corner offsets read from a table entry.
- `set_state_and_call_tables`: Sets a state word to two, passes a fixed 128-byte length and two table-selected values to routines, then copies the state word into another field when it exceeds three.
- `copy_table_entry_into_slot`: Copies a 16-bit value from a 22-byte table entry selected by the argument into a fixed slot.
- `set_record_state_by_index`: Finds a record through a table lookup whose table is chosen by a flag bit.
- `set_record_state_from_word`: Finds a record through a word index into a table, then applies the same update: a state byte of 15 becomes 29, otherwise the argument byte is stored in the following field.
- `store_byte_to_offset`: Stores the byte held in a data register into the memory location five bytes past the address register pointer.
- `spawn_points_and_scan_list`: Spawns up to ten new entries at random positions from a library call when a count condition holds, each with a type index from 0 to 15.
- `spawn_ten_points_then_scan`: Creates ten entries at random positions from a library call, each with a type index from 0 to 15, then walks a record list and adds or updates entries for records inside a fixed rectangle.
- `scan_record_list_for_entries`: Walks a list of six-byte records and, for those inside a fixed rectangle, updates the current entry or adds a new one.
- `resume_record_list_scan`: Resumes the record-list scan partway through its loop body, advancing the record pointer and count, then updating or adding entries for records inside the rectangle and calling routines on a type change.
- `scan_record_list_with_skip`: Walks a list of six-byte records, skipping those outside a fixed rectangle, and updates or adds entries for the rest.
- `scan_record_list_flag_update`: Continues the six-byte record-list scan, writing a new entry's fields when an in-rectangle record is added.
- `scan_record_list_entry_fill`: Finishes filling a new list entry with state 13, or 15 for one record type, and copies the record's position into it.
- `scan_record_list_add_fresh`: Walks a list of six-byte records and adds a fresh entry for each in-rectangle record not yet tracked, choosing its subtype from a three-way test on the type byte.
- `scan_record_list_subtype_fill`: Walks a list of six-byte records and tracks in-rectangle records, setting a subtype byte to 0 for type five and 1 otherwise.
- `scan_record_list_sets_subtype`: Walks a list of six-byte records and tracks in-rectangle records, setting the entry subtype to 2 for type six, 0 for type five and 1 otherwise.
- `return_without_action`: An empty routine whose pseudo-code is a bare return with no operations.
- `prompt_and_match_three_words`: Repeatedly picks one of three pending strings with a library random call, reads a typed line of up to nine printable characters with backspace editing, and compares it with the chosen string.
- `run_paired_library_calls`: Calls three library functions through a library base held in a register, then makes two pairs of calls through a second library base read from a global pointer.
- `call_and_store_nonzero_result`: Calls a routine with two arguments and stores its result in a global.
- `acquire_df0_handle_checked`: Calls a library function with the text df0: and the value minus two, and stores the returned handle.
- `null_routine`: The routine is empty and contains no executable instructions.
- `wait_for_flag_then_clear`: Polls a flag byte up to 300 times, calling a routine with the value five on each pass to wait.
- `poll_flag_with_timeout`: Checks a flag byte, and if it is clear calls a routine with the value five and counts the pass, up to 300 passes.
- `poll_flag_until_timeout`: Waits through repeated calls to a routine with the value five, counting passes up to 300.
- `load_slot_file_tables`: Builds a file name from a letter derived from the argument, then opens that file from a path chosen by a flag.
- `load_winpic_state_tables`: Retries disk access in a loop, then opens a file named from a slot letter and copies several fixed-size state tables from it.
- `load_slot_with_disk_prompt`: Keeps trying to open a slot file while showing the referenced prompt and abort messages through a text routine.
- `retry_load_slot_file`: Polls disk status in a loop, and when a slot file opens it copies the fixed-size state tables and resets state fields.
- `copy_slot_file_tables`: Opens the slot file named from a letter, copies the fixed-size state tables from it when it opens, and closes it.
- `load_slot_tables_direct`: Opens the slot file without a retry loop, copies the fixed-size state tables from it, and closes it.
- `finish_save_game_with_error`: Shows the referenced game-save error message when its result is nonzero, then waits until the file operation completes.
- `finish_load_game_with_error`: Waits for a file operation to complete, then resets several state fields when a flag is clear and restores the display.
- `wait_disk_then_reset_state`: Shows the referenced please-insert-game-disk message and repeats a file operation until it returns nonzero.
- `reset_state_fields_and_redraw`: When a flag is clear, resets several state bytes and words, makes calls that set a value and clear two fields, then issues the final library call on the display record.
- `file_op_with_error_flag`: Performs a file operation on the open handle with two arguments, either through a library function or an alternate routine depending on a flag.
- `file_op_store_result_status`: Calls a library file operation with the handle and two frame arguments and stores the returned value in the frame.
- `set_rectangle_entry_around_point`: Stores four 16-bit edge values into a 12-byte table entry chosen by index, placing the edges 250 to the left and right of the given x and 200 above and below the given y.
- `advance_scene_setup_sequence`: Runs a fixed sequence of library and routine calls that redraw a display and set several state fields, then advances a counter.
- `animate_winpic_palette_steps`: Loads a colour table for the referenced winpic image, then loops through steps that copy shifted windows of the table into a 32-byte buffer.
- `fade_palette_steps_loop`: Loop form of the palette step animation: it clears the working colour buffer, copies shifted colour windows into it, applies random or fixed bias values, sets the palette and waits.
- `fade_palette_steps_inner`: Palette step animation that rebuilds each 32-byte colour buffer from a shifted window, applies a bias from a random source or fixed values, and sets the palette with a delay per step.
- `select_size_by_flag_byte`: Returns 10 when a byte from a table at the given index is nonzero, and 8 otherwise.
- `select_position_offset_call`: Returns a 2-bit state value.
- `call_routine_at_position`: Returns a 2-bit state value.
- `call_routine_with_fixed_position`: Calls one routine with the constants 100, 100, 100 and 1 plus a fixed buffer address, then returns.
- `dispatch_actor_action_mode`: Dispatches on an action code for a stored actor record.
- `dispatch_actor_action_alt`: Action-code dispatcher for an actor record that matches the main dispatcher but gates some routine calls on a random check drawn from a library call.
- `update_actor_distance_state`: Handles the proximity and state cases of the actor action dispatcher.
- `run_actor_action_when_flagged`: Action-code dispatcher for an actor that makes its routine calls only when a flag argument is nonzero.
- `process_actor_action`: The routine calculates direction codes based on signed distances and magnitudes, and updates a state byte in a record.
- `dispatch_actor_action_short`: Action-code dispatcher for an actor record covering codes 2 to 6 and 10, without the distance case.
- `dispatch_actor_action_no_random`: Action-code dispatcher for an actor covering codes 2, 3, 5, 6 and 10 only.
- `dispatch_actor_targets_only`: Action-code dispatcher for codes 2, 5, 6 and 10.
- `dispatch_actor_code_subset`: Action-code dispatcher for an actor covering codes 2, 6 and 10.
- `dispatch_actor_codes_six_ten`: Action-code dispatcher for codes 6 and 10.
- `call_table_target_when_flagged`: When a flag argument is nonzero, it picks a table target and calls a routine with it, then returns.
- `dispatch_code_ten_state`: When the action code is 10, it calls a routine if a flag argument is nonzero, then stores 12 into a state byte of the actor record.
- `subtract_counter_and_trigger`: Subtracts the argument from a 16-bit counter.
- `print_yum_message`: Passes the referenced text, which reads as Yum!, to the output routine and returns.
- `locate_and_load_record_rows`: Opens a data file with a fixed identifier and finds a matching record in a table by comparing it with a global key.
- `advance_counter_and_flag`: Clears a flag byte in the given structure and adds a 16-bit field to a 32-bit counter at a fixed offset.
- `interpret_script_commands`: Interprets a byte-coded script stream.
- `decrement_counter_and_update_state`: Writes a fixed word to two fields of an object record, then decrements a byte counter in the second argument when it is nonzero.
- `start_audio_channel_one`: Starts audio channel 1 by writing a sample pointer, length and period from the arguments to its registers, setting its volume to 64, clearing its interrupt request, enabling its interrupt, and turning on DMA for that channel.
- `silence_audio_channel_one`: Clears a flag bit in a state byte, then sets audio channel 1's volume to zero and its period to 2.
- `store_four_longs_in_globals`: Stores four 32-bit arguments into four fixed fields of the global data area.
- `init_audio_channel_state`: Initialises audio state by storing four arguments in two fields each, zeroing the volume registers of all four audio channels, and copying four words from a table entry into state fields.
- `mute_all_audio_channels`: Zeroes the volume of all four audio channels, sets a state byte to 255, and disables DMA for channels zero to three.
- `install_music_vblank_interrupt`: Sets up interrupt server and vector entries that name audio and music vblank handlers, installs them through the exec library's add-server and set-vector calls, and writes the interrupt enable and request registers for audio.
- `remove_music_vblank_interrupt`: Removes the music vblank interrupt server through the library, clears the audio interrupt enable bit in the custom chip, and restores the previous interrupt vector through the library's set-vector call.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_block_and_store_values`: Zeroes a block of 947 longwords starting at an offset derived from a global index, stores a register value and a global into two fields of that block, then calls a routine.
- `merge_zero_into_register`: The routine merges a zero value into a data register.
- `open_dos_and_parse_arguments`: Opens dos.library through the library base and returns early if it is missing.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `copy_string_pad_zeros`: Copies bytes from a source string into a destination until a null terminator is reached or the count runs out, then fills any remaining bytes of the count with zeros.
- `append_string_bounded`: Appends a source string to the end of a destination string, copying at most 32766 bytes.
- `copy_string_with_limit`: Appends a source string to the end of a destination string, stopping at a null terminator or when the counter in the third argument expires.
- `copy_string_to_buffer`: Copies a source string into a destination buffer including its terminator, and returns the destination pointer.
- `multiply_32_bit_values`: Multiplies two 32-bit register values using their 16-bit halves, producing the low 32 bits of the product in the high half of the returned 64-bit value.
- `apply_sign_to_result`: Calls a routine on the register operands, then negates its result when exactly one of the two inputs is negative.
- `negate_result_if_first_negative`: Calls a routine, then negates the value it leaves in the second register when the first input is negative, returning that value with the first operand.
- `call_and_return_concatenated`: Calls a routine and returns a 64-bit value built from the original second register and the value the call leaves in that register.
- `self_recursive_call_2`: Contains only a call to itself followed by a return, so each invocation recurses without a base case.
- `divide_32_bit_with_remainder`: Divides a 32-bit dividend by a 32-bit divisor and returns the quotient and remainder in a 64-bit value.
- `compute_string_length`: Walks a zero-terminated string to its terminator and returns its length, without changing memory.
- `call_optional_hook_then_routine`: Calls a stored hook routine if its pointer is set, then calls another routine and returns.
- `finish_stored_value_cleanup`: Calls one routine 20 times, invokes a stored hook if one is set, and calls routines on up to three stored values.
- `free_table_entry_by_index`: Checks that a table index is in the range 0 to 19 and that the entry is in use, otherwise setting error code 3 and returning minus one.
- `call_library_vector_36`: Calls the library function at offset minus 36 from the library base kept in a global variable, then returns.
- `call_library_vector_36_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_144_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_132`: Calls the library function at offset minus 132 from the library base kept in a global variable, then returns.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_84`: Calls the library function at offset minus 84 from the library base kept in a global variable, then returns.
- `call_library_vector_30`: Calls the library function at offset minus 30 from the library base kept in a global variable, then returns.
- `call_library_vector_30_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Calls the library function at offset minus 60 from the library base kept in a global variable, then returns.
- `call_library_vector_42`: Calls the library function at offset minus 42 from the library base kept in a global variable, then returns.
- `call_library_vector_66`: Calls the library function at offset minus 66 from the library base kept in a global variable, then returns.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_vector_90`: Calls the library function at offset minus 90 from the library base kept in a global variable, then returns.
- `call_library_vector_48`: Calls the library function at offset minus 48 from the library base kept in a global variable, then returns.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_468_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_414`: Calls the library function at offset minus 414 from a second library base kept in a different global variable, then returns.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_slot_request_block`: Reserves a free slot index through a helper, then allocates a 34-byte block with public and cleared flags.
- `release_allocated_block`: Releases a block allocated for a slot.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_stored_function_48`: Passes its argument and the value 48 to a function pointer kept in a global variable, then returns.
- `call_stored_function_with_arg`: Passes its argument to a function pointer kept in a global variable, then returns.
- `allocate_typed_block_if_requested`: Returns zero when the first argument is zero.
- `reset_block_and_release`: Returns zero for a null block.
- `reset_block_state_fields`: Marks a block's state byte as 255, sets two of its fields to minus one, and calls a stored function on the block.
- `call_library_vector_198`: Calls the library function at offset minus 198 from the library base kept in a second global variable, then returns.
- `call_library_function`: The routine performs an indirect call to a function via a pointer retrieved from a library structure.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_294`: Calls the library function at offset minus 294 from the library base kept in a second global variable, then returns.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Calls the library function at offset minus 132 from the library base kept in a second global variable, then returns.
- `call_library_vector_210`: Calls the library function at offset minus 210 from the library base kept in a second global variable, then returns.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Sets up a self-referencing pointer chain in a block: the first longword becomes the block's own address plus four, the next longword is cleared, and the third stores the block's address.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_552`: Calls the library function at offset minus 552 from the library base kept in a second global variable, then returns.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_258`: Library stub: loads a library base from a global variable and calls the library function at offset minus 258 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Calls the library function at offset minus 30 from the library base kept in a third global variable, then returns.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_462`: Calls the library function at offset minus 462 from the library base kept in a third global variable, then returns.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Calls the library function at offset minus 414 from the library base kept in a third global variable, then returns.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Calls the library function at offset minus 198 from the library base kept in a third global variable, then returns.
- `call_library_vector_468`: Calls the library function at offset minus 468 from the library base kept in a third global variable, then returns.
- `call_library_vector_360`: Calls the library function at offset minus 360 from the library base kept in a third global variable, then returns.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216`: Calls the library function at offset minus 216 from the library base kept in a third global variable, then returns.
- `call_library_vector_240`: Calls the library function at offset minus 240 from the library base kept in a third global variable, then returns.
- `call_library_vector_210_2`: Calls the library function at offset minus 210 from the library base kept in a third global variable, then returns.
- `call_library_vector_456`: Calls the library function at offset minus 456 from the library base kept in a third global variable, then returns.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354`: Calls the library function at offset minus 354 from the library base kept in a third global variable, then returns.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Calls the library function at offset minus 60 from the library base kept in a third global variable, then returns.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Calls the library function at offset minus 36 from the library base kept in a fourth global variable, then returns.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_150`: Calls the library function at offset minus 150 from the library base kept in a fourth global variable, then returns.
- `call_library_vector_144`: Calls the library function at offset minus 144 from the library base kept in a fourth global variable, then returns.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_routine`: The routine contains no executable instructions.

### Faery Tale Adventure, The.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spin_forever_no_operations`: Performs no work: its body is an endless loop that never returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `spin_forever_no_operations_2`: Performs no work: its body is an endless loop that never returns.
- `null_routine_2`: The routine is empty and contains no valid instructions.
- `spin_forever_no_operations_3`: Does nothing and never returns: its pseudo-code is an endless loop with an empty body.
- `perform_infinite_loop_3`: The routine executes an infinite loop that prevents further instruction execution.
- `enter_infinite_loop_17`: Executes an unconditional loop that prevents further instruction flow.
- `null_instruction_gap`: This routine is empty because the instruction stream contains invalid data that prevents disassembly.
- `execute_empty_instruction`: The routine contains no valid instructions.
- `null_routine_3`: This routine is empty and contains no executable instructions.
- `execute_invalid_instruction_9`: Contains data that does not form a valid instruction.
- `execute_invalid_instruction_10`: Consists of bad instruction data.
- `enter_infinite_loop`: The routine enters an infinite loop.
- `nop_padding_sequence_2`: This routine is empty and consists of a single instruction that serves as padding within the executable segment.
- `enter_infinite_loop_18`: Executes a continuous loop that prevents further instruction processing.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_invalid_instruction_11`: Contains data that cannot be decoded as a valid instruction.
- `execute_invalid_instruction_12`: Contains an instruction that is not recognized.
- `execute_infinite_loop`: The routine enters an unconditional infinite loop that prevents further execution of the calling code.
- `empty_routine_2`: This routine contains no executable instructions.
- `empty_routine_3`: The routine contains no valid instructions and performs no operations.
- `nop_padding_sequence_3`: This routine is empty as the segment contains non-executable data or padding.
- `execute_invalid_instruction_5`: Consists of a single byte that does not form a valid instruction.
- `empty_routine_4`: This routine contains no valid instructions and performs no operations.
- `execute_infinite_loop_2`: This routine enters an infinite loop that prevents further execution of the calling code.
- `execute_invalid_instruction_6`: Contains data that the disassembler cannot interpret as code.
- `perform_infinite_loop`: The routine enters an infinite loop that does not perform any operations.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_infinite_loop_3`: The routine enters an unconditional infinite loop that prevents further execution of the calling sequence.
- `enter_infinite_loop_2`: Enters a loop that never terminates.
- `halt_execution`: The routine contains no valid instructions and terminates execution flow immediately upon entry.
- `execute_invalid_instruction`: Triggers a control flow error due to bad instruction data.
- `enter_infinite_loop_3`: Runs a continuous loop without exit.
- `nop_padding_sequence_4`: This routine is empty and consists of non-executable data that occupies a gap in the instruction stream.
- `enter_infinite_loop_4`: Stays in a perpetual loop.
- `enter_infinite_loop_5`: Executes a loop that repeats indefinitely.
- `enter_infinite_loop_6`: Enters a state of infinite execution.
- `execute_infinite_loop_4`: The routine enters an endless loop that prevents further execution of the calling code.
- `execute_infinite_loop_5`: The routine enters an unconditional loop that prevents further execution of the current thread.
- `enter_infinite_loop_7`: Performs an endless loop of execution.
- `invalid_instruction_trap`: The routine contains invalid data that prevents instruction decoding.
- `idle_in_endless_loop`: The routine is an empty endless loop with no body, so once entered it never returns.
- `stub_bad_instruction_data`: Its single decoded instruction is invalid, so the bytes are data rather than a real routine.
- `idle_in_endless_loop_2`: The routine is an empty endless loop with no body, so once entered it never returns.
- `terminate_execution`: The routine contains no valid instructions and serves as an end-of-flow marker or a trap for invalid control paths.
- `stub_bad_instruction_data_2`: Its single decoded instruction is invalid, so the bytes are data rather than a real routine.
- `execute_infinite_loop_6`: The routine enters an unconditional loop that prevents further execution of the calling sequence.
- `empty_routine_6`: This routine contains no executable instructions and performs no operations.
- `enter_infinite_loop_8`: Stays in a permanent loop.
- `enter_infinite_loop_9`: Executes an infinite loop structure.
- `execute_infinite_loop_7`: The routine enters an infinite loop that performs no operations.
- `execute_invalid_instruction_2`: Contains an instruction that cannot be processed.
- `execute_invalid_instruction_7`: Consists of a single byte that results in a bad instruction error.
- `nop_padding_sequence_7`: This routine is empty and consists of non-executable data used as padding within the executable segment.
- `enter_infinite_loop_10`: Enters a loop that repeats indefinitely.
- `execute_invalid_instruction_8`: Contains a single byte that cannot be decoded as a valid instruction.
- `empty_routine_7`: This routine contains no valid instructions.
- `halt_execution_2`: The routine contains no valid instructions and terminates execution immediately.
- `execute_infinite_loop_8`: The routine enters an unconditional loop that repeats indefinitely without performing any operations.
- `execute_single_instruction`: Contains a single instruction that is executed during runtime.
- `contain_invalid_data`: Consists of a single byte of data that does not form a valid instruction.
- `execute_infinite_loop_9`: The routine enters an unconditional loop that repeats indefinitely.
- `execute_infinite_loop_10`: The routine enters an infinite loop that prevents further execution.
- `enter_infinite_loop_11`: This routine consists of an endless loop.
- `enter_infinite_loop_12`: The routine performs an infinite loop.
- `perform_single_operation`: Executes one instruction when called from an external routine.
- `enter_infinite_loop_13`: This routine enters a loop that never terminates.
- `execute_invalid_instruction_3`: The routine encounters bad instruction data.
- `execute_invalid_instruction_4`: This routine contains data that cannot be decoded as valid instructions.
- `null_routine_4`: The routine is empty and contains no valid executable instructions.
- `enter_infinite_loop_14`: The routine executes an infinite loop.
- `enter_infinite_loop_15`: This routine enters a continuous loop.
- `hold_unused_data`: Contains a single byte of data that is never reached during execution.
- `enter_infinite_loop_16`: Performs an endless loop that never exits.
- `perform_infinite_loop_2`: Enters a continuous loop without performing any operations.
- `run_single_instruction`: Performs a single instruction upon entry from outside the static flow.
- `execute_malformed_data`: Contains an instruction that cannot be processed by the processor.
- `run_unrecognized_opcode`: Attempts to execute an invalid or unrecognized instruction.
- `halt_on_bad_data`: Contains an instruction that causes a control flow error.
- `process_invalid_opcode`: Contains a single byte that is not a valid instruction.
- `nop_padding_sequence_5`: This routine is empty and consists of non-executable padding data located within a gap in the executable segment.
- `enter_infinite_loop_19`: The routine performs an endless loop that prevents further execution.
- `empty_routine_5`: This routine is empty and contains no valid instructions.
- `nop_padding_sequence_6`: This segment contains no executable instructions and serves as a data gap within the runtime segment.
- `process_single_byte`: Executes a single instruction during the program's run.
