# Game 955 (HiredGuns v1.0 1MB 1114): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00955). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (5328977 top-level bytes in 67 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 169 of 169 named (13 author symbols), 169 of 169 described (14 by rule, 154 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 781 of 781 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 4669440 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6995 of 6995 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 320964 |
| code | 20804 |
| data | 4987059 |
| padding | 150 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Crunch | 1 | 473 | 13 |
| Display | 2 | 6522 | 156 |
| HiredGuns_v1.0_1MB_1114.hdf (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/Atmos/Atmos00.sfx: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9562 -> 15320 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/Atmos/Atmos01.sfx: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (17244 -> 19686 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/Atmos/Atmos02.sfx: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4348 -> 11686 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/Atmos/Atmos03.sfx: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (16919 -> 19702 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/Atmos/Atmos04.sfx: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13440 -> 19908 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/Atmos/Atmos05.sfx: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10129 -> 19218 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/Atmos/Atmos06.sfx: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10888 -> 15068 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/Atmos/Atmos07.sfx: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14783 -> 19386 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/Atmos/Water.sfx: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11854 -> 19148 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/ChSelect/ChSelect.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (135407 -> 230272 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114_hdf/HiredGuns1MB/game/ChSelect/Characters.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2267 -> 16320 bytes)
- HiredGuns_v1.0_1MB_1114/HiredGuns_v1.0_1MB_1114.hdf: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 4669440 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Crunch

- `START`: Initializes the DMACrunch utility by opening required libraries and retrieving filenames.
- `GetFilenames`: Processes input strings to identify filenames and appends specific file extensions based on detected flags.
- `CopyName`: The routine copies bytes from a source pointer to a destination pointer until it encounters a null byte or a space, tab, or newline delimiter, then writes a null terminator at the destination.
- `DelSpaces`: The routine iterates through a string and returns the first character encountered that is not a space or a horizontal tab, or returns zero if a null terminator or newline is found.
- `WriteMessage`: The routine scans a zero-terminated string in a register until the terminator is found and then calls a function through a table stored in a global variable.
- `Uncompress`: The routine repeatedly reads a control bit and then a length value from the bitstream using another routine.
- `DoDuplicate`: The routine calls another routine and uses its return value as an index offset to copy bytes from a source area pointed to by a pointer backwards into itself, decrementing a loop counter until it reaches -1.
- `ReadBits`: This routine reads a bitstream from a memory pointer and reconstructs a bit sequence into a word by shifting bits and decrementing a counter.
- `CrunchFile`: Sets up a work area, repeatedly calls a compressor and occasionally updates a progress counter and message before storing a small header into the output buffer and returning a status value.
- `Crunch`: Analyzes a data stream to find repeating sequences and writes compressed bit patterns to an output buffer.
- `SizeLength`: The routine compares sequences of bytes to find matches, updates a table of lengths and counts, and writes bitstream data based on the results of these comparisons.
- `WriteBits`: Decrements a register in a loop and returns when the loop counter reaches zero.
- `PutInJmpHeader`: Processes a value by performing bitwise shifts and writes a sequence of bytes to a destination pointer.

### Display

- `initialize_display_subsystem`: Clears a memory block and initializes a global display state structure by invoking several internal routines.
- `execute_cleanup_callbacks`: The routine runs a series of stored callbacks from a structure, invokes two other routines, and conditionally runs two more callbacks based on a flag in the structure.
- `calculate_string_lengths`: Calculates the length of two null-terminated strings and calls a routine with the resulting length and pointer.
- `initialize_display_state`: Sets up a display-related state in a global structure by either copying a name string into a pre-existing buffer or allocating a new buffer and registering a new entry in a list.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_display_structures`: Initializes a display-related structure by retrieving various library pointers and setting up internal buffers.
- `wait_for_input_events`: The routine repeatedly polls an input routine via a function pointer stored in a global variable and checks the returned bitmask against a global mask and specific bits.
- `handle_display_update`: The routine calls another routine with the input parameters and branches on the result.
- `forward_argument_to_routine`: Passes a received parameter to another routine and returns the result.
- `update_display_state_3`: The routine updates a global display state structure, invoking callbacks from a table and conditionally calling another routine with parameters taken from the structure and from the argument.
- `poll_input_events_loop`: The routine repeatedly fetches an event from an input queue via a function pointer stored in a global variable and processes the event fields.
- `format_string_list`: Iterates through a range of indices, calling a routine to format a string into a buffer and then looking up a callback or pointer via an ID.
- `format_and_update_display`: Checks if a string is non-empty, calls a routine to format it, and then decrements or increments a counter or pointer.
- `check_display_flags_and_wait`: The routine checks a set of global flags and, when a flag is set, calls another routine through a table and waits in a loop using a flag.
- `parse_string_table_flags`: The routine iterates over a table of string pointers and scans each string character by character, setting word flags in a structure pointed to by A4 for specific letters found.
- `update_display_state_2`: Retrieves a pointer from a library, stores it in a global variable, and if not null, calls a function that potentially calls another routine via a pointer before calling a library cleanup routine.
- `update_display_state`: The routine updates a block of word fields in a structure pointed to by a register, using values returned by another routine called repeatedly with the same argument and a global variable.
- `set_display_flags_from_icon`: Queries an icon structure via icon.library and library calls, then calls another routine with a pointer from the icon and a global variable.
- `parse_string_flags`: The routine scans a zero-terminated byte string and updates specific word or byte fields in a structure based on the characters encountered.
- `check_icon_and_dispatch`: The routine calls another routine via a function pointer stored in a table and, if the result is non-zero, calls another routine with that result and a global variable.
- `wait_for_event_and_callback`: The routine enters a loop checking a global variable, calls a routine through a pointer, and conditionally decrements a counter and calls another routine before finally calling a routine from a global table.
- `clear_flag_and_call_callback`: The routine clears a word at a specific offset from a pointer and then calls a routine obtained by indexing a table stored in a global variable.
- `return_empty_stub`: An empty stub routine consisting of four instructions that performs no observable operation before returning.
- `parse_text_file_into_tokens`: Opens a file, reads its contents into a buffer, and counts newline characters to determine the number of lines.
- `parse_display_option`: The routine parses a display option flag from a string and stores the result through a pointer.
- `reset_display_state_3`: The routine clears a set of fields in a structure pointed to by its argument and, depending on flags in that structure, invokes two different callbacks obtained from a global variable table.
- `parse_iff_chunk_tree`: The routine walks a linked IFF chunk tree and dispatches processing to child chunks, returning early on error codes from the library.
- `count_nonzero_words`: The routine receives a pointer to an array of 32-bit values and iterates through the array until it finds a zero entry, counting how many entries were examined.
- `compare_library_ids`: Checks if a library-related structure contains specific identification or tag values matching provided parameters.
- `check_iff_chunk_match`: The routine reads a global variable, calls another routine to obtain a pointer, then calls another routine to get a chunk descriptor.
- `call_indirect_via_table`: The routine loads a function pointer from a global variable table and calls another routine indirectly, returning its result.
- `call_indirect_via_table_2`: The routine loads a function pointer from a global variable table and calls another routine through it, returning the result.
- `get_iff_chunk_id`: The routine calls a function through a pointer stored in a global variable and checks the return value.
- `check_pointer_and_return_status`: The routine loads a pointer from a table and checks the value at the destination.
- `branch_on_count`: Loads a pointer from a table and performs a series of conditional branches based on the value of a decremented counter.
- `clamp_index_to_limit`: Compares a longword in a data register against a maximum threshold.
- `perform_binary_search`: The routine performs a search by repeatedly calling another routine with adjusted bounds until a match is found or the search space is exhausted.
- `return_zero_and_branch`: The routine clears the accumulator and branches to another location.
- `clamp_value_to_limit`: Compares a register against a constant and sets it to that constant if the value is greater than the limit.
- `calculate_gcd_iterative`: The routine performs an iterative calculation using a subtraction-based algorithm to find a greatest common divisor and returns zero or a negative value based on the result.
- `clear_register_and_branch`: Clears the first data register and performs a short unconditional branch.
- `execute_looping_call`: The routine pushes registers onto the stack, calls a subroutine, and then enters a loop that increments a register and branches based on a comparison.
- `zero_d0_and_branch`: Sets the d0 register to zero and performs a short branch to another routine.
- `return_zero`: The routine clears the accumulator and returns a zero value.
- `indirect_call_via_table`: The routine performs an indirect call through a pointer stored in a global variable.
- `process_iff_data`: The routine uses a library base from a global variable to call a function, and if the result is zero, it calls another library function and compares its result to a parameter.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `cache_catalog_pointer`: The routine checks a global flag and a cached pointer.
- `check_and_call_callback`: The routine checks two fields of a structure pointed to by a register and, if both are non-zero, calls a function pointer stored in the structure.
- `select_display_mode`: The routine selects a display mode based on an input parameter and returns a pointer derived from a global variable.
- `resolve_display_parameter`: The routine maps a small integer parameter to a display routine address or a global variable address.
- `lookup_callback_by_id`: The routine walks a table pointed to by a4 plus an offset, comparing each entry with param_1.
- `call_indirect_via_table_3`: The routine receives a pointer in a0 and calls the routine stored at an offset from that pointer.
- `execute_conditional_draw`: Checks for a specific string in memory and calls a routine to draw a block if a condition is met.
- `draw_scanline_block`: Processes a scanline buffer by validating header dimensions and clipping pixel offsets to the current display region.
- `process_display_list`: The routine reads a value from the pointed-to structure and, if it is zero, returns one.
- `set_display_mode`: The routine checks a flag in the supplied structure and, if set, calls another routine via a table and a second routine to obtain a value.
- `clear_object_flags`: The routine checks a flag field in a structure passed by pointer.
- `decode_planar_pixels`: Decodes a packed source into a destination buffer by extracting nibbles through a loop.
- `determine_display_flags_from_struct`: The routine reads several fields from a pointer to a structure and computes a 32-bit flag word.
- `copy_with_repeat_and_run_length`: The routine copies bytes from a source buffer to a destination buffer while interpreting the source as a run-length encoded stream.
- `check_and_load_image`: The routine checks a pointer and then calls another routine repeatedly with different constant arguments and with values taken from the pointer.
- `execute_conditional_retry`: Calls a routine with a provided argument and, if the return value is non-zero, calls that same routine a second time before returning a boolean indicating if the initial result was non-zero.
- `load_and_draw_image`: Attempts to load an image and if it fails, tries an alternative pointer, and finally calls a function pointer if a certain value is non-zero.
- `check_and_repeat_call`: The routine calls another routine with the same argument, stores its short result, and if the result is non-zero it calls the same routine again with the same argument before returning a boolean indicating whether the result was non-zero.
- `fetch_and_render_message`: The routine looks up a data block using an ID, copies the block into a structure, and calculates display dimensions and flags.
- `call_and_return_argument`: It forwards the value it received to another routine and then returns without modifying anything.
- `initialize_display_window`: The routine prepares a window structure by copying a set of values from a table or a pointer stored in the object, then calls another routine to open a window and stores the resulting window pointer.
- `close_screens_and_window`: The routine clears screen and window handles stored in a structure and optionally calls a library routine to close a screen and a window, returning a flag that reflects whether a close operation was performed.
- `validate_and_reinitialize_display`: Checks if the current screen dimensions meet minimum requirements and verifies hardware availability.
- `mask_input_bits`: The routine masks the input value with a constant bit pattern and returns the result.
- `select_and_clip_rectangle`: The routine chooses between two rectangles defined by param_5 and param_6 based on their widths and heights relative to param_1 and param_2, then computes a centered sub-rectangle of the chosen one clipped to param_7.
- `call_indirect_via_table_4`: The routine reads a pointer from a global variable, adds a constant offset, and calls the routine found there via an indirect call.
- `create_display_entry`: The routine builds a display list entry from a source structure and a set of parameters.
- `copy_struct_and_enqueue`: The routine copies seven words from a global variable into a stack buffer, then calls a library function through a function pointer.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `check_and_forward_call`: The routine calls another routine with the first parameter, and if the result is zero it returns all bits set.
- `copy_string_with_counter`: The routine measures the length of a null-terminated string and then copies each character into a buffer pointed to by a global variable, using a global counter to decide when to recurse to another routine.
- `format_string_with_buffer`: The routine walks a format string, copying characters to a destination buffer while handling %d, %p, %s and %x specifiers.
- `parse_printf_format_specifier`: Parses a format specifier string to extract flags, width, and precision.
- `parse_format_string`: Recursively walks a null-terminated string and calls a callback for each character, attempting to handle percent-prefixed format specifiers.
- `reset_weapon_state`: Resets fields of a structure pointed to by a parameter.
- `process_display_buffer`: Updates a display buffer structure by managing its internal counters, flags, and pointers.
- `parse_display_buffer_stream`: This routine iterates through a data stream within a buffer structure, managing internal pointers, counters, and status flags.
- `execute_cleanup_callbacks_2`: Iterates backwards through a table of pointers, checking a flag bit, and calls a routine if the flag is set and a certain bit is clear.
- `lowercase_ascii_to_uppercase`: The routine walks a zero-terminated byte string in place and, for each byte, checks a flag in a global table indexed by the byte value.
- `copy_n_terminated_bytes`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, stopping early if a counter reaches zero and returning the original destination pointer.
- `copy_string_with_limit`: Copies a certain number of bytes from a source string to a destination string, ensuring the destination is null-terminated.
- `compare_case_insensitive_strings`: The routine compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `skip_bytes_with_flag_set`: The routine increments a pointer while a byte from a table indexed by the pointer value has bit 3 set.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `format_signed_digit_string`: The routine writes a minus sign into the destination buffer when the input value is negative, then repeatedly calls another routine to obtain a digit character and a pointer, storing each ASCII digit into the buffer.
- `parse_signed_decimal`: Parses a signed decimal integer from a string, storing the result in an integer pointer, and returns the number of characters consumed.
- `increment_and_store_param`: The routine increments a counter stored in a global variable and stores the incoming parameter through a pointer held in another global variable, then advances that pointer by one element.
- `clear_and_fetch_result`: The routine clears a global variable, stores an input value into another global variable, calls another routine with the stored value and an argument, then clears the stored value and returns the result written by the called routine.
- `insert_memory_block`: The routine inserts or merges a memory block described by a pointer and a size into a linked list of blocks kept in a global variable.
- `search_display_entry`: Calls a routine to find an entry and then checks if a certain global flag or status or pointer is non-zero before returning a result.
- `update_buffer_byte`: The routine decrements two counters in a structure, increments a third, and stores a byte from a register into a location pointed to by a register.
- `call_other_routine_with_offset`: The routine calls another routine, passing an undefined byte from D7 and an address derived from A4 plus a constant offset.
- `clear_flag_and_fetch_result`: The routine clears a global variable, calls another routine with a pointer to a stack area and a parameter, then calls another routine with a constant and an address derived from a register, and finally returns the value stored in the cleared global variable.
- `update_display_entry`: The routine scans a table of entries in a global structure and updates the next free slot with a computed index and a result from another routine.
- `copy_memory_block`: Copies a specified number of bytes from a source address to a destination address.
- `clear_display_list`: The routine walks a linked list of display items referenced through a global variable, invoking another routine for each item until the list ends.
- `clear_pending_callback`: Clears a stored pointer from a global variable by invoking another routine with it and then nulling the variable.
- `forward_display_call`: Searches for an entry and, if found, checks a global flag before returning either a default or a result from a secondary routine.
- `allocate_and_link_block`: The routine allocates a block via another routine and, if successful, initializes fields in the block and links it into a linked list stored in a global variable.
- `allocate_memory_block`: Tries to find or split a free memory block from a linked list held in a global variable to satisfy a size request, updating the remaining size in another global variable.
- `copy_data_nested`: The routine performs a nested loop to copy bytes from a source to a destination buffer, while managing a counter in a structure and checking for a termination value.
- `loop_and_call_routine`: The routine performs a loop that calls another routine while incrementing a value and checking for zero.
- `decrement_and_branch`: It decrements a long value in a register and then performs a short branch to another location.
- `increment_register_and_branch`: Increments a long value in a register and performs a short branch to another location.
- `empty_routine`: This routine is empty and performs no work.
- `check_bit_and_call_routine`: The routine tests a bit in a structure and, if set, calls another routine with a specific constant value.
- `compare_and_branch`: Compares two values, checks a bit in a structure, and performs conditional branches or arithmetic based on the results.
- `reset_display_state_2`: Updates several fields in a structure, clears a bit in a 32-bit word at a fixed offset, and calls another routine with a pointer and a calculated value.
- `clear_display_flag`: The routine clears a single bit in a 32-bit word located at a fixed offset from a pointer in A3.
- `copy_stream_to_buffer`: The routine iterates through nested loops to extract bytes from a source pointer and store them into a destination buffer, decrementing a counter and checking for a termination value.
- `process_stream_result`: The routine pushes a register to the stack, calls a subroutine, and then compares the result to a constant to determine a branch path.
- `store_and_branch`: It moves a long value to a register, stores a byte from that register to a location pointed to by a register, decrements a long value, and then performs a short branch.
- `increment_and_branch`: Increments a 32-bit value in a register and then performs a relative branch to another location.
- `return_from_subroutine`: Performs a single return instruction from a subroutine.
- `find_or_allocate_entry`: The routine walks a linked list starting from a global variable to find an entry whose sixth field is non-zero.
- `parse_display_command`: The routine parses a command string and uses another routine to perform an action based on the first character and a flag at the second position.
- `clear_display_state`: The routine clears a status flag in a structure and conditionally invokes another routine based on a byte flag and two fields in the structure.
- `traverse_display_element_list`: Iterates through a linked list of display elements, checking specific bit-flags in a metadata byte for certain property requirements.
- `check_display_state`: The routine clears a global flag and then calls a function through a table to obtain a status value.
- `seek_file_offset`: The routine clears a global flag and invokes a seek helper via a function pointer stored in a global variable.
- `check_and_update_state`: The routine checks a global flag and optionally calls another routine, then clears a global variable.
- `clear_screen_flag_and_run_hook`: If a global flag is set, the routine calls another routine.
- `call_rem_task`: Library stub: loads its arguments and the exec library base, calls RemTask and returns its result to the caller.
- `cleanup_display_context`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `clear_message_buffer`: The routine writes a byte and two words to a structure pointed to by its argument and then calls another routine via a global variable table.
- `reset_display_state`: The routine clears a global variable and then calls a set of functions through a table pointed to by another global variable.
- `check_and_update_display_state`: The routine checks a global flag and optionally calls another routine, then clears a global variable and invokes several function pointers from a table held in a global variable.
- `close_display_if_active`: The routine checks a global variable and, if non-zero, calls another routine.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `normalize_fixed_point_divide_2`: Performs a 32-bit fixed-point division by normalizing the divisor and calculating the quotient.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `create_message_node`: It obtains a global variable and calls a routine through a table; if the result is a sentinel, it returns zero.
- `set_message_type_and_target`: When a non-zero parameter is passed, the routine calls another routine via a function pointer stored in a global variable table and, if the call returns a non-zero pointer, writes a message type byte, a zero byte, the original parameter as an integer, and a 16-bit value into fields of the returned structure.
- `clear_object_flag`: Obtains a pointer from another routine and, if the pointer is non-null, clears a flag in the pointed-to structure and optionally clears a global variable.
- `reset_display_flag_and_get_entry`: The routine clears a flag in a global variable and then checks a global index and a table entry for the given parameter.
- `parse_decimal_value`: Recursively calls a routine to parse a value and stores the result in a local or temporary variable.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_612`: Library stub: loads a library base from a global variable and calls the library function at offset minus 612 of that base, returning its result to the caller.
