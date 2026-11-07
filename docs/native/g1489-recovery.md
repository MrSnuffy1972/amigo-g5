# Game 1489 (deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01489). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911488 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 142 of 142 named (0 author symbols), 142 of 142 described (10 by rule, 16 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 62 of 62 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 101376 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1996 of 1996 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 19491 |
| code | 11992 |
| data | 784732 |
| padding | 41 |
| unused | 95232 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf:bootblock | 1 | 43 | 2 |
| logged disk load from Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1800 | 139 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf bytes 13312-14336 (code never loaded by a reference run) | 1 | 153 | 1 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf bytes 28160-29184 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf bytes 497664-526336 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf bytes 566272-636928 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: custom-format (non-DOS) disk, 1572 non-empty blocks: matched against track-loaded segments
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: the boot block reads 16896 bytes from disk offset 51200 to an address (found statically); recovered as a unit
- runtime segment an address: 62 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: bytes 13312-14336 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: bytes 28160-29184 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: bytes 497664-526336 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: bytes 566272-636928 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: 5 logged loads cover 118272 bytes of the image
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: 9 track-loaded blocks found in recovered runtime segments and classified through them
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 2-24 (bytes 1024-12800) outside the filesystem hold data, not code (opcode density 0.037, entropy 4.93)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 33-48 (bytes 16896-25088) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.56)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 50-50 (bytes 25600-26112) outside the filesystem hold data, not code (opcode density 0.047, entropy 5.63)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 52-53 (bytes 26624-27648) outside the filesystem hold data, not code (opcode density 0.047, entropy 4.34)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 61-61 (bytes 31232-31744) outside the filesystem hold data, not code (opcode density 0.039, entropy 4.12)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 65-65 (bytes 33280-33792) outside the filesystem hold data, not code (opcode density 0.023, entropy 4.30)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 68-68 (bytes 34816-35328) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.57)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 88-99 (bytes 45056-51200) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.59)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 154-170 (bytes 78848-87552) outside the filesystem hold data, not code (opcode density 0.043, entropy 5.84)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 172-194 (bytes 88064-99840) outside the filesystem hold data, not code (opcode density 0.044, entropy 6.24)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 198-360 (bytes 101376-184832) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.09)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 363-873 (bytes 185856-447488) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.98)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 875-880 (bytes 448000-451072) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.23)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.41)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 931-931 (bytes 476672-477184) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.59)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 934-937 (bytes 478208-480256) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.47)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 951-970 (bytes 486912-497152) outside the filesystem hold data, not code (opcode density 0.035, entropy 5.48)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 1089-1104 (bytes 557568-565760) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.32)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 1245-1745 (bytes 637440-893952) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.61)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)[cr skr]/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.055, entropy 5.54)
- 3 never-loaded region(s), 100352 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf:bootblock

