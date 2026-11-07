# Game 2423 (caesar deluxe (1993)(impressions)[cr atx](disk 1 of 2)[hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02423). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908528 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 301 of 301 named (0 author symbols), 301 of 301 described (12 by rule, 119 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1333 of 1333 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5728 of 5764 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3339 |
| code | 22208 |
| data | 70561 |
| padding | 4996 |
| unused | 807424 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:installto | 2 | 117 | 2 |
| ram-handler | 2 | 10 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:assign | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:echo | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:Anthrox | 6 | 251 | 6 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:cd | 2 | 117 | 2 |
| installto | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| install.exe | 3 | 1394 | 112 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:INSTALL.EXE | 3 | 1493 | 118 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1670 | 47 |

## Cases handled

- runtime segment an address: 1085 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:installto

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the destination pointer is set, copies sixteen 32-bit values from a fixed table into it, then calls a stored routine pointer.

### ram-handler

- `copy_memory_then_jump_entry`: Program entry: calls the exec library CopyMem function through the library base, then makes an indirect jump through a jump table the decompiler could not recover, so control passes to a routine that cannot be identified from the evidence.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:assign

- `scan_max_then_prepare_buffer`: Walks a linked list of entries to find the largest value in a size field, allocates a buffer for it through library calls, fills the buffer with an ascending word pattern and a routine address, saves and restores a sixteen-longword table around a further library call, and returns a failure code if an allocation fails.
- `copy_header_longs_then_call`: Copies sixteen 32-bit values from a fixed table at a structure offset into a destination when one is supplied, then calls a stored routine pointer.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:echo

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header of sixteen consecutive values into a buffer and stores a routine pointer.
- `copy_buffer_and_call`: Copies a fixed block of sixteen words from a source table into a destination buffer when the buffer pointer is valid, then jumps to a routine whose address was passed in.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:Anthrox

- `release_resources_on_exit`: Cleanup path: calls helpers, releases a resource through a library call in a loop of three passes, clears a pointer slot and, if a saved memory block carries a matching tag, restores its saved word and frees the memory.
- `decrement_until_minus_one`: Decrements a local counter until it reaches minus one, then returns, acting as a short busy wait.
- `decode_packed_bitstream_entry`: Starts a decompression routine: it clears the instruction cache through the exec library, then decodes a packed stream by reading bits and copying groups of bytes, and calls a helper at the end when a size field is large enough.
- `decode_backward_copy_runs`: Decompresses a packed stream by reading bits, decoding run lengths and distances from the stream, and copying earlier output bytes backwards into place until the output counter is exhausted.
- `decode_backward_literal_runs`: Second variant of the backward decompressor, using byte-sized state: it reads bits and writes decoded bytes from the end of the output buffer backwards until the counter runs out.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:cd

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value in a field, then writes a small header block containing that value and a sequence of incremented words.
- `copy_buffer_and_call_2`: Copies a fixed block of sixteen words from a source table into a destination buffer when that buffer is not null, then hands control to another routine.

### installto

- `find_max_and_copy_table`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen 32-bit values from a fixed table into a destination pointer before invoking a stored routine pointer.
- `copy_words_then_call`: Copies a fixed number of words from a source table into a destination buffer when the destination is valid, then calls another routine.

### assign

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value in a field, then, when a condition holds, writes a small header of sixteen consecutive values into a buffer and copies sixteen 32-bit values from a fixed table to a destination pointer before invoking a stored routine pointer.
- `copy_words_and_call`: Copies a fixed block of words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### echo

- `find_max_copy_table`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header of consecutive values into a buffer and copies sixteen 32-bit values from a fixed table into a destination pointer before invoking a stored routine pointer.
- `copy_words_then_call_2`: Copies a fixed number of words from a source table into a destination buffer when the buffer is not null, then calls another routine.

### cd

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header of sixteen consecutive values into a buffer and copies sixteen 32-bit values from a fixed table into a destination pointer before invoking a stored routine pointer.
- `copy_words_then_call_3`: Copies a fixed number of 32-bit words from a source table into a destination buffer pointed to by a stack argument, stopping early if that argument is null, then invokes a routine whose address is held in a global.

### install.exe

