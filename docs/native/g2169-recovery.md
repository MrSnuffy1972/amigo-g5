# Game 2169 (black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[f dtc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02169). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (920784 top-level bytes in 32 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 20 of 20 units identical (bytes and relocations), 15 raw-word lines |
| Functions named and described | pass, 388 of 388 named (0 author symbols), 388 of 388 described (32 by rule, 28 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1702 of 1702 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12045 of 12045 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 654718 |
| code | 38296 |
| data | 215788 |
| padding | 7374 |
| unused | 4608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:z.DYTEC | 6 | 251 | 6 |
| Run | 1 | 93 | 1 |
| Stack | 2 | 117 | 2 |
| Avail | 1 | 141 | 6 |
| runback | 3 | 709 | 39 |
| endcli | 2 | 117 | 2 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:z.crack | 1 | 36 | 2 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:InstallCrypt | 7 | 45 | 2 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:BlackCrypt | 7 | 253 | 7 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:maus | 1 | 3 | 1 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:note | 2 | 24 | 4 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:bcdfp | 3 | 6150 | 184 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:bcdfq | 2 | 983 | 56 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:bcdft | 7 | 45 | 2 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 766 | 25 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1600 | 45 |

## Cases handled

- runtime segment an address: 523 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 689 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[f dtc]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:z.DYTEC: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[f dtc]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:InstallCrypt: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[f dtc]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:BlackCrypt: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[f dtc]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:bcdft: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:z.crack

## Copy protection found

- none found

## Functions


### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:z.DYTEC

- `program_entry_with_cleanup_loop`: Program entry that frees memory through the exec library and runs a counted loop of calls through a table pointer before returning.
- `delay_loop_to_minus_one`: Decrements a 16-bit register in a loop until it reaches minus one, used as a timing delay, then returns.
- `decode_bits_and_clear_cache`: Calls a routine through a pointer several times, runs a bit-decoding loop that calls helper functions, and clears the processor cache through the exec library.
- `copy_stream_with_wrap_pointer`: Copies bytes between two pointers, wrapping the source pointer by a step when it reaches a limit.
- `decode_stream_with_window`: Decodes a compressed byte stream into a window buffer, using counters and pointer wrap checks, and returns a result value.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Run

- `start_background_cli_process`: Program entry that looks up libraries and tries to create a background CLI process.

### Stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longwords_then_call`: If the destination pointer is set, copies sixteen longwords from a source area into it, then calls another routine.

### Avail

- `calculate_available_memory`: Uses library calls to determine available memory and prints formatted text regarding chip, fast, total, and flush memory usage.
- `call_library_sequence_alloc`: Calls two library functions and four routines in sequence, one of which is an allocation through the exec library.
- `print_memory_summary_table`: Prints a table of memory types with available, in-use, maximum and largest columns for chip, fast and total memory, using library calls.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `sum_matching_list_records`: Walks a linked list of records and adds the difference of two fields for each record whose key matches a value.
- `set_copper_pointer_and_dma`: The decompiler timed out, so this is described from its register facts only: it writes the copper list pointer and DMA control, and calls two library functions.

### runback

- `execute_one_instruction_sequence`: A one-instruction routine that calls another routine.
- `run_background_task`: Parses a command line to extract an optional load delay and a program name with parameters, then executes the specified program as a background task.
- `clear_record_block_and_call`: Clears forty longwords inside a record, stores a stack address and a global value in it, and then calls another routine.
- `call_optional_hook_then_return`: Calls a handler stored in a field when that field is non-zero, then calls another routine.
- `open_libraries_and_init_state`: Opens the DOS and math floating-point libraries through library calls and records their bases, calling an error routine with a fixed code if either fails.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `run_ten_passes_then_hooks`: Calls a routine ten times, then calls an optional handler and a second optional routine when their stored pointers are set.
- `append_string_to_buffer`: Appends a zero-terminated string to the end of a buffer, copying at most 32766 characters.
- `append_string_with_limit`: Appends a zero-terminated string to a buffer, copying at most the count passed in, and returns the buffer end.
- `copy_zero_terminated_string`: Copies a zero-terminated string from a source to a destination, including the terminator.
- `measure_string_length`: Counts the characters of a zero-terminated string and returns the length.
- `copy_string_with_length_limit`: Copies at most a given number of characters from a source string to a destination, stopping at the terminator, and returns a pointer.
- `release_table_slot`: Releases an entry in a table of twenty slots, setting an error code when the index is out of range or the slot is empty.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_function_through_base`: Calls one library function through a library base held in a global variable, with no other operations.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_84`: Calls one library function at a fixed table offset through a library base held in a global variable, with no other operations.
- `call_library_vector_30_2`: A one-instruction library stub that loads a library base from a global variable and calls the library function at offset minus 30 of that base.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60_2`: A one-instruction library stub that loads a library base from a global variable and calls the library function at offset minus 60 of that base.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_90`: A four-instruction routine that calls a library function at offset minus 90 from a base address stored in a global variable.
- `call_library_function`: Performs an indirect call to a function located at an offset within a library base retrieved from a pointer.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Calls one library function at a fixed table offset through a base held in a global variable, then returns.
- `call_library_function_2`: Executes a function via a library vector offset after dereferencing a pointer.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_3`: Calls a library function using an offset from a base address stored in a global variable.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### endcli

- `scan_list_for_largest_entry`: Walks a linked list of table entries, tracking the largest value of a field in a stack buffer, and calls two routines.
- `copy_memory_block`: Copies a sequence of sixteen words from a source pointer to a destination pointer via a loop.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:z.crack

- `run_program_with_dos_library`: Program entry that opens dos.library and, when it succeeds, calls two library functions and closes the library.
- `store_constant_and_stack_pointer`: Stores a constant byte in a record at a fixed offset and saves a stack-frame pointer in a global variable.

### Port-Handler

- `load_hunk_executable_entries`: Program entry that fills hunk table entries from an executable header, computing offsets and clearing counters, then processes hunk records by their type codes.

### Disk-Validator

- `format_counter_text_digits`: Writes a counter into the characters of a text buffer, using a space when the value is below ten and digit characters otherwise, and calls a helper for the tens digit.

### Ram-Handler

- `copy_memory_and_jump_indirect`: Calls the exec CopyMem function through a table and then makes an indirect jump whose table could not be recovered.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:InstallCrypt

- `program_entry_free_and_loop`: Program entry that releases memory through the exec library, then runs a counted loop of calls through a table pointer before returning.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:BlackCrypt

- `reset_install_state_at_entry`: Program entry for Ambermoon_install that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_word_until_negative_one`: Decrements a 16-bit value stored in a register until it reaches negative one.
- `clear_cache_and_poll_bits`: Calls a routine through a pointer several times, runs a bit-polling loop, and clears the processor cache through the exec library.
- `advance_stream_window_pointer`: Advances a stream window pointer by a step when it reaches the end of a buffer, copying bytes through it until a counter runs out.
- `decode_stream_window_flagged`: Decodes a compressed stream into a window buffer, checking a global flag before the main loop and calling two routines for bit reads.
- `write_control_block_entries`: Writes a combined control byte, an input byte and a short field into a block through an address register, then calls a routine.
- `stub_bad_instruction_data`: Calls a helper and then reaches a bad-data marker, so this is a stub or misdecoded bytes rather than real logic.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:maus

- `wait_for_global_flag_clear`: Spins while bit six of a global variable is set, then returns.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:note

- `print_greeting_and_close_library`: Program entry that writes a fixed text banner to output through the DOS library, then closes the exec library.
- `open_dos_library_base`: Opens dos.library through the old open call and stores the returned library base in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:bcdfp

- `program_entry_clear_state_block`: Program entry that calls a routine, clears a block of 494 longwords in memory, stores a register value in a field, then continues with global setup.
- `open_graphics_intuition_and_setup`: Opens the graphics and intuition libraries, reporting a failure message for either that cannot be opened, then continues with setup calls through the library bases.
- `free_memory_and_close_libraries`: Makes library calls through a stored base, including one that releases memory, and passes a stored value to two further calls.
- `process_record_list_by_type`: Walks a linked list of twenty-byte records, copying each one and calling a routine for it, with extra work when the record's type byte is nineteen or a hash character.
- `initialize_four_channel_records`: Initializes four channel records of 168 bytes each, setting a per-channel start value and copying a default byte into each record.
- `initialize_four_entry_tables`: Fills four entries of index tables and channel records with values derived from their indices, adding small offsets to the fields.
- `build_and_write_character_file`: Clears a buffer of sixty entries, fills blocks of data for a set of character records, and writes them out through the DOS library by opening, writing and closing a file.
- `poll_library_until_ready`: Repeatedly calls two library functions until the second returns a non-zero value, then returns.
- `run_setup_until_six_checks`: Clears a counter word, calls a library function and two routines, and compares the counter with six.
- `initialize_library_state_two`: Stores the value two in a 16-bit state word, calls a library function through a jump-table entry, then calls a routine and calls it again if the state word is not six.
- `initialize_library_state_eight`: Stores the value eight in a 16-bit state word, calls a library function through a jump-table entry, then calls a routine and calls it again if the state word is not six.
- `set_state_three_release_lock`: Stores three and a parameter in global state, calls a library function and a wait-and-retry routine, then calls directory and lock routines and releases a lock when a check returns nonzero.
- `set_state_seven_and_return`: Stores seven and a 16-bit parameter in global state, calls a library function and a wait-and-retry routine, then returns the stored parameter value.
- `initialize_library_state_ten`: Stores ten and two parameters in global state, calls a library function and a wait routine, and calls the wait routine again if the state word is not six.
- `transfer_dungeon_save_record`: Checks a status byte and a stored code, then uses DOS open, read, write and close calls with dungeon file names to transfer a 52-byte record, returning 1 on a read failure and 2 on a size mismatch.
- `prompt_game_disk_until_ready`: Suppresses the system requester by setting a process field, locks the game disk, and loops showing disk-status messages until the check reports ready, then restores the field and unlocks the disk.
- `dispatch_sprite_draw_by_code`: Loops over queued draw entries and dispatches each by type: type-two entries are drawn according to a value range, and type-eight entries are routed through a switch on a code to rendering routines, ending when a terminal value is reached.
- `setup_sprite_group_loops`: Runs a series of setup calls, then for each of four groups calls a per-group setup routine and six per-item routines, and finishes with more setup calls.
- `calculate_nine_sprite_offsets`: Calls the sprite offset routine nine times, once for each index from zero to eight.
- `forward_sprite_offset_calculation`: Forwards its call directly to the sprite offset calculation routine and returns, adding no logic of its own.
- `store_sprite_slot_offset`: Computes a value from a passed coordinate plus an index times nine minus one, stores it in the entry for that index, then calls two routines and passes the index to a third.
- `configure_sprite_rendering_params`: Sets fixed word-sized parameters, calls the rendering setup routines for the indexed entry using an index-derived value, and calls the setup routine again with fixed values.
- `dispatch_sprite_render_case`: Checks an index against a stored value, derives a switch value from a computed difference, and dispatches to one of ten cases that call rendering routines, then makes three further calls.
- `find_matching_sprite_range`: Scans a ten-entry table of stored x and y bounds and returns the one-based index of the first entry whose range contains the given point, or zero if none match.
- `adjust_sprite_entry_counters`: When a per-entry value is not twenty and a second counter is nonzero, increments one counter for the given index and decrements the current entry's counter, then makes two calls.
- `check_sprite_value_tens`: Reads a stored value for the given entry, divides it by ten, and calls three routines when the quotient is zero or two routines otherwise.
- `draw_entry_text_buffer`: Clears the text buffer for an entry, positions its text, draws the string if the buffer is nonempty, and when a second flag is set draws a further text element in a fixed style.
- `clear_entry_text_byte`: Looks up an entry value and, if nonzero, tests two status bits, then clears either the entry's last character slot or its first byte before making two calls to refresh the display.
- `clear_entry_text_character`: Tests two status bits returned by a routine and, unless either is set, clears a character slot in the entry selected by a register argument, then makes two calls with the entry index.
- `clear_entry_char_and_refresh`: Clears a character slot in the entry selected by a register argument, calls routines with the value one, then makes two more calls and a final call with the entry index.
- `store_entry_text_character`: Reads an entry value and, when it is not twelve and a second routine returns nonzero, stores that result as a character at the index, clears the following slot, and makes four calls.
- `find_matching_table_index`: Scans a 26-entry table of 16-bit values for the given value and returns the matching position offset by sixty-five, or zero when no entry matches.
- `call_then_invoke_pointer`: Calls one routine, then invokes a second routine through a function-pointer slot in a global table, and returns nothing.
- `allocate_memory_block`: Calls a routine and then requests a memory block through the Exec library allocation call from the library table.
- `allocate_graphics_resources`: Allocates a state block and returns status one if that fails.
- `call_two_routines_in_order`: Calls two routines in sequence and returns, with no loop or condition in its body.
- `process_pair_list_with_library`: Calls a library function, then walks a list of two-word records, calling another library function for each record until a record's second word is minus one, and finally calls a third library function.
- `stub_no_operation`: Contains only a return and performs no work; it is a stub and not a meaningful routine.
- `cleanup_allocated_resources`: Releases entries from two tables of pointers through a release routine, closes a library if its handle is set, and then calls the release routine once more.
- `initialize_custom_chip_state`: Stores saved values into the state block, disables all interrupts, re-enables selected interrupt and DMA bits through the custom chip registers, sets a flag word, and calls nine routines.
- `configure_copper_and_dma`: Disables interrupts, writes DMA control values and copper list pointers taken from the state block, and re-enables selected interrupts, while also updating two global words.
- `wait_then_clear_interrupt`: Polls a status word until its interrupt bit is set, writes that bit to the interrupt request register to clear it, and returns the status value.
- `wait_and_clear_repeatedly`: Waits for the interrupt request bit and clears it, repeating this the number of times given by the parameter.
- `wait_for_byte_flag_loop`: Computes an iteration count from a parameter and a stored multiplier, then for each iteration clears a status byte and busy-waits until another routine sets it nonzero.
- `forward_single_routine_call`: Calls one routine and returns, with no other logic.
- `build_copper_list`: Writes a copper list into memory from stored bitplane and sprite pointers, adding sprite pointer moves, a block of color register moves from a table, and terminating words.
- `copy_seventeen_word_blocks`: Copies a 17-word block from a fixed offset in a source area into a buffer, then copies a second 17-word block from later in that area into the buffer at a further offset.
- `get_state_pointer_offset`: Returns a global pointer value increased by ninety-six.
- `store_state_word_param`: Stores the passed 16-bit value into a fixed field of the state structure.
- `fill_copper_entries_by_flag`: Checks a flag returned by a routine.
- `copy_global_words_to_list`: Stores the result of a routine into a state word, sets a second word to minus one, and copies six words from a global table into a list at eight-byte intervals.
- `run_clamped_motion_loop`: Inverts a flag byte and increments a counter, then waits until another flag is set by clearing words in two structures.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a register.
- `copy_masked_status_and_flag`: Copies the low bits of one status word, masked by a fixed value, into a register field of another structure and sets a flag byte to one.
- `write_eight_to_field`: Calls one routine, stores the value eight into a fixed field of a structure, and returns with the register values it received unchanged.
- `execute_indirect_jump_2`: Performs an indirect jump to an address retrieved from a memory location.
- `countdown_toggle_and_refill`: When a status flag is set, counts a timer down and on reaching zero toggles a state word and reloads the timer, then either fills six entries with a fixed value or copies six words from a global table into them, ending with an indirect call to an unresolved target.
- `reload_timer_and_copy_words`: Sets a countdown field to twenty-six, copies six words from a global table into a list at eight-byte intervals, then makes an indirect call through an unresolved jump-table target.
- `emit_pending_display_entries`: When a pending flag is set, clears it and copies twelve words from a source block into a list.
- `initialize_input_state`: Sets a pointer field to a block 1024 bytes past a base address, zeroes counters and markers, stores a byte from a global table along with its tenth-part quotient, sets a limit of ten, and stores the results of two routines in state bytes.
- `advance_sequence_value_every_third`: When a flag is set and a frame counter is a multiple of three, reads the next word from a sequence, restarting from a default value when it reaches zero, stores it in a structure field, then makes an indirect call to an unresolved target.
- `store_state_byte_and_dispatch`: Stores a byte returned by a routine into two state bytes, calls a routine, and makes an indirect call through an unresolved jump-table target.
- `update_input_region_state`: Reads a state value and, when it changes, scans a table of five-word regions for one whose bounds contain the current position and whose flag bits match, then runs the matching action.
- `scan_input_regions_and_update`: Scans a table of five-word regions for one whose flag bits match and whose bounds contain the current position, then runs its action.
- `check_second_region_then_update`: Runs the second-flag region check against the current position, then updates state bytes, decrements a counter when nonzero, and makes an indirect call to an unresolved target.
- `pre_call_second_region_update`: Makes two routine calls, runs the second-flag region check against the current position, then updates state bytes, decrements a counter when nonzero, and makes an indirect call to an unresolved target.
- `search_second_region_then_update`: Scans the five-word region table for a second-flag entry containing the current position and runs its action on a match.
- `store_input_state_and_dispatch`: Calls a routine, stores a byte returned by another routine into state bytes, calls a further routine and calls one more unless the result is minus one, decrements a counter when nonzero, and makes an indirect call to an unresolved target.
- `update_state_and_dispatch`: Stores two bytes from function return values into a structure, decrements a byte at an offset if it is positive, and performs an indirect jump.
- `decode_keyboard_serial_byte`: When a ready bit is set, clears the handshake flag, sets it again, and decodes the inverted, rotated keyboard serial byte.
- `set_key_state_bit`: Sets one bit, chosen by the low three bits of its argument, in a key-state byte and returns minus one.
- `read_state_byte`: Returns the byte stored at a fixed offset from the global state pointer.
- `clear_state_and_buffer`: Sets several state bytes and words to zero or fixed values, stores fifteen in a counter location, and clears a buffer of 336 words.
- `cycle_frame_buffer_pointers`: Advances a buffer offset in steps of two that wrap at forty-two, copies eight pairs of frame bytes from two source pointers into the buffer at interleaved offsets, and updates a pointer to the buffer.
- `cycle_frame_buffer_on_timer`: When a timer byte is nonzero it decrements it and, on underflow, resets it and performs the same frame-buffer copy and offset advance as its sibling routine.
- `step_row_index_and_copy`: Unless the row index is eight, increments it and copies a seven-word row from a table into a list, then makes an indirect call to an unresolved target.
- `call_then_return_first_result`: Calls a routine to obtain a 16-bit value, calls a second routine, and returns the first value.
- `read_next_buffered_character`: While data is pending, reads the next byte from an eighty-entry circular text buffer, moves the read index back with wraparound, and returns the character code minus thirty-two multiplied by eight.
- `clear_pending_flag_return_zero`: Clears a pending flag byte and returns zero.
- `write_text_string_to_buffer`: Walks a null-terminated string, passing each character to a routine and setting a pending flag, until the routine returns a nonzero value, which it returns; it returns zero at the end of the string.
- `write_circular_buffer_byte`: Writes a byte at the current index of an eighty-entry circular buffer, then decrements the index with wraparound.
- `set_circular_buffer_index`: Stores the passed value as the buffer index and returns zero.
- `decrement_nine_timers`: Decrements each of nine 16-bit timers that is nonzero, then makes an indirect call to an unresolved target.
- `initialize_motion_state`: Sets the position, bounds and motion fields of a state structure to fixed constants and zero, copies a value from a parameter block into a state word, clears flag bytes, and copies one stored longword into four consecutive slots, then calls a routine.
- `update_clamped_motion_position`: When a motion flag is zero, clears several counters; otherwise computes new coordinates from scaled offsets and a routine result, clamps them between stored bounds, and stores them.
- `clamp_position_then_dispatch`: Clamps two coordinate parameters against stored bounds, stores the results as the position, calls two routines, and makes an indirect call to an unresolved target.
- `store_position_clamped_y`: Stores the position from its inputs, clamping only the second coordinate against a stored bound, calls two routines, and makes an indirect call to an unresolved target.
- `advance_clamped_position`: Adds stored offsets to the current position, clamps the sums between stored bounds, stores the new position, calls two routines, and makes an indirect call to an unresolved target.
- `clear_motion_counters`: Zeroes several counter words and longwords in two structures, then makes an indirect call to an unresolved target.
- `compute_counter_axis_deltas`: Stores two new 8-bit counter values as the previous sample and returns a signed movement delta for each axis, unwrapping the 8-bit overflow of each difference.
- `compute_single_axis_delta`: Returns a value derived from one counter and a flag, and a signed delta between two 8-bit counter values with overflow unwrapped.
- `stub_immediate_return`: Contains only a return and performs no work; it is a stub.
- `record_sample_and_store_result`: Stores the current pair of position words into a four-entry history indexed by a rotating counter, advances that counter, and writes either a value from a routine or zero into two output words, depending on a flag and a count.
- `store_history_outputs_with_offset`: Decrements a counter that wraps to three, obtains a value from a routine, zeroes it when a count is zero, and writes it to several output fields, including one with eight added.
- `compute_packed_position_flags`: Adds stored position words to two inputs, sets bit flags from the high bits of the sums and the low bit of the first coordinate, and packs the results into a combined return value with a fixed flag bit set.
- `read_fire_button_state`: Returns minus one when bit six of a stored status byte is clear, and zero otherwise, which reports a pressed fire button.
- `sample_port_counters_safely`: Disables the master interrupt bit, saves the two 8-bit movement counters read from the mouse port data register into state bytes, stores two input bytes, and re-enables the master interrupt bit only if it was previously set.
- `store_state_words_atomically`: Disables the master interrupt bit, stores two word inputs into state fields, and re-enables the master interrupt bit if it had been set.
- `store_two_state_words`: Stores two word inputs into two state fields without changing any interrupt control.
- `pop_event_queue_entry`: Pops the next entry from a 128-entry ring of queued pairs.
- `clear_event_queue`: Resets three queue counters to zero, zeroes all 128 pairs of longwords in the queue table, and re-enables the master interrupt bit if it was set before.
- `set_queue_active_flag`: Sets the queue-active word to one.
- `clear_queue_active_flag`: Sets the queue-active word to zero.
- `store_masked_flags_check_bit`: Stores the input word with bits eleven to thirteen cleared into a state word, sets a state byte to minus one when its low bit is clear, and returns that low bit.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `enqueue_event_entry`: When the masked flag bits match and the queue has room, scans for a free slot in a 128-entry table, records its address in a ring, advances the ring index with wraparound, and stores two input words followed by two state words.
- `append_event_record`: Records a buffer address into a 128-entry ring, advances the ring index with wraparound, then writes two input words and two state words into that buffer.
- `compute_scaled_countdown_byte`: Multiplies the register input by a stored word, divides the product by sixty, makes a zero result one, and stores the low byte in a state byte.
- `copy_linked_records_to_buffer`: Walks a linked list of five-word records, copying each into a contiguous buffer with two fields derived by adding offsets, then writes a minus-one terminator and re-enables the master interrupt bit if it was set before.
- `render_text_with_blitter`: Clears a 16-entry text buffer, copies glyph rows and mask rows for each character of the string from two tables into it, then programs the blitter with pointers, sizes and masks and waits for each blit to finish, repeating six times.
- `stub_empty_return`: Contains only a return and performs no work; it is a stub that many routines call.
- `initialize_text_and_state_fields`: Points two state fields at offsets inside the state structure, zeroes several flags and counters, sets a timer to thirty, and writes a fixed DMA control value to the custom chip register.
- `swap_display_buffers_and_wait`: Copies six words from a source structure into state fields, toggles a buffer-select byte, swaps two pointers between buffer slots to match, and waits until a pending flag is cleared.
- `call_two_setup_routines`: Calls two routines in sequence and returns.
- `compute_sprite_buffer_offset`: Calls one routine, stores an offset computed as the register input times 576 plus 96 into a global variable, then calls another routine.
- `render_entries_until_terminator`: Repeatedly calls a routine for each five-word entry in a table, advancing five words at a time, until an entry's first word is negative.
- `render_sprite_stripes`: Reads a sprite descriptor, sets blitter control values from its coordinates and a row offset, and starts five blits, waiting for the blitter to be idle before each one.
- `store_render_params_and_render`: Stores three input values into global variables, then calls two routines.
- `stub_bad_instruction_data_2`: Calls two routines and then reaches bad instruction data that the decompiler could not decode, so no further behaviour can be recovered.
- `render_five_blit_stripes_indirect`: Stores a stacked pointer, derives a stripe offset from a descriptor's coordinates, programs the blitter, starts five blits while waiting for the blitter to be idle before each, then makes an indirect call to an unresolved target.
- `render_clipped_or_direct_blits`: Stores two input coordinates in a sprite record, then either programs five direct blits from the record's source addresses or, when a flag bit is set, computes clipped blit parameters into globals and returns early if the object is off-screen.
- `calculate_clipped_sprite_bounds`: Computes clipping bounds for a sprite from its position and size, stores the row offset and word masks in globals, and returns early when it falls outside the limits.
- `wait_and_write_index_bytes`: Loops sixteen times, each time waiting on two status flags, calling a routine, and writing a computed byte through a pointer.
- `decrement_nibbles_in_table`: Walks a table of four-byte entries, decrementing each non-zero nibble of the entry's second word by one and writing the word back.
- `increment_differing_nibbles`: Walks a table of four-byte entries, comparing the high, middle and low nibbles of each entry's second word with the matching nibbles of a source word table.
- `clear_five_longword_buffers`: Clears 2000 longwords at each of five buffer addresses taken from a pointer array in the state structure.
- `process_four_channel_records`: Walks four channel records from last to first, and for each active record a type code selects one action: set a channel bit, load a source record's address and length and enable its channel, or write a default length.
- `process_channel_records_loop`: Repeats the four-record channel processing with the same type-code actions, decrementing each record's type code until the index wraps past zero, then makes an indirect call to an unresolved target.
- `dispatch_channel_record_actions`: Initialises a record with a default length and a state value, then walks the four type codes from last to first, performing a set-bit, load-source or skip action for each and decrementing it, and finishes with an indirect call to an unresolved target.
- `write_jump_table_entry`: Writes a type code of two into a table entry at the given index, stores the upper word of the first argument alongside it, and stores a pointer offset from the routine's own location by eight times the lower word of the first argument.
- `write_dispatch_entry`: Stores a byte, a word, and a pointer into a table based on provided indices.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `write_dispatch_table_entry`: Writes the same type code, word and offset pointer as its neighbouring entry writer, storing them at the index given by the second argument.
- `forward_buffer_and_args_call`: Passes a fixed buffer address, the argument and the caller's stacked arguments to a routine and returns that routine's result.
- `copy_bytes_to_buffer`: Copies a counted run of bytes from a source address into a destination buffer one byte at a time and returns the destination address.
- `stub_return_without_work`: Contains only a return and performs no work.
- `allocate_and_tag_record`: Allocates a record and releases it if allocation fails.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_command_string`: Copies a command string into a buffer while splitting it into tokens at spaces, tabs, form feeds and line breaks, keeping double-quoted tokens whole.
- `append_string_with_limit_2`: Finds the terminator of the destination string, then appends characters from the source string until its terminator or a counted limit is reached, terminates the result, and returns the destination address.
- `append_string_capped_copy`: Finds the terminator of the destination string, then appends characters from the source string up to a counted limit, terminates the result, and returns the destination address.
- `read_icon_tooltype_entries`: Opens icon.library and looks up a tool-type argument named by the referenced text.
- `append_whole_string`: Finds the terminator of the destination string and copies the whole source string, including its terminator, to that point, returning the destination address.
- `append_string_at_terminator`: Locates the terminator of the destination string and copies the source string, including its terminator, to that point, returning the destination address.
- `copy_string_with_zero_padding`: Copies characters from a source string into a destination buffer up to a counted limit, stopping after the source terminator.
- `create_message_port`: Allocates a signal bit and a port record, returning zero if either fails.
- `release_message_port_record`: Releases a message port record: removes it through a library call when linked, marks its node type as invalid, sets a signal field to minus one, and calls two library routines to free the remaining resources.
- `format_printf_style_output`: Parses a format string, copying literal characters into an output buffer and expanding percent directives with flags, width, precision and argument values.
- `measure_string_length_2`: Advances through a zero-terminated byte string and returns the number of bytes before the terminator.
- `initialise_empty_list`: Initialises an empty linked list header: its head points at its own tail slot, the tail is cleared, and the tail-predecessor points back at the header.
- `process_records_in_range`: Calls one routine for each 22-byte record in a fixed address range, moving from the first record to the last.
- `write_buffered_stream_byte`: Writes one byte into a stream buffer, flushing it when full or when a newline arrives in line-buffered mode, and passing minus one flushes instead.
- `close_buffered_stream`: Releases a buffered stream.
- `close_stream_and_clear_fields`: Closes a buffered stream's handle, frees its generated five-digit name buffer when present, clears its fields, and returns zero when all steps succeed or minus one otherwise.
- `stub_return_zero`: Returns zero without performing any other work.
- `tiny_addq_addq`: Small routine of 2 instructions that adds 2 to a5, then adds 8 to d0.
- `flush_buffered_streams`: When given a stream, flushes that stream.
- `flush_remaining_streams`: Walks the remaining stream table entries, flushing each open entry, and returns minus one on the first failed flush or zero when the table is exhausted.
- `call_single_routine_return`: Calls one routine and returns.
- `stub_empty_routine`: Performs no work and returns; it is an empty stub.
- `call_and_sign_adjust_result`: Calls a routine, then returns the register pair with its second value negated when the first input was negative.
- `call_and_return_pair`: Calls one routine and returns a 64-bit pair made from the original low register value and the value the call leaves in the second register.
- `call_routine_then_return`: Calls one routine and returns, adding no other logic.
- `divide_32bit_with_remainder`: Divides a 32-bit dividend by a 32-bit divisor, using a shift-and-subtract loop when the divisor exceeds sixteen bits and 16-bit division otherwise, and returns the quotient and remainder as a packed pair.
- `allocate_node_buffer`: Checks a node's index against a fixed record, then gives the node a 1024-byte buffer from a library call, or falls back to a one-byte buffer inside the node when allocation fails.
- `unlink_and_free_node`: Searches a singly linked list for the given node, unlinks it when found, and frees it through a library call.
- `find_and_unlink_node`: Walks a singly linked list to locate the node passed in a register, unlinks it by joining its predecessor to its successor, and frees it through a library call, returning that result.
- `unlink_node_from_predecessor`: Unlinks a node using a predecessor address held in a register, or the list head when there is none, then frees the node through a library call.
- `set_list_head_and_free`: Sets the list head to the first word of a node and releases that node through a library call.
- `free_all_list_nodes`: Releases every node in the linked list through a library call, then clears the list head and returns zero.
- `free_list_nodes_until_end`: Walks the linked list, calling a release routine for each node until the chain ends, then clears the list head and returns zero.
- `allocate_linked_node`: Allocates a node of the requested size through a library call, stores the size in it, and links it at the head of the list.
- `copy_string_to_buffer`: Copies a null-terminated source string into the destination buffer, including its terminator, and returns the destination address.
- `check_stream_is_interactive`: Validates a file slot index; invalid or empty slots set an error code of three and return minus one.
- `seek_stream_slot_position`: Validates a file slot index, then asks the file system for the current position.
- `delete_file_or_record_error`: Calls a DOS routine that deletes a file.
- `stub_constant_zero_return`: Returns zero without any other work.
- `write_to_stream_slot`: Validates a file slot index and that the slot is open for writing, returning minus one with error codes three or six when it is not.
- `check_status_flag_and_handle`: Calls a library routine for a status word; when bit twelve is set it returns that word if a stored pointer is nonzero, otherwise it calls a routine.
- `write_four_bytes_to_output`: Writes four bytes from a global buffer to the standard output, makes a second DOS call, and calls a routine with the argument one.
- `run_callback_list`: Runs each callback in a linked list in turn, advancing through the list, then calls a final cleanup callback if one is set, and finally calls a routine with its argument.
- `close_slots_and_release_resources`: Closes every open file slot, then tears down the state by invoking stored release callbacks, restoring a saved pointer, closing libraries, releasing a lock and freeing a saved buffer where present.
- `nop_padding`: This routine is empty and performs no operations.
- `close_stream_slot`: Validates a file slot index, closes its file through DOS unless the slot's flag bit is set, clears the slot, and returns zero.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:bcdfq

- `run_program_entry_setup`: Stores the startup argument, opens the required system libraries, copies startup fields into globals, runs setup steps including timed waits and a DMA enable, and then loops on interrupt and status flags.
- `is_fire_button_pressed`: Returns true when bit six of a stored status byte is clear, which reports a pressed fire button.
- `is_control_bit_clear`: Returns true when bit two of a hardware status byte is clear.
- `wait_for_condition`: Polls two adjacent short values in a structure, returning immediately if the first is zero, otherwise waiting until the second becomes non-zero.
- `fade_objects_each_frame`: Runs 105 frame steps, each waiting for the top of frame and then, for four graphics objects, reducing a timer by two (clamped at zero) and storing a scaled value derived from the object's stored byte.
- `open_data_file_check`: Opens a named data file through DOS, stores the returned handle, and returns true when the open failed.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `copy_source_offset_blocks`: Copies 3000 blocks of four longwords from a source area, offset by 12000 longwords, into the destination area.
- `clear_and_copy_bitmap_block`: Calls a graphics library clear routine, then copies 8000 longwords from a source area offset by 12000 longwords into the destination area.
- `read_entries_until_flag`: Walks a table of nonzero words, calling a DOS read routine through the library table for each entry, and stops at the first zero entry or when a flag in the structure becomes nonzero.
- `loop_call_until_flag`: Iterates through a table of short values, calling a function pointer from a global structure for each non-zero entry.
- `execute_table_callbacks`: Iterates through a table of short values, calling a function pointer derived from a global structure for each non-zero entry.
- `call_dos_read_routine`: Calls one DOS read routine through the library table and returns.
- `fade_palette_down_steps`: Runs sixteen steps, each waiting for the top of frame, decrementing the nibble values of a colour table and loading the palette with a graphics call.
- `decrement_nibbles_in_word_list`: Walks a list of words, decrementing each non-zero nibble of each word by one and writing the word back.
- `fade_palette_up_steps`: Runs sixteen steps, each waiting for the top of frame, incrementing nibbles that differ from a target palette and loading the palette with a graphics call.
- `increment_nibbles_toward_target`: Walks two word lists in step, incrementing each nibble of the first word that differs from the matching nibble in the second, and writing the word back.
- `blit_multi_source_blocks`: Takes exclusive use of the blitter, then for sixteen steps waits for frame timing and programs the blitter with source and destination addresses, modulo values and sizes, waiting for each block to finish.
- `store_channel_bit_mask`: Stores a single-bit mask for a channel index when the low six bits of the input are below four.
- `run_sixteen_steps_then_clear`: Calls one routine sixteen times, then sets a global variable to zero.
- `setup_channel_sample_state`: Looks up a pointer for a channel.
- `set_channel_sample_fields`: Stores a channel's sample address and length fields from its inputs.
- `set_channel_sample_pointer`: Writes a channel's length and auxiliary fields and computes its current sample pointer from the base address and offset, storing the result in the channel state.
- `store_channel_state_fields`: Stores a length value and an offset-derived pointer into the channel state, then copies a secondary value into two state fields.
- `halt_in_infinite_loop`: Contains no work and loops forever, so it never returns.
- `copy_channel_pointer_if_flagged`: When bit zero of the input is set and a channel's sample field is nonzero, copies that field and a 16-bit value into the destination record, then returns the input shifted right by one.
- `process_channel_flags_when_active`: When a global flag is nonzero, stores a value from a routine, calls four routines, then performs the same conditional copy as its neighbour and returns the shifted flag bits; otherwise it returns zero.
- `advance_sequence_playback_tick`: Advances a playback tick: when a counter reaches its limit it resets the counter and decodes the next block of packed entries, storing each channel's note, length and volume fields.
- `decode_sequence_channel_commands`: Decodes packed entries for four channels, each supplying a channel's note, length and volume, while command values near the top of the byte range select actions such as pausing, jumping or stopping playback.
- `apply_channel_command_step`: Applies the pending command to the current channel: a special marker value sets its state, a repeat command adjusts a stored count, and other commands call an effect routine.
- `apply_channel_command_variant`: A variant of the command step that returns the sequence position when it reaches a stop command, and otherwise updates the channel state for the next tick.
- `process_channel_command_byte`: Handles one command byte for a playback channel: a special value stores a 16-bit word from a lookup table into the channel record, another special value calls a handler, and other values pass into the shared stepping loop that advances the channel's tick counter and reloads the next sequence entry when the counter passes its limit.
- `advance_channel_sequence_tick`: Forwards a special command byte to the channel handler first, then runs the shared stepping loop: it advances the tick counter, and when the counter passes its limit it moves the sequence index forward with wraparound and fetches the next sequence pointer from the table.
- `increment_shared_tick_counter`: Increments a shared one-byte counter and then enters the common channel stepping loop, which advances each channel's tick counter and reloads its next sequence entry when the counter passes its limit.
- `dispatch_channel_parameter_command`: Applies one parameter command to a channel according to its code: it stores the value as a channel field, scales a level capped at 64, sets a 5-bit step value where zero means 32, splits a byte into two nibble fields, sets a global index, or loads a 16-bit word from a table, and then runs the shared stepping loop.
- `apply_channel_effect_parameter`: Applies an effect parameter to a channel by command code: it scales a level capped at 64, sets a 5-bit step value where zero means 32, splits a byte into two nibble fields, sets a global index, or loads a 16-bit word from a table, then continues into the shared stepping loop.
- `apply_channel_modifier_command`: Applies a modifier command to a channel: it sets a 5-bit step value where zero means 32, splits a byte into two nibble fields, sets a global index when it is within range, or loads a 16-bit word from a table, then continues into the shared stepping loop.
- `apply_channel_nibble_command`: Splits a command byte into its two nibbles and stores them in channel fields, sets a global index when the value is within range, or loads a 16-bit word from a table, then continues into the shared stepping loop.
- `apply_channel_index_command`: Sets a global index when the command value is within range, or loads a 16-bit word from a table for the channel, then continues into the shared stepping loop.
- `store_channel_table_offset`: Stores a 16-bit word fetched from a table, indexed by a channel value adjusted with a per-channel offset, into the channel record and saves the command byte in a channel field, then continues into the shared stepping loop.
- `store_channel_word_and_step`: Stores a 16-bit table word in the channel record, copies a non-zero byte into the channel output field, and then runs the shared stepping loop that advances tick counters and reloads sequence entries.
- `service_audio_channels_dma`: Walks the channel records, applying pending event bytes and updating a bit-flag table as it goes, and writes the DMA control register; the register use is recorded in the facts, and the writes with bit 15 set enable the selected DMA channels.
- `dispatch_channel_handler_table`: Walks a list of channel pointers, reads each channel's command byte, and calls the matching handler through a jump table in global data; it also counts down a per-channel counter and copies a level into the channel's output field when a flag is set.
- `lookup_or_divide_table_entry`: Returns a 16-bit value for the input index: for indices below eleven it reads an entry from a global table, otherwise it divides a global constant by the index.
- `store_indexed_value`: Selects a word from a table using an index if it is below a certain threshold, otherwise performs a division, and stores the result as two bytes in a global location.
- `lookup_or_divide_table_entry_2`: Checks if the input index is within bounds to read a 16-bit value from a global table; otherwise, it divides a global constant by the input.
- `tiny_ori_2`: Small routine of 1 instruction that merges 240 into d7.
- `store_global_address_and_read`: Stores the address of a global variable into a slot in the caller's structure and returns the byte at the input pointer together with that pointer advanced by six.
- `add_to_pointer_if_nonzero`: If the value pointed to by the first argument is non-zero, adds the second argument to it and writes the sum back through the pointer; otherwise it leaves memory unchanged.
- `adjust_pointer_offsets`: Calls several routines, then walks linked records: for each record whose flag byte is negative it adds the record's own base address to a run of slots after its header.
- `scale_source_table_into_globals`: Multiplies sixteen source bytes by a scaling byte from the structure, shifts each product right by four, and stores the results as 16-bit words in global storage.
- `check_mmd0_module_and_setup`: Calls a check routine first and returns a non-zero marker if it fails.
- `check_ciab_and_call_handler`: Calls a handler when a global flag is non-zero, then calls a second routine.
- `clear_flag_and_call_handler`: If a global handler pointer is set, clears bit zero of the byte it points to, clears the handler pointer, and calls the handler, returning its result; otherwise it returns zero.
- `open_ciab_resource_registers`: Opens the ciab resource through the exec library and, when that succeeds, stores the register addresses of the two CIA chips and their port pairs into global variables and clears the matching bits in a control byte.
- `close_ciab_resource`: If the stored resource base is non-zero, clears it and calls an exec library routine through that base, returning the result; otherwise it returns zero.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:bcdft

- `run_program_startup_sequence`: Serves as the program entry: it runs several routines and a short loop that calls a function pointer taken from a global variable, clears a word, and reads a structure field.
- `decrement_until_minus_one`: Decrements a 16-bit local counter in a loop until it reaches minus one, then returns.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[f DTC].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_status_and_reset_state`: Calls a routine through a pointer and saves a structure field, then polls a status word until its high byte is set and a bit clears.
- `initialize_audio_channel_tables`: Writes a fixed sequence of values to global registers, including the audio channel volume and DMA control registers according to the facts, then fills a table of pointers at a fixed stride, storing the low and high halves of each pointer into a pair of 16-bit fields.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `copy_strided_bytes_forward_back`: Copies bytes between two regions in two passes: the first walks forward in steps of four, and the second walks back over the same span in steps of four from the other end.
- `copy_strided_bytes_back_forward`: Copies bytes between two regions in two passes: the first walks backward in steps of four, and the second walks forward in steps of four over the same span.
- `call_then_increment_counter`: Calls a routine and then increments a 16-bit counter kept in global storage.
- `build_indexed_record_list`: Fills a buffer with fixed-size records, each holding an index, a value read from a polling routine, the next index, and a flag nibble.
- `build_record_list_with_step`: Fills a buffer with records, each holding an index, a value read from a polling routine, the next index and a flag nibble, advancing the index by a step taken from a data register until it reaches a limit, then ends the list with a zero byte.
- `step_pseudo_random_byte`: Updates a 16-bit global state by multiplying it by 125 and keeping its low 13 bits, then returns the byte from bits four to eleven.
- `update_record_list_values`: Walks a zero-terminated list of two-word records, combining each record's stored bits with the running value and adding one.
- `update_record_list_from_pointer`: Applies the same record-list update as its sibling routine, but starts from a base value passed in a data register and walks the list the caller points to.
- `tiny_ori_3`: Small routine of 1 instruction that merges 209 into d0.
- `tiny_ori_4`: Small routine of 1 instruction that merges 56 into d0.
- `start_blitter_and_copy_table`: Writes the blitter control, size and pointer registers, waits until the blitter has finished, and then copies one of several thirteen-entry tables into a fixed buffer, chosen by a mode value.
- `step_thirteen_entry_timers`: Loops over thirteen table entries, stepping each value by a fixed amount or counting down a companion counter, adding an offset when a value passes a threshold, and calling a routine for each entry.
- `copy_mode_table_to_buffer`: Clears a global value and calls a routine, then, if a data register is zero, copies one of several thirteen-entry tables into a fixed buffer, chosen by a mode value.
- `blit_character_run`: Walks a byte string, turning each character into an offset of eighteen bytes past a space code, and for each non-zero offset programs the blitter registers to copy one row from a table.
- `advance_audio_counter`: Increments a global counter and, upon reaching its limit, clears global variables, executes two delay loops, and copies four stored pointers into a destination.
- `swap_global_with_slot`: Exchanges a global variable's value with a second storage location, so that each receives the other's previous value.
- `initialize_sample_pointer_tables`: Walks a byte table to find a position, then fills a table of pointers into a buffer region, zeroing each entry and linking it to a stride-based slot.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `reload_audio_channel_pointers`: Advances a global counter and, on the wrap-around path, clears globals, waits in two short delay loops, and copies four stored pointers into the audio channel location registers.
- `process_channel_four_byte_entry`: Reads one four-byte entry for an audio channel and stores its sample pointer and effect value in the channel record.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `apply_channel_tick_effect`: Advances one audio channel's effect for the current tick: depending on the effect nibble it slides a stored period, applies an offset from a table indexed by a counter, steps a volume or position counter, or keeps the current value, then writes the result to the channel output field.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_table_and_dispatch`: Calls a routine, clears a block of longwords at an offset from a base register, stores a value in a base-relative slot, and then calls a handler through a library pointer, choosing between two paths by a flag bit and a returned value.
- `handle_workbench_disk_access`: Saves a window pointer in the task structure, locks the game disk volume, and runs an event loop that dispatches on event types, updating state and invoking library routines.
- `initialize_graphics_and_config`: Clears several global pointers, opens the graphics and intuition libraries through their library bases, and loads the configuration file named in the facts.
- `execute_shutdown_callbacks`: Checks several global handles and calls the matching function pointers from the library bases, including the intuition screen and window close routines according to the facts, then invokes a final shutdown routine.
- `delay_and_unload_segment`: Sets a state field to six, calls a routine, and then calls two library functions with the stored arguments; the facts record a DOS delay and a segment unload.
- `poll_input_event_loop`: Calls an input dispatch routine in a loop until a finish condition is met, handling event types one to four by updating global state, invoking another handler, or setting the completion flag.
- `convert_string_case`: Copies a length-prefixed string up to a fixed limit into a buffer, converts lowercase letters to uppercase, and appends a colon and a null terminator.
- `find_matching_list_entry`: Walks a linked list of entries from a global table, and for each entry with a non-zero field it converts the entry's name into a local buffer and compares it with the caller's argument.
- `wait_for_disk_insertion`: Returns at once if the disk-present flag is set.
- `wait_for_library_call`: Repeatedly calls a function pointer from a library base and then a second one, looping until the second returns a non-zero value, and returns that value.
- `load_segment_and_create_process`: Loads a code segment by name and, if that succeeds, creates a process from it, storing the process handle and arguments in global slots.
- `update_entry_state_and_wait`: Looks up an entry from a global table and compares its tag with a value from the caller's frame, reporting a mismatch if they differ.
- `handle_segment_load_unload`: Calls a routine, then loads a segment; if the load succeeds it calls the loaded entry with computed arguments and unloads the segment afterwards.
- `emit_zero_terminated_string`: Calls two routines, then passes each byte of a zero-terminated string from the argument, one at a time, to another routine, stopping at the terminator.
- `stub_return_only`: Contains only a return instruction and performs no work.
- `emit_zero_terminated_string_2`: Calls a routine, then passes each byte of a zero-terminated string from the argument, one at a time, to another routine, stopping at the terminator.
- `call_two_routines_in_order_2`: Calls two routines in sequence and returns, with no other work.
- `clear_bitmap_region`: Calls a routine and then calls the graphics library's block-clear function through its library base, clearing a region of a bitmap.
- `show_menu_text_entries`: Passes a set of positional arguments and five menu strings to a display routine; the facts list the referenced texts CONFIGURATION MENU, CREATE NEW PARTY, EXIT TO AMIGADOS, LOAD GAME and MAIN MENU. The routine returns five.
- `map_click_to_menu_item`: Polls the input message queue until a qualifying pointer message yields a result.
- `dispatch_input_event_handler`: Calls the graphics library's block-clear function through a library base, then dispatches on an event code to one of several sub-routines, runs cleanup calls, and returns a value taken from the processed event data.
- `store_register_values`: Stores the data registers D0, D1 and D2 into fixed global memory locations, and does no other work.
- `copy_tile_to_bitplanes`: Copies eight bytes of a tile into five bitplane-style destinations spaced 8000 bytes apart, writing each plane at a 40-byte row offset.
- `scan_global_variable_array`: Scans a global array in two passes, stepping two bytes at a time: while each byte is non-negative it calls routines to copy tile data, and it stops when a null byte follows.
- `decrement_nibble_loop`: Runs a fifteen-step loop that calls indirect function pointers and a routine that decrements nibble values, using the library calls listed in the facts.
- `decrement_nibble_values`: Walks a list of words, and for each one decrements its three nibble fields that are non-zero, leaving zero fields unchanged.
- `increment_nibble_loop`: Runs a fifteen-step loop that calls indirect function pointers and a routine that increments nibble values in the word lists, using the library calls listed in the facts.
- `increment_nibble_values`: Walks two lists of words, and for each entry increments the nibble fields of the first list where they differ from the corresponding fields of the second list.
- `copy_byte_array`: Copies a given number of bytes from a source pointer to a destination pointer and returns the destination.
- `compare_string_bytes`: Compares two byte strings character by character and returns one, minus one, or zero according to the first difference or equality.
- `execute_empty_routine`: An empty routine consisting of two instructions.
- `start_process_with_tag`: Gets a process record through a library call and exits if there is none.
- `split_arguments_into_pointers`: Splits a command string into arguments: it skips blanks, takes quoted arguments with doubled-quote escapes or plain words, ends each one in place with a zero byte, and records each start address in a pointer array terminated by a null pointer.
- `parse_command_string`: Parses a string into individual tokens by skipping whitespace and handling quoted substrings.
- `copy_string_with_limit`: Finds the end of the first string, then copies characters from the second string after it, decrementing a count for each character, and stops when the count reaches zero or the source ends.
- `copy_string_with_limit_2`: The routine finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `load_window_icon_table`: Opens the icon library and, when it obtains an object, reads a count and a table from it, recording the count and a table pointer in the given structure before releasing the object.
- `append_string_to_buffer_2`: Advances a pointer to the end of the first string and copies the second string into that location, including its terminator.
- `copy_string_or_pad`: Copies characters from a source string into a destination buffer up to a count.
- `allocate_record_and_init`: Calls a library routine and returns zero if it gives minus one.
- `initialize_library_context`: Checks a field in a structure and, if it is non-zero, calls a library routine.
- `calculate_string_length`: Advances a pointer to the zero terminator of a string and returns the number of bytes before it.
- `setup_pointer_chain`: Stores the address of the structure into its own first slot, advances that value by four, clears the next word, and stores the original pointer in the following word.
- `copy_string_simple`: Copies characters from a source string into a destination buffer up to and including the null terminator, and returns the destination pointer.
- `execute_fifteen_instruction_sequence`: A fifteen-instruction routine called by several different routines.
