# Game 1344 (tetris (1987)(mirrorsoft)[cr tristar][v blizzard v1.0]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01344). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904528 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 321 of 321 named (0 author symbols), 321 of 321 described (49 by rule, 268 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1069 of 1069 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7625 of 7647 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 382044 |
| code | 25822 |
| data | 46301 |
| padding | 2873 |
| unused | 447488 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Tetris (1987)(MirrorSoft)[cr Tristar][v Blizzard v1.0].adf:TETRIS | 15 | 5714 | 239 |
| Tetris (1987)(MirrorSoft)[cr Tristar][v Blizzard v1.0].adf:Tristar | 2 | 1122 | 44 |
| Tetris (1987)(MirrorSoft)[cr Tristar][v Blizzard v1.0].adf:bootblock | 1 | 39 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 768 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- 5 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- tetris (1987)(mirrorsoft)[cr tristar][v blizzard v1.0]/Tetris (1987)(MirrorSoft)[cr Tristar][v Blizzard v1.0].adf:TETRIS: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `pulse_port_bit_then_delay` (TETRIS): Clears a bit in a global variable, runs a fixed countdown that clears another bit on each pass, then waits for a bit in a CIA port register to clear. Kind: bit-banging disk timing/sync check. Pass path: the routine completes the loop and proceeds to u0__sub_7_00AA().
- `wait_while_port_bit_set` (TETRIS): While bit 4 of a global variable is set, it sets the low two bits to three and calls another routine, touching CIA port A and port B registers. Kind: disk hardware status check. Pass path: bclr.b #an address, an address.l.
- `poll_status_and_write_hardware` (TETRIS): Polls a bit in a global variable while performing a countdown loop and writing values to hardware registers and memory locations. Kind: disk hardware/track check. Pass path: the loop completes (iVar1 reaches 0) without the bitmask condition being met, allowing the code to continue to the next iteration or subsequent instructions.
- `clear_global_variables` (TETRIS): Sets multiple consecutive global memory locations to zero and then assigns specific constant values to several other global variables. Kind: disk hardware/sync check. Pass path: the branch at an address (after the bne.b an address instruction fails).
- `populate_data_structure` (TETRIS): Initializes a structure by performing a loop to calculate values, copies a sequence of words from a table into a buffer, and updates a function pointer within that structure. Kind: checksum/integrity trap. Pass path: the routine continues execution normally after the loop and the function returns.
- `check_global_flags` (TETRIS): Checks a series of global flags and calls another routine if they are non-zero. Kind: disk hardware/track check. Pass path: the routine continues to the next if-statement or returns if all flags are zero.
- `initialize_system_and_loop` (Tristar): Performs extensive hardware and library initialization, populates data tables, and enters a loop that repeatedly calls a routine. Kind: looping function pointer execution. Pass path: the loop continues executing the function pointers in the array until the counter reaches -1.

## Functions


### Tetris (1987)(MirrorSoft)[cr Tristar][v Blizzard v1.0].adf:TETRIS

