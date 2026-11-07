# Game 13 (Arkanoid v1.7 NTSC 1136): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00013). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (790900 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 128 of 128 named (0 author symbols), 128 of 128 described (2 by rule, 12 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3491 of 3491 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5520 of 5520 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 147510 |
| code | 18394 |
| data | 624993 |
| padding | 3 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 214 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5301 | 111 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |

## Cases handled

- Arkanoid_v1.7_NTSC_1136/Arkanoid_v1.7_NTSC_1136.hdf: hard-disk image (AmigaDOS OFS, 832 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree Arkanoid_v1.7_NTSC_1136_hdf, which is recovered file by file; no block outside the filesystem holds data
- Arkanoid_v1.7_NTSC_1136/Arkanoid_v1.7_NTSC_1136_hdf/ArkanoidNTSC/Arkanoid.2: high-entropy data file (8.00 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 3343 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `setup_display_copy_and_jump`: Sets up display, bitplane and blitter registers, clears the sixteen colour registers and copies a block of data into low memory.
- `xor_scramble_block_if_ok`: Calls a loader-table entry and, when that call succeeds, makes two further calls and XORs 230 bytes starting at a shared variable with a decreasing counter.
- `call_loader_entry_between_steps`: Calls one routine, then a loader-table entry at a fixed offset, then another routine.
- `xor_scramble_global_block`: XORs 230 bytes starting at a shared variable with a decreasing counter and returns 230.
- `xor_fixed_block_if_ok`: Calls a loader-table entry and, when that call succeeds, makes two further calls and XORs 230 bytes at a fixed address with a decreasing counter, returning 230.
- `invoke_via_global_pointer`: It invokes another routine, then writes a fixed value through a pointer derived from a global variable, then invokes that same routine again before returning.
- `xor_scramble_fixed_block`: XORs 230 bytes starting at a fixed address with a decreasing counter and returns 230.
- `repeat_call_count_times`: Calls one routine as many times as the count in the input word, counting down to zero.
- `repeat_call_while_counter_within`: Calls a routine repeatedly, counting up from one until the counter passes the value that routine returns.
- `fade_palette_by_ratio`: Scales each 4-bit colour component of a source table by a ratio, dividing by a value from another routine, and writes the results to the colour registers in turn.
- `wait_for_flag_edge`: Waits for the lowest bit of a shared variable to become set and then clear again, so it waits for one complete change of that flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `step_object_toward_target`: Moves an object one step toward a target coordinate, consulting a grid of lookup tables to test for blocked cells and set movement flags.
- `check_record_hits_in_window`: Walks three records and checks the input position against each record's horizontal and vertical window.
- `append_record_to_bounded_buffer`: Appends a record to a bounded buffer that wraps back to a fixed start address at its end, choosing one of three record layouts from a flag value.
- `append_record_with_flag_check`: Writes one record into the same bounded buffer, with a layout that depends on a flag value.
- `write_record_with_routine_calls`: Writes a record into a fixed buffer layout chosen by a flag, makes three calls to other routines, and makes a fourth when a count is small and a flag bit is clear.
- `write_plain_record_then_calls`: Writes a two-word record at an offset from a base address, advances a buffer pointer, and makes three calls to other routines.
- `copy_tile_rows_to_planes`: Blanks a set of rows in four bitplane areas, then copies eight words from a source table into each of four destination rows.
- `clear_tile_column_in_planes`: Clears a column of eight row groups across the bitplane areas, with the start address computed from a shared value and a register and a 26-byte row stride.
- `compute_table_derived_fields`: Negates and masks a 16-bit value, looks up a scaled entry and two further table fields from it, and stores the results in fields of a structure.
- `set_limit_flags_if_exceeded`: Compares two input values against limits and, when one is exceeded, sets a status word and flag bits in a structure.
- `set_nonpositive_limit_flags`: Sets a status flag and flag bits in a structure when the first input is zero or less, and does the same with a second flag when the second input is zero or less.
- `set_upper_limit_flag`: Sets a status flag and a flag word in a structure when a value reaches a limit.
- `set_lower_limit_flag`: Sets a status flag and a flag word in a structure when a value is zero or less.
- `copy_block_groups_to_planes`: Copies blocks of words from a source record into four bitplane areas spaced 8192 bytes apart, in one of two passes chosen by a flag, and calls 24 other routines.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `advance_stepped_sequence`: Advances a stepped sequence record: once its delay counter expires, it reads the next value from a list, updates direction and step fields, and calls a function pointer for each step until the count runs out.
- `run_stepped_callbacks_then_store`: Walks a count of steps, calling a function pointer on each one and adjusting a value by a fixed step, with a wrap at 2048 units depending on a flag.
- `initialize_record_from_tables`: Sets up a record from two tables indexed by a register, calls a list of routine routines, and waits on the raster position before finishing.
- `build_cell_entry_list`: Clears a fixed buffer and then scans a grid of table words, writing a three-word entry for each cell whose flag bit is set, with a type code chosen by the flag pattern.
- `scan_cells_and_emit_entries`: Scans a grid of table words and, for each flagged cell, marks it and calls other routines to emit entries.
- `decode_run_length_rows`: Fills a table of row pointers, then decodes a run-length list into a buffer, where a negative count repeats the following value and a non-negative count copies literal words.
- `copy_flagged_tile_rows`: When a bit in a table word is set, copies a group of eight rows from a source table into a destination area with a 42-byte row stride.
- `copy_flagged_tile_rows_extended`: Like its neighbour, copies a group of eight rows from a source table when a table bit is set, and writes one extra column for each row.
- `call_routine_per_record`: Walks a list of six-word records while the first two fields are nonzero, calling a routine for each record.
- `chain_two_subroutine_calls`: Calls two unnamed routines one after another and returns, with no other work.
- `invoke_blit_twice_return_inputs`: It invokes two other routines in sequence, each of which configures the blitter for a single rectangle.
- `call_if_field_range_one`: Calls a routine only when a shared field lies between one and five inclusive.
- `call_if_field_range_two`: Calls a routine only when a shared field lies between two and six inclusive.
- `call_pairs_by_clamped_count`: Clamps a shared count to six, then calls two routines for each step of a loop that runs the count minus two times, when the count is above one.
- `keep_high_byte_of_word`: Writes the low byte and then the high byte of a shared word into the same location, so the high byte is what remains.
- `advance_timed_state`: Advances a timed state variable by a step returned from another routine.
- `select_highest_active_record`: Scans eight records and chooses the one with the largest second field below 200 among those whose bit is set in a mask.
- `compute_absolute_motion_deltas`: Reads a shared input state and computes the absolute difference between the previous and current values on two axes, returning both as a pair.
- `allocate_free_record_slot`: Looks for a free slot among two records using a bit mask, sets that slot's bit, fills in its two fields from shared values, and returns a pointer to it.
- `claim_slot_and_initialise_pair`: When the input index is below two and its bit is set in a mask, it marks the slot as taken, decrements a shared count and initialises two records for that slot.
- `update_position_with_threshold`: When a record's flag is clear, it calls another routine and then sets a second record's position field from a shared value.
- `start_entry_if_idle`: When the input index is below eight and two shared flags are clear, it marks a state byte, stores four input values and calls other routines, returning zero.
- `mark_entry_if_flagged`: When a shared flag byte is nonzero, it sets another state byte to all ones and returns zero.
- `set_position_from_offset`: Clears a field, calls another routine and, if that field is still zero, sets a position value to a stored offset plus the input.
- `dispatch_state_handler`: Depending on a shared state value other than minus one, calls up to three routines and then dispatches through a table of handlers.
- `set_flag_bytes_to_ones`: Sets two byte flags to all ones and writes one to a shared variable.
- `chain_four_routine_calls`: Calls four unnamed routines one after another and returns, with no other work.
- `reset_record_flags`: Clears a shared variable and then, for seven records, sets a status field to one wherever it is currently nonzero.
- `clear_record_if_active`: When a shared pointer is nonzero, writes a fixed value into two fields of the record it points to, then clears the pointer.
- `reset_state_structures`: It stores a fixed word through a pointer field of a global variable and clears a second field of that same structure, repeating the writes for a second instance before clearing the global pointer.
- `set_flag_and_clear_global`: Writes all ones into a byte field of a record and clears a shared variable.
- `initialise_slot_record_with_ratios`: When an index below eight is unused, it marks the slot as taken, stores position and speed fields, computes two quotient and remainder pairs by dividing by a shared count, and calls another routine.
- `release_slot_and_count_down`: When an index below eight is in use, it marks a flag bit and decrements a shared count.
- `set_first_pending_slot_ready`: Walks a table of eight fixed-size records and, in the first record whose status word is nonzero, sets that word to one and returns; if none qualifies it returns without change.
- `reset_eight_slot_records`: Calls a helper routine in two loops of eight passes each, then initialises eight consecutive records: it clears the leading and several flag fields, copies the shared global value into four fields, sets a count of eighteen and writes a per-record index byte.
- `clear_and_seed_status_words`: Zeroes three fixed memory words, sets four other fixed words to the shared global value, and clears one further word.
- `seed_three_word_groups`: Writes the value one hundred twenty-five to the first word of each of three four-word groups spaced sixty-eight bytes apart, and zeroes the other three words in each group.
- `forward_to_inner_routine`: Does no work of its own: it calls one other routine and returns.
- `store_constants_between_helper_calls`: Alternates three calls to a helper routine with three stores of fixed long constants into memory words, the first and last constants being identical.
- `store_five_helper_results`: Five times over, calls a helper once without using its result, then calls it again to get a 64-bit value and stores its upper and lower 32-bit halves into the shared global in turn.
- `set_default_pair_words`: Calls a helper routine five times around stores of the fixed values one hundred twenty-five and two hundred one into two adjacent words, and zeroes a third word between the calls.
- `build_four_record_table`: Fills four fifty-byte records in turn: copies a long from a source table, stores a value derived from the shared global pointer, calls a helper, clears most remaining fields, and sets a value of eighty, a sequence index, and a fixed marker word.
- `invoke_helper_five_times`: Calls one unnamed routine five times in succession and returns, with no other work in its body.
- `copy_indexed_table_block`: Uses the index in the low word of the data register to select a sixteen-byte table entry, saves two words from it into the shared global, calls a helper, stores the same fixed long constant into two words, and copies thirty-two words from a pointer held in the entry into a fixed buffer.
- `init_two_pointer_table_headers`: Initialises two structures in turn: each writes a marker word, fills eight consecutive pointer slots with addresses forty bytes apart, then sets fixed fields including values drawn from the shared global pointer and two constants.
- `init_bitplane_descriptor`: Clears a forty-byte descriptor and fills in the row byte width, height and depth; the width is the pixel width rounded up to whole sixteen-pixel units.
- `checksum_descriptor_planes`: Returns two byte checksums: one sums the forty descriptor bytes, the other sums every byte of each plane's data, sized by row width times height, across all planes.
- `advance_audio_sound_sequences`: Each tick it updates a playback counter for each of four audio channels and writes the result to that channel's volume register.
- `disable_audio_dma_channels`: Sets the shared global to a fixed sentinel value and writes the low four bits to the DMA control register with the set bit clear, which disables DMA for the four audio channels.
- `dispatch_table_store_by_flags`: Dispatches on flag bits in its input: if the top flag is set it calls another routine; if the next flag is clear it stores the shared global plus the input word times eighteen into an indexed table; otherwise it stores the input word into a table indexed by the shared global.
- `claim_free_slot_and_init`: Scans up to four slots for one whose bit in a status byte is clear.
- `init_slot_pair_records`: Given a slot index and two base pointers, marks the slot's bit in both status bytes, initialises two matching record blocks with a fixed pointer, zero fields and positions offset from the stored coordinates, and increments the shared counter.
- `stub_return_stack_value`: Does no work; it returns the value held in its stacked argument unchanged, so it behaves as a pass-through stub.
- `spawn_slot_from_free_bit`: Looks for a free one of three slots by checking its bits in two status bytes.
- `retire_active_slot_entry`: For a slot index below three whose active bit is set, it marks the matching bit in a second status byte, decrements the shared counter, and picks one of two fixed values depending on whether a record field equals two hundred fifty-six, calling a helper in the other case.
- `or_blit_four_columns`: Runs four passes of eight-row writes: each byte written into the buffer is the bitwise OR of one byte from each of two pattern tables, with the table for each row chosen by a rotating mask bit, and rows are spaced by a row stride.
- `blit_masked_pattern_columns`: It runs four passes of eight-row writes, clearing mask bits in each destination byte and ORing a pattern byte into it.
- `mask_and_or_blit_columns`: Runs four passes of eight-row writes: each destination byte has the bits of a mask cleared and is then ORed with a pattern byte, with the pattern table for each row chosen by a rotating mask bit.
- `delegate_to_nested_helper`: Does no work of its own beyond calling one other routine and returning.
- `pass_through_helper_call`: Calls one other routine and returns, with no other work in its body.
- `emit_nibbles_via_callback`: Walks the eight four-bit digits of a 32-bit value from most significant to least, skipping leading zeros but always handling the final digit, and calls a supplied routine through a pointer for each digit it emits.
- `emit_string_characters`: Calls one routine for each byte of a zero-terminated string until the terminator; the branch on the mode word runs the same loop either way.
- `process_terminated_entries`: Walks a list of entries that ends with a two-byte all-ones marker, calling one routine for each entry and then skipping that entry's zero-terminated string.
- `copy_offset_columns_to_planes`: Copies an eight-row byte column for each of four planes from a source buffer to a destination.
- `or_composite_into_two_targets`: If a selected four-bit field of a stored value is nonzero, or the index is at least two, it ORs eight row bytes from two selected pattern tables and writes the result into two destination buffers at offsets taken from two pointer tables.
- `extract_nibble_from_field`: Returns a stored long value shifted right by four times the low nibble of the index argument, extracting one four-bit field from it.
- `run_conditional_call_sequence`: Makes a fixed sequence of calls to helper routines, repeating some of them only while the shared counter is above one.
- `call_helper_for_six_indices`: Calls one routine six times in turn, passing it each index from zero to five together with the two arguments it received.
- `update_record_peak_and_flags`: When a record's enable word and the shared flag are both nonzero, it stores a helper's result as the record's value, sets a first-crossing flag and bumps a counter at each of two fixed thresholds, then updates the per-record and overall maxima of that value.
- `pass_through_long_result`: Calls one other routine and returns with the data-register values unchanged, so it forwards that routine's result to its caller.
- `add_packed_decimal_bytes`: Adds two four-byte values as independent packed-decimal byte pairs, applying decimal adjustment to each byte sum.
- `subtract_packed_decimal_bytes`: Subtracts one four-byte packed-decimal value from another byte by byte, decimal-adjusting each result and propagating the borrow from lower bytes.
- `call_helper_three_times`: Calls one unnamed routine three times in succession and returns, with no other work in its body.
- `setup_four_helper_pairs`: Clears one word, calls a helper eight times, then four times calls it with two addresses that advance by four bytes per step, starting from a fixed table location and the shared global.
- `emit_calls_by_tag_marker`: Makes a fixed run of helper calls and branches on whether a data register holds the four-byte marker spelling ALL followed by a zero byte: the matching branch makes one extra call, the other two.
- `store_word_then_call_pairs`: Makes a helper call pair, writes the value one hundred ninety-two to a word at a fixed offset from the base register, then repeats eight pairs of helper calls.
- `call_pairs_advancing_word`: Three times over, it makes two helper calls and then adds eight to a word at a fixed offset from the base register.
- `iterate_clipped_tiles`: Walks a rectangle in steps of a tile size, clipping the last step at each edge, and calls one routine for each tile position.
- `service_audio_channel_interrupts`: Reads the audio interrupt and DMA status, and for each audio channel with a pending request either decrements its countdown or, once the countdown has run out, switches that channel's DMA off and marks it finished.
- `start_audio_channel_sample`: Starts one audio channel: it marks the channel active, pauses briefly, loads the sample address, length and volume from a four-word block into the channel's registers, records bookkeeping values, and then enables that channel's DMA together with the DMA master enable.
- `pack_sprite_position_words`: Packs an Amiga sprite's position into its two control words: horizontal start and vertical start and stop as bit fields, plus the high-bit flags, stored byte-swapped in the target block.
- `setup_sprite_pointer_and_control`: Packs sprite position control words as its neighbour does, then stores the sprite data pointer as separate high and low words into one of two per-buffer tables chosen by the shared global, and writes the control words into a second table, offsetting the pointer when the input flag is set.
- `store_pointer_into_table_pair`: Writes the high and low halves of a 32-bit pointer argument into the same index slot of two parallel tables.
- `decode_packed_parameter_word`: Decodes a packed word into several parameters: it stores the word, scales a value by a table entry selected by the word's low byte, reads a long, a byte and a word from three lookup tables chosen by bit fields, and derives a final field from the word.
- `store_flag_or_input_word`: Writes a word into two record fields.
- `blit_masked_rectangle_copy`: Sets up the blitter for a masked rectangle copy between two bitmaps: it computes shift and first and last word masks from the horizontal coordinates, writes the modulo and pointer registers for source and destination, runs one blit per row waiting for each to finish, and writes the interrupt and DMA control registers at the end.
- `compute_bitplane_byte_offset`: Returns the byte offset of a position within a bitplane: the row number times the row stride, plus the horizontal position rounded down to a multiple of sixteen and divided by eight.
- `blit_masked_rectangle`: It sets up the blitter for a masked rectangle copy between two bitmaps, computing the shift and first and last word masks from the horizontal coordinates.
- `blit_rows_with_shifted_source`: Sets up the blitter for a copy of a rectangle with a bit shift: it derives the shift and first-word mask from the horizontal coordinates, writes the modulo and pointer registers for the source and destination planes, and starts one blit per row, waiting for each to finish.
- `blit_shifted_source_rectangle`: It sets up the blitter for a shifted-source rectangle copy, deriving the shift and first-word mask from the horizontal coordinates and the row modulo from the width.
- `configure_single_blit`: It configures the blitter for a single rectangle, computing the first and last word masks and the row modulo from the horizontal start and width, then starting the transfer.
- `invoke_blit_config`: It calls another routine to configure the blitter for a rectangle and then returns without doing anything else with the data.
- `configure_and_start_blit`: Configures the blitter for one rectangle: it computes first and last word masks and the row modulo from the horizontal start and width, sets the copy control bits from a flag, starts a blit of the combined size, and then calls one routine twice.
- `build_clear_mode_register_word`: Returns the bitwise complement of its word argument with the top bit forced on before inversion, so the result has its top bit clear.
- `blit_planes_by_mask_bit`: Sets up the blitter for a rectangle and blits each bitplane in turn.
- `invert_word_for_clear_write`: Returns the bitwise complement of its word argument with the top bit forced on before inversion, so the result has its top bit clear, which makes a control-register write clear the chosen bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_no_work`: Consists only of a return instruction, so it does nothing when called.
- `empty_return_routine`: It is an empty routine that returns immediately without touching any data or hardware.
- `do_nothing`: This routine contains no instructions and does nothing.
- `do_nothing_2`: The routine contains no instructions; it does nothing and simply returns.
- `mark_empty_routine`: This routine contains no instructions and does nothing.