- `run_boot_disk_io_sequence`: Runs as the boot block entry and issues a sequence of exec DoIO requests on an I/O request block.
- `issue_boot_read_request`: Sets the command, data pointer, length and offset fields of an I/O request to the boot start code and issues it through the exec DoIO call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_return_register_value`: Calls one routine and then returns the value held in a data register, which the call leaves unchanged.
- `cycle_table_entries_into_slots`: Advances a global index and, when it is nonzero, copies words selected from a table by that index into two destination slots.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `install_interrupt_server`: Stores two constant bytes and two routine addresses into fixed global locations, then calls the exec library through a vector to add an interrupt server.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `issue_io_request_command_nine`: Sets the command field of an I/O request to nine, clears its offset field and a flag byte, and issues the request through the exec DoIO call.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `issue_io_request_command_five`: Sets the command field of an I/O request to five and clears its flag byte, then issues the request through the exec DoIO call.
- `issue_io_requests_in_chunks`: Splits a large transfer into chunks of 5632 units, calling a routine for each full chunk.
- `prepare_io_command_nine`: Clears an offset field and a flag byte within an I/O request structure, sets its command field to nine, and executes the request via a function pointer.
- `read_block_via_io_request`: Sets up an I/O request with a buffer pointer and a length of 5632 and issues it through the exec DoIO call.
- `copy_quarter_length_longs`: Copies long words from a source buffer to a destination, with a count equal to a quarter of the byte count passed in the data register.
- `run_io_pair_and_latch`: Issues two I/O requests in sequence and then copies a long word from a fixed offset in a global block into a global variable.
- `compute_bitplane_region_offsets`: Computes screen offsets for a region from a descriptor record, using a row stride of forty bytes and a screen of two hundred rows across four bitplanes.
- `update_row_offset_pointers`: Stores a new value in a global when it differs from the last one and calls a routine to recompute dependent state.
- `reset_row_offset_pointers`: Stores a new value, calls a routine to refresh a stored offset, and computes row pointers using a forty-byte stride.
- `merge_masked_bitplanes`: Combines four bitplanes row by row with bitwise mask operations, writing each plane's result into the destination.
- `blit_masked_bitplanes_clipped`: Blits four bitplanes into a screen area, clipping the row count to two hundred rows.
- `decode_packed_bitplane_rows`: Reads a descriptor record for a region and decodes packed bitplane data from a byte stream into the target planes.
- `decode_bitplane_block_planes`: Reads a descriptor record and decodes a block of packed bitplane data into the planes, writing the second plane at an offset of 32000 bytes.
- `decompress_rle_bitstream`: Decodes a custom run-length encoded bitstream into a destination buffer.
- `decompress_rle_data`: Decodes a custom run-length encoded bitstream into a destination buffer.
- `decode_masked_run_length`: Decodes run-length coded bytes, using the low six bits of each control byte as a repeat count and writing the output with a wrap-around stride.
- `decode_repeat_byte_pairs`: Writes byte pairs from the input stream to the destination with a wrap-around stride, decrementing a counter that tracks the remaining repeat count.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_planar_word_stream`: Decodes a packed stream into sixteen-bit words for a block of bitplanes, deriving a count from the remaining width and storing it in a global variable.
- `decode_stream_into_strided_rows`: Decodes a stream whose control byte selects a literal or repeat mode, writing output into rows that advance by a fixed stride.
- `decode_compressed_planar_stream`: Decodes a multi-level compressed data stream into a planar bitstream by processing control bytes that define run lengths, byte pairs, or repeated single bytes.
- `decode_row_stream_with_counter`: Decodes a row stream by reading a control byte and writing byte pairs with a row counter that decides when the stride wraps.
- `decode_pairs_across_rows`: Writes byte pairs from the stream across successive rows, decrementing a row counter and returning once it reaches zero.
- `decode_rows_by_control_count`: Decodes rows selected by a control count, copying byte pairs into each row and advancing the destination by a stride.
- `decode_word_runs_with_mask`: Decodes word-sized runs from a stream, where the low six bits of a control word give the run length.
- `return_without_action`: The routine contains only a return and does no work.
- `blit_masked_planes_by_offsets`: Combines bitplanes row by row using a mask derived from four plane words, then writes each result back to the destination.
- `fill_tile_rows_and_mark`: Stores the low byte of a working value and a record field, marks the record's first word as 65535, and fills rows of the screen buffer with a nested loop that steps forty bytes per row.
- `fill_clipped_tile_rows`: Computes a row range from a record and a working value, clamping the end to row 199, and fills the matching forty-byte rows of the screen buffer.
- `build_offset_table`: Copies a long word from a stack-relative location and clears a counter.
- `tiny_ori`: Small routine of 1 instruction that merges 188 into d3.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 2 into d4.
- `tiny_ori_4`: Small routine of 1 instruction that merges 130 into d4.
- `rebase_offset_table_entries`: Clears a counter and, for each of six table entries, adds a fixed base offset to nonzero values, storing the results in a second table.
- `update_timed_event_records`: Walks a table of twenty-four-byte event records and, for each record with an active pointer, dispatches on its type code.
- `scan_event_records_for_refresh`: Decrements a countdown in the current record and steps through an array of twenty-four-byte records, calling a routine when the array is exhausted.
- `dispatch_event_by_type_code`: Dispatches a record on its type code.
- `advance_record_position_by_step`: For a type six record, decrements a counter and, when it is zero, adds the record's step amount to its position.
- `set_record_step_and_advance`: Stores a new value into a record field and advances the record's position by its step amount.
- `invoke_callback_for_type_twenty`: When a record's type code is twenty and a callback pointer is set, it calls the callback and continues the record walk.
- `clear_records_and_count_active`: Clears the first long word of each record in an array of twenty-four-byte records and counts the records that are active.
- `accumulate_active_channel_flags`: Walks an array of records and folds the value of each record whose flag word is nonzero into an accumulator.
- `run_command_stream_step`: Reads command words from a stream and dispatches on the low byte.
- `handle_stream_setup_commands`: Reads command words in a loop and stores long-word operands into the channel record for command two.
- `store_word_and_long_operands`: Processes operand commands in a loop: command two stores a long word into the record and command one stores a word.
- `store_word_and_set_global`: Processes operand commands: command one stores a word into the record, and command four stores a word into a global variable and calls a routine through the global vector.
- `set_global_call_offset`: Handles command four by storing a word into a global, then calling a routine through the global vector with an offset computed from a base constant.
- `store_long_and_return_pointer`: Handles command five by storing a long word into the record and saving the operand pointer, then returns.
- `store_long_pair_and_return`: Handles command six by storing two long words and a pointer into the record and returning.
- `add_operand_to_record_fields`: Adds a word operand to one of two record fields depending on the command code, then continues reading operands from the stream.
- `add_operand_or_jump_back`: Handles command eight by adding a word operand to the second record field.
- `jump_back_or_combine_value`: Handles command nine as a relative backward jump and command ten by calling a routine and combining its result with an operand, storing the result in the record.
- `store_masked_helper_value`: Handles command ten by calling a routine, ANDing its result with an operand from the stream, and storing the masked value and a pointer in the record.
- `dispatch_commands_eleven_to_thirteen`: Handles command eleven by calling a routine and advancing the operand pointer.
- `set_or_reload_operand_pointer`: Handles command twelve by storing an operand-derived pointer into the record.
- `set_global_byte_from_operand`: Handles command thirteen by reloading the record's stored stream pointer.
- `store_base_offset_pointer_marker`: Handles command fifteen by computing a pointer from an operand plus a fixed base offset, storing it in the record, and marking the record's first word with 254.
- `set_pointer_and_clear_global`: Handles command fifteen by storing a pointer computed from an operand, and command sixteen by setting a global word to 65535.
- `choose_random_masked_operand`: Handles command seventeen by taking a value from a routine, masking it to its low four bits, and reducing it by a stored threshold.
- `apply_random_nibble_offset`: Handles command seventeen by drawing a returned value, masking it to four bits and adjusting it against a stored value.
- `dispatch_commands_eighteen_to_twenty`: Dispatches command codes eighteen to twenty.
- `loop_until_command_changes`: While the command code equals eighteen, clears a flag and reads further commands in nested loops until the code changes.
- `store_operand_or_clear_field`: Handles command twenty by storing the command word and an operand pointer in the record and returning.
- `clear_record_status_word`: Clears one word in a record, the status field at offset six, and returns.
- `increment_and_clear_buffer`: Increments a global counter and then clears 2000 groups of four long words starting at a global buffer pointer.
- `wait_for_hardware_flag`: Calls a routine through a pointer offset from a global base, then busy-waits until a bit in a hardware status global is set.
- `halt_in_infinite_loop`: The listing contains only an infinite loop and performs no other work.
- `poll_right_middle_buttons`: Reads the right and middle mouse button state from the POTGOR hardware register, then calls seven routines that handle the result.
- `poll_fire_and_disk_status`: Reads the CIA port A register, which holds the joystick and mouse fire buttons and the disk status, and calls four routines based on its value.
- `dispatch_two_input_routines`: Calls two routines in turn and has little other work of its own.
- `read_ciaa_peripheral_status`: Reads the status of the joystick fire buttons and disk drive from the CIAA peripheral address register.
- `remove_interrupt_server_mouse`: Checks the right and middle mouse buttons in the POTGOR register and, depending on them, removes an interrupt server through the exec library stub.
- `tiny_ori_5`: Small routine of 1 instruction that merges 4 into d0.
- `dispatch_two_helpers`: Calls two routines in sequence with minimal work of its own.
- `call_pair_of_routines`: Calls a pair of routines with no further work of its own.
- `poll_until_magic_value`: Loops until a checked value equals a fixed magic constant, calling a routine and polling a hardware flag on each pass.
- `close_device_and_remove_port`: Sets a state variable to one, closes a device and removes a message port through exec calls, and stores two global values.
- `clear_block_of_longs`: Clears 2000 groups of four long words starting at a global buffer pointer.
- `decrement_nibbles_each_tick`: Sets a timing global to 256 and, on each tick once a hardware status bit is set, decrements the low four bits of each of fifteen words when they are nonzero.
- `run_nibble_decrement_tick_loop`: Runs the nibble-decrement tick loop after setting its timing global to 256 and clearing the tick counter.
- `decrement_nibbles_each_tick_2`: Waits for the hardware status bit on each tick, then decrements the low four bits of each of fifteen words when they are nonzero.
- `set_bit_and_reset_timer`: Sets bit one of a control global, stores 256 in a timing global, then calls a routine.
- `initialize_music_player_state`: Stores two pointers derived from a base pointer, clears a series of player globals, and sets a master value of fifteen.
- `update_channel_period_accumulators`: Walks four-long channel records and, for each record with a nonzero first long, replaces that value and adjusts an accumulator by the difference.
- `advance_channel_period_chain`: Updates a chain of four-long channel records in sequence, replacing each value and adjusting its accumulator by the difference.
- `reset_channel_volumes_and_dma`: Sets the volume-register globals of all four audio channels to zero and a master value to fifteen.
- `force_low_bit_and_call`: When a status bit in a global is set, calls a routine, then calls another routine with the low bit of its data argument forced on.
- `advance_pattern_step_counter`: Increments a tick counter and, when it reaches the stored tick limit, resets it and advances a step counter.
- `advance_pattern_row_pointer`: When the step counter reaches sixty-five, advances the pattern position through its table and reloads the row address.
- `start_pattern_playback`: Waits until a pattern table field is nonzero, points the row pointer at its entry, marks the player active, and calls eight routines before clearing a control global.
- `advance_row_and_run_channels`: Advances the row pointer by sixteen bytes, calls eight routines in turn, and clears a control global.
- `apply_channel_command_word`: Decodes a command word whose top nibble selects one of four audio channels.
- `load_channel_zero_registers`: Loads the sample pointer, length, period and volume values for audio channel zero from the channel table and writes them into that channel's hardware-state globals.
- `load_channel_one_registers`: Handles two modes for audio channel one: mode two loads the sample pointer, length, period and volume from the channel table, and mode three stores a pointer for the channel.
- `load_channel_two_registers`: Handles the third audio channel: mode three stores the sample pointer, length, period and volume from the channel table, and other modes store a pointer and clear a flag.
- `load_channel_three_registers`: Loads the sample pointer, length, period and volume for the fourth audio channel from the channel table, and clears its status global.
- `apply_volume_effect_command`: Decodes a command byte in which the middle nibble selects an effect.
- `set_player_flag_from_command`: Handles three special command codes: code 2048 stores an operand in a global word, code 1536 clears bit one of a control global, and code 1792 sets that bit.
- `clear_or_set_control_bit`: Clears bit one of a control global when the command code is 1536, and sets it when the code is 1792.
- `set_control_bit_on_code`: Sets bit one of a control global when the command code is 1792.
- `set_channel_pattern_counter`: Stores a decremented step count in a global and derives a pointer into the pattern table from it, then sets a control global to 64.
- `store_negated_value_by_index`: Negates the data value and stores it in one of four player globals, chosen by an index in a data register.
- `store_value_three_way`: Stores a value into one of three player globals chosen by an index in a data register.
- `store_value_if_index_three`: Stores a value into a player global when the index is three, and otherwise into a default player global.
- `store_default_player_value`: Stores the data-register value into a single player global without any index test.
- `store_value_four_way`: Stores a value into one of four player globals, chosen by an index of one, two or three, with any other index selecting the fourth global.
- `store_value_two_way`: Stores a value into one of three player globals based on an index of two or three; any other index selects the third global.
- `store_value_by_index_flag`: Stores a value into a player global when the index is three, and otherwise into a second player global.
- `store_value_into_global_slot`: Stores the data-register value into a fixed player global without any index test.
- `update_audio_channel_registers`: Adds per-channel period and volume deltas to the player globals for all four audio channels, then writes the pointer, length, period and volume values to the audio hardware registers.
- `add_table_byte_to_global`: Reads a byte from a table addressed by a player global and adds it to that global, returning the masked sum.
- `clear_audio_register`: Clears a value through a pointer.
- `update_audio_register`: Performs a multi-instruction operation involving a pointer and a global variable.
- `add_audio_value`: Adds a value to a location via a pointer.
- `increment_audio_value`: Increments a value at a memory location using a pointer.
- `move_audio_value`: Moves a value to a memory location through a pointer.
- `store_audio_value`: Stores a value into a memory location using a pointer.
- `tiny_ori_6`: Small routine of 1 instruction that merges 0 into d5.
- `adjust_audio_pointer`: Adjusts a pointer and modifies a value at that location.
- `set_audio_parameter`: Sets a value at a memory location using a pointer and arithmetic operations.
- `reset_audio_counter`: Clears or decrements a value at a memory location via a pointer.
- `update_period_chain_pointer`: Calculates a new address by adding an offset from a global variable to a pointer and stores the result back to that pointer.
- `add_table_byte_to_targets`: Reads a byte from a table indexed by the low 15 bits of a word, masks off one bit, and adds the result to a global variable and to that same word, returning the low byte of the sum.
- `update_indexed_offset_pointer`: Calculates a new value by adding a global variable to an offset derived from a combination of a register and a memory value, then updates a global pointer with the result.
- `add_indexed_byte_to_global`: Looks up a byte in a table indexed by the low 15 bits of a word, masks off one bit, adds it to a global variable and stores the sum back into the global.
- `initialize_four_channel_records`: Fills four consecutive four-word records with values derived from a base offset and fixed addresses, then sets the low four bits of a status word to mark them as valid.
- `copy_selected_channel_records`: Selects a 14-byte source entry by index, or a global default when the index is zero.
- `copy_channel_records_from_table`: Selects a 14-byte source entry from a table by index, then for each bit set in the mask copies its pointer and three words into the matching channel record and sets that channel's flag bit.
- `guarded_fetch_entry_fields`: Returns the input value unchanged when a guard word is set.
- `select_entry_and_store_fields`: Uses a word argument times fourteen as an offset into a table to fetch an entry.
- `update_dma_control_state_machine`: Processes a bit-flagged control word against a set of channel records, updating counters, pointers and a state block.

### Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2)[cr SKR].adf bytes 13312-14336 (code never loaded by a reference run)

- `setup_bitplanes_and_load_libraries`: Opens the Amiga system libraries, checks available memory, and clears a small table.