- `initialize_system_environment`: Locates the current task and initializes global structures by querying system library offsets.
- `release_system_resources`: Calls a function through a pointer in a global structure, then performs several conditional calls to other routines to release or close various resources and memory.
- `call_two_table_entries`: Calls two function pointers read from a table at fixed offsets from a base register, in sequence, then returns.
- `call_and_store_result`: Loads an address from a table into a register, calls a routine, and stores the returned value into a global variable.
- `tokenize_input_string`: Parses a character string into a list of pointers by identifying whitespace-separated tokens or substrings enclosed in double quotes.
- `call_global_library_function`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `validate_and_get_table_slot`: Clears a word in a structure, then checks if an index is within a valid range and ensures the corresponding entry in a table is non-zero.
- `release_indexed_slot_status`: Looks up a pointer, returns a sentinel value if it is null, and returns zero if a flag bit is set.
- `init_and_invoke_callbacks`: Conditionally loads a value through an indirect function call from a global variable's table, copies a structure field, and calls two other routines.
- `close_flagged_nodes_in_list`: Walks a singly linked list of records starting from a fixed offset, and for each record whose flag bits match a certain condition it calls another routine when two stored words differ.
- `test_char_is_digit`: Returns one if the low word of its argument lies between the character codes for zero and nine inclusive, and zero otherwise.
- `dispatch_indexed_table_call`: Looks up an entry through an indexed table and returns minus one if it is absent.
- `insert_node_into_list`: Recursively traverses a linked list by calling another routine to find a target node, then updates the target node's pointer field with a new value and updates a global list head if the list was previously empty.
- `link_node_after_head`: Stores a computed value into a field of a node, invokes another routine to link that node into a list, and if the list head field is empty, sets it to the node.
- `free_linked_list_entries`: Walks a singly linked list from a structure field, calling a function through a global variable on each node's stored word, then clears the list head and tail pointers.
- `update_linked_list_node`: Copies a value from a source pointer into a destination structure, clears the destination's head word, links the destination after the source's tail, and points the source's tail at the destination.
- `process_queued_request_slot`: Clears a global word and scans a table of six-byte records to find the first unused slot, returning minus one if none is free.
- `lookup_slot_or_fail`: Calls another routine to locate a slot from a stored pointer, returns minus one if the lookup finds nothing, and otherwise calls a second routine with the slot's stored value and returns minus one if a status word in a structure is non-zero.
- `set_block_size_and_free`: Stores a fixed word value into a global variable, then calls another routine with the two parameters, passing the second parameter's low word twice.
- `compare_and_branch_on_value`: Loads a value from a pointer, compares it to zero, and branches based on the result.
- `insert_into_sorted_list`: Calculates a size value and iterates through a linked list to insert a new node into a specific position based on pointer comparisons.
- `traverse_linked_list`: Iterates through a linked list to perform comparisons and updates to structure fields and global values based on node data.
- `decrement_and_set_byte`: Compares a value from a memory location to a register, and if the condition is met, subtracts a value from a global variable and sets a register to a constant value.
- `decrement_global_value`: Compares two values and, if a specific condition is met, subtracts a value from a global variable.
- `update_linked_list_node_2`: Adds a value to a field in a structure, checks if a pointer is null, and if not, compares it to a value before performing a series of moves and additions to update the structure and a linked list.
- `transfer_pointer_and_branch`: Moves a pointer to a stack location, updates another stack location, and then updates a pointer via a memory access before branching to another routine.
- `initialize_structure_data`: Writes a long value to a location pointed to by another long value and clears a second long value.
- `append_byte_increment_counter`: Increments a word counter in a structure, stores the low byte of its argument through a pointer held in that structure, then advances that pointer by one byte.
- `store_result_and_format_text`: Clears a word counter in a global block, stores the first parameter as a pointer there, and calls a text formatting routine with the second parameter.
- `parse_signed_decimal_string`: Reads an optional leading sign and then consumes decimal digits from a character string, accumulating them into a signed word stored through the second argument.
- `format_signed_decimal_string`: Writes a minus sign into a buffer when the given value is negative, then appends the decimal representation of the value by calling another routine.
- `format_unsigned_decimal_string`: Converts a 16-bit unsigned value to a null-terminated decimal string of up to five characters, written into a buffer supplied by the caller, with leading zeros suppressed.
- `compute_packed_time_value`: Copies bytes from a source buffer into a local structure and runs two byte-counting loops over those values.
- `parse_packed_time_field`: Copies three bytes from a source buffer into a structure, parses a signed decimal value, multiplies it by a constant, and stores the result along with subsequent characters.
- `lookup_and_call_indexed_entry`: Looks up a record through an indexed table and returns minus one if it is absent.
- `parse_recursive_expression`: Recursively parses a string to evaluate mathematical expressions by identifying numeric tokens, decimal points, and operators via a lookup table.
- `parse_expression_token`: Identifies and processes individual characters or tokens from a string to facilitate recursive expression evaluation.
- `parse_expression_string_recursive`: A recursive descent parser that processes a string of characters to evaluate mathematical or symbolic expressions.
- `parse_expression_string`: Parses a string containing numeric values, decimal points, and special characters to evaluate an expression.
- `emit_formatted_text_characters`: Scans a byte string, copying ordinary characters and doubled percent signs through an output routine and delegating each conversion specification to a parser.
- `divide_signed_32bit_by_16bit`: Performs signed division of a 32-bit dividend by a 16-bit divisor using normalization and bit shifting.
- `normalize_and_divide_16bit`: Normalizes a 16-bit divisor by shifting it left until its top bit is set, tracking the shift count, then divides a 32-bit dividend by it and corrects the quotient and remainder.
- `invoke_callback_sequence`: Loads a pointer from a global table, calls a routine via that pointer, pushes a constant to the stack, and calls another routine.
- `multiply_32bit_words`: Multiplies two 32-bit values taken from the data registers using 16-bit partial products, adding the two cross terms to the low-word product.
- `allocate_and_fill_record`: Allocates a memory block through a library call, stores a byte, a word, and several flag bytes into it, and writes a result pointer.
- `normalize_day_counter`: Reads a 16-bit value through a pointer and repeatedly subtracts a year-length amount, using a leap-year pattern based on a counter, until the value is below 366.
- `subtract_table_values_loop`: Reads a 16-bit value through a pointer and subtracts entries from a byte table in a loop of up to twelve steps while the value is at least each entry.
- `divide_word_by_seven`: Divides a 16-bit value taken from the parameter by seven and returns the quotient in the upper half and the remainder in the lower half of a 32-bit result.
- `allocate_port_block`: Allocates a 34-byte block using a slot index and initializes several fields including a pointer and a byte value.
- `initialize_allocated_block`: Attempts to allocate a memory block via another routine and, upon success, populates a byte, a word, and a longword at specific offsets within that block using the provided arguments.
- `release_port_chain`: Recursively releases a linked list of records by setting status bytes and calling itself on the next link in the chain.
- `release_ext_io_block`: Sets status bytes and clears longword fields in a block before calling another routine to release the associated memory.
- `release_resources_and_reply`: Calls an optional cleanup function pointer stored in a structure, then calls another routine, and conditionally invokes library close and free calls.
- `check_input_button_state`: Reads a value through a function pointer in a global table, masks it, and if the result is non-zero calls a second function pointer and signals an exec library routine when that call returns non-zero.
- `read_value_with_fallback`: Clears a word in a global variable, calls a function through a pointer table to obtain a value, and if that value is minus one it reads a status code through another function pointer, stores it in a global variable and sets a second global word to five.
- `poll_value_with_error_code`: Clears a word in a global variable, calls a table-indexed function to obtain a value, and when that value is minus one stores a status code in a global variable and sets another global word to five.
- `combine_value_by_mode`: Clears a global word, calls a function to get a value, and on a minus-one result records an error code and a second global word.
- `check_status_return_minus_one`: Clears a global word, calls a table-indexed function, and if it returns zero stores an error code in a global variable, sets a second global word to two, and returns minus one.
- `invoke_table_function_once`: Calls a function through a pointer table and returns zero.
- `check_and_retry_status`: Clears a global word, calls a table-indexed function and, if non-zero, invokes two further table functions.
- `check_nested_status_return`: Clears a global word and calls a table-indexed function.
- `parse_string_tokens`: Parses a string into tokens by skipping whitespace and handling quoted substrings.
- `reset_game_state_and_loop`: Copies sixteen words from one memory location to another and enters an infinite loop that manages state flags, clears a large memory buffer, and resets several data tables.
- `release_resources_and_reply_2`: Calls an optional cleanup function pointer from a structure, then calls another routine and conditionally invokes library functions.
- `execute_callback_chain`: Executes a sequence of several subroutine calls in succession and returns a global variable.
- `initialize_interrupt_and_dma`: Copies four words from hardware registers into global variables, then writes a fixed value to a global variable and to several hardware control registers to set up interrupt and DMA enables.
- `pulse_port_bit_then_delay`: Clears a bit in a global variable, runs a fixed countdown that clears another bit on each pass, then waits for a bit in a CIA port register to clear.
- `delay_then_wait_port_bit`: Runs a fixed countdown of three thousand iterations, then spins until bit five of a global variable that holds the CIA port A status is clear.
- `wait_while_port_bit_set`: While bit 4 of a global variable is set, it sets the low two bits to three and calls another routine, touching CIA port A and port B registers.
- `poll_status_and_write_hardware`: Polls a bit in a global variable while performing a countdown loop and writing values to hardware registers and memory locations.
- `clear_memory_offsets`: Stores zero into two different word-sized offsets relative to a pointer.
- `wait_for_status_bit_set`: Spins until bit four of a global status byte is set.
- `copy_words_with_set_high_bit`: Stores four global words into fixed offsets of a base pointer, each with the high bit forced to one.
- `toggle_global_flag`: Checks if a global variable equals a specific value and sets it to either zero or one based on that comparison.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `install_serial_interrupt_server`: Fills in a global interrupt structure with a priority, a type byte, name and data pointers, and a handler address, then registers it through a library call.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `install_interrupt_server_handler`: Writes a set of constant bytes, words and a long into a global data block, including a code pointer to a local routine, then invokes a library function through a pointer stored in a global variable at a negative offset.
- `call_rem_int_server_2`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `initialize_resource_tables_and_handlers`: Opens system libraries, requests memory slots, and populates multiple tables with returned pointers while retrying on failure.
- `swap_buffer_and_call_handler`: Stores a pointer selected from a table indexed by a toggling flag into a global variable, flips that flag, invokes a function through a library base pointer, and then calls another routine.
- `wait_for_raster_line`: Polls a hardware register through a pointer until the vertical position field matches the high word of the parameter.
- `wait_for_blitter_idle`: Polls a hardware status register through a pointer until a particular bit clears, indicating a busy flag has been released.
- `cleanup_resources_and_libraries`: Updates memory locations, conditionally calls a library function based on a loop through four sets of pointers, and then closes several libraries.
- `close_libraries_and_cleanup`: Conditionally calls one library function if a global variable is nonzero, then calls two further library functions and two more via a global library base, with no return value.
- `invoke_global_handler_with_params`: Loads a function pointer from a global variable at a fixed negative offset and calls it with eleven arguments, several of them constants and two derived from global base offsets.
- `select_pointer_then_dispatch`: Chooses one of two global-relative addresses based on whether a global word is zero, then calls a routine with a fixed set of arguments including that address.
- `select_and_draw_frame_buffer`: Chooses one of two data tables based on a global flag, passes it with a fixed set of constants to another routine, and then calls a second routine.
- `select_and_draw_frame_buffer_2`: Chooses one of two alternate display or buffer tables based on a flag stored in a global variable, then passes that choice together with fixed dimensions and parameters to another routine that performs a drawing or setup operation, and finally calls a second routine.
- `fill_stack_buffer_and_copy`: Fills a sixteen-word stack buffer with the 16-bit value of a parameter, then passes that buffer to another routine that copies it.
- `request_and_release_resource_slots`: Calls a library function through a stored offset, repeatedly requests a free slot until one is granted, then calls another routine five times to release four stored pointers and one further value before calling two more library functions.
- `acquire_and_release_resource_slots`: Repeats a loop to acquire a resource slot until a valid one is returned, then calls a routine four times with values extracted from a table, followed by two additional calls with different parameters, before releasing the resource.
- `acquire_slot_and_release_pointers`: Calls a library function, then retries a slot request until it succeeds, then calls a routine four times with stored pointers and once with a further value.
- `acquire_slot_and_release_offsets`: Calls a library function, retries a slot request until it succeeds, and then calls a routine four times with pointers taken from another set of structure offsets plus one further value.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `wait_then_copy_buffer`: Calls a routine with a value derived from a global base offset, then calls another routine that copies sixteen words.
- `build_nibble_plane_pixels`: Expands a count of 16 nibble-packed source words into a destination table by iterating over 16 bit-plane levels.
- `expand_nibbles_into_buffer`: Copies sixteen 16-bit words from one buffer into a second, then for each nibble-packed word runs a loop over values from fifteen down to zero, writing the highest-value nibbles into the second buffer.
- `wait_for_raster_line_2`: Repeatedly polls a video hardware position register through a pointer loaded from a global structure, busy-waiting until the beam reaches a fixed scanline, for a count given by the parameter.
- `copy_sixteen_words_twice`: Copies sixteen 16-bit words from a source buffer into two destinations, each reached through a pointer read from a global variable and advanced by four bytes per word.
- `store_two_words_globally`: Stores the two 16-bit halves of its 32-bit argument into two consecutive words of a global variable block.
- `process_and_copy_data_block`: Checks a byte at a pointer, and if non-zero, masks it, increments a value in a stack frame, pushes the masked byte to the stack, and calls another routine.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `skip_null_terminated_string`: Iterates through a null-terminated string by incrementing a pointer until a zero byte is encountered.
- `clear_and_copy_data_block`: Calculates a destination address based on global values, then clears several bytes and copies data from a source table into a destination buffer in a loop.
- `add_eight_to_word`: Adds eight to a 16-bit value held in a global variable block.
- `copy_eight_rows_to_planes`: Iterates eight times, writing zero to three destination byte locations and copying one source byte into a fourth, with each set of pointers advanced by fixed row strides; the destination set is chosen by a global flag.
- `return_global_value`: Returns a 32-bit value stored at a fixed offset within a global structure.
- `compute_state_code`: Checks a series of byte flags in a global variable block and hardware-related input words, overwriting a result byte in priority order, and returns that byte as a small code from 1 to 7.
- `copy_stack_word_to_global`: Calls another routine to fill a two-byte local buffer on the stack, then copies the resulting halfword into a global variable.
- `test_and_branch_to_routine`: Tests a word at a specific offset from a global pointer and branches to a different routine if the value is non-zero.
- `call_library_function_minus`: Loads an address relative to a global base and calls a function at a fixed negative offset from a library base pointer stored in a global variable.
- `call_library_function_minus_2`: Calls a function at a fixed negative offset from a library base pointer stored in a global variable.
- `call_library_with_string_length`: Scans a null-terminated string to find its length, then calls three functions through a library base pointer at fixed negative offsets, passing the string along the way.
- `invoke_indirect_library_function`: Loads a pointer from a global data area and calls a function through a fixed negative offset in a jump table, then returns.
- `call_library_then_measure_string`: Calls two library functions through the jump table, then advances a pointer past a null-terminated string and calls a third library function.
- `release_block_entries_when_slot_granted`: Calls a library function and makes one slot request; if granted, it calls a routine ten times with values from a pointer table and twice more with fixed offsets, then calls further routines to finish.
- `wait_then_release_resource_slots_alt`: Calls a routine to check for a slot, and if granted, iterates through a table to call another routine ten times, then makes several more calls including two indirect calls.
- `swap_byte_and_index_table`: Reads a byte from a global variable, clears that byte and sets the following byte to a constant, then returns a byte fetched from a table indexed by the saved value.
- `update_keyboard_state`: Performs bitwise operations on a global variable and uses the result to write a zero or a non-zero byte to a memory location pointed to by a value derived from that global variable.
- `check_indexed_slot_status`: Looks up a pointer, returns a sentinel value if it is null, and returns zero if a specific flag bit is set.
- `lookup_slot_or_fail_2`: Calls another routine to locate a slot from a stored pointer, returns minus one if the lookup fails, and otherwise calls a second routine.
- `reset_game_state_2`: Calls another routine that clears and initializes several global memory locations.
- `compute_packed_time_value_2`: Copies bytes from a source buffer into a local structure, performs two byte-counting loops, and stores a calculated sum into a pointer.
- `process_linked_list`: Walks a singly linked list and calls another routine for each record that meets specific bitwise flag and value comparison criteria.
- `copy_data_sequence`: Performs a word-based copy from one memory location to another based on specific conditional checks.
- `execute_callback_chain_2`: Executes a sequence of several subroutine calls in succession and returns a global variable.
- `process_queued_request_slot_2`: Clears a global word and scans a table of six-byte records to find the first unused slot, returning minus one if none are free.
- `lookup_indexed_entry`: Looks up a record through an indexed table and performs conditional checks on the result before returning a value.
- `call_global_function_pointer`: Calls a function through a pointer retrieved from a global variable.
- `sync_and_reset_game_state`: Copies sixteen words from one memory location to another and enters a loop that manages state flags and clears a large memory buffer.
- `initialize_round_and_spawn_piece`: Clears the flag grid and specific cell flags to a default state, resets two global counters and a timer, and selects a random piece index.
- `draw_flagged_cells_grid_2`: Iterates through a grid of byte flags using nested loops, checking specific column offsets within each row.
- `update_grid_flags_and_redraw`: Writes values into a flag grid at a calculated position, clears a secondary sub-region of flags, and updates a second array using indexed values when a specific condition is met.
- `clear_four_table_bytes`: Loops four times, computing an offset from two 16-bit values in a global variable area and a byte from a table, and stores a zero byte at each resulting address in a global variable area.
- `fill_stack_buffer_with_word`: Populates a sixteen-word buffer on the stack by repeating a sixteen-bit input value sixteen times.
- `release_resource_slots`: Calls a routine to check for a slot, and if successful, iterates through a table to call another routine ten times before making several additional calls.
- `invoke_global_handler`: Loads a function pointer from a global variable at a fixed negative offset and calls it with eleven arguments.
- `request_resource_slots`: Repeatedly calls a routine until a slot is granted, then calls another routine four times with values from a table and twice more with specific arguments.
- `fetch_global_offset_value`: Retrieves a 32-bit value from a specific offset within a global data structure using a base address passed in a register.
- `dispatch_conditional_pointer`: Chooses one of two global-relative addresses based on whether a global word is zero, then calls a routine with a fixed set of arguments.
- `invoke_sequence_of_routines_2`: Calls another routine with a constant, then calls another routine with a value derived from a global base offset, and finally calls two other routines.
- `poll_state_until_zero`: Loops a counter stored at a parameter-supplied location, decrementing it on each pass while a global state byte is zero and a computed state is not four.
- `store_stack_word`: Calls a routine to populate a stack buffer and then stores that two-byte value into a global variable.
- `render_game_board_bitplanes`: Copies pixel data from a source table into four bitplane buffers using bitwise OR operations.
- `swap_buffer_and_call_handler_2`: Stores a pointer selected from a table indexed by a toggling flag into a global variable, flips that flag, and invokes a function through a pointer.
- `wait_for_raster_line_3`: Polls a hardware register through a pointer until the vertical scanline position matches the high word of the provided parameter.
- `expand_nibbles_into_buffer_2`: Copies a sequence of 16-bit words into a destination buffer and then expands each nibble within those words into a full 4-bit value by iterating through a descending loop of possible nibble values.
- `determine_state_code`: Checks a series of byte flags in a global variable block and hardware-related input words, overwriting a result byte in priority order.
- `read_text_input_into_field_2`: Clears a global counter, draws labels and a boxed field, then reads characters into a selected text buffer of up to ten characters, handling backspaces and carriage returns.
- `run_menu_selection_loop_2`: Draws several text items, then loops up to 75 times polling input, adjusting a bounded selection counter on two input codes and redrawing the selection.
- `shift_table_rows`: Searches a table for a specific word and, if found, shifts rows of data within that table by copying values from a preceding row.
- `expand_nibble_packed_bitplanes`: Expands a list of nibble-packed values into a multi-plane pixel table by iterating through bitplane levels.
- `wait_and_copy_buffer`: Calls a routine with a value derived from a global base offset and then calls another routine that copies sixteen words.
- `call_table_builder`: Calls another routine and returns zero.
- `allocate_resource_pointers`: Requests multiple memory allocations or resource handles through a system library and stores the resulting pointers into various tables.
- `update_bit_state`: Updates two global variables by performing bitwise operations and shifts on a byte value and incrementing a secondary byte.
- `copy_sixteen_words_twice_2`: Copies sixteen 16-bit words from a source buffer into two separate destinations identified via pointers stored in global memory.
- `wait_for_raster_line_edge`: Polls a value through a pointer derived from a global structure until a specific bitmask in the middle byte matches a constant, then waits for that condition to change.
- `apply_masked_block`: Ors bytes from a source buffer into four different destination tables using a mask to preserve existing bits.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `acquire_resource_slots`: Loops until a routine returns a valid slot, then calls another routine four times using values from a table and twice more with specific arguments.
- `select_and_draw_buffer`: Chooses one of two data tables based on a global flag, passes it with a fixed set of constants to another routine, and then calls a second routine.
- `call_library_function`: Calls a library function using an offset from a library base stored in a global variable.
- `process_keyboard_input`: Reads a byte from the keyboard serial register, performs bitwise operations, and stores a modified byte into a memory location pointed to by a register.
- `clear_byte_and_global`: Stores zero into a memory location addressed by a base register plus a 16-bit offset and clears a global variable, returning zero.
- `shift_conditional_data`: Checks multiple conditions and a global variable before shifting a sequence of words from a source to a destination.
- `load_address_and_branch`: Loads a long value into a register, sets a word value in another register, and performs a short branch.
- `shift_data_block`: Copies a sequence of words from a source table to a destination table using a loop.
- `call_table_builder_and_return`: Calls another routine that builds a table of entries, then returns zero as its result.
- `forward_to_routine`: Calls another routine and returns.
- `call_reset_game_state`: This routine calls another routine that clears and initializes several global memory locations.
- `return_global_word`: Returns a 16-bit value from a global variable.
- `advance_byte_state_pair`: Updates two global byte variables: one is combined from its previous value and a rotated feedback of itself, the other is incremented by one or two depending on a bit of the first.
- `or_masked_block_into_table`: Ors bytes from a source region into four interleaved rows of a table, using complemented mask bytes to preserve existing bits, over a fixed number of iterations with conditional handling of the second column.
- `initialize_table_of_entries_2`: Builds a table of entries by reading size-and-divisor pairs from a data table, storing computed values into a global array, and initializing several fields.
- `populate_data_structure_2`: Initializes a structure by calculating values in a loop, copying words from a table into a buffer, and setting a function pointer.
- `reset_game_state`: Clears a series of consecutive global memory locations to zero and subsequently assigns specific constant values to several other global variables.
- `update_dma_control_parameters_2`: Iterates through a data table to calculate and update DMA control structures, including pointer offsets and hardware register values.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `draw_piece_into_bitplanes`: Reads a shape table and a sequence of per-row values, then ORs the shape's bytes into four bitplane buffers at computed offsets, shifting by eight bits per step and looping over a fixed number of rows.
- `search_and_shift_table_rows`: Searches a table for a specific word, and if found, shifts rows of data within that table by copying values from a preceding row.
- `run_menu_selection_loop`: Draws several text items, then loops up to 75 times polling input, adjusting a bounded selection counter on two input codes and redrawing the affected text, and exits early when a global flag becomes zero.
- `draw_playfield_cell_grid`: Loops over ten rows and, for each row, calls another routine to draw a fixed-size block using a value read from a table indexed by the row, then issues a series of draw calls with fixed offsets and constant sizes.
- `poll_state_until_zero_count`: Loops a counter stored at a parameter-supplied location, decrementing it on each pass while a global state byte is zero and a computed state code is not four.
- `read_text_input_into_field`: Clears a global counter, draws labels and a boxed field, then reads characters into a selected text buffer of up to ten characters, handling a backspace code by blanking the last character and a carriage return to finish.
- `invoke_sequence_of_routines`: Calls another routine with a constant, then calls another routine with a value derived from a global base offset, and finally calls two further routines with no arguments.
- `process_string_with_library_calls`: Calls three different functions from a library and iterates through a null-terminated string to determine its length.
- `execute_library_function`: Retrieves a pointer from a global table and calls a function located at a fixed negative offset from that pointer.
- `store_two_words_globally_2`: Stores the two 16-bit halves of a 32-bit argument into two consecutive words of a global variable block.
- `format_signed_decimal_string_2`: Writes a minus sign into a buffer when the given value is negative, then appends the decimal representation of the value by calling another routine.
- `fetch_table_value_and_reset_byte`: Reads a byte from a global variable, clears that byte, sets the following byte to a constant, and returns a value from a table using the original byte as an index.
- `traverse_null_terminated_string`: Increments a pointer through a sequence of bytes until a null byte is reached.
- `process_string_with_library_calls_2`: Calls two library functions, iterates through a null-terminated string, and then calls a third library function.
- `toggle_buffer_pointer`: Updates a global variable with a pointer selected from a table using an index that is toggled between zero and one.
- `expand_nibble_data`: Copies a series of words into a destination buffer and then expands each nibble within those words into a full byte value through a series of descending loops.
- `evaluate_state_code`: Checks several byte flags in a global block and bit patterns in hardware-related words to return a specific byte value.
- `select_and_draw_buffer_alt`: Chooses one of two alternate display or buffer tables based on a flag stored in a global variable, then passes that choice together with fixed constants to another routine.
- `expand_nibble_packed_bitplanes_2`: Expands a source table of nibble-packed values into a destination table of bitplane-separated values.
- `call_table_builder_and_return_2`: Calls another routine that builds a table of entries and returns zero.
- `reset_and_format_string`: Clears a word counter in a global block, stores a pointer to a string in that block, and calls a text formatting routine.
- `copy_words_to_destinations`: Copies sixteen 16-bit words from a source buffer to two different destination memory locations.
- `wait_for_raster_position`: Polls a video hardware register until a specific vertical beam position is reached.
- `call_indirect_library_function`: Loads a pointer from a global variable and executes a function located at a fixed offset within that pointer's target.
- `release_resource_slots_2`: Checks for a slot, and if granted, iterates through a table to call another routine ten times before calling several other routines.
- `initialize_table_of_entries`: Builds a table of entries by reading size-and-divisor pairs from a data table, storing computed values into a global array, and initializing a loop of four records with fixed field values.
- `clear_global_variables`: Sets multiple consecutive global memory locations to zero and then assigns specific constant values to several other global variables.
- `execute_conditional_routines`: Checks a series of global flags and calls another routine for each non-zero flag encountered.
- `update_dma_control_parameters`: Iterates through a data table to calculate and update DMA control structures, including pointer offsets and hardware register values.
- `parse_dma_control_stream`: Iterates through a data stream to populate a control structure with pointers, offsets, and calculated values from a lookup table.
- `parse_and_configure_dma`: Parses a data stream to populate a structure with offsets, pointers, and calculated values from a table.
- `update_falling_piece_position_2`: Calculates a new vertical position for a falling object by modifying a coordinate value based on a direction flag and updates hardware registers.
- `update_falling_piece_position`: Calculates a new vertical position for a falling object by modifying a coordinate value based on a direction flag and updates hardware registers and data structures with the result.
- `update_piece_position_and_state`: Calculates a new position for a falling object by adding or subtracting a value based on a flag, then updates hardware registers and a data table with the resulting coordinates and state.
- `update_and_branch_data`: Subtracts one value from another, stores the result in a location relative to a register, and conditionally branches or moves the result to another location.
- `calculate_indexed_pointer_2`: Compares a byte value, adjusts it based on a threshold, calculates an offset into a table using multiplication, and stores the resulting value into a memory location.
- `calculate_indexed_pointer`: Compares a byte value, adjusts it based on a threshold, calculates an offset into a table using multiplication, and stores the resulting pointer into a global variable.
- `execute_indexed_function`: Calculates an index based on a byte value, retrieves a pointer from a table, adds it to a base address, and stores the result into two offsets of a structure.
- `call_indexed_table_function`: Calculates an index from a value, fetches a function pointer from a table, and calls it.
- `populate_data_structure`: Initializes a structure by performing a loop to calculate values, copies a sequence of words from a table into a buffer, and updates a function pointer within that structure.
- `copy_table_to_buffer_and_delay`: Clears a flag byte, stores a value into a global variable and a small structure, copies ten words from a table at an offset into a buffer, then sets further flag bytes and a computed pointer before running a countdown loop.
- `update_bit_streams`: Manipulates bits from a global variable and a memory location to shift them into two different bitstreams and returns a calculated value.
- `check_global_flags`: Checks a series of global flags and calls another routine if they are non-zero.
- `update_dma_display_parameters`: Decrements several counters within a data structure and performs bitwise shifts on specific bytes to update timing or scaling values.
- `update_display_parameters`: Updates a data structure by decrementing counters, performing bitwise shifts on specific bytes, and modifying a pointer and a value in a hardware-related register block.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `create_port`: Obtains a slot index from an unnamed routine and, when valid, allocates a 34-byte block with flags 65537.
- `delete_port`: Recursively releases a linked chain of records, processing the linked record first when its link field is set.
- `create_ext_io`: Calls a library-backed allocation routine with a second argument, and if it returns a non-null block, stores a byte value, a word from the second argument, and the first argument pointer into fixed offsets of that block.
- `delete_ext_io`: If the provided pointer is non-null, it sets a byte field to 255, clears two longword fields, and calls another routine with the pointer and a word field from the structure.
- `call_indirect_via_global`: Calls a function pointer retrieved from a global variable.

