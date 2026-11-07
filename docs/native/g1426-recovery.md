# Game 1426 (Manager): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01426). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904464 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 22 raw-word lines |
| Functions named and described | pass, 545 of 545 named (0 author symbols), 545 of 545 described (88 by rule, 91 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4622 of 4622 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 34463 of 34569 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 118046 |
| data | 99927 |
| padding | 3763 |
| unused | 682496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Manager.adf:setmap | 13 | 627 | 33 |
| Manager.adf:manager | 3 | 31838 | 426 |
| d | 1 | 13 | 1 |
| 17 | 1 | 2 | 1 |
| 8 | 1 | 115 | 2 |
| Manager.adf:man.int | 1 | 76 | 5 |
| diskfont.library | 15 | 1040 | 61 |
| Manager.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 844 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 713 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Manager/Manager.adf:man.int: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Manager.adf:setmap

- `dispatch_cli_or_workbench_start`: Program entry point.
- `close_libraries_and_reply`: Cleanup handler that closes the library held in its first global slot when it is open, and when the second slot is set it forbids task switching and replies to the startup message.
- `call_two_library_functions`: Calls two library functions through fixed negative offsets from the library base held in a register, one after the other, and returns nothing.
- `open_dos_library_or_fail`: Opens dos.library by name through the exec library.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `release_signal_device_library`: Releases resources held in global slots, each only when its handle or flag is set: it frees a signal, closes a device and closes a library, then passes its incoming parameter to a final release call.
- `store_parameters_and_call_routines`: Saves its two arguments into two global variables, then calls one routine with constant values and global addresses, and then calls a second routine with a single constant.
- `load_keymap_into_console`: Gets a keymap name from the arguments or the icon tool types and lowercases it.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_head`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `call_find_name`: Library stub: loads its arguments and the exec library base, calls FindName and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_alloc_signal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_open_resource`: Library stub: loads its arguments and the exec library base, calls OpenResource and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `forward_global_and_stack_buffer`: Calls another routine with a global value, its own parameter, and the address of an eight-byte stack slot, then returns.
- `call_indirect_table_entry`: Calls one library function through a pointer read from a global variable at a fixed negative offset, then returns with no other work.
- `format_message_and_output`: Formats a message into a 140-byte stack buffer with RawDoFmt, measures the string length up to its terminator, and passes the buffer and length to the output routine when the string is not empty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Manager.adf:manager

- `run_manager_startup_sequence`: Program entry point of the manager module.
- `run_short_init_sequence`: Short sequence that calls one table routine, two routines and then a run of further table entries in order, with no logic of its own.
- `run_long_table_call_sequence`: Makes a long run of calls through pointers at fixed offsets, repeating a small set of table entries, and returns.
- `reset_flags_after_repeated_calls`: Calls the same table routine many times in a row, then clears several global flags and counters to zero.
- `update_flag_and_call_setup`: Reads a global flag and either calls a setup routine and clears the flag, or sets the flag to all ones; it then calls a series of table routines with constant and global arguments and copies two global values.
- `read_values_into_indexed_table`: Runs several table routines to start, then loops over values read through a table routine, storing each one into an indexed table in memory at a position computed from a second value.
- `initialise_indexed_table_entries`: Clears entries of an indexed table in memory, setting two fields to fixed constants before each pass, with loop counters taken from table routine results.
- `seed_triples_and_run_loop`: Stores two sets of three constant values into two structures, sets two flags, calls table routines, and then runs a loop that calls table routines for each entry.
- `store_global_address_slots`: Stores the address of a shared data block into a series of global slots, and after each store makes three table calls.
- `process_executable_hunk_records`: Compares values with AmigaDOS hunk type codes while walking the records of a loaded executable, calling routines for each record it handles.
- `validate_and_count_input_values`: Repeatedly reads values through table routines and accepts only those inside a fixed range; for accepted values inside a narrower band it updates a counter byte in a structure, then repeats.
- `validate_hunk_type_range_2`: Checks if a value falls within specific ranges corresponding to AmigaDOS hunk type codes.
- `validate_hunk_type_and_update_state`: Checks a value against a range of valid AmigaDOS hunk type codes.
- `dispatch_hunk_type_actions`: Iterates through a range of values to identify specific AmigaDOS hunk type codes.
- `handle_hunk_type_dispatch`: Identifies specific AmigaDOS hunk types within a loaded executable and executes corresponding logic based on the record type.
- `handle_hunk_type_records`: Iterates through executable hunk records and performs specific actions based on their type.
- `parse_executable_hunk_types`: Iterates through executable hunk records and compares their type codes against specific AmigaDOS hunk constants.
- `parse_executable_hunk_types_2`: Iterates through executable hunk records to identify specific AmigaDOS hunk types.
- `validate_hunk_type_range`: Checks if a value falls within specific ranges corresponding to AmigaDOS hunk type codes.
- `call_table_routines_in_order`: Makes a long, fixed sequence of table calls, writing a stack slot before each call, and returns once the sequence completes.
- `call_two_table_routines`: Calls two game-internal routines through pointers at fixed offsets from a base register and returns, with no other work.
- `execute_indirect_function_pointers`: Retrieves two function pointers from memory using an offset from a register and executes them sequentially.
- `classify_input_code_ranges`: Waits for a value from a table routine that falls inside a band of codes, then stores one fixed constant when the value lies in one range and a different constant when it lies in a second range.
- `scan_grid_until_marker`: Initialises a range field, then reads grid values through a table routine until one equals a fixed marker, counting the other entries in a byte field, and then acts on that count.
- `process_grid_cells_by_type`: Initialises the same range fields, then walks grid cells read through a table routine and calls different table routines depending on whether a cell is empty or holds the value one.
- `copy_grid_cells_by_index`: Clears a counter, then loops over pairs of values read through table routines, copying a cell from one grid array into another at a position computed from the pair.
- `scan_grid_rows_for_nonzero`: Initialises range fields, then loops over grid entries read through table routines, resetting a row field and testing each cell for a non-zero value, with one table call per pass.
- `compare_grids_cell_by_cell`: Loops over every cell of two grids and increments the second grid's cell wherever the two values differ, then calls one table routine at the end of each pass.
- `reset_fields_run_table_calls`: Clears four global fields and makes several table calls, then, when a flag is set, makes a longer run of further table calls.
- `store_returned_handle_if_empty`: Makes a long series of table calls, then, if a stored pointer is still zero, makes two more calls and saves the returned value into that pointer.
- `release_pointer_slot_if_set`: Makes a few table calls, then, if a global pointer slot is set, makes two more calls and clears that slot before one final call and return.
- `call_three_routines_when_set`: Makes a few table calls and, only when two global pointer slots are both non-zero, makes three further table calls before returning.
- `dispatch_by_two_flag_states`: Makes a few table calls, then tests two byte flags and takes one of several paths depending on whether each flag is set to one or to zero, calling a different routine on each path.
- `initialise_range_and_scan_loop`: Makes several table calls and initialises a range field to fixed bounds, then loops while a length field is zero, reading values through table routines on each pass.
- `chain_table_calls_on_result`: Makes a sequence of table calls, feeding each returned value into the next call as its argument.
- `chain_table_reads_on_results`: Makes several table calls, then repeatedly reads a value and passes each result into the next read, chaining the results together.
- `read_entries_two_step_offsets`: Reads a value through a table routine, then reads again at positions two greater than the previous result, repeating that step several times in a row.
- `run_counted_grid_pass_loop`: Sets a flag and makes a table call, then loops over a counter held in a byte, initialising range fields and calling table routines on each pass until the counter passes a limit read from a second byte.
- `repeat_range_init_and_read`: Repeats a block that calls a table routine, initialises the range fields to fixed bounds, and reads values through another table routine while a flag stays set.
- `identify_amiga_hunk_type_2`: Compares a value against known AmigaDOS hunk type constants to differentiate between executable code segments and relocation segments.
- `classify_amiga_hunk_type`: Compares a provided value against known AmigaDOS hunk type constants to differentiate between executable code segments and relocation segments.
- `poll_flag_and_read_values`: Loops on a table routine that returns a flag; while the flag is set it initialises the range fields and reads values through table routines before polling again.
- `parse_amiga_hunk_types_3`: Iterates through a data structure to identify and process AmigaDOS hunk types, distinguishing between executable code and relocation segments.
- `validate_amiga_hunk_type`: Compares a value against known AmigaDOS hunk type constants to distinguish between executable code segments and relocation segments.
- `parse_amiga_hunk_metadata`: Iterates through a data structure to identify and process AmigaDOS hunk types used for executable loading and relocation.
- `parse_hunk_metadata`: Iterates through a data structure to identify and process specific hunk type codes.
- `parse_amiga_hunk_types`: Iterates through a data structure to identify and process specific hunk type codes.
- `identify_amiga_hunk_types`: Compares a value against known AmigaDOS hunk type codes to identify executable loading or relocation metadata.
- `validate_hunk_type_codes_2`: Compares data values against AmigaDOS hunk type codes to identify executable loading or relocation segments.
- `parse_amigados_hunk_types`: Iterates through a data structure to identify and process specific hunk type codes.
- `parse_amiga_hunk_types_2`: Iterates through a data structure to identify and process specific hunk type codes.
- `validate_amigados_hunk_types`: Compares data values against AmigaDOS hunk type codes to identify executable loading or relocation segments.
- `classify_hunk_type`: Compares a value against AmigaDOS hunk type constants to distinguish between executable code segments and relocation segments.
- `identify_hunk_type_2`: Compares a value against AmigaDOS hunk type constants to determine if a memory segment is for code execution or relocation.
- `identify_amiga_hunk_type`: Compares a value against known AmigaDOS hunk type constants to distinguish between executable code segments and relocation segments.
- `compare_indexed_entry_to_threshold`: Polls a table routine, then while the flag is set reads values and compares an indexed table entry with a threshold of 99, branching on whether the entry is above it.
- `build_constant_value_table`: Writes a sequence of fixed constant pairs into stack slots, calling a table routine after each pair, to build a table of constant values before further calls.
- `process_chained_table_lookups`: Performs a sequence of table-based lookups where the result of one read is used as the index for the next.
- `scan_cells_with_range_init`: Initialises a range field to fixed bounds, then loops over cells read through a table routine, comparing each cell's byte value with a fixed code before continuing.
- `validate_hunk_type_code`: Compares a value against AmigaDOS hunk type codes to identify executable loading or relocation data.
- `lookup_entry_and_set_bounds`: Clears a counter, reads a value through a table routine and uses it to index a table of stored values, then computes two bounds from fields of a structure.
- `clear_counters_and_scan_loop`: Clears six global counters and fields, sets a range field to fixed bounds, and then runs a loop of table calls while a flag stays set.
- `validate_hunk_type_codes`: Compares data values against specific AmigaDOS hunk type constants to identify executable loading or relocation code.
- `clear_indexed_grid_row_entries`: Sets a byte flag to one, then in a loop writes a fill value into a byte and zeroes entries of an indexed table whose position is computed from the flag and the table width, calling table routines in between.
- `identify_hunk_type_3`: Compares a value against a set of AmigaDOS hunk type codes to determine the specific executable loading or relocation category.
- `identify_hunk_type_4`: Compares a value against known AmigaDOS hunk type constants to determine the specific executable loading or relocation category.
- `dispatch_hunk_type_callback`: Executes a specific function from a table based on a provided identifier, such as an AmigaDOS hunk type.
- `step_counter_and_reinitialise_ranges`: Increments a byte counter in nested loops, and when it passes six it initialises the range fields to fixed bounds and calls table routines before repeating.
- `parse_dos_hunk_types`: Iterates through a data stream to identify and process AmigaDOS hunk type codes.
- `process_amigados_hunk_types_2`: Identifies and executes specific callback routines based on AmigaDOS hunk type codes.
- `process_amigados_hunks`: Iterates through a sequence of data structures, comparing values against AmigaDOS hunk type codes to determine how to handle executable loading or relocation data.
- `handle_amigados_hunks`: Iterates through a sequence of data structures to identify and process AmigaDOS hunk types.
- `dispatch_hunk_type_callback_3`: Identifies and executes specific handler routines based on AmigaDOS hunk type codes.
- `identify_hunk_type_5`: Compares a value against AmigaDOS hunk type codes to determine the nature of an executable hunk.
- `process_hunk_type_callbacks`: Iterates through a sequence of values to identify and dispatch specific AmigaDOS hunk type handlers.
- `process_amigados_hunk_types_3`: Iterates through a sequence of values to determine how to handle specific AmigaDOS hunk types for executable loading or relocation.
- `process_amigados_hunk_types`: Compares a value against AmigaDOS hunk type codes to determine how to handle executable loading or relocation data.
- `initialize_hunk_processing_logic`: Iterates through a sequence of values to determine how to handle AmigaDOS hunk types.
- `dispatch_hunk_type_callback_2`: Compares a value against AmigaDOS hunk type codes to determine the appropriate callback routine.
- `clear_bitmap_bit_and_scan`: Clears one bit in a bitmap at the position given by a byte index, then runs a loop that calls table routines for each position.
- `stub_truncated_call_sequence`: The pseudo-code for this entry is cut off before any statements, so only its call pattern is known: a long run of table calls and small register moves.
- `identify_hunk_type`: Compares a value against known AmigaDOS hunk type codes used for executable loading or relocation.
- `run_flag_gated_call_sequence`: Makes a table call, then, if a byte flag equals one, makes a long sequence of table calls with values placed on the stack first; other flag values skip that sequence.
- `process_event_dispatch_table`: Reads a sequence of values from a data structure and executes a series of subroutine calls based on a dispatch table.
- `perform_complex_math_and_data_management`: Performs extended-precision floating-point arithmetic, manages memory block allocations and deallocations, and handles record key comparisons.
- `dispatch_on_state_byte`: Calls a table routine, then branches on a state byte with values one through four, calling a different routine for each value.
- `poll_input_until_code_matches`: Makes fixed table calls with constant arguments, then loops calling a table routine and continuing until its result equals 112, the value the loop checks for.
- `initialize_and_poll_input_buffers`: Initializes multiple memory buffers and performs a series of memory copies and allocations.
- `copy_memory_buffers`: Initializes and copies data between multiple source and destination memory locations using various byte-counting and block-copying methods.
- `initialize_and_copy_memory_buffers`: Initializes several memory buffers by copying data between multiple source and destination locations.
- `copy_memory_blocks_3`: Allocates memory and copies data between multiple source and destination locations using various library functions.
- `copy_memory_blocks_2`: Copies data from a source to a destination using memory allocation and library functions.
- `copy_memory_blocks`: Copies a specific number of bytes from a source to a destination after performing several library function calls and register manipulations.
- `execute_polymorphic_dispatch_loop`: Executes a series of subroutine calls with varying arguments and enters a loop that repeatedly calls a routine until a specific return value is met.
- `poll_code_band_and_return`: Loops reading values through a table routine until one falls in a band from 112 to 125, then reads again and returns once the second value is in a narrower range; a few specific high values trigger one extra call before returning.
- `nop_placeholder_routine`: This routine is empty and performs no operations.
- `sum_and_validate_table_ranges`: Iteratively sums values from a table into a destination field until the sum reaches zero.
- `poll_code_band_and_dispatch`: Loops reading values through a table routine until one falls in a band from 128 to 141, then reads again and breaks out when the second value lies between 100 and 157; for certain high values it makes two calls before continuing.
- `nop_padding_sequence`: This routine is empty and contains no executable instructions.
- `teardown_environment_and_cleanup`: Terminates the program by invoking a function pointer, closing a series of library handles in a loop, and preventing task switching.
- `dispatch_event_handler_sequence`: Executes a series of function calls with varying parameters and state flags, performing conditional memory updates and pointer-based assignments.
- `store_result_and_call_hook`: Stores a table call result in a global, and when a register check fails it stores 255 instead; it then calls two routines, one of them only when a function pointer is set.
- `tiny_moveq`: Small routine of 1 instruction that loads 69 into d0.
- `execute_cleanup_and_callbacks`: Updates a global variable, performs conditional memory writes, and executes a series of function pointers and cleanup loops.
- `run_four_pass_cleanup_loop`: Stores a global value, calls a function pointer when one is set, calls a routine that fetches a global, and then repeats a second routine call in a countdown loop of four passes.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a jump table.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `format_signed_number_text`: Scans a buffer until it finds a 127 marker, then writes a 16-bit value as a signed decimal number of up to three digits into fixed memory bytes, using a minus sign for negative values and a space otherwise.
- `shutdown_libraries_and_memory`: Tears the program down: it calls a function pointer when one is set, closes a set of library handles in a four-pass loop, forbids task switching, frees the memory block at a stored address and replies to the startup message.
- `stub_bad_instruction_data`: Stub that the decompiler could not decode: it holds bad instruction data, so no behaviour can be recovered from it.
- `call_routine_hundred_times`: Calls one routine one hundred times in a countdown loop, then returns.
- `clear_active_slot_table`: Makes three setup calls, then for each of the sixteen slots whose entry is non-zero calls a routine; afterwards it compares a stored value with a field and clears two fields when they match.
- `release_slot_and_clear_entries`: Clears a slot entry in a table of sixteen, resets two fields when a stored value matches, then walks the sixteen entries and does further work for each non-zero one.
- `call_routine_eight_times`: Calls one routine eight times in a countdown loop, then returns.
- `save_stack_and_register_globals`: Stores an incoming stack value, a register value and a stack address into three global variables and returns.
- `set_mask_bits_in_byte`: Returns its byte argument with the bits of the constant 117 set.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `tiny_nop_nop`: Small routine of 2 instructions that does nothing for one instruction, then does nothing for one instruction.
- `clear_flag_table_entries`: Iterates through a table to clear indexed flag entries for one hundred consecutive indices.
- `clear_indexed_flag_table_entry`: For an index below one hundred, reads a flag byte from a table; when it is set, clears that entry and a companion byte, then calls routines depending on the old flag value and the companion byte.
- `clear_record_field_if_set`: Reads the first field of a six-byte record selected by an index; when it is non-zero it clears the field and calls a routine.
- `flush_pending_state_and_call`: Calls two routines when a pending flag is set, clears two state fields, then calls a routine on the old value and returns the result of a call through a global pointer.
- `clear_slot_pointer`: This routine is empty.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `close_device_handle`: Closes a device through a function pointer stored in a global variable at a fixed offset, then returns.
- `call_stored_library_function`: Calls one library function at a fixed offset from a library base read from a global variable, then returns.
- `copy_words_with_alignment_adjust`: Checks that the sum of two 16-bit fields is non-negative, then copies 16-bit words between two memory blocks, first moving the pointers by one word when the source address is odd.
- `compare_byte_strings_until_mismatch`: Compares two byte strings one byte at a time for up to a counted length, stopping at the first mismatch.
- `return_minus_one_after_call`: Makes one routine call and returns the constant minus one as its result.
- `link_fixed_node_into_list`: Checks the requested size against a stored limit, and when it fits links a new node with a fixed header into a list and calls a routine through an indirect pointer; otherwise it stores a result in a global.
- `call_library_when_positive`: Calls a routine and, when its result is positive, calls one more function through a pointer read from a global base, then returns.
- `dispatch_optional_handler_or_error`: Calls a handler through a stored pointer when one is set and returns its result; otherwise it stores 75 in a global and reports the error through a routine.
- `mask_value_by_range_table`: Returns its input unchanged below a boundary, and for larger values returns it ANDed with a constant read from a fixed table, selected by the distance from the boundary.
- `select_masked_or_unchanged_value`: Returns the input value unchanged or a masked table value depending on a range check; when the second argument is negative it calls routines and compares their results before returning.
- `check_index_bounds_or_fail`: Checks a signed index against a range and returns its arguments unchanged inside it.
- `check_record_bounds_or_fail`: Checks a record's counter against a range and returns its fields unchanged inside it.
- `link_parameter_node_into_list`: Checks the requested size against a stored limit, and when it fits links a new node holding two parameters into a list and calls a routine through an indirect pointer; otherwise it stores a result in a global.
- `lookup_slot_position_by_key`: Returns the entry stored at an index below sixteen; for larger keys it searches a table of sixteen entries for the key and returns its position, or zero when the key is absent.
- `lookup_slot_value_by_key`: Returns the entry stored at an index below sixteen; for larger keys it searches a second table of sixteen entries for the key and returns the position where it matches, or zero when it is absent.
- `call_if_any_field_set`: Returns at once when two counter fields are both zero; otherwise it calls one function through a pointer read from a global base.
- `call_conditional_function`: Checks if two integer values at specific offsets in a structure are zero before calling a function via a pointer.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `copy_words_with_alignment_adjust_2`: Checks that a 16-bit field plus four is non-negative, then copies 16-bit words between two memory blocks, first moving the pointers by one word when the source address is odd.
- `forward_single_routine_call`: Calls one routine and returns, with no other work.
- `uppercase_copy_of_string`: Copies a counted string from one buffer to another, converting lowercase letters, including the Latin-1 accented range, to uppercase; empty strings return at once.
- `link_byte_node_into_list`: Checks a requested size against a stored limit, then links a new 12-byte node holding a byte and a long word into a list and calls a routine through an indirect pointer; otherwise it stores a result in a global.
- `call_library_function`: Calls a function using a pointer retrieved from a structure offset.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_indirect_library_function`: Fetches a function pointer from a table located at an offset within a structure and executes it.
- `call_do_io_request`: Calls the exec DoIO function through a pointer read from a global base, then returns.
- `convert_digit_to_integer`: Converts an ASCII character representing a decimal digit into its corresponding integer value.
- `add_with_carry_chain_sum`: Adds several shifted copies of its two register inputs using carry-propagating arithmetic and returns the combined total.
- `return_fixed_64bit_constant`: Returns a fixed 64-bit value in which only the highest bit is set.
- `compute_block_address_or_error`: Writes a fill byte, then when a stored count is positive computes an address from a base and a scaled count and returns it; otherwise it stores 15 in a global and reports an error.
- `compute_block_address_from_count`: Stores an address calculated from a base pointer and a count multiplied by four, less a fixed offset, then returns the scaled count added to the low word of the base.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `open_libraries_and_init_state`: Initialises program state and calls library vectors for opening libraries and resources and for allocating memory, then returns an integer status.
- `compute_request_offset_difference`: Reads a 16-bit field from either the current request block or a stored one, subtracts a stored offset from it, and returns the result.
- `read_button_state_bits`: Converts three button bits from a global copy of the input register into a small value: one for the fire button, two for the second button and four for the right or middle button; each is added when its bit reads zero.
- `return_result_or_report_error`: Returns the value of a routine call when it is non-zero; otherwise it stores 71 in a global, reports the error through the same path its neighbours use, and returns.
- `lookup_slot_value_by_key_2`: Returns the entry stored at an index below sixteen; for larger keys it searches a table of sixteen entries for the key and returns the position where it matches, or zero when it is absent.
- `call_offset_function`: Calls a function located at an offset within a pointer retrieved from a structure.
- `call_offset_function_2`: Retrieves a pointer from a structure and calls a function at a specific negative offset from that pointer.
- `call_offset_function_3`: Calls a function using a pointer found at a specific offset within a structure.
- `setup_library_calls_with_flags`: Stores a byte value combined with 99 into a global, then makes a long sequence of library calls at fixed offsets, routing each call through a pointer when one is set.
- `compute_scaled_difference_value`: Computes a 16-bit value from the difference of two fields: when a counter is at least two it divides a weighted difference by three for small counters or shifts a weighted difference right by two otherwise; below two it halves the plain difference.
- `init_four_word_record`: Stores zero into the first and third fields of a four-field record, a fixed constant into the second and zero into the fourth.
- `decrement_counters_wrap_by_two`: Decrements each of ten 16-bit counters, adding two back when a counter goes negative, then points a link field at a fixed offset inside the same structure.
- `init_five_field_from_table`: Stores a value from a fixed constant table and values derived from a counter plus one into a five-field record.
- `clear_twenty_two_bytes`: Writes zero to 22 consecutive bytes starting at the given pointer, then returns.
- `copy_text_lines_until_bar`: Copies characters from a source text into a buffer row by row, ending each row at a vertical bar character and stopping when the row or character counter runs out.
- `return_from_subroutine_3`: This routine performs a single instruction to return control to the calling function.
- `reply_pending_port_messages`: Calls a library routine, then checks two stored fields and, when they are set, calls further routines to reply to the pending message and clear them.
- `compare_two_records_by_value`: Compares two records by a signed 16-bit field, then by a 32-bit value and a 16-bit tie-break, and returns the first record's value when it orders first.
- `execute_cleanup_sequence`: Calls a routine that updates a global variable, performs conditional memory writes, and executes a series of function pointers and cleanup loops.
- `store_value_and_minus_four`: Stores its register argument into the first long word of a record and the constant minus four into the following 16-bit field.
- `call_offset_function_4`: Calls a function via a pointer stored at a specific offset within a structure.
- `tiny_ori_ext_ext`: Small routine of 3 instructions that merges 64 into d0, then sign-extends d0, then sign-extends d1.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `copy_string_into_buffer`: Stores a call result into a structure field, then copies a zero-terminated string into a fixed buffer and appends a zero terminator.
- `call_library_vector_534`: Library stub: loads a library base from a global variable and calls the library function at offset minus 534 of that base, returning its result to the caller.
- `tiny_move_move`: Small routine of 2 instructions that copies (a0)+ to d0, then copies (a0)+ to d2.
- `scale_value_by_exponent`: Shifts a 64-bit fixed-point value right or left by an amount derived from a 16-bit exponent field; values below a threshold pass through unchanged.
- `call_get_msg_routine`: Calls the exec GetMsg function through a library base read from a global variable, then returns.
- `process_sixteen_pending_entries`: Loops over sixteen table entries, and for each non-zero one calls a routine with the entry's two stored values when the first routine returns true.
- `call_offset_function_5`: Uses a pointer from a structure to call a function at a negative offset.
- `store_result_and_call_library`: Stores a routine result into two fields of a record and then calls a library function at a fixed offset.
- `pack_args_and_check_size`: Stores its register arguments in a stack record, calls a routine to get a size, and returns that size rounded up to an even value; for sizes in a set range it makes a further call.
- `pack_args_and_check_size_2`: Stores its register arguments in a stack record, calls a routine to get a size, and makes a further call for sizes in a set range before returning the size.
- `remove_port_and_free_memory`: Calls one routine, then removes a message port through the exec library and frees a memory block through a function pointer.
- `reset_slot_table_minus_one`: Clears one long field, sets two 16-bit fields to fixed values, then fills eight long words in a table with minus one and stores a further constant.
- `call_routine_then_library_function`: Calls a routine and then calls a library function at a fixed offset from a library base read from a global variable.
- `call_offset_function_6`: Calls another routine and then executes a function at a negative offset from a pointer in a structure.
- `execute_conditional_library_function`: Checks three flag fields in a structure and, if all are zero, executes a function retrieved from a library base pointer stored within that same structure.
- `set_flag_byte_from_result`: Calls one routine and stores eight into a flag byte when its result is non-zero, otherwise zero.
- `compare_register_pairs_for_match`: Compares two pairs of register arguments; it returns zero when they match and minus one when any pair differs.
- `update_table_entry_and_scan`: Writes a constant into a table entry and increments a neighbouring byte, then runs a loop of table lookups; the decompiler did not fully recover the loop.
- `update_table_entry_and_scan_2`: Writes the input byte into a table entry and increments a neighbouring byte, then runs a loop of table lookups; the decompiler did not fully recover the loop.
- `reset_byte_and_word_fields`: Clears one byte field and sets a 16-bit field in the same record to minus four.
- `jump_through_device_vector`: Loads a function pointer from a global structure at a fixed offset and calls it, with no other logic.
- `copy_block_or_report_error`: When the length argument is non-negative, copies a block from source to destination, handling odd alignment before moving words and then single bytes.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_rem_port`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `poll_records_until_match`: Repeatedly fetches a record from an unnamed routine until one has a type field of 2 and a stored value equal to the argument.
- `store_value_into_struct_field`: Calls a library function, writes a register value into a fixed field of the structure pointed to by the address register, then calls another routine.
- `call_handler_library_function`: Loads a library base pointer from a fixed structure field and calls the function at a fixed negative offset from it.
- `compare_stored_pair_ordering`: Compares a stored 32-bit value and a 16-bit value with two register arguments, using the sign of a size argument to choose the comparison direction.
- `claim_free_slot_or_dispatch`: When a flag byte is set, stores a byte from the source into the destination and returns the routine result.
- `store_byte_via_routine`: Calls one unnamed routine, then stores a byte argument into the location pointed to by the address register.
- `advance_string_pointer_measure`: Scans forward through a zero-terminated string from a pointer stored in a structure, writes the advanced pointer back and returns the number of characters consumed.
- `copy_bytes_counted`: Calls an unnamed routine, reads a 16-bit count from its result and copies that many bytes one at a time from the source register to the destination register.
- `copy_memory_block`: Copies a block of memory from the address in a2 to the address in a1 one byte at a time, using a dbra loop counter, and returns.
- `call_two_setup_routines`: Calls two unnamed routines in order and returns.
- `shift_value_by_exponent`: Derives a shift amount from a 16-bit exponent argument and returns a shifted 32-bit value, negated when the exponent is negative.
- `call_library_function_via_base`: Loads a library base pointer from a fixed structure field and calls the function 48 bytes before it.
- `call_add_port`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `copy_stored_value_into_field`: Calls a library function, reads a stored 32-bit value and writes it into a fixed field of the structure pointed to by the address register, then calls more unnamed routines.
- `copy_bytes_by_length_result`: Obtains a 16-bit count from an unnamed routine and copies that many bytes one at a time from the source register to the destination register.
- `move_list_node_or_copy`: When the source and destination nodes differ and the source lies in the stack range, moves the node links and size to the destination and clears the source size byte.
- `dispatch_library_function_slot`: Loads a library base pointer from a fixed structure field and calls the function 90 bytes before it.
- `copy_block_then_store_word`: When the length argument is valid, copies a block with alignment handling from source to destination, then stores the 16-bit argument at the end of the destination.
- `forward_call_to_routine`: Calls a single unnamed routine and returns its result unchanged.
- `clear_indexed_entry_and_jump`: Calls an unnamed routine, zeroes the 32-bit entry at the index held in a register, then makes an indirect call through a pointer whose target is not resolved.
- `clear_indexed_word_and_jump`: Calls an unnamed routine, zeroes the 16-bit entry at the index held in a register, then makes an indirect call through a pointer whose target is not resolved.
- `spin_delay_loop_then_call`: Runs an empty countdown loop from a register count, then makes an indirect call through a pointer whose target is not resolved.
- `record_error_and_call_handlers`: Stores a fixed error code in a global, calls the reporting routines, runs a loop of four calls through a function-pointer table and then makes a final library call.
- `store_code_and_dispatch_error`: Stores a byte argument as the error code in a global, then runs the common reporting sequence: reporting calls, a four-call table loop and a final library call.
- `raise_fixed_error_code_89`: Stores a fixed error code of 89 in a global and then runs the common reporting sequence of calls, a four-call table loop and a final library call.
- `store_saved_value_in_struct`: Pushes a function pointer, calls two unnamed routines, then writes a stored 32-bit value into a fixed field of the structure pointed to by the address register before calling another routine.
- `raise_fixed_error_code_71`: Sets a fixed error code of 71 in a global and runs the common reporting sequence of calls, a four-call table loop and a final library call.
- `raise_fixed_error_code_123`: Sets a fixed error code of 123 in a global and runs the common reporting sequence of calls, a four-call table loop and a final library call that uses the 390-byte library offset.
- `pass_through_single_call`: Calls one unnamed routine and returns without any other work.
- `invoke_library_vector_slot`: Loads a library base pointer from a fixed structure field and calls the function 348 bytes before it, with no other logic.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `store_result_and_two_words`: Calls an unnamed routine and stores its 32-bit result plus two 16-bit register values into consecutive fields of the structure pointed to by the address register.
- `normalize_long_to_float_parts`: Normalizes a 32-bit integer for floating-point style storage by shifting its magnitude until the top bit is set, while keeping a running exponent count.
- `normalize_word_discard_result`: Shifts a 16-bit argument left in a bounded loop until its sign bit is set, then returns zero whatever the shifted value was.
- `convert_word_to_float_parts`: Converts a 16-bit signed argument into a normalized 64-bit value by shifting its magnitude until the top bit is set from a fixed starting exponent.
- `call_library_function_390`: Calls a library function through a stored library base at a fixed offset of 390 bytes.
- `store_record_or_report_overflow`: When the address register plus six lies below a stored limit, writes a 32-bit argument and a zero word into the record and makes an indirect call.
- `allocate_list_node_or_fail`: If enough space remains below a limit, initializes a list node with zeroed fields, links it in front of the list head and sets its size to ten.
- `clear_byte_test_then_restore`: Saves a byte at an address built from a base and a 16-bit offset, clears it, then calls an unnamed routine.
- `free_memory_via_exec_library`: Frees a memory block by calling exec.FreeMem through the stored exec library base.
- `store_result_and_words_backward`: Calls an unnamed routine, then writes its 32-bit result and two 16-bit register arguments into the structure at the address register using negative offsets.
- `multiply_extended_float_values`: Multiplies two extended-precision values by combining 16-bit partial products of their mantissas, then normalizes the 64-bit product.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d4.
- `multiply_float_after_helper_call`: Calls an unnamed routine first, then multiplies its extended-precision result by a second mantissa using 16-bit partial products.
- `tiny_move_move_move`: Small routine of 3 instructions that copies (a0)+ to d3, then copies (a0)+ to d4, then copies (a0)+ to d5.
- `multiply_float_extra_word_term`: Multiplies two extended-precision mantissas and also adds a 16-bit operand term to the partial products before normalizing the result.
- `multiply_float_with_library_call`: Multiplies an extended-precision value by a 16-bit operand, normalizes the product and returns it.
- `copy_block_or_claim_slot`: When a flag byte in the structure is set, copies a block whose length comes from an unnamed routine and returns a failure word.
- `store_two_bytes_in_state`: Writes two byte arguments into consecutive fields of a state structure at a fixed offset.
- `dispatch_library_vector_426`: Loads a library base pointer from a fixed structure field and calls the function 426 bytes before it.
- `fill_memory_with_byte_pattern`: Fills a memory area with a repeated byte pattern written as 16-bit words.
- `fill_blank_space_words`: Fills the destination with a pattern of two ASCII space bytes written as 16-bit words.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `clamp_and_set_status_bits`: Takes a packed pair of 16-bit values from an unnamed routine, clamps each to a limit from the arguments and passes them on to another routine.
- `scale_float_by_exponent`: Scales an extended-precision value by a power of two given as a 16-bit exponent, shifting its mantissa and adjusting the exponent.
- `add_extended_float_values`: Adds or subtracts two extended-precision values depending on their signs, first shifting the smaller operand's mantissa to align exponents.
- `add_float_mantissas_normalized`: Adds or subtracts two extended-precision values by aligning their mantissas, then normalizes the result.
- `add_packed_float_operands`: Adds or subtracts two extended-precision operands with exponent alignment and normalization, and returns the high 32-bit half of the packed result.
- `add_float_values_wide_operand`: Adds or subtracts two extended-precision values through a wider operand path, aligning their mantissas and normalizing the result.
- `find_table_index_or_fail`: When a stored value is nonzero, scans a table of 32-bit entries for the first entry equal to it and returns that position.
- `link_record_or_report_overflow`: When there is room below the limit, links a four-word record into the list at the address argument, sets its size field to four and makes an indirect call.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `tiny_exg`: Small routine of 1 instruction that exchanges d1 and d0.
- `allocate_memory_via_exec`: Allocates a memory block by calling exec.AllocMem through the stored exec library base.
- `dispatch_library_function_pair`: Loads a library base pointer from a fixed structure field and calls the function 48 bytes before it, with no other logic.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `link_record_at_list_head`: When there is room below the limit, initializes a record with the word argument, links it in front of the list at the address argument and sets its size field to six.
- `offset_position_pair_checked`: Adds two 16-bit offsets to a stored pair of coordinates and returns both.
- `offset_position_pair_and_notify`: Adds two 16-bit offsets to a stored pair of coordinates and returns both.
- `call_routine_if_flags_clear`: Calls an unnamed routine only when three flag fields in the structure are all zero.
- `manage_linked_list_nodes`: Traverses a linked list to find a matching node, allocates memory, and updates list pointers.
- `lookup_cached_library_by_name`: Compares a name argument against up to five zero-terminated names in a table and returns the cached value for the first match, filling the cache on first use.
- `open_library_via_exec`: Opens a library by calling exec.OpenLibrary through the stored exec base.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `clear_and_search_slots`: Clears specific fields in a structure and searches through a table of entries to match a value, potentially freeing memory or calling a function via a pointer.
- `compare_record_key_order`: Stores a key pair and a pointer into a new record when there is room, then compares that key with an existing record and returns zero or minus one.
- `compare_record_key_pairs`: Compares two records by a 32-bit key and a 16-bit key, with the sign of the size argument deciding the direction.
- `compare_record_against_key`: Compares a 32-bit key and a 16-bit key from an unnamed routine's result against a stored record.
- `store_state_tag_minus_two`: Increments seven state registers when one flag field is zero, stores them in a state block and multiplies a sequence of 16-bit values.
- `store_state_tag_minus_one`: Increments seven state registers when one flag field is zero, stores them in a state block and multiplies a sequence of 16-bit values.
- `store_state_tag_zero`: Increments seven state registers when one flag field is zero, stores them in a state block and multiplies a sequence of 16-bit values.
- `store_state_tag_one`: Increments seven state registers when one flag field is zero, stores them in a state block and multiplies a sequence of 16-bit values.
- `store_state_tag_two`: Increments seven state registers when one flag field is zero, stores them in a state block and multiplies a sequence of 16-bit values.
- `negate_routine_result`: Calls one unnamed routine, then calls a second routine and returns the negation of its result.
- `relocate_table_entries_by_offset`: Walks a table of records from its top and, for each entry with a nonzero size code, adds the offset returned by an unnamed routine to the 32-bit pointer fields that refer into the table range.
- `relocate_table_entries_by_delta`: Walks a table of records from its top and adds the register argument to the 32-bit pointer fields that refer into the table range.
- `relocate_pointer_list_by_delta`: Stores the address argument as the list cursor, then walks a list of pointers and adds the register delta to each pointer that falls inside the table range.
- `relocate_pointer_list_downward`: Walks a pointer list and subtracts the register offset from pointers that fall inside the table range, copying entries down as it goes.
- `shift_entries_backward_with_patch`: Subtracts a delta from a pointer entry, then walks the list backward, copying each block toward the destination and patching in-range pointers until the list start is reached.
- `compact_blocks_backward`: Walks a list backward from the given address, copying each block toward the destination and updating pointers that fall inside the table range.
- `compact_blocks_backward_subtract`: Subtracts a delta from a pointer and walks the list backward, copying blocks and patching in-range pointers until the list is exhausted.
- `copy_counted_bytes_after_setup`: Calls two unnamed routines, takes a 16-bit count from the second result and copies that many bytes one at a time from the source register to the destination register.
- `manage_bitfield_and_memory`: Checks a value in a structure, performs memory operations, and clears a bit in a bitfield based on a table lookup result.
- `copy_block_then_store_byte`: Copies a block with alignment handling, moving word and byte steps, then stores a single byte argument at the end of the destination.
- `add_delta_to_pointer_range`: Adds a delta to each 32-bit entry from the given address up to the table end.
- `copy_backward_aligned`: Copies a block backward from the end pointers, selecting byte, word and longword steps from the low bits of the count so that overlapping blocks copy safely.
- `subtract_delta_from_pointer_range`: Subtracts a delta from each 32-bit entry from the given address up to the table end.
- `copy_forward_aligned_by_bits`: Copies a block forward from the source and destination pointers, selecting byte, word and longword steps from the low bits of the count so that the copy is aligned.
- `shift_extended_value_in_place`: Shifts a packed extended-precision value stored at the address argument by a count held in a 16-bit field and writes the result back.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `find_slot_index_for_key`: For keys below 16 it returns the slot value at that index when it is nonzero.
- `copy_block_direction_aligned`: Copies a block of a given length, choosing forward or backward direction from the relative addresses of the source and destination.
- `offset_coordinate_list_points`: Adds two stored 16-bit offsets to each coordinate pair in a list of four-word entries, storing the results in the structure.
- `resolve_record_key_or_load`: Returns the key directly when it is below 16 and has no table entry.
- `swap_record_state_and_clear`: Saves a pointer field in the structure, runs a sequence of unnamed routines, then restores the pointer.
- `execute_conditional_calls`: Retrieves a value from a nested pointer and, if non-zero, calls four different routines sequentially.
- `store_four_words_and_notify`: Stores four 16-bit register arguments into consecutive slots of the structure and then calls three unnamed routines.
- `clear_structure_fields`: Checks a value at an offset within a structure and, if non-zero, clears three consecutive longwords and calls a function via a pointer from a global variable.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `check_five_entries_or_fail`: Checks a five-entry table of 16-bit values and returns the given values when all five are nonnegative.
- `store_value_in_state_slot`: Stores the register argument into a fixed 32-bit field of the state structure.
- `multiply_result_by_word_high`: Calls an unnamed routine and returns the high 16 bits of its low word multiplied by a 16-bit operand.
- `toggle_stored_flag_value`: Calls an unnamed routine to initialise a stored flag word when it is zero, then stores one minus a second routine result there.
- `multiply_signed_fixed_point`: Multiplies two 32-bit values by combining their 16-bit halves with cross terms.
- `multiply_fixed_point_halves`: Multiplies two 32-bit values by combining their 16-bit halves with cross terms and returns the 32-bit product.
- `post_request_when_owner_matches`: Returns early unless the stored owner pointer equals the argument.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `post_request_with_arguments`: Waits until a stored value is set, then writes the register and address arguments into the associated request record with status three and calls a library function at a fixed offset.
- `tiny_movea`: Small routine of 1 instruction that copies d0 to a0.
- `dispatch_library_function_450`: Loads a library base pointer from a fixed structure field and calls the function 450 bytes before it.
- `format_float_as_decimal_text`: Writes a sign character and then converts an extended-precision value into decimal digits, scaling it by powers of ten and writing the result to a text buffer.
- `format_scientific_text_digits`: Writes the digit characters of a value into a text buffer and appends an exponent marker with a sign.
- `format_float_text_fixed_point`: Builds printable text from a digit buffer and an exponent, using fixed notation with a decimal point and trimming trailing zeros when the exponent is in range.
- `dispatch_library_function_330`: Loads a library base pointer from a fixed structure field and calls the function 330 bytes before it, with no other logic.
- `compute_offset_from_record_field`: Reads a 16-bit coordinate from the active record, or from the secondary record when no active record exists, and returns its difference from a stored 16-bit value.
- `invoke_unnamed_routine_only`: Calls one unnamed routine and returns.
- `map_condition_to_minus_one`: Calls an unnamed routine and returns minus one when the zero flag it leaves set is true, or zero otherwise.
- `map_condition_inverse_result`: Calls an unnamed routine and returns zero when the zero flag it leaves set is true, or minus one otherwise.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `dispatch_library_function_90`: Loads a library base pointer from a fixed structure field and calls the function 90 bytes before it, with no other logic.
- `pack_scaled_components`: Calls an unnamed routine and combines its 64-bit result into a 32-bit packed number, scaling the low and high parts by fixed factors.
- `read_library_base_quad_field`: Returns a 64-bit value from a fixed field of the library base referenced by a structure field.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `dispatch_library_function_174`: Loads a library base pointer from a fixed structure field and calls the function 174 bytes before it, with no other logic.
- `tiny_ori_move_rts`: Small routine of 3 instructions that merges 4 into d0, then copies a0 to d0, then returns.
- `release_block_and_return_size`: Returns the size word of a memory block header at the address argument.
- `select_slot_record_by_index`: For an index below 16, stores the index as the active slot and returns the record kept for it.
- `select_slot_record_by_register`: Selects the active slot record from a 16-slot table using a register index, filling an empty slot through an unnamed routine.
- `parse_prefixed_number_text`: Temporarily terminates a text buffer at the given length, skips leading spaces and parses an integer that may carry an ampersand base prefix such as the octal form.
- `initialise_table_region`: When the cursor equals the stored base, stores a size from an unnamed routine as the table base, computes the end pointer and zeroes a region of 16-bit words.
- `reserve_hunk_table_region`: Raises a requested size to at least 1000, and when the cursor matches the stored base it reserves and clears a table region.
- `dispatch_library_function_42`: Loads a library base pointer from a fixed structure field and calls the function 42 bytes before it, with no other logic.
- `open_required_amiga_libraries`: Opens a list of Amiga libraries by name through exec.OpenLibrary, including dos.library, and storing each base in a fixed slot.
- `clear_region_and_build_table`: Clears a region of 16-bit words sized from a global, then walks a zero-terminated byte table and writes entries into the region according to each byte's value and the alignment of the destination.
- `return_immediately_stub`: Returns to the caller without doing anything.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `parse_decimal_number_text`: Parses decimal text from a string pointer, skipping leading spaces, reading an optional sign, fraction digits and exponent, and stores the value as an extended-precision result.
- `parse_decimal_text_entry_two`: Enters the decimal-number parser at a second entry point.
- `parse_decimal_text_entry_three`: Enters the decimal-number parser at a third entry point.
- `parse_decimal_text_entry_four`: Enters the decimal-number parser at a fourth entry point.
- `parse_decimal_text_entry_five`: Enters the decimal-number parser at a fifth entry point.
- `parse_decimal_number_full`: Parses decimal text including sign, fraction digits and exponent, and stores the value as an extended-precision result.
- `copy_bytes_counted_by_routine`: Obtains a count from an unnamed routine and copies that many bytes one at a time from the source register to the destination register.
- `store_byte_after_setup_call`: Calls one unnamed routine, then stores the byte argument at the address register.
- `store_word_after_setup_call`: Calls one unnamed routine, then stores the 16-bit argument at the address register.
- `store_long_after_setup_call`: Calls one unnamed routine, then stores the 32-bit argument at the address register.
- `tiny_moveq_3`: Small routine of 1 instruction that loads 0 into d0.
- `resize_or_allocate_block`: Allocates a block of the requested size from the free area below a stored top pointer and records the size in the block header.
- `allocate_slot_block_from_table`: Finds the first free entry in a table of up to 1000 records, then allocates a block of the requested size below the stored top pointer and records it in that entry.
- `allocate_byte_block`: Allocates a block of memory for a single byte using a table-based allocation routine.
- `allocate_word_block`: Allocates a block of memory for a word using a table-based allocation routine.
- `allocate_long_block`: Allocates a block of memory for a longword using a table-based allocation routine.
- `free_block_and_merge_neighbours`: Walks the block list from the top pointer and merges the given block with adjacent free neighbours, linking the combined block back into the list.
- `compute_free_space_remaining`: Calls an unnamed routine, then returns the distance between the stored top pointer and the address argument, which is the free space remaining.
- `mark_block_free_in_chain`: When the header address lies in the stack-range area, clears its marker byte and, if the block size is nonzero, writes the negated offset into the link word of the chain entry.
- `store_two_strings_with_lengths`: Stores a pointer to the first zero-terminated string argument, measures it and the second string, saves both lengths in the structure and then calls a library function at a fixed offset.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `dispatch_library_function_96`: Loads a library base pointer from a fixed structure field and calls the function 96 bytes before it, with no other logic.
- `relink_slot_record_node`: Computes the six-byte slot record for an index and, when it differs from the source node, moves the node links into the slot.
- `scale_mantissa_exponent`: Performs scaling operations on a mantissa and an exponent.
- `scale_mantissa_by_exponent`: Shifts a 16-bit mantissa argument left until its top bit is set, adjusting the exponent held in the second register.
- `read_slot_record_pair`: Reads the slot record for an index.
- `write_buffer_chunk_or_fail`: Writes data into a chunked buffer and allocates a new chunk through an unnamed routine when the current one is full.
- `append_byte_to_buffer_record`: Appends a byte to a record buffer, incrementing its count and setting a flag bit in the header.
- `adjust_counter_and_clear_slot`: Adds the difference of two register values to a stored counter and clears a two-word record.
- `clear_slot_and_dispatch_library`: Stores a fixed sentinel value in a record field and clears a slot in the state table.
- `update_pointer_offset_or_fail`: Adjusts a stored 16-bit pointer offset inside a record when the new value fits its range, and otherwise recomputes the record and calls unnamed routines.
- `store_slot_result_or_fail`: Clears the first word of the record, calls unnamed routines with a slot value, and stores the result word in the record when the value is nonnegative.
- `raise_error_with_routine_code`: Runs the common error-reporting sequence with the error code produced by an unnamed routine.
- `release_record_if_matching`: Clears the record's links and marker bit, calls an unnamed routine with its size and, when the result equals the stored size, adds the result to a field and returns the stored pair.
- `store_count_or_report_error`: Clears the first word and calls an unnamed routine.
- `check_state_word_or_report`: Returns the register pair when the sign flag is clear and the state word is set.
- `return_from_call`: This routine is empty and performs no operations before returning to the caller.
- `raise_error_code_26`: Runs the common error-reporting sequence with error code 26.
- `accumulate_result_or_report`: Clears the first word and calls an unnamed routine.
- `free_linked_list_node`: Traverses a linked list to find a specific node and frees it using a library function.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d2.
- `dispatch_library_function_84`: Loads a library base pointer from a fixed structure field and calls the function 84 bytes before it, with no other logic.
- `copy_node_fields_checked`: Copies a block from the pointer argument into a new node when there is room below the limit and the argument lies outside the stack-range area.
- `multiply_signed_16bit_operands`: Multiplies two 16-bit operands after normalising their signs, combining partial products into a 32-bit result.
- `multiply_unsigned_partial_products`: Multiplies two unsigned operands by ordering them by size and combining their 16-bit partial products into a 32-bit result.
- `wait_for_port_message`: Calls exec.WaitPort through the stored exec library base to block until a message arrives on the port.
- `call_reply_msg`: Library stub: loads its arguments and the exec library base, calls ReplyMsg and returns its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `check_count_sign_for_status`: Checks a counter argument's sign and zero state.
- `check_zero_count_status`: Calls an unnamed routine.
- `read_pair_or_report_status`: Reads a 32-bit value and a 16-bit value from the record and passes the length field to an unnamed routine.
- `read_record_pair_for_length`: Reads a 32-bit value and a 16-bit value from the record and checks a length argument.
- `tiny_exg_exg_exg`: Small routine of 3 instructions that exchanges d3 and d0, then exchanges d4 and d1, then exchanges d5 and d2.
- `process_record_for_length`: Updates a record for a length argument by calling an unnamed routine.
- `create_flagged_record_or_fail`: Creates a record from the register arguments, storing its flag word and linking the previous pointer, and sets two extra fields when flag bit 11 is set.
- `store_clipped_rectangle_record`: Stores a four-word rectangle and a 32-bit extra value in the record, then clips its extent against limits taken from a second structure.
- `load_state_from_slot_entry`: Finds the table slot whose pointer equals the argument, then copies four stored 32-bit fields from that slot into the state block.
- `compute_record_extent_fields`: Reads position and size fields from the active record, falling back to a default record when none is set, and computes a clipped extent.
- `copy_text_template_into_buffer`: Stores four 16-bit arguments in the structure, then copies a 19-word template into a buffer chosen from the structure.
- `store_pair_and_mark_flag`: Stores values returned by unnamed routines into state fields and sets flag bit 0 in a state byte when they are nonzero.
- `clear_device_status`: Processes messages, sets a status bit in a structure, and clears several longword fields.
- `create_record_or_report_failure`: Creates a record through an unnamed routine and stores the state value in its field, returning the record.
- `set_console_request_fields`: Stores a request pointer and a type value of four in an I/O request, calls an unnamed routine and copies two request fields into the state block.
- `clear_status_bit`: Clears a specific bit in a byte located at an offset from a pointer and calls a library function.
- `store_state_value_in_result`: Calls an unnamed routine and, when its result is nonzero, stores the state value into a fixed field of that result.
- `set_names_and_dispatch_library`: Chooses a record from the state, or a fallback record when none is set, stores two string pointers in it and calls a library function at a fixed offset of 282 bytes.
- `build_48_byte_record_chain`: Allocates a block through an unnamed routine and fills consecutive 48-byte records with a type code of five.
- `set_request_type_and_dispatch`: Sets a type of two and a state pointer in a request record, marks its count as one and calls a library function at a fixed offset of 462 bytes.
- `set_request_type_after_calls`: Calls unnamed routines first, then sets a type of two and a state pointer in a request record, marks its count as one and calls a library function at a fixed offset of 462 bytes.
- `find_table_entry_pointer`: Scans a table of 32-bit pointers for the argument and stops at the matching entry.

### d

- `null_routine`: The routine is empty and performs no operations.

### 17

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `init_chipset_registers_and_table`: Reads chipset status registers, writes the interrupt enable register with a mask and fills a table with fixed word values.

### Manager.adf:man.int

- `init_chipset_colour_and_table`: Reads chipset status registers, writes the interrupt enable register and sets the COLOR00 colour register while filling a table with fixed word values.
- `decode_variable_length_codes`: Runs a bit-controlled loop that fetches values from an unnamed routine and sums them while each fetched value equals three.
- `return_if_shifted_bits_zero`: Tests a shifted bit count and returns in both cases.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `read_bits_from_stream`: Shifts bits out of a bit buffer one at a time, fetching a new byte from the source pointer when the buffer empties, and repeats for a count held in a register.

### diskfont.library

- `store_argument_if_obtained`: Library code that calls a routine through the library base to obtain a pointer.
- `store_argument_if_obtained_2`: Calls a library routine through the library base and, when the returned pointer is non-zero, writes the argument at a fixed offset in that structure and calls a second library routine.
- `open_libraries_and_init_list`: Opens the two libraries named in its text, dos and graphics.
- `purge_idle_list_entries`: Walks a linked list, unlinking each node whose reference count is not positive, releasing it through a library call and decrementing a count held in the structure.
- `release_if_unreferenced`: Checks whether a structure has no outstanding references and a zero count.
- `set_pending_release_flag`: Sets the high bit of a flag byte at a fixed offset in the structure and returns zero.
- `return_without_action`: An empty routine that only returns, referenced as a handler pointer from the library's data.
- `clear_flag_and_increment_counter`: Clears the high bit of a flag byte at a fixed offset in a structure and increments a 16-bit counter at another fixed offset.
- `decrement_counter_and_release`: Decrements a 16-bit counter at a fixed offset in the structure.
- `return_constant_zero`: Returns zero with no other work.
- `compute_scaled_offset`: Computes a signed integer from two structures: a scaled difference between one 16-bit field in each, then for each of seven bit positions in two flag bytes where the bits differ it subtracts a weight from a global table.
- `load_and_install_disk_font`: Searches the FONTS: location for a font file, reads and checks its header, allocates a table of candidate entries and selects the highest-scoring one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `load_dos_segment`: Thin wrapper that calls the DOS library's segment loader through the library base and has no other work.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `allocate_exec_memory`: Thin wrapper that calls the exec library's memory allocator through the library base.
- `free_exec_memory`: Thin wrapper that calls the exec library's memory release function through the library base to free a block passed in by the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `copy_null_terminated_string`: Copies a zero-terminated byte string from the source buffer to the destination, including the terminator.
- `append_null_terminated_string`: Appends a zero-terminated source string to the end of a destination string by moving to the destination's terminator and then calling the string copy routine.
- `advance_to_terminator_and_call`: Advances a pointer to the terminating zero of a zero-terminated string and passes that pointer to a library stub that calls the exec memory allocator.
- `count_string_length`: Returns the number of bytes before the terminating zero of a zero-terminated string.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte.
- `compare_strings_return_on_end`: Compares two zero-terminated strings passed in address registers.
- `compare_strings_by_register_pointers`: Compares two zero-terminated strings addressed by two pointer registers.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_first_matching_character`: Scans a zero-terminated string from its start for the first occurrence of a given byte and returns a pointer to it.
- `scan_string_for_register_byte`: Scans a zero-terminated string from a pointer held in an address register for a byte held in a data register, and returns a pointer to the first match.
- `find_next_matching_byte`: Advances a pointer past its current position and returns the next location holding a given byte.
- `find_last_occurrence_in_string`: Finds the last occurrence of a given byte in a zero-terminated string by measuring its length and scanning backwards from the end.
- `find_delimiter_and_advance_pointer`: Looks for a slash character in a zero-terminated path and, if it is absent, for a colon.
- `build_available_font_entries`: Fills a caller-supplied array with font entries, first copying entries from the in-memory font list.
- `append_memory_font_entries`: Walks the in-memory font list and appends each entry's name and attributes to the caller's array.
- `scan_font_names_into_array`: Walks a list of font name strings byte by byte, copying each name and its attributes into the caller's array.
- `scan_fonts_directory_entries`: Locks and examines the FONTS: directory and walks its entries, appending each font file's name and attributes to the caller's array.
- `scan_dot_font_files`: Walks a list of directory entries and selects those whose name ends with the font file extension.
- `scan_directory_handle_entries`: Walks a directory listing from an open lock handle, copying each entry name into the working buffer and appending matching font entries to the caller's array.
- `read_font_names_from_file`: Reads bytes from an open file in chunks and assembles them into name strings in the working buffer.
- `walk_directory_collecting_fonts`: Walks the entries of a locked directory with a next-entry call, collecting the names of font files and reading each file's header to append matching entries to the caller's array.
- `return_zero_stub`: Returns zero and performs no other operations.
- `build_font_table_from_directory`: Locks the directory named by its first argument and counts its entries.
- `load_disk_font_table`: Counts the entries of a font directory listing, allocates a table with a header and a zero count, and copies each matching font entry's name and metric bytes into it while incrementing the count.
- `forward_stack_pointer_argument`: Reads a pointer and its argument from the stack and passes both to another library routine.
- `forward_call_without_arguments`: Calls one other library routine with no arguments and then returns, with no other work.
- `forward_call_without_arguments_2`: Calls one other routine with no arguments and then returns, with no other work.
- `forward_call_without_arguments_3`: Calls one other routine with no arguments and then returns, doing nothing else.
- `forward_call_without_arguments_4`: Calls one other routine with no arguments and then returns, doing nothing else.

### Manager.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_copper_text_loop`: Clears a block of memory, fills a copper list with colour and bitplane words, and programs the blitter and DMA registers.
- `fill_three_random_offset_tables`: Fills three tables of one hundred entries each with starting values, adding a value from the random generator to a running position as it goes.
- `advance_random_seed_modulo`: Advances a seed value by doubling it and XORing with a fixed constant when the shift overflows, then stores the seed reduced modulo a range limit as a 16-bit result.
- `compute_cumulative_offset_table`: Derives a base value from a stored count and the largest of several 16-bit fields, then fills fourteen consecutive 32-bit table entries with a running sum.
- `reset_color_register_state`: Sets four consecutive global locations to zero and one further location to fifteen.
- `update_audio_channels_per_frame`: Writes the register words of the four audio channels from their per-channel state records.
- `process_sound_channel_commands`: For each of four sound channels, reads the next command from its sequence table and applies it according to the low nibble, such as setting a volume or period, adding or subtracting a pitch offset, or storing a byte.
- `load_channel_descriptor_state`: Clears an active flag in a channel record, copies eight longwords of saved state from a table entry into the record's buffer, and sets the record's fields from a per-entry descriptor table.
- `store_table_value_by_index`: Looks up a 16-bit value in a table using a byte index doubled as the offset, and stores it at a fixed offset inside the structure passed in the second address register.
- `service_flagged_channels`: Walks four channel records and calls the channel worker for each record whose active flag is set.
- `advance_channel_timed_updates`: For one channel, decrements per-step countdowns and, when each one reaches zero, reloads it from the channel's sequence table and applies a table-driven change to a volume, period or buffered data value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_without_action_2`: Empty routine that only returns and does nothing else.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `loop_forever_without_exit`: Unreached routine whose only body is an endless loop with no exit.
- `empty_routine`: This routine contains no executable instructions.
