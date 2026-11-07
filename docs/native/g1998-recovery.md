# Game 1998 (operation stealth (1990)(u. s. gold)(fr)[cr strike force]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01998). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2713136 top-level bytes in 49 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 231 of 231 named (0 author symbols), 231 of 231 described (47 by rule, 21 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1384 of 1384 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4973 of 4974 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2377410 |
| code | 16188 |
| data | 260810 |
| padding | 10600 |
| unused | 48128 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| setmap | 13 | 653 | 31 |
| Prompt | 2 | 117 | 2 |
| Type | 2 | 117 | 2 |
| ADD21K | 3 | 637 | 38 |
| border | 3 | 228 | 7 |
| Wait | 2 | 117 | 2 |
| my | 1 | 4 | 1 |
| Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 1 of 3).adf:SD01 | 2 | 121 | 3 |
| Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 1 of 3).adf:SD02 | 2 | 121 | 3 |
| Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 1 of 3).adf:Delphine | 2 | 209 | 8 |
| Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 756 | 23 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1765 | 107 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 542 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 427 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- operation stealth (1990)(u. s. gold)(fr)[cr strike force](disk 2 of 3)/Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- operation stealth (1990)(u. s. gold)(fr)[cr strike force](disk 3 of 3)/Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- operation stealth (1990)(u. s. gold)(fr)[cr strike force](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 1 of 3).adf:SD01: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- operation stealth (1990)(u. s. gold)(fr)[cr strike force](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 1 of 3).adf:Delphine: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `read_control_word_variants` (runtime segment an address): Calls the exec library's forbid routine, then reads a hardware control value and stores it in a global word. Kind: disk hardware/control word check. Pass path: the branch not taken by beq.w an address (the fall-through path).

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_and_call_pointer`: If the buffer pointer is non-null, copies sixteen longwords from a fixed offset in the structure into it.

### setmap

- `start_setmap_program`: Program entry for the setmap program.
- `restore_patch_state`: Stores a parameter into a slot of the structure reached through a task pointer, then releases the associated resources.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `propagate_map_updates`: Conditionally calls a routine with several global values, passing a global by reference when a flag is set.
- `store_parameters_and_call_helpers`: Stores its two parameters into two global variables, calls a routine with a fixed set of arguments that reference a global and constant values, then calls another routine with a constant argument.
- `normalize_keymap_name_and_lookup`: Lower-cases a keymap name and searches the installed keymaps for it, storing the matching handle and size in global variables.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_498_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `pass_params_to_subroutine`: Calls another routine with a global variable, its parameter, and the address of an eight-byte stack slot.
- `call_indirect_table_entry`: Calls one function through a pointer read from a global variable at a fixed offset, then returns.
- `call_global_and_measure_buffer`: Calls a function through a global pointer, then counts the characters of a 140-byte stack buffer up to its null terminator.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Prompt

- `fill_block_from_largest_entry`: Program entry.
- `copy_and_call_routine`: The routine copies a block of data from a table to a stack location and then calls another routine.

### Type

- `traverse_list_and_update_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a structure with this value and a generated sequence of data.
- `copy_memory_block`: Copies a fixed number of longwords from a source pointer to a destination pointer.

### ADD21K

