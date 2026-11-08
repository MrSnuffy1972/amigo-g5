# Game 213 (TopSecret v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00213). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1943020 top-level bytes in 42 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 35 raw-word lines |
| Functions named and described | pass, 261 of 261 named (0 author symbols), 261 of 261 described (2 by rule, 126 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2988 of 3010 (99.27%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7016 of 7038 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 802545 |
| code | 28240 |
| data | 1112161 |
| padding | 74 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 259 | 25 |
| Intro | 3 | 230 | 7 |
| TopSecret | 1 | 3105 | 112 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 75 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 64 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2902 | 76 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 277 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 7 |

## Cases handled

- TopSecret_v1.0/TopSecret_v1.0.hdf: hard-disk image (AmigaDOS OFS, 2112 blocks): its filesystem parses and all 27 files in it are byte-identical to the extracted tree TopSecret_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- runtime segment an address (play-long-joy): 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 64 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 42 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2349 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 277 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)

## Cracktros (kept, not protection)

- Intro

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `clear_palette_and_start_sequence`: Clears all thirty-two colour registers, calls a handler from its input table, then checks a stored value against a fixed code and calls a second handler; the referenced texts Intro and TopSecret appear in the body.
- `set_global_and_zero_offset`: Sets a word-sized offset in a structure to zero and assigns a constant value to a global variable.
- `conditionally_set_global`: If a global variable is non-zero, it sets a word-sized offset in a structure to one and assigns a constant to another global variable.
- `toggle_and_set_global`: Toggles the lowest bit of a global variable and then sets that variable to a constant value.
- `call_routine_if_flag_set`: Checks if a global variable is non-zero and, if so, calls another routine.
- `halt_in_infinite_loop`: Enters an infinite loop that performs no operations.
- `test_bit_in_table`: Checks if a specific bit is set in a word located at an offset from a pointer.
- `write_copper_list_constants`: Writes four fixed words into the copper location area, including a pair of copper list values, and returns.
- `read_fire_buttons_state`: Tests the two fire-button bits in the input port state, returning zero when both bits are high and a full-byte value otherwise, after calling an unnamed routine.
- `call_table_entry_return_pair`: Calls the handler at a fixed offset of a base table and returns the data register pair unchanged.
- `write_and_wait_low`: Writes a value to a memory offset and waits in a loop until a specific bit in a global variable is cleared.
- `write_and_wait_high`: Writes a different value to a memory offset and waits in a loop until a specific bit in a global variable is cleared.
- `write_hardware_value`: Writes a 16-bit value to a memory location and waits for a specific bit in a global variable to be cleared.
- `write_hardware_value_2`: Writes a different 16-bit value to a memory location and waits for a specific bit in a global variable to be cleared.
- `write_hardware_and_wait`: Writes a value to a memory location and waits in a loop until a specific bit in a global variable is cleared.
- `clear_memory_block`: Increments a global pointer and clears a block of memory using a loop.
- `check_global_flag`: Compares a global variable against another value and returns if they are not equal.
- `execute_callback_with_scramble`: Decrements a counter and, if a global variable is zero, executes a routine through a pointer while sandwiching it between two scrambling operations.
- `call_slot_test_then_follow`: Calls a test handler in a table slot and, when it returns nonzero, calls a second table slot with a saved value and then an unnamed routine; the referenced text TopSecret.highs appears in the body.
- `call_slot_when_flag_clear`: When a flag byte is zero, calls an unnamed routine, then a table entry with the A1 value, then another unnamed routine; the text reference TopSecret.highs appears in the body.
- `xor_scramble_byte_block`: XORs each byte of a block of about 242 bytes with a counter that counts down, scrambling the block in place.
- `configure_cia_interrupt_control`: Writes a sequence of values to the CIA control and interrupt registers, clearing one bit and writing an interrupt enable mask so the interrupt sources are set up.
- `process_interrupt_logic`: Manipulates a global variable and performs bitwise operations on a byte retrieved from memory.
- `acknowledge_interrupt_request`: Reads and writes the same interrupt request cell, leaving it unchanged in the recovered code; the write behaves like an interrupt acknowledge step.

### Intro

- `program_entry_setup_hardware`: Program entry point: records the system base and a custom register value, writes interrupt and DMA disable masks to the hardware, then makes a long sequence of unnamed setup calls.
- `allocate_copy_hunk_sections`: Loads executable sections: walks a table of section entries, allocates memory for each through the system allocator, copies the section data in, and frees temporary blocks.
- `stub_return_immediately`: Contains only a return and does no work; it is one of several empty callback slots.
- `unpack_stream_with_bit_reader`: Unpacks a compressed stream with a bit reader that refills from previous longwords, writing output bytes at the destination with offsets taken from the source header.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream like its neighbour, reading bits through a shift routine that takes a 32-bit value from the four bytes before the source pointer, shifts it right and sets its top bit.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bit_buffer_refill`: Shifts a 32-bit bit buffer right by a count taken from the second data register, reloading it from the previous longword when it empties and setting the top bit on each refill.

### TopSecret

- `open_libraries_and_init_hardware`: Opens the DOS and graphics libraries, allocates memory blocks, and configures interrupt, DMA and copper registers while calling eleven unnamed routines.
- `reset_form_chunk_state`: Makes ten unnamed calls and clears two state fields, with the text FORM referenced; it resets the state used when reading a FORM block.
- `set_form_chunk_flag`: Makes five unnamed calls, sets one state field to one, makes a final call, and references the text FORM in its body.
- `init_form_state_fields`: Makes an unnamed call, clears six state fields, sets one field to fifty, then makes another call.
- `wait_for_raster_line`: Busy-waits until a masked video beam position register reaches a fixed line value, then returns.
- `load_music_module_file`: Opens, reads and closes a DOS file named TopSecret mod dot music, storing the read result in a state field.
- `load_map_data_file`: Opens, reads and closes the DOS file TopSecret Map.Dat through the DOS library, storing the read result in a state field.
- `load_win_lose_data_file`: Opens, reads and closes one of the DOS files TopSecret LoseTro.crk or TopSecret WinTro.crk, then makes one unnamed call.
- `load_graphics_files_pair`: Loads two graphics files through DOS open, read and close sequences, with an unnamed call between them, using the file name TopSecret GrubNuts gfx.
- `load_grubnuts_graphics`: Repeats the open, read and close sequence for the graphics file TopSecret GrubNuts gfx twice in order, with an unnamed call between, in the same pattern as its neighbour.
- `parse_body_chunk_records`: Scans a data block for the four-letter tag BODY, then reads fixed-width entries that follow it into a table, counting each entry as it goes.
- `expand_bit_rows_into_bytes`: Reads a header-sized block of bit data and expands each source byte into output bytes by testing each bit position, writing one fill value for clear bits and another for set bits.
- `fill_row_offset_table`: Writes five pairs of row address words into a table, each row address advanced by a fixed row stride from the previous one.
- `parse_colour_map_chunk`: Scans a data block for the four-letter tag CMAP, then reads colour entries after it, converting each into a palette word and storing it for the colour registers.
- `copy_palette_words_to_buffer`: Copies thirty-two words from the source pointer into a fixed buffer two words apart, staging a full colour palette.
- `copy_indexed_table_words`: Copies twelve words from a table entry chosen by a state index into a fixed buffer, two words apart.
- `copy_indexed_table_quad`: Copies four words from a table entry selected by a second state index into a fixed buffer, two words apart.
- `run_text_screen_loop`: Runs a screen loop that clears state, makes many unnamed calls, and polls the fire button; the referenced text includes WELL DONE, HI SCORE TABLE and WILL.
- `set_flag_and_call`: Sets a state field to one and then calls an unnamed routine.
- `call_once_and_return`: Calls one unnamed routine and returns, with no further work.
- `initialize_score_table`: Executes a routine that resets a score table to its default initial values.
- `reset_score_table_values`: Performs the setup of default values or constants within a score table.
- `apply_score_table_defaults`: Initializes a score table with a specific set of predefined constants.
- `init_score_table_defaults`: Sets initial values for a score table: two words to all-ones, two words to twenty, a fixed limit word, then loops over five entries copying data pointers into the table.
- `wait_for_fire_or_timeout`: Polls the fire-button state in nested counted loops and returns as soon as the button bit reads low, calling an unnamed routine between passes if the count runs out.
- `print_name_field`: Walks a twenty-character field and calls an unnamed routine for each character that is not a space, leaving spaces alone.
- `wait_for_button_with_delay`: Repeats an unnamed call and polls an input bit in nested counted loops, calling a routine between passes and using longer counted delays when the bit state changes.
- `cycle_four_frames_polling_input`: Advances a four-step counter that wraps back to zero, calls two unnamed routines per step, then polls an input bit in nested counted loops before continuing.
- `format_score_digits_and_name`: Converts a 16-bit score into six decimal digits by repeated division by ten, writes the digits into a text buffer, and references the name WILL.
- `run_name_entry_screen`: Clears a twenty-character name buffer to spaces, then runs a loop of unnamed calls and input polling; the referenced texts include YOUR NAME and MOOG.
- `fill_record_arguments`: Writes a 32-bit argument, resets control words, and populates five records with a marker word.
- `draw_score_table_rows`: Makes seventeen unnamed calls to lay out the score table; the referenced text includes HI SCORE TABLE and a list of player names.
- `shift_table_entries_down`: Shifts a table of words down by one position, moving each entry into the slot before it, to advance a rolling table.
- `handle_level_interrupt_request`: Interrupt handler: when a request bit is set it clears the bit and calls unnamed routines, then saves a custom register snapshot, writes interrupt and DMA disable masks, and calls two routines.
- `scan_envelope_level_table`: Scans a table of about 127 levels and selects the first entry above the current level, then applies the result to the envelope state and the audio volume registers.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `advance_channel_step_counter`: Advances a per-channel counter and, when it reaches the stored limit, resets it and clears a state word, makes four unnamed calls, then waits a fixed count before continuing.
- `start_audio_channel_pointers`: Makes four unnamed calls, then while a channel flag is set clears it, advances a sample index with a seven-bit mask, and writes the channel pointer registers.
- `select_envelope_value_by_index`: Chooses one of three values according to a counter taken modulo three, using a word or a nibble of a source byte, then scans a table of about thirty-six entries for the first one at or below it.
- `lookup_nearest_table_entry`: Scans a table of about thirty-six words for the first entry not greater than a stored value, then writes the entry selected by a nibble of the source byte into a channel field.
- `copy_channel_word_field`: Copies a word from the input record into a channel field.
- `lookup_table_entry_by_index`: Scans a table of about thirty-six words for the first entry not greater than a stored value, then stores the entry selected by the index in the data register into a channel field.
- `store_channel_word_value`: Stores the data register word into a channel field.
- `start_audio_dma_channels`: Clears a channel field, makes four unnamed calls, waits through two fixed delay loops, sets the DMA enable bits, and writes the audio location registers from stored pointers.
- `decode_packed_note_record`: Decodes a packed note record: combines a nibble from two bytes into an index, looks up a sample pointer and a period for that index in tables, and writes them into the channel fields.
- `apply_note_effect_fields`: Stores the sample offset and volume in the channel record, then either calls an unnamed routine when the effect command equals a specific value or copies the period and effect fields into the channel.
- `copy_note_fields_to_channel`: Copies a note's period, sample and effect fields into the channel record, masks the period to its low twelve bits, and writes the DMA control word for the channel.
- `restart_audio_dma_channels`: Copies stored pointers and lengths for the four audio channels into the custom chip registers, pausing briefly between steps and setting the DMA enable bit.
- `compare_masked_value_against_target`: Masks a 16-bit field down to its low 12 bits and copies the result to a second field.
- `clear_structure_word_field`: Sets one 16-bit field of a structure to zero and returns.
- `step_value_toward_target`: Moves a 16-bit position toward a stored target by a step size held in a byte field, adding or subtracting according to a direction flag.
- `subtract_step_from_position`: Subtracts a step value passed in a data register from a 16-bit position.
- `advance_table_driven_offset`: Looks up a scale value in a global table using bits of a phase byte, multiplies it by the low nibble of a rate byte, and adds the signed result to a position.
- `subtract_and_advance_phase`: Subtracts one input value from another and stores the difference in a global variable.
- `copy_position_to_output`: Copies a 16-bit position field from the object into a field of the output structure.
- `dispatch_effect_command_nibble`: Decodes the low nibble of a command word and applies the matching effect: a table search and lookup, a fixed-step adjustment, a clamped increment or decrement, a step toward a target, or a table-scaled offset.
- `adjust_level_counter_clamped`: Adds the high nibble or subtracts the low nibble of a parameter byte from a counter field, clamping the counter between zero and 64.
- `decrement_counter_clamped_zero`: Subtracts the low nibble of a parameter byte from a 16-bit counter, clamps the counter at zero, and copies it to an output field.
- `decrement_position_clamp_minimum`: Subtracts a byte parameter from a 16-bit position.
- `increment_position_clamp_maximum`: Adds a byte parameter to a 16-bit position.
- `apply_control_nibble_command`: Reads the low nibble of a command word and performs one of a few actions: it returns for one value, toggles a flag byte for another, loads a counter from the parameter, clamps a level to 64 and copies it out, or resets a sub-counter.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `toggle_flag_byte`: Inverts every bit of one flag byte in the structure.
- `load_counter_and_toggle_flag`: Stores the parameter byte minus one into a counter byte and inverts a flag byte in the same structure.
- `clamp_parameter_to_maximum`: Clamps a parameter byte to a maximum of 64 and copies the result to an output field.
- `load_counter_from_parameter_bits`: Takes the low five bits of a parameter byte and, when they are nonzero, resets one counter byte to zero and stores those bits in a second counter byte.
- `toggle_indexed_word_periodically`: Counts ticks while an enable word is nonzero and, on every fifth tick, flips one of several fixed 16-bit words between two constant values.
- `toggle_fixed_word_value`: Flips one fixed 16-bit memory word between two constant values.
- `toggle_global_word_value`: Checks if a global word equals a specific constant and, if so, updates it to a different constant, otherwise reverts it to the original constant.
- `toggle_global_word_2`: Writes one of two constant values to a global variable depending on its current state.
- `toggle_global_word`: Checks a global variable and toggles its value between two specific constants.
- `write_fixed_word_through_pointer`: Writes one fixed 16-bit constant through the pointer held in an address register.
- `write_fixed_word_through_pointer_2`: Writes a different fixed 16-bit constant through the pointer held in an address register.
- `xor_adjacent_field_bits`: Returns the exclusive-or of bit eight and bit nine of a 16-bit field from a structure, placed at bit nine of the result.
- `apply_directional_movement_step`: Moves an object by adding or subtracting a fixed 32-bit step to several position fields, choosing the direction from input values and state flags.
- `update_grid_cell_state`: Clears two counters and reads one 16-bit cell from a global grid at the object's position.
- `consume_marker_cell_accumulate`: Checks a grid cell at a fixed offset from the object for one special value.
- `set_flag_if_marker_cell`: Clears a flag, then sets it when the grid cell at a fixed offset from the object holds one of two marker values.
- `check_cell_and_entry_hits`: Checks the grid cell under the object's position for one marker value and, if found, clears it or decrements a counter, sets state flags and adds 100 to a 32-bit accumulator.
- `resolve_entry_overlap_events`: Tests five active entries for overlap with the object's position.
- `dispatch_event_code_actions`: When an enabled flag and an input word are both set, dispatches on an event code to one of several actions.
- `nudge_position_on_flag_clear`: Shifts the object's position fields by a fixed step when a trigger flag has just been cleared or two flags are both zero.
- `start_timer_on_button_press`: When no other motion is pending, reads the fire-button bit of the joystick port status and, if it is pressed and a delay counter is clear, starts a 25-step timer and records a mode value.
- `move_position_along_timer_path`: While a timer is running, moves the object's position fields by a step chosen from a table and a direction flag.
- `advance_entry_phase_table`: Advances five active entries through a short cycle, writing each one's value from a table plus an object offset.
- `move_entries_along_grid_path`: Updates active entries that move through the level grid, using a table-driven speed for each.
- `move_entries_from_pointer_list`: Moves entries in a counted list addressed through a pointer register, applying the same grid-path rules as the update routine.
- `advance_alternating_phase_indices`: Alternates between two countdown phases.
- `process_active_entries_in_turn`: For each of five active entries, copies its two position words and a 32-bit value into a scratch record and then calls the next routine to process that record.
- `initialise_five_record_block`: Sets a group of control words in an output structure to constant values, with some derived from the scratch position.
- `fill_animated_grid_records`: Resets several control words, advances a counter that wraps at 50 and another that wraps at four, then walks a 12-by-20 block of level grid cells.
- `fill_grid_block_records`: Resets the same control words as the animated grid filler, then writes five 32-bit records for each cell of a 12-by-20 block without reading the grid values.
- `fill_grid_cell_records`: Resets the same control words as the other grid fillers, then walks a 12-by-20 block of level grid cells and writes five 32-bit records for each cell, using the cell value to offset the record address.
- `fill_scrolling_block_records`: Resets control words, writes two groups of four 32-bit records from a block chosen by a counter, and advances that counter, which wraps at seven.
- `update_animation_frame_records`: Counts down a delay and steps a frame index that wraps at four, copying frame data into pointer records for two groups of entries.
- `toggle_value_fill_five_records`: Toggles between two stored 32-bit values and writes five records from the newly selected value at a fixed stride.
- `initialise_object_state`: Clears most fields of an object structure and sets a group of counters and limits to constant values.
- `reset_object_starting_state`: Resets a subset of the same object fields to their starting constants, covering flags, counters and four 32-bit starting positions.
- `load_entry_records_from_table`: Copies four entry records from a table selected by an index into the entry block.
- `reinitialise_entry_when_timer_expires`: Decrements a countdown and, when it reaches zero, copies a fixed set of table words into a secondary entry record and clears its counters.
- `load_five_entries_and_reset`: Copies five entry records from a table selected by an index and zeroes their counters.
- `call_then_set_control_value`: Calls a routine and then writes the constant 256 into one control field of the object.
- `call_then_set_flagged_control`: Calls a routine and then writes a different constant, with its high bit set, into the same control field.
- `reset_positions_and_call_next`: Writes the same four 32-bit starting positions used by the object reset, then calls the routine that follows it.
- `decode_keyboard_key_code`: Reads the keyboard serial register, converts the byte to a key code and looks up the matching character in a key table.
- `handle_keypress_actions`: Reacts to the decoded key value: the letter p toggles a stored word once per press, the escape key clears one counter, and one other special key code clears a second counter.
- `call_routine_seven_times`: Calls the routine that follows it seven times in a row.
- `divide_value_ten_seven_times`: Copies a 32-bit value into a scratch word, then seven times divides the scratch word by ten and calls the following routine after each division.
- `call_next_routine_once`: Calls the routine that follows it once and returns, with no other work.
- `divide_counter_ten_twice`: Copies a 16-bit counter into a scratch word, then divides it by ten twice, calling the following routine after each division.
- `divide_incremented_counter_twice`: Adds one to a 16-bit field, copies the result into a scratch word and divides it by ten twice, calling the following routine after each division.
- `fill_five_records_from_args`: Stores a 32-bit argument, resets a group of control words to constants, and writes five 32-bit records with a fixed marker word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `unpack_bitstream_to_memory`: Reads a bitstream from a source and performs bitwise shifts and masking to reconstruct bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `allocate_and_initialize_memory_blocks`: Allocates several contiguous memory blocks by repeatedly calling a memory allocation routine and storing the resulting pointers and size ranges into a structured table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_joystick_button_state`: Reads a value from a hardware register and toggles a specific bit within a memory location using a bitwise XOR operation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_bitstream`: Reads a bitstream from a source and uses the bits to perform conditional operations, bit shifts, and memory copies into a destination buffer.
- `shift_pointer_value_2`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `xor_and_decrement_pointer`: Xors a global variable with a value and decrements the byte pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_structure_offsets`: Clears two word-sized values at specific offsets within a structure.
- `copy_memory_block_2`: Performs a loop that copies data from one memory location to another using a decrementing pointer and a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block_reverse`: Copies a block of memory by decrementing pointers and transferring data from a source to a destination.
- `copy_memory_block_3`: Copies a block of memory from one location to another using a loop and a decrementing pointer.
- `shift_and_set_bit_2`: Returns a value derived by shifting a word from a pointer right by one and setting the highest bit.
- `search_and_reset_fields`: Iterates through a memory block searching for a zero value via bit shifting, then clears two specific offsets in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_structure_fields`: Clears two specific word-sized offsets within a structure and calls two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of data from one memory location to another using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_table_entry`: Calls another routine and then stores a zero in a word at an offset from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning to its caller.
- `shift_and_set_bit`: Returns a 32-bit value by shifting a word from a pointer offset by negative four and setting the highest bit.
- `loop_and_shift_data`: Iterates a specified number of times, shifting a value and reloading it from a pointer if it reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_word_value`: An extremely short routine that performs a word-sized write to a structure offset.
- `clear_sprite_attributes`: Waits for a bitmask to match in a 32-bit value and then clears several fields in a structure using zero values.
- `wait_for_bitmask`: Enters a loop that waits until a specific bitmask is matched within a 32-bit value at a pointer offset.
- `unknown_routine`: A routine consisting of six instructions with no available pseudo-code.
- `update_joystick_state`: reads a value from a hardware register and stores it into a global variable.
- `expand_bitmask_data_2`: Calls a routine that uses a bitmask to selectively copy bytes from a source to a destination.
- `expand_bitmask_data_3`: A routine consisting of 31 instructions that calls a routine to selectively copy bytes using a bitmask.
- `expand_bitmask_data_4`: A routine consisting of 90 instructions that calls a routine to selectively copy bytes using a bitmask.
- `expand_bitmask_data`: Uses a bitmask to selectively copy bytes from one source to another destination based on bit states.
- `fill_table_with_increments`: Writes a series of 16-bit values into a table, where each value is an incremented version of an initial integer.
- `search_and_process_string`: Searches for a specific four-character string and then processes subsequent bytes into a table using arithmetic operations.
- `copy_word_block`: Copies thirty-two words from a source pointer to a destination table.
- `copy_indexed_words`: Copies twelve words from a calculated offset in a table to a destination table.
- `copy_word_elements`: Copies four words from a calculated memory location to a destination table.
- `set_flag_value`: Writes a constant word value to a specific offset within a structure.
- `initialize_loop_data`: A routine consisting of 3 instructions that initializes structure fields and executes a loop five times.
- `initialize_loop_data_2`: A routine consisting of 4 instructions that initializes structure fields and executes a loop five times.
- `invoke_recursive_call`: This routine performs a single recursive call to itself before returning.
- `invoke_recursive_call_2`: The routine performs a single recursive call to itself before returning.
- `initialize_and_loop_data`: Initializes several fields in a structure with constant values and then executes a loop five times that updates two structure members using values returned from another routine.
- `shift_table_elements_2`: Shifts word-sized elements within a table by moving each value to the next position and placing the original first value into the last position.
- `iterate_string_buffer`: Iterates through a memory buffer for a fixed number of steps, calling another routine whenever a non-space character is encountered.
- `wait_for_input_bit`: Polls a bit from a hardware register and performs nested delay loops based on whether the bit is set or cleared.
- `update_sprite_and_wait`: Updates a sprite attribute counter with a wrap-around check and performs hardware register polling with delay loops.
- `compare_and_update_table`: Compares values from a table against a numeric value derived from division and modulo operations, then performs memory copies and updates a structure with decimal digits.
- `process_input_and_update_table`: Reads the joystick and mouse fire button status from the CIAA Port A and executes a sequence of operations including a recursive call, string buffer iteration, and table element shifting.
- `call_populate_routine`: Calls another routine and then returns.
- `repeat_string_iteration`: Iterates through a memory buffer and executes another routine multiple times.
- `shift_table_elements`: Performs a series of word-sized shifts within a table by moving each value to the next position and placing the original first value into the last position.
- `toggle_memory_value_2`: Increments a counter in a structure and, upon reaching a specific value, selects a memory address from a table to toggle its contents.
- `process_audio_data`: Iterates through a data stream to find a specific byte value, then clears a series of memory locations and resets several global variables.
- `reset_globals_to_constant`: Writes zero to five global variables and writes a constant value to a fifth global variable.
- `update_state_and_audio`: Increments a counter in a structure, modifies several global pointers, and updates hardware audio channel registers.
- `process_joystick_input`: Parses a packed input byte to extract a command index and sub-command, then updates a coordinate state or status flags based on the resulting values.
- `update_status_flag`: Extracts a value from a structure, stores it in a new location, and sets a status byte based on a comparison with another value in the structure.
- `update_coordinate_state`: Updates a coordinate or position value within a structure based on a control mode.
- `toggle_memory_value`: Increments a counter in a structure and, upon reaching a specific value, selects a memory address from a table to toggle its contents between two specific constants.
- `calculate_bit_manipulation`: Performs bitwise operations on a word retrieved from a structure and returns the result.
- `update_entity_coordinates`: Updates a set of four 32-bit coordinate values by adding or subtracting large increments based on specific state flags and direction indicators.
- `update_joystick_state_2`: Calculates a pointer to a specific entry in a data table based on coordinate offsets and updates state flags.
- `update_score_on_match`: Calculates an index into a table based on values in a structure and, if a specific value is found at that index, decrements one variable and increments another by one hundred.
- `check_status_flag`: Sets a byte in a structure to one if a value retrieved from a calculated table index matches one of two specific constants.
- `process_collision_events`: Checks a table of objects for spatial overlaps with a coordinate-derived area, updating various state variables and counters if a match is found or if a specific table value is met.
- `handle_object_interactions`: Iterates through a list of objects to check for positional overlaps and modifies several structure members and counters based on the state of the matched object.
- `process_input_command_event`: Checks a status flag and a joystick bitmask before processing a command ID stored in a structure.
- `apply_coordinate_offset_increments`: Checks specific conditions within a data structure based on short integer comparisons and, if met, adds a constant value to several integer fields.
- `update_joystick_state_3`: Checks specific memory offsets and the CIAA_PRA register to update a global variable and modify two memory locations.
- `modify_coordinate_table`: Updates two long integer values in a table based on a value retrieved from a lookup table and a flag.
- `process_object_list`: Iterates through a list of objects to update their properties and memory pointers based on their type and state.
- `update_entity_behavior_state`: Updates an entity's internal state flags and coordinate values by performing lookups in a tile or object table.
- `update_entity_state_and_position`: Iterates through a list of data structures to update internal state flags and coordinate values based on lookups in a table.
- `decrement_timer_counters`: Decrements two different counters and resets them or increments a third value when they reach zero.
- `iterate_object_list`: Iterates five times through a table to copy coordinate and attribute data into a structure before calling another routine.
- `draw_moving_objects`: Calculates a starting position and iterates five times to call a routine that draws objects at specific coordinates.
- `render_sprite_table`: Iterates through a table of values to determine sprite positions and calls a drawing routine multiple times for each entry.
- `fill_screen_area`: Uses nested loops to call a drawing routine repeatedly, filling a large area based on a base memory address.
- `draw_fixed_pattern`: Iterates through a hardcoded table of values to calculate positions and calls a routine to draw objects in a grid-like pattern.
- `draw_sequence_of_shapes`: Performs two separate loops to call a routine with different coordinate sets and then increments a counter in a table.
- `update_sprite_attribute_tables`: Copies data from various source tables into a destination buffer using pointer arithmetic and conditional checks.
- `update_table_values`: Compares two values in a structure and selects one to be used in a loop that populates a table with incrementing values.
- `initialize_structure_data`: Clears and initializes a large number of fields within a structure, including several zeroed word and long values.
- `set_specific_constants`: Sets several fields in a structure to zero or specific constant values, including several long values.
- `copy_table_data`: Copies data from a source table to a destination table in a loop, performing specific field reordering and zeroing.
- `decrement_and_load_pointers`: Decrements a value in a structure and, if it reaches zero, loads several pointers from a table into the structure.
- `copy_and_reset_data`: Copies data from a source table to a destination table and then resets various fields in a structure to specific constants.
- `set_word_value`: Writes a specific word value to an offset within a structure.
- `set_word_value_alt`: Writes a different word value to an offset within a structure.
- `initialize_data_buffer`: Writes several longword values into a destination buffer at specific offsets.
- `read_keyboard_input`: Reads a byte from the keyboard serial register and compares it against a value in a table to potentially reset two bytes in a structure.
- `handle_input_commands`: Checks a character in a structure and performs different actions, such as bitwise inversion or calling another routine, based on its value.
- `execute_repeatedly`: Executes another routine seven times in a loop.
- `divide_and_repeat`: Copies a longword from one location to another and then repeatedly divides that value by ten seven times while calling another routine.
- `populate_structure_via_iteration`: Iterates five times to fill a data structure with values while invoking another routine during each loop cycle.
- `divide_short_value`: Copies a word value into a longword and then repeatedly divides that value by ten twice while calling another routine.
- `increment_and_divide`: Increments a word value, stores it in a longword, and then repeatedly divides that value by ten twice while calling another routine.
- `populate_list_via_routine`: Iterates five times to populate a structure with values and calls another routine during each iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_disable_restore_chip_enables`: Saves the current DMA and interrupt enable state, then clears the custom chip enable bits with several register writes and calls a series of setup routines.
- `decode_packed_rle_bitmap`: Searches the input for the tag text BODY, then decodes run-length packed data into rows of 40 bytes.
- `repeat_wait_then_call_forever`: Calls a fixed external routine 1000 times, then calls the following routine and starts the count again, so it never returns.
- `write_five_fixed_pointer_records`: Writes five pointer-pair records into a fixed block, starting from a fixed address and advancing by a fixed stride each time.
- `convert_cmap_to_color_words`: Searches the input for the tag text CMAP, then converts 32 three-byte color entries into 16-bit color words.
- `copy_thirty_two_words_strided`: Copies 32 16-bit words from the input into a table, writing each one with a four-byte stride.
- `rotate_word_ring_by_one`: Rotates a ring of 16-bit words by one position: each word moves into the next slot and the first word is carried around to the last slot.
- `forward_call_and_return`: Calls the routine that follows it and then returns.
- `reset_and_invoke_external`: Resets a set of control words to constant values and then invokes an external routine five times using two provided addresses.
- `reset_and_iterate_external_routine`: Resets a set of control words to fixed constants and then executes a specific external routine five times using two provided addresses.
- `reset_and_invoke_external_2`: Resets a set of control words to constant values and then calls an external routine five times using two provided addresses.
- `call_external_five_times`: Resets a set of control words to constants, then five times calls a fixed external routine with two addresses.
- `reset_and_call_external_five`: Resets a similar set of control words and then makes the same five calls to a different fixed external routine, advancing the returned addresses by 10240 and 7680 each time.
- `write_masked_field_value`: Writes a constant 16-bit value to an offset relative to a pointer in an address register.
- `write_different_masked_field`: Writes a different constant 16-bit value to an offset relative to a pointer in an address register.
- `wait_for_masked_field_value`: Busy-waits until selected bits of a 32-bit word at a fixed offset from the address register equal a set value, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_nothing`: This routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `perform_no_operation`: The routine is empty and performs no actions.
- `nop_placeholder`: This routine is empty and performs no operations.
- `return_immediately`: This routine is empty and returns to its caller immediately without performing any operations.
- `return_immediately_2`: This routine is empty and returns to its caller.
- `return_immediately_stub`: The routine consists of two instructions and does nothing but return to its caller, so it has no effect on registers, memory, or hardware.
