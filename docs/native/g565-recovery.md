# Game 565 (Robocop 2): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00565). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1804688 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 115 of 115 named (0 author symbols), 115 of 115 described (1 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 846 of 846 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3920 of 3920 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 43034 |
| code | 17318 |
| data | 1741259 |
| padding | 5 |
| unused | 3072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf:bootblock | 1 | 45 | 2 |
| logged disk load from Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf boot-block load | 1 | 2823 | 69 |
| logged disk load from Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf boot-block load | 1 | 21 | 0 |
| logged disk load from Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf boot-block load | 1 | 430 | 21 |
| logged disk load from Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf boot-block load | 1 | 119 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 482 | 21 |

## Cases handled

- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- robocop 2 (1991)(ocean)[cr melnok](disk 2 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1752 non-empty blocks: matched against track-loaded segments
- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: the boot block reads 42496 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: the boot block reads 512 bytes from disk offset 845312 to an address (found statically); recovered as a unit
- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: the boot block reads 2048 bytes from disk offset 845824 to an address (found statically); recovered as a unit
- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: the boot block reads 20992 bytes from disk offset 870400 to an address (found statically); recovered as a unit
- runtime segment an address: 277 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: 4 logged loads cover 66048 bytes of the image
- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: 7 track-loaded blocks found in recovered runtime segments and classified through them
- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: blocks 644-812 (bytes 329728-416256) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.97)
- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: blocks 814-977 (bytes 416768-500736) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.17)
- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: blocks 1656-1699 (bytes 847872-870400) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.54)
- robocop 2 (1991)(ocean)[cr melnok](disk 1 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf: blocks 1741-1759 (bytes 891392-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)
- robocop 2 (1991)(ocean)[cr melnok](disk 2 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 2 of 2).adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- robocop 2 (1991)(ocean)[cr melnok](disk 2 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 2 of 2).adf: blocks 813-1008 (bytes 416256-516608) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.67)
- robocop 2 (1991)(ocean)[cr melnok](disk 2 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 2 of 2).adf: blocks 1010-1016 (bytes 517120-520704) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)
- robocop 2 (1991)(ocean)[cr melnok](disk 2 of 2)/Robocop 2 (1991)(Ocean)[cr Melnok](Disk 2 of 2).adf: blocks 1022-1759 (bytes 523264-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.84)
- 5 packed track region(s), 1042432 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_disk_read_dma` (Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf boot-block load +an address to an address (2048 bytes)): Writes the disk sync word, the buffer address, the disk length and the DMA and disk control values, then calls a subroutine to start the transfer. Kind: disk hardware/sync check. Pass path: sub_03EC returns normally to u3__sub_03AA.

## Functions


### Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf:bootblock

- `load_regions_parse_entry_table`: Boot block entry that issues device I/O requests through the operating system to read data into memory, copies three data regions to fixed addresses, then walks a table of entries whose command byte stream sets per-entry state.
- `copy_regions_parse_entry_table`: Copies three data regions to fixed memory addresses, then walks four table entries with a command byte stream, storing per-entry state; it calls a system library routine and returns its result.

### logged disk load from Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf boot-block load

- `load_configuration_entry_and_reset`: Loads a 40-byte configuration entry chosen by an index into global cells, writes a fixed set of configuration values, resets position fields for each active record and makes four routine calls to finish setup.
- `fill_record_rows_after_wait`: Waits for bit six of a status byte to clear, writes a set of control fields into the record and fills four rows of entries from a table, each row stepping by 1920 bytes.
- `update_scroll_and_queue_rows`: Updates a scroll value in steps of eight and clamps it, and when the scroll crosses row boundaries it waits for a status bit, writes control fields and row addresses from tables and returns a status word.
- `clear_counters_count_table_entries`: Clears eight global cells, then walks a table of words until a negative terminator, incrementing a counter and calling a routine for each entry.
- `dispatch_handler_from_table`: Jumps through a table of offsets indexed by the word at an address register, so it dispatches to a handler chosen by that index.
- `hit_test_rectangles_and_dispatch`: Scans a list of four-word rectangles until a negative entry, testing whether a point at a fixed offset from a global lies inside each; on the first hit it calls the handler selected from a table by a stored index.
- `blit_visible_table_entries`: Walks a list of six-word entries until a negative terminator, and for each entry inside a window it copies its word rows into the destination buffer at a stride of 160 bytes and updates the record fields of the object being drawn.
- `blit_visible_entries_continue`: Continues the same entry walk as its neighbour: for each in-window entry it copies word rows at a stride of 160 bytes, updates record fields and dispatches a handler selected from a table, returning when the list ends.
- `compute_row_address_and_fill`: Converts two input coordinates into a row offset and step, returns minus one for positions outside the limits, and otherwise fills a block of four entries with row addresses and steps, returning zero.
- `set_state_word_from_flag`: Sets a state word from a routine result when a flag byte is zero; otherwise it writes two fixed values to the state word with one routine call between them.
- `set_state_pair_with_routine`: Writes a fixed value to the state word, calls one routine, then writes a second fixed value to the state word.
- `decrement_bcd_counter_pair`: Decrements a two-digit packed decimal counter held in two bytes, using decimal adjustment and borrowing from the high digit when the low digit was zero.
- `store_four_fields_by_index`: Stores four input values into four parallel tables of the record at an offset selected by the low bits of a global index, then advances that index by two.
- `state_dispatch_blit_setup`: Runs a state machine over a global state byte, dispatching to a handler table and calling itself on some branches.
- `call_nonzero_table_entries`: Walks a table of 16-bit words and calls the routine addressed by each nonzero entry through an indirect call.
- `advance_counter_with_bit_update`: Sets one bit in a control byte chosen by a bit index, clears another bit, and adds a fixed negative step to a 16-bit word.
- `move_with_tile_collision`: Moves a 16-bit position by a direction value and checks the destination in a tile map of fixed-width rows.
- `blit_image_record_rows`: Looks up an image record from a table, waits for the blitter to be idle, then programs its mask, size and source registers and writes row addresses at a fixed stride.
- `start_blit_with_pointers`: Waits for the blitter to be idle, then loads its control, mask, pointer and size registers from the caller's register values.
- `blit_tile_grid_cells`: Converts a counter into a table offset, programs the blitter with fixed masks, and loops over a grid of cells.
- `find_matching_zone_and_mark`: Scans a short table backwards for an entry whose bounds contain the current value and a second coordinate.
- `rotate_and_xor_with_timer`: Rotates a 16-bit state word by an amount read from a hardware byte, then XORs in a value built from two timer-low bytes.
- `read_joystick_port_state`: Reads the joystick port data word, keeps two of its bits, and maps them through a lookup table into a state byte stored in a global.
- `copy_byte_and_shadow_global`: Copies a byte from a source pointer into the byte just before a destination pointer, then copies one global byte into a second global and returns that value.
- `mark_first_box_hit_entry`: Scans a table of entries whose type byte is nonzero and tests the given x and y coordinates against each entry's box, with box size looked up from a table by entry type.
- `scan_entries_for_point_hit`: Continues scanning a table of typed entries from a start pointer until it reaches one whose box contains the given coordinates.
- `update_active_object_records`: Walks a table of object records, skipping inactive ones, and updates each live record's position and flags against fixed coordinate bounds.
- `advance_object_records_entry`: Runs the same object-record walk as its neighbouring routine from a second entry point, and returns the accumulated value once the record pointer passes the end of the table.
- `stub_range_check_noop`: Converts a coordinate to a column index and compares it with two bounds, but both branches return immediately, so the routine changes no state.
- `step_timed_record_list`: Advances a global timer and walks a fixed list of records, updating each active record's counter and passing it to a per-type handler through a dispatch.
- `stub_endless_loop_wait`: The listing shows only an endless loop with no body, so the routine never returns and does no work.
- `claim_free_slot_record`: Searches a flag table from the last slot down for a free slot.
- `step_active_record_motion`: Walks eight record slots from last to first and, for each active one, adds its velocity to its position and applies a per-type acceleration to its vertical speed.
- `add_packed_decimal_counter`: Adds a four-byte value into a four-byte decimal counter one byte at a time, applying a decimal adjust after each byte so every byte stays in binary-coded decimal form.
- `copy_three_byte_table_entry`: Clamps an index to at most ten, then copies three consecutive bytes from a lookup table at that index's position into the buffer the caller points to.
- `countdown_and_blit_two_rows`: Clears a six-entry flag table, calling a subroutine for each entry that was set, then runs a countdown on a global timer that reacts to two special codes.
- `init_and_copy_strided_words`: Sets a global field to a fixed value, then copies fifteen consecutive words from the input buffer into a global table, writing every other word position.
- `copy_strided_words_to_table`: Copies fifteen words from the input buffer into a global table at a stride of two words, without first setting any field.
- `clamp_nibbles_into_table`: Steps a global level value up or down by one within a limit.
- `update_four_slot_velocities`: Updates four slots from highest to lowest.
- `add_slot_record_and_count`: Stores a flag byte, a fixed position value, a word and a long velocity into one of four slots chosen by a counter, then increments that counter.
- `test_entry_boxes_against_area`: Computes corner coordinates from input values, then checks one group of five entries and one of four against that area.
- `blit_rows_from_index_table`: Stores a table-selected value in a global, waits for the blitter to be idle, and programs its masks and size.
- `add_bytes_to_bcd_counters`: Adds one byte from the input pointer into a decimal counter and a second counter, applying a decimal adjust to each.
- `store_integer_to_pointer`: Checks if the value at a pointer location is zero and, if so, stores an integer into that location before incrementing a global index.
- `fetch_next_glyph`: Increments a pointer within a data stream and updates a global index.
- `draw_text_glyph_from_stream`: Reads the next byte from a text stream held behind a slot pointer.
- `reset_game_state_block`: Clears the global state block by zeroing its bytes, sets several global fields to fixed initial values, and writes an initial copper list address and a DMA control value to the custom registers.
- `setup_cia_and_dma_registers`: Sets a global flag, calls two subroutines, then clears the top bit of a CIA port byte and writes fixed values to CIA registers.
- `interrupt_count_and_reset`: Sets a global value to four and increments a byte counter.
- `setup_bitplanes_and_sprites`: Writes four bitplane pointers, a scroll value and four sprite pointers into the custom chip registers, computing each sprite's position word from a table.
- `blit_three_row_block`: Waits for the blitter to be idle, programs its masks and size, then writes three row pairs.
- `reset_then_call_eight_times`: Sets a global value to a fixed start, calls a subroutine, adjusts the global by a computed amount when it is nonzero, and then calls a subroutine eight times in a loop.
- `call_eight_times_if_set`: When the input value is nonzero, calls a subroutine eight times in a row.
- `parse_text_metadata_control_chars`: Reads a control character from a string and uses it to either modify a jump table entry at the current routine's address or update a global pointer to an element within a table of structures.
- `parse_text_string_metadata`: Iterates through a string to identify specific control characters, updating a global pointer and a counter, and populates a structure with data extracted from a table based on the character values.
- `run_text_object_state`: Runs a text object's state machine.
- `combine_object_bits_into_plane`: Adds the object's position to a pair of offsets and, when the row lies inside the visible area, combines four rows of bits into the bitplane bytes.
- `dispatch_stream_command`: Reads the next command pair from a stream pointer and either runs a table-driven handler selected by the command byte or loads new parameters from the stream.
- `branch_to_offset_9768`: Performs a short branch to the instruction at offset 9768.
- `match_stream_marker_and_mark`: Increments a counter and compares the next stream byte with a value derived from it.
- `branch_to_offset_9864`: Performs a short branch to the instruction at offset 9864.
- `branch_to_offset_97c6`: Performs a short branch to the instruction at offset 97c6.
- `jump_to_reset_or_call_24_times`: Jump stub: passes control straight on to reset_or_call_24_times.
- `process_stream_data`: Checks a 16-bit value from a global variable, and if it equals 37, updates a pointer in a global variable, clears several global words, and returns; otherwise, it increments the global variable, performs a loop that calls draw_glyph_cell_at_offset 12 times, and returns.
- `reset_or_call_24_times`: If the global counter is not at its end value, clears it and calls a subroutine twenty-four times in a row.
- `draw_glyph_cell_at_offset`: Draws one character cell at a computed plane offset.
- `dispatch_triggered_script_entries`: Scans a table of trigger entries for one whose stored pattern matches the current byte, incrementing its count.
- `scan_trigger_entries_from_pointer`: Scans trigger entries from a start pointer until an end marker.

### logged disk load from Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf boot-block load

- `unpack_table_entry_fields`: Copies a four-byte table entry into a work area and splits a 16-bit table word into a low twelve-bit field and a high four-bit field, storing both.
- `find_matching_index_in_four`: Searches four stored bytes from the last to the first for one that equals a stored value, and records the matching position in a field.
- `service_four_channel_status`: For each of four channels whose entry is not the sentinel value, calls several subroutines and checks a status bit.
- `pulse_control_bit_handshake`: Saves a control byte, then repeatedly clears and sets bit one of it, calling subroutines in between, and stops when a status bit goes clear.
- `wait_status_bit_clear`: Waits in a loop while bit five of a status byte is set, then returns.
- `reset_cia_control_byte`: Keeps the top two bits of a control byte, sets a low bit, and then stores a fixed value into the same byte.
- `select_drive_port_bits`: Updates a drive-control port byte by setting and clearing groups of bits, then clears one bit chosen by a channel index plus three.
- `run_optional_step_sequence`: Calls a subroutine, then calls two more subroutines only when a flag byte is zero, and finally calls one more subroutine.
- `load_timer_and_wait_flag`: Stores the low and high bytes of the input word into the timer registers, then waits in a loop until the timer's flag bit is set.
- `wait_bit_then_clear_word`: Repeatedly calls a subroutine while bit four of a status byte is set, then clears a 16-bit word at an offset derived from the input index.
- `pulse_drive_control_bits`: Sets bit one of a drive-control byte, clears bit zero, then sets bit zero again, and calls a subroutine afterwards.
- `clear_drive_control_bits`: Clears bits one and zero of a drive-control byte, sets bit zero, and calls a subroutine.
- `compute_track_and_sector_index`: Divides a position counter by eleven to get a track value, sets a flag when the track has not changed since last time, and stores the remainder as the current sector and the following index as the next sector.
- `seek_track_to_target`: Steps the head toward a target track one position at a time, calling a subroutine for each step and recording the track reached.
- `setup_disk_read_dma`: Writes the disk sync word, the buffer address, the disk length and the DMA and disk control values, then calls a subroutine to start the transfer.
- `wait_disk_block_then_clear`: Waits until a status word has its second bit set, then writes two fixed control values to hardware registers.
- `decode_mfm_sector_from_track`: Scans a raw MFM track buffer for sync words, decodes the odd and even bit streams of each sector header, and when the sector falls in the wanted range copies its data block into the destination buffer.
- `copy_sector_bytes_with_count`: Copies bytes from a source buffer to a destination buffer until a remaining count reaches zero or the 512-byte limit is hit, then stores the updated pointer and remaining count.
- `strobe_drive_control_high_bits`: Sets the upper five bits of a drive-control port byte, clears the middle bits, and sets the upper bits again, producing a strobe pulse on those lines.
- `decode_variable_length_bitstream`: Reads bits from a stream of 32-bit words, least significant bit first, and decodes variable-length codes through a subroutine.
- `read_n_bits_from_stream`: Reads a count of bits from a stream of 32-bit words, accumulating them into a word one bit at a time, and refills from the preceding word when the current one is used up.

### logged disk load from Robocop 2 (1991)(Ocean)[cr Melnok](Disk 1 of 2).adf boot-block load

- `fetch_shifted_bit_word`: Returns the longword just before the address pointer, shifted right one bit with its top bit set.
- `advance_bit_reader_count`: Shifts a bit register right once per count step, refilling it from the preceding longword with its top bit set whenever it empties, and stops when the count is used up.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_music_voice_streams`: When the input is one, sets up four voice records from a table, each with a stream pointer, and reads the first command bytes of each stream.
- `update_four_voice_flags`: On each tick, updates four voice-state words: when a pending bit is set it advances a counter, merges each voice's flags with the saved values, calls subroutines for voices that have data, and clears the pending bits.
- `reset_four_voice_state`: Clears two global values and calls a subroutine four times, once per voice, then clears a pending-state word.
- `init_voice_record`: Clears the first word of a voice record, sets its active flag and clears its next field, then clears a word at a second pointer.
- `start_voice_streams_from_table`: Sets up voice streams from a table in the same way as its neighbouring routine, reading the first command bytes of each stream that has an entry.
- `init_voice_streams_with_value`: For each of four voice table entries whose offset is nonzero, clears the voice record fields, stores the input value into the record, and reads its command stream with the same command decoding as its neighbours.
- `decode_voice_command_stream`: Decodes the command bytes of a voice stream: code one stores the stream pointer, codes two and three store parameter bytes, a non-negative byte selects a table entry, and the sentinel ends the stream.
- `decode_voice_stream_commands`: Decodes voice stream bytes: code one stores a parameter byte, code two stores another, a non-negative byte selects a table entry, and the sentinel ends the stream.
- `decode_voice_stream_loop_codes`: Decodes voice stream bytes, storing a loop pointer on the loop code and a parameter byte on the following code.
- `init_voice_records_from_streams`: For each voice whose table offset is nonzero, clears its record fields, stores the input value, and decodes its stream with the same codes as its neighbours.
- `forward_unrecovered_call_short`: A 19-instruction routine that calls one unnamed routine and is reached from three unnamed routines.
- `update_buffer_pointer_and_counters`: Decrements byte counters at offsets 24 and 25 of a structure in a4 and updates a pointer at offset 6 or offset 2 based on the value of a byte read from a table.
- `store_pointer_and_branch`: Stores the value in a3 into a global variable at an offset from a4 and then branches to another routine.
- `set_bit_and_move_byte`: Sets bit 5 of d7, moves a byte from the address in a3 to offset 81 of a structure in a4, and then branches to another routine.
- `clear_bit_and_branch`: Clears bit 5 of d7 and then branches to another routine.
- `increment_offset_and_branch`: Adds 256 to the word at offset 82 of a structure in a4 and then branches to another routine.
- `modify_bits_and_clear_word`: Clears bit 4 and sets bit 7 of d7, clears a word at offset 76 of a structure in a4, and then branches to another routine.
- `set_bit_and_move_bytes`: Sets bit 3 of d7, moves a byte from the address in a3 to offset 36 of a structure in a4, moves another byte from the incremented address in a3 to offset 37 of a structure in a4, and then branches to another routine.
- `forward_unrecovered_call_tiny`: An eight-instruction routine that calls one unnamed routine and is reached from outside normal control flow.
- `run_unrecovered_large_body`: A 181-instruction routine called by one unnamed routine that calls one other.
- `update_four_audio_channels`: Writes the DMA control register and, for each channel whose flag bit is set, sets its period and volume registers and loads its sample pointer and length.