- `start_program_clear_globals`: Program entry.
- `open_system_libraries_or_alert`: Opens the dos, graphics and intuition libraries through the exec library, storing each library base in a global slot.
- `stub_return_immediately`: Does nothing and returns at once.
- `allocate_cleared_entry_table`: Allocates a cleared memory block sized six bytes per entry, using a count held in a global, and stores the block pointer in a global slot.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_command_line_arguments`: Parses a command line by skipping whitespace and copying quoted arguments into a newly allocated buffer, treating a doubled quote as a literal quote.
- `append_string_bounded`: Appends a source string to the end of a destination string, copying characters up to the null terminator with a fixed upper bound on the count.
- `append_string_with_limit`: Appends a source string to a destination string using a count passed in a stack slot.
- `copy_string_pad_zeros`: Copies a source string into a destination buffer up to a count, stopping at the null terminator, then fills the rest of the count with zeros.
- `open_file_and_load_entry`: Opens a named file through the dos open wrapper, uses the handle to look up an entry in a loaded list, and stores two of the entry's values in the structure.
- `call_cleanup_callback_then_return`: If a cleanup callback pointer in a global slot is set, calls it.
- `call_cleanup_callback_if_set`: If a cleanup callback pointer in a global slot is set, calls it, then calls one further routine with no arguments.
- `release_loaded_resource_block`: Releases a loaded resource descriptor.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `release_resource_slot`: Checks that a slot index is in range and that the slot is in use, otherwise sets an error code and returns failure.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `dispatch_library_vector_414`: Jumps through the library vector at offset -414 from the library base pointer held in a global slot.
- `call_library_vector_414`: Calls the library function at vector offset -414 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_198`: Calls the library function at vector offset -198 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_294`: Calls the library function at vector offset -294 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Calls the library function at vector offset -210 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `dispatch_library_vector_552`: Jumps through the library vector at offset -552 from the library base pointer held in a global slot.
- `call_library_vector_552`: Calls the library function at vector offset -552 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Calls the library function at vector offset -384 from the library base pointer held in a global slot and returns its result.
- `call_library_vector_498`: Calls the library function at vector offset -498 through a library base pointer held in a global slot and returns its result.
- `call_library_vector_96`: Calls the library function at vector offset -96 through a library base pointer held in a global slot and returns its result.
- `call_library_vector_90`: Calls the library function at vector offset -90 through a library base pointer held in a global slot and returns its result.
- `call_library_vector_78`: Calls the library function at vector offset -78 through a library base pointer held in a global slot and returns its result.
- `call_library_vector_384_2`: Calls the library function at vector offset -384 through a library base pointer held in a global slot and returns its result.

### border

- `setup_display_wait_for_flag`: Program entry.
- `load_hunk_sections_and_relocate`: Loads executable hunks.
- `return_from_relocation`: This routine is an empty function that performs no operations before returning to its caller.
- `decode_bitmask_back_references`: Reads a longword of flags and shifts it one bit at a time.
- `decode_bitmask_stream_block`: Decodes a block driven by a bitmask.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bits_and_store_pointer`: Repeats a count of times: stores the low word of a pointer into a global slot, shifts a bit register right one place, and reloads it from the longword before the pointer when it empties.

### Wait

- `update_list_and_sync_buffer`: Traverses a linked list to find the maximum value in a specific field, then updates a header structure and performs a series of memory copies between a source buffer and a destination structure.
- `copy_memory_block_2`: Copies a fixed number of words from a memory location to a destination pointed to by a stack value.

### my

- `terminate_execution`: The routine is empty and contains no valid instructions.

### Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 1 of 3).adf:SD01

- `unpack_compressed_hunk_data`: Program entry.
- `read_shifted_previous_longword`: Reads the 32-bit value just before the address register, shifts it right by one bit, sets its top bit and returns the result.
- `shift_bit_stream_backwards`: Shifts a bit register right one place.

### Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 1 of 3).adf:SD02

- `decompress_bitstream_data`: Decodes a compressed bitstream by reading longwords and shifting bits into a destination buffer.
- `fetch_shifted_bit_word`: Returns the longword located at the address pointed to by a register minus four, shifted right by one bit with the top bit set.
- `skip_zero_bits`: Decrements a pointer and fetches shifted longwords until a non-zero value is found or a counter reaches zero.

### Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 1 of 3).adf:Delphine

