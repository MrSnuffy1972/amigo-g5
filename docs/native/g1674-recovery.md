# Game 1674 (strikefleet (1987)(lucasarts)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01674). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 91 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 168 raw-word lines |
| Functions named and described | pass, 553 of 553 named (44 author symbols), 553 of 553 described (35 by rule, 98 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 64 of 64 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 17579 of 17615 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 479905 |
| code | 66250 |
| data | 152146 |
| padding | 24131 |
| unused | 178688 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| iconx | 12 | 806 | 37 |
| disk-validator | 1 | 386 | 1 |
| ppp.prg | 2 | 2707 | 76 |
| ppq.prg | 2 | 12605 | 365 |
| strikeiv.prg | 2 | 493 | 45 |
| sf.prg | 1 | 374 | 24 |
| Strikefleet (1987)(LucasArts)[a].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- strikefleet (1987)(lucasarts)[a]/Strikefleet (1987)(LucasArts)[a].adf:StrikeFleet/SF_MAP0.LZW: high-entropy data file (7.72 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longwords_to_buffer`: Copies sixteen consecutive 32-bit words from a fixed offset of a base structure into a caller-supplied buffer when that pointer is non-null.

### assign

- `scan_entries_fill_header_buffer`: Walks a linked list of entries to find the largest value in a chosen field, then allocates a block and fills it with ascending tag words.
- `copy_memory_block`: Copies sixteen bytes from a source address to a destination address pointed to by a stack value.

### iconx

- `start_iconx_with_argument_vector`: Program entry that creates an IconX process record, links it into a task list and opens the DOS library.
- `release_iconx_task_resources`: Swaps a new value into the current task's process record and returns the previous value.
- `open_iconx_console_window`: Handles the IconX startup path by opening a console window from a CON: specification, with a fixed delay.
- `run_iconx_wait_or_endcli`: Processes an icon request by building a command buffer and looking up a handler to dispatch it.
- `initialize_icon_library_functions`: Creates a library from its first field and registers its function table through the exec library calls.
- `build_path_from_string_nodes`: Joins the printable entries of a linked list of string nodes into a path, separated by delimiters.
- `concatenate_strings_into_buffer`: Copies one string into a destination and appends a second string to it.
- `count_string_length`: Returns the number of bytes before the terminating zero of a string.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination, one at a time, including the terminating zero.
- `advance_to_string_terminator`: Moves a pointer forward through a zero-terminated string until it reaches the terminator.
- `parse_signed_decimal_integer`: Parses a decimal integer from a zero-terminated string, accepting an optional leading plus or minus sign.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `forward_with_stack_frame_pointer`: Passes its two arguments to another routine and adds a pointer into the caller's stack frame as a third argument.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `format_stack_message_and_output`: Formats text into a 140-byte stack buffer through the exec formatting routine, then measures the result up to its zero terminator.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_raw_do_fmt_vector`: Calls the exec formatting entry through a library pointer held in global data.
- `store_byte_through_pointer`: A tiny routine that writes the low byte of a data register to the memory address held in an address register, then returns.

### disk-validator

- `validate_disk_bitmap_count`: Program entry that walks a bitmap of 32-bit words, testing each bit against a stored pattern and counting the blocks that differ.

### ppp.prg

- `start_program_and_idle_loop`: Program entry that stores pointers to global data and a handler routine, runs several setup calls, then loops forever calling two routines.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_paired_tables_wait`: Fills two tables with repeated pairs of fixed 16-bit values, then runs a nested busy-wait delay loop.
- `store_four_offset_longword_pairs`: Reads a 32-bit value from a fixed offset in a structure and stores four copies of it in global storage, each split into low and high 16-bit words.
- `combine_bitplane_mask_rows`: Copies a block of 480 32-bit words, then clears a set of rows and rebuilds them.
- `service_four_audio_channel_requests`: Checks four request flags, one per audio channel.
- `start_audio_sample_free_channel`: Starts a sound sample on a free audio channel by searching the channel table.
- `load_samples_into_audio_channels`: For each of four audio channel entries that is active and matches the current value, it copies the sample address, length and volume into that channel's hardware registers.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `open_and_read_file_block`: Opens a file through the DOS library, retrying until a handle is returned, then reads into the buffer and retries while the read reports an error.
- `close_file_and_return_value`: Closes the DOS file handle, then closes the library base, and returns the value it was given unchanged.
- `write_output_file_with_cleanup`: Opens a file through the DOS library and writes data to it.
- `open_dos_library_base`: Opens the DOS library through the exec old-open call and stores the returned base pointer in global data.
- `report_io_error_close_files`: Fetches the DOS error code, closes the file and then the library base.
- `open_file_query_then_close`: Opens a file through the DOS library and, if the handle is valid, makes a further DOS call on it and keeps the result in global data.
- `decode_keyboard_scancode_to_char`: Decodes a raw keyboard serial byte by inverting it and checking its key-up bit.
- `reset_modifier_flag_for_keys`: Clears a modifier flag when the masked key code is one of two special codes, then returns its argument unchanged.
- `install_graphics_interrupt_state`: Opens the graphics library and saves one of its view fields.
- `disable_and_restore_custom_interrupts`: Writes fixed clear masks to the DMA and interrupt control registers, restores two saved values, and then writes an enable mask.
- `handle_custom_interrupt_requests`: Interrupt handler that checks the request register.
- `poll_mouse_and_button_state`: Runs the same request path as the interrupt handler, then reads the fire and right and middle button registers.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_mouse_coordinates_within_bounds`: Converts the change in a mouse counter into signed movement on two axes, adds it to the stored coordinates and clamps each to fixed screen limits.
- `update_mouse_axes`: Calculates the difference between a current value and a previous value to update and clamp two separate coordinate variables.
- `store_mouse_counters_clamp_position`: Stores a new mouse counter value and adds a signed step to the horizontal position, with the sign chosen from a flag in the second register.
- `update_vertical_mouse_from_port`: Computes the signed change in the mouse port's vertical counter and adds it to the stored vertical position.
- `store_horizontal_position_with_clamp`: Stores an incoming horizontal value and adds a signed step to it, with the sign chosen from a flag.
- `clamp_position_to_screen_limits`: Clamps a vertical coordinate between a sprite-derived minimum and a fixed upper limit.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `forbid_task_switching`: Calls the exec Forbid routine to stop task switching, and returns its arguments unchanged.
- `swap_state_wait_for_interrupt`: Disables one interrupt source, swaps two saved 32-bit values and sets a ready flag.
- `wait_for_request_bit_enable`: Busy-waits until the request bit for one interrupt source is set, then writes the enable mask that re-arms that source.
- `match_record_parse_numeric_fields`: Compares a fixed-length input string against a stored record.
- `match_record_store_numeric_fields`: Compares a fixed-length input string against a stored record.
- `parse_string_data`: Compares a string against a global value and, if they match, extracts several numeric and pointer values from a table.
- `match_record_store_numeric_fields_2`: Compares a fixed-length input string against a stored record.
- `match_title_parse_fields`: Compares input against a stored title text and, on a full match, decodes fixed decimal and longword fields into global variables.
- `read_ring_buffer_or_reset`: Takes the next entry from a 32-entry ring buffer when it differs from the write index.
- `store_argument_in_context_slot`: Stores its stack argument into a fixed 32-bit slot of a global context structure and returns nothing.
- `reset_machine`: Executes the processor reset instruction and then jumps to the reset vector, restarting the machine.
- `setup_blitter_copy_for_sprite`: Waits for the blitter to finish, then programs its first and last word masks, modulo, source and destination pointers and blit size for a rectangular copy.
- `start_blitter_copy_stored_pointers`: Waits for the blitter to finish, then programs its masks, control registers, pointers and blit size from values stored in the context structure, and starts the blit.
- `load_table_entry_accumulate_totals`: Looks up an entry in a fixed table and scales two of its fields by 6000.
- `lzss_decompress_into_buffer`: Decompresses an LZSS-style stream into an output buffer.
- `advance_timed_sequence_object`: Updates a sequence object by counting down several timers in its record.
- `format_three_digits_blank_padded`: Writes a 16-bit number as three decimal digit characters into a buffer, splitting it into hundreds, tens and units.
- `render_sixteen_column_bitplane_tiles`: Draws a grid of tiles into four bitplanes: for each of twenty rows and sixteen columns it reads a source byte, masks it through a lookup table and writes four bytes into planes spaced 8000 bytes apart.
- `update_entry_highlight_flags`: Scans two lists of entries and sets a highlight flag on each entry whose stored value matches the current selection, clearing it otherwise.
- `advance_cyclic_selection_index`: Advances a cyclic selection of fourteen positions by an offset taken from two counters.
- `select_sequence_and_run_steps`: Selects a sequence record from a table when the index is valid, then runs its steps.
- `start_sequence_at_index`: Stores a chosen sequence index, loads the first entry pointer and then runs the same frame-synced step loop until the entry is zero.
- `add_decimal_digit_text`: Adds a constant numeric text to a stored string of decimal digits, from the least significant digit upward with carry.
- `subtract_constant_digits_in_place`: Subtracts a fixed table of digit values from a seven-digit numeric text in place, borrowing between digits.
- `decrement_numeric_string`: Subtracts a value from each digit in a seven-character numeric string, handling borrows and character offsets.
- `run_frame_synced_sequence_steps`: Loops over a sequence of entries, waiting on the frame counter before each step, until the next entry is zero.
- `draw_text_string_to_bitplanes`: Draws a zero-terminated string into four bitplanes one character at a time, using an eight-row glyph table.
- `advance_list_pointer_and_repeat`: Advances a list pointer and, when it reaches a sentinel value, rewinds it to a saved position.
- `run_seven_calls_three_times`: Calls a subroutine seven times in each of three passes, with no other work.
- `fill_panel_rows_on_change`: When a tracked state value differs from its previous value, it counts the change and calls a subroutine for the first few changes.
- `fill_plane_block_with_byte`: Fills a rectangle of one bitplane with a constant byte, eight columns wide and seventy-two rows tall, or seven columns wide for one index.
- `render_stats_screen_with_labels`: Draws a statistics screen by copying a data block into the bitplanes and clearing a table.
- `read_planes_to_interleaved_buffer`: Copies eighty-four rows of four bitplane words, spaced 8000 bytes apart, into a contiguous buffer, interleaving the planes as it goes.
- `write_interleaved_buffer_to_planes`: Copies a contiguous buffer back out to four bitplanes spaced 8000 bytes apart.
- `copy_screen_buffer_and_permit`: Copies a stored 32-longword block and then copies four bitplanes to the display buffer, clearing the remainder.
- `setup_screen_buffers_and_state`: Records a key code and a status word, normalising one specific key code.
- `render_glyph_data`: Iterates through a string to either clear a buffer or write bitplane data for specific characters using a lookup table.
- `draw_masked_tile_block`: Draws a block of tiles into four bitplanes, four tiles wide and eight rows deep.
- `build_sprite_mask_and_rows`: Builds a sixteen-entry mask by rotating a word, sets a limit from the item's coordinates, and copies plane data for each sprite row into a buffer.
- `read_input_update_control_record`: Reads the fire buttons and a stored input state, then updates a control record's coordinates and frame counter.
- `write_sprite_control_words`: Computes a sprite's screen position from stored coordinates and writes its position and control words into two sprite buffers.
- `set_sprite_frame_advance_counter`: Looks up a frame value from a table by index and stores it in the record.
- `set_alt_sprite_frame_advance`: Looks up a frame value from a second table by index and stores it in the record.
- `check_overlap_advance_sprite_frames`: Scans a list of sprite records and skips any that lie outside a rectangle around a given point.
- `load_sprite_frame_longwords`: Copies an eight-byte frame entry from a table into a record as two longwords, then calls a subroutine.
- `shift_and_merge_plane_rows`: Shifts four source bitplane words right by a bit count and merges them into the destination planes through a mask.

### ppq.prg

- `copy_block_and_loop_forever`: Program entry that copies six longwords from a source block into a fixed area, then stores pointers to global data and a handler.
- `initialize_data_tables`: Fills a table with repeating word pairs, performs a long decrementing loop, and populates a series of global variables with calculated offsets and values.
- `copy_indexed_table_row`: Copies twelve 16-bit values from a row of a table, selected by a stored index, into a strided global buffer.
- `initialize_plane_pointers`: Sets multiple global variables to point to the same memory address and its subsequent offsets.
- `combine_bitplane_mask_rows_2`: Copies a block of 768 32-bit words, then clears a set of rows and rebuilds them.
- `update_interrupt_flags`: Checks specific bits in a global variable and updates it with predefined values based on bitwise conditions.
- `manage_active_entries`: Searches a list for an entry that does not match a specific value and, if found, initializes it with new data and pointers.
- `transfer_audio_parameters`: Compares a value against a list of entries and, if a match is found, copies parameters to hardware registers and updates a status flag.
- `execute_indirect_callback_chain`: Retrieves a pointer from a data structure and executes a sequence of indirect function calls.
- `manage_file_operations`: Uses library functions to open, read, and close files while handling return values in a loop.
- `handle_file_error`: Checks a return value from a function and performs library calls to close resources or handle errors based on the result.
- `open_dos_library_2`: Calls a function from a library retrieved via a pointer in a global variable.
- `close_library_resources`: Calls several functions using pointers stored in a global variable and a stack value.
- `handle_library_error`: Attempts to execute a function through a pointer and, if unsuccessful, calls several other functions to clean up resources.
- `process_keyboard_input`: Manipulates a global variable using bitwise operations and compares the result against specific character values.
- `check_byte_mask`: Checks if the lower seven bits of a byte match specific values and sets a global variable to zero if they do.
- `initialize_system_libraries`: Opens a library, retrieves a pointer from it, and updates several global variables and hardware registers including interrupt and DMA control registers.
- `configure_hardware_registers`: Writes specific values to hardware registers and global variables to manage interrupts and DMA.
- `verify_five_longword_signature`: Compares five longwords of stored data against five longwords taken from the routine's own code area, and loops forever on any mismatch.
- `verify_code_signature`: Compares five longwords from a table against five longwords in the current code area, entering an infinite loop if any mismatch is found.
- `process_interrupt_status`: Checks hardware status bits and, if certain conditions are met, updates a global variable and a table entry before calculating a value based on a global variable.
- `update_position_data`: Calculates a difference between a 16-bit value from a pointer and a global variable, then stores the result along with bitwise flags into a global variable.
- `sample_mouse_buttons_and_position`: Reads the fire and right and middle button registers and stores the button bits in global data.
- `update_clamped_values`: Calculates differences between a global variable and other values, applies a signed offset, and stores the result in global variables after clamping them within specific ranges.
- `update_clamped_values_alt`: Performs arithmetic on global variables using differences from another value and stores the results in clamped ranges.
- `process_dual_clamping`: Performs arithmetic to update two separate global variables using differences from other stored values and applies clamping to both.
- `update_multiple_clamped_values`: Updates two different global variables by calculating differences from other values and applying range clamping.
- `update_clamped_position`: Calculates the difference between a global variable and a value from a hardware register, then updates a clamped global variable.
- `clamp_value_range`: Adjusts a value by adding an offset and then constrains it within a specific minimum and maximum range.
- `clamp_variable_range`: Clamps a value within a specific range by comparing it against constant bounds and a value from a global variable.
- `wait_for_flag_clear`: The routine spins until a global variable no longer has a specific bit set, then returns.
- `permit_task_switching`: Calls the exec Permit routine to allow task switching again, and passes its arguments back unchanged.
- `forbid_execution`: Calls a function through a pointer retrieved from a global variable.
- `swap_and_wait_interrupt`: Swaps two values in a table, writes a value to a memory location, and waits in a loop for a specific bit in a global variable to be set.
- `wait_for_interrupt_flag`: Enters a loop that waits for a specific bit in a global variable to be set before updating that variable.
- `parse_data_string`: Compares a global variable to a string, and if a match is found, parses numeric values from the string into several global variables.
- `parse_data_string_alt`: Identifies a matching string and extracts numeric and pointer data from the string into various global variables.
- `parse_data_string_2`: Compares a global variable against a string in a table and, if a match is found, extracts several values from the table into global variables.
- `extract_string_values`: Identifies a matching string in a table and populates multiple global variables with parsed numeric values and pointers derived from that table.
- `parse_string_table`: Compares a global variable to a string and, upon a match, extracts numeric values and pointers from the string into several global variables.
- `increment_table_index`: Increments a global index and uses it to fetch a value from a table, potentially triggering a reset if a specific condition is met.
- `increment_table_index_2`: Increments a global index used to fetch a function pointer from a table and checks for a specific reset condition.
- `store_stack_value`: Stores a value from the stack into a memory location offset from a register.
- `execute_reset`: Calls a reset routine and then another routine.
- `dispatch_exec_version_or_reset`: Compares a word from the exec library base against a threshold.
- `call_dos_then_reset`: Makes a DOS library call, then resets the machine and jumps through a table that the decompiler could not recover.
- `reset_then_dispatch_through_table`: Resets the machine and then jumps through a table that the decompiler could not recover.
- `compute_bitplane_base_addresses`: Looks up a bitmap through the graphics library base and stores its first plane address and a second base address.
- `initialize_graphics_pointers`: Retrieves a pointer from a system structure and calculates several offsets and values based on that pointer to store in global variables.
- `store_upper_limit`: Calculates a value by adding a constant to a pointer retrieved from a system structure and stores it in a global variable.
- `store_base_pointer`: Retrieves a pointer from a system structure and stores it in a global variable.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `store_first_offset`: Calculates the difference between a calculated upper limit and a base pointer and stores the result in a global variable.
- `store_second_offset`: Calculates the difference between a second calculated upper limit and a base pointer and stores the result in a global variable.
- `store_final_offset`: Calculates the difference between a final calculated upper limit and a base pointer and stores the result in a global variable.
- `open_graphics_setup_planes`: Opens the graphics library by name and sets up the bitplane addresses from its bitmap.
- `initialize_control_block_and_tables`: Sets default sizes and clears counters in a large control block, and fills two sprite tables with the value minus one.
- `unpack_and_blit_image`: Fills a memory area with a constant pattern and then unpacks bitstream data into bitplanes at a destination address.
- `copy_packed_image_into_planes`: Stores values for each entry of a pointer table, then copies a 4000-word block from a source into four bitplanes spaced 8000 bytes apart.
- `empty_routine_returns_immediately`: Does nothing and returns straight away.
- `run_record_draw_loop`: Sets up a drawing record from a block of fields and chooses a buffer size of 32000 or 64000 bytes from a flag.
- `merge_source_bits_into_column`: Walks a column of source bytes, looks up a mask for each one and merges the result with the existing values in four bitplanes.
- `merge_bitplane_data`: Iterates through a source buffer to perform bitwise operations and merge data into a destination table.
- `update_timing_totals_for_mode`: Updates timing totals from two counters, scaling them by 6000 or 3000 depending on the mode.
- `run_frame_synced_status_loop`: Runs a frame-synchronised loop that toggles status words and calls subroutines, advancing in steps of three frames.
- `run_status_update_if_done`: Returns at once if a countdown field has not reached three.
- `run_status_update_sequence`: Calls a fixed sequence of subroutines, sets a status word from a stored value, toggles a flag and clears a countdown field.
- `check_record_bounds_and_mark`: Compares a record's position against the current coordinates and a size window.
- `run_main_update_state_machine`: Runs the frame's main state machine.
- `copy_record_fields_and_wrap`: Copies position, size and flag fields from one record into another and clears the destination's active flag.
- `wrap_record_coordinates_modulo_375`: Wraps the two stored coordinates of a record by a modulus of 375, adjusting each by a supplied offset.
- `blank_character_cell_block`: Blanks a block of character cells by writing the space character at several offsets from a base pointer, stepping across rows.
- `build_status_text_line`: Builds a status text line in a text buffer by blanking the cell area and copying a label string, followed by a colon and a second string.
- `store_index_pair_for_record`: Stores its argument into a record field.
- `scan_entries_until_row_match`: Initialises a scan record and repeatedly calls a subroutine to read the next entry.
- `swap_record_index_pair`: Stores a new index and, when it is non-negative, swaps it with the previously stored index.
- `traverse_linked_records`: Walks a chain of records using relative offsets and compares values against a global variable until a condition is met.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 0 into d5, then loads 4 into d7.
- `repeat_calls_with_masked_counter`: Repeats a loop for a given count, calling subroutines on each pass and advancing a masked counter by one each time.
- `call_subroutine_and_return`: Calls a single subroutine and returns.
- `store_byte_in_indexed_slot`: Writes a byte into a field of a table entry.
- `build_labeled_value_text_line`: Blanks a text area, then copies a label, a colon and a second text string into it.
- `format_decimal_string`: Converts a numeric value into a three-digit ASCII string representing hundreds, tens, and units.
- `format_three_digits_zero_padded`: Writes a 16-bit number as three decimal digit characters by adding each digit to a template of zero characters.
- `format_two_digits_zero_padded`: Writes a 16-bit number as two decimal digit characters by adding each digit to a template of zero characters.
- `format_four_digits_blank_padded`: Writes a 16-bit number as four decimal digit characters into a buffer, splitting it into thousands, hundreds, tens and units.
- `copy_planes_clear_text_rows`: Copies a four-plane image into the screen buffers and clears a block of text rows.
- `build_bit_spread_lookup_tables`: Builds two lookup tables: the first holds bit-spread patterns for byte values, and the second holds Gray-code bit patterns for 512 values.
- `blank_text_rows_with_spaces`: Writes space characters across a block of text rows, with the number of rows chosen from a stored flag.
- `fill_memory_with_pattern`: Fills a range of memory with a constant value based on a condition checked in a global variable.
- `copy_text_entries_into_rows`: Copies a list of text entries into a text buffer, writing each character and its attribute byte.
- `render_text_cells_four_passes`: Renders a block of text cells across four passes, combining glyph bit patterns from lookup tables into the bitplanes and clearing cells as it goes.
- `update_timing_totals_for_mode_2`: Updates timing totals from two counters, scaling them by 6000 or 3000 depending on the mode.
- `render_text_to_bitplanes`: Processes a string of characters, either clearing bitplanes or fetching character data from a table to write into bitplanes.
- `wait_for_frame_count`: Busy-waits until the frame counter has advanced by the requested number of frames.
- `draw_status_string_with_update`: Copies a pending value into a display slot when a flag is set, then draws a text string into the bitplanes glyph by glyph using a stored mask.
- `initialize_display_planes_from_table`: Copies a 32-longword header and an image of about 1840 longwords into the bitplanes, spaced 8000 bytes apart.
- `render_string_to_bitplanes`: Iterates through a string to either clear bitplane data or write character patterns into bitplanes using a lookup table.
- `run_frame_update_for_display`: Per-frame update that records the frame counter, syncs state fields with their pending values and calls twenty-two subroutines to refresh the displayed items.
- `draw_current_item_entry`: Checks a per-item flag and, depending on whether the current position matches the stored one, selects and draws an entry.
- `composite_masked_plane_block`: Composites a block of 10 by 7 words from source planes into the destination.
- `draw_text_string_masked`: Draws a zero-terminated string into four bitplanes glyph by glyph, combining each glyph byte with the existing plane bytes through a mask.
- `draw_shifted_item_graphic`: Computes a screen address from the item's position and draws its graphic into the bitplanes.
- `update_draw_visible_object_list`: Updates the list of visible objects by moving a stored value halfway toward its target each time.
- `draw_object_records_loop`: Walks a list of object records, drawing each one through a subroutine call with its position and frame fields.
- `clip_rectangle_to_screen_marks`: Tests whether a rectangle lies inside the screen bounds and returns a failure value when it is fully outside.
- `traverse_and_draw_object_records`: Walks a linked chain of object records, dividing a total by twenty-two to set a count, and draws each record through subroutine calls.
- `blend_two_values_by_fraction`: Splits a stored value between two fields using a fraction derived from a byte.
- `project_object_into_draw_record`: Projects an object's size and position into a draw record.
- `project_object_to_record_alt`: Projects an object's size and position into a draw record, using a different scale choice for two special frame types.
- `project_flagged_object_to_record`: Projects an object whose flag bit is set into a draw record.
- `draw_object_list_with_clips`: Walks the list of object records and draws each one that passes a clip test, calling subroutines for the clip and draw steps.
- `clip_rectangle_wide_bounds`: Tests whether a rectangle lies inside a set of bounds and returns a failure value when it is fully outside.
- `step_heading_lookup_value`: Counts down a per-object timer and advances an index through a lookup table each time it expires.
- `fill_plane_block_color_one`: Fills a rectangular block of a four-plane screen buffer, forty bytes per row, by setting the first bitplane to ones and clearing the other three, which gives every pixel colour index one.
- `initialise_object_state_and_update`: Sets a per-object state block to fixed start values: a counter of one, a flag word, and two cleared fields.
- `copy_bitplane_rows_between_buffers`: Copies rectangular blocks of four-plane bitmap data row by row from a source buffer into a destination buffer, moving each bitplane at its own fixed offset.
- `scan_object_list_and_mark`: Walks a chain of object records until its counter runs out, skipping the record whose index matches a stored value.
- `dispatch_cell_handler_by_position`: Tests whether a position lies inside a window, converts it into a grid cell using two base values, and reads a handler code from that cell.
- `queue_block_draw_pass_set`: Issues four passes of parameter words to one shared memory-mapped target, advancing the source and destination pointers by one block width on each pass.
- `queue_draw_passes`: Sets up a series of parameters in a global structure and calls another routine multiple times in a loop.
- `queue_offset_block_draw_passes`: Issues four passes of parameter words to the same shared target as its neighbours, but with a different vertical offset and a different control word.
- `queue_strip_draw_commands`: Writes parameter-word groups for a strip of blocks to a shared target.
- `reset_state_vblank_paused`: Clears two per-object state fields, then turns off the vertical blank interrupt, calls two helper routines, and turns it back on.
- `increment_frame_counter`: Adds one to a 16-bit frame counter stored in the current game-state block and returns with the register values unchanged.
- `poll_fire_button_input`: Increments the frame counter, then reads the fire-button register and the right and middle button register.
- `process_fire_button_state`: Stores the current fire input in a global and clears the held-fire state when the input matches a stored value.
- `fire_projectiles_from_free_slots`: When fire input is active and the trigger is ready, sets a trigger count to eight and fills free entries of a seven-slot projectile table.
- `launch_projectile_group_unchecked`: Runs the same projectile filling logic as its neighbour, but without the zero-flag test on entry.
- `spawn_projectile_group_secondary`: Handles the other branch of the fire-trigger case, computing spawn values from the owner's position and a lookup.
- `fill_projectile_slots_with_coords`: Sets the trigger count to eight, then writes a new projectile record into each free slot of a seven-entry table.
- `write_projectile_record`: Computes the position and angle components of one projectile from the owner's coordinates and a scaled lookup.
- `claim_free_projectile_slot`: Searches a table of seven-word projectile records for the first record whose first word is the free marker.
- `compute_object_display_position`: Computes an object's on-screen position from its stored coordinates, using a scaled lookup for the vertical component.
- `write_display_position_words`: Takes a position from the input registers, scales it through a lookup, and writes the packed position words into one of two display entries chosen from the object's state.
- `enable_sprite_dma_entry`: Writes the object's two position words into its sprite display entry and records that entry's address in globals.
- `run_phase_step_handler`: Runs the handler for the object's current phase.
- `check_rectangle_target_hit`: Runs one phase step for the object.
- `find_point_inside_rectangle_entry`: Walks a list of rectangle entries, each with four bounds, until it finds one that contains the given point.
- `check_object_overlap_box`: In phase four, and only while the object is below a fixed coordinate limit, it builds a bounding box around the object.
- `dispatch_indexed_handler_and_store`: Calls a handler chosen by the index in the data register and notes whether a follow-up flag is set.
- `prepare_object_record_pointer`: Resets a state counter.
- `dispatch_phase_handlers_and_test`: Calls one or two phase handlers from a pointer table, depending on the object's phase number.
- `call_phase_handler_box_test`: Calls the phase handler selected by the index in the input value, then in phase four and within a coordinate limit builds the object's box.
- `advance_object_phase`: Advances the object's phase counter, or resets it to zero when the input value is eight.
- `increment_phase_and_store_values`: Issues a fixed control word, increments the object's phase counter, and clears a timer field.
- `find_overlapping_target_entry`: Builds a bounding box around the object, issues a control word, and walks a list of records until an active one overlaps the box.
- `scan_records_for_box_overlap`: Issues a control word, then builds a bounding box for the object and walks the record list.
- `mark_phase_started_and_advance`: Sets a started flag and calls a helper, then issues a control word.
- `match_box_against_record_list`: Issues a control word, builds the object's bounding box, and tests it against each record's bounds in turn.
- `scan_records_in_region`: Writes two fixed control values and a clear word, then walks a list of records.
- `load_target_and_advance_phase`: Copies a target value from the input record into the object state, sets a flag, increments the object's phase, and clears a timer field.
- `copy_four_plane_segments`: Copies eight consecutive forty-byte segments from each of the four bitplanes between two screen buffers.
- `dispatch_by_code_table_entry`: Returns at once for a zero code.
- `call_handler_for_code_group`: Calls one of two handlers from a table, choosing the first when the code is twelve or twenty-four and the second otherwise.
- `call_fixed_table_handler`: Calls the handler stored at one fixed offset in a pointer table, with no conditions.
- `call_handler_at_offset`: Calls the handler at the table offset given in the data register.
- `call_positioned_handler_from_record`: Looks up a handler through a pointer in the object record and calls it with the record's position field added to a base value.
- `call_current_entry_handler`: Calls the handler named in the entry that the object state currently points to, passing that entry's stored offset plus the object's base value.
- `activate_object_and_record_position`: Resets a counter, and when the object is active it points at the matching record and sets a mode value from the input.
- `build_glyph_list_from_text`: Copies a text string up to a colon separator into a buffer and measures it.
- `advance_record_cycle_and_lookup`: When the object is active it advances a small cycle index stored in the record by the input value, wrapping back to zero above four.
- `resolve_status_string_length`: Selects a status string based on a phase byte and calculates its length.
- `toggle_flag_and_lookup_key`: When the object is active it toggles a flag byte in the record.
- `repeat_helper_twenty_times`: Clears a state flag, calls a helper twenty times in a loop, and then restores the flag.
- `call_three_helpers_in_order`: Calls three helper routines one after another and returns the value passed through.
- `call_two_helpers_in_order`: Calls two helper routines one after another and returns the value passed through.
- `call_one_helper_passthrough`: Calls one helper routine and returns the value passed through unchanged.
- `dispatch_drawing_tasks`: Calls two separate routines to queue parameter sets for memory-mapped drawing operations.
- `queue_draw_parameters`: Calls a routine to prepare and queue four sets of parameter words for a memory-mapped drawing operation.
- `queue_block_draw_parameters`: Prepares and queues four sets of parameter words for a memory-mapped drawing routine.
- `call_helper_return_data_regs`: Calls one helper routine and returns the two data register values that were passed in.
- `call_helpers_with_saved_field`: Saves a field in the object state, sets it to a value taken from the address register, and calls two helpers.
- `call_indexed_handler_table`: Calls one entry of a pointer table, selected by the value in the data register and stepped in groups of four bytes.
- `toggle_flag_and_draw_text`: Toggles a flag in the object record while it is active, and looks up a matching key when the record's sentinel is set.
- `select_and_draw_label_text`: Chooses one of two text strings from the program data according to a flag in the record, skips leading spaces, and builds the glyph pointer list needed to draw it.
- `draw_fixed_label_text`: Measures a fixed text string from the program data, builds a glyph pointer for each character, and positions the string by its accumulated width.
- `format_and_draw_text_buffer`: Writes a fixed character and a terminator into a text buffer, skips leading spaces, and builds the glyph pointer list for the resulting string, positioned by its accumulated width.
- `draw_text_from_pointer`: Measures a text string passed in through the address register, builds a glyph pointer for each character, and positions the list by its accumulated width.
- `update_status_display_string`: Selects a status string based on a phase byte and calculates its length.
- `draw_status_label_text`: Chooses a status string according to a phase byte in the record, using a fixed label for phases zero and four.
- `draw_flag_text_glyphs`: Draws a text label chosen by the input value: a fixed label for one code, and otherwise a short two-character string written into the buffer.
- `draw_two_char_buffer_label`: Writes a two-character label into a buffer at the record, then builds the glyph pointer list for it, positioned by its accumulated width.
- `format_signed_number_text`: Takes the absolute value of the input number and writes it as decimal text.
- `clear_text_buffer_and_draw`: Writes a terminator at the start of a text buffer, then builds the glyph pointer list for the record's text, positioned by its accumulated width.
- `format_number_field_text`: Takes a number from one field of the record, corrects its sign, converts it to decimal text in a buffer, and builds the glyph pointer list for drawing it.
- `format_record_count_text`: Formats a count field from the record as decimal text, with the sign corrected first.
- `format_signed_field_text`: Formats a signed field from the record as decimal text.
- `format_record_value_text`: Takes a value from a second field of the record, corrects its sign, and converts it to decimal text in a buffer.
- `compare_distance_scaled_roll`: Checks whether the record's target is still eligible.
- `draw_bit_pattern_and_digits`: Toggles one bit in a bitplane byte, sets several bytes of a screen pattern, and writes up to three decimal digits of a value into a text buffer.
- `write_three_digit_value`: Writes a value as decimal digits into a text buffer, skipping leading digits that are zero, and updates a few screen bytes first.
- `write_decimal_digits`: Writes an unsigned value as decimal digits into a text buffer, skipping leading zero digits and always writing at least a units digit.
- `write_one_or_two_digits`: Writes a value as one or two decimal digits into a text buffer, writing a tens digit only when the value is above nine.
- `draw_text_centred_in_field`: Measures a text string from the address register, builds a glyph pointer for each character, and positions the list centred in a field using the accumulated glyph width.
- `render_glyph_list_to_bitplanes`: Draws a list of glyphs into four-plane screen memory.
- `draw_glyph_list_with_shift`: Draws a fixed number of glyphs from a list into four-plane screen memory, shifting each glyph's rows by a horizontal sub-byte offset and combining them into the planes.
- `render_glyph_strip_bitplanes`: Draws a strip of glyph rows into four-plane screen memory, counting down a glyph count and shifting each row by its offset before combining it with the planes.
- `return_immediately`: This routine is empty and performs no operations before returning.
- `refresh_grid_cell_cache`: Converts the object's coordinates into grid cells by dividing by a fixed scale.
- `update_edge_span_deltas`: Calculates and stores coordinate deltas for edge spans by performing fixed-point arithmetic and scaling based on object properties.
- `calculate_polygon_vertex_offsets`: Calculates and stores scaled vertex coordinates for a polygon by applying fixed-point multiplication to distance and slope values.
- `refresh_grid_cell_cache_2`: Converts the object's coordinates into grid cells by dividing by a fixed scale.
- `build_pairwise_range_table`: Fills a fifteen-by-fifteen table with projection values for pairs of entries from two lookup tables.
- `compute_pairwise_projection_table`: Iterates through two sets of data using nested loops to calculate projection values for every pair of entries.
- `advance_stepped_fill_sequence`: When its step counter reaches eight it clears a set of columns in screen memory to the fill value and resets its accumulator.
- `scan_index_list_for_hits`: Walks a list of entry indices and, for each entry not yet marked, marks it and examines three related entries.
- `sort_triplet_and_test_edges`: Orders three values from a table by size, then checks their flags and calls a helper when the qualifying pair is found.
- `compute_edge_span_deltas`: Computes the horizontal and vertical differences between two entries and sets up the values used for a span fill, taking the sign of each difference into account.
- `fill_clamped_vertical_span`: Clamps a vertical range to a fixed window and writes a fill value into a column of screen rows across the four bitplanes, stepping forty bytes per row.
- `build_row_offset_table`: Fills a table of two hundred words with the byte offset of each screen row, stepping forty bytes per row.
- `poll_input_and_move_object`: Increments a frame counter and reads the fire-button and secondary-button registers.
- `update_display_state_with_dma`: Updates the frame's display state by recording the current fire value and advancing the phase counter while fire is active.
- `advance_phase_modulo_ten`: Sets a record field from a table indexed by the input value, negating a second field for one specific input.
- `update_entity_animation_state`: Increments an animation frame counter and updates entity position and attribute structures based on specific state conditions.
- `update_record_near_point`: Searches a list of records for the first active one whose stored position lies within a small box around the input point.
- `spawn_display_entry_when_armed`: When the armed flag is set and the phase matches, it builds new entries from the object's coordinates across a list and calls helper routines to finish each one.
- `load_mask_pair_from_table`: Copies a pair of 32-bit mask values from a table into the object state.
- `composite_masked_bitplanes`: Combines source bitplane words with the destination in groups of four, using a stored mask and shift.
- `copy_sprite_entries_from_list`: Reads a list of display entries terminated by a negative word.
- `populate_entity_data_list`: Iterates through a source data structure to populate an array of entity records with positional, attribute, and sprite-related information.
- `populate_entity_attribute_structures`: Iterates through a list of data entries to extract coordinate, sprite, and property information into a series of target structures.
- `populate_entity_records`: Iterates through a source list to extract coordinate, property, and attribute data into a destination table of records.
- `claim_empty_record_slots`: Walks sixteen records of 96 bytes each and sets a word field to the input value in every record whose field is zero, so every free record is claimed.
- `walk_record_chain_for_mask`: Walks a chain of records linked by a relative offset, stopping at the first record whose mask word passes the flag test.
- `search_linked_list`: Traverses a linked list of structures to find a node that matches specific bitmask and value criteria.
- `search_linked_list_with_criteria`: Traverses a linked list of structures, evaluating each node against a set of bitmask and value comparisons.
- `start_blitter_block_copy`: Waits for the blitter to be free, then programs its mask, modulo, control and size registers for a block copy.
- `start_blitter_masked_sprite`: Waits for the blitter to be free, programs its mask words, modulo and size registers, and starts a masked block copy using two pointers from the object record.
- `compute_column_bit_mask`: For an x coordinate inside the range of minus fifteen to 319, computes a shift value and a bit mask for one column.
- `compute_column_bit_mask_alt`: Works like its neighbour for an x coordinate inside the range of minus fifteen to 319, but stores the computed mask and shift in alternate words.
- `populate_blit_parameter_structures`: Calculates memory offsets and bitmask values based on a global index to populate a series of data structures.
- `populate_blit_parameter_structures_2`: Calculates memory offsets and color data based on a global index and populates a series of data structures in a loop.
- `compute_column_bit_mask_alt_2`: For an x coordinate inside the range of minus fifteen to 319, computes a shift value and bit mask for one column.
- `program_column_strip_blits`: Computes column mask values for an x coordinate, then waits for the blitter and programs its registers for a set of strip blits.
- `queue_plane_blit_sequence`: Runs four passes that each program the blitter with a mask pair, a control word and a size word.
- `queue_clipped_blit_sequence`: Computes a start offset from a state value, using a default when that value is negative.
- `queue_fixed_blit_sequence`: Sets fixed start offset and size words, then runs four blit passes, advancing one bitplane per pass and waiting for the blitter before each start.
- `queue_two_stage_blit_sequence`: Issues one blit with a first set of offsets, then runs the four-pass blit loop with a second set of offsets.
- `queue_scaled_blit_sequence`: Sets the start offset and size words from a state value, runs one blit, then runs the four-pass loop with offsets computed from the same value.
- `run_four_plane_blit_loop`: Waits for the blitter, then runs four passes that program mask, control and size words from preset values.
- `render_tile_rows_to_screen`: Walks a range of tile columns and rows between two bounds, calling helper routines to draw each cell into the screen buffer.
- `render_graphics_frame`: Performs complex graphics operations including buffer swapping, line span table construction, and timing synchronization for frame rendering.
- `copy_planes_and_clear_rows`: Copies a block of four-plane screen data from one buffer into four working planes, then fills a band of rows with all-ones words.
- `swap_buffers_after_vblank`: Swaps the two buffer pointers stored in the object record and marks the record as updated.
- `copy_working_planes_to_screen`: Copies four working planes back into a block of four-plane screen data, reversing the direction of the copy used by its neighbour.
- `build_scroll_column_mask`: Clears a block of ten state words, then uses a helper value to place a bit-pattern byte in a column buffer and decrement a run of bytes.
- `clamp_view_window_to_object`: Chooses window limits from a mode flag, then clamps the object's coordinates into that window.
- `scan_targets_with_weighted_score`: Initialises a search state, then loops over candidate records and computes a weighted score from a byte in each record and a table value.
- `update_record_heading_toward_target`: Takes a target value from the record or from a helper, then moves the record's stored value toward it by a step limited by a speed shift.
- `apply_scaled_axis_step`: Looks up a table value for a direction, multiplies it by an input factor, and adds the shifted result to the record's two position fields.
- `run_sixteen_step_division`: Takes the absolute differences between an input point and the record's position, then runs a sixteen-step shift-and-subtract loop that produces a fixed-point ratio of the two.
- `perform_fixed_point_division`: Calculates the quotient and remainder of two values using a loop-based division algorithm.
- `calculate_fixed_point_division`: Computes a fixed-point division using a sixteen-step loop and performs a large integer multiplication on the result.
- `clear_entry_fields_per_record`: Walks a list of entries until a negative terminator.
- `render_tile_entries_to_planes`: Walks a list of tile entries terminated by a negative word.
- `build_line_span_table`: Steps a fixed-point line from one end to the other, writing an index and an accumulated position into a list for each step.
- `select_target_index_for_update`: Checks whether the object's current entry has changed since the last call.
- `select_target_index_reset`: Clears the change flags, then returns the current entry index when it has changed.
- `check_target_status_code`: Returns zero for an entry with no status, the status code from the entry when a shared condition is met, and a fixed code otherwise.
- `resolve_target_status_value`: Retrieves a status value from a target object's data structure, accounting for nested or indirect references.
- `check_target_status_flags`: Reads the entry's status from one of two sources and returns zero when it is empty.
- `execute_null_call`: This routine is empty and performs no operations.
- `start_target_sequence_from_table`: When the entry's status is nonzero and the state allows it, scans a table for an entry whose code is not six.
- `initialize_target_sequence`: Checks specific status flags and global conditions to determine if a new entry should be initialized.
- `nop_routine`: This routine is empty and performs no operations.
- `reset_sequence_phase_flag`: Clears one word in the object state, which resets the sequence phase flag, and returns.
- `append_entry_to_sequence_table`: Looks for a free slot in a table of up to thirty-two entries.
- `advance_sequence_state_machine`: Runs a four-phase sequence.
- `render_text_row_to_bitplanes`: Draws a null-terminated text string into four-plane screen memory one character cell at a time.
- `format_long_as_decimal`: Converts a 32-bit value into decimal text by repeated subtraction against a table of powers of ten.
- `format_and_update_display_buffer`: Converts a sequence of integers into a decimal string representation within a buffer, handling negative signs.
- `render_text_to_bitplanes_2`: Iterates through a string of characters to write pixel data into bitplanes.
- `tick_timed_record_scan`: Subtracts a speed-scaled amount from a timer and, when the timer falls below its threshold, lowers the threshold and runs a record search with a mask.
- `update_state_and_scan`: Decrements a value in a structure and, if it falls below a threshold, performs a series of state updates and enters a loop that iterates through a list of short values, negating the first negative value found.
- `scan_entries_negating_first_negative`: Scans eight word entries for the first negative value, negates it, and calls the update helper until the record's flag shows the search is complete.
- `resume_negated_entry_scan`: Continues from the entry after the current one, negating the next negative value it finds.
- `store_state_values_and_return`: Stores three input values and one trailing value into the object state fields, then returns.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `return_immediately_3`: This routine performs no operations and returns to its caller immediately.
- `load_default_entry_table`: Copies a table of twenty-five entries, twelve bytes each, into the object's entry list.
- `copy_background_planes_to_screen`: When the object's flag is set and the mode is clear, it copies a four-plane background block into the screen buffer, writing each plane at its own fixed offset.
- `add_digit_string_with_carry`: Adds a table value to an eight-character decimal string, propagating the carry from the least significant digit.
- `subtract_digit_string_with_borrow`: Subtracts a table of digit values from an eight-character decimal string, propagating the borrow.
- `draw_information_screen_panels`: Draws a fixed text-based information screen.
- `wait_for_fire_press_release`: Polls the port A and pot input registers in a fixed sequence, returning once the button state has cycled from pressed to released.
- `animate_and_draw_object_frame`: Advances a four-step animation counter.
- `update_current_target_selection`: Compares the input entry with the object's current entry and, when they match, marks the selection active and clears a 180-byte table to ones.
- `match_entry_to_record_index`: When the entry's index is marked unused, searches a list of records for the one whose index matches the object's stored index.
- `search_entries_by_stride`: Searches a list for an entry whose value matches the object's current value, stepping through the list by a field stored in each entry.
- `draw_rotated_sprite_blit`: Chooses a frame from the object's angle, computes a source offset, and waits for the blitter.
- `find_nearby_target_entry`: Returns the current value unchanged while the selection is active.
- `select_special_entry_target`: For an entry with the special flag set, runs a search over the entries with a fixed mask.
- `run_effect_countdown_frame`: While its state value is set, it counts the value down and resets the object when the count passes a limit.
- `fill_free_slots_from_table`: Fills up to eight free slots in a pending list with entries from the object's table.
- `update_all_active_records`: Walks the active record list and updates each record, calling helper routines for movement, drawing and state changes.
- `update_record_flags_and_timers`: Walks a record list in which flag and counter fields drive each record's state.
- `update_tracked_entry_pair`: Compares the input entry with two tracked indices and clears any tracked pair that matches.
- `call_five_helpers_in_order`: Calls five helper routines one after another and returns the value passed through.
- `repeat_helper_until_zero`: Calls a helper repeatedly, calling a second helper after each call, until the first helper returns zero.
- `walk_flagged_record_list`: Walks a list of records that ends with an all-ones marker.
- `advance_pseudorandom_seed`: Advances a 16-bit pseudorandom seed by multiplying it by 125 and reducing the result modulo 65521, then returns.
- `append_list_entry_with_bounds`: Adds an entry to a list of bounding boxes.
- `reset_list_terminator`: Writes an all-ones terminator at the list's current end and moves the list pointer past it.
- `initialise_record_from_table_entry`: Fills a new record from a table entry.
- `finish_record_initialisation`: Copies the remaining attributes from the source record and sets the default state values.
- `return_passthrough_value`: Returns the value passed in without changing anything.
- `call_mode_helper_once`: Sets up a helper call with a fixed mode and mask, calls the helper once, and returns the value passed through.
- `call_mode_five_helper`: Sets the search mode to five with a fixed mask, calls the search helper once, and returns the value passed through.
- `call_helper_passthrough_value`: Calls one helper routine and returns the values passed through, with no change in either branch of the check.
- `search_list_for_mask_match`: Sets the search mode to five with a fixed mask and calls the search helper.
- `initialize_data_structure`: Stores a value from a memory location into a structure and initializes several fields with constant values.
- `return_immediately_4`: This routine performs no operations and returns immediately to its caller.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `store_two_longs_in_record`: Copies the D4 and D5 register values into two 32-bit fields of the record addressed by D2, 56 and 60 bytes in.
- `increment_global_byte_by_index`: Adds one to a byte of the global array selected by the D0 word index.
- `update_entry_table_proximity`: Walks a fixed table of 64-byte entries, skipping invalid or flagged ones, and calls per-entry update routines for active entries.
- `scan_entries_for_proximity`: Walks the same entry table and applies a per-entry flag test, then a proximity test with a 5000-unit limit.
- `mark_targets_in_range`: Clears a status bit on every table entry, then walks up to 100 target indices.
- `resume_target_range_scan`: Continues the target-list walk from the current position with the same validity, range and counter checks, overwriting invalid list slots with -1.
- `advance_target_range_scan`: Steps to the next target-list slot and returns when the count runs out.
- `update_entry_status_flags`: Copies a status bit from the entry's flag word into an active-state bit, and sets a further state bit according to two other flags.
- `stub_return_only`: Returns immediately without touching any state, so it is an empty handler.
- `store_value_in_free_slot`: Searches a 100-entry signed word table for the first negative (free) slot and stores the D0 value there.
- `tiny_move_movea_rts`: Small routine of 3 instructions that copies (a7)+ to d1, then copies (a7)+ to a0, then returns.
- `commit_entry_position_or_candidate`: Sets a state byte from the entry's flags, then either copies pending coordinates into the current ones and sets a settled flag, or, when a candidate flag is set, picks the candidate with the lowest key and clears its search flag.
- `choose_lowest_key_candidate`: Walks a candidate list for the entry with the lowest key and records it on the entry.
- `compute_scaled_squared_distance`: Takes the absolute x and y differences between two points, divides each by 1000, and returns the sum of their squares.
- `check_entry_action_cooldown`: Compares the time since an entry was last stamped with a threshold picked by a type byte.
- `stamp_entry_when_cooldown_expires`: Compares a stored value with a threshold passed in D1.
- `check_code_without_effect`: Compares the D0 word with a constant and returns either way, so the comparison result is discarded and nothing changes.
- `select_highest_ranked_slot`: Scans a nine-slot pointer list for the largest value among three ranked fields of ordinary entries or a fourth field of flagged entries, records a pointer to the winning field, and calls two routines when a winner exists.
- `resume_highest_ranked_selection`: Continues the same ranked selection over the remaining slots from a given position, then calls two routines if a winner was recorded.
- `find_minimum_key_match`: Walks a linked list until its null terminator, keeping the entry with the smallest key that matches a stored code value, and stops early when a flag test passes.
- `scan_list_for_lowest_key`: Walks a linked list and tracks the lowest key among entries whose code matches a stored value, stopping early when a flag test passes.
- `initialise_scan_cursor`: Stores the current position in the object and sets four status words to fixed constants, then walks a list comparing values until the list end is reached.
- `return_void`: This routine performs no operations and returns immediately.
- `walk_word_list_subtracting`: Walks a list of words from a start pointer, subtracting each stored word from a value taken from the object's list head, and returns once the value reaches zero or goes negative.
- `record_event_in_free_slot`: Places a value into the first free slot of one of two small tables.
- `relink_and_compact_entry_table`: Updates each entry's link to its next position and clears child slots that are marked finished.
- `relink_entry_table_from_position`: Performs the same relinking and compaction as the full table routine, but its first pass starts from a given pointer rather than the start of the table.
- `advance_entry_or_compact_table`: Walks the entry table, clearing finished child slots and advancing each entry's link.
- `select_adjacent_grid_cell`: Looks up an entry's cell in a 6000-unit grid table and picks the neighbouring cell nearest the target coordinates, returning scaled grid coordinates for that cell.
- `convert_grid_cell_to_coordinates`: Converts two grid cell indices in D0 and D1 to world coordinates by multiplying each by 6000, with the first also offset by one more cell.
- `find_nearest_rectangle_to_point`: Scans a table of rectangles whose size is given in its header and computes the squared distance from a point to each rectangle's nearest edge.
- `scan_rectangles_for_nearest_edge`: Continues the rectangle scan from a given rectangle, measuring squared distance to each rectangle's edges and remembering the nearest one.
- `scan_rectangles_for_nearest_point`: Another variant of the nearest-rectangle scan, starting from a given rectangle pointer and updating the nearest-match globals when it finds a closer rectangle.
- `lookup_code_from_table`: Maps the indices 5, 6 and 7 to fixed codes 67, 68 and 69, and otherwise reads a word from a two-dimensional table indexed by both inputs, with the second input doubled and 142 replaced by 76.
- `read_table_word_by_index`: Returns the word stored in the global table at the offset given by D0.
- `throttle_entry_update_by_stamp`: Performs one update step only when enough time has passed since the entry's stamp.
- `refresh_cached_slot_value`: Returns a cached value if it was refreshed less than 200 ticks ago.
- `test_point_in_rect_discard`: Tests whether the point in D4 and D5 lies inside the rectangle at the record pointed to by A0, but the result is discarded and the routine returns in every case.
- `trigger_pending_action_once`: Sets a flag, and when the object's pending counter is negative it triggers an action using a per-slot table value and resets the counter to zero.
- `clear_flag_and_trigger_action`: Clears the flag, then triggers the same pending action as the sibling routine when the counter is negative, and resets the counter to zero.
- `trigger_table_action_if_pending`: When the object's pending counter is negative, it sets a flag and triggers an action using a value from a per-slot table at the given base, then resets the counter to zero.
- `check_and_reset_value`: Checks if a short value at a specific offset is negative, and if so, sets a global byte to one and calls another routine twice before resetting that offset to zero.
- `store_scaled_slot_table_value`: Looks up a byte in a five-byte-stride table for the object's current slot, doubles it, and stores the result in a global word.
- `store_scaled_previous_slot_value`: Stores a byte from the previous slot's table entry, scaled by another value, into a global word.
- `compute_weighted_chain_score`: Computes a weighted score from table bytes for the current slot, the summed values along a linked chain, and a ratio of accumulated totals.
- `compute_weighted_chain_score_alt`: Performs the same score computation as its sibling, but stores the chosen value in a separate global slot.
- `store_clamped_quotient_with_counter`: Divides an input by a byte from the object record, clamps the quotient to 12, and records it.
- `initialise_alt_scan_record`: Stores the current position in the object's scan record and sets four status words to constants.
- `pass_through_after_call`: Calls one routine and then returns the D0 and D1 register pair unchanged on both branches, so it has no visible effect of its own.
- `increment_flag_selected_counter`: Chooses a counter array, either a default or one from a matching list entry, and increments the byte selected by a bit field in the entry's flags.
- `increment_counter_from_bits`: Increments a byte in the given record at an index taken from a bit field of the D1 flags, using a fixed index for code 140.

### strikeiv.prg

- `run_music_player_main`: Program entry point: sets the task priority, initialises the music player, installs the interrupt, waits for the end signal, then restores the interrupt and shuts the music player down.
- `Wait_For_End`: Polls the CIAA_PRA register and returns as soon as the fire bit reads low.
- `Set_Irq`: Writes a fixed sequence of values to the CIA-B control and interrupt registers, sets the timer high and low bytes, and stores the interrupt handler address.
- `Return_Irq`: Writes a fixed value to the CIA-B interrupt control register, restores a saved word, and writes a small fixed value to the DMA control register.
- `Set_Task`: Calls the Exec library to find the current task, then calls it again to set that task's priority.
- `New_Interupt`: Interrupt handler that touches the interrupt request register and calls the music playback routine.
- `mt_init`: Initialises the ProTracker music replay: reads the song's pattern order table, sets up the sample pointers and resets the song position.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `mt_music`: ProTracker music replay tick, called once per frame: advances the song and feeds new notes and effects to the four Paula audio channels.
- `mt_wait`: Contains an empty loop that never exits, so it never returns.
- `mt_nonew`: Runs the per-channel command routine for each of the four channels, then steps the song position forward, wrapping at 128 and reloading a stored value when it matches.
- `mt_playvoice`: Reads the next four-byte note entry for one audio channel, loads its sample, period and volume from tables, and applies the effect command: position jump, pattern break, speed change, or volume set.
- `mt_noloop`: Advances one channel's note state for a non-looping sample, sets its sample pointer and volume, and dispatches the effect command to the portamento, stop or speed and volume handlers.
- `mt_stopsound`: Applies a channel's effect command from its effect nibble: fine volume, pattern break, position jump, speed change or volume set.
- `mt_setport`: Sets a channel's portamento target from its note period and direction flag, then applies the same effect dispatch as the other channel handlers.
- `mt_clrport`: Zeroes the channel's portamento speed word.
- `mt_port`: Applies one portamento step to a channel's period.
- `mt_port2`: Performs one portamento step on a channel's period without latching a new speed, moving toward the target and writing the result out.
- `mt_sub`: Subtracts the D0 amount from a channel's period, clamps at the target value, and writes the result to the output period.
- `mt_vib`: Applies one vibration step.
- `mt_vib2`: Applies one vibration step using the depth already stored for the channel, adding the sine offset to the period and advancing the phase.
- `mt_vibsub`: Writes the difference between two inputs to the channel's output period, then advances the vibration phase by the upper nibble of the depth byte.
- `mt_arp`: Selects an arpeggio note by searching the period table for the first period not below the channel's base period, then offsetting it by a nibble of the effect byte.
- `mt_arp2`: Performs the same period search as the arpeggio routine, using only the low nibble of the effect byte as the offset.
- `mt_arpfound`: Writes the period table entry at the located index, offset by the D0 amount, to the channel's output period.
- `mt_normper`: Copies the channel's base period to its output period without changing any state.
- `mt_com`: Dispatches a channel's effect command by its effect nibble to the arpeggio, portamento, vibrato, volume slide or set-volume handlers and returns the resulting value.
- `mt_portup`: Lowers the channel's period by its slide amount, clamps it at the minimum period of 113, and writes it to the output period.
- `mt_portdown`: Raises the channel's period by its slide amount, clamps it at the maximum period of 856, and writes it to the output period.
- `mt_volvib`: Runs a vibration step and then a volume slide.
- `mt_volport`: Runs a portamento step and then a volume slide.
- `mt_vol3`: Lowers the channel's volume by the low nibble of its effect parameter, floors it at zero, and writes it to the output volume.
- `mt_com2`: Applies the effect commands that do not slide pitch: filter, pattern break, position jump, speed and set volume.
- `mt_filter`: Sets bit 1 of the global mirror of the CIAA_PRA register from the low bit of the effect parameter.
- `mt_pattbreak`: Sets the song position variable to 1, the value used to request a pattern break.
- `mt_songjmp`: Sets the song position variable to one less than the effect parameter.
- `mt_setvol`: Clamps the effect parameter to a maximum of 64 and stores it as the channel's volume and output volume.
- `mt_setspeed`: Sets the playback speed from the effect parameter, clamped to a maximum of 31 and raised to 1 when it is zero.
- `mt_speed`: Small routine of 1 instruction that adds 0 to d0.
- `mt_songpos`: Updates a global variable representing the current position within a music track.
- `mt_samplestarts`: This routine is called during the initialization phase to set up the starting parameters for audio samples.
- `mt_voice1`: This routine is called by music management functions to initiate a specific voice or sound sample playback.
- `mt_voice2`: Performs a bitwise OR on a byte in a register and checks if the result is negative.
- `mt_voice3`: Performs a bitwise OR operation on a byte from a register with a constant and checks the result against zero.
- `mt_voice4`: Modifies a byte value by performing a bitwise OR operation with a constant.

### sf.prg

- `start_program_main_loop`: Program entry point: stores initial pointers and values in globals, calls several setup routines, then loops forever calling two routines.
- `read_file_and_close_libraries`: Opens a DOS file, reads from it, closes the file, and closes the library.
- `write_file_and_close_library`: Opens a DOS file and writes to it when the open succeeds, then closes the file and the library.
- `open_dos_library`: Opens dos.library through Exec's OldOpenLibrary and stores the returned base in a global.
- `fetch_io_error_and_close`: Reads the DOS I/O error code, closes the file handle and the library, and returns a failure value.
- `setup_graphics_interrupt_state`: Forbids task switching, opens graphics.library, saves one of its words and replaces it with a handler address, then writes enable values to the interrupt and DMA registers before closing the library.
- `handle_audio_channel_interrupts`: Interrupt handler that checks the request bits for the first two audio channels in the interrupt request register.
- `count_flagged_interrupt_event`: When bit 5 of the D0 value is set, restores a saved pointer, calls a routine and increments a counter.
- `initialise_table_and_delay`: Fills a seven-group table of word pairs with fixed values, waits in a nested countdown loop, and writes two fixed values into a second table when a flag is set.
- `wait_for_interrupt_then_enable`: Polls the interrupt request register until bit 5 is set, then writes the value that re-enables that same interrupt bit in the interrupt-enable register.
- `split_bitplane_pointers`: Splits a base address into four bitplane pointers spaced 8000 bytes apart and stores the high and low words of each in separate globals.
- `execute_indirect_call`: Calls a function through a pointer retrieved from a memory location.
- `release_task_switching_lock`: Calls Exec's Permit routine to re-enable task switching, then returns the register pair unchanged.
- `forbid_task_switching_2`: Calls Exec's Forbid routine to disable task switching and returns the register pair unchanged.
- `run_selected_handler_or_halt`: Calls a library routine and, when it returns nothing, forbids task switching and loops forever.
- `restore_graphics_field_and_write`: Opens graphics.library, stores a saved value in one of its fields, and writes a short text string to DOS output.
- `empty_routine`: This routine is empty and contains no executable instructions.
- `expand_nibbles_into_buffer`: Splits a 32-bit input into eight nibbles and writes eight bytes into a global buffer, each one looked up in a table indexed by its nibble.
- `deinterleave_image_into_bitplanes`: Copies a four-plane image from a staging buffer, skipping a 128-byte header, into four separate bitplane buffers by moving one word from each of four interleaved positions per step across 4000 words.
- `deinterleave_image_with_setup`: Performs the same four-plane de-interleaving copy as its sibling, skipping the leading setup calls.
- `allocate_frame_buffers`: Allocates one memory block through Exec and carves four sub-buffers from it at offsets 32000, 64128 and 88704 bytes, storing each address in a global.
- `free_frame_buffer_block`: Frees the memory block through Exec's FreeMem routine and stores the returned value in the block pointer global.
- `execute_strikeiv_program`: Executes the strikeiv.prg program through DOS, then closes the library.
- `decompress_lzss_stream`: Decompresses an LZSS-style stream.

### Strikefleet (1987)(LucasArts)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
