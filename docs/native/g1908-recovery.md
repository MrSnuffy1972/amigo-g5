# Game 1908 (international karate + (1988)(system 3)[h]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01908). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901140 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (30 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 46 of 46 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 362496 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2404 of 2728 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1410 |
| code | 9368 |
| data | 394576 |
| padding | 1706 |
| unused | 494080 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| International Karate + (1988)(System 3)[h].adf:ikload | 1 | 43 | 2 |
| International Karate + (1988)(System 3)[h].adf:more | 19 | 2290 | 76 |
| International Karate + (1988)(System 3)[h].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| International Karate + (1988)(System 3)[h].adf:ik+ (raw code file, load address unknown) | 1 | 372 | 1 |

## Cases handled

- 5 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- international karate + (1988)(system 3)[h]/International Karate + (1988)(System 3)[h].adf:ik+: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 362496 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `show_help_and_reopen_messages` (more): A 175-instruction routine reached from a gap in the file. Kind: sabotage path / error handler. Pass path: not determined.

## Functions


### International Karate + (1988)(System 3)[h].adf:ikload

- `stub_decompile_timeout_open_dos`: Pseudo-code is unavailable because decompilation timed out.
- `stub_decompile_timeout_pcrel_seven`: Pseudo-code is unavailable because decompilation timed out.

### International Karate + (1988)(System 3)[h].adf:more

- `open_file_then_message_loop`: Program entry that opens the current directory, opens a named file, allocates memory, and opens the dos library, then loops over messages from its port.
- `release_file_resources_and_reply`: Saves a parameter into a state slot and conditionally calls cleanup routines from a table, closing libraries and files, freeing memory and replying to a message.
- `run_file_viewer_messages_loop`: Large routine of 340 instructions that calls many library stubs.
- `dispatch_four_table_values`: Loops four times, passing one field from a structure and each of four table values to a routine.
- `detect_control_bytes_in_block`: Calls a routine twice to fill and check a buffer, then scans up to 80 bytes for control characters outside the normal text ranges.
- `wait_for_signal_with_timeout`: Waits in a loop on a signal with a an address timeout and gives up when an abort flag is seen.
- `show_help_and_reopen_messages`: A 175-instruction routine reached from a gap in the file.
- `init_struct_loop_until_done`: Sets two fields of a structure, makes a call with values from it and stores the result, then calls further routines.
- `store_result_and_bump_counter`: Calls two routines with values derived from the structure, stores the second result in a field, and if a flag field is zero increments a counter reached through a pointer.
- `scan_buffer_for_line_break`: Reads a block into a buffer through a routine and records its size in a field.
- `page_lines_with_more_prompt`: Computes a line limit from structure fields, then loops calling routines to draw entries one line at a time until the limit or end of buffer.
- `run_search_with_progress_text`: Selects one of two global values using a flag and calls routines to prepare a search.
- `run_setup_calls_return_result`: Runs a fixed sequence of routine calls using values from a parameter block and a global, and returns the result of one of those calls.
- `call_pair_then_conditional_call`: Calls a routine with two arguments, then checks a short field in the structure and calls another routine with one argument only when that field is zero.
- `call_bounds_then_scroll`: Calls a routine with a field from the structure and a fixed value, then calls another routine with a value taken from a structure field.
- `call_then_call_decremented_bound`: Calls a routine with a field from the structure and fixed arguments, then calls another routine with that structure field decremented by one.
- `iterate_text_lines_limited`: Iterates over a text buffer for up to 128 characters, calling routines to measure and advance each line.
- `forward_then_call_with_global`: Calls one routine with its argument, then calls a second routine with a pointer taken from the structure and the global block.
- `set_flag_and_forward`: Sets a short field to one when a field in the structure is zero, then calls a routine with a pointer from the structure and the global block.
- `dispatch_by_flag_pointer`: Chooses between two pointers based on a flag in the parameter block, then calls a routine with the chosen pointers and a value from the structure.
- `format_text_by_flag`: Calls a routine to format a text string using the format %s%s.
- `clamp_negative_result_to_zero`: Calls a routine twice with the same first argument and different third values, then clamps a negative return value to zero before returning it.
- `copy_from_buffer_advancing_offsets`: Compares a requested count with a remaining-size field in a structure and refills through a routine when the buffer is short.
- `refill_buffer_and_advance`: Clears and copies state fields in a structure, then calls a library routine to fill the remaining space of a 4096-byte block.
- `compute_scroll_target_position`: Calculates a target position from a mode value and an offset, using start and end fields of a window structure.
- `read_line_into_buffer_limited`: Reads bytes one at a time through a routine into a destination buffer until a newline or about 127 bytes, then writes a terminating zero and returns the count of bytes read.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination one at a time, including the terminating zero.
- `convert_string_to_uppercase`: Converts each lowercase letter in a null-terminated string to uppercase by subtracting 32, and leaves other characters unchanged.
- `parse_signed_decimal_string`: Reads an optional plus or minus sign and then the decimal digits of a string, returning the signed integer value they form.
- `call_library_mask_when_set`: When a pointer argument and a pointer field in the structure are both non-null, calls a library function with that field, the argument and a full mask value.
- `process_message_structure`: Recursively processes a message structure.
- `send_single_value_message`: Obtains a port pointer from a routine, stores its argument in a stack buffer, and calls a library message function with that buffer, a fixed code and a size of one.
- `allocate_request_copy_fields`: Allocates a 36-byte request block and sends it through a library message routine with a fixed code.
- `allocate_template_linked_object`: Allocates a 68-byte object, links its fields to a template and a source block, and copies a count of words from the source into it.
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
- `forward_with_stack_slot`: Calls one routine with two arguments and a pointer to a fixed stack slot, then returns.
- `read_byte_or_minus_one`: Calls a routine to read one byte into a small stack buffer.
- `format_stack_buffer_and_send`: Calls a library function through a global base, then measures a 140-byte stack buffer up to its null terminator.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `divide_longword_by_word`: Divides a 32-bit value by a second value using repeated shift and subtract steps, with a separate path for divisors that fit in 16 bits.
- `multiply_longwords_partial_products`: Multiplies two 32-bit values by combining 16-bit partial products.
- `forward_single_call_gap`: Calls one other routine and returns.
- `negate_result_on_flag`: Calls a routine and returns its first result value, negated when a second result value is non-zero.
- `call_formatted_output_vector`: Calls a formatted-output library function through an offset from a global base.
- `store_byte_at_pointer`: Writes the low byte of a register to the address held in a second register, then returns.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_and_fill_message_block`: Allocates a 34-byte message block and fills its fields with a parameter, a fixed type code, a flag byte and a value from a routine.
- `delete_message_port_block`: Marks a message port block as deleted and frees its signal bit through a library call.
- `search_text_for_substring`: Searches a null-terminated text for a second string, trying each starting position in turn.
- `initialise_pointer_list_head`: Sets up a small list header in the block it is given.

### International Karate + (1988)(System 3)[h].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library_call_parameters`: Loads a long value into register d1, clears registers d2 and d3, and calls a function at an offset from a library base address.

### International Karate + (1988)(System 3)[h].adf:ik+ (raw code file, load address unknown)

- `stub_decompile_timeout_raw_region`: Pseudo-code is unavailable because decompilation timed out.