### Tetris (1987)(MirrorSoft)[cr Tristar][v Blizzard v1.0].adf:Tristar

- `branch_to_offset`: Performs a short branch to a specific offset within another routine.
- `execute_infinite_reset`: Enters an infinite loop that repeatedly calls a reset routine.
- `initialize_bitmap_and_loop`: Allocates a raster and initializes a bitmap, then iterates through a table to store values from a routine, entering an infinite reset loop if a return value is zero.
- `initialize_system_and_loop`: Performs extensive hardware and library initialization, populates data tables, and enters a loop that repeatedly calls a routine.
- `update_input_and_clear_buffers`: Manages input state by checking hardware registers and updates a series of memory buffers through a loop.
- `update_global_state_and_enable_interrupts`: Sets a global variable to zero, updates two other global variables with constant values, and enables interrupts by writing to the INTENA register.
- `write_interleaved_data`: Iterates through a list of longwords, splitting each into two shorts and storing them into a destination table with incrementing values.
- `initialize_dma_and_tables`: Sets up DMA parameters and populates various data tables including a prefix sum table and interleaved data.
- `configure_dma_channel`: Sets a pointer and a value in a specific memory location and enables a DMA channel by writing to a hardware register.
- `repeat_routine_loop`: Executes another routine fifty times in a loop.
- `initialize_dma_parameters`: Sets several memory locations to zero or to the address of a global variable.
- `fill_prefix_sum_table`: Populates a destination table with a running sum of values taken from a source table.
- `configure_dma_and_interrupts`: Sets values in hardware registers and calls another routine, potentially twice, to manage DMA and interrupt enable bits.
- `update_bitmask_from_table`: Iterates through a table to set or clear specific bits in a memory buffer based on a bitmask and a status byte.
- `calculate_bitmask_from_table`: Constructs a 32-bit integer by checking specific bits in a memory buffer through a series of table lookups.
- `modify_single_bit`: Sets or clears a single bit in a memory buffer at an offset determined by a calculated index.
- `process_string_and_blitter`: Iterates through a string to find a specific character, updates a global counter, and then executes another routine three times.
- `delay_if_char_in_range`: Checks if a byte in a register is within a specific range, and if so, runs a fixed countdown loop calling another routine.
- `configure_blt_hardware`: Sets various Blitter registers including mode, address, and size, then enables the DMA controller.
- `configure_blt_hardware_alt`: Sets Blitter registers with different parameters and enables the DMA controller.
- `update_global_table_values`: Calls another routine three times, storing the results modified by constant offsets into a global table.
- `update_obfuscated_value`: Performs arithmetic on a global variable and returns a value derived from a division and remainder operation.
- `execute_triple_call_sequence`: Stores a global variable address into a destination and then calls another routine three times in succession.
- `initialize_recursive_pointer_chain`: Sets a global variable to a specific address and then performs a series of recursive calls to itself.
- `store_pointer_and_call`: Stores the address of a global variable into a location pointed to by a register and then calls another routine three times.
- `process_complex_data_block`: Performs multiple passes of data manipulation including bit shifting, byte packing, and memory copying between various pointers and tables.
- `copy_memory_block`: Copies a block of data from one memory location to another using a specified count.
- `process_conditional_data_loop`: Iterates through a range of data to copy values from a global table and conditionally executes a routine based on a bit in a structure.
- `copy_conditional_data`: Copies data from a source to a destination, potentially performing sign-based logic or conditional iterations.
- `initialize_graphics_library`: Opens a library via a pointer in a table and executes a sequence of subroutines using offsets from the returned library base.
- `manage_graphics_library`: Opens a library, performs several operations using library offsets, and enters an infinite loop if a specific condition is not met.
- `call_library_functions`: Calls three different routines that interact with a library base and its offsets.
- `call_offset_routine`: Loads a library base, modifies a register, and calls a routine at a negative offset from that base before storing a result into a global variable.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `parse_data_structure`: Traverses a linked list of structures to find a specific signature, then extracts and processes fields into a local table.
- `configure_dma_and_interrupts_2`: Writes specific hardware register addresses and values to a global variable and calls another routine four times with varying parameters, then modifies global values and hardware registers.
- `reset_audio_hardware`: Checks a global variable, updates it with specific values, and calls another routine to manage hardware state.
- `initialize_sequencer_channels`: Writes specific hardware register addresses and values to a global variable and calls another routine four times with varying parameters.
- `boot_reset_sequencer_channel`: Resets one sequencer channel record of the boot copy of the music player.
- `prepare_audio_reset`: Loads several memory addresses into a register and calls another routine sequentially for each address.
- `boot_silence_one_channel`: Silences one audio channel and writes its volume from the table.
- `set_interrupt_enable_bits`: Writes specific values to a hardware register and calls another routine multiple times.
- `update_dma_control_and_linked_list`: Manages a linked list of data structures by iterating through nodes and updating a control register.

### Tetris (1987)(MirrorSoft)[cr Tristar][v Blizzard v1.0].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state_and_loop`: Resets the game board and enters a main loop that synchronizes data between two memory buffers, clears a large memory block, and manages state flags through a nested polling loop.
- `reset_board_and_start_round`: Clears the flag grid and a set of cell flags to 255, resets two global counters and a timer word, then reads a random value to set a state index, and finally draws a shape into the bitplanes and updates a score table via further routines.
- `draw_flagged_cells_grid`: Walks a 20-row grid of byte flags and, for each non-zero flag in columns 13 through 22, calls another routine to draw a block at that position.
- `update_cell_flags_and_redraw`: Writes a value into a flag grid at a position computed from global variables, clears a sub-region of flags, and when a global word is set, writes indexed values into a second array.
- `clear_board_cell_column`: Loops over four consecutive entries, using a 16-bit value scaled by 5 and added to a table index to read a byte from the global area, then combines it with two other 16-bit global values, scaled by 40, to form a target address, and writes a zero byte at each of those four addresses.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
