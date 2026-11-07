# Game 2421 (caesar deluxe (1993)(impressions)[cr atx](disk 1 of 2)[a][hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02421). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908528 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 301 of 301 named (0 author symbols), 301 of 301 described (12 by rule, 80 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1333 of 1333 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5728 of 5764 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3364 |
| code | 22208 |
| data | 71097 |
| padding | 5459 |
| unused | 806400 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:installto | 2 | 117 | 2 |
| ram-handler | 2 | 10 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:assign | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:echo | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:Anthrox | 6 | 251 | 6 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:cd | 2 | 117 | 2 |
| installto | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| install.exe | 3 | 1394 | 112 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:INSTALL.EXE | 3 | 1493 | 118 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1670 | 47 |

## Cases handled

- runtime segment an address: 1085 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_and_execute_setup` (install.exe): Configures various hardware registers including display and DMA settings, clears memory blocks, and executes a sequence of setup routines. Kind: disk hardware/track check loop. Pass path: u10__l0_020C.
- `init_display_chips_for_menu` (INSTALL.EXE): Runs the same chip setup and key-driven menu loop as the program entry, reached through a handler pointer; the dos and exec library calls named in the evidence are made through the saved library base. Kind: loader/menu loop with conditional branch. Pass path: if (_u11__d0_24F6 == 0xd) break;.

## Functions


### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:installto

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen longwords from a fixed offset in the structure at the A2 register into a caller-supplied buffer when that buffer is given, then calls another routine.

### ram-handler

- `copy_memory_then_dispatch`: Calls the operating system CopyMem routine through its library vector after a short setup, then makes an indirect call to an address that could not be recovered.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:assign

- `build_header_block_from_max`: Walks a linked list of entries to find the largest value in a field and then writes an ascending run of longwords into a new block sized from that value.
- `copy_memory_block_3`: Copies sixteen bytes of data from a memory location offset from a pointer to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:echo

- `update_linked_list_and_sync_buffer`: Traverses a linked list to find the maximum value in a specific field, then updates a structure with this value and populates a buffer by copying data from a source pointer.
- `copy_buffer_and_call_callback`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer and then executes a function via a pointer.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:Anthrox

- `free_resources_and_restore_entry`: Frees memory through the operating system library after a short setup, repeating a small sequence of library calls around it.
- `decrement_until_minus_one`: Decrements a 16-bit register until it reaches minus one, then returns.
- `clear_cache_then_decode_runs`: Clears the processor cache through the operating system library, then runs a bit-driven loop that decodes packed byte runs.
- `decode_bitstream_copy_loop`: Decodes a bit-packed stream and copies the decoded byte runs from a source pointer into a destination buffer.
- `decode_run_length_data`: Expands compressed data by reading a byte and replicating it a specified number of times.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:cd

- `update_max_and_sync_structure`: Traverses a linked list to find the maximum value in a specific field and updates a structure with this value.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then calls another routine.

### installto

- `find_max_and_expand_list`: Traverses a linked list to find the maximum value in a specific field and then expands the list structure by allocating or calculating space for new entries.
- `copy_memory_block_and_call_2`: Copies a fixed number of words from a memory location to a destination pointed to by a stack value, then calls another routine.

### assign

- `update_linked_list_and_sync_buffer_2`: Traverses a linked list to find the maximum value in a specific field and then updates a structure with this value and a sequence of incrementing longwords.
- `copy_memory_block_and_call_3`: Copies a fixed number of words from a memory location to a destination address and then executes another routine.

### echo

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field and then updates a structure with this value.
- `copy_memory_block_4`: Copies a fixed number of words from a memory location to a destination pointed to by a stack value.

### cd

- `find_max_and_reallocate_buffer`: Traverses a linked list to identify the maximum value in a specific field and then expands a data structure by writing a header and copying existing data into a newly sized memory block.
- `copy_short_block`: Copies sixteen bytes from a source address to a destination address.

### install.exe

- `run_install_menu_loop`: Writes several custom chip registers to set up the display and interrupt state, clears a block of memory and then loops reading a key value until Enter or Escape is pressed.
- `configure_hardware_and_execute_setup`: Configures various hardware registers including display and DMA settings, clears memory blocks, and executes a sequence of setup routines.
- `copy_data_block`: If a global variable is zero, it copies a block of forty-eight bytes from a fixed source to a destination in a table and increments a counter.
- `find_pending_slot_and_restore`: Scans the table slots in turn, calling a check routine on each, and stops at the first slot that reports a set flag.
- `transfer_slot_with_retry`: Calls a library routine, records the current slot address and a fixed buffer size, and runs a transfer step.
- `lock_install_directory`: Calls the DOS routines to lock the install directory and make it the current directory, then runs the same slot transfer step.
- `create_install_directory`: Creates the install directory with a DOS call and then releases the lock it was returned through another DOS call.
- `handle_text_entry_key`: Handles one key of text entry: a normal key is stored in the input buffer and the position advances up to a maximum.
- `handle_text_entry_key_2`: Handles one key of text entry: a normal key is stored in the input buffer and the position advances up to a maximum.
- `set_default_display_parameters`: Stores fixed default values into several global parameter variables, then calls another routine to apply them.
- `run_seven_setup_steps`: Calls seven routines in a fixed order with no logic of its own.
- `run_eight_setup_steps`: Calls eight routines in a fixed order with no logic of its own.
- `execute_repeated_calls`: Executes a specific routine seven times in succession.
- `execute_complex_data_processing_sequence`: Performs a series of nested operations including memory block copying, run-length decoding, and color table population.
- `wait_for_enter_key`: Polls a key-event routine repeatedly until a key event arrives, and returns once the key code is the Enter key.
- `call_two_routines`: Calls two routines in sequence with no other logic.
- `execute_empty_callback`: This routine is an empty function that performs no operations before returning.
- `clear_table_entries_3`: This routine calls another routine that writes zero to ten thousand longwords starting at the address in the A0 register.
- `clear_table_entries`: Writes zero to each of 10,000 longwords starting at the address in the A0 register, then returns.
- `forward_to_one_routine`: Calls one other routine and then returns.
- `copy_memory_block`: Copies 10,000 longwords from the source address in A0 to the destination in A1, advancing both pointers on each step.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `reset_state_and_clear_block`: Clears four global variables and then zeroes ten groups of three longwords starting at a fixed RAM address.
- `initialize_memory_block`: Sets several global variables to zero and then fills a memory block with zeros using a loop.
- `reset_cohort_state`: Clears four global variables and then zeroes a block of memory in RAM by iterating nine times and writing zeros to three consecutive words per iteration.
- `invoke_callback_and_store_result`: Calls a routine taken from the global function table and, when the returned value is non-zero, stores it in a global variable.
- `open_and_store_file_handle`: Calls a first routine and, if it succeeds, calls a second one whose result is stored as a handle.
- `write_block_with_status`: Runs a one-time setup step, then calls a routine whose result gates a sequence of three further calls.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_table_entry_store_result`: Calls a DOS library routine selected from the global function table and stores its return value in a global variable.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `lock_path_and_store_lock`: Locks a path through the DOS library and stores the returned lock value in a global variable.
- `scan_directory_entries`: Examines a locked directory and repeatedly advances through its entries, calling a further routine for each one.
- `set_failure_flag_and_return`: Sets a failure flag to one and returns the stack value unchanged.
- `set_status_flag`: Sets a global variable to one.
- `run_first_time_init`: On the first call, sets a one-time flag and calls an initialisation routine.
- `get_flagged_value`: If a global variable is zero, it sets that variable to one and calls another routine to return its result.
- `enforce_single_execution`: Checks a global flag to determine if the routine has already been executed.
- `prevent_recursive_execution`: Checks a global flag to determine if the routine has already been executed.
- `prevent_recursive_execution_2`: Checks a global flag to ensure a specific routine is only executed once.
- `initialize_structure_offsets`: Stores a base value from a global variable into a flag and five consecutive fields of a structure.
- `initialise_block_offsets`: Stores a base value read from a global into a flag variable and into five consecutive fields of the structure at the A6 register, spaced by fixed steps of 8000.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d7.
- `record_entry_after_delay`: Stores a fixed sequence into a control byte and waits through a countdown loop.
- `update_table_index`: Calculates an index based on a bit-shifted value and updates a table and several global variables.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_table_entry`: Modifies a global variable and updates a specific entry in a table based on bitwise operations and a calculated index.
- `clear_slot_byte_and_flag`: Clears one byte of a table at the A0 register plus the D0 offset and resets a flag variable.
- `wait_for_nonzero_and_call`: Waits in a loop until a global value is non-zero, then calls one of two routines depending on whether a pointer matches a fixed address.
- `infinite_loop_stub`: Loops forever with no operations, so it never returns.
- `infinite_loop_stub_2`: Loops forever with no body in the decompiled code, so control never returns.
- `set_global_pointers`: Assigns the address of a global variable to three other global pointer variables, then returns.
- `set_global_pointers_4`: The routine assigns the address of a global variable to a global variable and assigns the same constant value to two other global variables.
- `nested_loop_calls`: Runs an outer loop of fifteen passes, each making a call.
- `call_three_routines`: Calls three routines in sequence with no other logic before returning.
- `capture_three_register_words`: Calls three routines in turn and stores the low word of the data register left by each call into a separate global variable.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `extract_nibble_fields`: Splits a 16-bit value into three 4-bit fields, storing the low nibble, the next nibble and the third nibble in three separate global variables.
- `store_packed_table_entry`: Builds a 16-bit value from two global nibble fields and a fixed value, and stores it into a table entry at an index shifted left by one.
- `load_table_entry_word`: Loads a 16-bit value from a table at an index derived from a global variable and stores it into another global variable.
- `decode_packed_color_entries`: Calls two routines, then copies 32 words from the table at the A3 register into a global color table.
- `decode_packed_color_entries_2`: Calls one routine, then decodes the 32-entry packed color table held in a global variable.
- `delay_loop`: Burns time with a nested countdown: an outer loop of 2,500 passes, each running a short inner countdown.
- `repeat_call_seven_times`: Calls another routine seven times in succession using a countdown loop, then returns.
- `repeat_call_six_times`: Calls another routine six times in succession using a countdown loop, then returns.
- `call_until_zero_result`: Repeatedly calls another routine and keeps going until the value it returns is zero, then returns.
- `delay_long_countdown`: Counts a 16-bit value down from 15,000 to minus one to waste time, then returns.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d1.
- `write_decimal_digits_to_buffer`: Converts a value to decimal digits and writes them as ASCII characters into a buffer, last digit first.
- `write_decimal_digits_to_buffer_2`: Converts a value to decimal digits and writes them as ASCII characters into a buffer, last digit first.
- `blank_leading_zeros`: Replaces leading zero characters in a text buffer with spaces, stopping at the first non-zero digit or non-digit character.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `write_letter_digits_to_buffer`: Writes the digits of a value into a buffer as letters: each nonzero digit becomes an uppercase letter counted up from A, and a zero digit becomes a lowercase a.
- `write_nibbles_as_hex_text`: Writes the five low nibbles of a value into a buffer as characters from a sixteen-character digit table, then calls two routines.
- `write_split_decimal_digits`: Splits a value at ten thousand and writes the four digits of the quotient and the four digits of the remainder into the buffer.
- `format_split_decimal_digits`: Splits a 32-bit integer into two 16-bit halves and converts each half into a four-digit decimal string.
- `convert_integer_to_letter_buffer`: Extracts decimal digits from a value and maps them to uppercase letters, storing the resulting characters into a destination buffer.
- `convert_digits_to_letters`: Converts the digits of a value into uppercase letters and stores them in a buffer.
- `write_zero_padded_digits`: Writes a value into a buffer as decimal digits padded with zeros, then fills a second group of digits and calls one further routine.
- `convert_digits_to_chars`: Converts the digits of a value into character representations and stores them in a buffer.
- `walk_text_with_parameters`: Loads four 16-bit parameters from the block at the A1 register into globals, then walks a zero-terminated byte string that follows them.
- `compute_screen_offset`: Computes a byte offset from a row and a column, using forty bytes per row, and stores it with the low nibble of the column.
- `calculate_pointer_offset`: Calculates a pointer address and a secondary value using bitwise operations and stores them in global variables.
- `calculate_bitmask_offset`: Calculates a base address for a bitmask pattern by scaling a value and adding a shifted offset.
- `calculate_bitmask_offset_2`: Calculates a base address by scaling a value and adding it to a shifted offset.
- `calculate_bitmask_offset_3`: Calculates a base address for a bitmask pattern by scaling a value and adding a shifted offset.
- `dispatch_by_mode_flag`: Checks a flag in a table indexed by a value and, when it is set, stores a pointer and dispatches to one of two routines depending on a mode value.
- `apply_bit_mask_pattern`: Reads five source bytes and shifts each into a bit mask, then clears and sets bits in five groups of longwords in a destination table chosen by a mode value.
- `apply_mask_to_two_planes`: Processes five source bytes, building a bit mask from each and clearing then setting bits in two destination tables.
- `iterate_cell_grid`: Walks a rectangular range of cells row by row and column by column, setting the cell position globals and calling a routine for each cell.
- `write_cell_bits_to_planes`: Computes the address of a cell from its column and row, clears its bit in five bitplanes, then sets the bit in each plane whose flag bit is set in a flag byte.
- `modify_bitplane_data`: Clears a specific bit across multiple memory planes and conditionally sets it based on a bitmask in a global variable.
- `update_bitmask_segments`: Calculates a base pointer using arithmetic on global values and applies a bitmask to five distant memory locations.
- `modify_bitmask_at_offsets`: Calculates a pointer based on several global values and applies a bitmask to five specific word-aligned offsets.
- `decode_run_length_buffer`: Decodes a run-length packed byte stream from the source at A0 into the destination at A1.
- `decode_run_length_buffer_2`: Decodes a run-length encoded byte stream from a source buffer into a destination buffer.
- `merge_run_length_buffer`: Decodes a run-length packed byte stream like its sibling routine, but ORs each decoded byte into the destination rather than overwriting it.
- `or_fill_run_stream`: Reads run-length control bytes from the source at A0 and ORs each repeated fill byte into the destination for its run.
- `merge_run_length_buffer_2`: Decodes a run-length encoded byte stream and performs a bitwise OR of each decoded byte into the destination buffer.
- `merge_run_length_buffer_alt`: Decodes a run-length encoded byte stream and performs a bitwise OR of each decoded byte into the destination buffer using a different control flow.
- `stub_return_stack_value`: Returns the value found at the top of the stack without changing it.
- `decode_transparent_run_buffer`: Decodes a run-length packed stream into the destination but writes only non-zero bytes, so existing destination bytes stay where the decoded value is zero.
- `decode_transparent_run_buffer_2`: Decodes a run-length packed stream while skipping zero bytes, so existing destination bytes are kept.
- `decode_transparent_run_buffer_3`: Decodes a run-length encoded byte stream into a destination buffer, writing only non-zero bytes.
- `unpack_run_length_stream`: Expands a packed byte stream by repeating one byte or copying literal bytes, with the top two bits of each control byte selecting the mode, and writes the output to a buffer until the end counter is reached.
- `nop_padding_sequence_2`: This routine is empty and performs no operations.
- `open_dos_file_handle`: Calls the dos library Open function through the library base with the arguments already in place and stores the returned handle in a global variable, without checking the result.
- `poll_input_character`: Waits for input on a file handle with the dos library and, when a byte is ready, reads it and keeps the input flag set only if the byte is nonzero and below 123; otherwise the flag is cleared.
- `read_input_byte_flagged`: Sets the input flag, reads one byte from the input handle with the dos library, and clears the flag again unless the byte is nonzero and below 123.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:INSTALL.EXE

- `setup_display_and_menu_loop`: Program entry point: it saves the library and interrupt state, writes display, sprite and DMA control registers, clears a block of variables and waits briefly, then reads key presses in a loop until Return ends the menu or Escape jumps to shutdown, after which the saved state is restored.
- `init_display_chips_for_menu`: Runs the same chip setup and key-driven menu loop as the program entry, reached through a handler pointer; the dos and exec library calls named in the evidence are made through the saved library base.
- `copy_entry_into_slot`: When the entry counter is below one, copies a fixed block of 48 bytes from a table into the slot selected by the current entry index and then advances that index.
- `scan_entries_restore_registers`: Walks the stored entries calling a check routine on each until one reports a match; if none matches it returns, otherwise it writes back saved display and DMA control values with the enable bit set and jumps through a stored routine pointer.
- `execute_indirect_call`: Performs an indirect call through a pointer and updates several global variables.
- `lock_caesar_dir_submit_request`: Carries the same slot submission body as the preceding routine, while the evidence also lists a directory lock and a change of current directory that refer to the caesar directory on the hard disk.
- `copy_slot_three_words`: Calls a helper, then copies three consecutive stored values from the slot address held in a global variable into three other globals, and calls several more helpers.
- `create_caesar_directory`: Creates the caesar directory on the hard disk through the dos library, keeps the returned lock, and releases that lock again.
- `handle_text_entry_key_3`: Appends the pressed character to a fixed-length name buffer and advances the cursor, holding it at the buffer end; on the backspace code it clears the previous position and steps the cursor back, never below the start, and returns the character read.
- `record_text_key_press`: Applies the same text-entry handling as the neighbouring routine, storing the pressed character or handling backspace, without returning the character.
- `apply_default_parameter_values`: Stores five default numeric settings, sixty, seventy, two hundred, eight and four, into global variables and then calls a helper to apply them.
- `run_eight_helper_sequence`: Calls eight helper routines in order with no logic of its own, so it is a fixed sequence of initialisation calls.
- `call_eight_routines_in_order`: Calls eight helper routines one after another and returns, with no data handling of its own.
- `run_seven_helper_sequence`: Calls seven helper routines in order and returns, with no logic of its own in the evidence.
- `call_seven_routines_in_order`: Calls seven helper routines one after another and returns, with no data handling of its own.
- `run_eight_helper_steps`: Calls eight helper routines one after another and returns, with no logic of its own.
- `wait_for_return_key`: Repeatedly calls a helper until a key-press flag is set, then keeps waiting until the key code equals the Return code, and returns.
- `call_two_routines_in_order`: Calls two helper routines one after the other and returns, with no data handling of its own.
- `call_two_routines_pair`: Calls two helper routines in sequence and returns, with no data handling of its own.
- `call_two_routines_then_return`: Calls two helper routines and returns, with no logic of its own in the evidence.
- `clear_table_entries_2`: Clears 10000 consecutive 32-bit entries to zero through a pointer, then returns.
- `forward_to_single_helper`: Calls one helper routine and returns, with no logic of its own.
- `copy_memory_block_2`: Copies 10000 consecutive 32-bit words from a source pointer to a destination pointer, advancing both by one word per copy.
- `nop_padding_sequence_3`: This routine is empty and consists only of instructions that perform no operation.
- `reset_globals_and_zero_block`: Clears four global variables and then zeroes ten groups of three consecutive 32-bit words in a block of memory, then returns.
- `clear_memory_block`: Sets a series of consecutive memory locations to zero using a loop.
- `reset_cohort_state_2`: Clears four global variables and then zeroes a block of memory in RAM by iterating nine times and writing zeros to three consecutive 32-bit words per iteration.
- `open_library_store_base`: Calls a routine from the global function table that opens a library by name, and when the returned library base is nonzero stores it in a global variable; the evidence shows the dos library name referenced nearby.
- `validate_and_store_size`: Calls a first helper and, if it reports success, calls a second helper and stores its result in a global value unless that result is minus one, then makes a third call; a failed check sets the failure flag.
- `run_checked_sequence_once`: On its first call runs a one-time setup helper, then calls checks that must pass, runs three more helpers and returns the result of the last one; a failed check sets the failure flag.
- `open_dos_file_store_result`: Calls the dos library Open function through the library base and stores the returned value in a global variable.
- `indirect_table_call`: The routine calls another routine obtained from a global variable table.
- `open_dos_file_store_handle`: Calls the dos library Open function through the library base and stores the returned handle in a global variable.
- `call_indirect_via_table`: The routine calls another routine indirectly through a pointer obtained from a global variable.
- `close_dos_file_handle`: Calls the dos library Close function through the library base with the handle already in place, and returns.
- `lock_abc_path_store`: Locks the path named ABC: through the dos library, stores the returned lock in a global variable, and then calls a helper.
- `lock_df0_store_handle`: Locks the DF0: path through the dos library, stores the returned lock in a global variable, and then calls a helper.
- `iterate_directory_entries_display`: Examines a locked directory with the dos library, then calls the next-entry function repeatedly for up to 201 passes, writing two display color values on each pass and stopping early when no further entry is returned.
- `nop_padding_sequence_4`: This routine is empty and performs no operations.
- `scan_df0_directory_entries`: Locks the DF0: path, examines it, then calls the next-entry function up to 201 times with a helper call on each pass, and finally unlocks the directory.
- `set_failure_flag`: Sets the failure flag to one and returns.
- `mark_failure_and_return`: Sets the failure flag to one and returns, with no other effect.
- `run_first_time_setup`: On the first call runs a one-time setup helper and returns its result; later calls return without doing anything.
- `execute_once_callback`: Executes a routine once and returns its result by checking a global flag.
- `check_initialization_flag`: Checks a global byte and, if it is zero, sets it to one and calls another routine.
- `prevent_recursive_execution_3`: Checks a global byte to determine if the routine has already been executed.
- `open_library_and_store`: Opens a library by name and stores the resulting base address in a global variable.
- `check_vertical_blank_flag`: Reads the interrupt request register and, when the vertical blank bit is set, calls a helper; the evidence lists the interrupt and DMA registers touched around it.
- `initialize_offset_fields`: Sets an initialised flag, stores a base value into one field of a structure, then stores four more values spaced 8000 apart from that base into the following fields.
- `decode_key_code_handshake`: Runs a short handshake on the controller registers with a counted pause, then converts the saved raw key code into a table index, clears a status bit and stores a flag byte in that table entry.
- `update_bitplane_data`: Performs a series of bitwise operations and a loop to update a global variable and write a byte to a memory location calculated from a global variable.
- `update_bitfield_state`: Calculates an index based on a bitwise transformation of a global variable to access a table.
- `update_bitfield_and_flag`: Calculates an index based on a bit-shifted value from a global variable to access a table.
- `handle_keyboard_serial_byte`: Runs the keyboard handshake on the serial line and decodes the received byte into a table entry, touching the CIA control and serial registers along the way.
- `clear_key_table_entry`: Clears one byte of a table at the index passed in, clears the stored flag, clears the serial mode bit of the CIA control register and sets that register to one.
- `wait_for_flag_then_dispatch`: Spins until a flag variable becomes nonzero, then calls one of two helpers depending on whether a stored pointer equals the global address, and finally calls a third helper.
- `loop_forever_stub`: Spins in an empty infinite loop and never returns, so reaching it blocks execution.
- `loop_forever_unreached`: Empty infinite loop reached only as a gap in the code; it never returns and does no work.
- `set_global_pointers_2`: Stores the address of one shared variable into three pointer variables and returns.
- `set_global_pointers_3`: Assigns the address of a global variable to three different global variables.
- `run_nested_counter_loops`: Runs an outer loop of 15 passes, and on each pass an inner loop of 32 steps that stores the step counter in a shared variable and calls helpers, with one more helper call per outer pass.
- `run_three_helpers`: Calls three helper routines in order and returns, with no logic of its own.
- `store_three_helper_results`: Calls a helper three times, storing the value returned in a register after each call into a separate global variable.
- `return_from_subroutine_2`: The routine performs an immediate return to the calling function.
- `split_value_into_nibbles`: Splits a 12-bit value held in a global variable into its low, middle and high 4-bit fields and stores each field in its own global variable.
- `pack_fields_into_table_entry`: Combines the stored high, middle and low fields into a 16-bit value and writes it into a table at the index held in a global variable.
- `read_table_entry_into_global`: Reads a 16-bit value from a table at the index held in a global variable and stores it in a global variable.
- `sum_color_nibbles_table`: Walks 32 packed color words from a source pointer, sums the three 4-bit fields of each word into a stored value in a global table, and touches the color register.
- `decode_packed_color_table`: Walks 32 packed color words held in a global table, sums the three 4-bit fields of each, writes the result back in place, and touches the color register.
- `delay_loop_nested`: Busy-waits with a nested counting loop of about 2500 outer passes, each with two inner steps, and returns without a result.
- `call_helper_seven_times`: Calls one helper routine seven times in a row and returns.
- `call_helper_six_times`: Calls one helper routine six times in a row and returns.
- `poll_helper_until_zero`: Calls a helper repeatedly until it returns zero, then returns.
- `busy_wait_short_loop`: Counts a 16-bit counter down from 15000 to zero as a busy wait, then returns.
- `write_decimal_digits_buffer`: Converts a 16-bit value to decimal, writing one digit per pass backwards from the end of a buffer, and then calls two helpers to finish.
- `write_decimal_digits_to_buffer_3`: Writes the decimal digits of a 16-bit value into a buffer as ASCII characters, from the last position backwards, one digit per pass, and then calls two helpers.
- `write_decimal_digits_then_call`: Writes the decimal digits of a 16-bit value into a buffer backwards as ASCII characters, then calls one helper.
- `blank_leading_zero_digits`: Replaces leading zero characters in a digit buffer with spaces while the following character is also a digit, so the number prints without leading zeros.
- `return_from_subroutine_3`: The routine is empty and performs no operations before returning.
- `encode_digits_as_letters`: Writes the decimal digits of a 16-bit value into a buffer backwards as letters, where digit n becomes the uppercase letter A plus n and a zero digit becomes lowercase a, then calls a helper.
- `write_five_hex_digits`: Writes five hexadecimal digit characters from the low twenty bits of a value, looking each four-bit group up in a sixteen-symbol digit table, then calls two helpers.
- `write_split_decimal_digits_2`: Splits a value into a quotient and remainder by ten thousand, writes the four digits of the quotient with leading zeros printed, writes the remainder digits into the next buffer field, and calls a helper.
- `write_split_decimal_digits_alt`: Splits a value into a quotient and remainder by ten thousand, writes the quotient digits with leading zeros printed and the remainder digits after them, and then calls a helper.
- `encode_digits_as_letters_wide`: Writes the digits of a 32-bit value into a buffer as letters, with zero digits as lowercase a, working backwards through the buffer, then calls a helper.
- `encode_low_digit_letters`: Writes the low-order digits of a 32-bit value as letters, A plus digit with lowercase a for zero, backwards through a buffer, then calls a helper.
- `write_padded_digit_pair`: Writes the high digits of a 32-bit value into a buffer with zero characters for empty positions, then writes the remaining digits into the second field and calls a helper.
- `write_digits_second_field`: Writes the high digits of a 32-bit value into a second buffer field with zero characters for empty positions, then calls a helper.
- `process_record_list`: Copies four 16-bit fields from a record into shared variables, then walks a zero-terminated byte list, calling two helpers for each byte and advancing a shared position by six after each entry.
- `compute_row_offset_and_bit`: Computes a byte offset as the position word shifted right by three bits and made even, plus a row value times forty, and stores the low four bits of the position word in a separate field.
- `compute_row_offset_to_slot`: Computes a byte offset from the position word and a row value times forty, stores it in a shared word variable, and stores the low four bits of the position word in a separate field.
- `calculate_offset_and_index`: Calculates a value based on a global variable and another value, then stores the result in a global variable along with a masked version of the original value.
- `calculate_offset_and_mask`: Calculates a value based on a bitwise operation and a multiplication, then stores it in a global variable along with a masked byte.
- `compute_row_offset_and_call`: Computes a byte offset from the position word and a row value times forty, stores the low four bits of the position word separately, and returns after the calculation.
- `dispatch_flagged_entry`: Looks up a flag in a table indexed by a character code minus thirty-two, and when the flag is set stores a value and calls one of two helpers depending on a stored mode value.
- `apply_mode_masked_bitplane_rows`: Reads five source bytes, shifts each by a stored amount, and merges them into bitplane rows: it clears the masked bits in the destination and ORs the shifted pattern into the planes selected by a stored mode value, advancing one row per byte.
- `apply_mode_masked_plane_pair`: Reads five source bytes, shifts each by a stored amount, and applies the same clear-then-set masking to two destination tables at once, with the bit pattern chosen by a stored mode value, advancing both destinations one row per byte.
- `draw_cell_grid_region`: Walks a rectangular range of cell columns and rows held in shared variables, storing the current column and row and calling the cell routine for each cell.
- `draw_cell_bit_planes`: Computes the word address of a cell from its column and row, where the row stride is forty bytes, builds a single-bit mask for the column within that word, clears that bit in five bitplanes and sets it again in each plane selected by a stored value.
- `draw_cell_bit_planes_alt`: Computes the word address of a cell from its column and row, builds a single-bit mask for the column within that word, clears that bit in five bitplanes and sets it again in each plane selected by a stored value.
- `modify_bitplane_data_2`: Performs bitwise AND and OR operations on several memory locations at fixed offsets from a calculated pointer based on a bitmask.
- `apply_bitmask_to_buffer_segments`: Calculates a pointer to a memory buffer based on several global values and applies a bitmask to five specific segments of that buffer.
- `unpack_packed_picture_stream`: Records a pointer 32000 bytes past the output start, reads a big-endian count and decodes a packed picture stream: a control byte with top bits 00 repeats the byte that follows, 40 and 80 repeat one of two stored bytes, and C0 copies literal bytes, with the run length in the low six bits plus one.
- `unpack_packed_picture_stream_alt`: Decodes a packed picture stream into an output buffer with the same control-byte scheme as its neighbour: a 00 control repeats the following byte, 40 and 80 repeat one of two stored bytes, and C0 copies literal bytes, each run being its low six bits plus one.
- `unpack_packed_picture_or`: Decodes the same packed picture stream as its neighbour but ORs each decoded byte into the output buffer instead of overwriting it.
- `decode_packed_runs_or_fill`: Fills an initial run into the output buffer by ORing a stored byte, then decodes the remaining packed control bytes, ORing each run into the output.
- `unpack_picture_or`: Decodes a packed data stream and performs a bitwise OR of the decoded bytes into a destination buffer.
- `or_packed_pixel_stream`: Decodes a compressed bitstream and applies the resulting bytes to a destination buffer using bitwise OR operations.
- `return_stack_value`: This routine is empty and simply returns the value currently residing at the top of the stack.
- `decode_packed_stream_skip_zero`: Decodes the packed picture stream into the output buffer, writing a decoded byte only when it is nonzero, so zero bytes leave the existing contents untouched.
- `decode_packed_runs_skip_zero`: Fills an initial run into the output buffer, writing only nonzero bytes, then decodes the remaining packed control bytes with the same nonzero-only writes.
- `unpack_picture_skip_zero`: Decodes a packed data stream and writes decoded bytes to a destination buffer only if they are non-zero.
- `expand_run_length_encoded_stream`: Decodes a custom run-length encoded data stream by reading control bits to determine repetition counts and byte values.
- `discard_null_byte`: This routine is empty.
- `open_raw_console_window`: Opens a raw console window described by a text specification through the dos library and stores the returned handle in a global variable.
- `read_input_byte`: Calls a routine to read a byte and, if the byte is within a specific range, updates a global variable and sets a flag.
- `read_input_byte_checked`: Sets the input flag, reads one byte with the dos library, and clears the flag again unless the byte is nonzero and below 123.
- `close_dos_file_handle_b`: Calls the dos library Close function through the library base with the handle already in place and returns; the evidence also lists the color register touched nearby.
- `drain_pending_input_bytes`: Repeatedly waits for input on a file handle, and while input is pending sets the input flag and reads a byte to discard it, stopping once no further input is waiting.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[a][HD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_graphics_copper_sequence`: Opens the graphics library, saves interrupt and DMA state, waits on the raster beam, clears sprite control registers and fills a table of sprite values, then runs sixty passes that wait for raster lines while stepping a counter and two pointers through fixed tables, and finally restores the saved interrupt and library state.
- `swap_buffer_pair_store_offset`: Calls four helpers, stores a value 350 bytes past the second saved buffer address into a shared pointer, then swaps the two saved buffer addresses.
- `dispatch_index_table_lookup`: Reads a stored index: when it is below ten it stores the index minus one and calls a helper; when it is above sixteen it looks up four bytes in a table at a position derived from the index and stores them in shared variables.
- `swap_buffer_pair_back_offset`: Calls two helpers, stores a value two bytes before the second saved buffer address into a shared pointer, then swaps the two saved buffer addresses.
- `set_table_pointers`: Stores a pointer to a table and a second pointer calculated from the first word of that table.
- `store_scaled_position_pair`: Calls a helper, stores the incoming position shifted right by three bits into one shared variable, calls another helper, then stores the position shifted right by six bits into a second shared variable.
- `combine_and_mask_coordinate_pair`: Adds each of two word pairs to its partner and masks both results to clear the low bit and the top four bits, keeping the values even and within a limited range.
- `start_blitter_full_mask_blit`: Calls a helper first, then sets full first and last word masks, a destination-only control word, a zero modulo, the destination address passed in, and a fixed blit size, and starts the blit.
- `configure_blitter_registers`: Writes values to several blitter control and size registers and waits for the blitter to become idle.
- `vertical_blank_state_machine`: On each vertical blank interrupt, calls a helper to read a value and advances a small state machine: a waiting state counts down into a running state, the running state calls two helpers and counts frames, and after 110 frames it moves to a final state; the vertical blank request is acknowledged each time.
- `start_blitter_short_blit`: Calls a helper first, then sets full first and last word masks, a destination-only control word, a zero modulo, the destination address passed in, and a shorter fixed blit size, and starts the blit.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_blitter_idle`: Spins while the blitter busy flag in the DMA control status register is set, then returns.
- `reset_text_stream_state`: Resets a text stream: points the read position at a fixed start address, zeroes the column counter, sets the line counter to one and clears a byte in shared memory.
- `process_text_stream_character`: Reads the next character of a text stream: a zero byte skips padding, a one byte restarts the stream, a carriage return starts a new line, and any other character is passed to a helper and advances the column, starting a new line after 41 columns.
- `step_text_stream_character`: Advances the column counter and starts a new line at 41 columns, then reads the character at the stream position: a zero byte skips padding, a one byte restarts the stream, a carriage return starts a new line, and other characters are passed to a helper.
- `begin_text_line_character`: Starts a new text line by resetting the column counter and advancing the line counter, then handles the character at the stream position: a zero byte skips padding, a one byte restarts the stream, a carriage return starts a new line, and other characters go to a helper.
- `setup_blitter_aligned_copy`: Calls a helper, sets blitter source masks and modulo, and chooses the blitter control word and size depending on whether two address values are odd or even, offsetting the source and destination by fixed amounts in the odd case.
- `build_copper_list_first_half`: Writes two fixed copper instruction words at the start of a copper list in shared memory, then copies 64 word pairs from a source table into the list, advancing six words per source entry.
- `build_copper_list_second_half`: Calls a helper, then writes two fixed copper instruction words into a second copper list and copies 64 word pairs from the source table into it, advancing six words per source entry.
- `build_copper_list_from_pointer`: Writes two fixed copper instruction words at the destination pointer, then copies 64 word pairs from the source table into the list, advancing six words per source entry.
- `set_copper_list_pointers`: Stores the high and low words of four list addresses, each a shared base address plus a fixed offset, into shared pointer variables, and sets a control constant.
- `encode_sprite_position_words`: Encodes a stored position into the words of two sprite control entries: the start position is halved, a fixed offset is added to the stop position, and a bit field derived from a second stored value is merged in.
- `set_table_and_end_pointers`: Stores a pointer to a table and a second pointer computed from the table's first word times four plus two.
- `toggle_frame_parity`: Calls a helper, then flips the lowest bit of a shared flag so that it alternates between zero and one.
- `store_arguments_and_run_helpers`: Saves the incoming address and two register values into shared variables, then calls three helpers in order.
- `test_memory_bit`: Polls a specific bit within a byte at an offset from a pointer until that bit is cleared.
- `setup_blitter_rect_copy`: Reads two corner coordinate pairs from shared storage, computes the blit width in words and the height in rows from their differences, sets a full-mask destination-only blit with a modulo from the width, and starts it using an address supplied by a helper.
- `start_blitter_masked_rect_copy`: Computes the blit width and height from two stored corner coordinate pairs, sets a full-mask blit with a combined source and destination control word and modulo values, stores the destination address and starts the blit.
- `project_points_scaled_to_screen`: Walks a table of points, scales each coordinate pair by a stored factor shifted right by ten bits, adds fixed centre offsets of 160 and 128, stores the results and calls a helper for each point.
- `process_indexed_point_pairs`: Walks a list of index pairs, looks up two words for each index in a table, and passes them with the paired value to a helper, once per pair.
- `reset_default_clip_rect`: Stores four default bounds values, 160, 160, 128 and 128, into a shared rectangle record.
- `expand_bounds_to_include_point`: Widens a shared bounding rectangle so that it includes the given point, lowering the minimum and raising the maximum for both coordinates.
- `setup_blitter_line_octant`: Sets up the blitter for a line between two points: computes the start address from the first point, the step counts from the coordinate differences, a line-octant code from their signs and comparisons, and the control word, then stores them for the blit.
- `build_slot_pointer_table`: Scans a sorted table for its largest entry, then builds 30 slot pointers spaced by a fixed row size, clearing each slot's block and storing its address in a pointer table, then clears playback state bytes and sets a status flag.
- `advance_music_tick`: Advances a music tick counter; on each tick it either reloads the audio channel registers from stored sample pointers and lengths after two short delays, or handles a pending effect, then updates the playback position and volume state.
- `silence_audio_channels`: Writes zero to four channel volume registers and then sets the channel volume to the maximum value of fifteen.
- `run_channel_effect_tick`: Runs one tick of an effect on an audio channel, selected by the effect nibble of its stored command: pitch slides, a vibrato-like change, a volume slide, a position change and extended commands; it writes the resulting period to the channel's period register and returns it.
- `load_channel_pattern_row`: Loads the next pattern row for an audio channel when its row counter is empty, copies the sample address, length, period and volume from the instrument table, then processes the row's effect command and updates the channel's audio registers.
- `run_channel_tick_effects`: Applies the per-tick effect of an audio channel from the effect nibble stored in its row: a slide or note change, a volume slide, a retrigger or a delay, then returns the new period.
- `copy_channel_period_word`: Copies a 16-bit period field from the channel record into the audio period slot passed in.
- `set_channel_target_period`: Looks up the period-table entry at or below the channel's current period for its instrument, lowers it one step when the channel's flag is set, stores it as the target period and sets a slide-direction flag by comparing it with the current period.
- `slide_channel_period_to_target`: Moves a channel's current period toward its target by a fixed step each tick, up or down depending on a direction flag, stops at the target, then looks up the nearest period and writes it to the audio period slot.
- `apply_channel_vibrato_offset`: Computes a vibrato-style period offset from a waveform table value scaled by the channel's depth, adds it to the channel's base period, and advances the channel's phase.
- `dispatch_channel_effect_command`: Dispatches the effect command for an audio channel from its command nibble: it sets speed, handles volume, retrigger and delay commands, writes volume changes into the channel record, and returns the value to be applied.
- `store_channel_low_nibble`: Stores the low four bits of a byte from the channel record into a control field of the same record.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
