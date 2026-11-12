# Game 708 (Defender): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00708). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 22 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 513 of 513 named (0 author symbols), 513 of 513 described (77 by rule, 138 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 335 of 335 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12604 of 12604 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 271552 |
| code | 46228 |
| data | 89657 |
| padding | 5747 |
| unused | 487936 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Defender.adf:Defender | 6 | 5017 | 50 |
| More | 19 | 2321 | 79 |
| Assign | 2 | 435 | 7 |
| EndCLI | 2 | 182 | 3 |
| Echo | 2 | 241 | 10 |
| LoadWB | 12 | 536 | 30 |
| Ram-Handler | 2 | 10 | 1 |
| Defender.adf:Apc-Trainer | 1 | 99 | 3 |
| ramdrive.device | 3 | 436 | 37 |
| mathtrans.library | 18 | 848 | 106 |
| icon.library | 10 | 1421 | 110 |
| diskfont.library | 15 | 1040 | 76 |
| Defender.adf:bootblock | 1 | 18 | 1 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EB8) A7=an address (A7)=an address, slid to exec.library -an address): 490 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- Defender/Defender.adf:Apc-Trainer: TetraPack 2.2 (xfd) stream found by ancient scan (10952 -> 65536 bytes) but only 1 of 13 sampled windows are in reference RAM: not used

## Cracktros (kept, not protection)

- Defender.adf:Apc-Trainer

## Copy protection found

- `execute_unreached_code` (More): Executes a sequence of calls including a routine with a global variable, another routine, and then a function pointer held in an uninitialized register. Kind: indirect jump/control flow obfuscation. Pass path: the valid address loaded into A3.

## Functions


### Defender.adf:Defender

- `start_program_with_task_check`: Program entry that finds the running task and checks a field of it.
- `read_write_scores_file`: Opens the s:defender_scores file through the dos library and reads or writes its contents, XORing each stored 32-bit word with a constant key.
- `update_audio_volumes_by_state`: When a state value equals one, clears a table of entries, computes a scaled value from a 16-bit field and writes it to the audio channel volume registers, then resets counters.
- `accumulate_nibble_table_into_globals`: Walks a table of bytes in groups of four, accumulating the high and low nibbles of each byte into two global counters and calling a helper whenever a counter is nonzero.
- `draw_glyph_indexes_to_bitplane`: Reads 16-bit glyph index entries from a global table until a sentinel value, and for each entry writes eight bytes of glyph data into a bitplane buffer.
- `update_audio_channel_records`: Updates audio channel records for four channels from stored state values, returning early when a playback flag is set and its mode equals three.
- `select_nonzero_channel_entry`: Walks a table of four 16-bit entries from the top down, storing each nonzero entry into a global variable.
- `decrement_countdown_update_position`: Decrements a countdown global and, when it reaches zero, updates a position value from a stored table and writes a combined result to another global.
- `blit_descriptor_list_entries`: Walks a global table of descriptors until a sentinel value, and for each entry waits for the blitter to become idle before programming its source and destination registers.
- `configure_blitter_copy_block`: Waits for the blitter to become idle, then writes control words and pointers into a blitter control block to configure a scanline copy.
- `update_active_entry_positions`: Walks a global list of entries until a sentinel value, skipping inactive ones, and for each active entry with a type code from one to thirteen computes a screen position from its stored coordinates.
- `set_scroll_mask_from_position`: Computes a scroll mask from the low bits of a position global and stores it in another global.
- `update_table_entries_by_flag`: Walks a table of 16-bit entries and, for each empty entry, checks a global flag bit.
- `update_horizontal_position_from_flags`: Adjusts a horizontal position global by two steps when flag bits are set, clamping at a fixed limit, and calls two helper routines when a counter is below one.
- `decode_control_word_flags`: Reads a 16-bit word from a structure at a fixed offset and sets a mode value and flags in globals according to its low bits and bits eight and nine.
- `combine_key_bits_into_control`: Tests several bytes of a keypress table and ORs a bit into a control word for each byte that is set, then updates a control byte from another table entry.
- `update_entry_counter_flags`: Increments a counter in a per-entry table selected by an index.
- `draw_masked_rows_to_bitplanes`: Iterates over a small set of rows, computing a screen offset for each, and writes masked bits from a source table into the bitplane buffers.
- `compute_wrapped_position_difference`: Computes a wrapped horizontal difference between two position globals, adjusting it by 2048 when it falls outside a range, then scans a table until a sentinel value.
- `blit_counter_block_below_six`: Waits for the blitter to become idle, then when a counter is below six programs blitter registers for a fixed block and copies data from a source table.
- `blit_second_block_below_six`: Returns early when a second counter exceeds five.
- `blit_counter_block_in_range`: When a counter lies between one and five, waits for the blitter, programs its control and mask registers, and copies a block from a source table.
- `blit_second_block_in_range`: When a second counter lies between one and five, programs the blitter control registers and copies a block from a source table.
- `score_draw`: The routine checks a global variable against itself and calls another routine if they differ.
- `draw_two_digit_pair_glyphs`: Splits a byte into its two nibbles, maps a zero nibble to the value ten, and copies glyph data for each resulting digit into a bitplane buffer eight times.
- `dispatch_interrupt_request_handlers`: Reads the interrupt enable and request registers together and calls a handler for each of two request bits that are set.
- `set_global_value_32`: Stores the constant 32 into a global variable and returns.
- `advance_timer_spawn_object`: When a flag is set, the routine clears it, advances an animation counter that wraps around, and decrements a countdown timer.
- `decode_keyboard_serial_key`: Once the pending flag is set, it reads a keyboard byte from the CIA-A serial data register, shifts out its bits and records a pressed or released state in a per-key table.
- `update_object_table_entries`: Initialises a set of counters and then walks a table of object entries until it reaches an end marker.
- `add_packed_decimal_score`: Adds a packed decimal value into a score byte with decimal adjustment and carry propagation.
- `store_entry_in_slot`: If a pending slot index is valid, the flag bit is clear and the x coordinate is aligned to eight units within a window around a reference position, it writes the coordinate into that slot with an offset, writes a fixed value into a second table entry and a marker value into a third field.
- `draw_active_entries_bitplanes`: Loops over thirty eight-byte table slots, decrementing each slot's timer and clearing expired entries.
- `fill_pending_motion_entry`: Fills the pending table slot with a new entry: scaled and offset x and y positions taken from the inputs, two timer values chosen from the type input, and two motion fields derived from differences against reference values.
- `advance_and_plot_entries`: Loops over fifteen eight-byte entries, moving each by its stored step and removing entries that leave the visible range or reach a limit.
- `swap_bitplane_buffers`: Toggles a screen flag and saves the three current bitplane pointers into backup variables.
- `clear_screen_with_blitter`: Waits until the blitter is idle, then sets its masks, modulo and control word so only the destination channel is used with the logic function set to zero.
- `run_copper_setup_sequence`: Calls six routines in order and returns, with no data handling of its own.
- `store_constant_table_pairs`: Writes six fixed pairs of values into a block of global variables, each pair a value followed by a second constant, and returns.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `install_handler_enable_interrupts`: Stores the address of a handler routine in a global vector variable and writes an enable mask to the interrupt enable register with the top bit set, so the listed interrupt sources are switched on.
- `configure_cia_timer_interrupt`: Stores a handler routine address and writes a sequence of constants to the CIA-A control, interrupt and timer registers, configuring a timer interrupt.
- `dispatch_two_subroutines`: Calls two routines in order and returns.
- `save_view_and_hardware_state`: Calls a routine, then stores the current view and copper list pointers and the DMA control and interrupt enable register values into global variables.
- `disable_all_dma_and_interrupts`: Writes a value with every bit set except the top one into the DMA control and interrupt enable registers, which clears those enable bits.
- `restore_saved_hardware_state`: Clears the DMA and interrupt enable bits, then restores the saved copper list pointer and DMA and interrupt enable values with the top bit set so they are re-enabled.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `open_graphics_library_retry`: Repeatedly calls the exec library open function until it returns a non-zero library base, then stores that base in a global variable.
- `open_dos_library_retry`: Repeatedly calls the exec library open function until it returns a non-zero library base, then stores that base in a different global variable.
- `close_library_base`: Calls the exec CloseLibrary function on the library base held in a global variable and returns.

