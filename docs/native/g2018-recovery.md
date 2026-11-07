# Game 2018 (populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02018). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802304 top-level bytes in 47 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 15 raw-word lines |
| Functions named and described | pass, 754 of 754 named (0 author symbols), 754 of 754 described (47 by rule, 283 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3 of 3 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16195 of 16509 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 459859 |
| code | 60412 |
| data | 497676 |
| padding | 12773 |
| unused | 771584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:run | 2 | 117 | 2 |
| run | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| LoadWB | 12 | 536 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:IP.txt | 2 | 24 | 4 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:CREDITS.EXE | 2 | 167 | 7 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:patch | 1 | 99 | 3 |
| icon.library | 10 | 1375 | 109 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:POPULOUS.II | 6 | 13404 | 584 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:ip_intro | 1 | 132 | 7 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |

## Cases handled

- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 2 of 2).adf:awards.pak: high-entropy data file (7.70 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 2 of 2).adf:hands.pak: high-entropy data file (7.68 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 2 of 2).adf:end.pak: high-entropy data file (7.66 bits/byte), classified as packed data
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip](disk 2 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 2 of 2).adf:judge.pak: high-entropy data file (7.68 bits/byte), classified as packed data
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EB8) A7=an address (A7)=an address, slid to exec.library -an address): 494 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- populous ii - trials of the olympian gods (1991)(electronic arts)[cr vf][f ip](disk 1 of 2)/Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:CREDITS.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:ip_intro

## Copy protection found

- none found

## Functions


### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If its destination pointer is set, it copies sixteen longwords from a fixed offset in an address-register block into that destination.

### run

- `setup_buffer_from_list_maximum`: Program entry for the run utility.
- `copy_memory_block`: copies a fixed number of dwords from a memory location offset by a pointer to a destination block on the stack, then calls another routine.

### EndCLI

- `setup_buffer_from_list_maximum_2`: Program entry with the same code as the run utility's entry routine.
- `copy_block_of_data`: copies a fixed number of longwords from a source memory location to a destination stack or memory area and then calls another routine.

### LoadWB

- `run_program_entry_with_messages`: Program entry that finds its own task, allocates memory, opens the DOS library by name, and waits on and reads messages from a message port.
- `release_task_resources_and_reply`: Retrieves a task-related pointer, stores its parameter into a slot of a record, and conditionally calls cleanup routines through library pointers.
- `start_workbench_tool_task`: Checks the start-up arguments for a debug switch, allocates a buffer, and runs the same byte-wise XOR pass twice over a text.
- `compare_strings`: Compares two zero-terminated strings byte by byte.
- `build_linked_list_from_table`: Walks a list of table entries, allocating a node for each one, storing a value copied from the entry, and linking the nodes together.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `patch_release_version_text`: Compares a text with a stored release string.
- `write_period_and_spaces`: Writes a period followed by four spaces into five consecutive bytes of a text at a fixed offset from the address register.
- `match_stored_prefix_text`: Compares a text at an address register with a stored string byte by byte and returns as soon as the stored string ends or a mismatch is found.
- `return_without_action`: Performs no operation and returns.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `forward_with_stack_slot_pointer`: Forwards its two parameters to another routine together with a pointer to a fixed stack location, then returns.
- `format_stack_text_and_forward`: Formats text into a 140-byte stack buffer through the Exec RawDoFmt routine, measures the length of the result, and passes the buffer and its length to another routine when the result is not empty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Disk-Validator

- `compare_bit_tables_and_count`: Program entry for the disk validator.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:IP.txt

- `write_output_with_library_wrappers`: Program entry that calls wrapper routines to obtain an output handle and close a library.
- `open_dos_library`: Opens the DOS library by name through the Exec OldOpenLibrary routine and stores the returned library handle in a global.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:CREDITS.EXE

- `check_signature_then_copy_backwards`: Program entry that reads a leading longword and compares it with a fixed signature.
- `accumulate_32bit_from_bytes`: Reads four consecutive bytes from the address in an address register and combines them into a 32-bit value with the first byte in the highest position.
- `decode_and_copy_bytes`: Reads bits from a bit stream through a helper and combines them with a count taken from a global table.
- `execute_empty_instructions`: a routine that executes a sequence of instructions without performing any observable work or modifying any registers or memory before returning.
- `lookup_index_then_repeat_calls`: Calls a helper to obtain an index and looks up a byte in a global table at that index plus one.
- `call_helper_until_zero_result`: When a state value equals two it makes one extra call, then repeats the helper until its result is zero.
- `busy_wait_countdown`: Counts a data register down to zero as a delay loop.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:patch

