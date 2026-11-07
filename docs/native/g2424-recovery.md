# Game 2424 (caesar deluxe (1993)(impressions)[cr atx][a][fd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02424). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1809648 top-level bytes in 108 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 28 of 28 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 331 of 331 named (1 author symbols), 331 of 331 described (13 by rule, 126 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1333 of 1333 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6848 of 6884 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 526849 |
| code | 25646 |
| data | 298605 |
| padding | 32340 |
| unused | 926208 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:installto | 2 | 117 | 2 |
| ram-handler | 2 | 10 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:assign | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:echo | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:Anthrox | 6 | 251 | 6 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:cd | 2 | 117 | 2 |
| installto | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| install.exe | 3 | 1394 | 112 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:INSTALL.EXE | 3 | 1493 | 118 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:trainer | 3 | 176 | 8 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:play | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:assign | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:echo | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:lab | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:skip | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:csr.exe | 8 | 27 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:failat | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:Anthrox | 6 | 45 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:cd | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:endif | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:caesar.boot | 1 | 31 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:if | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:bootblock | 1 | 10 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1670 | 47 |

## Cases handled

- runtime segment an address: 1085 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- caesar deluxe (1993)(impressions)[cr atx][a](disk 2 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:csr.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:trainer

## Copy protection found

- `initialise_custom_chip_state` (install.exe): Writes display window, bitplane modulo, DMA and interrupt control registers to set up the custom chips, saves and restores the previous interrupt and sprite state through library calls, and calls dos.Exit on the exit path. Kind: loader/protection sequence. Pass path: u10__l0_020C.
- `setup_display_and_exit` (INSTALL.EXE): Programs the display window, bitplane modulo, DMA, interrupt and sprite registers after saving the system state, clears shared variables and runs a timed delay. Kind: disk/track check loop. Pass path: the branch where _u11__d0_24F6 == 0xd is taken (breaking the loop to proceed with installation).

## Functions


### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:installto

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the destination pointer is non-null, copies sixteen 32-bit words from a block at a fixed offset into it.

### ram-handler

- `call_global_then_unrecovered_jump`: Calls a function pointer read from a global variable at offset -624, then jumps through a pointer whose target is not recovered by the decompiler.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:assign

- `prepare_sized_table_block`: Finds the largest value in a linked list, uses it to size a table and fills the table with an increasing sequence of 32-bit words.
- `copy_memory_block_6`: Transfers a fixed number of words from a source offset to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:echo

- `find_max_and_update_list`: Traverses a linked list to identify the maximum value in a specific field and then updates a header structure.
- `copy_memory_block_8`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:Anthrox

- `cleanup_cached_block_entry`: Calls an unnamed routine, then makes four indirect library calls in a loop.
- `decrement_word_until_minus_one`: Decrements a 16-bit local counter until it reaches minus one, then returns.
- `clear_cache_after_bit_pass`: Runs two passes that shift bits from an input stream and add the resulting values into a buffer through unnamed routines.
- `decode_backward_bitstream`: Decodes a bit-packed stream by reading bits from a stored byte and copying run-length groups backwards from the input into the output buffer.
- `decode_backward_bitstream_run`: A second variant of the backward bit-stream decoder that uses the same backward copy loop and bit-reader state, with its run lengths read through unnamed routines.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:cd

- `update_linked_list_and_copy_data_6`: Traverses a linked list to find the maximum value in a specific field and then updates a structure with this value.
- `copy_memory_block_and_call_2`: Copies sixteen bytes from a memory location to a destination pointed to by a stack variable using a loop.

### installto

- `traverse_list_and_update_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a header and data block.
- `copy_memory_block_and_call_3`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then executes a function pointer.

### assign

- `find_max_and_populate_buffer_2`: Traverses a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_data_block_and_call`: Copies sixteen bytes of data from a memory location to a destination pointed to by a stack pointer, then calls another routine.

### echo

- `update_max_and_sync_buffer`: Traverses a linked list to find the maximum value in a specific field and writes it to a destination structure.
- `copy_data_block`: Copies sixteen bytes of data from a memory location to a destination pointed to by a stack pointer.

### cd

- `traverse_list_and_update_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then updates a header and performs a series of memory copies between a source block and a destination buffer based on the calculated maximum.
- `copy_stack_data`: Copies sixteen words from a source pointer to a destination pointer on the stack.

### install.exe

- `run_installer_entry_wait_key`: Program entry for the installer: it loads display and DMA control registers, clears a block of state, runs a timed delay, then waits for a keypress.
- `initialise_custom_chip_state`: Writes display window, bitplane modulo, DMA and interrupt control registers to set up the custom chips, saves and restores the previous interrupt and sprite state through library calls, and calls dos.Exit on the exit path.
- `append_fixed_table_record`: Copies a 48-byte record from a fixed data block into the global table at the current index and increments the entry count, but only when a stored value is below one.
- `process_first_flagged_entry`: Walks the table of entries from the start, calls an unnamed routine on each to check its state, and stops at the first one whose check returns one.
- `execute_indirect_call`: Calls a routine through a pointer stored at an offset from another pointer.
- `lock_install_directory`: Calls the DOS routines to lock the install directory and make it the current directory, then runs the same slot transfer step.
- `create_caesar_directory`: Creates the caesar directory through the DOS library and then releases the lock that it returned.
- `edit_text_buffer_on_key`: When the key code is not 8 (backspace), it appends the code to a 30-slot text buffer and advances the position, capped at 30.
- `handle_text_entry_key_3`: Stores a value in a buffer and increments an index, or clears the previous entry and decrements the index if a specific value is provided.
- `set_default_layout_values`: Stores the values 60, 70, 8 and 4 into four stored fields and sets a value of 200, then calls an unnamed routine.
- `run_seven_routine_sequence`: Calls seven unnamed routines in sequence and returns, with no other operations.
- `run_eight_routine_sequence`: Calls eight unnamed routines in sequence and returns, with no other operations.
- `run_seven_setup_steps`: Calls seven routines in a fixed order with no logic of its own.
- `execute_sequence_of_routines`: Calls a series of seven routines in a fixed order before returning.
- `wait_for_return_key`: Loops calling an unnamed routine until a key event is flagged and the key code equals 13 (return), then returns.
- `run_two_routine_sequence`: Calls two unnamed routines in sequence and returns, with no other operations.
- `execute_empty_routine`: This routine is empty and performs no operations before returning.
- `clear_ten_thousand_longs_2`: Writes zero to 10000 consecutive 32-bit words starting at the address held in a register.
- `clear_ten_thousand_longs`: Writes zero to 10000 consecutive 32-bit words starting at the address held in a register, counting down from 9999 to minus one, then returns.
- `forward_single_unnamed_call`: Calls one unnamed routine and returns, with no other operations.
- `copy_ten_thousand_longs`: Copies 10000 consecutive 32-bit words from one buffer to another by storing each source word and advancing both pointers.
- `stub_return_only`: It is a stub, most likely decoded from a gap between routines rather than real logic.
- `reset_global_state_and_table`: Zeroes four global fields, then zeroes ten groups of three consecutive 32-bit words starting at a fixed address.
- `clear_memory_block`: Sets several global variables to zero and then clears a block of memory using a loop.
- `reset_cohort_state`: The routine clears a set of global variables and then zeroes a block of memory in RAM by iterating nine times and writing zeros to three consecutive words per iteration.
- `open_dos_library_and_store`: Calls an open-library function pointer from a global table and, when it returns a nonzero base, stores that base in a global variable.
- `open_and_read_file_value`: Calls the DOS open wrapper and, on success, the read wrapper.
- `write_block_with_status`: On first use sets a flag and runs an initialisation routine.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `invoke_table_callback_store`: Calls a routine taken from a global table at an offset and stores its return value in a global variable.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `lock_path_store_handle`: Calls the DOS lock function from a global table, stores the lock in a global variable, then calls another routine.
- `scan_directory_entries`: Examines a directory through the DOS wrappers, then takes up to 201 entries in turn and calls an unnamed routine on each until none remain.
- `set_error_flag_and_return`: Sets the error flag to one and returns its stack argument unchanged.
- `mark_failure_and_return`: Sets the failure flag to one and returns, with no other effect.
- `run_once_then_return_byte`: On its first call sets a first-use flag and calls an unnamed routine, returning the byte it produces.
- `execute_once_conditional`: Checks a global flag and, if not set, sets it and calls another routine.
- `prevent_recursive_execution`: Checks a global flag to determine if the routine has already been entered.
- `prevent_recursive_execution_2`: Checks a global flag to determine if the routine has already been executed.
- `initialize_dos_library`: Checks a global flag and, if not set, calls a library function to open a library and stores the resulting base address in a global variable.
- `poll_interrupt_request_bit`: Writes 160 to a custom-chip register, then tests bit 32 of the interrupt request register and calls an unnamed routine when it is set.
- `initialize_offset_table_bases`: Stores a base value read from a global variable into a flag and into the first of five consecutive fields in a structure.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d7.
- `shift_serial_byte_into_table`: Saves a byte from a global register, writes control values to a hardware register and waits through a 1024-step loop.
- `update_custom_state`: Calculates an index based on a global byte, then writes a value from a table to a memory location and sets a byte to either zero or one based on a bit test.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_table_entry`: Calculates an index based on a global value, retrieves a value from a table, and writes a byte to a memory location based on a bit test.
- `clear_table_slot_and_flag`: Clears a flag variable, zeroes one byte of a table at an indexed offset, clears bit 64 in a global and sets that global to one, then returns.
- `wait_flag_then_run_sequence`: Busy-waits until a global value is nonzero, then calls an unnamed routine.
- `infinite_loop_stub`: The decompiled body is an infinite loop with no operations, so the routine never returns.
- `infinite_loop_stub_4`: An infinite loop that never returns.
- `set_global_pointers`: Assigns the address of the global block to three global pointer variables and returns.
- `set_global_pointers_3`: Assigns the address of a global variable to three other global pointer variables, then returns.
- `nested_loop_calls`: Runs an outer loop of 15 steps and, inside each, an inner loop of 32 steps that stores the inner counter in a global and calls two routines per step.
- `run_three_routine_sequence`: Calls three unnamed routines in sequence and returns.
- `capture_three_results`: Calls three unnamed routines in sequence and stores the 16-bit value each returns in its own global variable.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `split_value_into_nibbles`: Splits the low 12 bits of a global value into three 4-bit fields, storing the low and middle nibbles in two globals and the high nibble in a third, then returns.
- `store_indexed_table_entry`: Stores a 16-bit value built from two global nibble fields and a routine address into a table at an index taken from a global variable, then returns.
- `load_indexed_table_entry`: Reads a 16-bit value from a table at an index derived from a global variable and stores it into another global variable, then returns.
- `sum_palette_nibble_fields`: Calls an unnamed routine, then for 32 colour words stores into a global table the sum of each word's three 4-bit fields, masked with 3840, 240 and 15.
- `decode_packed_color_entries`: Calls another routine and then iterates through a 32-entry table, summing the three 4-bit components of each word and storing the result back in the table.
- `busy_wait_nested_loops`: Runs a nested counting loop of 2500 outer iterations, each with a short inner countdown, to waste time, then returns without calling anything.
- `call_single_routine_seven_times`: Calls one unnamed routine seven times in succession and returns.
- `call_routine_six_times`: Calls another routine six times in a counted loop, counting down from five to minus one.
- `call_routine_until_zero`: Repeatedly calls another routine and keeps looping while the 16-bit value that call returns is nonzero, then returns.
- `busy_wait_countdown`: Counts a 16-bit counter down from 15000 to minus one as a pure delay loop that touches no hardware or memory, then returns.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d1.
- `write_decimal_digits_to_buffer`: Writes the decimal digits of a 16-bit value into a buffer as ASCII characters, least significant digit first, filling backwards from a start offset for a fixed number of positions.
- `extract_digits_to_buffer`: Writes the decimal digits of a 16-bit value as ASCII characters into a byte buffer at a fixed width, filling backwards from the least significant digit.
- `blank_leading_zeros_in_buffer`: Walks a character buffer from its start and replaces each leading zero with a space, stopping at the first character that is not a zero followed by a digit, so the final digit is kept.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `write_letter_encoded_digits`: Writes a 16-bit value into a buffer as decimal digits encoded as capital letters, A for zero through J for nine, filling backwards.
- `write_nibble_digits_from_table`: Writes the low twenty bits of a value into a buffer as five characters, each picked from a 16-character digit table by its four-bit group, filling backwards.
- `write_eight_digit_split`: Writes a 32-bit value as eight zero-padded decimal digits in two groups of four, splitting it at ten thousand, and stores the packed split in a shared variable.
- `format_decimal_digits`: Splits a 32-bit value into two 16-bit parts and writes their decimal digits as characters into a buffer at specific offsets.
- `write_letter_digits_high_word`: Writes the decimal digits of the upper sixteen bits of a 32-bit value as capital letters, A for zero through J for nine, filling unused positions with a lowercase a.
- `write_letter_digits_compact`: Fills a buffer backwards with capital-letter digits, A for zero through J for nine, taken from the upper sixteen bits of a value, using a lowercase a as filler.
- `write_padded_digits_then_stored`: Writes the upper sixteen bits of a value as decimal digits padded with zeros, then writes four zero-padded digits of a shared stored value into the buffer.
- `write_upper_digits_second_field`: Writes the upper sixteen bits of a value as zero-padded decimal digits into the second field of a buffer, offset twelve, filling backwards.
- `process_zero_terminated_entry_list`: Copies four words from a parameter block into shared variables, then loops over a zero-terminated byte list, storing each byte in a shared variable, calling two routines for it, and advancing a position by six per entry.
- `compute_row_offset_and_nibble`: Computes an offset from a shared position value by shifting, masking and adding forty times a second shared value, storing the result in a shared variable, and stores the low four bits of the position in another shared variable.
- `store_row_offset_and_nibble`: Computes the same row offset from the shared position and row values, stores it in a fixed RAM word, and stores the low four bits of the position in a shared variable.
- `calculate_bitplane_offset`: Calculates a value by shifting and masking a register and adding a product of two other values, then stores the result and a remainder in global variables.
- `calculate_bitplane_offset_2`: Performs a calculation involving bit shifts and multiplications to update two global variables.
- `compute_row_offset_and_call`: Computes a byte offset from the position word and a row value times forty, stores the low four bits of the position word separately, and returns after the calculation.
- `store_value_if_table_flag`: Checks a flag in a lookup table indexed by a shared code value.
- `apply_shifted_bytes_to_bitplanes`: Takes five source bytes, shifts each into place by a shared bit offset, and clears and then sets bits in five stride-spaced bitplane words per row.
- `apply_mask_to_two_planes`: For five source bytes, shifts each by a shared bit offset and clears and sets bits in two sets of plane words at once, with the planes chosen by a shared mode value.
- `iterate_rectangle_cells`: Walks a rectangle of cells with nested row and column loops, setting shared column and row indexes for each cell and calling another routine once per cell.
- `write_pixel_to_five_planes`: Clears one pixel bit at a column and row in five bitplanes of a bitmap with forty bytes per row, then sets that bit again in each plane whose bit is present in a shared plane mask.
- `modify_bitplane_data`: Calculates a pointer to a table and uses a bitmask to clear or set specific bits across five different memory locations based on a global flag.
- `modify_bitplane_data_2`: Uses a calculated pointer and a bitmask to perform bitwise AND and OR operations on several memory locations.
- `modify_bitplane_data_3`: Calculates a memory address and applies a bitmask to clear or set bits in multiple words based on the state of a global variable.
- `decode_packed_run_stream`: Decodes a packed stream of byte runs and literal bytes from a source block into a destination buffer, stopping once the byte count in the block header has been produced.
- `decode_run_length_buffer`: Decodes a run-length encoded byte stream from a source buffer into a destination buffer, repeating fill bytes and copying literal bytes until the header's length is reached.
- `or_decode_packed_runs`: Decodes the same packed runs and literals as its plain sibling, but combines each output byte into the destination with a bitwise OR so decoded data merges over existing contents.
- `or_decode_runs_to_limit`: Decodes packed runs from a stream and ORs each run's byte into a destination until a shared counter reaches its limit, then returns.
- `or_decode_runs_with_fill`: Decodes packed runs and literals into a destination with a bitwise OR, using a shared fill value for one record type, and returns when the shared counter reaches its limit.
- `or_decode_packed_runs_nested`: Decodes packed runs and literals into a destination with a bitwise OR, handling one record type in a deeper nested loop, and returns when a shared counter reaches its limit.
- `stub_return_immediately`: A stub that only returns to its caller, with no work and no side effects.
- `decode_packed_runs_skip_zeros`: Decodes packed runs and literals into a destination while writing only nonzero bytes, so zero values leave existing destination data unchanged, and returns when a shared counter reaches its limit.
- `decode_runs_skip_zeros_fill`: Decodes packed runs and literals into a destination, writing only nonzero bytes, with the fill value for one run type taken from a shared variable.
- `decode_runs_skip_zeros_nested`: Decodes packed runs and literals into a destination while writing only nonzero bytes, handling one record type in a deeper nested loop.
- `decode_packed_runs`: Decodes a stream of packed run-length encoded data into a destination buffer, skipping zero values to preserve existing data.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `open_dos_file_handle`: Calls the DOS library's open routine and stores the returned file handle in a shared variable.
- `read_char_if_available`: Checks whether input is waiting on the operating system's file input.
- `read_input_byte_validated`: Reads one byte from the operating system's input into a shared variable and sets a ready flag, clearing the flag again when the byte is zero or 123 or above.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:INSTALL.EXE

- `setup_display_and_key_loop`: Saves the system state, programs display window, bitplane modulo, DMA, interrupt and sprite registers, clears a block of shared variables and runs a timed delay.
- `setup_display_and_exit`: Programs the display window, bitplane modulo, DMA, interrupt and sprite registers after saving the system state, clears shared variables and runs a timed delay.
- `append_record_to_table`: When a shared check value is not positive, copies a fixed block of forty-eight bytes into a shared table at the slot given by a shared counter, then advances that counter.
- `scan_records_then_finish`: Scans shared records one slot at a time, calling a routine for each until it reports a match.
- `retry_directory_lookup_with_lock`: Stores the current directory lock from the DOS library, points a shared record slot at the record table and calls a routine.
- `lock_directory_and_search`: Uses the DOS library to obtain the current directory and lock a volume path, then calls a routine to look up a record.
- `copy_record_header_words`: Calls a routine, then copies the first three words of a record slot into shared storage and calls four more routines.
- `create_directory_and_unlock`: Creates a directory for the path dh0:caesar through the DOS library, stores the returned lock in a shared variable, then releases a lock through the DOS library.
- `handle_text_entry_key`: Handles one key of a text-entry field.
- `handle_text_entry_key_2`: Handles one key of a text-entry field.
- `set_initial_config_values`: Writes five fixed constants, sixty, seventy, two hundred, eight and four, into shared variables, then calls one routine.
- `call_eight_routines_in_order`: Calls eight other routines in order with no other work.
- `run_eight_setup_steps`: Calls eight routines in a fixed order with no logic of its own.
- `call_seven_routines_in_order`: Calls seven other routines in order with no other work.
- `execute_seven_setup_routines`: Calls seven different routines in a fixed, sequential order before returning.
- `run_eight_helper_sequence`: Calls eight helper routines in order with no logic of its own, serving as a fixed sequence of initialisation calls.
- `wait_for_return_key_2`: Loops until a key-ready flag is set and the key value is the Return key, then returns.
- `call_two_routines`: Calls two other routines in order and returns.
- `call_other_routines`: Calls two other routines sequentially.
- `clear_table_entries_2`: Writes zero to ten thousand consecutive 32-bit entries starting at a pointer.
- `clear_table_entries`: Writes zero to ten thousand consecutive 32-bit entries starting at a pointer, then returns.
- `call_single_routine`: Calls one other routine and returns.
- `copy_memory_block`: Copies ten thousand consecutive 32-bit words from a source pointer to a destination pointer, then returns.
- `return_immediately`: An empty stub consisting of four instructions that performs no operations before returning to the caller.
- `clear_shared_state_block`: Zeroes three shared variables and ten triples of 32-bit slots in a table, then returns.
- `clear_memory_block_2`: Sets several global variables to zero and then executes a loop to zero out a specific block of memory using a stride of twelve bytes per iteration.
- `reset_shared_state_block`: Clears several global variables and then zeroes a block of memory in RAM by iterating nine times and writing zeros to three consecutive words per iteration.
- `open_library_and_store_base`: Calls the operating system's library-opening routine through a shared table entry, for the DOS library, and stores the returned library base in a shared variable when the call succeeds.
- `check_store_result_or_fail`: Calls a routine and, if it succeeds, calls a second one.
- `run_checked_sequence_return_result`: Runs a one-time setup step when a shared flag is clear, then calls routines in sequence.
- `open_file_via_table_store`: Calls the operating system's open routine through a shared table entry and stores the returned handle in a shared variable.
- `read_via_table_entry`: Calls the operating system's read routine through a shared table entry and returns.
- `call_table_entry_store_result`: Calls a DOS library routine selected from the global function table and stores its return value in a global variable.
- `write_via_table_entry`: Calls the operating system's write routine through a shared table entry and returns.
- `close_via_table_entry`: Calls the operating system's close routine through a shared table entry and returns.
- `lock_named_volume_and_store`: Obtains a lock through the DOS library for the volume path ABC:, stores the lock in a shared variable and calls one routine.
- `lock_floppy_path_and_store`: Obtains a lock through the DOS library for the floppy drive path DF0:, stores the lock in a shared variable and calls one routine.
- `step_directory_entries_color_flash`: Examines the first directory entry through the DOS library, then steps through up to 201 entries.
- `nop_padding`: This routine is empty and performs no operations.
- `scan_directory_entries_then_unlock`: Locks the floppy path DF0: through the DOS library, examines up to 201 directory entries and calls a routine for each, then releases the lock and calls one routine.
- `set_error_flag_and_return_2`: Sets a shared flag to one and returns with the caller's stack value unchanged.
- `mark_failure_and_return_2`: Sets a global variable to one and returns.
- `run_once_and_return_result`: Calls a routine on first use only, marks a shared flag so later calls skip it, and returns the value from that first run.
- `set_flag_and_return`: Checks a global variable, sets it to one if it was zero, and returns a value.
- `prevent_recursive_execution_3`: Checks a global flag to determine if the routine has already been executed.
- `check_initialization_flag`: Checks a global byte and, if it is zero, sets it to one and calls another routine.
- `open_dos_library`: Checks a global byte and, if it is zero, sets it to one and calls a routine to open the DOS library.
- `call_if_vblank_interrupt_enabled`: Reads the interrupt-enable register and, if the vertical blank interrupt bit is set, calls a routine.
- `store_five_bitplane_pointers`: Stores a base address in a shared variable and writes five bitplane pointers into a structure, each one 8000 bytes after the previous, then sets a shared flag.
- `decode_keyboard_byte_state`: Handles a raw keyboard byte.
- `update_custom_hardware_state`: Performs a loop for a fixed number of iterations and modifies several global variables and memory locations based on a bitwise transformation of a global byte.
- `initialize_hardware_parameter`: Executes a loop for a fixed number of iterations and updates global variables and memory based on the state of a global byte.
- `set_hardware_register_value`: Modifies a global byte and performs a loop to update specific memory locations based on bitwise operations.
- `reset_shared_state`: Clears several global variables, zeroes out a block of memory, and updates a table entry based on a bit-shifted value from a global variable.
- `clear_table_byte_slot`: Clears one byte in a table at a given index, clears a shared flag and sets a shared constant, then returns.
- `wait_for_flag_then_dispatch`: Spins until a shared value is nonzero, then calls one routine, calls one of two routines depending on whether a shared pointer equals the address of a shared variable, and finally calls another routine.
- `infinite_loop_stub_2`: An empty infinite loop that never returns, so it blocks execution.
- `infinite_loop_stub_3`: Contains only an infinite loop that never returns; the routines listed for it are not reached in the decompiled code.
- `set_global_pointers_2`: Sets three shared pointers to the address of the same shared variable, then returns.
- `set_global_pointers_4`: Assigns the address of a global variable to three other global pointer variables, then returns.
- `nested_loop_calls_2`: Runs a fifteen-by-thirty-two nested loop.
- `call_three_routines`: Calls three other routines in order and returns.
- `call_and_store_three_results`: Calls a routine, then stores the low word of the register it leaves into a shared variable, repeated for three calls in turn.
- `return_empty_routine`: The routine performs no operations and immediately returns to its caller.
- `extract_nibble_fields`: Splits a shared word into three four-bit fields, the lowest nibble, the next nibble and a third nibble, storing each in its own shared variable.
- `store_table_entry`: Computes a table index from a shared counter and stores a 16-bit value, built from two shared values and a routine's address, into that table slot.
- `load_table_entry`: Loads a 16-bit value from a table at the index given by a shared counter into a shared variable, then returns.
- `convert_color_words_to_table`: Calls one routine, then fills a shared table of thirty-two entries from a source pointer, each entry being the sum of three masked four-bit fields of its source word.
- `decode_color_table`: Calls one routine, then rewrites a shared table of thirty-two words in place, replacing each word with the sum of its three masked four-bit fields.
- `delay_loop`: Busy-waits through 2500 outer iterations, each with a short two-step inner countdown, then returns.
- `call_routine_seven_times`: Calls one routine seven times in a counted loop, then returns.
- `call_routine_six_times_2`: Calls another routine six times in a counted loop, counting down from five to minus one, then returns.
- `execute_delay_loop`: Repeatedly calls another routine in a loop until it returns zero.
- `delay_execution`: Runs a decrementing loop for fifteen thousand iterations.
- `convert_integer_to_string`: Converts a numeric value into a string of ASCII characters and stores them in a memory buffer.
- `write_decimal_digits_to_buffer_2`: Converts a value to decimal digits and writes them as ASCII characters into a buffer, last digit first.
- `write_decimal_digits_to_buffer_3`: Converts a value to decimal digits and writes them as ASCII characters into a buffer, last digit first.
- `replace_numeric_characters`: Iterates through a string and replaces specific characters with spaces if they match a certain numeric range.
- `return_from_procedure`: The routine is empty and performs no operations before returning to the caller.
- `convert_to_alphanumeric_text`: Converts a numeric value into a sequence of alphanumeric characters and stores them in a buffer.
- `write_nibbles_as_hex_text`: Writes the five low nibbles of a value into a buffer as characters from a sixteen-character digit table, then calls two routines.
- `format_split_numeric_string_2`: Splits a value into two parts and writes the decimal digits of each part into two separate locations within a buffer.
- `format_split_numeric_string`: Splits a value into two parts and writes the decimal digits of each part into two separate locations within a buffer.
- `format_alphanumeric_string_2`: Converts a numeric value into a sequence of alphanumeric characters and stores them in a buffer.
- `format_alphanumeric_string`: Populates a memory buffer with a sequence of characters derived from a numeric value.
- `format_numeric_string_with_padding`: Converts a numeric value into a string of digits and stores the result in a buffer with specific padding.
- `format_numeric_string`: Converts a numeric value into a string of digits and stores it in a buffer at a specific offset.
- `walk_text_with_parameters`: Loads four 16-bit parameters from the block at the A1 register into globals, then walks a zero-terminated byte string that follows them.
- `calculate_offset_value`: Calculates a value based on a bit-shifted global variable and a multiplier, then stores the result in a global variable.
- `calculate_offset_value_2`: Computes a value using a bit-shifted variable and a multiplier, then stores it in a global variable.
- `calculate_offset_value_3`: Performs a calculation involving a bit-shifted variable and a multiplier to update a global variable.
- `calculate_offset_value_4`: Updates a global variable by calculating a value from a bit-shifted variable and a multiplier.
- `compute_row_offset_and_nibble_2`: Computes a byte offset from a position word and a row value, stores the low four bits of the position word separately, and returns.
- `dispatch_by_mode_flag`: Checks a flag in a table indexed by a value and, when it is set, stores a pointer and dispatches to one of two routines depending on a mode value.
- `apply_bit_mask_pattern`: Reads five source bytes and shifts each into a bit mask, then clears and sets bits in five groups of longwords in a destination table chosen by a mode value.
- `apply_mask_to_two_planes_2`: Processes five source bytes to build bit masks that clear and set bits in two separate destination tables.
- `iterate_cell_grid`: Walks a rectangular range of cells row by row and column by column, setting the cell position globals and calling a routine for each cell.
- `update_bitmask_table`: Modifies specific words in a table by applying bitwise AND and OR operations based on a bitmask and a calculated bit position.
- `update_bitmask_table_2`: Performs bitwise operations on several entries in a table using a bitmask and a calculated bit position.
- `update_bitmask_table_3`: Updates a series of table entries by applying bitwise AND and OR logic controlled by a bitmask.
- `mask_memory_blocks`: Calculates a pointer to a table and performs bitwise AND operations on five specific memory locations using a mask derived from a global variable.
- `decompress_data_stream`: Processes a compressed data stream using various control codes to fill a destination buffer with repeated or single byte values.
- `unpack_packed_picture_stream_alt`: Decodes a packed picture stream into an output buffer using a control-byte scheme where specific patterns repeat bytes or copy literal bytes.
- `decompress_data_stream_3`: Decodes a compressed data stream by interpreting specific bit patterns to determine how many bytes to write to a destination buffer.
- `decompress_data_stream_2`: Processes a compressed data stream by reading control codes to perform repeated byte writes into a destination buffer.
- `decompress_data_stream_variant_a`: Iterates through a data stream to expand compressed values into a destination buffer using various bit-flagged control codes.
- `decompress_data_stream_variant_b`: Decodes a stream of bytes by interpreting specific bit patterns to determine how many bytes to write to a destination buffer.
- `return_stack_value`: This routine returns a value located on the stack.
- `decompress_non_zero_stream`: Processes a compressed data stream and writes only non-zero bytes from the source to a destination buffer based on control codes.
- `decompress_data_stream_variant_c`: Expands a compressed data stream into a destination buffer, skipping zero values during the write process.
- `decompress_data_stream_variant_d`: Reads a compressed stream and writes non-zero bytes from the source to a destination buffer based on control codes.
- `decompress_data_stream_variant_e`: Decodes a data stream by using control bits to decide whether to repeat a single byte or write a sequence of bytes.
- `return_uninitialized_stack_value`: This routine is empty and returns an uninitialized value from the stack.
- `open_raw_window_handle`: Opens the raw window named by a fixed specification string through the DOS library and stores the returned handle in a shared variable.
- `check_input_byte`: Calls a routine to read a byte and, if successful, stores the value in a global variable and sets a flag if the value meets specific criteria.
- `read_input_byte_flagged`: Sets a flag, reads a byte from an input handle using a library call, and clears the flag if the byte is zero or greater than one hundred twenty-two.
- `call_dos_close`: Calls a library routine to close a file handle.
- `drain_pending_input`: Repeatedly checks through the DOS library whether input is waiting, and while it is, reads a byte and sets a ready flag, until no input remains.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:trainer

- `init_graphics_and_keyboard_loop`: Opens the graphics library, saves the system's copper and interrupt state, programs the custom chips' display and copper registers, then loops reading keyboard input until a key status is clear.
- `copy_glyph_rows_to_bitmap`: Draws a string into a bitmap by copying each character's eight-byte glyph from a font table into eight rows spaced forty bytes apart.
- `step_value_up_with_delay`: Repeatedly adds 273 to a shared value, pausing with a countdown after each step, until the value reaches 4095, then returns.
- `step_value_down_with_delay`: Repeatedly subtracts 273 from a shared value, pausing with a countdown after each step, until the value reaches zero, then returns.
- `clear_shared_byte_block`: Zeroes a block of 10240 consecutive bytes in a shared memory region, then returns.
- `init_text_rows_poll_keyboard`: Fills a table of text rows with spaced characters and zero bytes, then toggles a shared byte and polls keyboard input in a loop.
- `wait_for_status_bit_transition`: Spins until a status bit in a hardware byte becomes set, then spins until it is clear again, then returns.
- `patch_nop_instructions_once`: If a shared flag is clear, stores seven address offsets and two no-operation instruction words into shared memory, then returns its first argument unchanged.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:play

- `find_max_build_record`: Walks a linked list to find the largest value in a field, then builds a new record with a counting pattern and exchanges sixteen words through pointer calls.
- `copy_sixteen_words_then_call`: If a destination pointer is set, copies sixteen 32-bit words from a fixed shared table into it, then calls one routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:assign

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field and then updates a header structure.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:echo

- `find_max_and_populate_buffer`: Traverses a linked list to determine the maximum value in a specific field and then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_memory_block_3`: Copies sixteen bytes from a memory location offset from a register to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:skip

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field, then updates a header and performs a series of memory copies between a source block and a destination buffer based on the calculated maximum.
- `copy_memory_block_4`: Copies sixteen bytes of data from a memory location to a destination pointed to by a stack value.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:csr.exe

- `run_timed_display_sequence`: Runs a timed display sequence: saves the system state, sets up the display, DMA and copper registers and a sprite table, then loops over frame values with delays.
- `decrement_until_minus_one`: Counts a 16-bit register down until it reaches minus one, then returns.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:failat

- `update_linked_list_and_copy_data_3`: Traverses a linked list to find the maximum value in a specific field and then updates a data structure with this value.
- `copy_memory_block_5`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:Anthrox

- `run_sequence_restore_saved_word`: Calls routines in sequence, looping a few times through one of them, then clears a word.
- `decrement_until_minus_one_2`: Decrements a value stored in a register until it reaches negative one and then returns.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:cd

- `update_linked_list_and_copy_data_4`: Traverses a linked list to find the maximum value in a specific field, then updates a header and performs a series of memory copies between a source block and a destination buffer.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then calls another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:endif

- `stub_return_zero`: A stub that returns zero immediately, with no other work.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:caesar.boot

- `ProgStart`: Opens the graphics and intuition libraries, sets a flag in the intuition display structure if it is clear, and in that case frees a graphics raster and remakes the display.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:if

- `update_linked_list_and_copy_data_5`: Traverses a linked list to find the maximum value in a specific field, then updates a header and performs a series of memory copies between a source block and a destination buffer.
- `copy_memory_block_7`: Copies a block of sixteen bytes from a source address to a destination address.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 1 of 2)[FD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Caesar Deluxe (1993)(Impressions)[cr ATX][a](Disk 2 of 2)[FD].adf:bootblock

- `open_library_and_set_flag`: Gets a library base through the system's library-opening routine and sets a bit in that library's flags when one is returned.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_graphics_copper_sequence`: Opens the graphics library, saves interrupt and DMA state, clears sprite control registers, and fills a table of sprite values.
- `swap_pair_with_offset_store`: Calls routines, then stores a shared value shifted by a fixed offset into a shared word and swaps two shared 32-bit values.
- `lookup_table_bytes_by_counter`: If a shared counter is nonzero, either sets a shared value and calls a routine when the counter is below ten, or, when it is above sixteen, loads one byte and three further bytes from a table at an offset derived from the counter into shared variables.
- `swap_buffer_pair_back_offset`: Calls a routine twice, stores a value two bytes before a second saved address into a shared pointer, and swaps two saved buffer addresses.
- `pass_through_helper_call`: Calls a helper routine and returns.
- `store_shifted_parameter_values`: Calls a routine, stores a parameter shifted right by three into a shared word, calls another routine, then stores the parameter shifted right by six into a second shared word.
- `advance_and_mask_position`: Adds a stored velocity pair to a two-word position and masks each coordinate to an even value below 4096.
- `start_blit_with_fixed_size`: Calls one routine, then programs the blitter's first and last word masks, control registers and a destination pointer, and starts a blit with a fixed size word.
- `setup_blitter_registers`: Waits for the blitter to become idle and then writes values to several blitter hardware registers.
- `vertical_blank_state_machine`: On a vertical blank request, reads a value through a routine, then advances a frame state machine: an idle state, a countdown, and a running phase that counts 110 frames before returning to idle.
- `start_destination_only_blit`: Calls a routine first, then loads the blitter first-word mask, a control word that enables only the destination channel with no shift, a zero destination modulo, the destination pointer and a small blit size, which starts the operation.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_blitter_idle`: Polls a status byte at a fixed offset from an address register and loops while its busy bit is set, returning once the blitter is idle.
- `reset_text_cursor_state`: Points the character stream pointer at a fixed start address, clears the column counter, sets the line counter to one and clears a flag byte, preparing the decoder for a new run of text.
- `advance_text_stream_cursor`: Reads the next byte of a character stream.
- `step_text_stream_column`: Advances the column counter and starts a new line at 41 columns.
- `start_new_text_line`: Resets the column counter and advances the line counter, then reads the byte at the stream pointer.
- `configure_blit_bit_alignment`: Calls a routine, then sets the blitter first-word mask and control word from the low bits of two position values.
- `copy_paired_words_from_records`: Writes two fixed longword headers at the start of two tables, then copies the first two words of each of 64 six-word source records into the first table and the second table respectively.
- `copy_paired_words_second_bank`: Calls a routine, then writes two fixed longword headers at the start of a second pair of tables and copies the first two words of 64 six-word source records into them.
- `copy_paired_words_to_pointers`: Writes two fixed longword headers at the start of a destination and at a second destination 64 longwords further on, then copies the first two words of 64 six-word source records into the two tables, one word each.
- `store_table_addresses_in_slots`: Stores the low and high words of four table addresses, each offset from a base address by a fixed amount, into two sets of memory slots, then stores one fixed value into another slot.
- `pack_position_into_table_entries`: Writes a position and the position 16 units further on into two tables, storing the halved values in the low bytes with the low bit of each original beside them, then merges two bit fields from a second parameter into the entries.
- `set_table_cursor_and_end`: Stores the input address as the current cursor, then stores an end address computed from it: the cursor plus the first word scaled by four, plus two.
- `toggle_global_buffer_flag`: Calls a routine and then flips the low bit of a global variable, so the value alternates between 0 and 1 on each call.
- `store_state_then_step_thrice`: Saves an input address into two globals, stores two input word values into two others, then calls a routine three times in a row.
- `wait_for_bit_set`: Loops until a specific bit is set in a byte located at an offset from a pointer.
- `start_rectangle_fill_blit`: Computes the width and height of a rectangle from four bounds values in a shared record, then loads the destination pointer returned by a called routine with full first-word masks, a width-based modulo and the blit size, enabling only the destination channel.
- `start_rectangle_copy_blit`: Computes the width and height of a rectangle from four bounds values in a shared record, then loads a pointer returned by a called routine as both source and destination, sets the modulo from the width and starts a blit using the source and destination channels.
- `project_points_to_screen_coords`: Calls a routine, then loops over a list of point pairs, multiplies each x and y by a fixed-point factor masked to ten bits and shifted right by ten, offsets them by 160 and 128, and stores the results, calling a routine for each point.
- `process_indexed_entry_pairs`: For each index pair in a count-driven list, reads a longword pair from a global table at the first index and passes it, with the entry at the second index, to a called routine; it returns the last longword pair read.
- `reset_bounding_box_to_centre`: Sets the four extents of a bounding box, both x values to 160 and both y values to 128, the centre of a 320 by 256 screen, as the starting point before the box is grown to include points.
- `expand_bounding_box_with_point`: Extends a four-value bounding box so it includes a point: the minimum and maximum x are lowered or raised to the point's x, and the minimum and maximum y likewise to its y, leaving each extent unchanged when the point lies inside it.
- `setup_blitter_line_draw`: Programs the blitter for a line between two points: it computes the start address from a row pitch and the column, the major and minor axis steps and the octant code from the signs and the dominant axis, then starts the blit in line mode.
- `build_indexed_record_lists`: Scans a byte table for the first entry above a running value.
- `tick_and_update_audio_channels`: Advances a tick counter; when it reaches its limit it runs a block of routines, pauses briefly and copies stored location and length values into all four audio channel register pairs.
- `reset_global_flag_word`: Writes zero to a global variable several times and then sets it to a fixed mask.
- `apply_channel_effect_tick`: After calling four routines, reads the effect nibble of a channel word and applies one tick of it: a pitch slide up or down, vibrato, a stepped three-tick arpeggio, a volume or panning value, or an extended command.
- `play_tracker_channel_row`: Advances one channel of a music tracker.
- `apply_pattern_effect_tick`: Runs the per-tick effect for one tracker channel from a record: depending on the effect nibble it steps the period up or down, slides toward a target, cycles a three-step arpeggio, applies a vibrato offset from a table or applies a volume slide.
- `copy_record_word_field`: Copies one word from a fixed field of a record into another word field of a second record.
- `find_target_period_entry`: Searches a row of a 37-word table for the first entry that is not above the record's current word, stepping back one entry when a flag is set.
- `slide_period_toward_target`: Moves a channel's current period one slide step toward its target, clamping at the target and clearing the slide once it is reached, then looks up the matching table entry and stores it as the output value.
- `compute_vibrato_period_offset`: Computes a vibrato offset from a phase value and a depth nibble: it reads a value from a waveform table, a ramp or a fixed level depending on the waveform setting, scales it by the depth, applies its sign and adds it to the base period, then advances the phase by the speed nibble.
- `dispatch_tracker_row_effect`: Interprets a row's effect command on the first tick.
- `store_low_nibble_field`: Copies the low nibble of one byte in a record into another byte field of the same record; the decompiled body is this single store, and the wider activity in the listed facts is not shown.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
