# Game 722 (gravity-force 2 v1.0 (1994)(bits)(pd)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00722). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 124 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 25 of 25 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 732 of 732 named (0 author symbols), 732 of 732 described (101 by rule, 531 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5672 of 5672 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 17867 of 17873 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 256897 |
| code | 65974 |
| data | 189397 |
| padding | 29492 |
| unused | 359424 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ask | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| Execute | 1 | 888 | 12 |
| LogMerge | 2 | 501 | 27 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| IconX | 2 | 404 | 8 |
| LoadWB | 12 | 536 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| d | 1 | 3 | 1 |
| GF2_D2M.DAT | 2 | 409 | 10 |
| GF.EXE | 7 | 7081 | 146 |
| GetPath | 2 | 159 | 5 |
| GF2_M.DAT | 4 | 32 | 1 |
| GF2_SPEC.DAT | 2 | 876 | 20 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 10 | 1375 | 109 |
| info.library | 25 | 2597 | 167 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 15 | 1040 | 76 |
| Gravity-Force 2 v1.0 (1994)(BiTS)(PD).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `check_bit_status` (GF.EXE): This routine checks if a specific bit is set in a value derived from a global variable and returns a status value. Kind: disk hardware/bit status check. Pass path: bmi.b an address.

## Functions


### Ask

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer.

### type

- `traverse_list_and_populate_buffer`: Traverses a linked list to find the maximum value in a specific field and then populates a data structure with a header and a sequence of incrementing values.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location offset from a pointer to a destination address, then executes a function pointer.

### assign

- `update_linked_list_and_copy_buffer`: Traverses a linked list to find the maximum value in a specific field, then updates a data structure with this value and populates a buffer using a loop.
- `copy_data_block_2`: The routine copies a block of sixteen words from a source address to a destination address and then calls another routine.

### endcli

- `find_max_and_initialize_buffer`: Traverses a linked list to find the maximum value in a specific field and then initializes a data buffer with a header and a sequence of incrementing values.
- `copy_data_block_3`: copies sixteen words from a source offset within a structure to a destination pointer and then calls another routine.

### echo

- `update_linked_list_buffer`: Traverses a linked list to find the maximum value in a specific field and then populates a data buffer with a header and subsequent entries.
- `copy_memory_block_and_call_2`: Copies sixteen bytes from a memory location to a destination pointed to by a stack value, then executes a function via a pointer.

### Execute

- `initialize_command_execution_environment`: Allocates and initializes a data structure for command processing by populating it with function pointers and clearing internal buffers.
- `process_command_state_machine`: Evaluates a command structure by executing various function pointers based on a status code.
- `process_input_buffer`: Iterates through a buffer of input characters, comparing them against specific boundary values and updating a data structure.
- `validate_string_length`: Checks a value returned by a routine against specific constants and verifies if a null-terminated string has a length of exactly one.
- `find_string_length`: Calculates the length of a null-terminated string and returns a value from a table if the length is exactly one.
- `execute_callback_pair`: extracts a pointer from an offset within a structure and calls two different routines via pointers found at specific offsets.
- `execute_callback_pair_2`: Extracts a pointer from a structure and calls two different routines via pointers found at specific offsets.
- `execute_offset_callbacks`: calls two different routines using pointers located at specific offsets within a provided structure.
- `store_registers_to_pointer`: the routine copies the contents of D1, D2, D3, D4, D5, D6, D7, A1, A2, A3, A4, A5 and A6 and the value from the stack into successive entries of the pointer passed in A0, and then returns.
- `normalize_param_and_copy_word`: the routine ensures the second argument is non-zero and then copies the first word of the structure pointed to by the first argument into a word located at an offset within that structure.
- `divide_fixed_point`: Performs a 32-bit fixed-point division using a normalization method.
- `normalize_fixed_point_divide`: the routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.

### LogMerge