- `start_hunk_loader_program`: Program entry for a hunk loader.
- `stub_empty_return_routine`: The routine is empty apart from a return.
- `busy_wait_for_count`: Spins in a loop, decrementing a register counter until it wraps past zero, then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 132 into d0.
- `decode_run_length_bytes_backward`: Repeatedly calls a routine whose returned values control the decode.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_wrap_to_space`: Decrements a byte register once per count given in a data register, wrapping it to a space character when it reaches zero.
- `empty_routine`: This routine is empty and contains no executable instructions.

### Operation Stealth (1990)(U. S. Gold)(Fr)[cr Strike Force](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_blitter_and_display_registers`: Clears a block of words, writes a series of values to blitter and display control registers through a global slot, and calls a routine that waits for the blitter to finish.
- `advance_frame_counters`: Advances two counters.
- `handle_pending_interrupt_bit`: Tests a bit in a global interrupt request word.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d4.
- `reset_blitter_when_counter_expires`: Decrements a countdown.
- `program_blitter_on_second_countdown`: Decrements a second countdown.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `advance_blitter_table_entry`: Walks a byte table, advancing an index until it finds an entry that is not the end marker.
- `advance_second_blitter_table_entry`: Walks a second byte table, advancing an index until it finds an entry that is not the end marker.
- `call_pair_then_set_flag`: Calls two routines with no arguments, then sets a global flag byte to one.
- `remap_bytes_from_table_pairs`: For each destination byte, walks a table of byte pairs to find the pair whose first byte matches, then copies its second byte into the destination.
- `set_plane_pointers_for_row`: Advances a row counter with bounds checks, then computes a row offset of forty-six bytes and writes three long pointers, spaced by fixed offsets, into a global table.
- `read_control_word_variants`: Calls the exec library's forbid routine, then reads a hardware control value and stores it in a global word.
- `open_graphics_and_start_copper`: Opens the graphics library through the exec library, sets the copper list pointer and enables a DMA channel, then closes the library and re-enables multitasking through the exec library.
- `fill_offset_table_from_lookup`: Looks up the next non-terminator entry in a byte table and stores six copies of it into fixed locations, each offset by a different constant.
- `reset_audio_control_words`: Clears four global words, sets one global word to fifteen, and clears bit one of it.
- `fill_table_slots_from_index`: Clears a global word and copies a flag byte when a version marker matches.
- `update_audio_channel_registers`: Decrements a global counter, then for each of four audio channels adds a delta from its block, stores the period and pointer words into the channel registers, and calls routines to select the next step when a channel is active.
- `set_audio_channels_from_pattern`: Loops over four audio channels, looking up each channel's pattern entry from a table, computing its volume and period from the entry's fields, and storing the result in that channel's control block.
- `initialize_audio_channel_block`: Clears a flag bit in the channel block and copies eight longwords from a source table into the destination.
- `store_channel_period_from_table`: Looks up a 16-bit value in a period table using a byte index and stores it in a field of the channel block.
- `apply_active_channel_updates`: Walks four audio channel blocks and calls a routine for each block whose active flag is set.
- `update_channel_envelope_step`: Updates one audio channel's envelope.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_failure_status`: Returns the all-ones value minus one with no other work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_state_and_open_library`: Initialises a block of eight longwords and saves the stack pointer.
- `setup_intuition_library_sequence`: Runs a sequence of seven routine calls that relate to the intuition library, whose name and a version string appear in the referenced text.
- `release_conditional_resources`: Conditionally releases resources held in two global slots.
- `open_console_device_and_setup`: Opens the console device through a routine call and stores its handle in a global slot.
- `setup_border_resource_entry`: Looks up a named entry through a routine using a fixed identifier.
- `allocate_border_entry`: When the state block is empty, allocates an entry through a routine and copies data from the state block into it.
- `apply_negated_offsets_to_record`: Stores two longwords into a record, then calls two routines with negated record offsets taken from the record and calls one further routine with a constant.
- `apply_negated_offsets_then_delay`: Calls two routines with negated offsets taken from a record, then waits ten units through the dos delay wrapper.
- `store_param_and_call_routine`: Stores its first argument in a global slot, then calls a routine with the second argument and the address of a stack slot.
- `open_dos_library_or_alert`: Opens the dos library by name through the exec library's old-open routine and calls the alert and supervisor routines as needed when the library cannot be opened.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `allocate_named_resource_entry`: Allocates a cleared table of six-byte entries sized by a global count and initialises its first entry.
- `parse_quoted_command_arguments`: Parses a command string by skipping whitespace and copying quoted arguments into an allocated buffer, treating a doubled quote as a literal quote.
- `parse_command_arguments_into_block`: Parses a command string into an allocated block of arguments, skipping whitespace and copying quoted text with doubled quotes treated as literal quotes.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros_2`: The routine copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `append_string_with_limit_3`: The routine moves to the zero terminator of a destination, then appends bytes from a source until the source's zero byte is copied or a limit is reached.
- `append_string_with_limit_2`: Moves to the zero terminator of the destination, then appends bytes from the source until the source's zero byte is copied or the counter in the third argument runs out.
- `compare_byte_strings_limited`: Compares the first string with the second byte by byte and returns minus one or one at the first difference, depending on which byte is smaller.
- `copy_zero_terminated_string`: Copies bytes from the source into the destination one at a time, including the zero terminator, and stops after that terminator.
- `lookup_window_icon_fields`: Looks up a window through a library call, follows its pointer chain to an icon entry, and stores two values from that entry into the caller's structure.
- `format_string_into_buffer`: Stores the destination pointer in a global slot, runs the formatting routine on the caller's arguments with a byte-writing callback, then writes a zero terminator at the next output position.
- `format_string_into_buffer_short`: Stores the destination pointer in a global slot, runs the formatting routine on the caller's arguments with a byte-writing callback, then writes a zero terminator at the end of the output.
- `emit_byte_to_output`: Writes the low byte of its argument at the current output position, advances that position by one, and returns the byte.
- `forward_format_arguments`: Passes its single argument on to the formatting routine, using the caller's stack arguments that follow it as the variable argument list.
- `call_with_stack_argument_list`: The routine consists of 48 instructions and calls another routine with a format argument and the address of its stack argument list.
- `interpret_format_specifiers`: Walks a format string, passing literal bytes through a character callback and expanding each percent conversion from the argument list.
- `perform_signed_division`: Works out whether the result should be negative from the signs of the two register inputs, calls a division routine, negates its result when needed, and returns it with the second input in a 64-bit pair.
- `call_and_return_concatenated`: Calls a subroutine and returns a 64-bit value whose high half is the second register left by that call and whose low half is the second register's original value.
- `call_itself_recursively`: Calls itself with no other work and then returns, so each invocation immediately makes another call to the same routine.
- `perform_32bit_division`: Divides a 32-bit dividend by a 32-bit divisor, using a bit-by-bit shift and subtract loop when the divisor is wider than 16 bits and a 16-bit division otherwise.
- `compute_string_length`: Walks forward from the given pointer to the zero terminator without changing memory, and returns the number of bytes before that terminator.
- `forward_call_no_arguments`: Calls one subroutine with no arguments and returns without a value.
- `invoke_three_subroutines`: A 21-instruction routine that makes three subroutine calls and is referenced from three other routines and by address.
- `return_without_effect`: Its decompiled body is a bare return with no operations, and it is referenced by address from one caller.
- `invoke_two_subroutines`: A 15-instruction routine that makes two subroutine calls and is referenced by two other routines and by address.
- `emit_structure_byte_output`: Writes the byte at a fixed offset of the structure held in one address register to the current output position, advances that position by one, and returns the byte.
- `visit_fixed_size_records`: Walks a table of 22-byte records from a fixed start address up to a fixed end address, calling a per-record routine on each record in turn.
- `release_record_resources`: Returns an all-ones error value when given no record; otherwise, for a record with its active flag set, calls helper routines selected by the flag bits, combines their results, then clears the record's three words and flag byte and returns the combined result.
- `release_record_without_null_check`: Takes its record from an address register and performs the same flag-driven helper calls and record clearing as the null-checked variant, without the null test.
- `write_byte_into_buffer`: Writes the byte from its argument into a small buffered output record, calling a helper to set up the buffer when none exists and marking an error when a write fails.
- `write_parameter_byte_to_buffer`: Writes the byte at a fixed offset of its parameter block into the same buffered output record and returns it.
- `emit_parameter_byte_buffered`: Resets the record when its sentinel field is minus one; otherwise writes the parameter byte directly, or through a helper when the element size is one.
- `emit_parameter_byte_direct`: Writes the parameter block byte into the output record, using a helper for single-byte element sizes and marking the record as errored if that helper fails.
- `write_byte_and_advance_buffer`: Sets the output pointer to the start of the buffer, marks the record as active, stores the parameter byte there and advances the pointer by one.
- `find_free_record_slot`: Scans the table of 22-byte records for one whose flag byte is clear, zeroes its three words, and returns its address, or a null pointer when every record is in use.
- `find_next_free_record`: From the record address in a register, scans forward through the 22-byte table for a record with a clear flag byte, zeroes its three words, and returns its address, or null at the end of the table.
- `clear_record_header_words`: Zeroes the first three 32-bit words of the record in an address register and returns, leaving the flag byte unchanged.
- `initialize_stream_buffer`: Sets up an output record: if the helper returns no buffer, it records a size of one and points the buffer at an inline slot; otherwise it stores the supplied buffer, a 1024-byte size and flag bits, and sets a further flag when a second helper reports success.
- `setup_buffered_stream_record`: Stores a 1024-byte size and a buffer flag bit in the record, takes the buffer pointer from the caller's frame, and sets another flag when a helper reports success.
- `release_all_list_nodes`: Walks a singly linked list from a global head pointer, calling a release helper on each node with the node's payload address, then clears the head and returns zero.
- `release_list_nodes_from_pointer`: Walks a linked list starting at the node in an address register, releasing each node through a helper call, then clears the list head and returns zero.
- `allocate_and_link_list_node`: Calls an allocation helper for a block sized from its argument plus eight, returning null on failure.
- `push_list_node_with_payload`: Pushes a node onto the head of a global linked list, storing the previous head and a payload pointer taken from the caller's frame, and returns the address after the two link words.
- `forward_low_word_argument`: Passes the low sixteen bits of its argument to a subroutine and returns nothing.
- `remove_list_node_by_payload`: Searches the global list for the node whose payload address matches the argument, unlinks it, and releases it through a helper using its stored size.
- `unlink_frame_node_from_list`: Walks the list from the head register looking for the node whose payload matches the pointer in the caller's frame, unlinks it, and releases it through a helper.
- `unlink_list_node`: Unlinks the node in an address register from a singly linked list by pointing its predecessor, or the list head when there is none, at the node's successor.
- `pop_list_head_node`: Sets the list head to the successor of the node in an address register, calls a helper, and returns zero.
- `check_table_entry_status`: Checks that the index argument falls inside the table and that its six-byte entry is populated.
- `check_interactive_status`: The routine checks a value in a table and returns the result of an interactive status check.
- `return_zero_constant`: Returns zero without any other work and is referenced by address from one other routine.
- `handle_file_deletion`: Calls a routine to delete a file and, if the result is zero, calls another routine to fetch an error and stores that value in a global variable.
- `return_zero`: This routine is empty and returns zero.
- `dispatch_two_helper_calls`: A 19-instruction routine that makes two helper calls and is called from five places.
- `check_mode_flags_then_call`: Checks that the low two flag bits of a structure field are set; if not, it stores an error code of 5 and returns minus one.
- `dispatch_io_calls`: The routine performs a call and, if it returns a failure, executes a callback and stores the result in a global variable.
- `return_empty_callback`: Contains only a return with no operations and is referenced by address from one caller.
- `poll_mask_bit_result`: Calls a library function with zero and a fixed mask, and if that mask bit is set it returns the returned mask when a global option is zero.
- `call_helper_return_zero`: Calls a helper with no arguments and then returns zero.
- `write_fixed_text_then_callback`: The routine gets an output handle, writes a four-byte text fragment from a fixed address to it, and then runs a further routine.
- `invoke_optional_callback_then_helper`: Calls an optional callback held in a global slot when one is set, then calls a helper with no arguments and returns.
- `tear_down_entry_tables`: Releases a table of six-byte entries by calling a helper on each entry and then on the table block.
- `stub_empty_return`: Contains only a return instruction, sits in a gap that decodes as code, and is not called in the reference run.
- `release_table_entry_by_index`: Validates the index against the table count and checks whether the six-byte entry is populated.
- `close_dos_file_handle`: Calls the DOS library's Close function through the library base stored in a global slot.
- `call_dos_current_dir`: Calls the DOS library's CurrentDir function through the library base stored in a global slot and returns nothing.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dos_input_function`: Calls the DOS library's Input function through the library base stored in a global slot and returns nothing.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_dos_open_function`: Calls the DOS library's Open function through the library base stored in a global slot and returns nothing.
- `call_dos_output_function`: Calls the DOS library's Output function through the library base stored in a global slot and returns nothing.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_entry_108`: Calls the library function at offset minus 108 from the library base held in a global slot and returns nothing.
- `call_library_entry_450`: Calls the library function at offset minus 450 from the library base held in a global slot and returns nothing.
- `forward_to_library_entry_414`: Makes one call to the library function at offset minus 414 from the library base held in a global slot, then returns.
- `call_library_entry_414`: Calls the library function at offset minus 414 from the library base held in a global slot and returns nothing.
- `call_library_entry_330`: Calls the library function at offset minus 330 from the library base held in a global slot and returns nothing.
- `forward_to_library_entry_198`: Makes one call to the library function at offset minus 198 from the library base held in a global slot, then returns.
- `call_library_entry_198`: Calls the library function at offset minus 198 from the library base held in a global slot and returns nothing.
- `forward_to_library_entry_294`: Makes one call to the library function at offset minus 294 from the library base held in a global slot, then returns.
- `call_library_entry_294`: Calls the library function at offset minus 294 from the library base held in a global slot and returns nothing.
- `call_library_entry_132`: Calls the library function at offset minus 132 from the library base held in a global slot and returns nothing.
- `forward_to_library_entry_210`: Makes one call to the library function at offset minus 210 from the library base held in a global slot, then returns.
- `call_library_entry_210`: Calls the library function at offset minus 210 from the library base held in a global slot and returns nothing.
- `call_library_entry_336`: Calls the library function at offset minus 336 from the library base held in a global slot and returns nothing.
- `call_library_entry_372`: Calls the library function at offset minus 372 from the library base held in a global slot and returns nothing.
- `call_library_entry_444`: Calls the library function at offset minus 444 from the library base held in a global slot and returns nothing.
- `forward_to_library_entry_552`: Makes one call to the library function at offset minus 552 from the library base held in a global slot, then returns.
- `call_library_entry_552`: Calls the library function at offset minus 552 from the library base held in a global slot and returns nothing.
- `call_library_entry_366`: Calls the library function at offset minus 366 from the library base held in a global slot and returns nothing.
- `call_library_entry_378`: Calls the library function at offset minus 378 from the library base held in a global slot and returns nothing.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `forward_to_single_callee`: A one-instruction routine that makes a single subroutine call and returns.
- `stub_empty_no_instructions`: The listing reports zero instructions at this location, so it contains no code of its own.
