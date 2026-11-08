# Game 910 (top secret (1992)(c. simon)(pd)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00910). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 91 of 91 named (0 author symbols), 91 of 91 described (1 by rule, 88 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 672 of 672 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3434 of 3434 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 782611 |
| code | 13662 |
| data | 89698 |
| padding | 5933 |
| unused | 9216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Top Secret (1992)(C. Simon)(PD).adf:Intro | 3 | 230 | 7 |
| Top Secret (1992)(C. Simon)(PD).adf:godtr | 1 | 89 | 3 |
| Top Secret (1992)(C. Simon)(PD).adf:TopSecret | 1 | 3105 | 80 |
| Top Secret (1992)(C. Simon)(PD).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from

## Cracktros (kept, not protection)

- Top Secret (1992)(C. Simon)(PD).adf:Intro

## Copy protection found

- none found

## Functions


### Top Secret (1992)(C. Simon)(PD).adf:Intro

- `call_relocation_routine`: Calls a routine to apply relocations and then jumps to a zero address.
- `apply_amiga_hunk_relocations`: Iterates through a list of pointers to process data structures, performing pointer arithmetic and updating values in a destination.
- `process_pointer_list`: Iterates through a list of pointers, using them to index into a table and performing pointer arithmetic to update values in a destination structure.
- `set_data_registers`: Sets the low bytes of d0 and d1 to specific values and then clears d0 three times.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream by reading bits through a shift routine that takes a 32-bit value from the four bytes before the source pointer, shifts it right and sets its top bit, and writes the resulting bytes to a destination buffer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bit_buffer_refill`: Shifts a 32-bit bit buffer right by a count taken from the second data register, reloading it from the previous longword when it empties and setting the top bit on each refill.

### Top Secret (1992)(C. Simon)(PD).adf:godtr

- `process_bit_stream_data`: Loads a 32-bit word from a pointer in a0, performs bitwise operations and conditional branches, and writes a byte to a destination at a2 using a loop.
- `shift_and_store_byte`: Loads a 32-bit word from a pointer in a0, stores the lower byte of that word into a destination at a5, and returns with the 32-bit value rotated right by one bit.
- `process_bit_stream`: Shifts a 32-bit value right and uses a loop to write bytes to a destination buffer based on the bit count and value.

### Top Secret (1992)(C. Simon)(PD).adf:TopSecret

- `open_libraries_and_init_hardware`: Opens the DOS and graphics libraries, allocates memory blocks, and configures interrupt, DMA and copper registers while calling eleven unnamed routines.
- `initialize_game_state`: Calls several routines, loads a pointer into a1 from an offset of a4, and writes zero to offsets 106 and 124 of the structure in a4.
- `execute_sequence_of_calls`: Calls six different routines, loads a constant address into a0, moves a pointer from a4 to a1, and finally stores the value 1 at offset 124 of the structure in a4.
- `init_form_state_fields`: Makes an unnamed call, clears six state fields, sets one field to fifty, then makes another call.
- `wait_for_raster_line`: Busy-waits until a masked video beam position register reaches a fixed line value, then returns.
- `load_music_module_file`: Opens, reads and closes a DOS file named TopSecret mod dot music, storing the read result in a state field.
- `load_map_data_file`: Opens, reads and closes the DOS file TopSecret Map.Dat through the DOS library, storing the read result in a state field.
- `load_win_lose_data_file`: Opens, reads and closes one of the DOS files TopSecret LoseTro.crk or TopSecret WinTro.crk, then makes one unnamed call.
- `load_graphics_files_pair`: Loads two graphics files through DOS open, read and close sequences, with an unnamed call between them, using the file name TopSecret GrubNuts gfx.
- `load_graphics_files_pair_alt`: Loads two graphics files through DOS open, read and close sequences, with an unnamed call between them, using a different starting address for the first file.
- `decompress_body_data`: Searches for a specific four-byte tag in a buffer and then performs a decompression routine that expands bytes into a destination table based on bit-length and sign.
- `expand_bit_rows_into_bytes`: Reads a header-sized block of bit data and expands each source byte into output bytes by testing each bit position, writing one fill value for clear bits and another for set bits.
- `fill_row_offset_table`: Writes five pairs of row address words into a table, each row address advanced by a fixed row stride from the previous one.
- `parse_colour_map_chunk`: Scans a data block for the four-letter tag CMAP, then reads colour entries after it, converting each into a palette word and storing it for the colour registers.
- `copy_palette_words_to_buffer`: Copies thirty-two words from the source pointer into a fixed buffer two words apart, staging a full colour palette.
- `copy_indexed_table_words`: Copies twelve words from a table entry chosen by a state index into a fixed buffer, two words apart.
- `copy_indexed_table_quad`: Copies four words from a table entry selected by a second state index into a fixed buffer, two words apart.
- `manage_game_end_state`: Loads win or lose data files and graphics, parses color maps, and manages a state machine that handles the transition between game results and the high score table.
- `set_flag_and_call`: Stores the value 1 into a 16-bit word at offset 118 of the structure in a4 and then calls another routine.
- `call_init_score_table_start`: Loads a pointer from an offset of 20 in a structure into a0 and calls another routine.
- `call_init_score_table_offset_20`: Loads a pointer from an offset of 20 in a structure, adds 20 to it, and calls another routine.
- `call_init_score_table_offset_5152`: Loads a pointer from an offset of 20 in a structure, adds 5152 to it, and calls another routine.
- `call_init_score_table_offset_5172`: Loads a pointer from an offset of 20 in a structure, adds 5172 to it, and calls another routine.
- `init_score_table_defaults`: Sets initial values for a score table: two words to all-ones, two words to twenty, a fixed limit word, then loops over five entries copying data pointers into the table.
- `wait_for_fire_or_timeout`: Polls the fire button bit in CIAA_PRA within nested counted loops and calls another routine between passes, returning if the bit is low or if the outer loop counter reaches zero.
- `print_name_field`: Iterates through a field of 20 bytes from a pointer in a1, calling another routine for each byte that is not a space, and uses values from a4 to calculate offsets for a table and a second destination.
- `wait_for_button_with_delay`: Repeats an unnamed call and polls a bit in CIAA_PRA using nested loops, calling another routine between passes and using longer counted delays when the bit state changes.
- `cycle_four_frames_polling_input`: Advances a four-step counter at offset 128 of a structure in a4, calls two unnamed routines per step, and then polls an input bit in CIAA_PRA within nested counted loops.
- `format_score_digits_and_name`: Converts a 16-bit value from a structure in a4 into decimal digits, writes them to a buffer, and references the text WILL.
- `run_name_entry_screen`: Clears a twenty-character buffer to spaces, then enters a loop that polls a hardware register for input and updates the buffer by incrementing, decrementing, or toggling characters based on which button is pressed.
- `fill_record_arguments`: Calculates three different destination addresses using offsets from a4 and a value from a global variable, then calls another routine.
- `draw_score_table_rows`: Calls a routine to set a vertical position and then calls print_name_field to display a name and a score for eight consecutive rows of a high score table.
- `shift_table_entries_down`: Shifts a table of words down by one position, moving each entry into the slot before it, to advance a rolling table.
- `check_interrupt_status`: Checks bit 4 of a custom register and, if set, writes the value 16 to offset 156 of a custom register, then either calls two routines or performs arithmetic and jumps.
- `scan_envelope_level_table`: Scans a table of about 127 levels and selects the first entry above the current level, then applies the result to the envelope state and the audio volume registers.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_command_structure`: Increments a byte at offset 172 of a structure in a4, then either calls four routines to update audio registers or performs arithmetic and calls four routines to update a value in a structure.
- `process_command_structure`: Fetches a command structure from a table, extracts an index and command type, and updates various properties in a target structure including a pointer, two 16-bit values, and a 16-bit value at offset 10.
- `update_status_flag`: Extracts a value from a structure, stores it in a new location, and sets a status byte based on a comparison with another value in the structure.
- `update_structure_values`: Processes a command byte from a structure in a6 to perform various arithmetic operations on 16-bit and 8-bit values at offsets 16 and 18, or modifies bytes at offsets 3 and 170 of a structure in a4, and updates a destination value at offset 6 of a structure in a5.
- `toggle_memory_value`: Increments a counter in a structure and, upon reaching a specific value, selects a memory address from a table to toggle its contents between two specific constants.
- `xor_adjacent_field_bits`: Returns the exclusive-or of bit eight and bit nine of a 16-bit field from a structure, placed at bit nine of the result.
- `apply_directional_movement_step`: Decrements or increments several 32-bit position fields in a structure by fixed steps of 65536, an address, or an address based on input values and state flags, while also updating 16-bit counters and status bytes within the same structure.
- `update_grid_cell_state`: Clears two counters and reads a 16-bit cell from a global grid at the object's position.
- `consume_marker_cell_accumulate`: Checks a grid cell at a fixed offset from the object for one special value.
- `set_flag_if_marker_cell`: Clears a flag, then sets it when the grid cell at a fixed offset from the object holds one of two marker values.
- `check_cell_and_entry_hits`: Checks the grid cell under the object's position for one marker value and, if found, clears it or decrements a counter, sets state flags and adds 100 to a 32-bit accumulator.
- `resolve_entry_overlap_events`: Tests five active entries for overlap with the object's position.
- `dispatch_event_code_actions`: Checks an enabled flag and an input word, then branches based on an event code to perform specific actions: adding 100, 500, or 1000 to a 32-bit accumulator, incrementing or decrementing various 16-bit counters, setting counters to constant values, or stepping through a 15-entry sequence table to select a sub-action.
- `nudge_position_on_flag_clear`: Clears a 16-bit value at offset 58 of the structure in a4 if it is non-zero and the value at offset 98 is within a specific range, then adds a constant to six 32-bit position fields at offsets 0, 4, 8, 12, 16, and 20 relative to offset 174 if two 16-bit flags at offsets 60 and 62 are zero and a 16-bit flag at offset 54 is zero; it then calls another routine.
- `start_timer_on_button_press`: When no other motion is pending, reads the fire-button bit of the joystick port status and, if it is pressed and a delay counter is clear, starts a 25-step timer and records a mode value.
- `move_position_along_timer_path`: While a timer is running, moves the object's position fields by a step chosen from a table and a direction flag.
- `advance_entry_phase_table`: Advances five active entries through a short cycle, writing each one's value from a table plus an object offset.
- `update_entity_state_and_position_2`: Iterates through a list of structures, updating a 16-bit value at offset 2 of each structure by subtracting a value retrieved from a table u.
- `update_entity_state_and_position`: Iterates through a list of structures, updating a 16-bit value at offset 2 of each structure by subtracting a value retrieved from a table using an index at offset 24.
- `update_timer_and_counters`: Decrements a 16-bit counter in a structure and, when it reaches zero, resets it to 50 and increments two other counters that wrap at seven and fifteen respectively.
- `process_active_entries_in_turn`: For each of five active entries, copies its two position words and a 32-bit value into a scratch record and then calls another routine to process that record.
- `initialise_five_record_block`: Sets a group of control words in an output structure to constant values, with some derived from the scratch position.
- `fill_animated_grid_records`: Resets several control words, advances a counter that wraps at 50 and another that wraps at four, then walks a 12-by-20 block of level grid cells.
- `fill_grid_block_records`: Resets the same control words as the animated grid filler, then writes five 32-bit records for each cell of a 12-by-20 block without reading the grid values.
- `fill_grid_cell_records`: Resets the same control words as the other grid fillers, then walks a 12-by-20 block of level grid cells and writes five 32-bit records for each cell, using the cell value to offset the record address.
- `fill_scrolling_block_records`: Resets control words, writes two groups of four 32-bit records from a block chosen by a counter, and advances that counter, which wraps at seven.
- `update_animation_frame_records`: Decrements a delay counter in a structure and, if non-zero, increments a frame index that wraps at four.
- `toggle_value_fill_five_records`: Toggles between two stored 32-bit values and writes five records from the newly selected value at a fixed stride.
- `initialize_structure_fields`: Sets numerous 16-bit and 32-bit fields within a structure at a4 to specific constant values, copies a pointer from offset 8 to offset 4, and stores a new pointer at offset 36.
- `reset_object_starting_state`: Resets a subset of the same object fields to their starting constants, covering flags, counters and four 32-bit starting positions.
- `load_entry_records_from_table`: Copies four entry records from a table selected by an index into the entry block.
- `reinitialise_entry_when_timer_expires`: Decrements a countdown and, when it reaches zero, copies a fixed set of table words into a secondary entry record and clears its counters.
- `load_five_entries_and_reset`: Copies five entry records from a table selected by an index and zeroes their counters.
- `wait_for_raster_line_256`: Calls another routine to wait for a raster line and then writes the value 256 into a 16-bit word at offset 150 of the structure in a5.
- `wait_for_raster_line_32896`: Calls another routine to wait for a raster line and then writes the value 32896 into a 16-bit word at offset 150 of the structure in a5.
- `reset_positions_and_call_next`: Writes four 32-bit starting positions into a structure at offset 174 of a4 and then calls another routine.
- `decode_keyboard_key_code`: Reads the keyboard serial register, converts the byte to a key code and looks up the matching character in a key table.
- `handle_keypress_actions`: Reacts to the decoded key value: the letter p toggles a stored word once per press, the escape key clears one counter, and one other special key code clears a second counter.
- `call_routine_seven_times`: Calls another routine seven times in a loop.
- `divide_value_ten_seven_times`: Copies a 32-bit value into a scratch word, then seven times divides the scratch word by ten and calls another routine after each division.
- `call_next_routine_once`: Calls another routine once and then returns.
- `divide_counter_ten_twice`: Copies a 16-bit counter into a scratch word, then divides it by ten twice, calling another routine after each division.
- `divide_incremented_counter_twice`: Adds one to a 16-bit field, copies the result into a scratch word and divides it by ten twice, calling another routine after each division.
- `fill_five_records_from_args`: Stores a 32-bit argument, resets a group of control words to constants, and writes five 32-bit records with a fixed marker word.

### Top Secret (1992)(C. Simon)(PD).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
