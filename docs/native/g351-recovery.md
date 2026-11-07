# Game 351 (Abyss): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00351). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901708 top-level bytes in 22 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 319 of 319 named (0 author symbols), 319 of 319 described (125 by rule, 187 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2578 of 2578 (100.00%) |
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
| Abyss.adf:Run | 2 | 117 | 2 |
| Abyss.adf:Main | 3 | 11246 | 199 |
| Disk-Validator | 1 | 386 | 1 |
| SKID_ROW | 2 | 24 | 4 |
| 15 | 1 | 2 | 1 |
| Diskfont.library | 13 | 830 | 53 |
| Abyss.adf:4 | 2 | 405 | 15 |
| Abyss.adf:bootblock | 1 | 10 | 1 |
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

- `update_channel_from_sequence_entry` (4): Loads the next sequence entry for an audio channel by copying a pointer and length from a source table and adding a stored offset. Kind: track/sync check. Pass path: the instructions following the bsr.w an address calls, specifically the logic that updates a5/a6 or modifies d0/d1, or the branch where the routine completes without triggering a failure (the code modifies hardware registers like an address, an address.

## Functions


### Abyss.adf:Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Copies a longword from one register to another and then performs a short branch.

### Abyss.adf:Main

- `clear_buffer_and_initialize`: Zeroes a block of two hundred longwords, updates a global pointer, and calls a library function or another routine based on a bit check and a library return value.
- `execute_conditional_logic`: Checks a return value from a function call and either assigns a constant or executes a sequence of three function calls with different arguments.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a1).
- `xor_scramble_buffer_bytes`: Walks a buffer and exclusive-ors each byte with the sum of its index and the buffer's first byte.
- `copy_block_into_allocated_buffer`: Allocates a buffer, copies a block of data into it if successful, releases the buffer, and then calls another routine.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a2).
- `stage_block_then_set_state`: Sets a state byte, attempts to allocate a buffer to copy data, releases it, and then sets a state byte to a new value before calling a routine with a specific parameter.
- `push_word_to_stack`: Writes a word from a register to a location pointed to by a decrementing pointer register.
- `set_palette_to_white`: Iterates through sixteen color registers, setting each to a fixed white color via an indirect call, then calls another routine via an indirect pointer.
- `draw_eight_segment_outline`: Draws a fixed eight-segment outline at a given position and size, choosing a pen color based on the sign of the vertical argument.
- `draw_four_fixed_shapes`: Clears a rendering area via an indirect call and then draws four shapes at fixed positions and sizes using an outline routine.
- `draw_hall_of_fame_table`: This routine draws the title ABYSS HALL OF FAME, then loops over ten entries, drawing each row through a row routine, changing pen and drawing mode before the fourth entry, and finishes by writing a buffer to the rastport.
- `draw_hall_of_fame_row`: Draws one table row: a rank label that is blank for the first nine entries, a value formatted from the nibbles of a stored word, and a seven-character name field right-aligned by its measured width.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `shift_table_entries`: Checks a condition against a table and, if met, shifts table entries upward and updates a value or a character in a loop.
- `initialize_and_draw_header_rows`: Sets the palette to white, clears and draws fixed shapes, and initializes a rank-label row.
- `draw_rank_row_fixed_columns`: Draws a rank label that is blank for the first nine entries, then a value at column 100 and a three-character field at column 170.
- `update_rank_index_from_input`: Processes messages from a port and updates a counter or value based on certain input or message content, while also handling specific return or stop conditions.
- `format_hall_of_fame_string`: Constructs a formatted string for a table or message by parsing a lookup table and writing characters or numeric values into a buffer.
- `branch_to_address`: Performs a relative branch to a specific address.
- `drain_message_port_replies`: Repeatedly takes messages from a port through an indirect call and replies to each one until none remain.
- `wait_for_message_value`: Waits for a message to arrive on a port, replying to the message, and returns a word field taken from the message content.
- `initialize_graphics_and_palette`: Initializes a library, sets a pointer, and if successful, sets a palette or color register, clears a rendering area, and resets a global variable.
- `draw_moment_message_panel`: Clears the rastport, draws a panel at fixed coordinates, and writes a text message beginning with a specific string.
- `wait_until_flag_set`: Repeatedly calls a routine until a certain global byte becomes non-zero.
- `initialize_game_state_and_graphics`: Initializes global pointers and memory buffers, configures certain hardware registers, and sets up a sequence of conditional checks to prepare the system or display state.
- `center_and_draw_string`: Calculates a horizontal offset by subtracting a string length from a fixed width and dividing by two, then calls a routine to draw a string at that offset, followed by a routine to draw a string at the original position.
- `setup_blitter_copy_registers`: Sets the blitter control, mask, and modulo registers to fixed values and configures source and destination pointers with a fixed block size.
- `draw_grid_background_and_labels`: Clears the rastport and draws a grid of vertical and horizontal lines using alternating pen colors.
- `count_frame_and_ack_interrupt`: Increments a frame counter byte and writes 32 to the interrupt request register, acknowledging the vertical-blank interrupt.
- `setup_interrupt_and_frame_loop`: Installs an interrupt handler and an initial countdown, then enters a loop that clears a frame state, waits on a countdown and a flag, copies four values into a table, and configures the interrupt-enable and DMA control registers.
- `draw_message_boxes_and_text`: Clears the rastport and draws three boxes at fixed positions, writes a string beginning with a specific text, and finishes by writing a sixteen-character buffer.
- `draw_two_panel_screen_layout`: Clears the rastport and draws boxes and horizontal rules at fixed coordinates, then writes a seriesed of text labels, centering some by measuring their width.
- `stop_audio_channel_zero_2`: Checks a flag in a global variable and, if set, clears the flag and updates the audio channel zero or DMA control registers with a new pointer and a length value.
- `stop_audio_channel_zero`: Clears a flag and writes to the audio and DMA control registers to stop audio channel zero.
- `stop_audio_channel_one_2`: Checks a flag in a global variable and, if set, clears the flag and updates the audio channel one or DMA control registers with a new pointer and a length value.
- `stop_audio_channel_one`: Clears a flag and writes to the audio and DMA control registers to stop audio channel one.
- `toggle_audio_channel_two_state`: Checks a global flag and, if set, clears it and updates the audio or DMA control registers to stop audio channel two or reset its length.
- `stop_audio_channel_two`: Clears a flag and writes to the audio and DMA control registers to stop audio channel two.
- `stop_audio_channel_three_2`: Checks a flag in a global variable and, if set, clears the flag and updates the audio channel three or DMA control registers with a new pointer and a length value.
- `stop_audio_channel_three`: Clears a flag and writes to the audio and DMA control registers to stop audio channel three.
- `init_audio_channel_handlers`: Clears four pairs of channel flags and installs the routine's address into a table of handler pointers.
- `update_entity_attribute_table`: Checks a flag-based table to find an available index, then updates a table of attribute values and a secondary table of calculated offsets or parameters.
- `randomise_thirteen_entry_pointers`: Fills thirteen table pointers, each computed from random selections of a table offset, then sets several state words and bytes to fixed starting values.
- `advance_timed_state_machine`: Counts a timer word down from 1rec, and, at fixed points of its count, adjusts a position word, sets an output byte and toggles a bit in a flag byte.
- `blit_sprite_records_with_wait`: Programs the blitter with masks, modulo values, and pointers, then loops over sprite records and blits each active one, waiting for a status bit to clear between operations.
- `update_sprite_slots_each_frame`: Loops over thirteen object slots and, for each slot whose active bit is set, updates its entry.
- `find_record_by_two_byte_key`: Iterates through a table of eight-byte records, comparing two bytes at a fixed offset within each record against two provided parameters, and returns the address of the first matching record.
- `blit_three_segment_sprite`: Programs the blitter for three sprite segments, choosing each segment's control values and pointer offsets from a table.
- `calculate_and_call_blitter_setup`: Calculates a new address by adding an offset to a pointer retrieved from a table, calls a routine, and then checks a local variable to decide whether to load a constant into a register and branch away.
- `program_blitter_sprite_mask`: Programs the blitter control, mask, modulo and size registers for a sprite part, taking its source address and anoffset from a record.
- `update_bcd_counter_and_timers`: Updates a packed-decimal counter and related state bytes when a timer byte reaches zero, while simultaneously incrementing or decrementing several other timer-based counters or position-related offsets within a table.
- `dispatch_setup_by_mode`: Dispatches on a mode argument to perform table or state resets, such as clearing or masking table entries and resetting certain memory or object counts.
- `wait_for_dff_bit`: Polls a specific bit in a hardware register until it becomes or remains certain.
- `load_library_functions`: Loads function pointers from a library into a global table by executing indirect library calls.
- `initialize_level_memory_layout`: Calculates and stores memory offsets and pointer tables for level-specific data or graphics buffers.
- `execute_conditional_logic_2`: Checks a global flag and either assigns a constant or executes a sequence of function calls with various arguments and a return value.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to -(a0).
- `update_object_properties`: Calls a routine to check a value, then uses that value to call another routine multiple times with different parameters and a final return value.
- `tiny_move_5`: Small routine of 1 instruction that copies d0 to d1.
- `initialise_entity_tables_from_record`: Reads a record selected by a stored index and computes a position and size from it.
- `set_copper_display_mode`: Writes to copper list registers and jump registers, and updates a global variable based on a flag check.
- `advance_lfsr_random_state`: Advances a 32-bit linear-feedback shift register eight steps, taking feedback from bits one and three, and stores a 1re-bit random value in a global word.
- `setup_register_and_branch`: Clears a register, pushes a register to the stack, and writes a constant value to a hardware register before branching to another location.
- `unpack_and_distribute_nibble_data`: Unpacks bit-level data from a source table into a destination table using bitwise shifts and masks, then copies or redistributes certain values into a secondary table.
- `pack_nibble_data_2`: Copies data from a source table into a destination table and packs two nibbles into a single byte in a global variable.
- `pack_nibble_data`: Copies and shifts data from a source table into a destination table, then packs two nibbles into a single byte in a global variable.
- `copy_three_pairs_set_bplcon`: Copies three paired entries into a structure, packs two nibble values into one byte and writes a control word.
- `initialize_system_state`: Clears certain memory locations, calls several routines to initialize tables and registers, and modifies a global variable and a handler pointer before returning a value.
- `randomise_grid_entries`: Waits until a status nibble is at most nine, then fills a grid of eighteen by ten entries with values chosen by random selections, calling a routine for each.
- `swap_handler_pointer_slot`: Saves the pointer at a fixed offset of a structure and installs the handler pointer there when the argument is one.
- `random_offset_plus_nibble`: Draws two random values and returns their sum plus the low nibble of a stored word.
- `reset_control_state_once`: Checks if a global value equals a certain constant, then resets it, calls another routine, and toggles bits in two flag bytes.
- `initialise_game_state_and_input`: Adds a stored value to a running total, sets default state words and bytes, and initializes a timer to 9000, then calls a series of setup routines while reading the keyboard serial and certain button/status registers.
- `copy_four_by_four_tiles`: Copies a four-by-four block of tile bytes from a source table into destination planes, taking the source offset from a stored index.
- `update_entity_position_and_velocity`: Adjusts two position bytes by table deltas chosen by an index, and adds a random offset to a position longword that it copies into a stored pair.
- `advance_double_buffer_index`: Writes three values derived from a table with an offset into a structure, sets a counter to four, and toggles a buffer index bit.
- `wait_port_bit_timeout`: Counts a value down while a status bit of a CIA port register is set, stopping at zero.
- `wait_for_flag_then_clear`: Waits until a flag byte becomes non-zero, then clears it.
- `offset_position_entries`: Adds a first argument to two stored longwords and a second argument to three other stored longwords, shifting a set of positions by those amounts.
- `update_entity_positions_recursively`: This recursive routine updates a set of position offsets or counters by adding a value retrieved from a table, while performing wrap-around checks.
- `scan_rows_for_tile_entries`: Loops over a range of rows, checking if a value is within a certain range, and updates pointers and counters or calls a routine if a bit is set.
- `match_code_to_table_index`: Maps a nine-bit code to an index using a table or hardcoded exceptions, and stores the index with a position word derived from a table entry.
- `read_joystick_direction_code`: Reads the joystick movement register and the fire-button status, then combines the directional bits into one of eight direction codes.
- `clamp_packed_table_range`: Clamps the packed nibble fields of a range of table words against a limit byte and stores the clamped values back into a second table.
- `update_timer_and_flags`: Decrements a global counter and updates other global flags or resets a counter to a fixed value if it reaches zero.
- `reset_seven_table_entries`: Sets seven table words to a maximum value and sets a counter to two.
- `unpack_word_run_lengths`: Decodes a run-length compressed block of sixteen-bit words, handling both literal copies and repeated value insertions.
- `set_edge_bits_in_rows`: Sets the lower five bits and upper three bits of two bytes at a calculated offset in a table for a specified number of rows.
- `reset_blits_and_timers`: Copies a pointer set into blitter registers, programs blitter registers with certain values, and resets a timer and certain flags.
- `blit_three_offset_instances`: Programs the blitter for a sprite and draws it three times at offsets derived from a stored value and an index.
- `reset_state_and_initialize_table`: Zeroes a block of state bytes, sets a state word to zero, and writes specific constant values (272, 288, 107, and 107) into a table using an offset derived from a state byte.
- `handle_entity_type_logic`: Checks a type-identifier or flag within a table and executes a series of conditional branches to update entity attributes, reset table entries, or modify state flags.
- `update_entity_state_and_timers_2`: Decrements internal counters and state-related timers, and updates entity attributes or position offsets based on conditional checks and lookup tables.
- `update_state_machine_logic`: Decrements a global counter and branches to different logic paths based on a state variable.
- `init_entity_with_random_placement`: Updates certain global bytes using parameters, calls a function or table to set new pointer-based offsets, and resets several state flags or bytes in a global structure.
- `update_state_on_match`: Checks if certain global bytes match provided parameters, then updates a global byte and a pointer-based value if a third parameter is non-zero.
- `update_game_state_recursively`: A recursive routine that checks a set of global flags and, if certain conditions are met, updates a global timer or counter, modifies a lookup table index, and updates several global state variables or buffers based on a random or calculated offset.
- `process_entity_type_logic`: Checks a specific entity ID against a set of conditional bounds and flags, then updates attribute or position tables, or triggers a secondary routine with new parameters.
- `update_entity_position_and_bounds`: Updates a global position or offset by adding or subtracting values from a lookup table based on a calculated index.
- `update_entity_state_and_timers`: Checks a flag-based condition and updates a state-related byte or a timer-like counter.
- `update_entity_visual_state`: Checks a flag-based table and coordinate bounds to determine if an entity requires a state or size update.
- `process_entity_type_logic_2`: Checks a flag-based value and executes conditional branches that update entity-related buffers, modify lookup table offsets, and recursively call other routines with new parameters.
- `update_entity_lifecycle_and_state`: Updates entity-related counters and state flags, potentially triggering recursive state updates or resetting certain entity parameters.
- `get_offset_from_difference`: Calculates the difference between two sets of parameters and uses the result to look up a value in a table.
- `copy_table_row_to_slot`: Copies three 16-bit words from a row of a lookup table into a buffer slot chosen by the first argument, with the row offset taken from the second argument scaled by eight.
- `search_word_and_lookup_byte`: Scans a 16-bit word list for a specific value, then returns a byte from a table using an offset derived from a second parameter and the result of a function call.
- `reset_entity_tracking_state`: Clears a block of memory, initializes a table with pointers, and resets several counters or state bytes.
- `update_cell_with_state_toggle`: Updates a word in a state block by masking its lower bits and applying a new value, stores a new value in a different word, toggles a parity byte, and recursively calls itself while toggling the parity byte back.
- `swap_two_plane_tile_blocks`: Swaps a 3-by-3 block of 16-bit words between two interleaved planes, stepping by fixed strides per row and column.
- `increment_attribute_and_update_offset`: Decrements a byte, then increments a value in a table or a secondary table if a certain condition is met.
- `shift_entity_slots_between_rows`: Copies two entity slots and their word and byte records from one row index to another, overwriting the source slots with a sentinel and calling routines to clamp packed table ranges and set state bytes.
- `set_state_byte_by_selector`: Stores the second argument into one of four state bytes chosen by the first argument, where selectors 0, 2, 4 and 6 map to different bytes.
- `update_entity_position_and_reorder_list`: Checks if a value at a specific index matches a provided parameter, then performs a recursive search through a table to shift or reorder index-based records.
- `decrement_count_and_rebalance_buckets`: Decrements a pending count, inserts a value into a bucketed table, then moves entries down the bucket chain and stores a marker in the top bucket.
- `remove_entity_from_linked_list`: Searches a table for a specific entity ID and, if found, removes it from a linked list or chain by updating pointers and clearing a sentinel value.
- `clear_buffer_and_call_library`: Zeroes a block of two hundred longwords and saves a value to a global pointer.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `allocate_record_and_register`: Allocates a record whose size comes from a stored count and fills its open-state fields.
- `copy_register_d0_to_d5`: Copies the contents of register d0 to register d5.
- `parse_string_to_token_list`: Parses a source string into individual tokens, handling quoted strings and skipping whitespace, and stores the resulting pointers into a destination table.
- `set_d0_to_max_and_branch`: Sets the d0 register to a maximum signed 16-bit value and performs a short branch to another location.
- `insert_string_at_end`: Finds the end of a destination string and writes a source string into that position for a limited count, ensuring a terminator is present.
- `copy_limited_string_with_pad`: Copies bytes from a source pointer to a destination pointer for a given count, stopping on a zero byte, and fills any remaining space with zero bytes.
- `initialize_window_structure`: Opens a file or resource, retrieves a pointer from a library, and if successful, populates a structure with a pointer and a value from a library-retrieved offset before closing the resource.
- `copy_register_data`: copies a longword from one data register to another.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: Checks the sign of two registers and negates them if necessary while updating a counter.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original d1 contents with the value left in d1 by the call.
- `recursive_call`: The routine calls itself without performing any other operation.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1, returning the result in a concatenated 64-bit value.
- `execute_optional_hook`: Checks a global pointer and calls an optional hook routine if the pointer is non-zero before proceeding to call another routine.
- `release_entries_and_buffers`: Releases the stored entry records and their table buffer in turn, runs an optional shutdown hook and frees any remaining buffers that are set.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: Closes a file handle using a DOS library call and updates a global variable with a status or zeroed pointer.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `execute_indirect_close`: Executes a function call via a pointer retrieved from a global table offset.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `execute_indirect_function_pointer`: loads a function pointer from a nested pointer or table and executes it.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `execute_indirect_function`: Loads a function pointer from a table or structure and executes it.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_2`: loads a function pointer from a nested pointer or table and executes it.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `execute_library_function`: Executes a library function using a pointer retrieved from a global library base and a specific offset.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `execute_indirect_library_function`: loads a function pointer from a table offset relative to a library base and executes it.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `execute_indirect_library_function_2`: loads a library base from a global variable and executes a function at a fixed offset relative to that base via a pointer-to-pointer dereference.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_library_function_3`: loads a library base from a global variable and executes a function at a fixed offset relative to that base.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_indirect_function`: Loads a function pointer from a table and executes it.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `execute_indirect_library_function_4`: loads a function pointer from a table or offset relative to a global library base and executes it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `execute_indirect_library_function_5`: Loads a library base from a global variable and executes a function at a fixed offset relative to that base via an indirect pointer.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
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
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.

### Disk-Validator

- `format_string_and_initialize_buffer`: Converts a numeric value to a string and stores it in a buffer, then initializes a memory range with a specific bitmask or constant value.

### SKID_ROW

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to load dos.library.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.

### 15

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### Diskfont.library

- `set_font_handle_if_valid`: Checks if a library function returns a non-zero value and, if so, stores a value from a register into a structure field before calling another routine.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: Attempts to open a library and, if successful, initializes a structure by setting a pointer to a new memory location and clearing a field.
- `process_font_buffer_list`: Iterates through a linked list of font buffers to remove or update entries based on a size check.
- `update_font_buffer_state`: Checks if a font buffer is empty or invalid, then deallocates memory and updates a linked list or pointer table.
- `set_bit_and_clear_register`: Sets a specific bit in a memory location and clears a register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `branch_on_bit_status`: Decrements a register and branches if a certain bit in a memory location is not set, otherwise jumps to another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_offset`: Calculates a signed offset value based on the difference between two 16-bit values and a bitwise comparison of two byte arrays.
- `update_font_character_metrics`: Iterates through a table of font character structures to find the entry with the highest metric value.
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
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `scan_string_for_null`: This library routine checks a string for a null terminator and increments a pointer if a certain byte is not zero.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `calculate_string_length`: Iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_string_bytes`: This library routine compares bytes from a string against a value, potentially returning a constant or zero.
- `check_string_bounds`: Moves a byte from a pointer, decrements a register, and branches or sets a register to a constant based on a comparison.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_font_byte`: This library routine loads a byte from a pointer and compares it against a value from the stack.
- `increment_pointer_and_branch`: This library routine increments a register and branches to another location.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward`: This library routine calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle`: This library routine checks the validity of a font handle by attempting two specific operations.
- `populate_font_buffer_from_list`: Iterates through a linked list of font structures to calculate string lengths and copy character data into a destination buffer.
- `iterate_and_copy_font_strings`: Iterates through a linked list of font-related structures, calculating string lengths and copying character data into a destination buffer while updating a remaining-length counter and a pointer-based metadata structure.
- `copy_font_string_to_buffer`: Calculates the length of a string and copies its characters into a destination buffer, while decrementing a remaining-length counter.
- `parse_font_metadata_and_load_glyphs_2`: This routine parses a font definition block to calculate string lengths and copies glyph or font metadata into a destination structure.
- `parse_font_metadata_and_load_glyphs`: Iterates through a font definition block to calculate string lengths and copy glyph or font metadata into a destination structure.
- `parse_font_header_and_load_glyphs`: Parses a font definition block by reading a header or string, then iterates through a list of glyph or font metadata to populate a destination structure with font properties and pointer offsets.
- `parse_font_string_buffer`: Reads a string or block of data from a buffer and copies or parses its contents into a destination structure while decrementing a remaining-length counter.
- `evaluate_register_sign`: This routine calls a routine using a pointer from a memory location, followed by two other routine calls, and returns the absolute value of a register if it is negative, otherwise it returns zero.
- `return_zero_constant`: The routine immediately returns a zero value without performing any other operations or accessing memory.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### Abyss.adf:4

