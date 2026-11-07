# Game 731 (Strikefleet): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00731). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 92 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 168 raw-word lines |
| Functions named and described | pass, 561 of 561 named (44 author symbols), 561 of 561 described (36 by rule, 52 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 64 of 64 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 17812 of 17848 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 480137 |
| code | 66794 |
| data | 169530 |
| padding | 23891 |
| unused | 160768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| iconx | 12 | 806 | 37 |
| Strikefleet.adf:TRSI! | 2 | 233 | 8 |
| disk-validator | 1 | 386 | 1 |
| ppp.prg | 2 | 2707 | 76 |
| ppq.prg | 2 | 12605 | 365 |
| strikeiv.prg | 2 | 493 | 45 |
| sf.prg | 1 | 374 | 24 |
| Strikefleet.adf:bootblock | 1 | 10 | 1 |

## Cases handled

- Strikefleet/Strikefleet.adf:StrikeFleet/SF_MAP0.LZW: high-entropy data file (7.72 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_table_then_call_pointer`: If its output pointer is non-null, copies sixteen 32-bit entries from a fixed table into that buffer.

### assign

- `find_max_entry_and_setup`: Program entry point.
- `copy_entries_and_call`: Copies a fixed number of four-word entries from a source table into a destination table, then invokes an indirect routine from a table.

### iconx

- `parse_command_line_and_run`: Program entry point.
- `store_and_release_state_slot`: Looks up the current task's state block, stores its parameter in the state slot, then conditionally calls cleanup routines from fields of that record and makes one more call.
- `run_icon_program_with_buffer`: Sets up options from its parameter block and looks up a window or icon handler, then allocates a 1536-byte buffer and fills it through a loop of helper calls before passing it to a processing routine.
- `handle_icon_request_result`: Clears a byte in the buffer and runs a lookup on the request, reading its first character.
- `initialise_library_and_functions`: When its parameter pointer is non-null, it makes a library from the record's first field, creates its functions from a second field, and enables them.
- `join_path_nodes_with_colon`: Allocates a 260-byte scratch buffer, then walks a linked list of text nodes and appends each node's string with a separator, skipping short entries.
- `chain_calls_with_offset`: Makes two calls that pass its three arguments through, uses the result of a third call on its second and third arguments to compute an offset from the first, and calls one routine with that offset.
- `count_string_length`: Returns the number of bytes before the terminating zero of a string passed by pointer.
- `copy_null_terminated_string`: Copies bytes from a source string into a destination buffer, including the terminating zero byte.
- `skip_string_end_and_call`: Advances a pointer to the terminating zero of a string, then calls one routine with that position and the second argument unchanged.
- `parse_signed_decimal`: Reads an optional leading plus or minus sign and then the decimal digits of a number, stopping at the first non-digit, and returns the signed value.
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
- `call_with_stack_slot_pointer`: Calls one routine with its two arguments and the address of its stack-argument area, then returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `format_buffer_and_measure`: Calls a library routine through a global table entry, which the facts identify as a text formatting call, into a 140-byte stack buffer.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_format_library_entry`: Calls the library routine stored at a fixed offset in a global table, which the facts identify as a text formatting call, then returns.
- `store_byte_at_output_pointer`: Stores the low byte of the data register through the address register pointer, without moving the pointer.

### Strikefleet.adf:TRSI!

- `load_hunk_file_with_relocation`: Program entry for loading an AmigaDOS executable.
- `delay_loop_count_down`: Busy-waits by decrementing a 16-bit count register until it wraps past zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_compressed_stream_backwards`: Unpacks a compressed stream into a buffer from its end toward its start.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_buffer_by_count`: Shifts a bit buffer held in two registers one bit per step, for a count given in a 16-bit register.
- `stub_bad_instruction_data`: The decompiler reports bad instruction data after a trap instruction and truncates the block, so no behaviour is recovered.

### disk-validator

- `scan_bit_masks_and_count`: Program entry point whose pseudo-code is truncated in the bundle.

### ppp.prg

- `run_forever_main_loop`: Program entry point that stores a few fixed addresses, calls five routines in sequence, then loops forever calling two routines.
- `stub_empty_return`: A routine that does nothing but return.
- `init_table_entries_with_delay`: Fills groups of table entries with fixed words, then busy-waits through a nested counting loop.
- `split_base_into_pointers`: Reads a 32-bit base address from a fixed structure and writes its high and low 16-bit halves into several pointer locations.
- `copy_table_and_combine_rows`: Copies 480 longs from one table to another, then runs four groups in which words from three source rows are combined by bitwise operations into a destination row.
- `process_audio_channel_requests`: Checks four per-channel request bits in a state word.
- `start_audio_channel_playback`: Starts playback on a free audio channel by choosing an entry whose state is not six, then storing its sample pointer, length and period fields.
- `load_ready_channels_into_hardware`: For each of four channel blocks that is marked active and matches a sequence value, copies its pointer, length and period into the channel's hardware registers.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `open_and_read_file_retrying`: Opens a file through the dos library, repeating the open while it returns zero, then repeats the read while it reports an error.
- `close_dos_file_and_library`: Closes a file through the dos library, then closes the library base and returns its stored argument.
- `write_file_or_fail`: Opens a file and writes through the dos library.
- `open_dos_library_base`: Opens the dos library by name through the exec library and stores its base in a global variable.
- `close_after_io_error`: Fetches the dos error code, closes the file and the dos library, and returns a failure marker.
- `open_and_record_handle`: Opens a file and, when the open succeeds, calls one more library routine and stores its low-word result in a global.
- `decode_keyboard_scancode`: Reads and clears a pending keyboard byte, inverts it, and decodes it to a character through a table of printable keys from the first keyboard row.
- `clear_flag_on_key_codes`: Clears a global flag when the low seven bits of its argument are 96 or 97, and returns the argument unchanged otherwise.
- `install_hook_and_enable_interrupts`: Opens the graphics library, saves one of its fields and replaces it with a pointer, then writes interrupt control values and a handler address to hardware.
- `restore_dma_and_interrupt_state`: Calls one routine, then writes fixed control values to the DMA and interrupt registers, writes two saved values back, and finally sets interrupt enable bits with a fixed mask.
- `service_interrupt_request_bits`: Checks interrupt request bits.
- `queue_button_event_when_set`: Handles a button-state input passed in a register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_clamped_position_counters`: Updates two position counters from the change in a global input since the last call.
- `update_clamped_position_counters_2`: Performs the same clamped position update as its neighbouring routine, reached from another caller.
- `apply_clamped_position_delta`: Adds a signed step computed from two register inputs to one position counter and clamps the result to a fixed range.
- `update_position_from_mouse_port`: Clamps one position counter, updates the second from the change in a value read from the mouse movement register, clamps it to a fixed range, and applies the minimum from a table.
- `set_clamped_position_from_inputs`: Sets one position counter from a register input plus a signed step, clamps it to a fixed range, and applies the same minimum from a table and maximum clamps as its neighbours.
- `clamp_position_counters`: Clamps the position counters to their fixed ranges, raising one to a table minimum and capping the other, with no other changes.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `forbid_task_switching`: Calls the exec library's Forbid routine to stop task switching, then returns its input registers unchanged.
- `swap_pointers_then_wait`: Disables one interrupt, swaps two saved longs in a structure, and sets a flag word to one.
- `wait_for_interrupt_then_enable`: Busy-waits until the interrupt request bit is set, then writes the interrupt enable value for that interrupt.
- `match_text_and_parse_fields`: Compares a block of 16 characters at a fixed location with the text at a stored pointer.
- `match_string_and_parse_fields`: Compares a stored string against a reference string of fixed length, and on a full match extracts several numeric fields from the buffer into global variables before clearing flags and returning.
- `match_text_and_parse_fields_2`: Same text matching routine as its neighbours, reached from a different caller.
- `parse_number_from_text`: Compares a stored text string against the text pointed to by a global variable and, when they match, extracts the digits of a number from the source text.
- `match_text_and_parse_fields_3`: Same text matching and field parsing routine as its neighbours, reached from a different caller.
- `dispatch_queued_handler`: When a queue index differs from the write index, it takes the next queued handler from a 32-entry table and advances the index with wrap at 32.
- `store_argument_in_state_field`: Stores its stack argument into a fixed field of the state block.
- `reset_then_invalid_call`: Calls a reset routine and then calls a fixed low address, which is not a valid routine, so the routine likely reaches an invalid target.
- `program_blitter_from_state`: Waits for the blitter to finish, then programs its mask, modulo, pointer, control and size registers from positions in a state record and a table, and starts the operation through the size register.
- `reprogram_blitter_from_saved_pointer`: Waits for the blitter to finish, then reprograms its mask, control, modulo and pointer registers from a saved pointer in the state record, and starts the size register.
- `compose_entry_list_and_text`: Copies table values into state fields, computes a total from fixed multipliers and delay counts, and builds a list of entries from a second table.
- `decompress_lzss_stream`: Decompresses a stream in the classic LZSS style.
- `scan_records_and_update_state`: Large routine with 212 instructions whose pseudo-code is truncated in the bundle.
- `format_three_digit_number_text`: Writes a 16-bit number as three decimal digit characters into a buffer.
- `split_pixels_into_four_arrays`: Looks up each masked value of 20 rows of 16 entries in a table, combines it with a fixed mask, and writes one byte to each of four byte arrays spaced 8000 bytes apart.
- `update_entry_flags_next`: Sets a flag bit on list entries that match a current index and clears it on the others, then selects the next item from a pointer list and updates a row byte from a table.
- `advance_countdown_table_entry`: When a countdown counter expires, it reads the next entry from a table, stores an index, and calls routines to step through the data.
- `select_entry_and_wait_timer`: Selects an entry by index, records it in the state, and loads its pointer and count fields from tables.
- `select_entry_and_wait_timer_2`: Same table-driven entry selection and timer loop as its neighbour, without the first check against the current index.
- `combine_digit_string_with_table`: Combines an eight-character decimal digit string with eight table bytes, working from the first digit onward and carrying between digits.
- `subtract_digit_string_with_table`: Subtracts eight table bytes from an eight-character decimal digit string, working from the first digit onward with a borrow between digits.
- `subtract_digit_string_with_table_2`: Same complement and subtract digit routine as its neighbour, reached from another caller.
- `step_timed_entries_until_end`: Repeatedly waits for a timer value to reach a stored time, advances that time by four, and calls two routines, until the table entry it reads is zero.
- `draw_text_glyphs_to_planes`: Walks a null-terminated text and draws each character as eight rows of glyph bits into four byte arrays spaced 8000 bytes apart, using a font table indexed by character code.
- `repeat_call_eleven_times`: Fetches the next word of a stream, saving a pointer when the word is the sentinel, then calls one routine eleven times.
- `repeat_routine_in_three_passes`: Calls one routine seven times per pass across three passes, with no other work.
- `fill_block_after_counter`: When one state value differs from another it advances a counter.
- `fill_block_with_register_byte`: Fills a block of bytes with the byte passed in a data register, using a table offset selected from the register value and the same stride as its neighbour.
- `initialise_display_state_and_labels`: Initialises display state: sets 594 word pairs to zero and 65535, copies a source table of 4000 words into four arrays spaced 8000 bytes apart, copies 8192 longs back, and sets a few flags and values.
- `pack_four_arrays_into_words`: Copies from four 16-bit arrays spaced 8000 bytes apart into one interleaved table, writing four words per entry for 84 rows of seven entries and advancing the source 20 words per row.
- `unpack_table_into_four_arrays`: Copies from an interleaved table of four words per entry back into four 16-bit arrays spaced 8000 bytes apart, for 84 rows of seven entries.
- `run_display_update_loop`: Sets up a display frame, copies arrays, and builds entry lists from tables.
- `initialise_display_buffers_and_pattern`: Initialises display state: copies 32 longs and a block of 4000 words into arrays, rearranges byte groups into four pattern arrays, and fills a 256-entry table with complement values.
- `draw_text_glyphs_to_planes_2`: Draws each character of a null-terminated text as eight rows of glyph bits into four byte arrays, combining the glyph with a masked source word.
- `blend_masked_planes_into_block`: Combines three masked source words into four byte arrays for three groups of eight rows, choosing its masks from a state value.
- `initialise_pattern_table_and_interrupts`: Initialises a pattern table, copies rows of data from four arrays into a working buffer, and sets position and clamp values.
- `update_position_from_buttons`: Reads fire and side button bits from the button registers and sets a direction value from them.
- `update_position_and_entries`: Performs the same position and entry update as its neighbour, using the stored direction value instead of reading the buttons.
- `set_entry_angle_from_table`: Sets a field of an entry from a table indexed by a value, and negates a second field when the value is eighteen.
- `set_entry_angle_second_table`: Sets a field of an entry from a second table indexed by a value, and negates a second field when the value is eighteen.
- `find_entry_and_advance_state`: Searches a list of entries for one whose two position fields match the arguments, then updates its direction-related field from a table and calls routines.
- `load_table_entry_into_state`: Looks up an 8-byte entry in a table using a field of its argument, copies the entry's two longs into state fields, and calls one routine.
- `merge_masked_rows_into_arrays`: Loops 16 times over two sets of four arrays spaced 8000 bytes apart.

### ppq.prg

- `run_forever_main_loop_2`: Program entry that copies six longs from its argument block into a fixed location, stores a pointer, calls four routines, then loops forever calling four routines.
- `init_table_entries_with_delay_2`: Fills six groups of table entries with fixed words, busy-waits through a nested counting loop, and if a flag global is nonzero writes fixed words to several locations and adds a scaled value to two series of table entries.
- `copy_twelve_words_from_row`: Copies twelve words from a row of a table, selected by a state field divided by four, into a destination with a stride of six words.
- `split_base_into_pointers_2`: Reads a 32-bit base address and writes its high and low 16-bit halves into several pointer locations.
- `copy_table_and_combine_rows_2`: Copies 768 longs between two arrays, then runs groups of rows that combine four source words by bitwise operations.
- `process_audio_channel_requests_2`: Checks four per-channel request bits in a state word, and for the first channel also requires a signed value to be nonnegative.
- `find_free_slot_and_store`: Searches a table of records for a free slot, then stores a computed index, a mode value, and copied data into that slot before returning.
- `process_matching_records`: Walks a table of records, and for each matching entry copies fields into a destination structure, sets status bits, and marks the record as consumed.
- `wait_for_timer_results`: Calls a timer routine and waits for a non-zero result, then calls a second timer routine and waits for a non-negative result before invoking two more indirect routines.
- `open_and_read_file_retrying_2`: Same retrying open and read routine as its neighbour, reached from another caller.
- `query_library_and_retry`: Queries a library handle, and on success calls a further library routine and a RAM-offset routine before returning a value, otherwise it closes libraries and a RAM-offset routine before returning.
- `open_exec_library`: Calls an exec library routine and stores the returned handle into a global variable.
- `close_libraries`: Closes a dos library handle, closes an exec library handle, and invokes an indirect routine before returning a value.
- `query_library_and_store`: Queries a library handle and, when a high-order condition is met, stores the result into a global variable before returning, otherwise it closes libraries and a RAM-offset routine before returning.
- `translate_keyboard_input`: Reads a byte from a global variable, clears it, and maps a keyboard register value through a lookup table.
- `compare_byte_and_clear`: Compares a byte passed in a register against two distinct values and, for either match, clears a global variable before returning.
- `open_graphics_library_2`: Opens the graphics.library library, saves its previous interface pointer, and installs a new interface pointer in place of the old one.
- `setup_interrupts`: Copies values between global variables, stores several constants into global variables, and enables interrupts via the interrupt register before returning.
- `verify_table_or_hang`: Compares six longs of a table against the values at a fixed code location, one per step.
- `verify_table_or_hang_2`: Same six-long comparison as its neighbour, reached from another caller.
- `process_queued_event_2`: Waits for the vertical blank interrupt, then reads a pending event status byte and queues the matching handler address into a circular buffer, updating the buffer index and several related state variables.
- `process_queued_event`: Detects a pending event from a status byte and queues a handler address in a circular buffer.
- `encode_button_state_for_port`: Combines the fire and side button bits into a status byte and writes it with a position-derived value to a fixed location.
- `clamp_position_differences`: Computes the signed difference between two stored values, clamps each result to a range, and stores the updated values back.
- `update_clamped_position_counters_3`: Performs the same clamped position update as its neighbouring routines, reached from another caller.
- `update_position_counters`: Computes a clamped difference between a stored value and a global variable, updating one accumulator.
- `update_clamped_position_counters_4`: Stores two incoming values and adds a clamped delta to a stored accumulator.
- `update_mouse_position`: Reads mouse port movement, computes a signed difference against a stored value, and adds the clamped delta to a position accumulator.
- `clamp_position_accumulator`: Adjusts an unaffiliated value by a delta, adds it to a stored accumulator, and clamps the result to a fixed range.
- `clamp_stored_values`: Clamps a stored value against lower and upper bounds, then clamps a second stored value against a bound read from memory before returning.
- `wait_for_flag_clear`: The routine spins until a global variable no longer has a specific bit set, then returns.
- `permit_task_switching`: Calls the exec library's Permit routine through the library base to allow task switching again, then returns its input registers unchanged.
- `forbid_preemption`: Calls an exec library routine to forbid preemption and returns the two-word result.
- `swap_pointers_then_wait_2`: Disables one interrupt, swaps two saved longs in a structure, and sets a flag word to one.
- `wait_flag_then_set_int`: Waits in a loop until a global variable's bit 5 becomes set, then stores a 16-bit constant combining that bit with bit 15 into the same variable.
- `match_marker_and_parse_fields_2`: Compares a global variable against a marker and, if the following bytes match, decodes a run of ASCII digits into numeric fields stored in several globals.
- `decode_match_fields`: Compares a stored string with the one pointed to by a global variable and, if they match, decodes the following bytes into several numeric fields.
- `parse_marker_fields`: Compares a stored marker string with a text buffer and, when they match, decodes the following digits into numeric fields and copies two routine entry pointers.
- `decode_marker_fields`: Compares a stored marker string against a buffer and, when they match, decodes the following digits into a two-digit value, a three-digit value, and a single digit while copying two stored words.
- `match_marker_and_parse_fields`: Compares a stream of bytes against the text marker DIGITAL PERSUASION and, on a full match, decodes decimal digits and words stored just before the marker into several game variables and clears a counter.
- `read_ring_buffer_byte`: Reads the next byte from a 32-entry ring buffer, advancing and wrapping the read index.
- `handle_ring_buffer_byte`: Performs the same ring buffer read as its neighbour, advancing the index with a 32-entry wrap and forcing a machine reset when a flag is clear and the byte is the character a.
- `store_stack_long_into_state`: A tiny store routine that copies the 32-bit value passed on the stack into one fixed field of the state record.
- `force_machine_reset`: Forces a machine reset by calling the reset routine and then jumping to the reset vector.
- `dispatch_index_or_reset`: Compares a 16-bit value in the state block against 35.
- `call_library_then_reset`: Calls a library entry point through a register-relative pointer, performs a machine reset, and then jumps through an indirect table pointer.
- `reset_then_jump_indirect`: Performs a machine reset and then transfers control through an indirect pointer taken from a table.
- `store_memory_block_addresses`: Requests a memory block through a library call and stores its base address, the address 8000 bytes past it, and a second block's base and end addresses, along with the differences between them, in global variables.
- `store_memory_block_addresses_2`: Entry point into the same body as the previous routine: it requests a memory block through a library call and stores its base address and the derived addresses 8000 bytes apart in global variables.
- `store_memory_block_addresses_3`: Alternate entry into the memory block address routine: it requests a block through a library call and records its base and the addresses offset by 8000 bytes in global variables.
- `store_memory_block_addresses_4`: Another entry into the same memory block address routine, which calls a library function and stores the base and offset addresses in global variables.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `store_memory_block_addresses_5`: Entry into the shared memory block address routine, which fetches a block through a library call and stores its base and derived addresses in global variables.
- `compute_offsets_and_store`: Fetches a pointer from a structure and stores several derived integer values, including differences and offsets of 8000, into a set of globals before returning.
- `compute_data_block_size`: Reads a pointer from the graphics library and follows a chain of offsets to locate a data block, then computes the block's size and its end address relative to its start.
- `open_graphics_library`: Opens the graphics library by name through the system open call and closes it again when finished.
- `initialize_state_and_planes`: Sets starting values in the game state and fills several tables with all-ones sentinels.
- `decompress_lzss_stream_2`: Unpacks an LZSS-style compressed stream using a 4096-byte sliding window pre-filled with spaces.
- `deinterleave_words_into_planes`: Walks a list of table entries, storing a value at each entry's offset, then splits a block of four-word groups so that each group's words are written to four planes spaced 4000 words apart.
- `return_immediately`: Returns immediately without performing any operation.
- `dispatch_visible_object_handlers`: Walks a linked list of object records, skipping any outside a bounding window or carrying the wrong flag.
- `blend_source_into_bitplanes`: For each source byte, combines the four plane bytes stored 8000 bytes apart with values looked up from a table indexed by source bits, then writes the four bytes back.
- `blend_source_into_bitplanes_2`: Alternate entry into the same bitplane merge loop, using the same table lookups, 8000-byte plane spacing and 40-byte row stride.
- `advance_position_accumulators`: Updates two 32-bit position accumulators by adding a 16-bit delta multiplied by a scale that depends on a mode value, and stores the step constants for that mode.
- `run_timed_update_loop`: Repeats a sequence of helper calls while a timer counter advances, waiting for each target tick, and exits once a stored value reaches 124.
- `run_update_with_counter_check`: Returns early if a signed counter is negative, otherwise calls a sequence of helper routines while toggling two flag fields and copying one value from a global.
- `run_fixed_step_sequence`: Runs the same fixed helper sequence as its neighbour without a counter check: it sets and clears two flags around the calls and then zeroes the pending counter.
- `record_cell_if_in_window`: Checks whether a record's position lies inside a rectangular window.
- `run_frame_update_loop`: Large per-frame routine that processes pending state changes and runs helper steps in a loop.
- `copy_record_fields_snapped`: Copies position and state bytes from one record into another, then adjusts the copied x and y coordinates by the remainder of each against 375, using two supplied offsets.
- `adjust_record_coordinates_modulo`: Adjusts the x and y coordinates of a record by subtracting the difference between each coordinate's remainder modulo 375 and a supplied offset.
- `clear_text_panel_spaces`: Fills three rows of a character buffer with space characters at fixed strides, then clears a short run in the following row.
- `format_status_text_line`: Builds a status text line by copying name strings from records into a buffer with separators and fixed label characters.
- `store_selector_and_result`: Stores a 16-bit selector into one state field.
- `scan_records_from_index`: Points a state record at the entry for a given index, sets fixed mode and flag values, then repeatedly calls a helper and advances through records until a terminating flag is found.
- `update_selector_history`: Stores a new selector value.
- `scan_records_with_mode_three`: Performs the same record scan as its neighbour but with a mode value of 3 and a different flag word, advancing through records until a terminating flag is found.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 0 into d5, then loads 4 into d7.
- `tick_counter_fixed_passes`: Runs a counted loop in which each pass calls two helpers and advances a 16-bit counter, clamped at 4 and masked to keep its sign bit clear.
- `forward_to_helper`: A thin wrapper that calls one helper routine and returns, adding no logic of its own.
- `store_byte_into_indexed_table`: Looks up a 16-bit offset in a table using the low byte of the input, doubled as an index, then stores the supplied byte at that offset from the base register and calls a helper.
- `format_status_text_panel`: Blanks two rows of a text buffer, then writes a name from a record followed by a colon when a flag allows it, a fixed label, and either a bracketed name or a fallback label with a two-digit value.
- `write_three_digit_decimal`: Writes a 16-bit value into three character cells as decimal digits, hundreds first.
- `add_three_digit_decimal`: Adds the hundreds, tens and units digits of a 16-bit value into three character cells that are first set to the ASCII digit zero, producing a zero-padded three-digit number.
- `write_two_digit_decimal`: Writes the tens and units digits of a 16-bit value into two character cells as zero-padded ASCII digits.
- `write_four_digit_decimal`: Writes a 16-bit value as four decimal digit characters, thousands first.
- `copy_planes_and_init_grid`: Copies four 4000-word planes back into a single block, sets default state values, and builds a grid table of 33 by 7 sequential values with a row step of 320.
- `build_bit_expansion_tables`: Builds two lookup tables: the first stores each byte value with every bit doubled into adjacent positions, and the second stores 512 Gray-code derived patterns.
- `fill_text_rows_with_spaces`: Fills a block of text cells with space characters, using 63 or 50 entries depending on a flag.
- `fill_text_rows_with_spaces_2`: Fills a block of text cells with space characters, using 63 or 50 entries depending on a flag.
- `draw_text_list_entries`: Walks a list of entries, each holding a colour byte, a row and column pair and a zero-terminated string.
- `render_text_bitplanes_by_row`: Runs four passes over the text cells, drawing each character into the bitplanes by looking up an 8-byte glyph in the font table, masking it with a colour table, and writing it across four planes with a 40-byte stride.
- `configure_timing_and_buffers`: Sets two buffer pointers in the state, then stores timing constants for the current mode.
- `draw_text_string_glyphs`: Draws a zero-terminated string into the bitplanes one character at a time.
- `spin_until_timer_count`: A busy-wait stub that loops with an empty body while comparing a timer counter with a value held in a register.
- `draw_status_text_glyphs`: Optionally records a position from state, then either draws the glyphs of a zero-terminated string into the bitplanes or, when another flag is set, shifts a block of stored character codes and redraws six glyph rows stepping 16 units each time.
- `reset_plane_state_and_copy`: Copies a 32-entry block and a block of about 1840 words of four-plane data, clears a 160-entry region with plane sentinels, and sets default counter constants.
- `draw_indexed_text_string`: Looks up a zero-terminated string inside a table at an index-based offset and draws its glyphs into the bitplanes with the same per-character glyph logic, where a space clears its cell.
- `run_timed_display_loop`: Repeatedly updates display state from current records and calls a long series of helpers.
- `copy_record_table_into_state`: Very large routine, truncated in the record.
- `overlay_masked_plane_block`: Overlays a block of seven rows by ten words of four-plane data onto a destination.
- `draw_inverse_text_glyphs`: Draws a zero-terminated string into the bitplanes like the plain glyph routine, but combines each glyph with the colour table in a complementary way so characters blend with the existing plane contents.
- `blit_masked_tile_block`: Copies an 8-by-8 block of four-plane tile data from a source map into the bitplanes at a computed position.
- `project_visible_objects_to_screen`: Projects object records to screen coordinates using a 360-step angle table and a distance scale, storing position, size and step values for each visible object.
- `draw_sprite_record_chain`: Walks a chain of sprite records, calling a draw helper for each one and updating a per-record link value.
- `clip_sprite_to_window`: Tests a sprite's column and row against a window of 40 columns by 240 rows.
- `sort_projected_sprites_by_depth`: Projects object records to screen values as its neighbour does, then runs swap-based passes that reorder the linked entries by a depth field.
- `apply_fractional_offset`: When a byte field is set, computes a fraction of a 16-bit value scaled by 64 minus that byte, stores the fraction, and subtracts it from a companion field.
- `update_sprite_geometry_from_table`: Recomputes a sprite record's size, position and step values from a table entry indexed by a stored byte, scaling by a divisor.
- `update_sprite_by_mode_table`: Recomputes sprite size, position and step values from a mode-indexed table for records with a visible flag.
- `update_sprite_from_fixed_table`: For records with a specific flag, recomputes size, position and step values from fixed offset tables indexed by a stored mode value, scaled by a divisor.
- `render_sprite_chain_queue`: Walks the sprite chain, drawing records whose position falls below a threshold through a helper, then waits until a pending counter reaches zero before returning.
- `clip_sprite_to_screen_area`: Tests a sprite against a 102-by-320 area.
- `advance_angle_timer_table`: Counts down a short timer and, when it expires, loads the next entry from a table.
- `clear_rectangle_in_planes`: Clears a rectangle in the bitplanes by writing zero to three planes and all-ones to the fourth.
- `mark_record_after_helper_call`: Sets up a record pointer and a mode value, calls a helper once, and then marks the record's byte at offset 27 as all-ones.
- `copy_panels_and_update_objects`: Copies several plane regions from source buffers into working buffers and fills their sentinel areas.
- `map_records_to_screen_cells`: Walks the record chain and, for each record in view, converts its position to a screen cell using scaled coordinate offsets.
- `dispatch_position_in_range`: Checks a position against two small bounding ranges.
- `setup_blitter_rectangle_copy`: Programs blitter hardware registers for a three-row block copy: it writes mask words, a control word with the logic-function value, source and destination addresses and row modulo values.
- `setup_blitter_rectangle_copy_2`: Alternate entry into the blitter setup loop: it writes the same mask, control and address register values for four planes spaced 8000 bytes apart, calling a helper between them.
- `setup_blitter_masked_copy`: Programs the blitter for the same four-plane block copy as its neighbour, with a different row size and mask values, and waits for the operation to finish through a flag-wait helper.
- `setup_blitter_multi_pass_copy`: Programs the blitter registers in three passes over the planes, then issues one more pass with a different control word and a size of 256.
- `disable_then_restore_interrupts`: Clears two state fields, writes the interrupt enable register with the enable bit clear, waits through two helper calls, then sets the enable bit again and clears a further field.
- `increment_pending_step_counter`: Adds one to a 16-bit pending step counter held in the state block at a fixed offset.
- `read_controls_and_fill_slot`: Reads the fire-button and second-button registers and updates a direction value with change detection.
- `fill_slot_and_update_sprites`: Continues the slot update: fills the first free slot when fire is active, writes two pairs of 16-bit position words into the display buffer, and then writes a set-bit value to a hardware control register.
- `update_fire_slots_and_sprites`: Updates the slot list and sprite entries when a flag is set, otherwise it takes the other branch.
- `write_slot_sprite_entries`: Fills the first free slot from a table, writes two pairs of position words into the display buffer, and sets the hardware control register with a set-bit value.
- `test_slot_against_rectangles`: Checks a slot's position against a table of 20 rectangular ranges.
- `fill_slot_and_hit_test`: Fills the first free slot from a table with position and velocity values, writes display buffer entries, and then tests the slot against a set of rectangular ranges, calling a handler on a match.
- `store_projected_slot_position`: Computes a scaled position for one slot from an angle-based offset and stores it with a size value.
- `flag_slot_when_idle`: Sets a state flag to 1 when a stored counter is zero or a table entry is zero, as part of the slot update.
- `check_slot_inside_panel`: Checks whether a slot lies inside a fixed rectangle of screen coordinates and, if it does, points the current record at that slot and sets its mode.
- `select_slot_display_mode`: Chooses one of several hardware control words for the current slot display mode, stores them in the state, and continues into the common update path.
- `test_slot_box_enable_dma`: Compares a slot's coordinates with a stored bounding box and, when they overlap, updates the display state and writes the DMA control register.
- `store_pending_slot_arguments`: Clears a pending flag, calls a helper, and stores the three arguments passed in as the current slot's state values.
- `save_slot_state_arguments`: Stores the first three arguments into the current state words and returns the fourth as its result.
- `save_slot_state_arguments_2`: Alternate entry of the argument-saving routine: it stores three arguments into the current state words and returns the fourth argument.
- `save_slot_state_arguments_3`: Another entry into the argument-saving routine, which stores three arguments into the current state words after a helper call and returns the fourth.
- `dispatch_slot_handler_by_index`: Calls a handler chosen by an index from a table, sets a state flag when a counter is non-zero, writes a control value to a hardware register, and then stores three arguments into the current state words.
- `lookup_slot_table_value`: Looks up a value in the slot table through a pointer and a helper call, returning the result as a 32-bit value.
- `dispatch_mode_handlers_then_save`: Calls a table-indexed handler once for low modes and twice for higher modes, with an extra call for mode 8, then stores three arguments into the current state words.
- `test_mode_four_slot_rectangle`: Calls a table handler and, in mode 4, checks the slot's position against a fixed rectangle after setting a control word.
- `reset_or_advance_slot_mode`: When the input value is 8, resets the slot mode to zero and calls a helper if one is set.
- `advance_slot_mode_counter`: Writes a control value, calls a helper, increments the slot mode, clears a pending flag, and stores three arguments into the current state.
- `update_slot_inside_panel_mode`: When the slot's counter is below 176, writes a control value and tests the slot against a panel rectangle.
- `update_slot_inside_panel_mode_2`: Performs the same panel rectangle test as its neighbour without the counter threshold, pointing the state at the slot when it is inside and choosing control words by a flag.
- `mark_slot_mode_advanced`: Sets a pending flag, calls a helper, writes a control value, increments the slot mode, and stores three arguments into the state.
- `update_slot_inside_panel_mode_3`: Writes a control value and runs the panel rectangle test; inside the rectangle the state points at the slot and control words follow a flag, then the arguments are stored.
- `scan_slots_for_rectangle_hit`: Writes two control words, then walks a list of slots and stops at the first visible slot whose position falls inside the given rectangle.
- `store_pointer_value_advance_mode`: Copies a 16-bit value from a pointer into the state, sets a pending flag, increments the slot mode and clears another flag.
- `copy_plane_block_to_buffer`: Copies an eight-row block of four-plane words from one buffer into a second buffer, 40 bytes per row, with plane spacing of 2000 words, after a helper call.
- `dispatch_by_mode_index_table`: Returns at once when the input is zero.
- `select_handler_for_index`: Calls one of two table handlers depending on whether the input is 12 or 24; any other value uses the default handler entry.
- `call_default_slot_handler`: Calls the default handler from a fixed table entry and returns.
- `call_handler_by_offset`: Calls a handler at the table offset given in the input register and returns.
- `call_handler_with_slot_offset`: Looks up a handler and an offset in a state table entry, then calls the handler with the offset added to the base value stored in the state.
- `call_handler_from_pending_entry`: Calls a handler taken from the pending entry, passing that entry's offset added to a base value.
- `lookup_slot_value_by_index`: Looks up a value for the current slot through pointer tables and a helper call, returning it as a 32-bit value.
- `copy_text_from_table_entry`: Entry into a routine that copies character data for a table entry into a text buffer, using offsets taken from a table and a helper call.
- `advance_record_frame_index`: Advances a per-record frame index by the current step, wrapping back to zero when it passes the top value.
- `advance_record_frame_index_2`: Alternate entry of the frame update, which advances the frame index with the same wrap and writes it into the matching record found by scanning the slot list.
- `toggle_record_flag_and_scan`: Toggles a flag byte on a record and scans the slot list, writing the returned value into the matching record.
- `pulse_helper_twenty_times`: Temporarily clears a state flag, calls a helper twenty times in a loop, then restores the flag and calls another helper.
- `forward_three_helper_calls`: Calls three helper routines in order and returns the value passed on the stack.
- `forward_two_helper_calls`: Calls two helper routines in order and returns the value passed on the stack.
- `forward_one_helper_call`: Calls one helper routine and returns the value passed on the stack.
- `forward_one_helper_call_2`: Calls a single helper routine and returns its result unchanged, serving as a passthrough wrapper.
- `forward_helper_call_2`: Invokes a single helper routine and returns its result unchanged, acting as a thin passthrough wrapper.
- `forward_helper_call`: Calls a single helper routine and returns its result unchanged.
- `forward_helper_return_pair`: Calls one helper routine and returns the pair of 32-bit values held in the data registers.
- `swap_field_around_helper`: Saves a 32-bit state field, replaces it with a value passed in, calls a helper, then restores the saved value.
- `call_handler_indexed_word`: Calls a handler from a table using a word index that is multiplied by four.
- `format_state_text_field`: Builds a text field from state data by copying characters through helper calls and returns a 16-bit position value.
- `build_text_field_from_pointer`: Builds a text field from data reached through a pointer, copying characters with helper calls and returning a 16-bit length value.
- `write_text_field_prefix`: Sets a text pointer into the status buffer and writes a short sequence of characters up to a fixed end marker.
- `write_label_after_helper_call`: Calls a helper and then writes a short run of character codes into the text buffer that the helper has prepared.
- `measure_text_length`: Counts the characters of a zero-terminated text up to the terminator, then uses that length to set a text buffer position.
- `select_text_label_by_index`: Chooses a text label from a table and copies it into the text buffer, returning a short value from the copy.
- `write_stop_label`: Writes the STOP label into a text buffer through helper calls and returns a short value for the position used.
- `write_full_label`: Writes the FULL label into a text buffer, using a value passed in to choose the row, and returns.
- `write_marker_character`: Calls a helper and then writes an exclamation mark into the text buffer at the position the helper prepared.
- `format_numeric_text_field`: Formats a numeric value into a text field through helper calls and returns a short position value.
- `clear_text_marker`: Calls a helper and then clears the marker byte at the text position it prepared.
- `format_text_field_with_length`: Builds a text field by copying characters through helper calls and returns a short position value.
- `write_text_field_variant`: Writes a text field through helper calls, using a different text source from its neighbour, and returns a short value.
- `return_short_after_helper`: Calls one helper while writing a text field, then returns a short value.
- `format_text_field_alt_entry`: Alternate entry into the text field routine: it builds a text field through helper calls and returns a short position value.
- `process_table_entries_until_end`: Walks table entries by an offset, copying fields from each entry into the state until an end marker.
- `copy_text_cells_with_offset`: Copies character cells between text positions using a computed offset, moving runs of bytes and words within the text buffer.
- `copy_text_row_cells`: Copies a few words and bytes between two rows of the text buffer, using offsets from the caller.
- `write_number_decimal_digits`: Writes a 16-bit value as ASCII decimal digits, hundreds first, and leaves out any leading digit that is zero.
- `write_compact_decimal_value`: Writes a value as one or two decimal digit characters, adding a tens digit only when the value is 10 or more, then the units digit.
- `copy_text_label_entry`: Copies a text label from a table into the text buffer through helper calls.
- `write_indexed_text_entry`: Writes a text entry chosen by a register index into the text buffer through helper calls.
- `fill_text_row_remainder`: Fills the rest of a text row by counting down from sixteen minus a width value and writing characters until the row is full.
- `copy_word_pairs_until_end`: Walks a byte-coded list and copies word pairs into an output list, stopping at an end marker.
- `return_immediately_2`: This routine is empty and simply returns without performing any operation.
- `update_cell_and_mask_columns`: Entry into a routine that converts record positions to cells by dividing by 6000, compares them with the last cell, and when they change refreshes the column mask table and redraws affected column lines.
- `update_cell_and_mask_columns_2`: Alternate entry into the same cell update routine, which divides positions by 6000 and refreshes the column mask table when the cell changes.
- `update_cell_and_mask_columns_3`: Another entry into the cell update routine that converts a position to a cell, compares it with the previous cell and refreshes the column masks on change.
- `update_cell_and_mask_columns_4`: Converts a record's position into a cell by dividing by 6000.
- `fill_handler_grid`: Fills a grid of fifteen by fifteen entries with a handler pointer, starting from an offset held in the state.
- `fill_handler_grid_with_checks`: Fills the same fifteen-by-fifteen grid as its neighbour, with extra checks on each cell before the handler pointer is stored.
- `draw_column_fill_lines`: Draws vertical fill lines into the planes for each column that has a valid start, writing a fill value upward with a 40-byte stride, and returns a short result.
- `mark_visible_edges`: Walks an edge list of three endpoint pairs and marks a slot as visible when the endpoints fall inside the clip area.
- `order_pair_and_test_flags`: Orders two coordinate values, then checks flag table entries at both ends to decide whether the edge is visible, returning a 32-bit value.
- `clip_edge_to_window`: Clips a line edge against a window by computing the intercept from halved deltas and storing the clipped endpoints.
- `order_pair_and_call_helper`: Orders two 16-bit values so the smaller comes first, then calls a helper and returns both values packed into one result.
- `update_table_with_interrupts_off`: Disables an interrupt source through the interrupt enable register, updates a table of entries with computed values through helper calls, then restores the enable bit.
- `poll_joystick_buttons_state`: Reads the joystick fire-button port and the potentiometer register, updating button state variables and returning a 32-bit value.
- `setup_display_dma_and_planes`: Writes the DMA control register and programs the display plane pointers for the playfield, calling helpers at each step.
- `return_stack_value_after_calls`: Calls two helper routines and returns the value passed on the stack unchanged.
- `return_stack_value_after_calls_2`: Calls helper routines and returns the value passed on the stack unchanged.
- `toggle_interrupt_and_update_state`: Writes the interrupt enable register to disable a source, updates a state table through helper calls, then re-enables it and returns the stack value.
- `update_state_table_entry`: Looks up an entry in a state table using a 16-bit index, updates its fields through helper calls, and returns.
- `load_table_pair_into_state`: Loads a pair of 32-bit values from a table entry chosen by an index into fixed state fields, then calls a helper.
- `copy_word_block_between_pointers`: Copies a block of 32-bit words between buffers selected by pointers.
- `copy_masked_word_rows`: Entry into a routine that writes 16-bit words into a row buffer, shaped by a byte selector and a width value.
- `copy_masked_word_rows_2`: Alternate entry into the same word row routine, which starts at a different point in the row buffer.
- `copy_masked_word_rows_3`: Entry into the word row routine with a different selector, writing 16-bit words into a row buffer.
- `copy_masked_word_rows_4`: Word row routine entry that calls a helper and writes words into the row buffer; the record is truncated.
- `fill_empty_entries_with_value`: Walks sixteen entries spaced 96 bytes apart and sets each entry's empty field to a supplied value when that field is zero.
- `forward_state_pair_to_helper`: Reads a pair of 16-bit state fields and a table reference, calls a helper with them, and returns two 32-bit values.
- `scan_masked_linked_entries`: Walks a linked list of entries, testing each entry's flag word against masks, and returns the stack value.
- `scan_masked_linked_entries_2`: Alternate entry of the masked list scan, stepping through the list with a different link offset.
- `setup_blitter_copy_registers`: Writes mask words, a control word, and address and modulo values to the blitter registers from state fields, then calls a helper.
- `setup_blitter_source_registers`: Writes mask words, control words, and source address and modulo registers for the blitter from state values, then returns.
- `write_column_blit_control`: For a column index within the range minus fifteen to 319, computes a control word from the index shifted into the top bits, combines it with a fixed logic value, and writes it to the blitter control register.
- `write_column_blit_control_2`: Alternate entry of the column control routine, which computes and writes the same blitter control word for an in-range column index.
- `write_column_blit_control_3`: Alternate entry that computes the column control word for an in-range column and stores it in a shadow location instead of the blitter register.
- `write_column_blit_control_4`: Writes the column control word to the blitter control register for an in-range column index; a neighbouring entry does the same computation.
- `write_column_blit_control_5`: Entry into the column control routine that computes the blitter control word for an in-range column index.
- `setup_blitter_column_copy`: Programs the blitter registers for copying one column of tiles, setting mask words, modulo, address and control values.
- `setup_blitter_four_pass_fill`: Runs a four-pass loop that writes constant values and all-ones masks into a table, calling helpers between passes, after setting a base pointer from the state.
- `set_row_pointer_from_state`: Computes a buffer pointer from a state value, offset by a fixed amount and scaled by 40, then writes fixed constants and calls two helpers.
- `draw_row_block_fixed_offsets`: Sets a buffer pointer at a fixed offset from the state, writes fixed constants, and runs a four-pass loop that calls a helper on each pass.
- `draw_row_block_second_layout`: Writes a second set of pointer offsets and constants into the shadow registers, then calls helpers three times in sequence.
- `set_row_pointer_from_slot`: Computes a buffer pointer from a slot value scaled by 40 and writes fixed constants, then calls a helper and repeats with a second pointer.
- `setup_blitter_four_pass_copy`: Runs a four-pass loop that writes the blitter mask words, control word and address registers, calling a helper between passes.
- `convert_tile_rows_to_bitplanes`: Copies a block of tile rows between two buffers, then converts tile bytes through lookup tables into masked bitplane words and merges neighbouring rows with a masked exclusive-or.
- `allocate_and_store_memory_blocks`: Requests several memory blocks through a library call, storing their base and derived addresses into a structure.
- `copy_planes_and_swap_buffers`: Copies four bitplanes from the source buffer into separate destination planes and fills blank areas with ones, then swaps the two screen buffer pointers.
- `swap_buffers_and_wait_busy`: Swaps the two buffer pointers held in the state block and sets a flag.
- `copy_planes_back_to_buffer`: Copies 1280 longwords from four separate plane buffers back into the interleaved destination image.
- `fill_checkerboard_mask_rows`: Clears a block of ten longwords in the state block, stores a value looked up from a table, and fills a region of rows with alternating checkerboard words.
- `compute_clipped_viewport_bounds`: Chooses one of two layout presets from a mode flag, then clamps the stored x and y position into the range allowed by that preset.
- `update_object_list_offsets`: Resets two list-head fields, then walks a linked list of object records in two passes.
- `turn_angle_toward_target`: Turns a record's current angle toward a target angle by a speed-limited step, taking the shorter path and wrapping at a full 360-degree circle.
- `advance_subpixel_position_by_table`: Multiplies a signed table value by a scale factor for each axis, adds the integer part to the record's position and stores the masked remainder as a sub-pixel byte.
- `compute_angle_to_target_vector`: Works out the angle from a record's position to a target point using an iterative shift-and-add approximation over a table of constants.
- `compute_angle_to_target_vector_2`: Same body as the neighbouring entry point, computing the angle from a record's position to a target point with an iterative shift-and-add approximation and quadrant correction.
- `compute_angle_to_target_vector_3`: Same angle computation as the neighbouring entries: it works out the angle from a record's position to a target point with an iterative shift-and-add approximation and applies quadrant correction from the sign flags.
- `render_masked_spans_to_planes`: For each row, walks a list of horizontal spans and ORs table-driven mask bits into accumulators, then writes three bitplane words per column.
- `render_spans_to_four_planes`: For each row, walks a list of spans, builds four plane accumulators from table masks, and writes each finished byte into four bitplanes.
- `build_scanline_edge_table`: Builds a per-scanline edge table for a shape by stepping a fixed-point slope accumulator across the rows.
- `allocate_slot_for_object_cached`: Returns a cached index for the current object when it has not changed since the last call.
- `allocate_slot_for_object`: Clears the cache flags and selects a slot for the current object, returning either its index or its scaled slot offset.
- `allocate_slot_or_pass_input`: Runs the same object-slot allocation as its neighbours, but when a gate check fails it returns the caller's input value unchanged.
- `select_slot_if_mode_clear`: Returns the caller's input unchanged while a mode word is nonzero.
- `select_object_slot_unconditional`: Computes the object's slot value from its flags and fields, then selects or allocates a slot in the same way as the neighbouring routines.
- `stub_immediate_return`: The body is a single immediate return with no work, so it is a stub that does nothing visible.
- `claim_slot_and_start_transfer`: When an object has a nonzero value and no transfer is pending, scans the slot table from the top for a free entry.
- `claim_slot_for_linked_object`: Follows the object's link when the input is non-negative.
- `empty_return_routine`: The routine contains no instructions and returns immediately without reading or writing any data.
- `reset_transfer_state_machine`: Sets the transfer state word to zero, returning the transfer state machine to idle.
- `enqueue_transfer_record`: Stores the next input item into a free entry of a table of up to 32 twelve-byte records and gives it the next sequence number.
- `run_transfer_state_machine`: Advances a three-stage transfer state machine.
- `draw_text_string_to_planes`: Renders a zero-terminated string into four bitplanes one character at a time, using an eight-row glyph table.
- `format_digits_and_step_timer`: Converts a number held in a fixed register into decimal digits by repeated subtraction of a table of powers of ten, writing a minus sign for negative entries.
- `format_digits_and_check_marker`: Performs the same decimal digit formatting and timer step as its neighbour.
- `draw_string_or_patch_labels`: Draws a zero-terminated text string into four bitplanes, like its neighbour, using the string at a fixed code location.
- `advance_countdown_and_scan_records`: Subtracts a shift-scaled step from a countdown and clamps it at zero.
- `advance_countdown_scan_records_unguarded`: Performs the same countdown step and record-list scan as its neighbour, without the early exit check on the mode word.
- `scan_list_and_restore_state`: Scans a record's eight-entry list for a negative marker and flips it to positive.
- `flip_marker_and_restore_state`: Negates the entry just before the current list pointer, then steps through the record list until it is finished.
- `restore_saved_list_state`: Writes the four saved list-state words back into the state block.
- `build_variant_sequences`: Builds eight variant sequences by loading values from a source table and storing them into a destination table through a pointer, repeating the process eight times.
- `copy_twelve_bytes`: Copies 12 bytes from a source table into a destination table, advancing both pointers by one byte per iteration until all bytes are transferred.
- `build_eight_variant_sequence_tables`: Copies 25 two-word entries from a staging table into a buffer, then builds eight variants of that buffer by selecting each source value or a transformed value depending on the variant index.
- `redraw_panel_and_format_readout`: Copies 4000 words for each of four bitplanes from a source image into the screen, then updates the panel state.
- `add_decimal_digits_to_text`: Adds an eight-digit decimal table to the eight-character text at the pointer, carrying from the least significant digit and rewriting each character.
- `subtract_decimal_digits_from_text`: Subtracts an eight-digit decimal table from the eight-character text in place, carrying a borrow into the next digit.
- `unpack_picture_to_four_planes`: Expands packed picture bytes into four bitplanes row by row, handling odd-byte alignment and an optional inverted, scrambled mode.
- `wait_for_fire_button_cycle`: Polls the fire-button and second-button bits in the hardware input registers until the fire bit reads pressed with the second button set.
- `update_hit_flags_in_slice`: Advances a frame counter and walks the object list in time slices, selecting records by a phase value.
- `select_record_and_fill_buffer`: Marks the record as selected when it is the current one, copies its identifier and fills a 180-byte buffer with ones.
- `insert_record_into_sorted_list`: When a record is at the end of its list, saves the list state and finds the sorted position for its key.
- `search_list_for_matching_record`: Searches the record list for the current selection key by stepping through records with their offset field.
- `blit_object_across_four_planes`: Works out a column offset from the object's animation field and a table, then programs a fixed block of hardware registers for each of four plane rows.
- `find_record_in_proximity_box`: Returns the record's flags if it is already finished or selected.
- `reselect_finished_record`: When the record is marked finished, runs one step of the list scan in a specific mode.
- `animate_marker_entries_in_planes`: Runs a short countdown for the current selection, and while it is active it updates a table of up to eight three-byte marker entries.
- `spawn_pending_objects_from_lists`: Takes up to eight pending entries from a queue and turns each into an object record, copying its position and type from the input.
- `update_nearby_object_damage`: Walks the object list and, for each record within 64 units of the current target, reduces its energy value and bumps its hit counters.
- `advance_object_state_timers`: Walks the object list and advances each record's timer and state byte through a set of thresholds.
- `clear_matched_pending_ids`: Compares the input identifier with two pending identifiers.
- `chain_five_helper_calls`: Calls five helper routines in order and returns.
- `repeat_helper_until_zero`: Repeatedly calls a helper and a second helper until the helper's result register is zero, then returns.
- `test_point_against_region_list`: Walks a list of six-longword region entries selected by an index and checks whether the point in the two data registers lies inside a region's bounds.
- `advance_random_seed_mod_65521`: Advances a 16-bit random seed by multiplying it by 125 and reducing the result modulo 65521.
- `build_bounds_entry_from_offsets`: Builds a region entry in the next free slot from a source record.
- `terminate_entry_list`: Marks the entry at a fixed offset with the end marker and points the free-entry pointer just past it.
- `initialize_object_record_from_template`: Fills a new object record from a template in the level's data table, copying its graphics references, type code, position and hit parameters.
- `update_object_record_from_template`: Copies template fields into an existing object record and sets its defaults and flags, like the tail of its neighbour.
- `stub_immediate_return_2`: The body is only an immediate return that passes back its input, so it is a stub that does nothing visible.
- `request_transfer_in_mode_two`: Writes a transfer request block with mode two and a control value with its top bit set, then calls a helper to carry it out.
- `request_transfer_in_mode_five`: Writes a transfer request block for mode five and calls a helper to process it, then returns.
- `call_helper_and_return`: Calls a helper routine and returns.
- `find_first_unflagged_list_entry`: Issues a request block to a helper, then walks a list of values ended by a sentinel.
- `request_transfer_in_mode_five_2`: Writes the same transfer request block for mode five as its neighbour, then calls a helper to process it.
- `empty_return_routine_2`: This routine contains no instructions; it is an empty routine that returns immediately.
- `empty_return_routine_3`: This routine is empty.
- `store_position_into_record`: Stores the two coordinate registers into fixed offsets of an object record, setting its position fields.
- `increment_table_byte_by_index`: Adds one to the byte in the global table at the index held in the low word of the data register.
- `update_objects_and_check_proximity`: Walks the object table from its start, skipping empty and inactive entries.
- `resume_object_walk_with_proximity`: Performs the same object-table walk and proximity check as its neighbour, but resumes from the current table pointer rather than the start.
- `fire_weapons_at_targets`: Clears a marker bit on every object, then walks up to 100 active entries.
- `fire_weapons_at_targets_2`: Continues the same target-selection loop as its neighbour from a count parameter.
- `fire_weapons_at_targets_3`: Another entry into the same target-selection loop, with the same range test and marking.
- `update_targeted_object_flags`: When an object is marked as targeted, sets its active bit and sets a second status bit chosen by two other flag bits.
- `stub_immediate_return_3`: The body is a single immediate return with no work, so it is a stub that does nothing visible.
- `store_value_into_negative_slot`: It scans a table of signed words until it finds a negative entry, stores the incoming value into that slot, and stops.
- `tiny_move_movea_rts`: Small routine of 3 instructions that copies (a7)+ to d1, then copies (a7)+ to a0, then returns.
- `update_object_state_and_target`: Sets the object's display state byte from its flags.
- `select_nearest_candidate_or_reset`: Selects the candidate with the smallest value from a linked list and stores its index.
- `compute_scaled_squared_distance`: Takes the absolute differences of two coordinate pairs, scales each by 1000 and returns the sum of their squares.
- `try_assign_threat_to_target`: When an object is neither inactive nor disabled, it checks whether its timer has passed a threshold chosen by its type code.
- `try_assign_target_against_limit`: Checks a candidate's value against the limit in the data register.
- `compare_value_to_constant`: Compares the low word of the data register with the value 69 and returns either way, so the comparison has no effect.
- `pick_largest_weighted_field`: Scans a fixed range of records and compares each one's weighted counter fields, keeping the largest.
- `pick_weighted_field_from_pointer`: Performs the same weighted counter scan as its neighbour, but it continues from the current pointer and keeps its best value in registers.
- `search_list_for_best_match`: Walks a candidate list, tracking the smallest value for entries with one code and otherwise testing each entry's flags and index range.
- `search_list_for_best_match_2`: Walks a candidate list with a different index-range test and the same tracking of the smallest value for one code.
- `walk_record_list_until_record`: Sets up a request block and walks the record list from its first record, comparing each entry's key against the value in the data register.
- `empty_routine`: This routine contains no instructions and performs no operations.
- `walk_list_until_key_match`: Walks the record list from the top, subtracting a stored key from each record's field until the difference reaches zero or the given record is reached.
- `queue_request_by_code`: Stores a request in one of two fixed queues depending on a code value.
- `relink_and_compact_record_table`: Rebases each record's link to an absolute address, then removes records flagged as finished from the table.
- `relink_and_compact_record_table_2`: Performs the same link rebasing and compaction pass as its neighbour, using a second record row layout.
- `relink_and_compact_record_table_3`: Performs the same link rebasing and compaction pass again, reading the links from a different row offset before compacting the table.
- `clamp_point_to_grid_cell`: Finds the nearest point of an adjacent grid cell, on a 6000-unit grid, to a given position.
- `grid_cell_to_world_corner`: Converts a grid cell index pair to world coordinates by scaling each index by 6000 units.
- `find_nearest_rectangle_to_point`: Walks a table of rectangles and computes the squared distance from a point to each one, which is zero inside.
- `find_nearest_rectangle_stepped`: Performs the same nearest-rectangle search as its neighbour on a different table layout.
- `find_nearest_rectangle_x_first`: Performs the nearest-rectangle search, testing the horizontal bounds first before the vertical ones.
- `map_type_index_to_code`: Returns a fixed code for three special type values.
- `lookup_word_at_table_offset`: Returns the word stored in the global table at the byte offset given by the low word of the data register.
- `tick_timer_with_link_swap`: When an object's timer has come due, swaps its link field with another value and runs helper checks, possibly decrementing a shared count.
- `select_sequence_for_current_type`: Picks an entry from one of several tables depending on the current type code and the timer, sets a status flag accordingly and returns a sequence value.
- `test_point_in_rectangle`: Tests whether the point in the two data registers lies inside the rectangle at the pointer.
- `set_flag_and_queue_entry`: Sets a status flag.
- `clear_flag_and_queue_entry`: Clears the status flag.
- `process_negative_check`: Reads a signed word from a structure and, when that value is not negative, sets a global flag, invokes two other routines, and clears the checked word.
- `set_flag_and_clear_field`: It checks a signed word in a structure and, when the value is not negative, sets a global flag, invokes two other routines, and clears that structure field.
- `store_scaled_byte_for_type`: Stores a byte from a per-type table, doubled, into the shared value word for the current type.
- `store_table_byte_times_factor`: Stores a byte from a per-type table, multiplied by a fixed factor, into the shared value word for the current type.
- `compute_weighted_score_and_level`: Sums table weights over a list of objects to produce a weighted score.
- `compute_weighted_score_alt`: Performs the same weighted score and level calculation as its neighbour, storing the chosen index in a different global word.
- `compute_index_from_divided_value`: Divides the data register value by a byte from the record, clamps the result at 12, and stores it unless it is the no-value marker.
- `request_transfer_in_mode_one`: Writes a transfer request block for mode one with a high control value, then calls a helper and returns.
- `return_helper_pair_values`: It calls a helper routine and returns a pair of values, taking the result directly from the incoming condition flags regardless of whether the flag is set.
- `increment_category_counter_for_record`: Increments one of the record's per-category counters.
- `increment_category_counter_in_table`: Increments the per-category counter in the table at the pointer, choosing the category from the lowest set bit of a flag field.

### strikeiv.prg

- `run_music_interrupt_session`: Sets the task priority, initialises the music player, installs the timer interrupt and waits for the fire button.
- `Wait_For_End`: Polls the fire-button and second-button bits in the input registers and returns as soon as the fire-button bit reads clear or the second-button bit reads clear.
- `Set_Irq`: Programs the CIA-B timer and interrupt control registers with fixed values, stores a previous register value, and points the handler at the new interrupt routine.
- `Return_Irq`: Writes the interrupt control register, restores the saved register value, and clears the enable bits for the low four DMA channels.
- `Set_Task`: Finds the current task through the system library and sets its priority.
- `New_Interupt`: Writes the interrupt request register and calls the music routine mt_music.
- `mt_init`: Initialises the ProTracker music replay: reads the song's pattern order table, sets up the sample pointers and resets the song position.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `mt_music`: ProTracker music replay tick, called once per frame: advances the song and feeds new notes and effects to the four Paula audio channels.
- `mt_wait`: The decompiled body is only an empty infinite loop, so it stalls with nothing else to do.
- `mt_nonew`: Runs the effect update for each of the four voices, then advances the song position with wrap at 128.
- `mt_playvoice`: Reads the next note event for one voice from the pattern data.
- `mt_noloop`: Sets the voice's sample position after a loop, then processes the note and effect command in the same way as the playback routine.
- `mt_stopsound`: Applies the effect command of a stopped voice.
- `mt_setport`: Sets the portamento target period and direction flag for a voice from its note period, then applies the same effect command handling as the playback routine.
- `mt_clrport`: Clears the voice's portamento target word to zero.
- `mt_port`: Applies one step of the portamento slide for a voice.
- `mt_port2`: Performs one portamento slide step for a voice.
- `mt_sub`: Subtracts the step in the data register from the voice period and clamps it at the target, clearing the target once it is reached.
- `mt_vib`: Applies a vibrato step to a voice.
- `mt_vib2`: Performs the same vibrato step as its neighbour, without first updating the depth from the effect parameter.
- `mt_vibsub`: Writes the difference between two values to the voice period, then advances the vibrato position by its speed.
- `mt_arp`: Applies an arpeggio step.
- `mt_arp2`: Applies an arpeggio step using the low nibble of the effect parameter as the note offset.
- `mt_arpfound`: Sets the voice period to the entry found at an offset into the arpeggio period table, given by the data register.
- `mt_normper`: Writes the voice's base period back to its period output, with no change to the value.
- `mt_com`: Dispatches the effect command for one voice by its type nibble: arpeggio, portamento up or down, tone portamento, vibrato, or the combined slide and vibrato commands.
- `mt_portup`: Raises the pitch by subtracting the effect speed from the voice period, clamping at the highest allowed period, and writes the result to the output.
- `mt_portdown`: Lowers the pitch by adding the effect speed to the voice period, clamping at the lowest allowed period, and writes the result to the output.
- `mt_volvib`: Applies a vibrato step and then a volume slide.
- `mt_volport`: Applies a tone portamento step and then a volume slide.
- `mt_vol3`: Lowers the voice volume by the low nibble of the effect parameter, not below zero, and writes the result to the volume output.
- `mt_com2`: Applies the secondary effect command for a voice: a filter toggle on the shared DMA control value, a pattern break, a pattern jump, a speed change, or a set volume capped at 64.
- `mt_filter`: Sets or clears one bit of a shared port register from the low bit of the effect parameter.
- `mt_pattbreak`: Sets the song position counter to 1, which ends the current pattern early.
- `mt_songjmp`: Jumps playback to the song position given by the effect parameter.
- `mt_setvol`: Sets the voice volume from the effect parameter, capped at 64, and writes it to the volume output.
- `mt_setspeed`: Sets the playback speed from the effect parameter, limited to a range of one to 31.
- `mt_speed`: Small routine of 1 instruction that adds 0 to d0.
- `mt_songpos`: The recovered pseudo-code is only bad-instruction stops, so this address holds data rather than code.
- `mt_samplestarts`: The recovered pseudo-code is only bad-instruction stops, so this address holds a table rather than code.
- `mt_voice1`: Loads a byte from a global variable into a register and then branches to a routine whose address was taken.
- `mt_voice2`: It reads a byte from a global variable and, depending on its value, stores a byte back through a pointer.
- `mt_voice3`: Reads a byte from a global variable, ORs it with the value 12, and stores the result back to the same location.
- `mt_voice4`: Reads a byte from a global variable and stores it back into the same global variable.

### sf.prg

- `run_main_loop_after_setup`: Program entry point.
- `open_read_and_close_file`: Opens a file through the DOS library and reads from it when the open succeeds.
- `write_file_through_dos_library`: Opens a file through the DOS library and writes to it, closing the file and the library afterwards.
- `open_dos_library`: Opens the DOS library by name through the system library call and stores its base address for later calls.
- `close_file_and_report_error`: Fetches the DOS error code, closes the file and the library, and returns an error marker.
- `install_copper_and_mask_vblank`: Forbids task switching and opens the graphics library.
- `dispatch_pending_interrupt_bits`: Checks the interrupt request bits.
- `handle_vblank_interrupt_bit`: When the vertical-blank bit is set in the input, restores the saved copper list pointer, calls a helper and increments a counter.
- `fill_table_words_and_pause`: Fills eight entries of a four-word table with two fixed words, then waits through a nested countdown loop.
- `wait_vblank_then_enable_interrupt`: Waits until the vertical-blank interrupt request bit is set, then re-enables that interrupt through the interrupt enable register.
- `split_bitplane_addresses_into_words`: Splits a 32-bit buffer address into high and low words, and stores the pointer pairs for four bitplanes spaced 8000 bytes apart, ready for the display hardware to read.
- `call_exec_permit_and_return`: Calls the exec permit routine to re-enable task switching, then returns.
- `call_indirect_routine`: Invokes an indirect routine located via a fixed RAM offset and returns the two registers supplied at entry.
- `forbid_task_switching_2`: Calls the exec forbid routine to stop task switching, then returns.
- `run_routine_from_jump_table`: Gets a selector value from a library call.
- `restore_copper_and_write_output`: Restores the saved copper list pointer through the graphics library, then writes a short text to the DOS output stream.
- `stub_bad_instruction_data_2`: The recovered pseudo-code is only a bad-instruction stop, so this address holds data rather than code.
- `write_eight_digit_characters`: Converts a 32-bit value into eight characters by mapping each four-bit nibble through a lookup table.
- `copy_image_planes_to_buffer`: Copies 4000 words for each of four bitplanes from the stored image into the display buffer.
- `copy_image_planes_to_buffer_2`: Performs the same plane copy as its neighbour without the leading helper call.
- `allocate_display_buffers`: Allocates a block of memory through the exec library and stores its base address.
- `free_allocated_memory_block`: Calls the exec free-memory routine through the system library and stores the value it returns in the buffer variable.
- `load_strikeiv_program_file`: Runs two helper routines around a call into the DOS library, then closes the library.
- `unpack_lzss_stream`: Decompresses an LZSS-style stream into a buffer.

### Strikefleet.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