- `process_log_merge`: A routine that performs a sequence of conditional calls, manipulates 64-bit values stored in global variables, and returns a status code.
- `run_fixed_helper_sequence`: Runs a fixed sequence of helper calls, with one extra call when a flag is set, then returns.
- `emit_entries_skipping_marked`: Walks a list of entries and, for each entry whose key is not a specific value, calls helpers to write its fields, with extra calls if an ampersand character is present.
- `accumulate_counts_for_two_passes`: Runs a helper pass that adds its count to a running total.
- `merge_two_entry_lists`: Walks two lists of entries together and merges them by comparing key fields and calling helpers for matched or unmatched entries.
- `copy_zero_terminated_string`: copies a zero-terminated string from the source to the destination, including the terminator.
- `write_negated_sum_checksum`: sums a run of 32-bit values, then stores the negated sum XORed with a fixed tag into the first field of a header.
- `verify_checksum_and_reset_records`: Recomputes a checksum of a block and, if the sum is non-zero, resets a run of records by writing fixed values into their fields.
- `xor_decode_words_backward`: Walks a buffer of 16-bit words from its end toward its start, XOR-combining each word with the word after it and a fixed key.
- `xor_fold_words_forward`: Walks a buffer of 16-bit words forward, replacing each word with the XOR of itself, the following word and a fixed key.
- `call_routine_five_times`: Calls another routine five times in succession.
- `call_twice_if_flag_set`: Invokes a routine once, and invokes it a second time only if a global flag is non-zero.
- `perform_recursive_call`: The routine performs a direct recursive call to itself without executing any other instructions before returning.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 20 into d0, then returns.
- `parse_file_path_tokens`: Analyzes a string to identify specific characters and patterns, setting a global flag based on the presence of certain characters at specific offsets.
- `call_shared_routine_once`: Makes a single call to another routine and returns.
- `copy_quoted_or_space_token`: Copies one token from a source string into a destination buffer.
- `search_buffer_for_characters`: Iterates through a buffer to check if any bytes match a set of target characters, returning a status value.
- `return_error_constant`: An unreachable routine that returns a constant value.
- `search_for_characters`: Iterates through a buffer to check if specific characters match any bytes within the data.
- `read_file_into_buffer`: Gets a file's size by locking and examining it, then reopens the file and reads its contents into a buffer sized from that length.
- `write_buffer_to_file`: Opens a file through the dos library, writes the given buffer to it, and closes the file.
- `print_null_terminated_string`: Iterates through a pointer to find the end of a null-terminated string and then calls a routine to output that string.
- `open_dos_library_and_streams`: Opens the dos library and fetches its standard input and output handles.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `allocate_into_free_slot`: Finds an empty slot in a fixed table of pointer pairs, records a requested size in it, and asks the exec library to allocate memory.
- `free_tracked_memory_blocks`: Walks the allocation table from the start and calls the exec FreeMem routine on each entry it reaches, stopping at the first empty entry.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_initialize_buffer_2`: Traverses a linked list to identify the maximum value in a specific field and then initializes a data buffer with a header and a sequence of incrementing values.
- `copy_data_and_call_routine`: Copies a sequence of words from a table into a destination buffer and then calls another routine.

### IconX

- `initialize_application_environment`: Initializes system libraries and checks for valid application state by querying global structures.
- `call_library_function`: Calls a function from a library using an offset provided in a register.
- `process_buffer_data`: The routine checks a global structure and, if certain conditions are met, allocates memory and iterates through a linked list to populate a buffer with values retrieved from another routine.
- `invoke_table_entry`: The routine invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `copy_bytes_with_termination`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `copy_c_string_2`: Copies a null-terminated byte string from a source pointer to a destination pointer.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.

### LoadWB

- `initialize_workbench_environment`: Initializes the application environment by retrieving system state, allocating memory, and parsing command-line arguments or directory strings.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_workbench_icon`: Processes a global variable by calling several other routines, performing a double XOR decode on a zero-terminated byte string via a pointer, then stores a result from another routine into a buffer obtained earlier.
- `compare_strings`: Compares two null-terminated strings byte by byte.
- `build_linked_list_from_table`: Iterates through a table of pointers, allocating a new node for each entry and linking them into a new list.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `compare_workbench_version`: Compares a string against a global constant to check for a specific version identifier and performs a string copy if a match is found.
- `write_constant_bytes`: Stores five specific byte values into a memory location starting at an offset from a pointer held in a register.
- `compare_strings_2`: Compares two null-terminated strings byte by byte until a mismatch or a null terminator is encountered.
- `empty_routine_5`: The routine contains four instructions and performs no observable operation; it ends with a return.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer`: Stores a byte from a data register into a memory location pointed to by a register.

### Disk-Validator

- `update_display_buffer`: Converts a numeric value into a character string and writes it into a specific offset within a memory buffer.

### d

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### GF2_D2M.DAT

- `build_sorted_directory_index`: Opens a directory and iterates through its entries to build a table of names paired with packed four-character keys.
- `populate_string_table`: Iterates through a data source to construct a formatted table of strings.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `execute_indirect_calls`: This routine loads values from a table and performs several indirect function calls using offsets from a pointer.
- `store_register_value`: This routine consists of a single instruction that stores the contents of a register into a global variable.
- `open_read_close_file_chunk`: Opens a file, performs several indirect function calls using offsets from a pointer, and closes the file.
- `initialize_string_table`: Clears a memory block and populates it with a series of formatted strings starting with a specific prefix and incrementing numeric suffixes.
- `initialize_string_table_2`: Constructs a table of strings by populating a memory buffer with a repeating pattern of characters and numeric values.
- `initialize_string_table_3`: Constructs a table of formatted strings by iterating through a loop to populate memory with specific character sequences.

### GF.EXE

- `initialize_system_environment`: Initializes global data structures and populates a table with string identifiers.
- `check_bit_status`: This routine checks if a specific bit is set in a value derived from a global variable and returns a status value.
- `handle_key_into_text_buffer`: Clears a flag, then examines a key code passed in a register.
- `clear_word_table`: Clears thirty-two consecutive 16-bit words in memory to zero and returns.
- `classify_control_character`: Examines a character code passed in a register.
- `find_matching_string_entry`: Walks a table of zero-terminated names, comparing each with a buffer.
- `forward_to_shared_routine`: Makes one call to a shared routine and returns, doing no other work of its own.
- `clear_flag_and_call_pointer`: Calls one routine, clears a global word, calls another routine, and then calls through a stored function pointer.
- `initialize_hardware_and_clear_memory`: Resets various global game state variables, clears large blocks of memory, and performs hardware initialization by touching color, copper, and DMA registers.
- `derive_default_totals_record`: Fills three default values in a small record when they are zero, derives a total from them and stores a value about two-thirds of that total.
- `tiny_asl`: Small routine of 1 instruction that shifts d0 left by 1.
- `store_fixed_values`: Stores two specific byte values into memory locations derived from a pointer.
- `sort_records_by_key`: Builds a table of records from two arrays and sorts them with an exchange sort.
- `format_five_digit_decimal`: Converts a number up to 99999 into decimal digits in a text buffer.
- `format_four_digit_decimal`: Writes a value into a text buffer as four decimal digits, capping it at 9999 and padding with zeros.
- `copy_indexed_twelve_byte_entry`: Copies a twelve-byte entry from a table, selected by an index, into two destination areas of six bytes each.
- `validate_and_sanitize_string`: Validates a series of configuration parameters against specific hardcoded values and performs a memory shift to ensure a string buffer does not start with an ampersand.
- `shift_buffer_past_ampersand`: Checks if the first byte of a buffer is an ampersand and, if so, shifts the buffer left by one byte; this process is repeated twice.
- `reset_audio_and_display_state`: Clears several blocks of game variables and then resets the hardware for sound and display.
- `store_fixed_values_2`: This routine stores two specific byte values into memory locations derived from a pointer.
- `build_symbol_lookup_table`: Builds a table of 16-bit symbol values from a list by subtracting a fixed offset from each and ending with a sentinel value.
- `skip_zero_terminated_strings`: Advances a pointer past a specific number of zero-terminated strings, where the count is derived from a table index and a multiplier.
- `skip_strings_by_table_index`: Advances a pointer past a number of zero-terminated strings, with the count taken from a stored index multiplied by another stored value.
- `update_fire_button_palette`: Writes three palette colour values to fixed hardware addresses, choosing between two sets according to the fire-button state read from the CIA port register.
- `reset_game_variables_block`: Clears two counters and a block of sixty-four words of game variables, copies one stored value into another, calls another routine and then clears a further flag.
- `wait_for_input_state`: Polls input and status flags in a loop until specific bit patterns in a global variable and other conditions are met.
- `wait_for_input_port_match`: Repeatedly calls three routines and reads two state bits of the input port until their combination matches the expected pattern, then returns.
- `compute_screen_offsets_from_value`: Derives several screen position values from one input by replicating its low nibble and adding fixed offsets.
- `draw_image_records_from_list`: Waits for the blitter, then walks a list of image records and draws each one into the display buffer.
- `track_stick_position_while_held`: Starts a position value from an input and loops while the input port reports the controls as held, nudging the value up or down according to the directional bits it reads.
- `fill_copper_pattern_table`: Fills a table of 127 entries with a repeating pattern, placing a special entry at one index.
- `find_closest_image_record`: Iterates through a list of image records to find the one with the minimum distance to a target coordinate.
- `dispatch_table_entry_by_type`: Dispatches on a type code stored in a record.
- `increment_global_byte`: Increments a global byte in a loop until it wraps to zero and the low bit is clear.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `increment_global_byte_2`: Increments a global byte repeatedly until it reaches zero and satisfies a bitwise parity check.
- `spin_until_counter_wraps`: Increments a byte in a loop until it wraps to zero and the low bit is clear.
- `wait_for_matching_counter_value`: Spins until a stored byte matches the value passed in the data register, then continues while its low bit is set.
- `spin_until_byte_nonzero`: Spins while a stored byte is zero, then repeats while its low bit is clear.
- `wait_for_counter_match`: Spins until a value in a register matches a global variable, or until a global variable matches a value in a register and its low bit is cleared.
- `update_entity_state_and_graphics`: Updates entity positions, animation frames, and physics by decrementing timers and checking boundary collisions.
- `clear_conditional_word_blocks`: Clears one of two blocks of 22 words depending on two flag tests, setting a sentinel in one slot.
- `update_animation_countdown_state`: Counts down an animation timer and reloads it from one of a few values when it expires, updating a state byte along the way.
- `init_object_record_fields`: Initialises an object record by storing the register arguments into its fields.
- `call_two_routines_in_sequence`: Makes two calls to other routines in sequence and returns.
- `test_bounding_box_overlap`: Checks whether a record's coordinates overlap a bounding box with a margin of seven units.
- `process_keyboard_input_and_state`: Reads a character from the keyboard serial register and updates various game state flags, including toggling pause and sound, or modifying cursor and command settings based on specific key codes.
- `draw_sprite_row_blitter`: Counts down two timers and, when they expire, selects a source image address and starts a blitter copy of it into the display.
- `stub_return_only`: Contains only a return, so there is no behaviour to recover from it.
- `clamp_sprite_position`: Clamps two position values to a screen range and updates counters that step them.
- `compute_clipped_sprite_coordinates`: Scales two coordinates by a factor, clamps them to a fixed limit and sets their low bits.
- `process_active_entry_list`: Walks a table of ten three-word entries and, for each active entry, calls two routines.
- `adjust_field_when_in_bounds`: Checks whether one rectangle lies within a bounds range and, if so, subtracts a stored amount from a 32-bit field of the second record.
- `add_field_when_in_bounds`: Applies the same bounds test as its neighbouring routine and adds a stored amount to the 32-bit field when the test passes.
- `advance_sprite_animation_frames`: Walks ten animation records, decrementing each countdown and advancing its frame when it reaches zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into -(a4).
- `update_table_index_and_pointer`: Increments a counter and updates several global pointers and indices using arithmetic operations and bitwise shifts.
- `update_state_table_pointer`: Updates a state index and calculates a new memory address based on a table lookup.
- `update_linked_list_state`: Traverses a linked data structure using bitwise checks to determine the next node.
- `update_frame_index_and_pointer`: Increments a local index and updates a pointer through a series of table lookups and linked list traversals.
- `advance_frame_and_program_blitter`: Increments a frame counter and updates several global indices and pointers through table lookups and linked list traversals.
- `fill_sprite_record_from_table`: Looks up two values by index in a table and writes them into a sprite record as adjusted coordinates, with a fixed offset applied.
- `fill_sprite_record_if_valid`: Looks up coordinate values for an index and writes them into a record when the index is in range, returning zero.
- `update_entity_physics_and_blitter_2`: Updates an entity's position and velocity using clamped bounds and lookup tables, then prepares and executes a blitter operation using calculated coordinates.
- `modify_coordinate_vectors`: Updates two 32-bit values by either bit-shifting them, negating them, or adding a scaled value retrieved from a table based on a control byte.
- `update_counter_values`: Increments or decrements two memory locations based on comparisons of values at specific offsets.
- `write_constant_on_match`: Checks a byte at an offset from a pointer and writes a constant value to a memory location if the byte matches a specific value.
- `scan_until_test_passes`: Loops over a set of entries, calling a test routine for each, and returns as soon as the test reports a hit.
- `update_object_physics`: Updates object position and velocity using arithmetic operations and bounds checking.
- `update_entity_physics_and_blitter`: Updates an entity's position and velocity using clamped bounds and lookup tables, then prepares and executes a blitter operation using calculated source pointers from a global table.
- `clamp_bounds_on_flag`: When a flag is set and a coordinate is negative, computes a clamped position from two limits, stores it into the output pair, and clears the source entry.
- `subtract_stored_deltas_if_clear`: Subtracts stored position deltas from two offset pairs when their enable flags are clear, and preserves saved values across the call.
- `lookup_distance_falloff`: Computes a squared distance between a point and a stored reference, takes a value from a lookup table when the distance is small enough, and returns a scaled result.
- `copy_and_mirror_row_pairs`: Copies source words into two destination buffers and writes a derived value, shifted and offset, alongside each copy.
- `select_position_and_write_glyphs`: Chooses one of several screen positions by comparing a stored coordinate against thresholds, then writes a sequence of character codes and attribute bytes into a buffer.
- `reset_audio_channels_and_dma`: Clears the audio channel state, selects volumes from two flags, and writes the DMA control value to enable the channels.
- `fetch_next_audio_entry`: Reads the next three-word entry from a sound list, stores its fields and advances the list pointer.
- `bounce_on_wall_contact`: Checks two objects against boundary tests and, when a contact code matches one of a set of values, reverses the vertical or horizontal velocity.
- `update_collision_counters_by_type`: Swaps two saved coordinate pairs, checks an object type code, and if it matches specific values, increments various counters and stats while updating a stored state.
- `compute_tile_cell_mask`: Computes the address of a tile cell from a row and column index, reads two words from it and combines them into a mask.
- `process_blit_mask_data`: Performs bitwise AND operations on a series of word-sized elements within a data structure.
- `initialize_blit_records`: Initializes and populates blit record structures with positional data, masks, and specific bitplane values.
- `init_blit_record_defaults`: Stores a position and a fixed set of blit values into a record, setting two of its fields to thirty-six, and returns the value passed in.
- `init_blit_record_with_mask`: Stores a position combined with a fixed mask and several blit values into a record, setting two of its fields to thirty-six, and returns the value passed in.
- `blit_entry_list_to_display`: Walks a list of ten image entries and, for each one that has a value, programs the blitter's source, destination and size registers to copy it.
- `start_blit_request`: Waits until the blitter is free, then fills a blit request record with a control word, source and destination pointers and a size from its arguments.
- `blit_second_entry_list`: Walks a list of image entries one per pass and, for those without a value, programs the blitter to copy them into the display.
- `clamp_scaled_table_offset`: Computes a value from a table entry and two inputs, scales it by sixteen, clamps it to a fixed range and adds the offset passed in.
- `update_positions_and_run_routines`: Updates two position values from stored inputs when their flags are clear, then calls ten routines in turn.
- `try_advance_object_position`: Checks whether an object is clear of a bound and has room to move, then advances its position by three and calls another routine.
- `try_advance_second_object`: Performs the same position check and advance as its neighbouring routine, using the second object's fields.
- `process_pending_action_flags`: Checks two specific global flags; if set, it clears them and repeatedly calls a routine to attempt the associated action.
- `apply_collision_velocity_damping`: When a collision is detected, it reduces an object's counter by four and shrinks its velocity components by one eighth.
- `process_player_input_frame`: Reads the fire button and direction state from the CIAA port, updates the player's velocity and fire state, and calls routines to move and act on them.
- `stub_empty_routine_2`: Contains only a return instruction, so calling it has no effect.
- `stub_empty_routine_3`: Contains only a return instruction, so calling it has no effect.
- `pack_input_bits_into_byte`: Packs bits from an input word into a state byte, using the low two bits for the first two inputs and setting higher bits from the fire state.
- `unpack_input_state_word`: Unpacks a state byte into an input word, setting bits for the first two inputs and for the matching fire state.
- `read_mouse_and_joystick_ports`: Reads the mouse and joystick port movement registers, converts the movement into direction values, and updates the fire and position state.
- `load_and_parse_asset_file`: Opens and reads a file to check for a specific header, then calculates a scaling factor based on file metadata.
- `store_palette_colour_entry`: Stores a 12-bit colour value and derives two further colour entries from it by shifting and masking.
- `update_tracked_pointer`: Iterates through a table until it finds an empty or invalid entry, then calls a function via a pointer and updates the table with the result.
- `free_all_tracked_blocks`: Walks the table of tracked memory blocks and frees each block through the exec library, skipping entries that are already empty.
- `free_tracked_block_by_key`: Searches the tracked memory table for a given key, marks the matching entry as released and frees its block through the exec library.
- `fetch_input_state`: Reads a value from a memory location and returns it through a register.
- `cleanup_system_resources`: Disposes of graphics views, disables the blitter, and frees all tracked memory blocks before unloading a segment and closing a library.
- `forbid_and_reply_message`: When a flag is set, it forbids task switching and replies to a message, then returns zero.
- `interpolate_line_points`: Computes midpoint coordinates for a pair of points and steps through interpolated positions between them for drawing.
- `set_clip_bounds_and_draw`: Sets a clipping rectangle expanded by four units beyond the given corners, then draws the requested shape within it, calling two routines to do the work.
- `draw_three_pass_layers`: Runs a drawing routine over three layers, stepping the buffer pointer by a fixed amount after each pass, and repeats until a counter wraps to zero.
- `draw_four_pass_layers`: Runs the same pass loop as its neighbouring routine, but over four layers, stepping the buffer pointer by a fixed amount each time.
- `start_blitter_copy_operation`: Works out a blitter source offset from two coordinates, waits for the blitter, then writes the modulus, pointer, first-word mask and size registers and starts the operation.
- `draw_blitter_line_octant`: Computes the absolute differences between two points and selects one of eight line directions from them.
- `format_numbered_label`: Builds a text label by placing a digit character first and then a source string, and copies the finished label into a destination buffer.
- `process_input_and_blitter_loop`: Copies a null-terminated string to a new location, performs a series of blitter operations on memory blocks, and enters a loop that waits for specific joystick input bits while checking a character in a buffer.
- `parse_keyboard_input_buffer`: Reads characters from the keyboard serial register and processes them into a buffer.
- `tiny_moveq`: Small routine of 1 instruction that loads 7 into d2.
- `update_state_for_type_codes`: Stores a value into state bytes for three type codes and returns true for two of them, and for the third only under a condition.
- `clamp_input_position_on_fire_2`: Checks the fire and direction bits of the input port and, when a movement is blocked, sets a limit value of 199.
- `execute_blitter_and_parse_string`: Performs multiple blitter operations by configuring hardware parameters and then processes a string to locate a specific character.
- `configure_blitter_parameters`: Sets various blitter control and dimension registers based on calculated values and waits for the blitter to finish.
- `parse_string_for_character`: Iterates through a string to find a specific character match while skipping certain prefixes and special characters.
- `clamp_input_position_on_fire`: Checks the fire and direction bits of the input port and, when a movement is blocked, sets a limit value of 199.
- `tick_frame_counter_and_toggle`: Increments a frame byte and counts down a timer.
- `absolute_product_swapped_halves`: Takes the absolute values of two inputs, multiplies the high word of one by the low word of the other, and returns the product with the second input's halves swapped.
- `check_horizontal_window_draw`: Checks whether a horizontal distance lies within a window running from just below zero to 106 units.
- `check_vertical_window_draw`: Applies the same window test as its neighbouring routine on the other axis.
- `build_address_table_entries`: Fills a five-entry table with addresses spaced 10752 bytes apart, then fills a block of entries starting from 384 and copies each to a companion word.
- `build_row_address_table_and_init_palette`: Fills a three-entry table with addresses spaced 42 bytes apart, then fills a block of words starting from a specific offset by copying each to a companion word.
- `initialize_sprite_position_table`: Stores a sequence of preset coordinate pairs into a table and then copies entries in a loop over the whole table.
- `compute_row_address_pair`: Clamps a row index between fixed bounds and computes two addresses for that row.
- `pack_tile_attribute_value`: Combines a 16-bit value and a flag into a 32-bit attribute word using a specific offset, which changes based on a global condition and the destination address.
- `pack_tile_attribute_pair`: Packs a 16-bit value and a flag into a 32-bit attribute word with a fixed offset, using a second offset when a special condition holds.
- `copy_or_clear_data_block`: Copies fifteen words from a source table to a destination or clears fifteen words to zero based on a condition.
- `clear_fifteen_long_words`: Clears fifteen consecutive 32-bit entries starting after the address passed in.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `store_masked_port_bits`: Masks a status word with a fixed bit pattern and stores the result in a global variable.
- `decode_joystick_direction_bits`: Decodes direction bits read from the two joystick ports into direction flags, cancelling out opposing directions.
- `update_object_coordinates`: Calculates a scaled offset based on input values and applies a bitwise reduction to two consecutive memory locations pointed to by a register.
- `blit_sprite_frame_at_position`: Computes a clipped sprite position from a stored origin, waits for the blitter and programs its registers to draw the frame in a single blit operation.
- `load_asset_file`: Opens a file, copies its contents into a buffer, and parses metadata to set various global pointers and values.
- `calculate_lookup_value`: Determines a value based on the signs of two inputs, compares them, and performs a table lookup using a calculated index.
- `process_timed_entry_list`: Walks a list of twelve-byte entries, skipping empty ones, and for each active entry checks a flag pair before calling a routine.
- `update_linked_list_entities`: Iterates through a linked list of data structures to update their positions and manage their existence.
- `test_bitmap_cell_occupied`: Computes a byte address and bit within a bitmap from a position, and checks that cell and two neighbouring bytes for a set bit.
- `find_matching_entry_position`: Computes two candidate positions from table lookups and walks a linked list looking for an entry that matches them, returning the match.

### GetPath

- `setup_gf2_assign_and_copy`: Reads a command-line argument into a global buffer, ensures it ends with a directory separator, and then executes a series of DOS commands to assign a device name and copy a startup file to a new directory.
- `write_file_returning_status`: Opens a file through the dos library and, when it opened, writes the buffer and closes the file, returning one.
- `write_zero_terminated_text`: Advances to the end of a zero-terminated text string and then calls the dos write routine.
- `open_console_window_for_output`: Opens the dos library and then a console window named by its text, keeping the handle for output.
- `close_console_and_library`: Closes the console handle if it was opened, then closes the dos library.

### GF2_M.DAT

- `fill_globals_with_constant`: Writes a single register value into a long list of global variables in one pass and returns.

### GF2_SPEC.DAT

- `manage_lookup_table_entries`: Searches a linked list or table for a specific key and appends a new record if the key is not found.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 152 into d0, then merges 152 into d0, then merges 152 into d0.
- `format_report_string`: Copies a substring from a source into a destination buffer and updates several pointers and global variables.
- `format_percentage_gain_text`: Formats a percentage figure as two decimal digits and writes it into a text buffer, but only when a file signature check passes.
- `print_separator_line_with_flags`: Writes a line made of two dashes and a bracketed label through a stored output routine, adding a flag digit for each of two optional settings.
- `write_times_phrase_text`: Writes a short phrase ending in 'times.' into a text buffer by copying characters from a source table.
- `format_elapsed_time_text`: Converts a frame count, taken at fifty frames per second, into hours and minutes text with singular or plural wording.
- `convert_integer_to_string`: Converts an integer into a decimal string representation and stores the characters in a buffer.
- `clear_record_and_mark_end`: Clears a 120-byte buffer and writes a terminator value into a word that the record refers to.
- `write_checksum_tag_into_header`: Sums a sequence of values in a table and stores the negated total, XORed with a fixed tag, into a header word.
- `xor_decode_words_via_pointer`: Walks a buffer of 16-bit words backward from an end pointer, XOR-folding each word with the next one and a fixed key.
- `xor_fold_words_via_pointer`: Walks a buffer of 16-bit words forward from a pointer argument, replacing each word with the XOR of itself, the following word and a fixed key.
- `find_or_add_table_key`: Searches a table of records for a specific key; if the key is not found, it appends a new record to the table and increments the record count.
- `copy_and_expand_data_block`: This routine copies a block of data from a source pointer into a destination buffer, performing zero-padding and expanding the data structure through nested loops.
- `blit_tile_buffer_expansion`: Initializes a destination buffer with zeros and copies data from a source pointer using a stride-based row-by-row approach.
- `copy_rows_with_stride`: Copies a block of words row by row from one buffer into another, moves the pointers back by a fixed stride, and copies a second block.
- `wait_for_blitter_idle`: Spins while the blitter busy bit of the DMA status register is set, then returns.
- `decompress_data_stream`: Reads a data stream containing length-prefixed blocks and expands them into a destination buffer.
- `fill_block_with_byte`: Fills a block of memory with a repeated byte value, aligning the start to a four-byte boundary first and then writing four bytes at a time.
- `check_and_initialize_global`: Calls a function through a pointer to check a value, then either sets a global variable to a constant in a loop or updates a global variable and calls two other functions.

### mathtrans.library

- `return_empty_routine_3`: This routine is empty and performs no operations.
- `store_register_to_global`: Stores the contents of a register into a global variable.
- `manage_library_state`: As library code, it updates a status byte and manages a linked list of structures.
- `set_bit_in_byte`: Sets the fourth bit of a byte located at an offset from a pointer.
- `increment_reference_count`: As library code, it increments a counter in a structure and calls another routine if the counter reaches one.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call_2`: The library code decrements a short counter stored in a structure and, if the counter reaches zero and a specific flag bit is set, calls another routine.
- `generate_bitmask_loop`: Performs a repetitive operation to construct a bitmask based on an input value.
- `calculate_bitfield_mask`: As library code, it performs a loop to generate a bitmask based on an input value.
- `adjust_bitfield_value`: The routine takes two input values and a global variable and repeatedly modifies the first value by setting a bit selected by the second value, then adjusts a running difference using a doubling accumulator and a shifting value, and returns a combined result of the modified first value and the high word of the second value when a termination condition on the difference is met.
- `mask_library_result`: As library code, it calls a function through a pointer and applies a bitwise mask to the return value.
- `call_jump_table_entries`: Calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `execute_conditional_callbacks_2_2`: Library code that executes a series of function pointers from a table based on the values of two registers and performs bitwise masking on a result.
- `execute_conditional_callbacks_2`: The library code executes a series of function pointers from a table based on the values of two registers and performs bitwise masking on a returned value.
- `execute_conditional_callbacks`: As library code, it executes a series of function pointers from a table based on the values of two registers.
- `branch_and_mask_result_2`: Library code that performs conditional branching based on register values to call different function pointers and returns a masked result.
- `branch_and_mask_result`: The library code performs conditional branching based on register values to call different function pointers and returns a masked result.
- `call_and_mask_offset`: As library code, it calls a function at a specific offset from a pointer and masks the result.
- `call_twice_and_mask`: As library code, it calls a function at a specific offset twice and masks the result of a subsequent call.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_indirect_via_pointer`: The library code calls a routine through a pointer located at a specific offset from a global variable.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_indirect_via_pointer_2`: The library code calls a routine through a pointer located at a different specific offset from a global variable.
- `perform_bit_shifting_loop`: Library code that executes a loop performing bit-shifting and decrementing based on a value derived from a register.
- `execute_loop_with_branching_2`: Library code that executes a loop performing bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `process_complex_arithmetic`: This library code performs bitwise shifts and arithmetic operations on values derived from a byte, then uses the result to iterate through a table.
- `normalize_bit_shift_parameters`: Validates a byte value against specific range constraints and calculates a bit-shift amount.
- `normalize_bit_shifted_value`: Takes a byte and performs arithmetic shifts based on its value to produce a normalized result.
- `perform_complex_math_loop`: This library routine performs a series of bitwise shifts and arithmetic operations on values derived from a global variable and a pointer.
- `process_arithmetic_shifts`: This library code performs bitwise shifts and arithmetic operations on values derived from a byte, then uses the result to iterate through a table and modify values via a pointer.
- `perform_complex_arithmetic_2`: As part of library code, this routine performs a loop of arithmetic operations involving bit shifts and additions using values from a table and registers.
- `process_arithmetic_data_stream`: Iterates through a memory buffer to perform a series of bit-shifted additions and subtractions on register values.
- `perform_complex_arithmetic`: Library code that performs a loop of arithmetic operations involving bit shifts and additions using values from a table and registers.
- `process_complex_math_2`: Library code that performs arithmetic operations including additions, subtractions, and bitwise shifts on values derived from registers and a table.
- `return_immediately_3`: This library routine is empty.
- `call_own_entry_recursively`: Calls the routine at its own entry point and then returns, so the code appears to recurse into itself.
- `execute_empty_returns`: Library code that performs two immediate returns.
- `process_signed_value_2`: The library code performs bitwise rotations and repeated multiplications on a value based on its sign and magnitude.
- `process_signed_value`: This library routine performs bitwise rotations and repeated multiplications on a value based on its sign and magnitude.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `call_routines_conditionally`: Library code checks a byte value and, if it meets specific criteria, performs a sequence of indirect calls with modified arguments.
- `frag_move_roxr_bra`: Short code fragment that copies an address to d7, then rotates d7 right through the extend flag by 1, then jumps to another point in the code.
- `return_from_subroutine_3`: This routine is an empty function that performs a single return instruction.
- `frag_move_tst_bpl_moveq_bra`: Short code fragment that copies the top of the stack to d6, then tests d6, then branches if the last result was not negative, then loads 0 into d7, then jumps to another point in the code.
- `frag_moveq_lsr_ori_movem_rts`: Short code fragment that loads 255 into d7, then shifts d7 right by 1, then merges 2 into the condition codes, then restores saved registers from the stack, then returns.
- `frag_move_lea_tst_rts`: Short code fragment that copies an address to d7, then loads the address of the memory at an offset from a7 into a7, then tests d7, then returns.
- `process_conditional_data`: Library code that performs several indirect calls and conditional logic based on the contents of a register.
- `call_multiple_routines`: Library code that calls another routine four times in succession before returning.
- `iterate_table_with_accumulator`: The library code walks through a table pointed to by a pointer and repeatedly adds or subtracts the current table entry to an accumulator.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `execute_complex_math_logic`: Library code that performs indirect calls and executes loops involving bit-shifting, subtraction, and repeated multiplication.
- `return_from_subroutine_5`: This routine is empty and performs no operations before returning to the caller.
- `process_complex_math`: Library code that performs a series of indirect calls and executes loops involving bit-shifting, subtraction, and repeated multiplication.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `process_signed_value_4`: Library code that performs conditional logic and bitwise manipulations on a value before calling another routine.
- `process_signed_value_3`: Library code performs conditional logic based on a register value, involving bitwise shifts and multiple indirect calls.
- `frag_add_beq_eori_asr_subi_swap_rol_rts`: Short code fragment that adds d7 to d7, then branches if the last result was zero, then exclusive-ors 128 into d7, then shifts d7 right by 1, then subtracts 130 from d7, then swaps the halves of d7, then rotates d7 left by 7, then returns.
- `return_immediately_7`: This library routine performs no functional operations and returns control to its caller.
- `return_immediately_6`: This routine performs no functional operations and returns control to its caller.
- `frag_andi_tst_beq_moveq_bra`: Short code fragment that masks d7 with 65279, then tests d7, then branches if the last result was zero, then tests d7, then loads 0 into d7, then jumps to another point in the code.
- `stub_empty_routine`: Contains only a return, so calling it has no effect.
- `frag_lsl_moveq_roxr_ori_bra`: Short code fragment that shifts d7 left by 8, then loads 255 into d7, then rotates d7 right through the extend flag by 1, then merges 2 into the condition codes, then jumps to another point in the code.
- `return_from_subroutine_4`: This routine is empty and performs only a return instruction.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `pass_through_register`: The library code returns the value originally held in a register.
- `return_d0_register`: Library code that returns the value held in the D0 register.
- `invoke_callback_and_return_3`: Library code that receives a pointer in a register and a value in another, calls the routine pointed to by the pointer, and returns the original value.
- `call_other_routine_2`: Library code calls another routine via a pointer and returns the original value in a data register.
- `invoke_callback_and_return_4`: This library code calls a routine through a pointer in a register and returns the original value from a register.
- `execute_callback_and_return_3`: Calls a subroutine via a pointer provided in a register and returns the original value held in a data register.
- `execute_callback_routine`: Executes a subroutine via a pointer provided in a register and returns the original value held in a data register.
- `store_register_via_pointer`: The routine invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `invoke_callback_wrapper`: Library code that calls another routine via a pointer and returns the original value from a register.
- `execute_callback_and_return`: Calls a routine via a pointer provided in a register and returns the original value from a data register unchanged.
- `execute_indirect_callback_2`: Performs an indirect call to a routine retrieved from a pointer and returns the original value held in a data register.
- `execute_indirect_callback`: Performs an indirect call to a routine via a pointer and returns the original value held in a data register.
- `recurse_with_stack_slot_2`: Library code that receives a pointer and a value, calls a routine through the pointer, and then calls another routine passing two parameters and a pointer to a stack location.
- `execute_callback_and_return_2`: Calls a routine via a pointer and returns the original value held in a data register.
- `call_indirect_routine`: This library code calls a routine through a pointer held in a register and returns the value that was in a data register.
- `invoke_callback_and_return_2`: This library code calls a routine via a pointer in A0 and returns the original value from D0.
- `return_immediately_8`: Library code that is empty and returns immediately.
- `call_indirect_with_offset_2`: This library code calls a routine at an address calculated by adding an offset to a pointer stored in a global variable.
- `call_indirect_with_offset_3`: This library code performs an indirect call to a routine using an offset applied to a global pointer.
- `invoke_callback_and_return_concat_2`: This library code calls a routine via a global pointer and returns the concatenation of the resulting D0 and D1 registers.
- `invoke_callback_and_return_concat_3`: This library code executes a routine through a pointer in a global variable and returns a concatenated 64-bit value from D0 and D1.
- `call_indirect_with_offset_4`: This library code calls a routine using an address derived from a global pointer and an offset in A0.
- `call_indirect_with_offset_5`: This library code invokes a routine at a location determined by adding an offset to a global variable pointer.
- `invoke_callback_and_return_concat_4`: This library code calls a routine via a global pointer and returns the combined contents of D0 and D1.
- `invoke_callback_and_return_concat_5`: This library code performs an indirect call using a global pointer and returns the concatenated D0 and D1 registers.
- `invoke_callback_and_return_concat_6`: This library code invokes a routine via a pointer stored in a global variable and returns the concatenated 32-bit values from D0 and D1.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_error_code`: The library code returns a constant value of negative one.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_status`: This library code sets a specific bit in a byte within a structure or performs a linked list manipulation if a condition is met.
- `set_status_bit`: Library code that performs a bitwise OR operation to set a specific bit in a byte located at an offset from a pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_table_pointers`: The library code copies two 32-bit values from an address derived from a register into two global variables and calls another routine.
- `copy_two_pointers_from_table`: This library routine copies two 32-bit values from an address derived from a register into two global variables.
- `execute_self_call`: Library code that performs an infinite loop by calling itself.
- `return_from_subroutine`: This routine performs a single jump to a return address, effectively acting as a wrapper for a subroutine return.
- `invoke_library_handler`: This library routine calls another routine and then returns.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `trigger_icon_hierarchy_update`: This library routine invokes a single routine to recursively traverse and update a tree-like structure of icon nodes.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `increment_and_recurse_parameter`: This library routine calls another routine with the same first argument and an increased second argument before returning.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `dispatch_handler_pointer`: This library routine retrieves a function pointer from a data structure and calls it.
- `invoke_handler_pointer`: This library routine retrieves a pointer from a data structure and calls the routine located at that address.
- `call_other_routine_3`: The routine calls another routine and then returns.
- `call_and_return`: This library routine calls another routine and then immediately returns.
- `call_other_routine_4`: The library routine calls another routine and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `process_nested_calls`: Library code that executes a series of conditional calls to other routines based on their return values.
- `update_icon_hierarchy`: This library code calls another routine to obtain a pointer, performs conditional logic, copies a block of data to a new location, and updates several fields within a structure.
- `copy_and_assign_structure`: This library code copies a block of data from one location to another and updates several fields in a target structure.
- `return_zero_stub`: Returns zero with no other work.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_node_hierarchy_2`: Recursively traverses and updates a tree-like structure of icon nodes by calling a callback function.
- `update_icon_node_hierarchy`: Recursively traverses and updates a tree-like structure of icon nodes by calling a callback function.
- `update_icon_nodes_2`: This library routine executes a sequence of function calls and updates specific offsets within a structure based on the returned values.
- `execute_callback_sequence`: This library routine executes a series of function calls from a list, updates a pointer, and manages a counter or a global variable.
- `call_and_return_zero`: This library routine calls another routine and then returns zero.
- `frag_move_movem_unlk_rts`: Short code fragment that copies d4 to d0, then copies several registers to or from memory, then removes the stack frame held in a6, then returns.
- `process_icon_metadata_callbacks`: Iterates through various metadata fields and linked tables within an icon structure to execute callback functions for each entry.
- `process_icon_metadata`: Iterates through icon metadata structures and linked tables to execute callback functions for each entry.
- `iterate_icon_table_2`: This library routine iterates through various linked structures and tables, invoking function pointers and updating internal counters or offsets.
- `iterate_icon_table`: This library routine walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list`: This library code iterates through a linked list structure and performs an action on each node.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `process_icon_list_structure`: As part of library code, this routine traverses a structure and performs conditional logic and bitwise manipulations before calling other library functions.
- `return_zero_value_3`: This routine returns a zero value to the caller.
- `return_zero_value_2`: This routine is an empty function that returns zero to the caller.
- `store_data_in_table`: This library routine decrements a counter through a pointer and uses the resulting value to store two parameters into a table at a calculated offset.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `manage_icon_node_logic`: This library routine performs a series of conditional calls to other routines, passing various parameters and offsets to manage data structures based on returned values.
- `return_void_2`: This library routine is empty and performs no operations before returning.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `incremental_recursive_call`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `search_list_for_character`: Library code that traverses a null-terminated list of pointers to find a specific character within a string and returns a pointer to the location following an equals sign.
- `find_character_match`: This library routine iterates through a list of pointers and compares memory to find the position of a specific character.
- `find_character_match_2`: This library routine traverses a null-terminated list of pointers to find a specific character within a string.
- `search_sequence_recursively`: This library code performs a recursive search through a data structure to verify a specific sequence of values.
- `validate_recursive_sequence`: Performs a recursive search through a data structure to verify if a specific sequence of values matches a target condition.
- `perform_recursive_search`: Library code that performs a recursive calculation involving incrementing values and conditional subtractions.
- `perform_recursive_calculation`: This library code performs a recursive calculation involving incrementing values and conditional subtractions.
- `search_recursive_sequence`: As part of library code, this routine performs a recursive search through a data structure to verify if a specific sequence of values matches a target condition.
- `parse_numeric_string_and_copy`: Converts a decimal substring into an integer and copies the resulting value and subsequent string data into a destination buffer.
- `parse_numeric_string_3`: Library code that converts a decimal string into an integer and performs memory comparisons.
- `parse_numeric_string_2`: This library routine converts a string of decimal digits into an integer value.
- `parse_numeric_string`: Library code that iterates through a null-terminated string to calculate a value based on decimal digits and performs memory comparisons.
- `clamp_and_offset_value`: This library code checks if a value is within a specific range, returning negative one if it is outside the range or an offset value if it is inside.
- `return_all_ones`: This routine returns a value consisting of all bits set to one.
- `execute_indirect_call_11`: This library code performs an indirect call to a function pointer located at an offset from a global address.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `fill_list_with_value`: This library code traverses a structure and writes a constant value to specific offsets within it.
- `return_zero`: Library code that returns zero.
- `compare_and_reset`: Library code that calls another routine and returns zero if the returned value does not match a passed parameter.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `validate_return_value`: This library routine calls another routine and returns zero if the result does not match a provided parameter.
- `return_void`: Library code that performs an empty return.
- `calculate_large_division`: This library code performs complex division and modulo operations on 32-bit values using bitwise shifts and rotations.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry_2`: This library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_address`: The routine stores a single byte from a register into a memory location pointed to by another register.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_character_in_string_2`: Library code that iterates through a string to find the first occurrence of a specific character.
- `return_zero_2`: This library routine is empty and simply returns zero to the caller.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_null_terminated_string_2`: This library code calculates the length of a string and then copies the contents of one string to another.
- `compare_memory_buffers`: This library routine compares two memory buffers byte by byte for a specified length and returns one if they match or zero otherwise.
- `return_constant_one_3`: This library code returns the integer value one.
- `scan_null_terminated_string`: Library code that iterates through a string until a null terminator is found.
- `call_another_routine_2`: This library code calls another routine and returns.
- `copy_string_with_limit`: The library code copies a string from one location to another up to a specified number of bytes.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `validate_icon_list`: This library routine iterates through a linked structure and returns zero if a specific value at an offset is zero, otherwise it returns one.
- `validate_list_node`: This library routine iterates through a structure and returns zero if a specific value at an offset is zero, otherwise it returns one.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `calculate_complex_structure`: This library code performs a series of nested calls and arithmetic operations to populate a field in a structure.
- `return_negative_one`: Library code that returns a value consisting of all bits set to one.
- `update_icon_nodes`: This library routine iterates through a linked sequence of structures, updating specific offsets within each node based on values returned from subsequent routines.
- `iterate_and_update_nodes`: This library routine loops through a sequence of calls, updating specific offsets in a structure based on returned values.
- `traverse_and_update_linked_structure`: Iterates through a linked sequence of structures, updating specific offsets within each node based on values returned from subsequent routine calls.
- `return_negative_one_2`: The library code returns a value representing negative one.

### info.library

- `return_empty_routine_4`: This routine is empty and performs no operations before returning.
- `store_library_state`: The routine stores the incoming register values into a global variable and a table, and writes a pointer through an offset in a structure, then stores that pointer into another global variable.
- `invoke_callback_routine`: This library routine calls a function through a pointer provided in a register and returns the original value from another register.
- `call_decrement_and_check`: Library code that calls another routine to decrement a short word and check a bit flag within a structure.
- `update_structure_flags_2`: This library routine checks a value at a specific offset within a structure and sets a specific bit in a byte if that value is non-zero.
- `return_immediately`: This routine is an empty function that performs no operations and returns immediately to its caller.
- `reset_library_state_recursive`: This library code calls itself indefinitely, resulting in infinite recursion, before finally calling a routine that resets the library state.
- `clear_flag_and_increment_counter_2`: The routine clears a specific bit in a byte field of a structure pointed to by its argument and increments a 16-bit counter field within the same structure.
- `update_structure_flags`: Library code that checks a value at an offset within a structure and sets a specific bit in a byte if the value is non-zero.
- `call_library_functions_with_offset_args`: This library routine retrieves a library base from a global variable and calls two library functions using offsets and calculated arguments.
- `decrement_and_check_flag`: Library code that decrements a short word at a specific offset within a structure and checks a bit in a byte at another offset.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `recursively_call_pointers`: The routine checks several global pointers and arguments, then calls itself recursively if a pointer is non-zero.
- `initialize_library_internal_structure`: Allocates a memory block and populates a global library structure with a series of calculated offsets and pointers.
- `initialize_info_library_struct`: Initializes a structure or table by calculating and storing several offsets or pointers derived from a base address.
- `execute_conditional_logic`: This library routine performs a series of conditional checks by calling another routine with a pointer and zero, then updates several global variables based on the results before calling another routine.
- `execute_conditional_callbacks_3`: Library code that invokes a function pointer multiple times with a global variable and calls another routine based on the results.
- `execute_conditional_callback`: Library code that calls a function pointer with a global variable and executes another routine based on the result.
- `check_routine_result`: Library code that calls another routine with a pointer and a zero value, then returns a boolean based on whether the result was non-zero.
- `return_constant_one_2`: Library code that returns the value one.
- `fetch_conditional_offset_longword`: Retrieves a longword from a location determined by checking a global structure's status flags and following either a direct offset or a pointer-based offset.
- `fetch_nested_longword`: Library code that retrieves a longword from a location determined by traversing multiple pointers within a structure.
- `fetch_nested_value`: Library code that traverses multiple levels of pointers within a structure to retrieve a longword value.
- `get_indirect_longword`: Library code that returns a longword from an address stored within a pointer found in a structure.
- `get_indirect_value`: Library code that retrieves a longword from a location determined by dereferencing a pointer within a provided argument.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `fetch_conditional_offset`: This library routine retrieves a value from a location determined by checking a global structure's status and following a pointer or offset.
- `check_library_status_and_set_flag`: Checks a boolean flag and a function return value to decide whether to update a global flag and retrieve a value from a nested pointer.
- `update_info_state`: As part of library code, this routine updates various fields in a structure, including a counter and pointers, based on the results of other library routines.
- `update_info_structure_state`: Updates a structure by incrementing a counter field and synchronizing several pointer fields based on the results of conditional library calls.
- `update_structure_fields`: Library code that updates several fields within a structure, including a counter and pointers, based on the results of other library calls.
- `update_structure_state_2`: This library routine updates several fields within a structure, including a counter and pointers, based on the results of other library calls.
- `update_structure_state`: Library code that updates various fields in a structure, including a counter and pointers, based on the results of other library routines.
- `copy_library_strings_and_data`: This library routine copies a string using a library function and then copies certain dwords or even dwords from one structure to another.
- `validate_and_execute_sequence`: This library routine initializes several global variables to zero, performs parameter validation, and executes a multi-stage sequence of calls involving a local stack buffer.
- `execute_library_sequence`: This library routine calls a library function via a vector, then proceeds through a series of conditional checks and function calls to update global variables.
- `execute_stateful_logic`: This library routine manages a complex sequence of calls that update internal state and use a function pointer to signal success or failure based on various conditional results.
- `process_callback_chain`: This library routine executes a sequence of calls involving a function pointer and a global variable, then uses a checksum-style operation to validate data before invoking a final callback.
- `execute_indirect_logic`: This library code executes several routine calls and uses a function pointer to signal success or failure based on the results of conditional checks.
- `update_library_state_2`: This library code executes several routine calls and uses a function pointer to signal success or failure based on the results of conditional checks and global variable states.
- `update_library_state`: This library code updates fields within a structure and manages state based on conditional offsets.
- `validate_library_parameters`: As part of library code, this routine checks a global variable, calls a library function via a global base, and executes a parameter-validated routine.
- `call_library_offset_offset_84`: This library code calls a routine located at a specific offset from a base pointer stored in a global variable.
- `execute_indirect_function`: Fetches a function pointer from a memory location and executes it.
- `execute_indirect_function_2`: Fetches a function pointer from a table and executes it.
- `execute_indirect_function_3`: Fetches a function pointer from a memory location using an offset and executes it.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `execute_indirect_call`: Library code that performs an indirect call to a function pointer retrieved from a global table.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `execute_indirect_call_2`: Library code that calls a function located at an offset within a global table.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `execute_indirect_call_3`: Library code that executes a subroutine via a pointer stored in a global table.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `execute_indirect_call_4`: Library code that performs an indirect jump to a function address found in a global table.
- `execute_indirect_call_5`: Library code that retrieves a function pointer from a global table and executes it.
- `execute_indirect_call_10`: Library code that executes a subroutine using a pointer from a global table.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `invoke_callback_from_table`: This library code calls another routine obtained by indexing a global table of function pointers with a fixed offset and then invoking it.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `execute_library_function_pointer`: Fetches a function pointer from a global table and executes it.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `invoke_callback_from_table_2`: The routine retrieves a function pointer from a global table using a fixed offset and executes it.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `execute_indirect_call_6`: Library code that calls a subroutine through a pointer located in a global table.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `execute_indirect_call_7`: Library code that executes a function by dereferencing a pointer from a global table.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `execute_indirect_call_8`: Library code that performs an indirect call to a subroutine stored in a global table.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `execute_indirect_call_9`: Library code that calls a function through a pointer retrieved from a global table.
- `invoke_library_functions`: Library code that calls two different library functions using a base address from a global variable and a pointer passed via a stack-allocated structure.
- `dispatch_command_type`: Library code that performs different actions based on a type field in a structure, including calling other routines or updating a global variable.
- `return_constant_integer`: This routine returns the integer value one to its caller.
- `toggle_bitmask_flag`: Library code that selects a value based on a type field and performs a bitwise XOR operation against a global variable.
- `store_register_to_global_2`: Library code that stores a register value into a global variable offset by a fixed amount and then calls another routine using a pointer from a structure.
- `validate_and_format_info_string`: Checks the type and validity of a string and its associated attributes within a data structure.
- `process_string_data`: Library code that performs several operations involving strings and pointers, including conditional checks and calls to other routines.
- `process_string_and_attributes`: This library routine copies a string, calls a library function, and performs several conditional checks involving pointer offsets and string copies, potentially updating a local variable with a value from a structure.
- `retry_function_calls_2`: This library routine attempts to execute a series of function calls in sequence, returning success if any call does not return a specific error value.
- `retry_function_calls`: Library code that repeatedly calls a function pointer up to three times, returning one if any call succeeds and zero if all return a specific error value.
- `retry_routine_on_error`: Library code that calls a routine and returns one if it succeeds or zero if it fails twice with a specific error value.
- `execute_until_error`: Library code that repeatedly calls a routine until it returns a specific error value.
- `call_other_and_return_one`: The library routine calls another routine and then returns one.
- `initialize_info_library_tables`: Library code that initializes internal tables by repeatedly calling another routine with various constant arguments and a pointer derived from the input parameter.
- `execute_list_of_library_calls`: This library routine calls another routine multiple times with a fixed first parameter and varying other constant arguments.
- `initialize_display_palette_entries`: The routine calls another routine or a sequence of calls with a fixed set of arguments, passing the same first parameter each time and varying the remaining constants.
- `register_info_table_entries`: The library code calls another routine nine times with a fixed set of six integer arguments each time, passing the same first parameter each time and varying the remaining constants.
- `set_info_table_entries`: The library code calls another routine nine times with a fixed set of six integer arguments each time, passing the unable to return a value and performs no other operations.
- `process_complex_structure`: Library code that branches into different execution paths to process linked structures based on a type byte and pointer comparisons.
- `process_linked_structures`: This library routine initializes linked structures and conditionally executes further library calls based on the contents of a pointer.
- `execute_multiple_library_calls`: The library code calls a library function or routine multiple times using a pointer from a structure.
- `update_linked_list`: This library routine traverses a list of pointers, calling a routine for each element and updating the list structure with the results.
- `update_linked_list_elements`: Traverses a list of pointers stored in a global variable, calling another routine for each element and updating the list structure.
- `update_structure_list`: This library routine iterates through a list, performing library calls to update a structure and store a value at a specific offset.
- `iterate_and_update_list`: Library code that traverses a list of pointers, updating each entry with a returned value and calling another routine for each element.
- `dispatch_callback_conditional`: Library code that compares values within a structure and executes a series of indirect calls or updates a structure member based on the result.
- `update_pointer_from_structure`: Library code that calls a routine with a pointer from a structure, then updates that structure member with a value from another pointer.
- `dereference_and_dispatch`: Library code that performs conditional checks on structure members and either executes indirect calls or updates a structure member with a value from a dereferenced pointer.
- `update_pointer_dereference`: Library code that calls a routine and then replaces a structure member with a value retrieved by dereferencing a pointer.
- `modify_global_state_and_execute`: Library code that sets a global variable, checks a condition, calls several library functions, and updates a global pointer.
- `execute_complex_callback`: Library code that sets a flag, performs conditional logic based on structure members, and executes multiple callbacks while updating a structure member with a value from a pointer or a dereferenced pointer.
- `update_structure_pointer`: Library code that updates a structure member using a value retrieved from a dereferenced pointer within the structure.
- `execute_sequential_callbacks`: This library routine calls three different routines using pointers or offsets provided in a structure.
- `extract_linked_list_data`: As part of library code, this routine iterates through a linked list to extract data from nested pointers and stores calculated values into a destination table.
- `process_linked_list_data_table_3`: Library code that iterates through a linked list to extract data from nested pointers and stores calculated values into a destination table.
- `allocate_and_populate_table`: Library code that calculates the size of a linked list, allocates memory, and populates a new table with data from the list.
- `populate_data_table_2`: Library code that iterates through a linked list to calculate values and stores them into a destination table, while also performing indirect calls and updating structure members.
- `process_linked_list_data_table_2`: Iterates through a linked list to extract data from nested pointers and stores calculated values into a destination table.
- `populate_data_table`: This library routine iterates through a linked list, calculates a value based on a string length, and stores it in a table.
- `process_linked_list_data_table`: Iterates through a linked list to extract data from nested pointers and stores calculated values into a destination table.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_callback_sequence`: Executes a series of library operations based on a value read from a data structure.
- `empty_routine_4`: This library code routine is empty.
- `validate_and_dispatch_info_type`: Validates a series of resource pointers retrieved from a data structure and then executes a specific sequence of function calls based on a type byte.
- `dispatch_by_type_code`: Library code that executes a sequence of function calls based on a type byte retrieved from a structure.
- `place_info_items_by_type`: Runs a fixed number of placement helpers chosen by a type byte, with two to five calls depending on the type.
- `frag_movem_unlk_rts`: Short code fragment that copies several registers to or from memory, then removes the stack frame held in a6, then returns.
- `perform_formatted_output`: Library code that performs a formatted output operation using data from a global structure and a provided parameter.
- `execute_formatted_output`: This library routine performs a formatted output operation and returns a value, ensuring a result of zero if the operation fails.
- `return_from_library_call`: This routine is an empty function that performs no operations and returns immediately.
- `verify_multiple_callbacks`: This library routine checks a series of function pointers and returns one if all specified callbacks succeed.
- `check_three_callbacks_succeed`: Library routine that calls a function pointer held in a register up to three times, stopping at the first call that returns minus one.
- `check_two_callbacks_succeed`: Library routine that calls a function pointer held in a register up to twice, stopping at the first call that returns minus one.
- `check_callback_result_not_failure`: Library routine that makes one call through a function pointer and reports whether its result differs from minus one.
- `return_constant_one`: Library routine that does no other work and returns the constant one.
- `iterate_entries_with_bit_mask`: Library routine that repeatedly fetches the next entry from a structure, recursively processes each entry it finds, and returns zero if any processing fails.
- `return_constant_zero`: Library routine that does no other work and returns zero.
- `dispatch_request_by_code`: Library routine that branches on a numeric code in its argument.
- `forward_call_without_arguments`: Library stub that calls another routine with no arguments and returns nothing.
- `call_library_then_return_zero`: Library stub that calls a routine through a library base function at a fixed offset, then calls a further routine, and returns zero.
- `execute_two_calls`: Library code that calls another routine twice in a row and returns one.
- `execute_multiple_callbacks`: Library code that invokes a set of callback routines in sequence on its argument.
- `format_protection_flag_strings`: Library routine that reads a status value from a global structure, masks its low bits, and calls a formatting routine four times to write the labels ARCHIVED, DELETEABLE, READABLE and WRITEABLE. Each label receives one bit of the masked value.
- `format_negated_flag_label`: Library code that builds a short flag label from a two-part template ending in the word NOT, choosing one of two strings from its flag argument.
- `format_padded_label_pair`: Library routine that builds an argument list on the stack from a stored value, a fixed template with a comma, and two string placeholders.
- `measure_cstring_length`: Library routine that walks a null-terminated string from its argument.
- `find_string_terminator_index`: Library routine that scans forward from a starting index, offset by a base value, until it finds a zero byte.
- `copy_cstring_to_buffer`: Library routine that copies a null-terminated byte string from a source pointer to a destination pointer, one byte at a time.
- `copy_counted_bytes`: Library routine that copies a given number of bytes from a source buffer to a destination buffer, one byte at a time.
- `copy_memory_block`: Library routine that copies a block of bytes from a source pointer to a destination pointer using a loop that runs at least once.
- `compare_cstrings_for_equality`: Library routine that compares two null-terminated strings character by character.
- `compare_nul_terminated_strings`: Library routine that steps through two null-terminated strings together and stops at the end of either one.
- `count_cstring_bytes_with_terminator`: Library routine that counts the bytes of a null-terminated string from its argument.
- `execute_library_operation_sequence`: Library routine that first calls another routine on its last argument and keeps the result.
- `format_into_local_buffer`: Library routine that fills a local buffer by calling another routine with a global pointer and its last argument.
- `copy_length_prefixed_string`: Library routine that reads a length byte from its source, copies that many following bytes to the destination, and appends a zero terminator.
- `copy_range_and_terminate`: Library routine that copies a run of bytes from a source offset into a destination.
- `initialize_linked_structures_and_check`: Library routine that fills two structures from its arguments and links the second to the first, setting fixed default values in the second.
- `execute_library_function`: Library code that calls a library function through a pointer stored in a field of its structure argument, then returns one.
- `execute_library_calls`: Library routine that makes three calls through a library base pointer stored in a field of its structure argument.
- `call_exec_raw_format`: Library routine that forwards to the operating system's formatted-output routine RawDoFmt.
- `store_byte_at_pointer`: Library routine that writes the low byte of its data register to the memory location addressed by its pointer register.
- `call_indirect_routine_via_table`: Library code that calls one routine indirectly through a pointer stored in a global data table and then returns without further work.
- `call_indirect_via_table`: Library code that calls one routine indirectly through a second pointer stored in the global data table, then returns without further work.

