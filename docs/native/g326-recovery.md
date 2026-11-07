# Game 326 (Trump Castle): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00326). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 32 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 283 of 283 named (0 author symbols), 283 of 283 described (74 by rule, 208 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1479 of 1479 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 23040 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 29654 of 29657 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 426163 |
| code | 115182 |
| data | 119214 |
| padding | 6641 |
| unused | 233984 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Trump Castle.adf:type | 2 | 117 | 2 |
| Trump Castle.adf:Trump_Castle | 3 | 29529 | 279 |
| Trump Castle.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| allfruit (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Trump Castle/Trump Castle.adf:data/allfruit: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 23040 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `generate_ascii_character` (Trump_Castle): Calculates an ASCII character value using a combination of arithmetic operations on registers and checks if the result falls within certain ranges. Kind: checksum/manual lookup trap. Pass path: the branch where uStack_c == uStack_10, resulting in the return or exit via u1__sub_0_20620.
- `validate_global_state_range` (Trump_Castle): Checks if a global variable falls within a large number of specific, non-contiguous valid ranges. Kind: range-based checksum/value trap. Pass path: the branchs where the conditional checks fail (i.e., the code skips the nested `if` blocks or proceeds to subsequent logic, rather than entering the failure-state nested blocks).

## Functions


### Trump Castle.adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: copies a fixed number of words from a source pointer to a destination pointer.

### Trump Castle.adf:Trump_Castle

- `initialize_memory_and_dispatch_calls`: Performs a zero-fill loop on a memory range and conditionally calls a function pointer or a library function based on a global variable or a return value.
- `initialize_config_and_string_tables`: Initializes or resets global data tables and string metadata, or shifts existing table entries and clears memory if certain validation checks fail.
- `process_string_instructions`: Executes a sequence of instructions that modifies a set of local or global counters based on certain return values.
- `parse_string_tags_to_buffer`: Sets global flags and calls a routine multiple times to process string tags or buffers using a local stack buffer.
- `initialize_table_limits`: Sets or resets certain memory table or array limits and initializes a range of memory or table entries to a constant maximum value.
- `update_global_state`: Sets certain global flags to a constant value and calls a routine that appends strings to a local buffer.
- `initialize_string_table_and_validate_data`: populates a table with string fragments or values and performs a check or loop involving string copying and pointer-based validation.
- `generate_ascii_character`: Calculates an ASCII character value using a combination of arithmetic operations on registers and checks if the result falls within certain ranges.
- `execute_fixed_sequence`: Executes a fixed sequence of three routine calls, including one with a zeroed argument.
- `traverse_and_update_structure`: Recursively traverses a linked or nested structure to check for a zeroed entry, then updates a target structure with new or cleared data if a match is found.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d1.
- `initialize_global_flags_and_structures`: Initializes a global state or structure by setting several flags and calling a routine to allocate or configure memory-related structures.
- `evaluate_global_flag_and_update`: initializes a local structure with a specific set of constant and global values, calls a library function, and then updates a global variable based on the returned value and a conditional bitwise or operation.
- `modify_global_bit`: sets or clears the least significant bit of a byte in a global variable based on the input parameter.
- `dispatch_conditional_calls`: Checks various bits of a global variable and calls different routines or library functions with specific arguments based on those bits.
- `process_event_handler`: Iteratively processes a pointer from a global structure, calls a routine with a bit-shifted value, and modifies a global variable based on certain values found in a table.
- `process_event_loop`: Iteratively retrieves a pointer from a global structure to call a routine with a bit-shifted value, while updating a global variable based on a certain value.
- `traverse_linked_routine_chain`: Follows a chain of pointers retrieved from a global structure, calling another routine for each non-zero pointer found.
- `execute_table_iterations`: Iterates a fixed number of times, calling a routine with a pointer from a table and an incrementing counter.
- `call_library_with_offset_parameter`: Calls a library function using a pointer or base address retrieved from a global variable plus a fixed offset, passing a constant length or count as a second argument.
- `execute_string_parsing_sequence`: Calls a routine using a pointer from a global structure and then invokes a routine that parses a list of string tags using a local buffer.
- `execute_conditional_fill`: loads data from a pointer into a local stack, calls a library function, and potentially calls another library function if a parameter is non-zero.
- `store_parameters_to_stack`: Stores passed arguments onto the stack and calls another routine with a pointer to a local stack variable.
- `execute_recursive_function`: Executes a function pointer from a library, then recursively calls itself using a pointer from a global table or a provided argument.
- `copy_words_with_stride`: copies a sequence of words from a source or a table to a destination buffer using a stride-based loop.
- `calculate_and_call_range`: Calculates a range or count from two parameters and calls a library routine with that calculated value and a range of parameters.
- `call_library_with_range`: Calls a library routine with a range or count, then recursively calls another routine using the return value.
- `populate_array_from_result`: Calculates a value using a routine, then iterates through a loop to store certain values into an array if a condition is met.
- `iterate_and_modify_table`: Iterates a certain number of times, performing calculations and updating a table or memory via a pointer.
- `get_library_result`: calls a library function and returns the value written to a local variable.
- `insert_word_into_array`: Retrieves a word from a table and inserts it into a memory array by shifting existing array elements backwards via a decrementing loop.
- `copy_memory_backwards`: Decrements a byte and copies a word from one memory location to another until a certain value is reached, then calls another routine.
- `update_memory_with_register`: Stores a value from a register into a memory location and then performs an indirect library call.
- `call_library_with_count`: Calculates a count from two parameters and calls a library routine using a pointer from a global table.
- `call_library_with_parameters`: Calls a library routine using a specific range or count and then recursively calls another routine.
- `execute_library_calls`: calls a library function twice using a pointer retrieved from a global variable.
- `traverse_pointer_chain`: Follows a chain of pointers by loading a value from a memory address, adding an offset, and dereferencing the resulting pointer twice.
- `build_string_structure_list_2`: Calls a routine to build a list of structures from a string and then calls another routine using the returned pointer.
- `parse_string_tags_and_validate_metadata`: Parses a string tag sequence to extract or validate configuration-related metadata, such as string lengths and buffer limits.
- `evaluate_state_transition`: Evaluates a chain of pointers and checks certain bitmask conditions or value ranges to determine a new state, potentially updating a global variable or calling other routines based on the result.
- `traverse_linked_lists`: Iterates through a nested linked list structure, calling another routine for each element or pointer encountered.
- `initialize_blinker_struct`: Initializes a structure or object or table with specific integer values and sets certain function pointers, including a pointer to itself, while also performing a series of initial data-population calls using hardcoded parameters or previously computed values.
- `initialize_game_state`: Initializes several global variables and structures, clears a small array, and configures an initial state for a conditional logic branch.
- `empty_routine`: the routine contains no executable instructions and performs no operations.
- `empty_routine_2`: The routine contains no executable instructions and performs no operations.
- `load_data_from_pointer`: Clears a 16-bit word at the address pointed to by a5, loads the value at the address in a0 into d0, and then loads the longword at the same address in a0 into d0.
- `configure_global_parameters`: Sets a local variable based on an input value and calls a routine multiple times with a pointer to a global variable and fixed parameters.
- `calculate_variable_offset_and_bounds`: Calculates or updates a global variable and certain stack-based offsets or boundaries by repeatedly calling other routines and checking bitmask-filtered return values.
- `execute_iterative_audio_setup`: Calculates a loop count based on a masked return value, then repeatedly initializes an audio timer with a masked global value and executes a delay-based routine a fixed number of times.
- `reconfigure_data_buffer_layout`: This routine copies and reorders blocks of data from one location in a global buffer to another, using a stride-based offset determined by the input parameter.
- `update_graphics_primitives`: updates or draws a series or sequence of primitive shapes or lines by updating a destination buffer or display area.
- `update_card_values_and_check_patterns`: Updates a table of five integer values by incrementing or decrementing them based on a provided parameter, then checks the new values against hardcoded patterns to update a secondary temporary table and a global flag.
- `initialize_and_populate_data_buffer`: Initializes a buffer by checking for valid pointers or certain input conditions, then copies a sequence of words or or larger data blocks from a source table or memory range into a destination array, or updates a structure with a specific amount of data or a calculated range.
- `handle_exit_prompt`: Checks a global variable against several constant values and recursively calls another routine with a pointer to that global variable.
- `update_global_and_recurse`: Updates a global variable by adding a parameter and then executes a sequence of recursive calls using values or pointers retrieved from a table and a conditional branch that selects different constants or registers.
- `empty_routine_3`: the routine is empty and performs no operations or data transfers.
- `process_event_chain`: Iteratively traverses a linked chain of routine pointers and processes event handlers.
- `traverse_state_tree_recursively`: A recursive routine that updates a global state variable and a secondary value based on a lookup table, then calls itself or other routines with modified parameters or or new state-dependent offsets.
- `branch_on_global_value`: Checks a global variable and sets local stack values before calling a routine with a calculated range or a multiplication routine.
- `update_state_table_entry`: updates or resets values in a table or global variable based on a range-checked state or index.
- `parse_string_tag_list`: Parses a string containing a delimited list of substrings separated by pipes and enclosed in brackets, storing the resulting string pointers into a table.
- `initialize_game_data_tables`: Initializes several memory-mapped tables or arrays to zero and populates them with data from hardcoded or pointer-based sources.
- `clear_register_d1`: Sets the d1 register to zero.
- `initialize_game_state_and_process_events`: Initializes a large memory-mapped table or array to zero and enters a main event loop that dispatches or handles certain event codes.
- `initialize_betting_limits_and_history_2`: Sets or resets betting limits and clears a 2D array or tracking history.
- `initialize_betting_limits_and_history`: Sets betting or chip value limits based on a global configuration and clears a 2D array used for tracking recent results.
- `execute_recursive_calls`: Performs a sequence of recursive or indirect calls using a global variable and a passed parameter, returning a value obtained from a previous routine execution.
- `update_global_state_2`: Updates a global variable or a value in a table using a sequence of conditional assignments based on a parameter and the result of a recursive or nested routine call.
- `fetch_word_and_decrement_pointer`: Loads a 16-bit word from a table using an offset derived from a base address in a0 and a value in d0 into d0, then performs a bitwise OR on a byte at the address pointed to by a5 while decrementing the pointer in a5.
- `traverse_node_tree`: Iterates through a multi-dimensional table or tree structure to check for non-zero values, recursively processing or clearing certain nodes or parameters based on the presence of values or a loop-driven state flag.
- `calculate_masked_sum`: Iterates through a table while masking certain bits and returns a calculated value based on masked register contents.
- `update_attribute_table`: Iterates through a table of values to check or modify certain parameters or pointers.
- `calculate_payout_constant`: calculates a constant value based on a comparison of input parameters and checks for even or odd evenness, then uses the result to index into a table and calls another routine to handle the resulting value or a recursive call.
- `traverse_state_tree`: recursively calls another routine with modified parameters based on the value of an input parameter.
- `calculate_recursive_range`: Calculates a value based on an input parameter and calls another routine with a range or a recursive call.
- `initialize_betting_system_state`: Resets or configures a range of betting values and clears associated memory-mapped buffers or arrays.
- `initialize_betting_state_and_reset_tables`: Resets several memory tables and arrays to zero, then configures betting limits based on a retrieved value.
- `initialize_betting_range_and_history`: Sets up or resets a betting range or chip value based on a configuration value and clears several memory-mapped buffers or arrays.
- `calculate_and_format_payoff`: Checks a table of bet amounts and multipliers, then calls a routine to format a string containing the calculated payoff amount.
- `format_bet_payoff_string`: Checks if a payoff or bet amount is zero or otherwise valid, then formats a string containing a bet amount and its corresponding payoff into a buffer.
- `format_bet_string`: Formats a string containing a bet amount and potential payoff, then appends or copies it to a buffer.
- `update_global_state_3`: Executes a series of calls involving a global variable and a provided parameter, returning the result of a previously executed routine.
- `update_polygonal_state_tree`: Recursively updates a state tree or linked structure by modifying a global variable and branching or recalculating node values based on a provided or calculated index.
- `update_state_tree_node_recursive`: Recursively traverses a state tree or linked structure, updating node values and character-related lookup tables based on conditional checks.
- `update_state_tree_node`: This routine recursively traverses a state tree, updating node values and character-related lookup tables based on conditional checks.
- `evaluate_recursive_state_tree`: This routine implements a recursive tree traversal that updates a table of values based on a conditional check of a lookup or state array.
- `validate_global_state_range`: Checks if a global variable falls within a large number of specific, non-contiguous valid ranges.
- `parse_string_tags`: Iteratively parses string tags and copies string data into a buffer, potentially adjusting the buffer length based on values in a table.
- `process_string_tags`: Iteratively checks a table of values to determine if certain string tags should be parsed or appended to a buffer.
- `execute_conditional_ranges`: Checks certain bits of input parameters and calls a library routine with various ranges or a function pointer from a table.
- `populate_and_update_table`: This routine populates a table with a cumulative sum calculated from a loop, and subsequently updates certain memory locations using values derived from a global variable and library-calculated ranges.
- `initialize_game_data_tables_2`: Initializes a sequence of memory-linked tables or arrays by calculating offsets and propagating values through iterative loops.
- `populate_array_with_random_bits`: Iteratively calls a routine to get values, masks certain bits, and stores those masked values into a local array while updating a global pointer.
- `traverse_function_pointer_table`: This routine recursively or iteratively traverses a table of function pointers, using offsets or calculated indices to call subsequent routines.
- `call_with_offset_index`: Reads a value from a table using an offset derived from a global variable and input parameters, then calls another routine with that value.
- `process_poker_game_logic`: Updates game state, evaluates hand patterns, and manages betting or win/loss strings based on certain card or hand conditions.
- `evaluate_hand_pattern`: Calculates a score or hand rank by analyzing the frequency of certain values within an array.
- `evaluate_set_pattern`: calculates the frequency of certain remainder values from a provided array and returns a priority-based score or pattern type based on those frequencies.
- `execute_nested_loops`: Executes a series of nested loops that perform library calls and pointer dereferencing or updating a global variable.
- `update_poker_hand_logic`: Updates a global variable or memory buffer by iterating through a series of calculated offsets and values, or checking hand-strength-related strings like Flush and Full House against a pointer-based table.
- `initialize_game_data_tables_3`: Initializes a series of memory-mapped tables or arrays by calculating offsets and storing incremental or fixed values.
- `initialize_bet_and_card_tables`: Sets up initial bet minimums and maximums based on a global state, clears a local array, and implements a loop that checks a global variable against specific range boundaries to potentially execute a sequence of conditional updates or string-based or pointer-based table modifications.
- `modify_global_tables`: iteratively calls routines to update or write values to different offsets within a global table or array structure.
- `calculate_weighted_score_sum`: iterates through a table of short integers to accumulate a weighted sum based on whether certain values match a target or fall within a specific range.
- `process_string_and_pointers`: Calculates pointer offsets and calls a routine with a pointer to a local stack variable and a string length or function pointer.
- `branch_and_iterate_with_pointer_access`: Checks if a parameter is zero to branch and call another routine with a different constant, then enters a loop that passes a local stack variable address to another routine while iterating through a range of values or a loop count determined by another routine's return value.
- `execute_delay_loop`: Performs a simple loop that increments a register until it reaches a fixed value.
- `update_card_display_table`: Iterates through a table to update string buffers or values based on card-related string references and calculated offsets.
- `build_string_structure_list`: Iterates through a null-terminated string to allocate and populate a linked list of structures, where each structure contains a pointer to a string-related object and an integer offset.
- `setup_audio_timer`: Initializes an audio timer structure and configures audio device parameters or nodes using library calls.
- `initialize_audio_timer_struct`: Initializes a memory structure with pointers and constant values, then configures timer and audio device parameters or node values.
- `initialize_audio_timer`: Stores three passed parameters into a global variable.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator, and returns the original destination pointer.
- `compute_string_length`: Receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminator, and returns the number of bytes before the terminator.
- `update_global_value`: Calculates a new value by adding a constant to a value retrieved from a global variable and stores the result back to a global variable.
- `store_word_to_global`: Stores a 32-bit value into a global memory location.
- `unpack_nested_stream_data`: Recursively calls a routine to extract bytes and writes them to a destination buffer, continuing until a counter or a return value of negative one is encountered.
- `update_structure_with_seek`: Performs a seek operation and updates a value in a structure based on conditional range validation or pointer manipulation.
- `calculate_array_difference`: Calculates a value based on the difference between two array elements and a file or stream seek or a third value.
- `subtract_array_difference`: If the first element of an array is non-zero, subtracts the difference between the first two elements from a stored longword and returns it.
- `insert_bytes_to_buffer`: Iteratively inserts bytes from a source pointer into a destination buffer, checking for valid pointer ranges and stopping if a failure or a certain count is reached.
- `increment_stream_pointer`: Increments a pointer stored in a memory location while extracting a byte from the pointed-to address, or recursively calls itself if a boundary or a certain value is met.
- `process_stream_and_index`: Reads data from a stream and updates a structure or a pointer-based index or flag based on certain range checks.
- `find_pointer_in_list`: Searches a list for a zeroed entry and updates a structure with a result if a match is found.
- `search_and_update_table`: Searches a table for a matching string and, if a match is found, updates a structure with a result and a flag.
- `compare_two_byte_strings`: Compares two byte strings byte by byte, returning a negative or positive value if a difference is found, or zero if they match or a limit is reached.
- `perform_no_operation`: This routine is empty and performs no operations before returning.
- `execute_indirect_call`: Performs an indirect call to a pointer stored in a register.
- `seek_file_position`: Performs a seek operation on a file or stream, updating a status variable or returning an error if the target position or pointer is invalid.
- `create_file_with_flags`: Calls a routine to create or truncate a file using a specific flag.
- `search_and_modify_table_entry`: Iteratively searches a table for a null entry while checking a loop limit, then conditionally performs a sequence of file-related operations and updates the table or a secondary value with new status or parameter data.
- `read_file_stream`: Reads a block of data from a file or stream, updating a status variable and checking for invalid file or buffer offsets.
- `clear_memory_and_execute_library_calls`: Clears a block of memory by zeroing a range of bytes, updates a global variable, and conditionally calls functions using a pointer from a global structure.
- `do_nothing_routine`: this routine is empty and performs no operations before returning.
- `parse_and_set_window_data`: Performs a complex sequence of library calls and pointer manipulations, including writing a specific string or constant to a memory location and updating a pointer-based structure.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_string_format_list`: Parses a source string to extract and store a sequence of formatted or quoted strings into a destination array.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `append_string_to_buffer`: finds the null terminator of a destination string and copies a source string to that location, stopping if a maximum buffer length is reached.
- `insert_string_at_end`: Finds the end of a destination string and writes a source string into that position for a limited count, ensuring a terminator is present.
- `copy_limited_string_with_pad`: Copies bytes from a source pointer to a destination pointer for a given count, stopping on a zero byte, and fills any remaining space with zero bytes.
- `update_window_pointer`: Updates a global variable with a result from a function call, then traverses a pointer-based structure to assign a new value to a field in a provided pointer.
- `set_and_restore_pointer`: Stores a pointer into a global variable and subsequently restores the original value to that same location.
- `store_and_execute_bytecode`: Stores a parameter into a global variable, calls a routine that parses a custom instruction stream, and then clears the stored global variable.
- `update_pointer_and_increment`: Increments a value at a pointer location and then stores a new byte at the address previously pointed to by that location.
- `copy_table_values_to_pointer`: Iteratively copies bytes from a table or memory location to a destination pointer using values returned by another routine, and optionally writes a constant value if certain sign-based or parameter-based conditions are met.
- `parse_bytecode_instructions`: Parses a custom instruction stream to update a destination pointer or a global variable using a lookup or arithmetic routine.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `handle_signed_division_result`: Performs a signed division and negates the result if the dividend or certain register input is negative.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1, returning a 64-bit result.
- `insert_byte_into_buffer`: Checks if a pointer-based range is valid, then stores a byte at the pointer's location and increments the pointer.
- `iterate_address_range`: Increments a value and calls another routine recursively until a certain limit is reached.
- `clear_and_traverse_node_data`: Checks a pointer and a flag in a structure, potentially calls a routine recursively, and then clears the contents of the structure or node.
- `update_index_range`: Updates a pointer or value in a structure based on range validation and conditional logic.
- `find_zero_in_list`: Iterates through a list of pointers, checking a byte at an offset, and returns a pointer to a zeroed entry if a certain byte is null.
- `validate_and_update_index`: Checks if an index is within a certain range and updates a structure with a new pointer or a certain value or flag based on the result of a table check.
- `traverse_and_clear_list`: Iteratively traverses a linked list and calls another routine on each node until a null pointer is encountered, then clears a global pointer.
- `traverse_linked_list`: Iteratively traverses a linked list and calls another routine on each node until a null pointer is encountered.
- `insert_into_list`: Inserts a new node into a linked list by updating a global pointer and copying a value from a source pointer.
- `forward_list_insertion`: Acts as a thin wrapper for adding a node to the head of a list by passing its argument directly to a linked list insertion helper.
- `search_and_replace_list_node`: Searches a linked list for a specific node and replaces a pointer in the list or a global variable with the pointer from the found node.
- `check_valid_index`: Checks if an index is within the bounds of a table and if the entry at that index is non-zero, returning a boolean-like value.
- `execute_file_operations`: Executes a file-related operation and updates a global variable with a result or returns an error code if the operation fails.
- `write_file_stream`: Writes a block of data to a file or stream, checking for valid stream alignment and updating a status variable if an error occurs or if the stream is invalid.
- `check_stream_buffer_status`: Checks if a certain bit is set in the return value of a function call and conditionally executes another routine or returns a status.
- `fetch_output_stream_and_write`: Gets the output stream through a DOS wrapper and passes it to a write helper, then makes a second helper call with the constant one.
- `execute_conditional_pointer`: Checks if a pointer in a structure is non-zero and, if so, calls the routine pointed to by that address.
- `execute_recursive_tasks`: Recursively calls another routine while performing a series of conditional pointer dereferences and indirect function calls.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `process_and_clear_entry`: Validates if an index is within a certain range and checks if a pointer-referenced value is non-zero, then potentially calls another routine before clearing that value.
- `multiply_32_bit_values`: Returns a 64-bit product formed from the high and low 16-bit halves of two 32-bit operands.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_indirect_function`: Loads a function pointer from an offset within a global pointer and executes it.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function`: calculates a function pointer by applying two offsets to a pointer retrieved from a global variable, then executes that function.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `initialize_struct_fields`: Checks a library function return value and, if successful, initializes several fields in a new memory structure with a pointer, a constant, and a byte value.
- `initialize_structure_with_library_call`: Calls a library function and, if successful, initializes certain fields in a new memory location with a pointer, a constant, and a value from another location.
- `initialize_struct_members`: Copies a pointer and some bytes to a new memory location and then calls another routine.
- `set_pointer_with_offset`: Stores a pointer to a memory location into another memory location with a fixed offset applied to that pointer.
- `release_and_reset_record`: If the pointer at offset 10 is set, calls an unnamed release routine on the record first.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `update_node_with_value`: If a certain pointer is non-zero, writes a constant byte and a word or dword to a destination offset within a structure.
- `update_target_data`: Calls another routine and, if the result is non-zero, writes a byte and a word or dword to a destination offset.
- `update_memory_values`: Stores a byte and a word or dword into a memory location using an offset from a pointer.
- `set_max_values`: Sets a byte and two dwords at a given pointer offset to maximum possible values before calling another routine.
- `initialize_memory_with_max_values`: Sets a byte and two dwords to certain maximum values and calls another routine.
- `call_indirect_library_function`: Loads a library function pointer from a global variable offset and calls it.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `process_pointer_chain_and_structs`: Copies a block of data from a global source, then uses a returned pointer to write new values to a structure and recursively calls other routines.
- `call_library_function_with_argument`: Calls another routine passing the input pointer as an argument via a library function offset.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_indirect_library_function_2`: Calls a library function using a pointer retrieved from a global variable offset.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_indirect_library_function_3`: Executes a library function located at an offset from a pointer stored in a global variable.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_indirect_library_function_4`: Performs an indirect call to a library function using a pointer from a global variable.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_indirect_library_function_5`: Retrieves a function pointer from a global variable and executes it.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_indirect_library_function_6`: Uses a pointer from a global variable to call a library function at a specific offset.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_indirect_library_function_7`: Invokes a library function via a pointer dereferenced from a global variable.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_indirect_library_function_8`: Calls a library function using a pointer offset from a global variable base.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Loads a function pointer from a memory offset and executes it.
- `call_library_vector_426`: Library stub: loads a library base from a global variable and calls the library function at offset minus 426 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `execute_indirect_function_3`: loads a function pointer from a table or a global pointer offset and calls it.
- `execute_indirect_function_4`: loads a function pointer from a nested table offset and executes it.
- `execute_indirect_function_pointer`: loads a function pointer from a table or pointer-chain and executes it.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `execute_indirect_function_5`: loads a function pointer from a table and executes it.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: loads a function pointer from a table located at a fixed offset from a global pointer and executes it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `execute_indirect_function_6`: Retrieves a function pointer from a nested table or structure and executes it.
- `execute_indirect_function_7`: Loads a function pointer from amemory offset and executes it.
- `execute_indirect_function_pointer_3`: Loads a function pointer from a secondary offset within a table and executes it.
- `execute_indirect_function_8`: Retrieves a function pointer from a nested pointer or table structure and executes it.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `execute_indirect_pointer`: Retrieves a pointer from a table using an offset from a global variable and executes it.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `execute_indirect_function_9`: Loads a function pointer from a nested offset within a global table and executes it.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `execute_indirect_pointer_large_offset`: Retrieves a pointer from a table using a large offset from a global variable and executes it.
- `execute_indirect_function_pointer_4`: loads a function pointer from a nested pointer or table and executes it.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_5`: Loads a function pointer from a nested pointer or table structure and calls it.

### Trump Castle.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