- `setup_display_menu_loop_b`: Program entry point: it saves the library and interrupt state, writes display, sprite and DMA control registers, clears a block of variables and waits briefly, then reads key presses in a loop until Return ends the menu or Escape jumps to shutdown, after which the saved state is restored.
- `init_display_chips_menu_b`: Runs the same chip setup and key-driven menu loop as the program entry, reached through a handler pointer; the dos and exec library calls named in the evidence are made through the saved library base.
- `copy_entry_into_slot_b`: When the entry counter is below one, copies a fixed block of 48 bytes from a table into the slot selected by the current entry index and then advances that index.
- `scan_entries_restore_registers_b`: Walks the stored entries calling a check routine on each until one reports a match; if none matches it returns, otherwise it writes back saved display and DMA control values with the enable bit set and jumps through a stored routine pointer.
- `invoke_and_dispatch_routine`: Invokes a routine whose address is read from a table and stores the result in a global variable, then computes a pointer into a global table indexed by another global and sets up a few more globals before calling a second routine that branches based on a flag.
- `lock_caesar_submit_request_b`: Carries the same slot submission body as the preceding routine, while the evidence also lists a directory lock and a change of current directory that refer to the caesar directory on the hard disk.
- `create_caesar_directory_b`: Creates the caesar directory on the hard disk through the dos library, keeps the returned lock, and releases that lock again.
- `handle_text_entry_key_b`: Appends the pressed character to a fixed-length name buffer and advances the cursor, holding it at the buffer end; on the backspace code it clears the previous position and steps the cursor back, never below the start, and returns the character read.
- `record_text_key_press_b`: Applies the same text-entry handling as the neighbouring routine, storing the pressed character or handling backspace, without returning the character.
- `apply_default_parameter_values_b`: Stores five default numeric settings, sixty, seventy, two hundred, eight and four, into global variables and then calls a helper to apply them.
- `run_seven_helper_sequence_b`: Calls seven helper routines in order and returns, with no logic of its own in the evidence.
- `run_eight_routines_in_order`: Calls eight unnamed routines one after another and then returns.
- `run_seven_routines_in_order`: Calls seven unnamed routines one after another and then returns, with no loop or condition.
- `run_seven_routines_in_sequence`: Calls seven unnamed routines in a fixed order and returns, with no loop, data access or branching of its own.
- `wait_until_status_word_matches`: Repeatedly calls an unnamed routine while a global flag stays zero, and repeats this wait until a second global word holds the value thirteen.
- `call_two_unnamed_routines`: Calls two unnamed routines in sequence and returns.
- `chain_two_routine_calls`: Calls two unnamed routines back to back and returns.
- `call_unnamed_routine_twice`: Calls the same unnamed routine twice in succession and returns.
- `clear_table_entries`: Writes zero to each of 10,000 consecutive 32-bit longwords starting at the pointer it is given.
- `call_single_unnamed_routine`: Calls one unnamed routine and returns.
- `copy_memory_block`: Copies 10,000 consecutive 32-bit longwords from a source pointer to a destination pointer, advancing both pointers after each word.
- `stub_return_only`: Contains only a return and is not called from anywhere in the file.
- `clear_globals_and_memory_block`: Sets four global variables to zero, then zeroes a block of memory in ten passes that each clear three consecutive 32-bit longwords.
- `clear_table_entries_2`: Clears a block of four-byte entries in a table by writing zeros through a pointer in a loop.
- `clear_globals_and_memory_block_2`: Zeros four global variables, then loops nine times writing zeros to three consecutive 32-bit words through a pointer, clearing a contiguous block of memory.
- `invoke_callback_and_store_result`: Calls a routine whose address is read from a global table at a fixed offset, and stores the return value in a global variable only when that value is non-zero.
- `open_read_close_file`: Opens a file through a dos library wrapper, then reads a value and stores it in a global variable and closes the file.
- `write_data_and_close_file`: Sets a global flag to one on first use, then opens a file and, when that succeeds, writes through a dos library wrapper and closes the file.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `invoke_table_routine_store_result`: Calls a routine chosen from a global table by a fixed offset and stores its return value in a global variable.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `lock_dos_path_handle`: Calls the dos Lock function through the library base to obtain a lock, stores the returned value in a global variable, then calls one unnamed routine.
- `scan_directory_entries`: Examines a directory entry, then repeatedly fetches the next entry and calls an unnamed routine on each one.
- `set_flag_and_return`: Sets a global flag to one and returns the value that was passed to it on the stack.
- `set_flag_and_return_3`: Sets a global flag variable to one and returns the value that was already in the return register without any further processing.
- `call_once_and_return_byte`: On its first use, sets a global flag to one and calls an unnamed routine, then returns the byte that routine produced.
- `set_flag_if_unset`: Sets a global flag to one only when it is still zero, then returns a value held in a register.
- `cache_first_routine_result`: It uses a global variable as a one-shot guard so the wrapped routine runs only on the first call.
- `guard_once_call_routine`: Uses a global variable as a one-shot guard so its body runs only the first time it is called.
- `set_flag_and_invoke_callback`: Sets a global flag to one only when it is still zero, then calls a routine whose address is read from a table and stores the return value in a global variable only when that routine reports success.
- `check_interrupt_enable_bit`: Writes a fixed control value to a hardware-mapped global, then reads the interrupt enable register and calls an unnamed routine only when one interrupt enable bit is set.
- `initialize_strided_field_offsets`: Reads a base value from a global variable, sets a global flag to one, and stores the base value plus four values offset by 8000, 16000, 24000 and 32000 into consecutive fields of a structure.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d7.
- `update_bit_table_after_delay`: Saves a control variable, writes a short sequence of control values, and waits through a countdown loop of about a thousand passes.
- `update_bit_table_after_delay_2`: Saves a control variable, writes a short sequence of control values, and waits through a countdown loop of about a thousand passes.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_bit_table_after_delay_3`: Saves a control variable, writes a short sequence of control values, and waits through a countdown loop of about a thousand passes.
- `clear_table_byte_and_flag`: Clears one byte in a table addressed by a pointer plus an index, and zeroes a second variable.
- `wait_for_flag_then_dispatch`: Spins until a global variable becomes non-zero, then calls one of two unnamed routines depending on whether a pointer global holds the address of a global variable.
- `stub_infinite_loop`: Loops forever with no operations and never returns.
- `stub_infinite_loop_2`: The decompiled body is an empty infinite loop and no caller in the file reaches it.
- `set_global_pointers`: Stores the address of one global variable into three other global pointer variables and returns.
- `set_global_pointers_2`: Assigns the address of a global variable to three other global variables and returns without further processing.
- `nested_loop_calls`: Runs an outer loop of fifteen passes and an inner loop of thirty-two passes.
- `run_three_routines_in_order`: Calls three unnamed routines in order and returns, with no loop, condition or data access of its own.
- `call_three_and_store_words`: Calls three unnamed routines in turn and copies the 16-bit result each one leaves into its own global variable.
- `copy_two_words`: It moves two words from one memory location to another, copying them through a temporary register.
- `extract_nibble_fields`: Reads a global value and splits its lowest three 4-bit fields into three separate global variables, one field per variable.
- `pack_table_entry_word`: Builds a 16-bit value from two global variables, shifting one left by eight bits and the other by four, and adds the address of an unnamed routine.
- `load_table_entry_word`: Reads a 16-bit value from a table at an index held in a global variable and stores it into another global variable.
- `copy_masked_palette_words`: Calls one unnamed routine, then copies 32 16-bit words from a source pointer into a global table, keeping only the low twelve bits of each word.
- `mask_palette_words_in_place`: Calls one unnamed routine, then masks each of 32 16-bit words in a global table in place, keeping only the low twelve bits.
- `busy_wait_delay_loop`: Burns time with a nested countdown: an outer loop of 2500 passes, each running a short two-pass inner loop.
- `call_routine_seven_times`: Calls one unnamed routine seven times in a row and returns.
- `call_routine_six_times`: Calls one unnamed routine six times in a row and returns.
- `call_until_zero_result`: Repeatedly calls an unnamed routine and keeps going until its 16-bit result is zero, then returns.
- `busy_wait_countdown`: Spins a 16-bit countdown of about fifteen thousand passes as a timing delay, then returns.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d1.
- `write_decimal_digits_to_buffer`: Writes the decimal digits of a 16-bit value into a text buffer, filling from the last position backwards as ASCII character codes, then calls two unnamed routines.
- `write_digits_and_call_once`: Writes the decimal digits of a 16-bit value into a text buffer from the end backwards as ASCII character codes, then makes a single call to an unnamed routine.
- `blank_leading_zeros`: Walks a text buffer from the start and replaces each leading zero with a space, for as long as the next character is also a decimal digit.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `write_letter_digits_to_buffer`: Writes a 16-bit value into a text buffer backwards, encoding each digit as a letter counted from A, with a lowercase a for trailing zeros.
- `write_digits_from_lookup_table`: Takes the low 4-bit field of each of five successive nibbles in a 20-bit value and writes the matching character from a fixed 16-symbol lookup string into a text buffer.
- `write_two_zero_padded_groups`: Splits a 32-bit value at ten thousand, stores the quotient and remainder packed into one global, and writes each part as four zero-padded decimal digits into a text buffer.
- `format_number_into_fields`: Splits a 32-bit value into its upper and lower halves and writes the decimal digits of each into two separate fields of a text buffer, padding leading positions with zeros, then calls another routine.
- `write_letter_digits_high_word`: Writes digits taken from the upper 16 bits of a 32-bit value into a text buffer backwards as letters, filling unused positions with a lowercase a.
- `write_letters_from_high_word`: Writes the upper 16 bits of a 32-bit value into a text buffer backwards as letter digits, starting each run with a lowercase a.
- `write_padded_high_word_digits`: Writes the decimal digits of the upper 16 bits of a 32-bit value over a field of zero characters, then writes a stored remainder as four zero-padded digits into a second field.
- `write_padded_digits_second_field`: Writes the upper 16 bits of a 32-bit value as decimal characters into a second field of a text buffer, padded with zeros, then calls one unnamed routine.
- `walk_string_with_position_calls`: Reads four header words from a record, then walks a zero-terminated byte string.
- `compute_bitplane_offset_and_shift`: Converts an x position and a row value into a byte offset in a bitplane whose rows are 40 bytes long and whose words hold 16 pixels.
- `compute_bitplane_offset_and_shift_2`: Converts an x position and a row value into a byte offset in a bitplane whose rows are 40 bytes long and whose words hold 16 pixels.
- `compute_index_store_globals`: It computes an index from a global word and multiplies another global word by 40, then stores the index and a low nibble of the first word into two globals.
- `build_offset_store_globals`: It derives an index from a global word and scales a second global word by 40, then saves the index and the low nibble of the first word into two globals.
- `compute_index_and_store`: Computes a table index from a global word and multiplies another global word by forty, then stores the low nibble of the first word into a global.
- `dispatch_flagged_table_entry`: Checks a flag byte in a global table at an index formed from its argument minus 32.
- `apply_mode_mask_to_planes`: Processes five source bytes in turn.
- `apply_mask_to_plane_pair`: Processes five source bytes in turn, clearing bits in five words of each of two destination tables and then setting pattern bits in some of those words.
- `iterate_rectangle_cells`: Walks a rectangle of cells with nested loops: the outer loop steps through rows and the inner loop through columns.
- `write_pixel_across_five_planes`: Computes the word address of a pixel from its x and y values, then clears the pixel's bit in five plane words spaced 8000 bytes apart.
- `clear_and_set_plane_bits`: It resolves a pointer into a large word array, clears bit 15 of five spaced entries, then sets that bit in some of them based on individual flags in a global byte.
- `mask_plane_entries_by_flags`: It resolves a pointer into a large word array, clears bit 15 of five spaced entries, then conditionally sets that bit in some entries according to flags in a global byte.
- `update_plane_flag_bits`: It resolves a pointer into a large word array, clears bit 15 of five spaced entries, then sets that bit in some entries based on flags in a global byte.
- `decode_run_length_stream`: Decodes a run-length compressed stream whose header holds a big-endian length word and two fill bytes.
- `decode_merge_run_length`: It reads a big-endian length and two fill bytes from a source buffer, then decodes a run-length stream into a destination while ORing each produced byte with the destination's existing content.
- `merge_run_length_stream`: Decodes a run-length stream like the plain decoder, but combines each output byte with the byte already in the destination using a bitwise OR. Decoding stops when the decoded count reaches the header length.
- `or_merge_run_length_stream`: OR-merges a run-length stream into the destination, combining each decoded byte with the byte already there using a bitwise OR. Control bytes repeat a stream byte, repeat a fill byte from a global variable, or copy literal bytes, and decoding stops at the header length.
- `write_merged_run_length`: It expands a run-length stream, ORing each decoded byte into the destination and stopping when the destination limit is reached.
- `emit_merged_run_length`: It expands a run-length stream, ORing each decoded byte into the destination until the destination limit is reached.
- `stub_return_only_2`: Returns immediately without any work.
- `decode_run_length_skip_zero`: Decodes a run-length stream into a destination but leaves the destination unchanged wherever the byte to write is zero.
- `decode_skip_zero_run_length`: It expands a run-length stream into a destination, writing each byte only when it is non-zero and stopping at the destination limit.
- `write_nonzero_run_length`: It expands a run-length stream into a destination, storing each decoded byte only when it is not zero and stopping at the destination limit.
- `decode_run_length_skip_zero_2`: It writes a repeated byte to a destination a variable number of times, using the low six bits of a source byte as a count and skipping writes where the value is zero, then continues decoding further run-length entries until a terminator is reached.
- `stub_return_only_3`: Returns immediately without any work and is not called from anywhere in the file.
- `open_installer_console_window`: Opens a console window through the dos Open function using a raw window specification from the file, and stores the returned handle in a global variable.
- `poll_input_character`: Checks with the dos WaitForChar function whether input is ready, and if it is, reads one byte with dos Read into a global variable.
- `read_input_byte_check_range`: Reads one byte with the dos Read function into a global variable, then sets a flag to one when the byte is non-zero and below 123, and to zero otherwise.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:INSTALL.EXE

- `setup_custom_chip_registers`: Program entry that calls one library function and then stores a fixed sequence of values into custom chip registers for interrupt, DMA, display and sprite control.
- `run_installer_hardware_setup`: Looks up the current task through exec, sets up display, sprite, DMA and interrupt hardware registers, and runs a long sequence of calls.
- `copy_record_into_table`: When a global counter is below one, copies a 48-byte record from a fixed source into a global table at a slot chosen by an index, then advances that index.
- `scan_entries_restore_hardware`: Calls an unnamed routine on each table entry in order until that routine sets a flag.
- `setup_and_call_routine`: It stores a pointer into a global variable, sets up several fields in a structure, and then calls another routine, repeating the setup and call when a flag does not equal one.
- `invoke_and_setup_structure`: Invokes a routine read from a global table and stores its result, then sets up fields in a structure and calls another routine, repeating the setup and call when a global flag is not set.
- `copy_three_longs_then_run`: Calls one unnamed routine, copies the first three 32-bit values from a block pointed to by a global into three global variables, then calls four unnamed routines.
- `create_directory_then_unlock`: Creates a directory with the dos CreateDir function, stores the returned lock in a global variable, and then releases a lock with the dos UnLock function.
- `edit_input_line_buffer`: Edits a text input line.
- `edit_input_line_buffer_2`: Edits a text input line.
- `store_fixed_parameters_and_run`: Stores five fixed numeric values, 60, 70, 200, 8 and 4, into global variables and then calls one unnamed routine.
- `run_eight_routines_in_turn`: Calls eight unnamed routines one after another and returns, with no loop, condition or data access of its own.
- `call_routines_in_sequence`: It invokes a sequence of eight other routines in order and then returns.
- `run_seven_routines_in_turn`: Calls seven unnamed routines one after another and returns, with no loop, condition or data access of its own.
- `call_routines_in_sequence_2`: It invokes a sequence of seven other routines in order and then returns.
- `call_routines_in_sequence_3`: Invokes a pair of unnamed routines twice in succession and then returns without modifying any data.
- `read_input_until_return`: It repeatedly reads a validated input byte and loops until the byte equals the carriage return value, then returns.
- `call_two_unnamed_routines_2`: Calls two unnamed routines in sequence and returns.
- `call_two_routines`: It invokes two other routines in sequence and then returns, performing no other operations.
- `call_two_routines_2`: Invokes one unnamed routine and then a second unnamed routine before returning.
- `clear_table_entries_3`: It clears a table of 10,000 32-bit entries by writing zero to each entry via a pointer, decrementing a counter from 9,999 down to -1.
- `call_single_unnamed_routine_2`: Calls one unnamed routine and returns.
- `copy_memory_block_2`: It copies a block of 10,000 32-bit words from a source pointer to a destination pointer by repeatedly storing each source word to the destination and advancing both pointers.
- `empty_routine`: It is empty, performing no operations and returning immediately.
- `clear_struct_and_table`: It zeroes four global variables, then loops nine times writing zero to three consecutive 32-bit words through a pointer, advancing the pointer by three each iteration, and returns.
- `clear_memory_block`: It clears a contiguous block of memory by repeatedly storing zero through a pointer.
- `reset_cohort_state`: It zeroes four global variables and then iterates nine times, writing zeros to three consecutive 32-bit words through a pointer that advances each pass.
- `invoke_callback_and_store_result_2`: Calls a routine whose address is read from a global table, and stores its non-zero return value in a global variable.
- `chain_calls_store_result`: Calls one unnamed routine, and when it returns non-zero calls a second routine and stores that result in a global variable if it is not minus one, then makes a third call.
- `chain_calls_with_init_flag`: On first use sets a global flag to one and calls an unnamed routine.
- `call_open_store_handle`: Calls the dos Open function through the library base and stores the returned handle in a global variable.
- `call_dos_read_wrapper`: Calls the dos Read function through the library base and returns, with no other work.
- `invoke_callback_store_result_2`: Reads a routine address from a global table using an offset, invokes it, stores the result in a global, and returns.
- `call_dos_write_wrapper`: Calls the dos Write function through the library base and returns, with no other work.
- `call_dos_close_wrapper`: Calls the dos Close function through the library base and returns, with no other work.
- `lock_dos_path_handle_2`: Calls the dos Lock function through the library base, stores the returned lock in a global variable, then calls one unnamed routine.
- `lock_dos_path_handle_3`: Calls the dos Lock function through the library base, stores the returned lock in a global variable, then calls one unnamed routine.
- `scan_entries_cycle_colors`: Examines the first entry of a directory, then calls an unnamed routine and fetches further entries with ExNext.
- `stub_return_only_4`: Returns immediately without any work.
- `scan_device_entries_locked`: Locks the text DF0: through dos Lock and examines it, then calls an unnamed routine for each entry that dos ExNext returns, for at most 201 entries.
- `set_flag_and_return_2`: Sets a global flag to one and returns the value that was passed to it on the stack.
- `set_flag_and_return_4`: It sets a global flag to one and returns the value that was passed in the stack slot as its result.
- `run_once_wrapper`: Executes a one-time initialization exactly once by checking a global flag, then forwards to a helper routine and returns its value.
- `execute_single_time`: Guards a block of code so it runs only on the first entry using a global flag, then delegates to a helper routine and returns its result.
- `guard_one_shot`: Performs a guarded one-shot setup by setting a global flag before calling a helper routine and returning its value.
- `ensure_single_run`: Ensures a routine body runs a single time by testing and setting a global flag, then returns the helper routine's value.
- `invoke_callback_store_result`: Invokes a routine whose address is read from a global table and stores its non-zero return value into a global variable.
- `check_interrupt_bit_and_call`: It writes a value to a global variable, checks a bit in an interrupt register, and calls another routine when that bit is set before returning.
- `initialize_cohort_offsets`: It stores a base value read from a global variable into a global flag and into five offsets within a structure pointed to by a register.
- `disable_interrupts_set_flag`: Disables interrupts, spins a delay loop, then updates a global flag and a memory location based on a bit of the saved interrupt state before restoring interrupts.
- `save_interrupt_state`: Saves the interrupt enable state, runs a short delay loop, records the state in a global variable, and restores the previous interrupt setting.
- `block_interrupts_delay`: Turns interrupts off, executes a fixed delay loop, and then re-enables them using the previously captured interrupt flag.
- `pause_interrupts_loop`: Disables interrupts, performs a short busy-wait loop, and restores the interrupt state captured at entry.
- `reset_keyboard_controller`: It disables interrupts, zeroes a global flag and a delay loop, then configures the keyboard controller register and sets or clears a bit in a global byte based on a saved value.
- `clear_slot_and_set_flag`: Zeros a global flag, writes zero to a byte addressed by a pointer and index, clears a bit in a global variable, sets that global to a constant, and returns.
- `wait_for_nonzero_and_call`: It spins until a global variable is non-zero, then calls another routine unconditionally, and calls another routine once more depending on whether a global variable equals a constant, otherwise calling a different routine.
- `infinite_loop_stub`: It contains an infinite loop with no operations, so it never returns.
- `call_pointer_routine`: It loads a pointer from a global variable and then calls another routine before returning.
- `assign_global_pointers`: Assigns the address of a global variable to three other global variables and then returns.
- `set_global_pointers_3`: It stores the address of a global variable into three separate global variables and then returns without performing any other operations.
- `nested_loop_calls_2`: It runs two nested loops, decrementing a counter from a start value down to zero.
- `store_shorts_between_calls`: Stores three incoming short values into global variables, issuing one call between each of the two remaining stores.
- `recursive_self_call`: It calls itself three times, storing three incoming short values from registers into global variables between the calls.
- `copy_global_value`: It loads a value from a global variable into a register, stores that value into another global variable, and then returns.
- `extract_nibble_fields_2`: It reads a byte from a global variable and splits it into three 4-bit fields.
- `store_table_entry_2`: It computes an index from a global variable and stores a 16-bit value built from two global variables and the address of another routine into a table at that index.
- `store_table_entry`: Reads a 16-bit value from a table using an index derived from a global variable and stores it into a global variable.
- `decode_color_table_2`: It iterates over a table of 16-bit words, extracting the three 4-bit nibbles from each word and storing their sum into a global array.
- `decode_color_table`: It calls another routine once, then iterates over 32 entries, extracting the three 4-bit nibbles from each 16-bit word, summing them into a 16-bit value, and writes the result back into the same global table.
- `delay_loop`: It runs a nested counting loop that decrements a short from a starting value down to -1, with an inner loop that decrements a second short from 1 to -1 on each outer iteration.
- `call_routine_repeatedly`: It stores a counter, then repeatedly calls another routine while decrementing the counter until it reaches its limit.
- `run_repeated_delay`: It runs a counting loop that calls another routine five times with a delay loop between calls.
- `run_spin_delay_loop`: Runs a counting loop that decrements a short from a fixed starting value down to -1.
- `spin_delay_loop`: It decrements a short from a fixed starting value down to -1 in a single counting loop.
- `convert_to_decimal_string`: It converts a positive integer into its decimal digit characters, writing one character per iteration to consecutive bytes of a buffer.
- `write_decimal_digits_to_buffer_2`: It writes the decimal digits of a value held in a register into a buffer pointed to by a pointer, storing each digit as an ASCII character in reverse order.
- `extract_digits_to_buffer`: It writes decimal digits of a value held in a register into a byte buffer pointed to by a pointer in a register, using a loop that decrements a counter in a register.
- `blank_time_characters`: Walks a byte buffer replacing characters that form a time value with spaces.
- `empty_routine_2`: It is an empty routine that returns immediately without accessing memory, calling other routines, or modifying any data.
- `store_digit_string_and_offset_2`: It converts a 32-bit value into its decimal digits, writing one ASCII character per iteration into a buffer while decrementing the destination index until it underflows.
- `decode_nibble_table`: Extracts four 4-bit nibbles from a register value and writes the corresponding table entries into four consecutive bytes, then invokes another routine twice.
- `split_and_store_halves`: Splits a 32-bit value into its high and low halves, writing each half as a two-digit decimal string into two separate buffers.
- `store_two_digit_strings_2`: It splits a 32-bit value into its high and low halves and writes each half as a two-digit decimal string into two separate buffers.
- `store_digit_string_and_offset`: Writes the digits of a 32-bit value into a buffer, then computes a bitplane offset and shift before returning.
- `store_uppercase_letters`: It writes uppercase letter characters to consecutive bytes of a buffer, then calls a routine that computes a bitplane offset and shift before returning.
- `store_two_digit_strings`: Writes the digits of a 32-bit value into a buffer, then processes a second value into another buffer before returning.
- `store_digit_string`: Writes the digits of a 32-bit value into a buffer, decrementing the destination index until it underflows.
- `invoke_routines_from_table`: It copies several values from a pointer into global variables, then walks a byte table calling a routine for each non-zero entry while advancing the pointer.
- `compute_table_offset_store`: It computes an address from a shifted register value and a table index, stores it in a global variable, and stores a masked register value in another global variable.
- `split_value_into_globals`: Splits a 16-bit value into a high part shifted right by three and masked, multiplied by 40, and a low nibble, storing each result in a separate global variable.
- `compute_byte_offset`: Computes a byte offset from a global variable by shifting one value right and combining it with another, then stores the result through a pointer while saving a masked nibble of the source into a separate global.
- `split_and_store_value`: Extracts the low nibble of a global value and stores it as a byte through a pointer, then computes a table index from the same value and stores the resulting word at a second pointer.
- `compute_table_offset_and_store`: Computes a table address from a shifted register value and a table index, storing it in a global variable.
- `conditionally_invoke_routine`: It checks a global flag, stores a register value into a global variable, and then calls another routine only when that flag is set and a value equals two.
- `apply_cohort_mask_pattern`: Reads a byte from a source pointer and a shift amount from a global variable, builds a mask and a shifted pattern, then applies an AND-clear followed by an OR-set to five groups of 32-bit words in a destination table selected by a mode global variable.
- `apply_mask_to_planes`: Applies a per-byte mask to two 32-bit destination tables, clearing bits in five words and setting bits in four of them for each of five source bytes.
- `invoke_nested_loop`: It adds two global values into temporary slots, then runs a nested loop that stores the slot values into globals and calls another routine for each combination of the two counters.
- `clear_and_set_buffer_bits`: It computes a pointer from a table index and a base value, then clears a bit selected by a register in five words of that pointer's buffer.
- `apply_bit_mask_to_buffer`: Computes a bit mask from a shift amount and applies it to five words spread across a large buffer, clearing or setting each one depending on flag bits.
- `apply_bit_mask_to_regions`: Computes a bit mask from a byte and a bit position, then applies that mask to five separate 32KB regions of a large RAM table.
- `apply_bitmask_to_buffer`: It computes a pointer from a global shift value and a global base, then clears or sets one bit in five words of that pointer's buffer depending on a global flag byte.
- `unpack_compressed_stream`: Decodes a packed byte stream into an output buffer, handling literal runs, repeated-byte fills, and copied sequences until the target length is reached.
- `decode_run_length_buffer`: It decodes a run-length encoded byte stream from a source buffer pointed to by a0 into a destination buffer pointed to by a1.
- `expand_rle_into_buffer`: It reads a length and control bytes from an input buffer, expands repeated-byte runs into an output buffer, and honors the two-bit control codes to fill or copy data until the target count is reached.
- `expand_rle_runs`: It decodes a packed byte stream by expanding repeated-byte runs into an output buffer, reading control bytes and honoring their two-bit codes to fill or copy data until the target count is reached.
- `or_bit_ranges`: It ORs a run of bits from one byte array into another byte array, where the bit positions are expressed as a 24-bit offset.
- `copy_bit_run`: It ORs a run of bits from one byte string into another, where the run length and start bit are encoded in the source bytes.
- `return_stacked_value`: Returns a value held on the stack without performing any other operation.
- `unpack_masked_run_length_stream`: Decompresses a packed stream whose header gives the output length and a fill byte.
- `unpack_remaining_run_packets`: Continues the same unpacking loop through the run and literal packets that follow the first one, until the output length is reached.
- `decode_packed_run_fragment`: Holds a fragment of the same unpacker that handles a two-byte run packet, a fill-byte run and a literal copy.
- `decode_run_packets_until_limit`: Reads packets in the same format and writes their bytes to the destination until the output count reaches its limit.
- `stub_return_stack_argument`: Returns the value it was passed on the stack without changing it, and performs no other work.
- `open_dos_file_handle`: Opens a file through the DOS library's open function and stores the returned file handle for later reads.
- `poll_and_read_valid_byte`: Polls the DOS input for a waiting byte.
- `read_validated_input_byte`: Reads one byte from the DOS input unconditionally, setting the valid flag before the read.
- `close_dos_file_handle`: Closes the file handle through the DOS library's close function, with no other work of its own.
- `drain_pending_input_bytes`: Repeatedly polls the DOS input and reads each waiting byte, setting the valid flag each time, until no input is left.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[HD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_custom_chips_and_copper`: Saves the current interrupt and DMA enable state, disables both, clears the sprite pointer registers and writes the copper list entries for colour and sprite data.
- `advance_ping_pong_buffer_pair`: Calls four helper routines, then writes a new address made from the second stored value plus 350 into two words, and swaps the two stored values so they alternate each time.
- `load_three_table_bytes`: For a nonzero index above 16, reads three byte values at a four-byte stride into three variables.
- `swap_buffer_pair_stepping_back`: Calls two helper routines, then writes a new address made from the second stored value minus 2 into two words and swaps the two stored values.
- `pass_through_helper_call`: Calls one helper routine and returns, with no other work of its own.
- `store_shifted_values_twice`: Calls a helper, stores the incoming value shifted right by three into one variable, calls the helper again, and stores the value shifted right by six into a second variable.
- `step_and_mask_coordinate_pair`: Adds a delta word to each of two position words, then keeps only bits 1 to 11 of each result, so the values stay even and within twelve bits.
- `start_blitter_operation`: Calls a helper, then programs the blitter by setting the word masks, control register, destination modulo and destination address, and starts a fixed-size blit.
- `start_blitter_operation_2`: Programs the blitter control registers and starts an operation after waiting for the previous one to finish.
- `advance_interrupt_state_machine`: When a flag bit is set, reads a value through a helper and steps a small state machine: a waiting state counts down to zero, a counting state runs for 110 steps before moving to a final state, and the flag bit is then cleared.
- `start_blitter_operation_variant`: Calls a helper, then programs the blitter with the same word masks, control register, modulo and destination setup as the other blitter starts, but with a different fixed blit size, and starts it.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_blitter_idle`: Busy-waits while the blitter busy flag is set in the hardware status register, then returns.
- `reset_text_scan_state`: Resets the text scanning state: points the read position at a fixed data address, clears a column counter, sets a row counter to one and clears a flag byte.
- `parse_text_stream_character`: Reads one character of a text stream.
- `advance_text_stream_after_char`: Advances the column count first, starting a new row at 41 columns, then reads the next character with the same rules: zero bytes are skipped, code 1 restarts the stream, carriage returns start a new row, and other characters go to a helper.
- `start_text_row_then_parse`: Clears the column count and increments the row count, then reads the next character of the stream with the same rules: zero bytes are skipped, code 1 restarts the stream, and other characters go to a helper.
- `setup_blitter_masked_copy`: Programs a blit by setting the source data registers to all ones, the modulo and pointers from two inputs, and the word masks and control values from whether each input is odd or even.
- `build_copper_colour_table_block`: Writes two fixed copper move pairs at fixed offsets from the block base, then copies 64 entries from a table of six-word records into two interleaved halves of the copper list.
- `build_copper_colour_table_second`: Calls a helper, then writes two fixed copper move pairs at a second pair of block offsets and copies 64 six-word table records into two interleaved halves of that copper list.
- `copy_copper_colour_table_block`: Writes two fixed copper move headers into the destination, then copies 64 six-word table records into two interleaved halves of the destination list through the given pointers.
- `set_copper_list_pointers`: Writes the low and high words of four copper list addresses, each offset from a base pointer by a fixed block offset, into the copper pointer locations, and then stores a fixed control word.
- `encode_sprite_position_words`: Halves a horizontal start value and a second value offset by 16, keeps the low bits of each, and merges in the high bits of a further value.
- `set_table_end_pointer`: Stores the input address as the current table pointer and computes the end of that table, which starts with a count word followed by four-byte entries, and stores that end address too.
- `toggle_flag_after_helper`: Calls a helper routine, then flips the lowest bit of a stored flag value.
- `save_parameters_then_run_helpers`: Saves the two address parameters and two word parameters into stored variables, then runs three helper routines in sequence.
- `wait_for_blitter_idle_2`: Waits in a loop until a status byte clears its busy flag before returning.
- `start_blitter_rectangle_blit`: Computes the width in words and height in rows of a rectangle from four stored coordinates, then programs the blitter with a first-word mask, control register, destination address and modulo, and starts a blit of that size.
- `start_blitter_rectangle_copy`: Computes the width and height of a rectangle from stored coordinates, then programs the blitter with source and destination pointers, modulos and a copy-mode control register, and starts the blit.
- `project_coordinate_list_points`: Walks a counted list of coordinate pairs, scaling each by a masked 10-bit fixed-point factor and offsetting it by 160 horizontally and 128 vertically into an output buffer, and calls a helper for each point.
- `dispatch_indexed_pairs_to_helper`: Walks a counted table of index pairs and, for each pair, looks up two words in a lookup table at four-byte stride, then passes them with the next index to a helper routine.
- `set_default_bounds_rectangle`: Writes four fixed words, 160, 160, 128 and 128, into the bounds record, which resets the stored rectangle to a default extent.
- `expand_bounds_rectangle`: Widens a stored bounding box to include a new point, lowering the minimum and raising the maximum of each axis where needed.
- `start_blitter_line_draw`: Orders two endpoints, computes the blitter line parameters from their horizontal and vertical distances, sets the line control and pixel masks, and starts a line draw into the destination.
- `build_linked_entry_table`: Builds a table of 30 linked entries in a memory block, zeroing each entry and recording its address in a pointer list, then sets the status flags to 15.
- `update_audio_channel_pointers`: Advances a frame counter and, when it reaches its limit, runs the audio helpers and loads the four audio channel start pointers from stored addresses, with short delays between writes.
- `clear_audio_channel_state`: Clears four audio state variables to zero and then sets a final control variable to 15, resetting the state of the four audio channels.
- `select_audio_channel_period`: Calls helpers, then looks up a value for the current channel state, choosing the table column from the frame count modulo three, and writes it to the audio channel register.
- `step_audio_channel_sequence`: Steps one audio channel through its sequence: when the current word is zero it loads the next word, then sets the channel sample address, length and volume from a per-instrument table, applying any effect in the word and switching the channel DMA on.
- `process_audio_channel_command`: Processes one tick of an audio channel command byte.
- `copy_record_word_field`: Copies one word from a fixed offset in a source record into a fixed offset in the destination record.
- `find_period_table_entry`: Searches a table of 37 word values for the first entry at least as large as the current masked period, steps back one entry when a flag bit is set, and stores the chosen value as the target, setting a direction flag when the target is below the current period.
- `apply_pitch_slide_step`: Moves a channel pitch value up or down by a step each tick until it reaches the target, then clears the slide count.
- `modulate_channel_period_wave`: Computes a modulation offset for an audio channel: it reads a waveform value from a table or a ramp, scales it by a depth nibble, adds it to the base period, and advances the phase.
- `dispatch_audio_effect_command`: Dispatches a channel effect nibble: some values store a byte as a position or volume, one sets a repeat counter, and one clamps the volume to 64.
- `store_channel_nibble_field`: Stores the low four bits of a byte from the channel record into a second field of that record.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