### More

- `start_program_parse_arguments`: Program entry point that allocates a working structure, copies the command-line arguments into a buffer and splits them into a table of strings, handling quoted text and escape codes.
- `release_working_resources`: Stores its argument in a working structure, then closes an open file and a library if they are present, frees the structure's memory with task switching disabled and replies to a message.
- `run_pager_session`: Opens a window and runs the pager session, reporting errors such as a failed window open, a file that is not found or an end of file.
- `apply_to_four_entries`: Calls a routine four times, each time passing a field from the input structure together with one of four consecutive global values in turn.
- `check_buffer_for_binary`: Calls a read routine to fill an eighty-byte buffer, then scans it and returns 1 if any byte is a control code other than tab, newline or form feed, or 0 when every byte looks like text.
- `read_pager_key_command`: Waits for a pending input event with a timeout and returns early when a break signal arrives.
- `process_pager_output`: Checks if a long value at offset 60 of the structure in a2 is non-zero, then calls a routine to output with flags and another to read a chunk and count lines.
- `execute_unreached_code`: Executes a sequence of calls including a routine with a global variable, another routine, and then a function pointer held in an uninitialized register.
- `initialize_state_run_steps`: Sets an activity counter to one and clears a flag in a state block, then calls a routine and stores its result.
- `update_state_increment_counter`: Calls two routines, passing the second one a field from the state block and the result of the first, and stores the second result back in the block.
- `read_buffer_find_line_end`: Reads up to 128 bytes into a buffer, scans for the first newline or form feed, and returns the number of characters up to and including it.
- `run_pager_loop_until_quit`: Chooses a loop count from state flags, then repeats a step that passes each table entry to a routine and reads a key.
- `search_text_until_found`: Selects a search buffer from a flag, prepares the search pattern and repeatedly fetches the next line and tests it until a match is found, then returns one.
- `wrap_call_with_setup`: Performs a fixed sequence of calls around a main call, with an extra call when a field is set, and returns the result of the main call.
- `call_pair_repeat_if_clear`: Calls a routine with two arguments, then calls it again with only the first argument when a flag in the state block is clear.
- `call_with_window_field`: Calls a routine with a stored pointer, a value and a fixed argument, then calls another routine with a short field read from a linked state block.
- `call_with_window_minus_one`: Calls a routine with a fixed zero value and a one, then calls it again with the window field from a linked state block minus one.
- `draw_text_buffer_lines`: Walks a text buffer and outputs up to the requested number of lines in chunks of at most 128 characters.
- `call_then_forward_pointer`: Calls a routine with the second argument, then calls another routine with a pointer field from that structure, the address of a global variable and the first argument.
- `set_flag_and_forward`: When the field at offset four is zero it first sets a short counter in a linked structure to one.
- `dispatch_by_flag`: Calls a routine with a stored pointer, a global variable address and a second value.
- `call_with_flag_arguments`: Calls a routine with a stored pointer and a global variable address.
- `clamp_negative_result`: Calls a routine twice with the same first argument and different third arguments, then clamps a negative result to zero and returns it.
- `consume_from_buffer`: Transfers up to the requested count of bytes from a buffer at a stored offset into the caller's destination, limited by the remaining size.
- `refill_buffer_window`: Returns zero early when a state field is not at its full value and pending data exists.
- `update_window_position`: Computes a target position from the argument and the structure's offset fields, then compares it with the window start and length.
- `read_line_from_buffer`: Reads characters one at a time through a routine into a destination string until a newline arrives or a fixed limit of about 127 characters is reached.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination one at a time, including the terminating null, and stops once the null has been copied.
- `convert_string_to_uppercase`: Walks a null-terminated string in place and changes each lowercase ASCII letter to uppercase by subtracting the case offset.
- `parse_signed_decimal`: Reads an optional plus or minus sign and then accumulates decimal digits from a null-terminated string, returning the signed integer value.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `print_message_and_reset`: Passes the structure's fields through a series of routine calls, writing the message string when one is given or running the default handler when it is empty.
- `send_one_byte_message`: Stores its argument in a one-element stack buffer and passes that buffer, with a size of one, to a routine together with a handle from a lookup and a constant code.
- `copy_allocated_block_fields`: Allocates a block of 36 bytes and reads two fields from it into the caller's structure, using the result of an intermediate call to select the source.
- `create_linked_object_copy`: Allocates a 68-byte block, links it to a handle obtained from a routine and copies a given number of words from a source into it.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `forward_with_stack_pointer`: Calls another routine with its two arguments and a pointer to a fixed location in the caller's stack frame, then returns.
- `extract_byte_from_call`: Calls a routine to fill a one-byte stack buffer and returns that byte as a number when exactly one byte was produced, or minus one otherwise.
- `format_and_forward_text`: Calls the exec formatting routine through a library base held in a global variable, then measures the null-terminated text left in a 140-byte stack buffer.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `divide_32_bit_values`: Divides a 32-bit value by a divisor, using a bit-by-bit long division when the divisor is wider than 16 bits and a 16-bit divide otherwise.
- `multiply_32_bit_integers`: Multiplies two 32-bit values using 16-bit partial products and combines them into a 32-bit product.
- `recursive_call`: The routine calls itself and then returns.
- `call_helper_return_register`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag`: Handles the signs of two values, calls a helper, and negates the result when the flag the helper leaves in an address register is nonzero before returning.
- `negate_on_flag`: Calls a routine and returns its first result, negated when the second result register is non-zero.
- `invoke_formatting_routine`: Calls the exec formatting routine through a function pointer stored in a global variable and returns.
- `store_byte_through_pointer`: Stores the low byte of the data register into the memory location held in an address register and returns.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Allocates a signal bit and, if that succeeds, allocates a memory block for a message.
- `release_message_port`: Removes the port from a list when it is linked, marks it as deleted and frees its signal bit, then releases the 34-byte memory block.
- `find_substring_in_string`: Searches a null-terminated string for a second string, trying each starting position in turn, and returns one when the second string is found.
- `initialize_self_linked_header`: Stores the block's own address into its first word, advances that pointer by four, clears the next word and stores the block address into the word after it.

### Assign

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_then_call`: If a destination pointer is given, copies sixteen 32-bit words from a fixed offset in a base structure into it.
- `update_entity_metadata_and_indices`: Updates an entity's index and pointer offsets within a structure, clears specific status flags, and calls several routines to synchronize entity data and handle potential index shifts or invalidation within a table.
- `synchronize_entity_indices_and_shift`: Updates an entity's index within a table and performs a loop to shift subsequent entries to fill gaps, while calling several routines to update entity states and properties.
- `update_entity_index_and_handle_removal`: Retrieves an index from a table, calls another routine, and if the index is zero, checks entity properties to either update a pointer in a structure or perform a removal sequence that involves setting a value to twenty at a specific offset and calling another routine.
- `reorganize_entity_table_and_update_state`: Synchronizes two indices within a table, performs a loop to shift subsequent entries to fill a gap, and calls several routines to update entity properties and state based on specific offset values.
- `update_assignment_state`: Manipulates several values in structures pointed to by a0 and a1 using offsets, calls a routine multiple times with different parameters, and ends with an indirect jump.