- `inspect_undecompiled_entry_routine`: The decompiler timed out on this 45-instruction program entry, so its behaviour is not recovered.
- `store_low_byte_and_shift`: Reads a longword at a fixed negative offset from an address register and stores its low byte, together with a byte from a data register, into a word location.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right one step per iteration for a count taken from a data register, and refills the buffer through a helper when it becomes zero.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code.
- `decrement_library_refcount`: Library code.
- `unlink_node_or_mark_pending`: Library code.
- `set_pending_flag_bit`: Library code.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_or_alert`: Library code.
- `store_saved_pointers_into_globals`: Library code.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `forward_to_helper_routine`: Library code.
- `call_library_handler`: Library code.
- `copy_icon_data_to_stack`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `initialize_icon_data_structures`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `invoke_icon_info_processing`: This library routine calls a helper routine to process icon information blocks.
- `call_library_handler_2`: This library routine simply calls another routine and then returns.
- `call_library_handler_3`: This library routine calls another routine and then returns, performing no other operations.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_allocate_icon_structure`: This library routine calls a routine to allocate an icon structure and then returns.
- `call_library_handler_4`: This library routine calls another routine and then returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_search_icon_list`: This library routine calls another routine and then returns.
- `call_parse_routine`: This routine calls another routine and then returns.
- `update_data_field`: Library code.
- `chain_helpers_with_cleanup`: Library code.
- `load_icon_record_from_buffer`: Library code.
- `store_record_fields_and_link`: Library code.
- `return_zero_stub_4`: The library routine is an empty stub that immediately returns the constant zero.
- `copy_record_then_call`: Library code.
- `read_info_file_header`: Library code.
- `load_info_header_blocks`: Library code.
- `load_remaining_icon_blocks`: Library code.
- `read_value_array_until_done`: Library code.
- `call_helper_return_zero`: Library code.
- `return_without_action_2`: Library code.
- `process_icon_info_blocks`: Library code.
- `process_info_file_blocks`: Library code.
- `process_pointer_table_blocks`: Library code.
- `measure_and_process_pointer_table`: Library code.
- `iterate_icon_table_entries`: Library code.
- `walk_node_chain_until_end`: Library code.
- `iterate_linked_list_nodes`: Library code.
- `call_helper_with_offset_argument`: Library code.
- `allocate_icon_structure`: Library code.
- `call_helper_and_return`: Library code.
- `return_null_pointer`: The routine returns a null pointer.
- `store_pair_in_counted_table`: Library code.
- `decrement_counter_and_store_pair`: Library code.
- `allocate_icon_structure_2`: Library code.
- `return_without_action_3`: Library code.
- `call_library_routine_then_zero`: Library code.
- `call_helper_with_shifted_argument`: Library code.
- `find_value_after_separator`: Library code.
- `scan_pointer_list_for_value`: Library code.
- `find_text_after_equals_sign`: Library code.
- `search_icon_list_for_match`: Library code.
- `search_icon_sequence_for_match`: Library code.
- `compare_string_sequences`: Library code that compares a sequence of values by checking if a returned value matches a target and increments a pointer or index.
- `library_search_sequence`: As library code, it calls another library routine to check a value and updates a register, potentially returning a calculated difference or a constant.
- `search_icon_sequence`: The library routine searches through a sequence or list, performing comparisons and pointer increments until a match or end-of-list is reached.
- `parse_copy_prefix_and_count`: Library code.
- `parse_decimal_count_field`: Library code.
- `parse_decimal_digits_from_text`: Library code.
- `parse_decimal_count_suffix`: Library code.
- `convert_decimal_digit_value`: Library code.
- `return_minus_one_stub`: Library code.
- `call_library_routine_and_return`: Library code.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `allocate_icon_structure_3`: Library code.
- `return_zero_stub_3`: The library routine returns a constant zero.
- `return_if_helper_matches`: Library code.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `library_verify_result`: As library code, it calls another library routine and checks if the returned value matches a provided parameter, returning zero if they do not match.
- `return_nothing_stub`: The library routine performs no work and returns.
- `divide_longword_with_remainder`: Library code.
- `shift_subtract_divide_sixteen_steps`: Library code.
- `multiply_32_bit_integers`: Library code.
- `call_raw_do_fmt_routine`: Library code.
- `store_byte_through_pointer`: Library code.
- `count_string_characters`: Library code.
- `find_char_in_string`: Library code.
- `return_zero_stub`: Library code.
- `copy_c_string`: Library code.
- `append_c_string`: Library code.
- `compare_strings_up_to_count`: Library code.
- `return_one_stub`: The library routine returns a constant one.
- `scan_string_with_length_limit`: Library code that walks a null-terminated string but stops once a given length limit is used up.
- `empty_routine_stub`: The library routine is empty and performs no work.
- `copy_string_zero_padded`: Library code that copies a zero-terminated string into a destination buffer, stopping at the size limit.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Library code that initialises a block of memory as a small self-referencing pointer structure.
- `process_block_if_valid`: Library code that returns success for an empty request.
- `walk_entry_list_checked`: Library code that walks a linked chain of entries until the link is empty.
- `check_paired_entry_chain`: Library code that walks a linked chain of entries, making two internal calls on each one.
- `return_zero`: This library routine always returns a zero value.
- `allocate_entry_with_buffer`: Library code that allocates a 20-byte control block and then a second buffer whose size comes from the caller's request.
- `return_error_value`: The library routine returns a constant value representing an error or invalid state.
- `build_entry_chain`: Library code that allocates a chain of 16-byte entries, one per requested count, and gives each a buffer sized from a byte inside the entry.
- `read_entry_chain_until_end`: Library code that reads a sequence of linked entries from its input, allocating each entry and a buffer sized from its header.
- `read_linked_entry_sequence`: Library code that reads entries in sequence, storing a buffer pointer in each one and linking it to the previous entry.
- `return_all_ones`: This library routine returns a 32-bit value where all bits are set to one.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:POPULOUS.II

- `start_program_dos_setup`: Program entry point that opens the DOS library and queries its current directory and input and output handles.
- `store_pointer_and_call_pair`: Stores a pointer at a fixed offset into a global variable, then calls two library routines through the program's base table.
- `call_two_library_functions`: Calls two library routines in sequence through entries in the program's base table and then returns.
- `clear_colour_registers_and_setup`: Writes zero to all thirty-two colour registers of the custom chip set, then runs a long sequence of display and system setup calls.
- `initialize_interrupt_handler`: Clears a flag, fetches a structure pointer through the library table and waits while that pointer is zero.
- `restore_vectors_and_enable_interrupts`: Copies a saved set of vector and handler pointers from RAM back into their locations.
- `return_immediately_stub`: Empty routine that returns to its caller without doing anything.
- `copy_field_and_store_constant`: Copies a field from a data structure into a global variable and then stores a fixed constant into another global.
- `set_global_constant`: Stores a constant value into a global variable.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `exchange_global_with_saved_values`: Exchanges a global variable with two saved variables, moving each value through a temporary.
- `install_interrupt_handler`: Saves the current interrupt-related global value, then stores a handler pointer and a RAM address into globals.
- `handle_pending_interrupt_flag`: Tests one bit of the interrupt request register.
- `clear_interrupt_request_bit`: Writes the interrupt acknowledge bit to the interrupt request register, clearing that pending request.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `record_setup_buffer_addresses`: Calls two setup routines, stores the address of a global buffer and a constant into RAM variables, and records two pointers eight bytes past the results of those calls.
- `build_copper_list_table`: Fills a block of memory with copper list words for display setup, choosing one of two colour sets based on a flag.
- `save_and_enable_interrupts`: Saves the current interrupt-related global value, installs a handler pointer and then writes a constant with the top bit set to the interrupt enable register.
- `swap_and_enable_interrupts`: Exchanges the interrupt-related global with a saved value and then writes a constant with the top bit set to the interrupt enable register.
- `update_keyboard_key_state`: Reads a key-state bit from the keyboard serial register.
- `clear_table_byte_decrement_count`: Writes zero into one byte of a table at an offset held in a data register, then decrements a counter if it is positive.
- `open_libraries_and_set_handler`: Opens system libraries through the exec library, then reads a flag from the returned structure and stores a handler pointer into a field of the opened library.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `swap_buffer_pointers`: Swaps two buffer pointer variables, toggles a table index between two entries, and loads a value from the table into the active global.
- `swap_global_pointers`: Swaps the values of two global variables, toggles a flag, and uses a table or pointer or offset to update a global variable.
- `rotate_interrupt_vector_table`: shifts a value in a list of pointers by updating a pointer to the next entry and updates a global offset or index.
- `swap_screen_buffers`: The routine swaps the values of two global variables, toggles a flag, reads a word from a global table at a computed offset into a global variable and sets another global variable to zero, then returns the incoming D0 value.
- `wait_until_flag_set`: Spins until a flag variable becomes nonzero, then returns to its caller.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `wait_for_non_zero`: Spins in a loop while a certain global variable is equal to zero.
- `update_mouse_position_and_buttons`: Reads the mouse movement counters, works out the change since the last reading and adds it to a horizontal and vertical position clamped to the screen.
- `update_secondary_button_state`: Clears one button's state, then reads a second button through the pot control register.
- `reset_secondary_button_state`: Clears one button's flag and counter, then calls one further routine.
- `split_mouse_counter_bytes`: Splits a 16-bit mouse counter word into its low byte and high byte, storing each in its own variable.
- `write_pointer_and_attribute_bytes`: Writes split pointer words for a global block into a structure, then fills two table positions with a derived byte.
- `set_global_var_pointers`: The routine checks a global variable against an input word and, if different, stores the address of a global variable plus the input word into two separate structures, writing the low and high parts of that address into fields of both structures and also stores a value returned by another routine.
- `reset_interrupt_state`: Stores constants into several RAM variables, installs a handler pointer and calls it, then masks a global to its low six bits.
- `initialize_global_state`: Assigns several constant values to global variables and calls another routine.
- `tiny_ori_2`: Small routine of 1 instruction that merges 143 into d0.
- `reset_interrupt_state_2`: Writes constants to a global variable and other global locations, updates a global pointer with a routine address, calls that routine, masks a global variable, and writes a constant to the interrupt enable register.
- `swap_handler_pointer`: Swaps the contents of a global variable with the value from another routine and then writes a constant to the interrupt enable register.
- `set_serial_period_from_clock`: Chooses one of two standard clock constants based on a flag, divides it by a stored divisor and stores the result minus one in a global.
- `abs_diff_with_optional_call`: Returns the absolute difference between two stored short values.
- `reset_two_globals`: Clears two global variables to zero and returns.
- `store_serial_byte_in_ring_buffer`: When the serial interrupt request bit is set, it stores the received byte in a ring buffer at the current write position.
- `return_stack_argument_stub`: Returns the value taken from its stack argument without changing it.
- `return_immediately_stub_2`: Empty routine that returns immediately without doing anything.
- `read_bytes_from_ring_buffer`: Copies bytes out of a ring buffer into a caller's buffer, stopping when the buffer runs empty or the requested count is used up.
- `copy_ring_buffer_until_count`: Copies bytes from a ring buffer structure into a caller's buffer, wrapping its read index.
- `copy_ring_buffer_until_zero_byte`: Copies bytes from a ring buffer into a caller's buffer until a zero byte is copied or the count is used up.
- `return_stack_value`: The routine is empty and returns a value previously stored on the stack without performing any operations or accessing external memory.
- `send_buffer_bytes_serially`: While the transmit-ready bit is set, copies the next byte from a buffer into the serial data word and sets a flag for each byte.
- `send_buffer_bytes_with_status`: Loads bytes from a buffer into the serial data word whenever the transmit-ready bit is set, and marks a status word in a structure as busy.
- `call_table_entry_passthrough`: Calls one routine through a pointer taken from a table and then returns the incoming values unchanged.
- `call_indirect_routine`: Calls a routine via a pointer stored in a global variable and returns a concatenated result of two registers.
- `call_close_work_bench`: Library stub: loads its arguments and the intuition library base, calls CloseWorkBench and returns its result to the caller.
- `set_default_state_variables`: Clears a flag and stores a set of fixed default values into consecutive RAM variables.
- `call_close_screen`: Library stub: loads its arguments and the intuition library base, calls CloseScreen and returns its result to the caller.
- `call_close_library_twice`: Calls a close-library routine through the program's base table twice in a row, then returns the incoming values unchanged.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `return_empty_routine`: The routine is empty and performs no operations before returning.
- `own_blitter_and_wait`: Repeatedly calls a polling routine and clears a flag until that routine returns zero.
- `forbid_and_own_blitter`: Calls the multitasking-disable routine and then takes ownership of the blitter through the graphics library.
- `permit_and_disown_blitter`: Calls the multitasking-enable routine and then releases the blitter through the graphics library.
- `run_polling_update_loop`: A large routine that repeatedly polls several global flags and calls many support routines to update game-state variables and memory bytes.
- `dispatch_indexed_handler`: When two state values are nonzero it calls one routine, then jumps through a table entry chosen by a stored index.
- `dispatch_handler_function`: If certain global variables are non-zero, it calls another routine before calling a routine pointer retrieved from a global table.
- `invoke_table_callback_sequence`: Runs a fixed five-step loop, each step choosing a table entry from a computed index and calling it together with support routines.
- `process_pending_entry_list`: Returns early when one flag is set, otherwise checks a region condition that may clear a flag and call a routine.
- `merge_tile_rows_into_bitplanes`: Walks a table of entries and calls a routine for each, then scans an output list with a limit check and calls three routines.
- `dispatch_grid_cell_at_position`: Converts a pair of input coordinates into grid cell positions using two bounds checks.
- `check_and_dispatch_handler`: When a flag and counter allow it, calls routines and may flip a byte in a table.
- `scan_table_for_matching_entry`: Returns zero unless a flag is set.
- `find_next_matching_table_entry`: Steps through a table in increments of thirty-two looking for an entry whose two stored bytes match the supplied values.
- `return_zero_stub_2`: Library code that does nothing except return zero to its caller.
- `update_counters_from_entry`: When an entry pointer is set, it reads two bytes from that entry, subtracts four from each and stores them as two counters.
- `scan_table_for_flagged_entry`: Returns zero unless a flag is set.
- `scan_table_for_match`: Iterates through a memory range, checking if a byte in a table matches a value and if a bit in a corresponding byte is set, then updates several global variables.
- `return_zero_2`: The routine returns a zero value.
- `queue_pending_entry`: Stores the supplied entry as the current pending entry.
- `invoke_handlers_for_cell_type`: Calls a routine when a state value is twelve, then calls another routine for each of four specific cell type values and always makes a final call.
- `xor_rows_with_repeating_key`: XORs source bytes into a screen buffer row by row, reusing a four-byte source pattern for each row.
- `expand_text_control_codes`: Copies a text stream into an output buffer byte by byte and expands high-bit control codes.
- `call_two_routines_then_return`: Calls two routines in sequence and then returns to its caller.
- `forward_call_and_return`: Calls one other routine and returns to its caller without any further action.
- `execute_double_call`: Calls another routine twice in succession before returning.
- `execute_subroutine`: The routine calls another routine and then returns.
- `convert_high_nibble_to_char`: Converts the high four bits of a byte into a single printable character, using letters for values above nine, and stores it at the output location.
- `convert_nibble_to_char`: Takes a nibble, converts it to an ASCII digit or letter, and stores the result in a buffer.
- `print_text_with_control_codes`: Walks a text string, printing ordinary characters through a routine and interpreting control bytes for line breaks, carriage returns, saved positions, and style or flag changes.
- `apply_masked_pattern_to_bitplanes`: Shifts a five-row pattern from a table and applies it to four bitplanes of the screen, clearing masked bits and combining pattern bits chosen by a flag byte.
- `do_nothing_2`: the routine is empty and performs no operations.
- `run_frame_dispatch_step`: Updates frame timing and fetches a packed word from a circular table, unpacking its fields into RAM variables.
- `increment_digit_and_dispatch`: Increments a decimal digit character stored in a buffer, wrapping from nine back to zero.
- `call_other_routine_2`: The routine performs a single call to another routine and then returns.
- `dispatch_table_entry_after_calls`: Calls a sequence of routines, skipping one when a flag is zero, then jumps through a table entry chosen by a stored index.
- `dispatch_after_three_calls`: Calls three routines in sequence, then jumps through a table entry chosen by a stored index.
- `advance_marker_in_buffer_then_dispatch`: Calls a set of routines and either calls one or clears a stored byte, depending on a state value.
- `skip_to_zero_terminator`: Advances through a zero-terminated string until its terminating zero byte is reached.
- `empty_routine`: The routine contains four instructions and performs no observable work.
- `update_clamped_stat_pair_and_dispatch`: Derives two values from stored counters, divides each by four with an offset added for one mode, and clamps both to the range one to ninety-nine.
- `update_global_value_with_clamping`: Calculates a new value by dividing a global short or or a modified version of it by four, then clamps the result between one and ninety-nine before storing it back to a global variable.
- `update_clamping_logic`: Calculates a new value based on a division or addition, then clamps the result between one and ninety-nine before storing it back into a global variable.
- `update_global_value_clamped`: Calculates a new value based on a division or addition involving a global variable, clamps the result between 1 and 99, and writes it back to a global memory location.
- `build_status_text_and_dispatch`: Derives two clamped values in the same way, then builds a text line from a fixed label and calls display routines.
- `reset_display_state_and_dispatch`: Sets a display-related variable and clears two state bytes, then calls several routines.
- `toggle_marker_characters_and_dispatch`: Scans a text buffer and toggles two marker characters according to the bits of a stored value, then calls display routines.
- `repeat_marker_toggle_and_dispatch`: Repeats a sequence of display routines and marker-character toggling while a flag is set and a position check holds.
- `dispatch_after_single_call`: Calls one routine that returns an index, then jumps through the table entry chosen by that index.
- `call_handlers_from_table`: Walks a grid of table entries and byte flags, calling the handler selected by each non-zero entry with two position arguments.
- `build_menu_string_pointers_and_dispatch`: Clears a block of twelve long words, then fills a table of pointers to consecutive strings in a text buffer.
- `select_mode_label_markers_and_dispatch`: Chooses which marker characters in a label string are enabled, based on a stored mode value.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `update_outcome_markers_and_dispatch`: Scans a label string and swaps marker characters according to the bits of a stored value.
- `toggle_label_markers_and_dispatch`: Scans a label string, toggling a marker character according to bits of a stored value and replacing another marker with a letter at a position taken from a table.
- `repeat_call_then_dispatch_index`: Repeatedly calls a routine while a mode value and an index stay within limits, then jumps through a table entry chosen by the index.
- `append_char_to_text_then_dispatch`: Searches a table in steps of eight for an entry matching a stored value.
- `process_input_char`: Converts a character to uppercase and appends it to a text string while potentially shifting existing string contents.
- `append_char_to_text_then_dispatch_2`: Takes a character from its input, converts it to upper case within a valid range, and appends it to a text string.
- `scan_between_marker_characters`: Scans backward in a text buffer to find a v marker and forward to a w marker, then calls one routine.
- `search_backwards_for_v`: Searches backward through a string or memory buffer until a specific character is encountered.
- `search_forwards_for_w`: Searches forward through a string or memory buffer until a specific character is encountered.
- `find_v_w_delimiter`: Steps back from a pointer to find a 'v' character, then steps forward to find a 'w' character, and calls another routine.
- `edit_text_line_input`: Reads typed characters into a text line, finishing on the return key, deleting on backspace, and inserting other characters at the cursor while the line is under its length limit.
- `update_string_buffer_2`: Modifies a string buffer by inserting or deleting a character.
- `modify_string_buffer`: inserts or deletes a character within a string buffer.
- `update_string_buffer`: Inserts or deletes a character in a string buffer.
- `update_text_buffer_on_input`: Inserts or deletes characters within a string buffer based on input, handling backspace-driven deletions and character-driven insertions that shift existing characters.
- `call_routine_and_return_zero`: Calls one routine and then returns zero to its caller.
- `copy_text_with_marker_then_call`: Copies a fixed number of characters from a source string into a buffer, filling with a filler character once the source ends.
- `reset_flag_and_call`: Clears a global flag to zero and then calls one routine.
- `toggle_last_marker_in_region`: When a flag is set and a position lies inside a region, scans a text span and counts characters that pass a lookup-table check.
- `format_string_with_braces`: Copies a format string into a global buffer, replacing each brace marker with the next string from an argument list and using a space for an empty argument.
- `edit_short_text_field_key`: Accepts a typed space, backspace, return or capital letter and edits a short text field.
- `shift_buffer_and_apply_key`: Shifts a text buffer one position toward its start, then applies the typed key.
- `store_printable_key_at_end`: When the key code is above the control range, stores it one position before the buffer end and writes a zero terminator at the end.
- `decrement_counter_and_call`: Decrements a positive counter, and if a global flag is set calls one routine twice.
- `execute_handler_pointers`: The routine calls another routine twice in a row.
- `render_text_glyphs_into_bitplanes`: Copies glyph bitmaps for each character of a text string into four bitplanes of the screen, advancing one row at a time for each character.
- `process_tile_entry`: When a table entry is nonzero and a byte from a second table is positive, it records an index and calls a series of routines, including two indirect handlers taken from the tables.
- `set_text_pointers_and_dispatch`: Computes offsets from stored position values and fills four string pointers by skipping through a text buffer.
- `update_pointer_tables`: Calculates and stores several pointers or offsets into a global table based on a global variable's value and then performs a series of string-like pointer increments.
- `update_string_pointers`: Calculates and stores multiple pointers into a global table using an offset or or a global variable, and then iterates through a list of null-terminated strings to find or update pointers.
- `set_label_pointers_and_dispatch`: Computes offsets from stored position values and fills four label pointers by skipping through a text buffer.
- `process_buffer_until_complete`: Clears several global variables and a pointer, then repeatedly processes a buffer through support routines, toggling a flag and advancing an index until a completion flag is set.
- `process_frame_stream_entries`: When a stored value is below the limit it sets up several RAM variables and calls support routines, then walks a stream of packed words.
- `decode_packed_stream_data`: Iterates through a stream of packed words to extract or calculate offsets and parameters, updating global state and triggering callback routines or secondary processing loops.
- `parse_packed_stream_entries`: Iterates through a packed word stream, decoding certain bits to calculate offsets and parameters for a callback routine.
- `resume_packed_stream_entries`: Continues walking a packed word stream from a saved position, processing each word the same way until the stream ends or a completion flag is set.
- `copy_bitplane_block_rows`: Copies rows of twelve bytes from a source buffer into four bitplanes of the screen, spreading the bytes across the planes.
- `set_status_label_and_dispatch`: Selects values from stored tables, writes a packed attribute byte into a fixed RAM location and copies six bytes from a table.
- `copy_word_block_into_bitplanes`: Copies a block of words from a buffer into the bitplanes of the screen, sixty-four lines deep, then calls two table-selected routines in turn.
- `copy_and_dispatch_blocks`: Copies certain blocks of data from a source pointer to a destination pointer and then executes a sequence of indirect calls or jumps based on a calculated or table-based address.
- `iterate_table_dispatch`: Repeats three times, each time combining a table value with a buffer byte to pick a function pointer from a table and calling it with the step counter.
- `nested_double_call_loop`: Runs an outer loop twice and an inner loop three times, calling two routines on each inner pass.
- `copy_rows_into_four_planes`: Copies up to thirty-two rows of data into the four bitplanes of the screen, one word per plane.
- `copy_block_into_four_planes`: Copies a fixed number of four-word rows from a source table into four bitplanes, advancing both pointers each row until a counter reaches zero.
- `walk_blitter_grid_entries`: Sets blitter control and modulo registers, then waits for the blitter to become idle before each pass.
- `start_blitter_transfers_for_entry`: Starts up to two blitter transfers for one table entry, waiting for the blitter to be idle before each one.
- `setup_clipped_blitter_parameters`: Looks up a source entry and computes its position clipped against limits, adjusting width and height offsets when it sticks out.
- `setup_blitter_rows_and_wait`: Writes blitter control and size values, waits for the blitter to become idle, then loops eight times writing row pointers and a size code with a wait after each.
- `start_blitter_with_pointers`: Waits for the blitter to become idle, programs its control and pointer registers from the given values, and starts a blit.
- `clear_table_and_iterate`: Clears a block of four thousand ninety-six long words starting at a global variable, then walks a table until a sentinel value, calling three routines for each entry.
- `generate_random_sequence`: Generates pseudo-random numbers with a linear congruential step, passing each value to a routine.
- `update_neighbour_tile_flags`: Increments a call counter, then walks the neighbouring cells of a 64-by-64 grid, setting flag bits and advancing small counters for each one that lies inside the grid.
- `clear_neighbour_tile_flags`: Increments a call counter, then walks the neighbouring cells of a 64-by-64 grid, clearing flag bits and decrementing small counters for each one inside the grid.
- `decrement_cell_flags_at_offset`: Clears the flag bits of one grid cell and decrements the counters of its four neighbours where they lie inside the 64-by-64 grid.
- `decrement_two_neighbour_flags`: Clears one flag bit of a grid cell and adjusts the counters of its neighbouring cells that lie inside the 64-by-64 grid.
- `decrement_one_neighbour_flag`: Clears one flag bit of a grid cell and decrements a counter in a neighbouring cell when it lies inside the 64-by-64 grid.
- `decrement_cell_counter_and_flag`: Decrements a small counter in a grid cell, masks it to its low three bits, clears one flag bit of that cell and increments a call counter.
- `return_null_pointer_2`: The routine returns a null pointer or a zero-value or any value previously loaded into a register.
- `check_cell_within_bounds`: Tests whether a pair of coordinates lies within the grid limits, leaving the values unchanged.
- `scan_neighbour_cells_for_marker`: Checks the four cells around a grid position, following each cell's linked entries for a marker value, and returns the input position unchanged.
- `return_stack_values`: Returns a 32-bit value formed by concatenating two 16-bit values from the stack.
- `scan_neighbour_cells_for_marker_2`: Checks three cells next to a grid position by following their linked entries for a marker value, and returns the input position unchanged.
- `return_empty_longword`: The routine returns a 32-bit value constructed from two 16-bit stack-allocated values without performing any operations or accessing memory or registers.
- `scan_neighbour_cells_for_marker_3`: Checks two cells next to a grid position by following their linked entries for a marker value, and returns the input position unchanged.
- `return_empty_stack_contents`: The routine is empty and performs no operations before returning, effectively returning whatever data was present on the stack or in the registers used for the return-address/stack-frame restoration.
- `scan_neighbour_cells_for_marker_4`: Checks one cell beside a grid position by following its linked entries for a marker value, and returns the input position unchanged.
- `return_stack_values_2`: Returns a 32-bit value formed by concatenating two 16-bit values from thestack.
- `return_stack_values_3`: Returns a 32 or 32-bit value formed by concatenating two 16-bit values from the stack.
- `call_two_and_combine_results`: Calls two routines and returns the result of the first in the high half of a 32-bit value and the result of the second in the low half.
- `combine_two_shorts`: Calls another routine twice and returns a 32-bit value formed by concatenating the short result of the first call in the high word with the short result of the second call in the low word.
- `fill_byte_grid_from_routine`: Calls one routine for each cell of a 17-by-17 loop, writing a fixed byte through a pointer each time, then returns the last result less eight.
- `decrement_byte_and_test_neighbours`: Decrements a signed byte in a grid structure, then checks neighbouring bytes for underflow or a small difference from the new value, calling a routine when a check fails.
- `increment_byte_and_check_neighbours`: Increments a signed byte in a grid structure, then compares neighbouring bytes with the new value and calls routines when a comparison fails.
- `store_index_then_clear_counter`: Stores the input value as the current index and calls one routine.
- `clear_counter_and_call`: Clears a counter to zero, then calls one routine and returns.
- `check_and_set_global`: Checks if a value is negative, stores it in a global variable, and then calls another routine, potentially clearing a global variable and calling another routine if the value was non-negative.
- `clear_global_variable`: Sets a global variable to zero and calls another routine.
- `fill_tile_map_from_table`: Walks a rectangular region of a 64-by-64 grid, looking up each cell's byte in a table and storing a transformed 16-bit value into a buffer.
- `call_callback_in_nested_loops`: Calls a stored routine pointer sixty-four times in each of sixty-four passes, making four thousand ninety-six calls in total.
- `call_routine_if_entry_flagged`: When a stored flag is clear and a second flag bit is set for an entry, and no pending state is active, it calls one routine.
- `call_routine_then_return_argument`: Calls one routine and then returns the value taken from its stack argument.
- `return_first_stack_argument`: Returns the first value passed on the stack without changing it, with no other work.
- `clear_global_table_words`: Writes zero into pairs of words in the global table, advancing four bytes per step until the pointer reaches a fixed end address.
- `compare_table_low_bits`: Walks the global table from both ends toward each other in four-byte steps, comparing the low three bits of each pair of bytes.
- `run_paired_calls_until_zero`: Makes one call to another routine, then repeatedly calls two routines in a loop until a short result returned in a data register becomes zero.
- `fill_random_entry_slots`: Draws values from another routine to choose table positions, then for each qualifying position claims a free 14-byte entry, sets its type tag and fields from the drawn values and a table lookup, and calls another routine to register it.
- `advance_tagged_entry_link`: Walks the chain of entries tagged with type code 22 looking for the one whose link matches the value passed in, and moves that entry's link on to the next value in a small ring of words.
- `find_next_ring_link`: Scans a ring of words for the value held by a tagged entry, and stores the word that follows it as that entry's new link.
- `initialize_tagged_entry_fields`: Writes a fixed type tag and fixed flag bytes into the entry at the given pointer, copies a global byte and a table word into their fields, and then calls another routine to link the entry in.
- `call_other_routine_in_loop`: Calls another routine once, then repeatedly calls two other routines in a loop until a short value returned in D1 is zero.
- `generate_random_indexed_value`: Repeatedly obtains values from another routine to drive a counter; each pass combines a masked random word with a table lookup and conditionally calls another routine.
- `claim_slot_entry_if_free`: Checks a slot byte taken from a register and the flag and word stored for that slot in the global table.
- `advance_tagged_entry_link_2`: Walks the chain of entries tagged with type code 24 looking for the one whose link matches the value passed in, and moves that entry's link on to the next value in a small ring of words.
- `search_and_initialize_struct`: Walks a linked list of entries until a certain value or a sentinel is reached, then initializes a structure with constant values and certain register or global contents.
- `initialize_message_struct`: Writes constant bytes and values from registers and a global variable into a structure pointed to by a pointer, sets a two-byte field to a constant, and then calls another routine.
- `update_active_entry_states`: Walks all 14-byte entries once per call, advancing each active entry's counters and direction; entries with type code 30 follow a separate countdown path.
- `try_placement_at_offset`: Takes a word of coordinates, offsets its two low three-bit fields by the given position registers, and rejects the request unless both results fall in the range zero to sixty-three.
- `call_two_routines_return_one`: Calls two other routines in sequence and returns the value one.
- `return_zero_result`: Returns zero with no other work.
- `clear_chained_entry_flag`: Follows a chain of entries starting from an indexed link word.
- `clear_chain_end_active_flag`: Follows a chain of entries from the given start pointer, and at the end clears the active byte of the entry reached and calls another routine.
- `clear_chain_entry_flag`: Follows the link chain of table entries from the start pointer until it reaches a zero link or a boundary of the table.
- `draw_pixel_or_line_bitplanes`: Plots a single pixel when the two input points are equal, setting or clearing it in four bitplanes of a 320 by 200 screen according to the low four bits of the mode value, and discards points outside the screen.
- `double_and_call_twice`: Calls another routine with the input word doubled, then calls that same routine again with no argument and returns.
- `call_and_return_concatenated_inputs`: Calls another routine, then returns the two input registers combined into one 64-bit result unchanged.
- `extract_bit_from_table`: Returns a single bit from a byte table, located by a row value multiplied by 40 plus the column byte taken from the first input word.
- `plot_checked_pixel_bitplanes`: Sets or clears one pixel in four bitplanes of a 320 by 200 screen, using the low four bits of a mode value to choose the setting for each plane.
- `plot_unchecked_pixel_bitplanes`: Sets or clears one pixel in four bitplanes using the low four bits of the mode value to choose each plane's setting.
- `clear_bit_across_four_planes`: Clears one bit, selected by the low three bits of a register, in the byte at the given pointer and in the three bytes 8000 bytes further on, which are the same position in the other three bitplanes.
- `set_blitter_window_and_mode`: When a flag byte in the table is set it calls another routine; if the word at the pointer is then zero, it writes fixed constants to the blitter first and last word masks, the modulo registers and the control registers.
- `dispatch_byte_compare_modes`: Compares two byte fields of a structure against each other using one of sixteen comparison modes selected by a table entry.
- `stub_single_instruction_routine`: The routine is one instruction long and is called from seven other routines.
- `stub_undecompiled_short_routine`: The routine is four instructions long and is called from one other routine.
- `dispatch_object_chain_handlers`: Walks a linked chain of object records from a start index, calling each record's handler through a function pointer table and following the chain links.
- `update_object_chain_from_entry`: Walks a chain of object records beginning at the record given as an argument, calling each record's handler through its pointer and following the chain links.
- `call_routine_twice`: Calls another routine twice in succession and returns.
- `increment_counter_and_dispatch_handlers`: Checks a word argument against a limit and alignment, increments a counter in the global table through an index table, then walks a linked list of short entries computing a value for each.
- `call_handler_from_flag_slot`: Selects a global variable block according to a flag byte in the object at the address register, then calls the function pointer stored eight bytes into that block.
- `call_indirect_if_flag_set`: Calls a routine through a stored function pointer only when a global flag word is non-negative, then returns the two input registers combined unchanged.
- `blit_masked_sprite_bitplanes`: Copies sprite rows into four bitplanes of a 320 by 200 screen, clipping rows outside the screen and shifting the pixel data by the sub-byte offset in the low bits of a register.
- `blit_masked_sprite_bitplanes_2`: Copies sprite rows into four bitplanes of a 320 by 200 screen, shifting the pixel data by the sub-byte offset in the low bits of a register and clipping rows that fall past the bottom edge.
- `write_unshifted_sprite_rows`: Writes a counted run of five-byte sprite rows into four bitplanes without shifting, combining each plane byte with the destination under the row's mask byte so destination bits are kept where the mask is set.
- `blit_rotated_sprite_rows`: Writes a counted run of five-byte sprite rows into four bitplanes, rotating each byte by a shift amount taken from a register and combining it with the destination under the rotated mask.
- `start_blitter_clipped_copy`: Clips the requested rectangle to the 320 by 200 screen, then programs the blitter's shift, mask, modulo and size registers from a table and starts a blit, polling the blitter busy flag while it programs them.
- `calculate_blitter_parameters`: Performs conditional checks on registers and updates a global variable using values loaded from a table.
- `start_blitter_unclipped_copy`: Programs the blitter's shift, mask, modulo and size registers from a table for a rectangle whose top edge has already been checked, and starts a blit while polling the blitter busy flag.
- `start_blitter_segment_copy`: Programs the blitter's shift, mask, modulo and size registers from a table for a short segment, polling the busy flag, then starts the blit and waits for it to finish.
- `return_without_work`: Returns immediately with no instructions.
- `start_blitter_table_blit`: Loads the blitter control and mask registers from a table indexed by a shift value, sets the source and destination modulo values and destination pointer, then starts a blit and waits for it to finish.
- `start_blitter_edge_blit`: Loads the blitter control and mask registers from a table for the final column, sets the modulo and destination pointer, then starts a blit and waits for it to finish.
- `start_blitter_shifted_blit`: Loads the blitter control and mask registers from a table for a shift value, sets the modulo and destination values, then starts a blit and waits for it to finish.
- `start_blitter_sprite_blit`: Clips a sprite position against the top and bottom edges of the screen, picks one blitter register set from a table for the body, middle or final column of the sprite, then starts the blit while polling the busy flag.
- `update_sprite_blitter_parameters`: Calculates and writes new blitter destination addresses and control registers based on a sprite's horizontal or vertical position and a lookup table.
- `start_unclipped_sprite_blit`: Clips a sprite position only against the bottom edge of the screen, picks one blitter register set from a table for the body, middle or final column of the sprite, then starts the blit while polling the busy flag.
- `start_blitter_column_blit`: Picks one blitter register set from a table for the body, middle or final column of a sprite, programs the blitter with the destination offset, and starts the blit while polling the busy flag.
- `setup_blitter_control`: Calculates an index and loads a control word and mask or mode or or or or into blitter registers.
- `setup_blitter_control_2`: Loads a value into blitter registers based on a calculated index and performs a loop waiting for a bit in a register to clear.
- `start_blitter_offset_blit`: Loads the blitter control registers from a table for a shift value, sets the source and destination pointers at a fixed offset, then starts a blit and waits for it to finish.
- `start_blitter_masked_offset_blit`: Loads the blitter control registers from a table for a shift value, sets the source and destination pointers at a fixed offset, then starts a blit with full mask words and waits for it to finish.
- `wait_for_blitter_and_setup`: Reads a shift-indexed table entry and writes its control and mask words to the blitter, then waits for the blitter busy flag to clear.
- `call_store_and_pack_result`: Calls another routine, stores its 32-bit result in a global variable, and returns that result shifted down by eight bits and combined with the input register.
- `multiply_32bit_halves`: Multiplies two 32-bit register values using their 16-bit halves, with the cross terms shifted into the upper half, and returns the product in the high word of a 64-bit result.
- `parse_signed_decimal_text`: Reads a decimal number from a text buffer, skipping spaces and tabs and accepting a minus sign before the digits.
- `skip_signed_decimal_token`: Advances through a text buffer past spaces, tabs and digits, stopping at a line break.
- `skip_numeric_text_field`: Advances through a text buffer past spaces, tabs and digits until it reaches a line break or a second minus sign.
- `skip_text_separators_to_number`: Advances through a text buffer past spaces and tabs until it reaches a number, stopping early at a line break.
- `copy_longword_blocks`: Copies 500 consecutive blocks of 16 longwords from a source pointer to a destination pointer, one block after another.
- `clear_zeroed_struct_array`: Writes zero into all eight longwords of each of 1000 consecutive records, starting from the pointer passed in.
- `lookup_signed_pair_value`: Takes two signed 16-bit values, reduces each by a shift of five bits and orders them so that one indexes a byte table.
- `read_table_byte_from_pair`: Returns a byte from a table 1024 bytes past the base pointer, indexed by combining the two register inputs with their shifted values.
- `lookup_negated_pair_value`: Looks up a byte in a table 1024 bytes past the base pointer, using the two input values to form the index, and returns that byte subtracted from minus 128.
- `lookup_negated_offset_value`: Looks up a byte in a table 1024 bytes past the base pointer, indexed by the two input values, and returns minus 128 minus that byte.
- `lookup_signed_pair_value_2`: Takes two signed 16-bit values, reduces each by a shift of five bits and orders them so that one indexes a byte table at the base pointer.
- `lookup_negated_table_byte`: Returns the negation of a byte from a table 1024 bytes past the base pointer, indexed by the two input values.
- `lookup_offset_byte_minus_128`: Reduces two signed inputs by a shift of five bits and orders them to form an index into a table 1024 bytes past the base pointer.
- `read_byte_from_table`: Reads a byte from a table 1024 bytes past the base pointer, indexed by the two input words, and returns it with 128 subtracted.
- `format_integer_as_decimal_text`: Writes the integer from the input register as decimal characters into a text buffer, using a zero-terminated table of decimal place values and suppressing leading zeros, then adds a terminating zero byte.
- `format_word_as_decimal_text`: Writes the low 16 bits of the input register as unsigned decimal characters into a text buffer, using a zero-terminated table of place values and suppressing leading zeros.
- `format_longword_as_decimal_text`: Writes the 32-bit integer from the input register as decimal characters into a text buffer, using a zero-terminated table of place values and suppressing leading zeros.
- `store_word_from_pointer`: Reads a word from a pointer and stores it into a destination address calculated from two registers.
- `fetch_keyboard_char_if_flagged`: Clears a global flag byte and, when the saved value had its low bit set, converts that value into an index and returns the matching byte from a character table that begins with the keyboard row text.
- `write_form_bitplane_rows`: Splits 16 packed words into nibble-sized bytes in a global table, then calls an operating system library routine for each of 200 rows across four planes spaced 8000 bytes apart.
- `interpolate_two_tables`: Steps a blend factor from zero upward over two 16-entry word tables, computing each blended word from the nibble fields of the paired source words and writing it into two output buffers.
- `compare_zero_terminated_text`: Compares a zero-terminated text at one pointer with a text at another pointer, one byte at a time, for up to a count held in a register.
- `check_text_is_nonempty`: Returns one if the byte at the given pointer is not zero, otherwise zero.
- `encode_word_as_table_bytes`: Transforms the input word with a multiply-and-add step, masks the result to 15 bits, and writes a pair of table bytes for each six-bit group of the value into a buffer.
- `encode_word_as_table_bytes_2`: Transforms the input word with a multiply-and-add step, masks the result to 15 bits, and writes a pair of table bytes for each six-bit group of the value into a buffer.
- `xor_two_longwords_then_loop`: Calls another routine, XORs two consecutive longwords at the pointer with fixed 32-bit constants, then calls another routine repeatedly until a short value in a data register reaches zero, and makes one final call.
- `encode_value_as_four_letters`: Writes four capital letters encoding the input value, with the letters derived by dividing the value by 17576, then 676, then 26, and taking each quotient or remainder as an offset from the letter A. The result is written through the output pointer.
- `swap_bytes_in_struct`: Swaps six pairs of bytes inside the structure at the address register: offsets 1 and 4, 2 and 8, 3 and 12, 6 and 9, 7 and 13, and 11 and 14.
- `rotate_bits_in_place`: Rotates an eight-byte block one bit to the left through a carry bit, then stores two longwords into the buffer, one of them the address of another routine.
- `store_thirds_if_divisible`: Calls another routine, then reads three values.
- `modify_buffer_with_xor`: Performs a loop that checks if a value is divisible by three and writes a quotient to a pointer.
- `return_minus_one_status`: Returns minus one with no other work.
- `decode_four_letters_to_value`: Reads four bytes at the pointer, subtracts the code of capital A from each so that A counts as zero, and combines them in base 26 into one integer that is returned.
- `unpack_packed_stream_backward`: Decompresses a packed stream by working from its end toward its start.
- `stub_flag_checked_return`: Returns when the zero condition flag is set and otherwise does not return.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `copy_and_transform_buffer`: Walks a table of source pointers and sizes until a negative pointer ends it.
- `dispatch_four_character_tag`: Compares a four-byte tag from the input against INIT, FRM2 and STOP. INIT calls one routine and returns the letter code M2, FRM2 calls two routines and returns a value from the stored state, and STOP returns the letter code OP; any other tag calls one routine and returns a value derived from the input.
- `dispatch_frm2_stop_tags`: Handles a four-character tag: FRM2 calls two routines and returns a value taken from the stored state, and STOP returns the letter code OP. Any other tag calls one routine and returns the input value minus one, or the result of another routine when the decremented value is one and the high word is zero.
- `extract_high_word_duplicate`: Takes the high 16 bits of the input register and returns them duplicated into both halves of a 32-bit value.
- `return_stop_tag_or_decrement`: Returns the four-character tag STOP unchanged when the input is that tag.
- `decrement_with_routine_check`: Calls one routine, then returns the input value minus one.
- `duplicate_word16`: Copies the low 16 bits of the input register into both halves of a 32-bit result.
- `unpack_rle_bitplanes`: Decodes a run-length packed stream into four bitplanes of a 320 by 200 screen, where count bytes with the high bit clear copy that many literal bytes and negative counts repeat the following byte.
- `unpack_rle_plane_stream`: Decodes a run-length packed stream into a set of bitplane rows, keeping its read and write positions in registers and stepping through 200 rows across three planes.
- `copy_longword_table_block`: Copies 8000 consecutive longwords from one global table to another, starting at the two table addresses.
- `copy_fifteen_short_values`: Scans two global tables for the marker value 384, then writes sixteen 16-bit values from the source pointer into both tables, starting after each marker.
- `clear_and_unpack_rle_planes`: Clears 32000 bytes of the destination, then decodes a run-length packed stream into four bitplanes.
- `unpack_rle_bitplane_rows`: Decodes a run-length packed stream into four bitplanes without clearing them first.
- `decode_rle_bitplanes_strided`: Decodes a run-length packed stream into bitplane rows, stepping each written byte by a stride taken from a register.
- `decode_rle_plane_state`: Decodes a run-length packed stream into bitplanes with its read and write positions and counters kept in registers.
- `decode_rle_stream_to_planes`: Decodes a run-length packed stream into bitplane rows, where positive count bytes copy literal bytes, negative counts repeat the following byte, and zero starts a longer run.
- `decode_rle_remaining_planes`: Continues decoding a run-length packed stream into the remaining bitplanes, advancing 8000 bytes per plane.
- `initialize_dma_channel_table`: Initialises a block of global values, then loops over a table of six-byte entries and computes a new value for each non-zero entry from a small threshold table.
- `run_state_loop_checked`: Calls a set of helper routines in a loop that checks a global state word.
- `run_checked_state_steps`: Loops over a helper step on a global state word until that step reports a non-zero result, then makes a fixed sequence of helper calls and returns zero.
- `copy_fields_and_scan_entries`: Copies two word fields inside a structure to other offsets in it and stores a third word in a global.
- `check_and_adjust_flags`: Calls another routine, and when its result is zero and the second input matches a global value, stores two bytes from the structure, each reduced by four, into globals.
- `initialize_free_slot_entry`: Finds the first unused 52-byte record in the global table, marks it used, and fills its fields from the input values and a table entry selected by a register.
- `dispatch_by_state_index`: Calls another routine, then calls the handler selected by a state word from an address table in the global area.
- `copy_initial_state_blocks`: Copies a 58-byte block and a 60-byte block of constant data into the global area.
- `filter_table_entries`: Walks a set of 314-byte structures and, for each, copies pairs from two global lists into per-structure lists whenever a flag byte chosen by the pair's index is positive.
- `reset_state_and_init_flags`: Clears several global variables and a byte range up to a fixed end address.
- `store_modulo_five_offset`: Calls two other routines, then stores the low word of the input modulo five, multiplied by 727, plus the second input register into a global.
- `or_accumulate_block_into_tables`: Calls another routine, then repeatedly ORs eight longwords from a source table into two destination tables, advancing the source by 250 bytes each pass.
- `update_timers_and_copy_state`: Waits in a short loop while two global values sum to a non-zero value, then resets a counter.
- `clamp_two_globals_to_range`: Clamps two global values so that negative values become zero and values above 55 become 56.
- `clamp_two_globals_to_range_2`: Clamps two global values so that negative values become zero and values above 55 become 56.
- `swap_two_entries`: Swaps the first eight bytes and two 16-bit fields between two 314-byte entries, one selected by the input index and one by a global index.
- `stub_undecompiled_call_routine`: A thirty-instruction routine that calls two other routines and is called from two places.
- `stub_undecompiled_three_call_routine`: A twenty-two-instruction routine that makes three calls to other routines.
- `decrement_counter_and_clear_flags`: Decrements a 16-bit counter in the global table for the entry indexed by a byte in the structure.
- `relink_entry_in_bucket_list`: Stores two new field values into a structure.
- `unlink_and_relink_entry`: Unlinks the entry at the given pointer from its chain, then links it in at the head of the chain selected by a table index and increments a count byte there.
- `insert_into_linked_list`: Clears the link field of the entry, then inserts it at the head of the chain selected by its key byte in the global table, pointing the previous head back to it.
- `remove_node_from_linked_list`: Removes the entry at the given pointer from the chain whose head is held in the global table under its key, relinking its neighbours and clearing its link field.
- `merge_entity_state`: Merges one entry into another: copies two fields when a flag bit is set, ORs the flag byte across, keeps the larger of two byte fields, and adds a longword from the source into the target.
- `accumulate_slot_totals_and_state`: Computes a value from a base word and optional fields of the source, adds it into the global accumulator for the slot indexed by the entry, and increments a count word there.
- `reset_entity_state`: Calls another routine, calls a second one when a short field at offset 40 equals two, then sets a state byte at offset 22 to 36 and clears a short field at offset 10.
- `set_entity_state_fields`: When the byte at the first pointer is four, calls two routines; otherwise, when the byte at the second pointer is four, calls two routines.
- `set_entity_state_fields_2`: Calls two routines when the byte at the pointer is four.
- `set_entity_values_and_call`: Writes a 16-bit field at offset 10 to zero and then to 6956, or to 7376 when the byte at offset 12 is not one, sets the byte at offset 22 to 66, and calls another routine.
- `set_entity_flags_and_timer`: Calls another routine, sets a 16-bit field at offset 10 to 7796, sets the first byte to 18, and sets the byte at offset 22 to 24.
- `link_entities_set_state`: Links two entries by writing each entry's offset into the other's link field, and when the first entry is in state ten it appends the second to a chain and sets state and timer bytes for both.
- `link_two_structures`: Links two structures by writing each one's offset into a link field of the other.
- `initialize_spawn_entry`: When bit two of the flags byte is clear, sets state, timing and flag bytes in the entry, calls another routine and returns if its result is positive.
- `stub_undecompiled_four_call_routine`: A forty-three-instruction routine that makes four calls to other routines.
- `select_entity_state_by_flags`: Writes a state byte and a timer word into the entry, with the values chosen by which bit masks are set in the input word.
- `set_entity_timer_and_state`: Writes 6508 into the 16-bit field at offset 10 and 46 into the byte at offset 22 of the entry at the address register.
- `select_entity_state_from_mask`: Chooses an entry state from the bit mask in the input register using the same selection structure as the routine before it, and writes state and timer fields for the chosen case.
- `initialize_struct_fields`: Writes 2012 into the 16-bit field at offset 10 and sets the byte at offset 0 to 10, the byte at offset 22 to 22, and the bytes at offsets 7 and 9 to minus 128.
- `update_entity_timer_and_state`: Updates a timer field and a state or type field within an entity structure, potentially loading a new timer duration from a table or a pointer-based lookup.
- `set_fixed_entity_state_pair`: Writes 2012 into the 16-bit field at offset 10 of the entry, calls another routine, then sets the byte at offset 0 to 12, the byte at offset 22 to 50, and bytes 7 and 9 to minus 128, and calls another routine.
- `select_entity_state_from_mask_2`: Chooses an entry state from the bit mask in the input register and writes state and timer fields for the chosen case.
- `set_object_header_fields`: Writes 3032 into the 16-bit field at offset 10, or 3084 when the byte at offset 12 is not one, then calls another routine.
- `update_entity_state`: Checks certain flags in a structure and writes a value or a value from a global table into a timer field, then recursively calls itself.
- `set_entity_flags_write_bytes`: Writes 2012 into the 16-bit field at offset 10 and, when a flag bit in the global table is set for the entry's indices, stores 15 into the table.
- `set_state_on_flag_bit`: Tests bit zero of the byte at the second pointer and, when it is clear, writes state and timer fields chosen by the entry's flags byte, calls other routines, then sets the byte at offset 0 to 16 and writes 56 at offset 22.
- `set_entity_state_write_bytes`: Writes 2012 into the 16-bit field at offset 10 and calls another routine.
- `set_entity_timer_and_mark`: When a flag bit is set and the entry is not in state 60, sets the 16-bit field at offset 10 to 1380, or to a value from the global table when bit two of the flags byte is set, calls another routine, and sets the byte at offset 22 to 60.
- `set_timer_and_call_state`: Writes 1380 into the 16-bit field at offset 10 of the entry, calls another routine, then sets the byte at offset 22 to 60.
- `mark_matching_entry_in_cell`: Searches the cell at the two coordinate inputs, when both lie below 64 and a flag bit allows it, for a type-4 entry whose key byte matches the input, and sets bit four of a match.
- `mark_matching_entry_in_list`: Walks the linked list from the entry's link field and, when it finds a type-4 entry whose key byte matches the input, sets bit four of that entry's flags and returns one.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a0, then returns.
- `stub_undecompiled_handler_routine`: A ten-instruction routine with one call, referenced from a handler pointer and from data.
- `stub_undecompiled_seven_instruction`: A seven-instruction routine with one call, referenced from data.
- `decrement_timer_and_expire`: Subtracts a value taken from a second structure at offset 20 from the counter at offset 26 of the entry, and returns zero while the counter stays positive.
- `set_entity_state_return_one`: Sets the byte at offset 22 of the entry to 44 and returns one.
- `return_zero_result_2`: Returns zero with no other work.
- `compute_step_distances_and_ratio`: Compares the entry's stored coordinate bytes with the two input values and computes the distance on each axis, storing a reduced distance for each in the words at offsets 14 and 16.
- `store_scaled_quotient_field`: Divides the second input by the word in the third data register and stores the quotient in the 16-bit field at offset 20 of the entry.
- `normalize_two_byte_fields`: Compares the entry's stored bytes at offsets 6 and 8 with the high bytes of the two inputs.
- `initialize_free_record_dispatch`: Finds the first unused 52-byte record, marks it used, sets its state to 60 and its flags byte to 68, clears its counters, and stores the two byte inputs and the word input.
- `initialize_record_and_dispatch`: Initialises the record at the address register by setting its state to 60 and its flags byte to 68, clearing its counters, and storing two byte inputs and a word.
- `return_without_work_2`: Returns immediately with no instructions.
- `validate_and_update_byte`: Checks the entry's byte at offset 1 and a value derived from the entry's position against a flag table, and calls another routine when the check passes.
- `forward_routine_result_word`: Calls another routine and returns its 16-bit result unchanged.
- `scatter_table_bytes_by_offset`: Walks a table of words that ends at the value minus 99, adding a base value to each word.
- `return_without_work_3`: Returns immediately with no instructions.
- `process_masked_table_entries`: Walks a table of offset words until the sentinel minus 99, testing each position against a mask table and a flag word.
- `validate_masked_table_walk`: Walks the same table of positions and masks as the routine before it.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_result_3`: Returns zero with no other work.
- `mark_matching_table_positions`: Walks a table of offset words for a fixed number of entries, and where a position passes the mask test and the byte there equals the input character, sets that byte to 15.
- `write_byte_at_matching_positions`: Walks a table of offset words until the sentinel minus 99, clearing the byte at each position that passes the mask test.
- `iterate_linked_structures`: Walks a set of 314-byte structures and handles each by type: type 4 structures have a counter decremented and reloaded from a second field when it drops below one, calling an external routine, while type 18 structures call routines to follow their chain.
- `process_linked_structure_node`: Checks the counter and link fields of a structure, and when a linked entry's threshold is passed or a timer is ready writes a code of 2 with two bytes from the structure into an output record and returns one.
- `update_linked_structure_neighbor`: Decrements a timer in the structure and, when it expires, searches the grid neighbours of the entry's position for one whose class value passes a comparison with its own.
- `search_matching_class_neighbor`: Searches the neighbouring positions of an entry in the global table for one whose class value equals the entry's own class, writing a state code of 4 and the matched position into the output record when found.
- `search_matching_class_positions`: Walks a list of position offsets from the input, computes each position's class value, and stops at the first one equal to the wanted class.
- `write_output_record_and_count`: Writes the given state byte and a position from the input word into the output record, copies the entry's link into the structure, and decrements a counter there.
- `clear_counter_return_zero`: Clears a 16-bit counter at offset 48 of the structure and returns zero.
- `decrement_flag_and_test_counter`: Decrements a counter in the structure and resets it to a constant when it reaches zero.
- `dispatch_handler_by_index`: When a global counter exceeds 249 and a flag in the global table is set, it takes an index from the structure, or from another routine's result taken modulo a size field, then calls the handler selected from the table and returns its result.
- `find_nearest_flagged_position`: Walks a table of offset words from the entry's position, skipping positions whose table flags lack bit 8, and keeps the candidate whose distance to the entry's word is smallest.
- `find_nearest_flagged_position_2`: Walks a table of offset words from a base position, skipping positions whose table flags lack bit 8, and keeps the candidate whose distance to the register value is smallest.
- `find_nearest_flagged_position_3`: Iterates through a table of offsets, checking certain bitmasks, and tracks the entry that has the smallest absolute distance from a reference point.
- `return_zero_3`: The routine is empty and returns zero.
- `check_entry_byte_match`: Compares two bytes of the entry with the bytes stored at a table position, writing status code 8 with those bytes to the output record and returning one when they differ.
- `clear_flag_if_below_quarter`: Clears the word at offset 38 of the structure when it is non-zero and the word at offset 148 is no greater than a quarter of its value.
- `store_offset_in_table`: Stores the structure's address offset, shifted by a fixed constant, into the global table at the slot chosen by the byte at offset 12, sets bit zero of the byte at offset 13, and returns the first input unchanged.
- `clamp_and_store_two_bytes`: Takes a byte from another routine and clamps it, together with the second input byte, to the range zero to 63.
- `update_table_entry`: Calculates an offset based on a register value and writes a pointer and several byte-sized values or constants into a table.
- `clear_flag_and_zero_table_entry`: Checks a flag byte in a structure; if set, it clears the flag and writes zero to a table entry indexed by a byte field in the structure.
- `update_struct_properties`: Calculates a table offset or pointer, modifies a structure using constant bytes or values from a table, and updates a pointer or certain bytes within a structure based on conditional checks.
- `write_struct_constants`: Stores constant bytes and a value from a global table into a structure pointed to by a register.
- `write_struct_bytes`: Stores two constant bytes into a structure pointed to by a register.
- `update_linked_list_node_state`: Iterates through a linked list of structures, checking flag bits and comparing a value from a structure to a value stored at an offset in a list element.
- `traverse_linked_list_and_modify_state`: Iterates through a list of pointers, checking certain flags and values, and potentially updates a structure or writes a constant or pointer to a memory location.
- `update_entity_state_via_list`: Iterates through a linked list of entities, checking for specific type flags and comparing a value against a calculated threshold.
- `traverse_linked_list`: Traverses a linked list while checking certain bounds and flags, potentially updating structure fields or links.
- `validate_and_update_list`: Traverses a linked list while checking certain bounds and flags, potentially updating a structure's fields or updating a link with a value from a table.
- `return_constant_3`: The routine returns a constant value.
- `process_structure_update_and_dispatch`: Retrieves a structure pointer from a table, modifies specific flag and character bytes within that structure, and then executes a function pointer retrieved from a global table.
- `update_structure_and_dispatch`: Modifies certain fields in a structure and executes a function pointer retrieved from a global table.
- `update_and_dispatch_handler`: Clears certain flags and table entries, modifies a structure, and then executes a function pointer retrieved from a global table.
- `find_nearest_structure`: Searches a list of structures for one that satisfies certain distance and flag criteria, then updates a pointer.
- `set_structure_offsets`: Stores a calculated offset into a structure and updates a byte in that structure before returning.
- `return_null_value`: The routine is empty and simply returns a zero value to the caller.
- `update_linked_structure_fields`: Checks a structure or a pointer-linked list for specific non-zero values and then either updates fields in a structure or writes constant values to several fields of a structure.
- `initialize_struct_fields_2`: Stores constant values into several fields of a structure pointed to by a register, including a word and two bytes to certain offsets and a word at a fixed offset.
- `link_and_update_entities`: Iterates through a list, compares a byte against a value in a structure, and writes certain bytes or values to a structure or links two entries depending on a conditional check.
- `update_pointer_data`: writes two consecutive bytes to a memory location pointed to by a register.
- `update_tile_bounds_and_properties`: Checks if a coordinate or size change or an invalid state has occurred, then updates a global table or structure with a new pointer and modified dimension or property bytes.
- `update_tile_data`: Updates a global table entry with a pointer and certain bytes from an input structure.
- `set_tile_properties`: Computes an index from a byte in an input structure and writes several fields into a global table entry.
- `clear_entity_links_and_flags`: Iterates over a linked list, clearing link pointers and updating flag and state bytes for each entry, and returns the value originally in a register.
- `copy_and_reset_table_entries`: Copies a range of word-sized entries from a source to a destination and clears certain fields in the destination structure.
- `copy_word_table_entries`: Copies a fixed number of word entries from a source pointer to a destination pointer, clears a few fields in the destination structure, then calls another routine with the source and destination pointers.
- `compute_indexed_value`: The routine reads a 16-bit value from a global variable table using an index derived from an input word, then optionally adjusts it by subtracting a division of itself by a byte taken from another table indexed by a second input word and the first index.
- `retrieve_stack_values`: Checks a condition involving a global variable and a table, calls another routine if necessary, and returns a 32-bit value formed by two 16-bit values from the stack.
- `return_stack_values_4`: The routine returns a 32-bit value formed by concatenating two 16-bit values from the stack.
- `update_list_structure`: Updates a linked list structure or inserts a new entry into a linked list.
- `set_node_value`: Checks if a pointer or address is within a certain range and, if so, sets a byte at an offset to zero.
- `clear_node_offset_2`: Checks if a pointer or address is within a certain range and, if so, sets a byte at an offset of a certain amount from that pointer to zero.
- `clear_node_offset`: The routine sets a byte at an offset of certain or a certain amount from a pointer to zero and then calls another routine.
- `call_list_update`: Performs a call to a routine that updates a linked list structure.
- `update_linked_list_structure`: A routine that updates a value within a structure and potentially inserts a new entry into a linked list.
- `insert_linked_entry`: Updates a value in a structure and inserts a new entry into a linked list if certain conditions are met.
- `initialize_linked_entry`: Sets several bytes and a word in a structure, inserts a new entry into a linked list, and calculates a new pointer or offset for a structure field.
- `return_zero_4`: the routine returns a zero value without performing any operations.
- `initialize_empty_entry_slot`: The routine scans a fixed-size table of 32-byte entries looking for an empty slot and returns when none are free.
- `traverse_list_and_insert_entry`: Traverses a list, checking for a sentinel value, and if a certain condition is met, updates a structure and inserts a new entry with several initialized fields or returns a value from the stack.
- `update_list_with_offset_and_return_stack`: Traverses a list to update a byte if a condition is met and returns a value from the stack.
- `update_list_with_offset`: Updates a structure with a new value and traverses a list to update a certain byte if a condition is met.
- `return_stack_value_8`: Returns a 64-bit value directly from the stack.
- `return_stack_value_2`: The routine returns a 64-bit value from the stack.
- `initialize_empty_node`: Searches a linked list for an empty entry and initializes it with a fixed set of values and a pointer.
- `find_empty_structure`: Searches a linked list or table for an entry where a specific byte is zero, then initializes that entry with fixed or register-derived values.
- `initialize_sprite_structure`: The routine writes a set of fixed byte and word fields into a structure pointed to by a register, including a byte from a register into a field of the structure and a pointer derived from another register into a word field of a different structure, then calls another routine.
- `update_linked_pair_colors`: Updates certain color bytes in a structure or certain fields in a linked list entry based on a table lookup.
- `update_linked_pair_entries`: Repeatedly calls another routine to fetch a value used to index a table, then uses the fetched bytes to update two color bytes in a structure, clamping them to a valid range, and links two structures by writing addresses into certain fields.
- `clear_active_entity_chain`: The routine checks a short entry from a table indexed by a byte in a register.
- `update_entity_bounds_and_initialize`: Checks a table or boundary condition and either initializes a new entity entry in a free-list or expands the existing entity's boundary dimensions and updates associated offset or size fields.
- `expand_structure_bounds`: Updates certain size or boundary fields in a structure based on a comparison with a new value.
- `update_entity_bounds`: Calculates a new value based on a difference and compares it against a byte in a structure, potentially updating that byte, and copies bytes between certain offsets in a structure.
- `initialize_entity_data`: Searches a list for an empty entry and initializes it with fixed bytes and values derived from registers, including a pointer or offset calculation.
- `initialize_entity_struct`: The routine writes a set of fixed bytes and two bytes derived from the inputs into a structure pointed to by a register, copies a value from a global variable adjusted by a bit-shifted byte from a pointed-to record, stores a constant into a field of that record, and writes an offset back into the record.
- `set_object_header`: Checks a byte in a structure and, if certain conditions or flags are met, writes a constant or a value from a table into a structure field, and sets a pointer or offset or an identifier or a certain byte.
- `initialize_object_header`: The routine writes constant values into fields of a structure pointed to by a pointer, stores a computed short derived from a word input, and returns the value passed on the stack.
- `reset_object_state`: Checks a flag in an object, and if non-zero, resets certain fields and updates a pointer with a value from a table.
- `iterate_table_entries`: The routine walks a table of structures, clears a status byte when a condition based on a lookup table is unable to be met, updates a global counter, optionally copies a 16-bit value from the entry to a table indexed by the status byte, and recurses via another routine; otherwise it advances a short offset within the entry if a bounds check passes.
- `update_map_node_links`: Recursively traverses a linked list of map nodes to check for certain bitmask conditions and updates node pointers or flags.
- `traverse_and_update_map_node_2`: Recursively traverses a linked structure or tree-like dataset, updating structure fields and clearing links based on bitmask checks.
- `traverse_and_update_map_node`: This routine performs a recursive or iterative traversal of a linked-structure or tree-like data-set, updating specific fields in a target structure or object while checking or setting flags in a global table.
- `update_structure_fields`: Checks a bitmask in a global table and updates certain fields in a structure, potentially clearing a link or updating a value from a global table.
- `traverse_and_modify_structure`: Iterates through a linked list of pointers to update certain fields in a structure based on certain bitmask checks or values in a global table.
- `traverse_linked_list_and_update_fields`: The routine traverses a linked list of pointers, updating certain fields in a structure and checking a bitmask in a global table, then returns a value from the stack.
- `return_stack_value_3`: The routine is empty and returns an undefined value currently residing on the stack.
- `process_structure_list`: Iterates through a list of structures, performing bitwise masking and updating certain fields if a specific bitmask matches.
- `fill_structure_with_calculated_data`: Calculates a value using a division/remainder pattern, traverses a linked list, and writes several fields or a calculated value into a structure.
- `fill_object_entry`: The routine repeatedly calls another routine, combines the returned value with a global word and writes several fields through a pointer to a global variable.
- `update_linked_list_structure_2`: The routine calls another routine that updates a value within a structure and potentially inserts a new entry into a linked list.
- `reset_structure_state`: Traverses a list of pointers, resetting certain structure fields and potentially updating a value from a global table or clearing a field if a specific character is encountered.
- `reset_object_state_and_update_pointers`: The routine resets several fields in a structure and traverses a list of pointers, potentially updating a global pointer with a value from a table and modifying a field in another structure.
- `initialize_entity_record_2`: The routine checks if certain byte values fall within a specific range and then traverses a list of structures to find an empty entry, where it writes several bytes and words from registers and a global variable into that structure.
- `initialize_entity_record`: Writes several bytes and words from registers and a global variable into a structure pointed to by a pointer, then calls another routine.
- `get_next_valid_index`: Uses a value from a global table to determine a new index, potentially calls another routine to find a valid value, and updates a byte in a global table.
- `combine_high_word_with_flag`: The routine takes a 32-bit value in D0 and a flag in D5 and, unless the flag equals one, calls another routine and uses the 16-bit result from that call as the high word.
- `return_maximum_value`: The routine returns a constant value representing all bits set.
- `store_byte_from_pointer_and_decrement`: The routine copies a byte from a source pointer offset by a constant to a destination pointer offset by a constant, then subtracts a constant from the destination byte.
- `swap_table_entry`: The routine builds an index from a byte in D0 and a byte in D1, uses it to address a global variable table, tests a flag byte at an offset derived from that index, and if the flag is set replaces the table entry with the value stored at the unable to be determined index given by the entry itself.
- `update_property_bitmask_and_attribute_2`: Iterates through a table of pointers to check for a specific bitset condition, then updates a property bitmask and a corresponding attribute or bitmask in a global array if certain conditions are met or if a specific value is found in a linked list.
- `update_property_bitmask_and_attribute`: Searches a table for a specific value and, if a condition involving a bitmask in a property table is met, updates a bitmask in a global array.
- `update_attribute_bitmask_2`: Checks a bitmask in a property table and updates a corresponding bitmask in a global array if a certain condition is met.
- `update_attribute_bitmask`: Iterates through a list of pointers to check a specific bit in a property-based table and updates a corresponding bitmask in a global array.
- `process_table_and_update_fields`: The routine traverses a list of pointers until a terminator is found, updates a field in a structure, and then iterates through a global table to check bitmasks and update a pointer-based field.
- `return_constant_4`: The routine returns a constant value of one.
- `iterate_table_entries_2`: This routine computes a start offset from an index and a table, then loops a calculated number of times.
- `compute_and_mark_table_entries`: The routine computes an index from a global variable and another routine's result, then iterates that many times.
- `update_list_values`: Calculates a value using a pointer-based lookup, then iterates through a linked list to update certain fields or mark an entry with a specific byte.
- `insert_linked_list_node`: Iterates through a global list to find an insertion point or a terminator, then initializes a new node with values or pointers copied from a lookup table.
- `initialize_and_insert_list_node`: Initializes a node in a linked list by setting certain property bytes and calculating or copying values from a lookup table, then inserts the node into a global chain.
- `return_empty_routine_2`: This routine performs no operations and immediately returns to its caller.
- `update_struct_and_search_list`: Iterates through a list of structures, and if a null terminator is found, it initializes several fields in the structure or writes a value from a table-lookup based on an index.
- `parse_input_to_struct`: Repeatedly calls another routine, extracts low bytes from its return value, builds a masked word, and if the mask is zero, writes several bytes and words into a structure pointed to by a register using a table lookup for one field.
- `initialize_struct_element`: Iterates through a linked list or array of structures to find an entry where a specific byte field is zero, then populates that structure with a byte and two bytes from registers.
- `initialize_struct_from_registers`: Writes a byte and two bytes from registers into a structure, computes a short value via a table lookup, and writes a constant byte.
- `update_map_attribute_table`: Iterates through a memory range or table, checking bitmask patterns and updating or clearing values in a global array based on calculated offsets.
- `update_table_with_masked_values_2`: Iterates through a memory range and checks if a bitmask or certain bits in a combined byte value match a certain pattern, then updates a table or global variable based on a calculated index and a masked value.
- `update_table_with_masked_values`: Iterates through a memory range and checks if a bitmask or certain bits in a combined byte value match a certain pattern.
- `increment_pointer_field`: Increments a word located at an offset of certain bytes from a pointer held in a register.
- `process_global_table`: Executes a loop that decrements a value, then iterates through a table to check if a certain bitmask or value matches a byte in the table.
- `execute_modulo_decrement_loop`: Takes the result of a routine modulo a global variable to use as a loop counter, then repeatedly calls two other routines while decrementing that counter until it reaches negative one.
- `search_and_insert_linked_node`: Iterates through a linked list to find a node containing a specific identifier, then initializes a new node or modifies an existing one with a weight or value.
- `append_list_node_with_random_weight`: Iterates through a linked list to find an empty entry and initializes a new node with a calculated weight or value.
- `initialize_list_entry`: Initializes certain fields in a structure and inserts a new entry into a linked list head, while setting or calculating certain numeric fields.
- `return_stack_value_4_2`: Sets a certain memory location to zero and returns a value previously stored on the stack.
- `return_stack_value_4`: Returns a value currently stored on the stack.
- `empty_routine_2`: This routine contains a small number of instructions and does not perform any visible or documented operations or calls.
- `initialize_structure_from_input`: Checks a condition against a mask and a null terminator, then searches a list to find an empty structure to populate with certain values and a value from another routine.
- `update_struct_from_input`: Writes several bytes or words into a structure pointed to by a register, calls another routine, and returns a value from the stack.
- `return_stack_value_5`: This routine returns an undefined value from the stack.
- `tiny_ori_3`: Small routine of 1 instruction that merges 100 into (a2).
- `modify_structure_fields_2`: Searches a list for a structure with a null terminator and writes a byte, a zeroed dword, and a short or byte to that structure's fields.
- `modify_structure_fields`: Writes a byte, a zeroed dword, and a short or byte to a structure, sometimes conditional on a check against a global variable.
- `return_zero_5`: the routine returns a zero or null value to its caller.
- `set_flag_for_linked_entries`: Walks a linked list of short entries, checking a byte field and a comparison byte before setting a flag byte and writing a constant word.
- `update_global_byte`: Computes an index using a register and a pointer, checks if certain bits in a global table are set, and if so, writes a constant byte to that table.
- `return_stack_value_6`: This routine returns a value currently stored on the stack.
- `process_global_table_entries`: Walks a global short table terminated by a specific value, and for each entry, computes an index, follows a linked list of entries, and updates several fields or calls another routine if certain conditions or type bytes match.
- `clear_byte_at_offset`: stores a zero byte at an offset of twelve from a pointer held in a register.
- `iterate_handler_table`: walks a table of handler pointers stored in a global variable, calls each handler, and clears two bytes in the returned structure until a certain address is reached.
- `store_byte_from_register`: stores a byte from a register through a pointer.
- `check_and_update_table_entry`: Checks a 16-bit value from a table indexed by a global index against a range, and if in range, advances the index, updates a global variable, and copies bytes from a fixed memory area.
- `initialize_handler_execution_environment`: Initializes several global variables and counters, then executes a sequence of handler pointers while iterating through a table of pointers and potentially traversing a linked or pointer-based structure.
- `process_state_buffer_stream`: Iterates through a buffer or stream, checking for specific flag conditions and updating a global state or pointer.
- `execute_state_steps`: Loops over helper steps on a global state word, reading bytes from a ring buffer, calculating absolute differences, and updating a global flag or pointer based on certain results or return values.
- `return_stack_value_7_2`: Calls another routine and returns an 8-byte value that was previously passed on the stack.
- `return_stack_value_7`: calls another routine and returns an 8-byte value that was passed on the stack.
- `update_global_pointers`: Loads several values from global memory and, if a certain condition involving a byte in a structure is met, calls another routine multiple times and performs bitwise OR operations on global variables.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 1 into d0, then returns.
- `reset_tile_flags`: calls another routine, then iterates three times over a table of structures accessed via a global variable, updating a byte at offset eight to a certain value if it is non-zero, and finally stores a value into a global variable.
- `process_recursive_structure`: iterates through a list of pointers, clearing certain values and calling other routines, and then iterates through a table of pointers to perform conditional calls and negate values.
- `process_recursive_structure_alt`: performs a sequence of pointer-based clearing and conditional calls, and then iterates through a table to modify values via negation.
- `clear_flags_and_process_table`: Iterates over a block of short flags starting at a global variable, clearing each non-zero flag and invoking another routine, with an extra call when a related entry is set.
- `toggle_and_recurse`: Calculates a relative offset to a code location, toggles a value at that location, and recursively calls itself with a modified bitmask.
- `execute_power_of_two_loop`: executes a loop that calls another routine with a bit-set address, doubling a value each iteration, and then conditionally calls another routine if a certain global variable is non-zero.
- `iterate_bit_mask_with_conditional_call`: Calls another routine repeatedly with a flag that doubles each iteration while a short value stays below a threshold, then conditionally calls the same routine once more if a global variable is non-zero.
- `update_global_flag_and_table`: The routine checks a global variable and, depending on its value, calls another routine twice with values built from a table and bit operations, or calls another routine twice with fixed constants.
- `update_tile_flags_and_neighbor`: The routine indexes a global variable table with an index from the first argument and calls another routine twice with values derived from the entry and a global table, then if a short field is non-zero it repeats the calls for the referenced entry.
- `check_flag_and_call_twice`: Checks a global flag and, if non-zero, writes the flag plus a constant to a global variable, stores the address of a global variable into another global variable, then calls another routine twice.
- `initialize_interrupt_handlers`: Writes a routine pointer and a global variable pointer to multiple memory locations, sets two bytes in RAM, registers an interrupt server, calls an indirect routine through a global variable, and then writes a sequence of bytes to a global variable before returning the concatenated input registers.
- `disable_interrupt_server`: checks a global variable, then sets a global variable to a certain value, calls a routine via a pointer, and finally sets a global variable to a different value.
- `set_unit_flags_by_mask`: Iterates over four entries in a table of unit structures and, for each entry selected by a bitmask, applies a set of flag and byte writes via pointers stored in the table.
- `search_table_for_match`: walks a table in steps of three shorts, stopping when it finds an entry equal to a provided short value or a zero terminator.
- `handle_data_pointer`: The routine receives a pointer in a register, loads a global variable and stores the pointer value into a table entry via an indirect address derived from the pointer.
- `process_flagged_entities`: Iterates over a table of structures and, for certain flagged bits in a 64-bit value, updates several bytes in the structure or clears certain hardware or global registers.
- `configure_dma_channels`: Configures DMA controller registers, such as the channel enable and address or count registers, using values derived from a pointer and a lookup table.
- `configure_dma_channel_parameters`: Configures a DMA channel by writing a pointer-derived value and a bitmask to the DMA controller, while simultaneously updating a table or a global variable with new pointers and bitmasking values.
- `update_entity_hardware_registers`: Updates a table with new pointers and values derived from a lookup table while simultaneously updating hardware registers and a global bitmask.
- `update_entity_table`: Updates a table with new values and pointers derived from a lookup table, while also updating a global bitmask and hardware registers.
- `update_position_and_flags`: Updates a pointer or value based on an offset, modifies a bit in a byte, and increments certain counters or values if specific conditions or certain bit-flags are met.
- `set_byte_bit`: Sets the fourth bit of a byte at an offset from a pointer.
- `update_map_node_traversal_2`: Updates a pointer-based list of map nodes by checking status flags, updating a counter, and potentially reassigning a pointer to a new node or a value from a global table.
- `update_map_node_traversal`: Iterates through a linked or pointer-based list of map or object nodes, updating status flags and pointers, and potentially resetting certain object or cell properties or clearing certain status flags or or resetting pointer-based or numeric fields.
- `traverse_and_update_map_nodes`: Iterates through a list of map or object-related pointers, performing recursive or nested lookups and updating status flags and pointer-based metadata in a linked structure or table.
- `update_entity_state_2`: Iterates through a table of bytes to update or reset certain properties of an object or entity.
- `update_node_properties_and_neighbor_state`: Updates a node or cell structure by clearing specific status flags, resetting certain pointer-based or numeric fields, and updating a neighbor's property or a global minimum value.
- `update_map_cell_state_2`: Modifies a map tile or node structure by clearing specific status flags and setting others.
- `update_map_cell_state`: This routine modifies flags in a structure representing a map tile or cell, specifically clearing certain status bits and setting others.
- `process_map_tile_node`: Calculates a pointer to a tile or node in a map structure and initializes a temporary data structure or node or state with its properties.
- `empty_routine_3`: the routine contains no instructions and performs no operations.
- `update_pointer_with_min`: Compares a byte from a pointer to a value at another pointer and stores the minimum or an offset-calculated value into a global pointer.
- `clear_dma_control_register`: The routine writes zero to the DMACON hardware register and sets a global variable to a bit mask derived from an input value.
- `update_map_node_traversal_3`: Iterates through a pointer-based list of map or object nodes to update status flags and pointers, potentially resetting certain node properties or resetting a counter.
- `initialize_map_node_properties`: Resets or modifies status flags and pointer-based data within a structure associated with a map or object node.
- `initialize_node_traversal_state`: Initializes a node or object structure with cleared or reset flags and pointers while traversing a pointer-based list.
- `traverse_map_node_list`: Iterates through a linked list of map or object nodes to update status flags, reset certain node properties, and potentially update pointers or certain node-related metadata or or any associated global table or variable.
- `transform_string_and_set_pointer`: Iterates through a source string to apply a bitmask to certain characters, storing them into a destination, and updates a global function pointer.
- `transform_and_copy_string`: The routine iterates through a source string, applies a bitmask to certain characters, and stores the modified characters into a destination buffer.
- `call_indirect_and_store`: The routine calls an indirect routine via a pointer, stores a value from a register into a global variable, and then calls another routine.
- `execute_conditional_callback`: The routine executes a series of indirect calls or conditional logic involving a function pointer and an integer check, potentially updating a global variable or checking a return value.
- `update_handler_and_test_library_return`: Overwrites a global function pointer with a specific routine and a constant, then executes a library function to check its return value.
- `evaluate_library_and_set_global`: Calls a library function and, if the result is not zero, checks if it equals a specific constant before storing a value and updating a global pointer.
- `evaluate_library_result`: The routine calls two library functions and returns a value containing a constant or zero based on a comparison, while also updating a global variable with a function pointer.
- `set_global_and_return_constant`: The routine stores the value from a global variable into another global variable and returns a constant value built from a constant and an uninitialized stack value.
- `reset_interrupt_state_handler`: Updates a global function pointer and then restores a previous function pointer from a table.
- `update_global_pointer_and_branch`: Retrieves a function pointer from a table, executes it, and then calls another routine.
- `store_global_and_call`: Stores a value from a global table into a global variable and then calls another routine, returning a 3lau32-bit value constructed from an input register and the stack.
- `update_per_instance_records`: Updates a per-instance record in a global table by temporarily changing a global control flag and calling a handler with pointers into that record.
- `transform_buffer_data`: Iterates through a buffer to copy and manipulate data, involving bitwise negation and block transfers of values.
- `check_and_update_two_flags`: The routine calls a function through a pointer stored in a global variable, checks the returned value, and if positive stores it into two global variables and a pair of memory locations, then calls another routine.
- `invoke_callback_and_return_value`: The routine checks a global variable and, if positive, calls a routine via a function pointer stored in a table.
- `copy_250_bytes_forward`: The routine calls another routine and then copies 250 bytes forward from one location in memory to another, using a signed counter that starts at 249 and decrements to -1.
- `copy_bytes_from_table`: The routine walks a table of entries in a global variable, each entry containing a source offset and a length.
- `increment_global_counter`: The routine calls another routine, increments a global variable by one, and returns a 64-bit value formed by concatenating the two input registers.
- `do_nothing_3`: this routine is empty and performs no operations before returning.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:ip_intro

- `initialize_library_pointers`: Retrieves a library pointer from a global variable, calls it, and then updates a global variable to point to itself.
- `initialize_hardware_and_memory`: Sets specific hardware or memory-mapped registers and enters loops that wait for certain bit patterns or register states to clear or change.
- `copy_string_block_2`: Iterates through a source string until a newline character is found, then copies a fixed number of bytes from a source table to a destination offset.
- `copy_string_block`: Copies a block of memory from a source to a destination and updates a pointer or offset or a global variable.
- `copy_string_and_calculate_offset`: Iterates through a string to find a newline, calculates a destination address based on string length, and copies a block of bytes from a source table to a destination.
- `return_unmodified_value`: The routine is empty and returns an uninitialized value from the stack.
- `update_table_with_offset_logic`: Decrements a global counter and, if zero, resets it and copies a sequence of words from a table using a fixed stride.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Populous II - Trials of the Olympian Gods (1991)(Electronic Arts)[cr VF][f IP](Disk 2 of 2).adf:bootblock

- `check_library_function_return`: calls a function pointer from a table and checks if the return value is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `invoke_library_functions`: executes two separate calls to functions within a system library.