- `copy_string_into_global_buffer`: Copies a counted string from the source pointer into a fixed global buffer and writes a terminator after it.
- `prepare_buffer_and_enable_dma`: This routine checks a register, enables DMA or certain hardware features, and calls a routine to prepare a buffer before clearing a register and returning.
- `update_interrupt_enable_state`: Saves the current value of the interrupt control register to a global variable, then writes a new vector address and a new control word that enables interrupts.
- `restore_saved_interrupt_state`: Writes a previously saved value to the interrupt control register and then re-enables interrupts by setting the enable bit.
- `check_global_bit`: Checks a specific bit in a global variable and enters an infinite loop if the bit is set.
- `close_library_resources`: This routine uses a pointer to call library functions, potentially closing a library or an old library, and checks a return value to call further functions.
- `tiny_move_rts`: Small routine of 2 instructions that copies 1 to d6, then returns.
- `initialise_pointer_table`: This routine builds a table of pointers by accumulating offsets from a source table, clears a range of memory, and toggles a flag and several state words.
- `update_audio_parameters_2`: Increments a global counter and, if a certain value is reached, resets it and updates an audio-related register or memory location using a value from a table, while also updating a pointer or index with a wrap-around check.
- `update_audio_parameters`: Updates audio-related registers or memory locations by adding or subtracting a value, applying a clamp to ensure the result stays within a certain range.
- `clamp_value_to_limit`: Checks if a value at a certain offset in a stack frame exceeds a limit and clamps it to that limit if necessary.
- `update_audio_register_with_offset`: This routine updates a register or a memory location with a value derived from a subtraction, ensuring the result does not drop below zero.
- `advance_position_with_wrap`: Sets a position based on a shifted and offset value from a register, resetting it to zero and incrementing a table index if a certain limit is reached, while wrapping that index.
- `update_channel_from_sequence_entry`: Loads the next sequence entry for an audio channel by copying a pointer and length from a source table and adding a stored offset.
- `open_intuition_then_loop`: Checks if a certain byte in a global variable matches a specific character, then opens the intuition library and enters an infinite loop calling a library function.

### Abyss.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop_2`: executes an infinite loop that prevents further instruction execution.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_5`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: This routine is an infinite loop that does nothing.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_counter`: Increments a global longword, checks if it reaches a certain value, and updates a global pointer or value based on that comparison.