### EndCLI

- `scan_list_and_build_block`: Program entry: walks a linked list to find the largest value, then creates a block sized from that value and fills it with an increasing sequence of values.
- `copy_task_data_and_jump`: This routine copies a block of sixteen bytes from an offset of a task structure to the stack before calling another routine.
- `modify_cli_state`: Performs bitwise operations on a word in a2, updates a value in a1, calls a routine multiple times with various parameters, and decrements a value in a structure pointed to by a0 before an indirect jump.

### Echo

- `initialize_task_structure`: Iterates through a linked list to find the maximum value in a field, then populates a new task structure with a sequence of incrementing values.
- `copy_stack_data`: Copies sixteen bytes from a memory location to a stack buffer and then calls another routine.
- `update_entity_range_and_sync_indices_2`: Calculates a new range for an entity by calling another routine with constant 244, updates the entity's index and range metadata, and then performs a conditional loop to synchronize indices based on the calculated range.
- `update_entity_range_and_sync_indices`: Updates an entity's index and range metadata by calling two routines with constant values 340 and 344, then calculates a new range based on a byte read from a table and performs a loop to synchronize indices through repeated calls to another routine.
- `copy_structure_longword`: Copies a longword from offset 336 of a structure in a1 to offset 328 of the same structure and branches to another routine.
- `update_entity_index_range`: Sets a value of 1 at offset 328 of a structure, then checks a table using an index derived from the structure's first long word.
- `copy_structure_longword_offset_14c`: Copies a longword from offset 336 of a structure in a1 to offset 332 of the same structure and branches to another routine.
- `rotate_and_fetch_table_byte`: Rotates three long values within a structure, performs a comparison, and if the condition is met, calculates an index into a table using a value from the structure and a shifted register, fetches a byte from that table, and calls another routine with the byte in d1.
- `execute_conditional_echo_2`: Shifts a long value in d1 left by 2 bits, checks a long value at offset 4 plus the shifted d1 in a0, and if non-zero, loads 10 into d1, 352 into d0, loads a pointer from offset 224 of a2 into a4, and calls another routine before branching to another location.
- `execute_conditional_echo`: Checks a long value at offset 584 of a2, and if non-zero, calls a routine three times with different constant values in d0 and a4 before an indirect jump.

### LoadWB

- `initialize_pager_session`: Allocates memory for a structure and parses a command line string into individual arguments.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_resident_tool`: Applies the same XOR byte twice to a hidden string, which restores the original text, then looks up a resident tool and starts it.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns minus one when the first is smaller, one when it is larger, or zero when they match.
- `build_linked_list_from_table`: Walks a table of linked entries, allocating an eight-byte node for each and storing a value from a routine in it.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_function_offset_84`: Calls a function via a pointer located at a specific offset within a global structure.
- `call_indirect_function`: Fetches a function pointer from a location within a global table and executes it.
- `execute_indirect_function`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_indirect_function_2`: Fetches a function pointer from a global table using a calculated offset and executes it.
- `call_indirect_offset_minus_366`: Calls a routine via a pointer located at a specific offset from a global variable.
- `call_indirect_function_3`: Fetches a function pointer from a global table and executes it.
- `call_library_function`: Fetches a function pointer from a table using an offset and executes it.
- `execute_indirect_function_2`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_indirect_function_4`: Fetches a function pointer from a global table and executes it.
- `call_library_entry_624`: Calls a library function at a fixed offset from the library base held in a global variable and returns its result.
- `call_reschedule_function`: Calls the exec Reschedule function through a pointer held in a global structure and returns.
- `call_dispatch_function`: Calls the exec Dispatch function through the library base held in a global structure and returns.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_function`: Calls the exec AllocMem function through the library base held in a global variable and returns the allocated address.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer`: Stores a byte from a register through a pointer.
- `match_reference_string`: Compares a string pointed to by a2 against a global string, potentially calling a routine to write padding if a specific character is found at offset 20.
- `write_dot_and_padding`: Writes a dot followed by four space characters into a buffer at a fixed offset, with no other effect.
- `skip_matching_prefix`: Advances through a buffer while its characters match a reference string, stopping at the first mismatch or at the end of the reference.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends with a return.

### Ram-Handler

- `dispatch_through_unrecovered_jump`: Program entry that calls a library function, then jumps through a table whose targets the decompiler could not recover.

### Defender.adf:Apc-Trainer

- `start_program_write_background_color`: Program entry point that writes to the background colour hardware register and calls four routines.
- `store_byte_and_return_shifted`: Reads a longword from just before an address register, writes a word made from a byte of it and the high byte of a second argument through a pointer, and returns the longword halved with its top bit set.
- `shift_bit_buffer_refill`: Shifts a bit register right once per iteration for a count taken from an argument, and calls a routine to refill the register whenever it runs empty.