### version.library

- `return_zero_library_entry`: Library entry point that does nothing except return zero.
- `store_library_pointers`: The routine writes two 32-bit values from registers into a structure pointed to by a register, storing one value at an offset and another at a different offset.
- `increment_counter_and_clear_flag`: Library code that increments a 16-bit counter in a structure and clears one bit of a flag byte in the same structure.
- `decrement_counter_and_call`: Library code that decrements a 16-bit counter in a structure.
- `complete_or_mark_pending_request`: Library code that either calls two library functions through a base pointer in the structure and returns a stored value, or sets a flag bit in the structure and returns zero.
- `call_library_pair_return_value`: Library code that reads a stored 32-bit value from a structure, calls two library functions through a base pointer, and returns the value it read.
- `return_zero_status_stub`: Library routine that does nothing and returns zero as a status.

### diskfont.library

- `set_field_after_library_call`: Library routine that calls a library function through a base pointer and, if it returns a non-zero handle, stores its argument at a fixed offset inside that handle and makes a second library call.
- `check_and_set_font_handle`: The routine is library code that calls a routine via a table entry using the library base.
- `open_two_libraries_init_list`: Library routine that opens the dos and graphics libraries by name through the operating system.
- `close_library_and_free_memory`: Library routine that closes a library and frees a memory block through operating system calls.
- `release_unused_list_entries`: Library routine that walks a linked list in its structure and unlinks each entry whose count is not negative, releasing it through another routine.
- `release_structure_when_idle`: Library routine that, when no users or entries remain, closes two libraries, frees its structure and returns a stored value.
- `set_flag_bit_return_zero`: Library routine that sets the top bit of a flag byte in its structure and returns zero.
- `return_immediately_stub`: Library routine that does nothing and returns immediately.
- `clear_flag_and_increment_counter`: Library code that clears the top bit of a flag byte in a structure and increments a 16-bit counter in the same structure.
- `decrement_counter_then_release`: Library code that decrements a 16-bit counter in its structure.
- `return_zero_no_op`: Library routine that does nothing and returns zero.
- `scan_font_directory_into_buffer`: Library routine that fills a buffer with font name entries.
- `copy_font_entries_to_buffer`: Library routine that walks a linked list of font entries, copying each entry's name into a buffer and recording its fields in a fixed-size table.
- `copy_font_names_from_string`: Library routine that walks a null-terminated text and a linked list of font entries, copying each name into a buffer and recording its fields in a table.
- `scan_fonts_directory_with_path`: Library routine that copies a name string into a buffer and walks a linked list of font entries, recording each entry's fields in a table.
- `open_font_files_and_record`: Library routine that walks a linked list of font entries and, for entries whose name carries the font suffix, opens the matching file and records its details in a table.
- `record_font_entries_from_text`: Library routine that copies name text into a buffer and records font entries from a linked list, scanning a directory for further entries when a second flag is set.
- `parse_font_directory_entries`: Iterates through directory entries to identify specific font files based on file attributes and metadata.
- `scan_directory_with_ex_next`: Library routine that repeatedly reads directory entries with the operating system's examine-next call, copying names and table fields for those that match.
- `frag_moveq_movem_unlk_rts`: Short code fragment that loads 0 into d0, then copies several registers to or from memory, then removes the stack frame held in a6, then returns.
- `compute_bitmask_distance_score`: Library routine that computes a signed score from two 16-bit fields and two flag bytes of its arguments, subtracting a table value for each of seven bit positions where the flags differ.
- `compute_bitmask_difference_score`: Library routine that starts from a value built from a 16-bit field and subtracts a table entry for each of seven flag bits where its two argument bytes differ.
- `subtract_flag_table_values`: Library routine that walks seven flag positions, subtracting a table value from its running total for each position whose flag bits differ.
- `subtract_alternate_flag_table_values`: Library routine that walks seven flag positions, subtracting table values from a running total for each position whose flag differs.
- `return_constant_value`: Library routine that returns a fixed negative constant without reading its arguments or registers.
- `open_font_and_register_entry`: Library routine that copies a font name and size from its argument and opens the font through the graphics library.
- `register_best_fonts_from_directory`: Library routine that locks the FONTS directory and repeatedly picks the best-scoring matching entry from a table.
- `scan_font_file_entries`: Library routine that opens a font file, reads its header and entries, and repeatedly selects the best-scoring entry.
- `select_best_font_entries`: Library routine that repeatedly picks the highest-scoring entry from a table of font entries and registers each matching one in the font list.
- `load_and_register_font_entries`: Library routine that repeatedly selects the best-scoring font entry from a table and registers each match in the font list with a list-append and a font-add call.
- `unload_font_entries_and_return`: Library routine that repeatedly selects font entries from a table and registers each match.
- `close_font_entries_and_release`: Library routine that walks font entries, closing each opened font and freeing its memory.
- `load_font_file_into_table`: Library routine that opens a font file named by its argument, counts its entries, and allocates a table with a header and entries for each matching font.
- `load_disk_font_table`: Library code that counts entries in a font file, allocates a table with a header and a count, and copies font metric and control bytes from the input into the table for each entry.
- `free_memory_from_stack_argument`: Library routine that reads a pointer and a size from the stack and passes them to the operating system's free-memory routine.
- `forbid_multitasking`: Library stub that calls the operating system's Forbid routine to stop task switching, through a pointer stored in a global base.
- `permit_multitasking`: Library stub that calls the operating system's Permit routine to resume task switching, through a pointer stored in a global base.
- `allocate_memory_through_exec`: Library stub that calls the operating system memory allocator AllocMem through a pointer stored in a global base.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: Library routine that copies bytes from a source pointer to a destination pointer, including the terminating zero byte.
- `seek_string_end_then_forward`: Library routine that moves past the null-terminated text at its first argument to the terminator, then passes that position and its second argument to another routine.
- `seek_string_terminator_then_call`: Library routine that advances past a zero-terminated string to its terminating zero byte, then calls another routine with that pointer.
- `count_string_characters`: Library routine that counts the characters in a zero-terminated string and returns the number before the terminator.
- `count_null_terminated_bytes`: Library code that counts bytes from a pointer up to and including the terminating zero byte, and returns that total.
- `compare_strings_ordered`: Library routine that compares two zero-terminated strings character by character.
- `compare_strings_zero_on_end`: Library routine that compares two zero-terminated strings and returns zero as soon as the first string ends.
- `compare_strings_until_mismatch`: Library routine that compares two zero-terminated strings and returns zero if they are identical.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_character_in_string`: Library routine that scans a zero-terminated string for a given character.
- `find_character_after_position`: Library routine that scans forward from a pointer for a given character, starting with the byte after the pointer.
- `find_next_character_occurrence`: Library routine that moves forward from a pointer to the next occurrence of a given character.
- `find_last_character_in_string`: Library routine that finds the last occurrence of a character in a zero-terminated string by measuring the string and scanning backwards from its end.
- `find_character_backwards_bounded`: Library routine that scans backwards from a pointer for a given character, stopping at a lower bound.
- `find_character_backwards_checked`: Library routine that steps backwards from a pointer looking for a given character, returning null once it passes a lower bound.
- `skip_past_path_separator`: Library routine that looks for a slash in a name and, failing that, a colon.
- `increment_returned_value`: Library routine that returns its register argument increased by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: Library routine that calls one other routine and then returns without other work.
- `call_other_routine`: Library routine that calls one other routine and then returns.
- `call_another_routine`: Library routine that acts as a thin wrapper: it immediately calls another routine and then returns without further work.

### Gravity-Force 2 v1.0 (1994)(BiTS)(PD).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_operation`: The routine is empty and performs no operations.
