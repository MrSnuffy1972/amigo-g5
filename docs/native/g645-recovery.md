# Game 645 (tetris (1987)(mirrorsoft)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00645). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 236 of 236 named (0 author symbols), 236 of 236 described (14 by rule, 222 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1975 of 1975 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5765 of 5765 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 284913 |
| code | 18576 |
| data | 33078 |
| padding | 2441 |
| unused | 562176 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Tetris (1987)(Mirrorsoft).adf:Tetris | 15 | 5751 | 234 |
| Tetris (1987)(Mirrorsoft).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- tetris (1987)(mirrorsoft)/Tetris (1987)(Mirrorsoft).adf:Tetris: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Tetris (1987)(Mirrorsoft).adf:Tetris

- `init_and_parse_command_line`: Checks whether the program was started from the command line or from the Workbench, and in either case sets up the globals the program uses.
- `release_resources_and_reply`: Calls an optional cleanup function pointer in a structure, then another routine, and conditionally invokes library closing and freeing calls on several non-null structure fields.
- `call_two_table_entries`: Calls two function pointers read from a table at fixed offsets from a base register, in sequence, then returns.
- `release_resources_and_return`: Allocates a resource through a library function and returns early with its result if the allocation fails.
- `parse_string_into_tokens`: Splits a null-terminated input string in place into up to thirty-two tokens by writing terminators at whitespace and quote boundaries, storing a pointer to each token in a table and incrementing a token count held in a global variable.
- `call_indirect_table_entry`: Calls one function through a pointer read from a global variable at a fixed offset and returns, with no other operations.
- `lookup_indexed_table_slot`: Clears a word in a structure, then validates an index against a stored count and checks that the indexed slot is non-zero.
- `release_indexed_slot_status`: Looks up a pointer, returns a sentinel value if it is null, and returns zero if a flag bit is set.
- `init_and_invoke_callbacks`: Conditionally loads a value through an indirect function call from a global variable's table and stores the result in a structure field, copies another structure field into a second field, and then calls a routine with fixed arguments before calling another routine.
- `close_flagged_nodes_in_list`: Walks a singly linked list of records starting from a fixed offset, and for each record whose flag bits match a certain condition it calls another routine when two stored words differ.
- `test_char_is_digit`: Returns one if the low word of its argument lies between the character codes for zero and nine inclusive, and zero otherwise.
- `dispatch_indexed_table_call`: Looks up an entry through an indexed table and returns minus one if it is absent.
- `allocate_and_link_node`: Allocates a memory block of a size derived from the input value through a library call, stores the adjusted size in the block, and links it into a list via another routine.
- `link_node_after_head`: Stores a computed value into a field of a node, invokes another routine to link that node into a list, and if the list head field is empty, sets it to the node.
- `free_linked_list_entries`: Walks a singly linked list from a structure field, calling a function through a global variable on each node's stored word, then clears the list head and tail pointers.
- `update_linked_list_node`: Copies a value from a source pointer into a destination structure, clears the destination's head word, links the destination after the source's tail, and points the source's tail at the destination.
- `process_queued_request_slot`: Clears a global word and scans a table of six-byte records to find the first unused slot, returning minus one if none is free.
- `lookup_slot_or_fail`: Calls another routine to locate a slot from a stored pointer, returns minus one if the lookup finds nothing, and otherwise calls a second routine with the slot's stored value and returns minus one if a status word in a structure is non-zero.
- `set_block_size_and_free`: Stores a fixed word value into a global variable, then calls another routine with the two parameters, passing the second parameter's low word twice.
- `free_memory_block_to_list`: Rounds a requested size up to at least eight bytes and to a multiple of four, then inserts the block into an address-ordered free list, merging it with adjacent free blocks.
- `insert_block_into_free_list_2`: Sizes a memory request up to a minimum and rounds it to a multiple of four bytes, then walks an address-ordered free list to find where the block belongs.
- `insert_block_into_free_list`: Traverses an address-ordered singly linked list to find the insertion point for a memory block, merging it with adjacent nodes when they touch.
- `insert_free_block_sorted`: Walks a singly linked list of address-ordered nodes to find the insertion point for a block, returning minus one if the block overlaps an existing node.
- `insert_and_coalesce_free_block`: Walks a singly linked list of address-sorted, size-tagged nodes to insert a block at its sorted position, merging it with adjacent nodes when they are contiguous.
- `adjust_size_and_unlink_predecessor`: Adds a value to a size field of a node, and if the node has a non-null link equal to a given pointer, merges the successor's size into it and copies the successor's link.
- `insert_node_into_sorted_list`: Walks a singly linked list of address-sorted, size-tagged nodes, inserting or coalescing a free block at its correct position.
- `unlink_node_and_zero`: Writes the stored predecessor pointer into the location referenced by a structure field, then clears the first word of the node it referenced and stores a register value in its second word.
- `append_byte_increment_counter`: Increments a word counter in a structure, stores the low byte of its argument through a pointer held in that structure, then advances that pointer by one byte.
- `store_result_and_format_text`: Clears a word counter in a global block, stores the first parameter as a pointer there, and calls a text formatting routine with the second parameter.
- `parse_signed_decimal_string`: Reads an optional leading sign and then consumes decimal digits from a character string, accumulating them into a signed word stored through the second argument.
- `format_signed_decimal_string`: Writes a minus sign into a buffer when the given value is negative, then appends the decimal representation of the value by calling another routine.
- `format_unsigned_decimal_string`: Converts a 16-bit unsigned value to a null-terminated decimal string of up to five characters, written into a buffer supplied by the caller, with leading zeros suppressed.
- `compute_packed_time_value`: Copies bytes from a source buffer into a local structure and runs two byte-counting loops over those values.
- `parse_packed_time_field`: Copies three bytes from a source buffer, or from a default location if a pointer is null, into a fixed buffer in a structure with a terminating zero.
- `lookup_and_call_indexed_entry`: Looks up a record through an indexed table and returns minus one if it is absent.
- `parse_printf_format_specifier`: Initialises a set of local format fields, then reads a printf-style conversion specification by matching flag characters through a table and jumping to a handler when one matches.
- `dispatch_printf_flag_handler`: Reads the character after a printf-style conversion, scans a small table for a matching flag or conversion character, and jumps to the corresponding handler routine when one matches.
- `parse_printf_conversion_spec`: Parses the flags, width, precision and length modifier of a printf-style conversion from a format string, advancing the string pointer and storing the parsed values in local variables.
- `parse_format_specifier`: Parses a printf-style format specifier from a character string: it skips an optional leading zero flag, reads an optional width (a literal number or a `*` taken from an argument list), an optional precision after a dot, an optional size modifier 'l' or 'h', and then the conversion character.
- `emit_formatted_text_characters`: Scans a byte string, copying ordinary characters and doubled percent signs through a output routine and delegating each conversion specification to a parser.
- `divide_32_by_16`: Computes a 32-bit by 16-bit division using a normalization loop that shifts the divisor so its top bit is set, then forms and corrects a quotient.
- `normalize_and_divide_16bit`: Normalizes a 16-bit divisor by shifting it left until its top bit is set, tracking the shift count, then divides a 32-bit dividend by it and corrects the quotient and remainder.
- `compute_packed_halfword_products`: Calls another routine through an indirect table entry and combines the packed 16-bit fields of its result by multiplying pairs of halfwords and summing them into a 32-bit value.
- `multiply_32bit_words`: Takes two 32-bit values from registers and returns their 32-bit product, built from 16-bit partial products including cross terms.
- `allocate_and_fill_record`: Allocates a memory block through a library call, stores a byte, a word, and several flag bytes into it, and writes a result pointer.
- `normalize_day_counter`: Reads a 16-bit value through a pointer and repeatedly subtracts a year-length amount, using a leap-year pattern based on a counter, until the value is below 366.
- `subtract_table_values_loop`: Reads a 16-bit value through a pointer and subtracts entries from a byte table in a loop of up to twelve steps while the value is at least each entry.
- `divide_word_by_seven`: Divides a 16-bit value taken from the parameter by seven and returns the quotient in the upper half and the remainder in the lower half of a 32-bit result.
- `allocate_and_initialize_block_2`: Allocates a memory block via a library call, stores a parameter, a byte value and flag bytes into it, and records a result pointer.
- `allocate_and_init_node`: Returns zero if the first argument is null; otherwise it allocates a memory block, stores a byte value of 5 and a word from the second argument into it, and stores the first argument as a longword pointer field.
- `reset_node_recursively`: Recursively processes a linked structure through a non-zero pointer field, then sets a byte field to 255 and a longword field to all ones before calling another routine on a byte field and a second routine with a fixed value.
- `reset_node_and_release`: If the given pointer is non-null, sets a byte field to 255 and two longword fields to all ones, then passes the pointer and a word field to another routine and returns its result.
- `release_resources_and_reply_2`: Calls an optional cleanup function pointer stored in a structure, then calls another routine, and conditionally invokes library close and free calls.
- `check_input_button_state`: Reads a value through a function pointer in a global table, masks it, and if the result is non-zero calls a second function pointer and signals an exec library routine when that call returns non-zero.
- `read_value_with_fallback`: Clears a word in a global variable, calls a function through a pointer table to obtain a value, and if that value is minus one it reads a status code through another function pointer, stores it in a global variable and sets a second global word to five.
- `poll_value_with_error_code`: Clears a word in a global variable, calls a table-indexed function to obtain a value, and when that value is minus one stores a status code in a global variable and sets another global word to five.
- `combine_value_by_mode`: Clears a global word, calls a function to get a value, and on a minus-one result records an error code and a second global word.
- `check_status_return_minus_one`: Clears a global word, calls a table-indexed function, and if it returns zero stores an error code in a global variable, sets a second global word to two, and returns minus one.
- `invoke_table_function_once`: Calls a function through a pointer table and returns zero.
- `check_and_retry_status`: Clears a global word, calls a table-indexed function and, if non-zero, invokes two further table functions.
- `check_nested_status_return`: Clears a global word and calls a table-indexed function.
- `install_serial_interrupt_server`: Fills in a global interrupt structure with a priority, a type byte, name and data pointers, and a handler address, then registers it through a library call.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `install_interrupt_server_handler`: Writes a set of constant bytes, words and a long into a global data block, including a code pointer to a local routine, then invokes a library function through a pointer stored in a global variable at a negative offset.
- `remove_interrupt_server`: Calls the Exec library function that removes an interrupt server, using a library base taken from a global variable.
- `initialize_resource_tables_and_handlers`: Opens system resources through library calls, then repeatedly requests slots and stores the returned pointers into several tables, retrying on failure and invoking a routine on each stored entry.
- `swap_buffer_and_call_handler`: Stores a pointer selected from a table indexed by a toggling flag into a global variable, flips that flag, invokes a function through a library base pointer, and then calls another routine.
- `wait_for_raster_line`: Polls a hardware register through a pointer until the vertical position field matches the high word of the parameter.
- `wait_for_blitter_idle`: Polls a hardware status register through a pointer until a particular bit clears, indicating a busy flag has been released.
- `shutdown_interrupts_and_libraries`: Stores a value into a structure field, writes a fixed control word to a hardware register, and then performs a sequence of library calls that clear interrupt handling and close the system libraries.
- `close_libraries_and_cleanup`: Conditionally calls one library function if a global variable is nonzero, then calls two further library functions and two more via a global library base, with no return value.
- `invoke_global_handler_with_params`: Loads a function pointer from a global variable at a fixed negative offset and calls it with eleven arguments, several of them constants and two derived from global base offsets.
- `select_pointer_then_dispatch`: Chooses one of two global-relative addresses based on whether a global word is zero, then calls a routine with a fixed set of arguments including that address.
- `select_and_draw_frame_buffer`: Chooses one of two data tables based on a global flag, passes it with a fixed set of constants (screen-like dimensions 320 by 200 and a 192 value) to another routine, and then calls a second routine.
- `select_and_draw_frame_buffer_2`: Chooses one of two alternate display or buffer tables based on a flag stored in a global variable, then passes that choice together with fixed dimensions and parameters to another routine that performs a drawing or setup operation, and finally calls a second routine.
- `fill_stack_buffer_and_copy`: Fills a sixteen-word stack buffer with the 16-bit value of a parameter, then passes that buffer to another routine that copies it.
- `wait_and_release_resource_slots`: Calls a library function through a stored offset, repeatedly requests a free slot until one is granted, then calls a routine five times to release four stored pointers and one further value.
- `wait_and_release_resource_slots_variant`: Repeats the same acquire-then-release sequence as another routine, using a single slot request passed in from the caller and different offsets into the structure for its four stored pointers and one further value.
- `acquire_slot_and_release_pointers`: Calls a library function, then retries a slot request until it succeeds, then calls a routine four times with stored pointers and once with a further value.
- `acquire_slot_and_release_offsets`: Calls a library function, retries a slot request until it succeeds, and then calls a routine four times with pointers taken from another set of structure offsets plus one further value.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `wait_then_copy_buffer`: Calls a routine with a value derived from a global base offset, then calls another routine that copies sixteen words.
- `build_nibble_plane_pixels`: Expands a count of 16 nibble-packed source words into a destination table by iterating over 16 bit-plane levels, and for each level writes a level value into the nibble fields of each word whose stored nibble is at or above that level (ascending or descending order depending on a flag).
- `expand_nibbles_into_buffer`: Copies sixteen 16-bit words from one buffer into a second, then for each nibble-packed word runs a loop over values from fifteen down to zero, writing the highest-value nibbles into the second buffer.
- `wait_for_raster_line_2`: Repeatedly polls a video hardware position register through a pointer loaded from a global structure, busy-waiting until the beam reaches a fixed scanline, for a count given by the parameter.
- `copy_sixteen_words_twice`: Copies sixteen 16-bit words from a source buffer into two destinations, each reached through a pointer read from a global variable and advanced by four bytes per word.
- `store_two_words_globally`: Stores the two 16-bit halves of its 32-bit argument into two consecutive words of a global variable block.
- `process_string_bytes_by_routine`: Iterates over a null-terminated byte string through a pointer parameter, calling another routine once per byte.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `process_null_terminated_bytes`: Walks a null-terminated byte string, advancing one byte per iteration and calling another routine for each byte until the terminator is reached.
- `copy_column_bytes_to_planes`: Computes a screen offset from two global variables, then over eight iterations writes zeros to three destination byte planes and copies one source byte into a fourth, stepping each pointer by a fixed stride.
- `add_eight_to_word`: Adds eight to a 16-bit value held in a global variable block.
- `copy_eight_rows_to_planes`: Iterates eight times, writing zero to three destination byte locations and copying one source byte into a fourth, with each set of pointers advanced by fixed row strides; the destination set is chosen by a global flag.
- `return_global_long`: Returns the 32-bit value stored at a fixed offset in a global variable block.
- `compute_state_code`: Checks a series of byte flags in a global variable block and hardware-related input words, overwriting a result byte in priority order, and returns that byte as a small code from 1 to 7.
- `copy_stack_word_to_global`: Calls another routine to fill a two-byte local buffer on the stack, then copies the resulting halfword into a global variable.
- `call_library_function_offset`: Loads a library base pointer from a global variable and calls a function at a fixed negative offset from it.
- `call_library_function_minus_234`: Loads an address relative to a global base and calls a function at a fixed negative offset from a library base pointer stored in a global variable.
- `call_library_function_minus_240`: Calls a function at a fixed negative offset from a library base pointer stored in a global variable.
- `call_library_with_string_length`: Scans a null-terminated string to find its length, then calls three functions through a library base pointer at fixed negative offsets, passing the string along the way.
- `invoke_indirect_library_function`: Loads a pointer from a global data area and calls a function through a fixed negative offset in a jump table, then returns.
- `call_library_then_measure_string`: Calls two library functions through the jump table, then advances a pointer past a null-terminated string and calls a third library function.
- `release_block_entries_when_slot_granted`: Calls a library function and makes one slot request; if granted, it calls a routine ten times with values from a pointer table and twice more with fixed offsets, then calls further routines to finish.
- `wait_then_release_resource_slots_alt`: Calls a library function, requests a slot and returns if it is not granted; otherwise it invokes a routine ten times with values from a pointer table, then twice more with fixed values, and finally makes two indirect calls.
- `swap_byte_and_index_table`: Reads a byte from a global variable, clears that byte and sets the following byte to a constant, then returns a byte fetched from a table indexed by the saved value.
- `shift_serial_bit_into_table`: Shifts one bit from a serial input register into a global variable, then stores either 255 or 0 into a byte table entry indexed by that variable's value.
- `release_indexed_slot_status_2`: Looks up a pointer and returns a sentinel value if it is null, or zero if a flag bit is set.
- `lookup_slot_or_fail_2`: Finds a slot from a stored pointer and returns minus one if none is found.
- `forward_call_to_another_routine`: Contains no logic of its own and simply calls another routine before returning.
- `compute_packed_value_sum`: Fills a local structure by calling another routine, runs two byte-counting loops over its fields, then adds a global variable, a byte field and a value returned by another routine.
- `walk_flagged_list_records`: Walks a singly linked list of records from a fixed offset in a global base, and for each record with a particular flag pattern whose two halfword fields differ, calls another routine.
- `rotate_table_entries_conditionally`: When a mode variable is nonzero and a flag byte is clear and odd, it selects one of three table offsets and rotates a run of 16-bit entries in place by one position, copying the displaced entry into the mirrored positions of two other tables.
- `return_zero_constant`: Empty body that returns the constant zero to the caller without any other effect.
- `queue_request_in_free_slot`: Clears a global word and scans a table of six-byte records for the first unused entry, returning minus one if none is free.
- `return_indexed_lookup_status`: Looks up a record through another routine and returns minus one if none is found.
- `call_global_function_pointer`: Loads a function pointer from a global variable at a fixed negative offset and calls it with no other operations.
- `initialize_table_of_entries`: Builds a table of entries by reading size-and-divisor pairs from a data table, storing computed values into a global array, and initializing a loop of four records with fixed field values.
- `silence_audio_and_reset_state`: Clears a set of global variables to zero, writes fixed values to several global variables, and clears the global variables used for audio control and DMA settings.
- `update_audio_channel_voices`: Iterates over four audio channel state blocks, advancing each channel's sample pointer, period and volume state and writing the results to the Paula audio hardware registers via DMA control.
- `advance_audio_channel_stream`: Reads the next command byte from a per-channel music or sound stream, which may set an envelope, instrument or pitch and advance the stream pointer, or otherwise handle a note event.
- `parse_music_stream_commands`: Reads a byte stream in a loop, dispatching each command byte: some bytes set per-channel pointers and tables, others set a timing value, and bytes in the low range are treated as jumps through a table of handlers.
- `update_audio_channel_voices_2`: Iterates over four audio channel state blocks, advancing each channel's sample pointer, period and volume state, and writes the resulting period and volume values to the hardware audio channel registers when a channel is not muted.
- `advance_four_audio_channels`: Loops over four per-channel state blocks in a table, each time loading a stored sample pointer and period, decrementing a per-channel countdown, and when it expires reloading its envelope data.
- `clear_state_and_set_values`: Sets a series of global variables to zero, then writes several fixed constant values into global variables in sequence.
- `advance_audio_channel_state`: Reads a music or sound-effect event stream through a pointer and, for each of the four audio channels, updates the channel's sample pointer, period and volume state blocks, then either programs the channel's hardware registers or recursively processes the next channel.
- `initialize_audio_channel_state`: Clears several global variables and a small structure, writes hardware control and audio length registers, copies a block of words from a table into a buffer, and sets a handler pointer before waiting in a short countdown loop.
- `copy_table_to_buffer_and_delay`: Clears a flag byte, stores a value into a global variable and a small structure, copies ten words from a table at an offset into a buffer, then sets further flag bytes and a computed pointer before running a countdown loop.
- `rotate_bits_into_accumulators`: Computes a byte from masked global bits and shifts one bit into each of two 16-bit global variables, then returns a scaled value of that byte.
- `update_sprite_state_and_copper`: Decrements a per-channel countdown and, on reaching zero, clears a flag and copies a block of four words into a second structure, then sets a DMA control bit.
- `reset_board_and_run_game_loop`: Calls an initialization routine, then repeatedly clears a 1024-byte region and seven state words and reinitializes several tables and routines, looping in an outer game-round structure while a flag word is set.
- `reset_board_and_start_round`: Clears the flag grid and a set of cell flags to 255, resets two global counters and a timer word, then reads a random value to set a state index, and finally draws a shape into the bitplanes and updates a score table via further routines.
- `draw_flagged_cells_grid`: Walks a 20-row grid of byte flags and, for each non-zero flag in columns 13 through 22, calls another routine to draw a block at that position.
- `update_cell_flags_and_redraw`: Writes a value into a flag grid at a position computed from global variables, clears a sub-region of flags, and when a global word is set, writes indexed values into a second array.
- `clear_four_table_bytes`: Loops four times, computing an offset from two 16-bit values in a global variable area and a byte from a table, and stores a zero byte at each resulting address in a global variable area.
- `fill_stack_buffer_and_copy_2`: Fills a sixteen-word stack buffer with the low 16 bits of its parameter by looping over each element, then passes the buffer to another routine.
- `release_block_entries_when_granted`: Calls a library function and requests a slot; if the slot is granted, it invokes a routine ten times with values taken from a pointer table, then twice more with fixed values, and finally a fixed routine.
- `invoke_global_handler_with_eleven_args`: Loads a function pointer from a global variable at a fixed negative offset and calls it with eleven arguments, several of them constants and some computed from the base register.
- `wait_then_release_resource_slots`: Repeatedly requests a slot through a routine until one is granted, then invokes a routine four times with values read from fixed structure fields, then once with a fixed value, followed by two indirect calls to release the slot.
- `return_global_long_2`: Returns the longword stored at a fixed offset in a global variable block.
- `select_pointer_then_dispatch_2`: Chooses one of two base-relative addresses depending on whether a global word is zero, then calls another routine with a fixed set of arguments and makes a further call.
- `invoke_four_subroutines_in_sequence`: Calls one routine with a constant argument, then calls another routine with a value computed from a global base offset plus a fixed displacement, then calls two further routines with no arguments.
- `poll_state_until_zero_count_2`: Loops over a counter read through a parameter, decrementing it each pass while a global state byte is zero and a polled value is not four.
- `fetch_word_into_global`: Calls another routine to fill a two-byte buffer on the stack, then copies the resulting 16-bit value into a global variable addressed relative to the data base register.
- `compose_shape_into_bitplanes`: Reads a list of per-row values and a shape table, then ORs shape bytes into four bitplane buffers at computed row offsets, looping over up to ten columns and shifting the row value by eight bits per step.
- `swap_buffer_and_call_handler_2`: Stores a pointer selected from a table by a toggling word into a global variable, flips that word between zero and one, calls a library function through a pointer, and then calls another routine.
- `wait_for_raster_line_3`: Loops reading a hardware register through a pointer until the vertical position bits match the high word of the parameter.
- `pack_nibble_max_into_words`: Copies a source array of 16-bit words into a destination buffer, then repeatedly scans the source words from the largest nibble value down to zero, overwriting the packed nibble fields of each destination word with the current value wherever a source nibble is at least that value, and then calls two other routines after each pass.
- `compute_state_code_2`: Checks a series of byte flags in a global variable block and two input words, overwriting a result byte with a value from a fixed priority order, and returns that byte.
- `prompt_name_entry`: Clears a global word, draws labels and a boxed field, then reads up to ten characters into a text buffer chosen by an argument, handling backspace and carriage return.
- `run_menu_selection_loop_alt`: Draws a set of text items, then loops up to 75 times polling input and adjusting a bounded counter on two input codes while redrawing the affected text, leaving the loop when a global flag is zero.
- `shift_rows_down_table_2`: Scans a table in rows of fixed width for a stored value; when a match is found it skips that row, otherwise it copies each byte down one row and increments a counter parameter, repeating until the scan index reaches zero.
- `expand_nibble_packed_bit_planes`: Expands a count of 16-bit source words, each holding three 4-bit values, into a destination table by sweeping 16 intensity levels; for each level it sets the matching 4-bit fields of every table entry whose source value is at or above (or at or below, depending on a flag) that level, then calls another routine twice to emit the table for that level.
- `call_buffer_copy_twice`: Calls a routine with a value computed from a global base offset plus a fixed constant, then calls a second routine with no arguments.
- `return_zero_after_table_build`: Calls another routine that builds a table, then returns zero to its caller.
- `allocate_resources_and_fill_tables`: Calls a library function through a base pointer to allocate memory blocks in a loop of four, storing each returned pointer into several tables and retrying or calling a failure routine when a pointer is null.
- `advance_byte_state_pair_2`: Updates two global byte variables: one is recomputed from its previous value and a rotated feedback of itself, then shifted repeatedly while adding carries into it; the other is incremented by one or two depending on the low bit of the first.
- `copy_sixteen_words_twice_2`: Copies sixteen 16-bit words from a source buffer into two destinations, each reached through a pointer held in a global variable, advancing all pointers by one word per copy.
- `busy_wait_on_counter`: Repeatedly polls a video position register through a pointer from a global structure, waiting for the beam to reach a fixed line and then leave it, for a count passed in as a parameter.
- `or_masked_block_into_table_2`: Ors bytes from a source region into four interleaved rows of a table, using complemented mask bytes to preserve existing bits, over a fixed number of iterations.
- `return_without_action`: This routine is empty: it contains a single return instruction and performs no operations.
- `wait_then_release_slot_from_argument`: Repeatedly requests a slot identified by a passed-in value until it is granted, then invokes a routine four times using values from fixed structure fields, once more with a fixed value, and finishes with two indirect calls that release the slot.
- `draw_frame_with_selected_table`: Selects one of two base-relative data tables based on a global word, passes it with fixed constants, and then calls another routine.
- `call_library_function_minus_222`: Loads a library base from a global variable and calls the function at offset minus 222 from that base, with no further work.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `initialize_self_referencing_pointer`: Stores the argument's own address into the memory it points to, adds four to it, clears the following word, and stores the original address in the word after that.
- `dispatch_and_cleanup_sequence`: Checks the stack against a limit, and if the check passes it calls an indirect routine, combines packed halfword products from its result, and returns the combined value.
- `init_board_and_enter_game_loop`: Calls one routine, then copies sixteen words of table data into another table, makes a series of calls that initialise several tables and state words, and then repeatedly clears a 1024-byte region and seven state words before re-running an inner loop.
- `invoke_cleanup_and_reply`: Calls an optional cleanup function pointer taken from a structure, then calls another routine, and conditionally closes a library.
- `shift_keyboard_serial_bit`: Shifts a bit from the keyboard serial register into a global variable and stores either 255 or 0 into a table entry indexed by that variable, returning zero.
- `clear_byte_and_global`: Stores zero into a memory location addressed by a base register plus a 16-bit offset and clears a global variable, returning zero.
- `advance_audio_channel_state_2`: Checks a global mode variable and a flag byte, then selects one of three table offsets and rotates entries of a table in place, copying the end value back to the start.
- `call_table_builder_and_return`: Calls another routine that builds a table of entries, then returns zero as its result.
- `forward_to_routine`: Calls another routine and returns, doing nothing else.
- `dispatch_to_defaults_routine`: Calls another routine and returns, with the body reduced to a single call after compilation.
- `return_word_global`: Returns the value of a 16-bit global variable to the caller.
- `advance_byte_state_pair`: Updates two global byte variables: one is combined from its previous value and a rotated feedback of itself, the other is incremented by one or two depending on a bit of the first.
- `or_masked_block_into_table`: Ors bytes from a source region into four interleaved rows of a table, using complemented mask bytes to preserve existing bits, over a fixed number of iterations with conditional handling of the second column.
- `initialize_table_of_entries_2`: Builds a table of entries by reading size and divisor pairs from a data table and storing computed values into a global array.
- `initialize_audio_channel_state_2`: Clears and sets several global variables and a small structure, writes control and length values to hardware registers, copies a block of words from a table into the structure, stores a handler pointer taken from a table, and then waits in a countdown loop before clearing a byte.
- `clear_globals_and_set_defaults`: Clears a set of global variables to zero, then writes fixed values into several other global variables.
- `advance_audio_channel_voices`: Iterates over four audio channel state blocks, advancing each channel's sample pointer, period and volume state and writing the results to the audio hardware registers for channels that are not muted.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_interrupt_and_dma`: Copies four words from hardware registers into global variables, then writes a fixed value to a global variable and to several hardware control registers to set up interrupt and DMA enables.
- `clear_flag_and_delay`: Clears bit seven of a global variable, runs a countdown loop of about one hundred outer iterations clearing bit four each time, then calls another routine.
- `spin_delay_then_wait`: Runs a counted countdown loop of about three thousand iterations, then loops until a specific bit of a global variable is clear.
- `loop_while_bit_set`: While bit four of a global variable is set, sets the low two bits of that variable and calls another routine, repeating until the bit clears.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `reset_port_and_wait_bit`: Clears one bit of a global variable and writes to hardware registers in a loop, then polls a parallel port status register for a set bit with a bounded countdown.
- `clear_word_and_set_field`: Writes zero to one word and the value sixteen to another word at fixed offsets from a register-held base pointer.
- `wait_until_flag_set`: Loops until a specific bit of a global variable becomes set, then returns.
- `copy_words_with_set_high_bit`: Stores four global words into fixed offsets of a base pointer, each with the high bit forced to one.
- `scan_longwords_for_marker`: Walks a block of memory word by word, shifting each longword bit by bit while comparing its upper word against a fixed marker value.
- `scan_words_for_marker`: Scans a block of memory word by word, shifting each longword bit by bit and comparing the upper word against a fixed marker value.
- `set_flag_from_comparison`: Compares a register value against a constant and stores zero or one into a global variable depending on whether the result is non-negative.
- `set_global_flag_to_one`: Stores the value one into a global variable and returns.
- `call_library_and_store_result`: Calls a library function at a fixed negative offset from a library base loaded from a global variable, and stores the returned value into another global variable.
- `call_library_function_no_result`: Calls a library function at a fixed negative offset from a library base loaded from a global variable, discarding any return value.
- `return_immediately_empty`: Empty routine that performs no operations and returns immediately.
- `draw_piece_into_bitplanes`: Reads a shape table and a sequence of per-row values, then ORs the shape's bytes into four bitplane buffers at computed offsets, shifting by eight bits per step and looping over a fixed number of rows.
- `shift_rows_down_table`: Scans a table in rows of fixed width against a stored value, and when a match is found skips a row, otherwise copies each byte down one row-width in the table, repeating until the top is reached.
- `run_menu_selection_loop`: Draws several text items, then loops up to 75 times polling input, adjusting a bounded selection counter on two input codes and redrawing the affected text, and exits early when a global flag becomes zero.
- `draw_playfield_cell_grid`: Loops over ten rows, and for each row calls another routine to draw a fixed-size block using a value read from a table indexed by the row, then issues a series of draw calls with fixed offsets and constant sizes.
- `poll_state_until_zero_count`: Loops a counter stored at a parameter-supplied location, decrementing it on each pass while a global state byte is zero and a computed state code is not four.
- `read_text_input_into_field`: Clears a global counter, draws labels and a boxed field, then reads characters into a selected text buffer of up to ten characters, handling a backspace code by blanking the last character and a carriage return to finish.
- `invoke_sequence_of_routines`: Calls another routine with a constant, then calls another routine with a value derived from a global base offset, and then calls two further routines with no arguments.
- `call_three_library_functions_strlen`: Calls two library functions through a library base pointer, scans a null-terminated string to find its length, then calls a third library function with the result.
- `call_indirect_library_entry`: Loads a pointer from a global variable, indexes a fixed negative offset into the structure it points to, and calls the function found there.
- `store_two_words_globally_2`: Stores the high and low 16-bit halves of its 32-bit argument into two consecutive words of a global variable block.
- `format_signed_integer_string`: Writes a minus sign into a buffer when the given integer is negative, then calls another routine to append the decimal digits.
- `swap_byte_and_index_table_2`: Reads a byte from a global variable, clears it and sets the following byte to a constant, then returns a byte fetched from a table indexed by the original value.
- `process_string_bytes_alternate`: Iterates over a null-terminated byte string through a pointer parameter, calling another routine once per byte.
- `call_library_then_measure_string_2`: Calls two library functions through a library base pointer, scans a null-terminated string to find its length, then calls a third library function.
- `swap_buffer_and_call_handler_3`: Stores a pointer selected from a table by a toggling flag into a global variable, flips that flag, and then invokes a function through a library base pointer before recursing into itself.
- `expand_nibbles_into_buffer_2`: Copies a block of 16-bit words into a second buffer, then for each value from fifteen down to zero, updates each word's nibble fields in that buffer with the value wherever the source nibble is at least that value.
- `compute_state_code_3`: Checks several byte flags in a global variable block and two hardware input words, overwriting a result byte in priority order.
- `draw_frame_with_alternate_table`: Chooses one of two base-relative tables based on a global word, passes it with a different fixed offset and fixed constants, and then calls another routine.
- `expand_nibble_bitplanes_to_table`: Fills a table of 16-bit words from a source array of nibble-packed words, iterating over 16 levels and, for each level, setting the high, middle and low nibbles of each entry to the current level where the source nibble is at or above it (ascending or descending order depending on a flag).
- `return_zero_after_call`: Calls another routine that builds a table, then returns zero as its result.
- `store_pointer_clear_word_and_format`: Zeroes a 16-bit value in a global block, stores the first argument as a pointer there, and calls another routine to format text using the second argument.
- `write_block_to_two_buffers`: Copies sixteen 16-bit words from the source buffer passed in, writing each word to two destinations reached through pointers held in global variables, and advancing the destination pointers by four bytes per word.
- `wait_for_raster_line_2_2`: Repeatedly polls a video hardware position register through a pointer loaded from a global structure, busy-waiting until the beam reaches a specific line, repeated a counted number of times.
- `dispatch_through_library_slot`: Loads a pointer from a global data area and calls a function through a fixed negative offset in a jump table, then returns.
- `conditionally_iterate_slots_and_release`: Calls a library function, then requests a slot and returns early if it is not granted.
- `allocate_and_initialize_block`: Calls a library function to allocate a memory block, fills in fields including a stored parameter and flag bytes, and calls further library functions, returning the block address or zero.
- `reset_object_and_recurse`: Recursively processes a linked structure when a pointer field is non-zero, sets a byte field to all ones and a four-byte field to all ones, then calls another routine on a byte field value and again with a constant second argument.
- `store_word_and_pointer_in_node`: Calls a library-backed allocation routine with a second argument, and if it returns a non-null block, stores a byte value, a word from the second argument, and the first argument pointer into fixed offsets of that block.
- `reset_node_and_release_it`: If the given pointer is non-null, it sets a byte field to 255 and clears two longword fields of the structure, then passes the pointer and a word field to another routine and returns its result.
- `call_indirect_via_global`: Loads a function pointer from a global variable at a fixed negative offset and calls it, with no other operations.

### Tetris (1987)(Mirrorsoft).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