### ramdrive.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_ram_unit_from_list`: Opens a library and walks a linked list of unit entries looking for one whose name matches a stored string.
- `build_ram_unit_structures`: Copies unit parameters, allocates a buffer for the unit's data, and fills a 128-entry table with an increasing sequence while zeroing the following block.
- `copy_words_then_build_unit`: Copies a block of words from a source into a destination, then builds the unit structures in the same way as its neighbouring routine.
- `initialize_ramdrive_device`: Stores pointers to internal buffers and control fields and links the new unit into a global list while interrupts are masked.
- `execute_double_indirect_call`: Calls the same routine twice using a pointer found at a specific offset from a value in an address register.
- `allocate_matching_list_entry`: Masks interrupts while it walks a linked list for entries whose flags contain the requested bits, records a value from a matching entry that reaches the threshold, and passes the result to an exec library routine.
- `scan_matching_list_entries`: Walks a linked list for entries whose flags contain the requested bits and, once a qualifying value is found, calls an exec library routine with it.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_interrupt_nest_count`: Decrements a nesting count stored in the device base and re-enables interrupts through a global write when the count falls below one.
- `store_value_at_offset`: Stores the data register value at a fixed offset inside a structure given in an address register, then returns zero.
- `dispatch_device_message`: Disables task switching, selects a handler from a table of sixteen entries using a code in the message, and stores that handler's result in the message.
- `return_minus_three_constant`: Returns the constant minus three with no other effect.
- `jump_to_tiny_moveq_rts_2`: Jump stub: passes control straight on to tiny_moveq_rts_2.
- `increment_counter_and_load_pointers_2`: Increments a 16-bit word at offset 50 of a structure pointed to by a value at offset 20 of a1, then performs a block copy of data from a memory location to a destination buffer and updates several pointers and counters within the structure.
- `copy_ramdrive_buffer_2`: Checks a bit in a structure, increments a 16-bit counter at offset 34 of a second structure, and performs a block copy of data from one memory area to another.
- `branch_to_offset`: Branches to a relative offset.
- `branch_to_offset_2`: Branches to a relative offset.
- `jump_to_tiny_moveq_rts_2_2`: Jump stub: passes control straight on to tiny_moveq_rts_2.
- `jump_to_tiny_moveq_rts_2_3`: Jump stub: passes control straight on to tiny_moveq_rts_2.
- `jump_to_tiny_moveq_rts_2_4`: Jump stub: passes control straight on to tiny_moveq_rts_2.
- `branch_to_offset_3`: Branches to a relative offset.
- `jump_to_tiny_moveq_rts_2_5`: Jump stub: passes control straight on to tiny_moveq_rts_2.
- `copy_ramdrive_buffer_3`: Checks a bit in a structure, increments a 16-bit counter at offset 34 of a second structure, and performs a block copy of data from one memory area to another.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `set_structure_member`: Stores the value one into a 32-bit word at offset 32 of the structure pointed to by a1, then returns.
- `update_status_conditionally`: Loads a longword from offset 24 of the structure in a1 into d0, and if it is not zero, branches to a relative offset; otherwise, it clears d0, stores d0 at offset 32 of the structure in a1, and returns.
- `increment_counter_and_load_pointers`: Increments a 16-bit word at offset 34 of a structure pointed to by a value at offset 20 of a1.
- `decrement_structure_word`: Decrements a 16-bit word at offset 34 of the structure pointed to by a6 and returns.
- `copy_ramdrive_buffer`: Checks a bit in a structure, increments a 16-bit counter at offset 34 of a second structure, and performs a block copy of data from one memory location to another using a loop.
- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 28 into d0, then returns.
- `increment_global_byte`: Loads a pointer from a constant address into a0, increments a byte at offset 295 of the pointed structure, and branches to another routine.
- `unlink_and_free_unit_node`: Decrements a counter in a unit structure and, when it goes negative while a link is pending, unlinks the node from its list and releases its memory through the exec library.
- `manipulate_linked_list_and_string`: Traverses a linked list to update a pointer, calls a routine to store a return value in a global field, and replaces a null terminator in a string with a colon.
- `terminate_string_with_colon`: Copies a longword into a destination, calls a library function, then advances to the end of a string and replaces its terminating null with a colon before calling the Permit function.

### mathtrans.library

- `stub_bad_instruction_data`: Library entry point of the math library that contains only an unimplemented instruction, so the decompiler stops here and no behaviour is recovered.
- `store_pointer_in_global`: Stores the address register argument into a global variable and returns, as a routine of the math library.
- `unlink_and_free_library_node`: If a short field is zero, it unlinks the structure from its list and frees its memory through the exec library, then returns a stored result.
- `mark_structure_pending_flag`: Sets the bit with value eight in a flag byte inside the structure and returns zero.
- `open_math_library_first_use`: If the flag bit with value eight is clear, it increments a use counter and, on the first use, stores the exec base and opens the library named in the text.
- `return_minus_one_constant`: Returns minus one with no other effect.
- `decrement_counter_and_call`: Decrements a short counter in the structure.
- `forward_call_and_return`: Calls a single routine and returns, with no data handling of its own.
- `compute_mantissa_bit_loop`: Returns at once for a zero input and passes a negative input to a routine.
- `adjust_bitfield_value`: Sets a bit in a value chosen by a count, then runs a loop that doubles a difference and shifts a mask until the count runs out.
- `call_and_adjust_status_bits`: Calls a routine through the math library and returns its result with the bit of value eight cleared and the bit of value two set.
- `call_math_entry_sequence`: Calls nine library entries through the math library base, alternating between two of them and calling a third entry once in the middle, then returns.
- `compare_values_adjust_result`: Calls several math library entries to compare and classify two operands, choosing one of several call sequences from the outcomes.
- `select_math_sequence_result`: Calls math library entries to test two operands and selects one of several call sequences based on the outcome.
- `select_math_entry_by_sign`: Calls a math library entry, then selects further entries based on a sign flag and a second value.
- `select_entries_by_two_flags`: Chooses among a few math library entries based on two flags and a sign value, then returns the result of a further call with the bits of value two and eight cleared.
- `choose_math_entry_by_sign`: Calls a math library entry, then chooses between two or three further entries based on a sign value and a second flag.
- `call_entry_then_mask_result`: Calls one math library entry, then returns the result of a further call with the bits of value two and eight cleared.
- `call_entry_twice_mask_result`: Calls the same math library entry twice, then returns the result of a further call with the bits of value two and eight cleared.
- `mask_return_value`: Calls a math library entry twice, then returns the result of a further call with the bits of value two and eight cleared.
- `call_indirect_via_table`: The library code fetches a function pointer from a table and executes it.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Fetches a function pointer from an offset within a global structure and executes it.
- `divide_mantissa_bit_loop`: Calls a routine when the exponent is large, then runs a 24-step bit loop that subtracts shifted values to build a quotient.
- `run_quotient_bit_loop`: Runs the same 24-step subtract-and-shift loop on its argument to build a quotient, and calls a routine when the loop overflows before returning.
- `execute_loop_with_branching`: Runs a shift-and-subtract loop that decrements a counter each step.
- `compute_cordic_style_value`: Reduces its angle argument by fixed constants, then runs a shift-and-add loop over a table of constants, as used for trigonometric values.
- `compute_cordic_range_checked`: Returns early when the argument is outside a set range.
- `compute_cordic_then_call`: Reduces the argument by fixed constants and runs a shift-and-add loop over a table of constants.
- `compute_cordic_signed_result`: Returns early when the argument is outside a set range.
- `compute_cordic_accumulate_only`: Reduces the argument, runs a shift-and-add loop over a table of constants and calls one routine with the result, with no branch on sign.
- `compute_cordic_with_sign_branch`: Reduces the argument, runs a shift-and-add loop over a table of constants, and then branches on a sign value passed on the stack to choose which routines to call.
- `run_cordic_loop_signed`: Runs a shift-and-add loop over a table of constants on its argument and calls routines whose path depends on a sign value passed on the stack.
- `run_cordic_loop_branch`: Runs a shift-and-add loop over a table of constants and then branches on a sign value to choose which routines to call with the result.
- `run_cordic_table_iteration`: Walks a table of constants, updating three running accumulators with shifts and adds for each entry, and calls routines depending on a sign value before returning.
- `return_without_action`: Does nothing and returns, with no register or memory access of its own.
- `recursive_call_wrapper`: Calls one routine that the decompiler shows as a self-call, then returns.
- `execute_recursive_calls`: This library routine performs two consecutive calls to itself before returning.
- `normalize_value_exponent_loop`: Negates a negative value and, when it lies outside a small range, shifts its bits left until the top bit is set while adjusting an exponent counter.
- `normalize_value_exponent_check`: Returns early when a second operand's upper byte is minus one.
- `call_one_then_two`: Calls one routine, then calls two more routines in sequence, with no branch that changes that path.
- `call_one_then_three`: Calls one routine, then calls three more routines in sequence, with no branch that changes that path.
- `call_incremented_value_pair`: If its byte argument is non-zero and not at the top of its range, it calls a routine and then calls two routines with the incremented byte value followed by a third routine.
- `return_from_subroutine_3`: This routine performs no operations and immediately returns to the caller.
- `return_immediately_2`: The library code consists of two instructions and returns immediately.
- `return_from_subroutine_4`: This routine performs a sequence of instructions that results in an immediate return to the caller.
- `empty_stub_2`: The library code contains five instructions and performs no observable work; it ends immediately.
- `empty_routine_2`: The library code consists of four 68000 instructions and performs no observable work, immediately returning to its caller.
- `shift_and_store_byte_value`: Returns at once for a zero input.
- `call_four_routines_in_sequence`: Calls four routines one after another and returns, with no data handling of its own.
- `sum_alternating_table_entries`: Walks a table of longword entries, adding each entry to a running total while the total is negative and subtracting it otherwise, and stops when the entry count runs out.
- `subtract_table_from_accumulator`: Adds the first table entry to an accumulator, then subtracts the following entries whenever the accumulator stays non-negative, and returns when the entry count runs out.
- `build_mantissa_with_exponent`: Returns at once for a zero input.
- `return_immediately_3`: This routine performs no operations and returns immediately to the caller.
- `build_mantissa_with_exponent_2`: Calls several routines around a bit-shifting loop that builds a mantissa while an exponent counter steps down, then stores the result through two more routines when the value is non-zero.
- `shift_loop_with_exponent_exit`: Runs a bit-by-bit subtract-and-shift loop and, on exit, calls a routine.
- `call_routine_three_times`: Calls one routine three times in succession and then returns.
- `store_signed_shifted_value`: Returns after one routine call when the input is negative.
- `store_shifted_value_checked`: Calls two routines, then returns early for a zero input.
- `empty_library_stub`: The library code contains no instructions and performs no operation.
- `stub_return_without_body`: Shows no body in the decompiled code beyond a return, although the listing reports calls to other routines, so the real body is not recovered.
- `execute_stub_routine`: This library code contains a body that performs no operations before returning.
- `return_immediately_4`: The library code contains six instructions and returns without performing any operations.
- `return_empty_routine`: This library code is empty and performs no operations before returning.
- `return_immediately_5`: The library code contains five instructions and returns without performing any operations.
- `return_immediately_6`: This library routine is empty and performs no operations before returning to the caller.
- `invoke_callback_and_return`: Calls the routine whose address is held in an address register, then returns the data register value it was given, unchanged.
- `call_indirect_return_d0`: This library code receives a function pointer in an address register, calls that routine, and returns the original value from a data register.
- `call_routine_return_d0`: Calls a routine and returns the data register value it was given, unchanged.
- `load_library_address_136`: As library code, it loads a 32-bit address from offset 310 into a0 and branches to the start of the routine.
- `load_library_address_13e`: As library code, it loads a 32-bit address from offset 318 into a0 and branches to another routine.
- `load_library_address_146`: As library code, it loads a 32-bit address from offset 326 into a0 and branches to the start of the routine.
- `load_library_address_14e`: As library code, it loads a 32-bit address from offset 334 into a0 and branches to the start of the routine.
- `load_library_address_156`: As library code, it loads a 32-bit address from offset 342 into a0 and branches to the start of the routine.
- `load_library_address_15e`: As library code, it loads a 32-bit address from offset 350 into a0 and branches to another routine.
- `load_library_address_166`: As library code, it loads a 32-bit address from offset 358 into a0 and branches to another routine.
- `load_address_and_branch`: As part of library code, loads a longword from a memory location into a0 and branches to a relative offset.
- `load_address_and_jump`: As part of library code, loads a longword from a memory location into a0 and branches to a relative offset.
- `invoke_callback_and_return_d0`: This library code calls a routine via an address register and returns the data register value unchanged.
- `execute_callback_and_return`: Calls a routine via a pointer and returns the original value held in a data register.
- `invoke_callback_and_return_2`: This library routine calls a routine whose address is held in an address register and returns the resulting data register value unchanged.
- `invoke_callback_and_accumulate`: This library routine calls a routine and then performs a shift-and-add loop over a table of constants to reduce an argument.
- `invoke_callback_and_cordic`: This library routine calls a routine and then reduces an argument using a table of constants via a shift-and-add loop.
- `invoke_callback_range_checked`: This library routine calls a routine and returns early if the provided argument is outside a specific range.
- `store_register_via_pointer`: Calls a routine, then stores the 32-bit data register value through the pointer held in another data register and returns.
- `invoke_callback_and_sequence`: This library routine calls a routine and then executes three subsequent routines in a fixed sequence.
- `invoke_callback_and_search`: This library routine calls a routine and then searches for a substring within a null-terminated string.
- `invoke_callback_with_increment`: This library routine calls a routine and then calls two additional routines using an incremented byte value if the input is non-zero.
- `invoke_callback_and_shift`: This library routine calls a routine and performs a shift-and-store operation, returning immediately if the input is zero.
- `invoke_callback_and_extract`: This library routine calls a routine to fill a one-byte stack buffer and returns that byte or an error value.
- `invoke_callback_triple_call`: This library routine calls a routine three times in succession before returning.
- `call_indirect_function_5`: This library code calls a function through a pointer provided in an address register and returns the original value from a data register.
- `call_and_return_d0`: Calls another routine and then returns the value left in data register D0 unchanged.
- `invoke_callback_indirect`: This library routine calls a routine whose address is held in an address register and returns the value from a data register unchanged.
- `call_jump_table_entry_d0`: Calls a routine through a jump-table entry at a fixed negative offset from a base pointer kept in a global variable, then returns the value in D0.
- `call_indirect_with_offset_2`: This library routine calls a routine at an address calculated by adding an offset to a base pointer stored in a global variable.
- `call_jump_table_return_pair`: Calls a routine through a jump-table entry at a fixed negative offset from a global base pointer, then returns data registers D0 and D1 together as one 64-bit value.
- `call_jump_table_return_64bit`: This library routine calls a routine through a pointer stored in a global variable and returns the concatenated values of two data registers.
- `call_indirect_with_offset_alt`: This library routine calls a routine at an address formed by adding an offset to a base pointer in a global variable and returns a data register.
- `execute_indirect_offset_call`: Performs an indirect call to a routine located at an offset from a base address stored in a global variable.
- `execute_indirect_function_3`: Calls a routine via a pointer retrieved from a global variable and returns the resulting data registers as a concatenated 64-bit value.
- `execute_indirect_call_and_concatenate`: Calls a routine via a pointer stored at an offset from a global variable and returns the resulting contents of two data registers concatenated into a single 64-bit value.
- `execute_indirect_call_and_concatenate_2`: Calls a routine via a pointer stored at a fixed offset from a global variable and returns the resulting values in the data registers concatenated as a single 64-bit value.
- `call_jump_table_return_pair_2`: Calls a routine through a pointer stored in a global variable, then returns data registers D0 and D1 concatenated as one 64-bit value.
- `call_indirect_with_offset`: Calls a routine at an address formed by adding the offset in register A0 to a base pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: Normalizes the signed 32-bit value in register D6 by negating negative input, swapping its 16-bit halves when the magnitude is below 32768, then shifting it left while decrementing a count until the sign bit is set or the count runs out.

### icon.library

- `return_minus_one_stub`: Returns the constant minus one and does no other work.
- `increment_counter_clear_bit`: Increments a 16-bit counter stored in a structure and clears one flag bit in a byte of that same structure, then returns.
- `decrement_library_refcount`: Decrements a 16-bit reference count in a library structure.
- `unlink_node_or_mark_pending`: When a count field in the structure is zero, it unlinks the structure from its doubly linked list and calls a library routine through a jump-table entry, returning a stored value.
- `set_flag_bit_return_zero`: Sets one flag bit in a byte of a library structure and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_or_alert`: Stores two arguments in fields of a structure, then calls a library routine that refers to the dos.library name.
- `copy_two_longs_to_globals`: Copies two 32-bit values from a structure addressed by a register into two global variables, then returns.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_write_icon_info`: This library routine unconditionally calls a helper routine to write an icon information structure to a file and then returns.
- `invoke_handler_pointer`: This library routine calls a routine via a pointer and then returns.
- `call_library_handler_2`: This library routine simply calls another routine and then returns.
- `call_library_helper`: This library routine calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine_2`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `invoke_handler_pointer_2`: This library routine retrieves a pointer from a data structure and calls the routine located at that address.
- `invoke_handler_pointer_3`: This library routine fetches a function pointer from a data structure and calls it.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_match_routine`: This routine calls another routine and then returns.
- `call_parse_copy_prefix_number`: This library routine calls another routine and then returns.
- `update_data_field`: Looks up a function through a jump-table entry whose base pointer is stored in the first argument's structure, calls it with that argument, and stores the second argument into a fixed field of the pointer it returns.
- `load_header_into_structure`: Allocates a structure through a library routine, reads a header into a stack buffer through another routine, then copies its ten longwords and several scalar fields into the structure.
- `copy_header_into_structure`: Copies ten longwords and several scalar fields from a frame-local header into a structure, and if the structure's linked object is set it writes the structure back into that object.
- `return_zero_stub`: Returns the constant zero and does no other work.
- `copy_icon_data_to_stack`: Builds a header on the stack by copying ten longwords and several fields from a structure, adds a fixed tag word and a version word, then passes the header with the first two arguments to another routine.
- `load_icon_info_file`: Loads a .info file by reading a fixed-size header, checking its magic word and version, then reading the optional image and text sections and allocating memory for each.
- `parse_icon_info_sections`: Continues icon loading from a header already held in registers: it checks the magic word and version, then reads optional image and text sections, allocating memory and storing the pointers in the structure.
- `read_icon_pointer_tables`: Reads two 32-bit values into the structure, then reads optional string and null-terminated pointer tables referenced from it, storing each result.
- `read_null_terminated_pointer_table`: Reads pointer-table entries one longword at a time until the remaining byte count runs out, storing each entry, then reads one optional trailing field.
- `call_and_return_zero`: Calls another routine and then returns zero.
- `return_immediately`: Returns at once without doing any work.
- `write_icon_info_file`: Writes an icon info structure to a file: it allocates a 260-byte work buffer, writes the fixed-size header and magic value, writes the optional image and text sections, then releases the buffer.
- `write_icon_info_sections`: Writes the optional string sections of an icon info structure, each preceded by its length, including a table of null-terminated strings.
- `write_icon_pointer_tables`: Writes the optional string and pointer-table sections of an icon structure, writing a length before each entry.
- `write_pointer_table_entries`: Writes each string in a null-terminated pointer table, preceding each one with its length, then writes one optional trailing string.
- `iterate_icon_table_entries`: Walks a null-terminated table of pointers, passes each entry to a routine that returns its length, writes that length, and invokes a callback for each entry.
- `process_circular_node_list`: Walks a circular list of nodes starting from the node after the one given, passing each node to a library routine until it returns to the start or runs out, then passes the final node to the same routine.
- `iterate_linked_list_nodes`: Walks a circular linked list from a head node, passing each node to a library routine and advancing with another routine that returns the next node, until the head is reached or no next node exists.
- `call_with_advanced_offset`: Calls another routine with the first argument unchanged and the second argument increased by a fixed step, then returns.
- `allocate_linked_struct`: Allocates a zero-filled 168-byte block through a library routine, initializes one field, and passes it through a second routine.
- `decrement_and_return_zero`: This library routine decrements a counter in a structure and then returns zero.
- `return_zero_2`: Library code that returns zero.
- `decrement_counter_store_pair`: Decrements a 16-bit count in a structure when it has run out, first calling a refill routine and returning zero if that fails.
- `decrement_counter_and_store_pair`: Decrements a 16-bit count in a structure and stores two register values as a pair in the array entry that the new count indexes, then returns one.
- `allocate_linked_entry`: Allocates a zero-filled 94-byte block through a library routine, initializes one field, and links it through two calls to other routines.
- `call_and_return_zero_2`: This library code calls another routine and then returns zero.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `incremental_recursive_call`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `find_value_after_equals`: Walks a null-terminated array of strings, checks each one with a comparison routine, and returns a pointer just past the equals sign of the matching entry, or zero when no entry matches.
- `find_equals_delimiter_2`: This library code traverses a list of values to find the offset of an equals sign character.
- `find_equals_delimiter`: This library routine traverses a list of values to find a specific offset where a character matches an equals sign, returning a pointer to the character immediately following it.
- `match_delimited_item_by_name`: Walks a chain of delimiter-separated items, comparing each item with a given name of the same length, and returns one when a matching item is found or zero when the chain ends.
- `match_delimited_item_of_length`: Walks delimiter-separated items and returns one when an item has the wanted length and passes the comparison routine, or zero when the chain is exhausted.
- `scan_delimited_items_for_length`: Scans delimiter-separated items for one whose length equals the wanted value and passes the comparison routine, returning one when found and zero when the text ends.
- `search_delimited_items_2`: This library routine iterates through a list of items, calculating lengths and comparing them against a target value using a specific delimiter.
- `search_delimited_items`: Library code that iterates through a list of items to find a match based on a specific length and comparison criteria.
- `parse_copy_prefix_number`: Checks the start of a name against referenced text fragments, reads an optional decimal copy number after it, and stores the number and remaining text in the structure, returning the structure.
- `parse_copy_count_suffix`: Reads a decimal number from a name at a fixed position and checks the text that follows it against a referenced fragment, then calls cleanup routines.
- `parse_decimal_then_match_text`: Reads consecutive decimal digits into a number and, when the number is non-zero, checks the text that follows against a referenced fragment.
- `parse_decimal_string`: Library code that converts a sequence of numeric characters into a decimal value and performs subsequent string operations.
- `convert_digit_char_value`: Returns the numeric value of a character when it is a decimal digit, or minus one when it is not.
- `return_maximum_value`: This routine returns a value consisting of all bits set to one.
- `call_library_vector_entry`: Calls a library function through a jump-table entry at a fixed negative offset from a base pointer kept in a global variable, then returns.
- `forward_control_via_table`: Library code that retrieves a routine from a table and forwards control to it.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `allocate_and_init_record`: Allocates a zero-filled 96-byte block, sets two fields to ten, passes the block to a library routine for initialization, and returns one when the allocation succeeded and zero otherwise.
- `return_zero_value`: This library routine returns zero.
- `call_and_check_equal_result`: Calls another routine with three arguments and returns its result when that result equals the fourth argument, otherwise returns zero.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `verify_parameter_equality`: This library routine calls another routine with three parameters and returns zero if the result does not match a fourth parameter.
- `return_from_subroutine`: This routine is empty and performs only a return instruction.
- `divide_unsigned_with_remainder`: Divides an unsigned 32-bit value by a second value and returns the quotient and remainder together.
- `divide_bit_by_bit_loop`: Performs unsigned division one bit at a time over sixteen passes: each pass shifts the dividend and remainder left, subtracts the divisor when the remainder reaches it, and records the quotient bit.
- `multiply_32_bit_integers_2`: Multiplies two 32-bit values from the data registers using 16-bit partial products, returning the low 32 bits of the product and a half-swapped copy of the second operand.
- `forward_call_to_helper`: As library code, it calls one helper and returns, with no other work.
- `call_helper_return_register_2`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag_2`: As library code, it handles the signs of two values, calls a helper, and negates the result when the flag the helper leaves in an address register is nonzero before returning.
- `negate_on_flag_2`: calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.
- `call_raw_format_routine`: Calls the exec formatting routine RawDoFmt through a jump-table entry at a fixed offset from a global base pointer, passing the arguments already in registers.
- `store_byte_through_pointer_2`: Stores the low byte of data register D0 at the address held in address register A3.
- `count_string_length`: Returns the number of characters before the zero terminator of a string by decrementing a 16-bit counter for each character, stopping early if the counter wraps.
- `find_character_in_string`: Scans a zero-terminated string for a given character and returns a pointer to its first occurrence, or zero if the terminator comes first.
- `return_zero_3`: This routine is an empty function that returns zero to the caller.
- `copy_c_string`: Copies bytes from a source string to a destination string one at a time, including the zero terminator, stopping after the terminator is written.
- `append_c_string`: Moves to the end of the destination string and copies the source string onto it, including its terminator.
- `compare_strings_up_to_count`: Compares two strings for up to a given number of characters, returning one as soon as they differ and zero when they match through the count or the terminator.
- `return_one_stub`: Returns the constant one and does no other work.
- `scan_string_with_count`: Walks a string while decrementing a count, stopping at the terminator or when the count runs out.
- `execute_empty_routine`: Library code that calls another routine and then returns.
- `copy_string_with_padding`: Copies a source string into a destination buffer up to a given length, stopping after the terminator, then zero-fills any remaining space in the buffer.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `initialize_list_header`: Initializes a list header inside a structure: it stores the header's own address plus four in its first word, clears the next word, and points the third word back at the header.
- `process_icon_request`: Returns one when the request argument is absent.
- `process_linked_entries`: Walks a linked list of entries from the given one, calling one routine with a fixed size and then a second routine with a stored pointer and a size taken from each entry.
- `verify_linked_entry_chain`: Repeatedly calls two routines while following the next-link field of a structure, and returns one when the chain ends with both calls succeeding, otherwise zero.
- `return_zero`: This routine is empty and simply returns zero to the caller.
- `allocate_and_init_entry`: Allocates a zero-filled 20-byte entry through a library routine, calls a second routine to set it up, and stores a value from a further call into one of its fields.
- `return_error_value`: This library routine returns a value of all bits set.
- `build_entry_chain`: Builds a chain of entries from a request list: for each item it allocates a zero-filled 16-byte entry through a library routine, stores a computed size and value in it, and links it to the previous entry.
- `link_new_entry_chain`: Walks a chain of entries, allocating and linking each new entry through library routines and recording a sized field for each.
- `process_linked_entry_chain`: This library routine iterates through a chain of structures, performing several calls to update fields and potentially returning negative one if any call fails.
- `return_negative_one`: Library code that returns a value of negative one.

### diskfont.library

- `check_and_set_font_handle`: Calls a library routine through a jump-table entry to obtain a block.
- `check_and_set_font_handle_2`: This library code calls a routine via a table entry using the library base.
- `open_libraries_and_init_list`: Opens the dos.library and graphics.library names through exec, and when both succeed initializes a list header inside the given structure and returns it.
- `close_library_and_free`: Calls two exec routines to close a library and free a memory block, then returns zero.
- `release_unused_font_entries`: Walks a list of font entries, unlinking each one whose count is not positive and releasing it through a library call.
- `close_and_free_when_idle`: If the structure's counter is zero, closes the two libraries, frees the structure, and returns its saved handle.
- `set_flag_bit_return_zero_2`: Sets the high bit of a flag byte in a structure and returns zero.
- `return_from_handler`: This routine is an empty function that performs no operations before returning.
- `clear_flag_and_increment_counter`: Clears the high bit of a flag byte in a structure and increments a 16-bit counter elsewhere in the same structure.
- `decrement_library_refcount_2`: Decrements a 16-bit counter in a structure.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `collect_font_entries`: Gathers font entries into an output table: when the first flag bit is set it copies names from a linked list of fonts, and when the second bit is set it scans a directory for font files, reading each file's header.
- `copy_font_list_then_scan`: Copies names from a linked list of fonts into an output buffer, then, when a flag bit is set, scans a directory for font files.
- `populate_font_metadata_list`: Iterates through a linked list of file entries to extract font metadata and copy file names into a destination buffer.
- `copy_fonts_path_and_scan`: Copies the referenced FONTS: path prefix into an output buffer and, when a flag bit is set, scans the font directory for matching font files.
- `scan_font_files_for_entries`: Walks a directory listing and, for each name ending in the font extension, reads the file header and records a matching entry in the output buffer.
- `scan_font_directory_entries`: Continues a font directory scan: it copies a file name into the output buffer, records the entry when it fits, and steps through the listing until the end.
- `read_font_directory_entries`: Copies file-name characters into the output buffer and records the matching font entry, reading each file header through a library call, then continues the directory walk.
- `finish_font_directory_scan`: Finishes a directory scan: it records any remaining font entries, releases the directory lock and scan buffers through library calls, and returns zero when everything fit, otherwise the overflow amount.
- `return_zero_stub_2`: Library code that returns zero and performs no operations on registers, memory, or hardware.
- `score_record_bit_differences`: Scores two records by comparing their bit flags: for each of seven bit positions it subtracts a table entry wherever the two records' flag bits differ, starting from a scaled difference of their 16-bit values, and returns the resulting score.
- `calculate_table_difference`: This library code iterates through a loop to subtract values from a global table based on bitwise comparisons of bytes.
- `process_table_arithmetic`: This library code performs arithmetic operations using values retrieved from a global table and a pointer passed in a register.
- `process_table_data`: This library routine performs arithmetic operations using values retrieved from a global table and a pointer passed in a register.
- `return_constant_value`: Returns a fixed constant value without using any arguments or registers.
- `select_best_font_glyph`: Iterates through a list of font entries to find the glyph with the highest score.
- `scan_fonts_directory_for_match`: Locks the FONTS: directory, then walks its font entries comparing each one with the requested font and loading any that match, releasing the lock afterwards.
- `load_font_file_entries`: Locks and reads a font file: it reads its header, allocates a 260-byte entry for each stored block, repeatedly selects the candidate with the highest score, and records each chosen entry in the font structures.
- `select_highest_scoring_entries`: Repeatedly selects the remaining candidate with the highest score, records each selected entry in the font structures, and returns the last one recorded.
- `select_best_font_entry_2`: Iterates through a list of font candidates to identify the entry with the highest score.
- `select_best_font_entry`: Iterates through a list of font candidates to identify the entry with the highest score.
- `release_selected_font_entries`: Walks the remaining font candidates, selecting and recording the highest-scoring ones, then releases the directory lock and memory blocks through library calls and returns the last entry recorded.
- `build_font_table_from_file`: Locks a font file, counts its entries, allocates a table with a 260-byte entry for each block, copies the matching entries into it, and returns a pointer to the table.
- `load_disk_font_table`: Builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `forbid_task_switching`: Calls exec's Forbid routine through a jump-table entry at a fixed offset from a global base pointer, which disables task switching.
- `permit_task_switching`: Calls exec's Permit routine through a jump-table entry at a fixed offset from a global base pointer, which re-enables task switching.
- `call_alloc_mem_vector`: Calls exec's AllocMem routine through a jump-table entry at a fixed offset from a global base pointer, passing the arguments already in registers.
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
- `copy_null_terminated_string_2`: Copies bytes from a source pointer to a destination pointer until a null terminator is copied.
- `pass_string_end_to_routine`: Advances to the zero terminator of a string and then passes that end position, with the second argument, to another routine.
- `skip_string_end_and_call`: Advances a pointer until it reaches the zero terminator of a string and then calls another routine with that pointer.
- `count_string_length_2`: Returns the number of characters before the zero terminator of a string.
- `count_null_terminated_bytes`: Adds one to the count in D0 for each byte read from a zero-terminated string, including the terminator, and returns the total.
- `compare_strings_ordered`: Compares two zero-terminated strings byte by byte and returns minus one, zero, or one depending on whether the first is smaller, equal, or larger than the second.
- `compare_null_terminated_strings_2`: This library code compares two null-terminated strings to determine their lexicographical order.
- `compare_null_terminated_strings`: Compares two null-terminated strings to determine their lexicographical order.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `search_forward_for_character`: This library routine searches forward through a null-terminated string for a specific byte.
- `find_character_in_string_2`: Scans a zero-terminated string for the character in D0 and returns a pointer to its first occurrence, or zero if the terminator comes first.
- `find_next_character_in_string`: Starts one byte past the current position and searches forward for a given character in a zero-terminated string, returning a pointer to it or zero if the terminator comes first.
- `find_last_character_in_string`: Scans a string backward from its end for a given character and returns a pointer to the last occurrence, or zero if there is none.
- `scan_backward_for_character`: Scans backward from the current position for a given byte, stopping at a lower bound, and returns a pointer to the match or zero.
- `scan_backward_for_character_2`: This library routine decrements a pointer to search backward through memory for a specific byte.
- `skip_past_path_separator`: Returns a pointer just past the first slash or colon in a name, or the original pointer when neither appears.
- `increment_returned_value`: Returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `forward_control_to_routine_2`: Forwards control to another routine in the diskfont library and returns, with no other work of its own.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `forward_formatted_text`: This library routine calls a text formatting routine via a global variable and then measures the length of the resulting null-terminated string within a fixed-size buffer.
- `call_another_routine`: This routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Defender.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
