# Game 972 (north & south (1989)(infogrames)(m5)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00972). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (921056 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 195 of 195 named (0 author symbols), 195 of 195 described (18 by rule, 176 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2444 of 2444 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5632 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4093 of 4093 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 20898 |
| data | 888851 |
| padding | 43 |
| unused | 11264 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| North & South (1989)(Infogrames)(M5).adf:bootblock | 1 | 189 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 31 | 31 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1000 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 141 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 33 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2680 | 123 |
| North & South (1989)(Infogrames)(M5).adf bytes 91136-96768 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- north & south (1989)(infogrames)(m5)/North & South (1989)(Infogrames)(M5).adf: custom-format (non-DOS) disk, 1736 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 31 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 484 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 124 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1581 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- north & south (1989)(infogrames)(m5)/North & South (1989)(Infogrames)(M5).adf: bytes 91136-96768 look like code no run loaded: recovered position-independently (load address unknown)
- north & south (1989)(infogrames)(m5)/North & South (1989)(Infogrames)(M5).adf: 1 logged loads cover 5632 bytes of the image
- north & south (1989)(infogrames)(m5)/North & South (1989)(Infogrames)(M5).adf: 22 track-loaded blocks found in recovered runtime segments and classified through them
- north & south (1989)(infogrames)(m5)/North & South (1989)(Infogrames)(M5).adf: blocks 2-21 (bytes 1024-11264) outside the filesystem hold data, not code (opcode density 0.037, entropy 6.25)
- north & south (1989)(infogrames)(m5)/North & South (1989)(Infogrames)(M5).adf: blocks 44-44 (bytes 22528-23040) outside the filesystem hold data, not code (opcode density 0.039, entropy 5.42)
- north & south (1989)(infogrames)(m5)/North & South (1989)(Infogrames)(M5).adf: blocks 52-174 (bytes 26624-89600) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.55)
- 1 never-loaded region(s), 5632 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 798208 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### North & South (1989)(Infogrames)(M5).adf:bootblock

- `call_relocation_routine`: Calls another routine and returns zero.
- `relocate_memory_hunks`: Allocates memory and processes a sequence of hunk types to perform relocation, clearing, or memory freeing operations.
- `process_hunk_data`: Iterates through a buffer of hunk types to perform memory operations such as relocation or freeing based on specific type codes.
- `apply_hunk_relocation`: Performs memory relocation by reading hunk types from a buffer and adding offsets to values stored at specific memory locations.
- `process_memory_hunks_2`: Iterates through a buffer of memory hunk type codes to perform relocation, clearing, or freeing operations.
- `process_memory_hunks`: Iterates through memory hunk types to perform relocation, clearing, or memory freeing operations based on specific type codes.
- `allocate_and_initialize_buffer`: Checks a counter against a threshold and, if below one, initializes a new memory block with specific values and a size, then recursively calls itself.
- `read_next_buffered_long`: Calls a helper, decrements a remaining-count field, and returns the next 32-bit value from a buffer while advancing the buffer pointer.
- `call_helper_repeatedly`: Calls a helper routine a specific number of times based on an input argument until the count reaches zero.
- `fill_array_from_buffer`: Repeatedly calls a helper routine to retrieve 32-bit values and stores them sequentially into a memory buffer.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `call_do_i_o`: Wrapper that calls the exec library function DoIO and returns its result to the caller.
- `call_alloc_mem`: Wrapper that calls the exec library function AllocMem and returns its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `initialize_and_update_game_state`: Initializes several global variables and hardware-related parameters through a series of function calls and pointer offsets.
- `calculate_scaled_value`: Calls a function pointer, adds the result to a constant and a value from a global offset, and returns a quotient and remainder based on a parameter.
- `search_list_for_bounds`: Iterates through a list of four-word structures to check if any element's values fall within a specific range defined by a global table.
- `swap_and_recurse`: Waits for a global variable to meet specific bitwise and magnitude conditions, swaps two adjacent 32-bit values in a table, and calls itself.
- `toggle_and_swap_state`: Toggles a bit in a 16-bit value and performs conditional 32-bit swaps between global memory locations based on a parameter.
- `process_and_link_list`: Retrieves data from a table, calculates a length, and updates a list of pointers by adding an offset to each entry.
- `extract_and_process_data`: Copies a block of data from a source to a destination and performs calculations to return a specific offset from a table.
- `fetch_and_store_result`: Calls a function to get a value, uses that value to call another function, and then stores the result into a table structure.
- `retrieve_table_value`: Fetches a pointer from a table, retrieves a value from that pointer, and stores it back into a table structure.
- `apply_bitmask_to_buffer`: Iterates through a list of indices to perform complex bitwise operations on a target buffer using values from multiple lookup tables.
- `initialize_bitmask_state`: Sets a global counter, updates several pointers based on a provided buffer, and populates a 32-bit mask using a parameter.
- `execute_single_instruction`: This routine executes exactly one instruction.
- `fill_memory_arrays`: fills two arrays with incrementing values in a loop and then calls a function via a pointer.
- `initialize_global_values`: assigns specific constant values to several global memory locations and calls another routine.
- `set_global_parameters`: writes two parameters and several constant values to global memory locations.
- `null_routine`: This routine contains a single instruction and performs no operations.
- `process_bitstream_data`: iterates through bits of a 64-bit value to update various memory locations and hardware-related registers.
- `update_system_state`: writes values to global variables and updates a function pointer before calling another routine.
- `copy_data_to_buffers`: copies a sequence of 16-bit words from a source pointer to two different destination memory locations.
- `insert_into_linked_list`: updates a function pointer in a global location and inserts a new node into a linked list structure.
- `remove_from_linked_list`: traverses a linked list to find a specific node and removes it by updating the preceding node's pointer.
- `execute_complex_sequence`: Performs a series of indirect function calls and updates multiple memory locations using values derived from those calls and pointer arithmetic.
- `find_empty_slot`: Iterates through a fixed-size table of integers to find the first zero entry and replaces it with a provided value.
- `update_sprite_attribute_table`: Calculates new coordinate and attribute values based on a table lookup and updates a series of memory locations.
- `perform_bitmask_area_clipping`: Calculates a rectangular intersection between two areas and applies a bitwise AND operation to the resulting memory block.
- `evaluate_boolean_condition`: Performs multiple comparisons and calls another routine to return a boolean result based on a parameter and global values.
- `copy_memory_block`: Copies a block of memory from a source to a destination, handling both forward and backward directions depending on the relative positions of the pointers.
- `calculate_mixed_product`: Computes a complex mathematical value using bitwise shifts and multiplications of two 32-bit values split into 16-bit halves.
- `execute_indirect_call`: Loads a function pointer from a table and executes it via an indirect jump.
- `execute_indirect_call_alt`: Retrieves a function address from a specific offset in a table and calls it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `skip_non_zero_bytes`: Calls another routine and then enters loops to skip over a sequence of null bytes or specific non-zero bytes in a global buffer.
- `rotate_data_blocks`: Copies a block of data to a local buffer and then performs a cyclic shift of the elements within that block.
- `transform_data_matrix`: Performs nested loops to manipulate and rearrange data within a structured memory area using offsets and multipliers.
- `apply_descending_brightness`: Iterates through a table of sixteen 16-bit values, applying a decreasing multiplier to specific bit nibbles and calling two other routines.
- `apply_ascending_brightness`: Iterates through a table of sixteen 16-bit values, applying an increasing multiplier to specific bit nibbles and calling two other routines.
- `update_global_pointers`: Calls a routine using a global variable address and stores the returned pointer and its subsequent data into several global locations.
- `update_display_parameters`: Fetches a pointer from a table, stores it in a global variable, and uses its contents to update several global values and call another routine with specific data.
- `call_function_via_pointer`: Calls a function using a pointer retrieved from a global variable.
- `load_data_structure`: Retrieves a pointer from a global variable, copies multiple fields from that pointed-to structure into a set of global variables, and calls another routine with a constant value.
- `execute_scripted_sequence`: Loads a data structure from memory, copies a string into a stack-based buffer, and executes a series of brightness and scripted events.
- `initialize_data_structures`: Retrieves several values from a table, stores them in global variables, and calls multiple functions to process them.
- `reverse_string_into_stack`: Retrieves a string via a function call and copies its characters in reverse order into a destination buffer.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 97 into d2, then loads 0 into d2.
- `execute_media_sequence`: Processes a sequence of data by calling several routines to manage values and performing a loop that updates a parameter based on a shifted value.
- `execute_scripted_media_sequence`: Loads and processes data from various media files to manage brightness levels, display parameters, and scripted events.
- `search_byte_in_list`: Compares a provided byte against a list of twenty-seven bytes and returns one if a match is found.
- `advance_byte_stream_pointer`: Calls a helper with a fixed argument, then advances a byte-stream pointer by one and wraps it back to a fixed start when the next byte is a negative terminator.
- `search_word_in_list`: Compares a provided word against a list of seventeen words and returns one if a match is found.
- `update_game_state`: Iterates through a loop to call functions with calculated parameters, updates several global counters, and performs specific hardware or display updates.
- `process_scripted_event_sequence`: Iterates through a data stream to execute a sequence of scripted instructions or events.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `generate_random_quotient_remainder`: Advances a 32-bit seed using a timing value from a helper and a constant, then returns the seed's quotient and remainder by a divisor taken from the argument, packed into the two halves of the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_callback_function`: Calls a function and passes itself as an argument.
- `call_indirect_function_4`: Calls a function through a pointer retrieved from a memory location.
- `poll_mouse_and_update_pointer`: Reads the joystick and mouse port counters and button registers, updates a stored pointer position from the movement deltas with clamping to the 320 by 200 screen, and writes the new position into a display-list word.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `write_constant_one`: Stores the value one into a memory location pointed to by a register.
- `update_structure_data`: Stores a calculated value into a memory location, updates a function pointer in a structure, and calls another routine through a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_array_elements`: Iterates through a list of values to check if they fall within a specific range defined by two global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_raster_then_swap_pointers`: Waits for the blitter to go idle and for the raster position to pass a fixed scanline, then swaps two saved pointers in the data area and calls a helper.
- `wait_raster_then_swap_pointers_with_param`: Waits for the blitter to go idle and for the raster position to pass a scanline defined by a parameter, then swaps two saved pointers in the data area and calls a helper.
- `toggle_and_update_pointer`: Toggles a bit in a word, and depending on the result, either copies a value from one location to another or updates a pointer to a new memory address.
- `copy_table_data_2`: Copies a 32-bit value from a table into two separate locations using an index.
- `copy_table_entries`: Copies a 32-bit value from a table into two different locations based on an index.
- `update_pointer_table`: Parses a data structure to retrieve a pointer and then iterates through a table of values, adding each value to its own memory address.
- `process_segmented_data_2`: Retrieves a data structure and copies a block of memory into a local buffer before performing calculations to determine a memory offset.
- `process_variable_length_data`: This routine parses a custom data structure consisting of a length byte followed by a sequence of values.
- `fetch_table_value`: Retrieves a value from a data structure and uses it to look up an entry in a table via a function pointer.
- `retrieve_data_offset`: Accesses a data structure to extract a value and then uses that value to perform a lookup through a function pointer.
- `store_two_words`: Stores two words from a parameter into a location relative to a global pointer.
- `initialize_bitmask_table`: Calculates a pointer to a data source and populates a four-element table with bitmask values derived from a provided integer.
- `apply_bitmask_to_memory`: Iterates through a list of values to perform bitwise operations on multiple memory locations using bit-shifting and masking logic.
- `calculate_accumulated_value`: Iterates through a buffer of bytes, looking up values in a table based on each byte, and returns the sum of these values plus a base constant.
- `nop_placeholder`: This routine is empty and performs no operations.
- `execute_blitter_sequence`: Performs a sequence of operations involving blitter hardware registers, waits for the blitter to become idle, and executes a library function and an indirect function call.
- `setup_bitplane_pointer_table`: Builds two arrays of four bitplane pointers 8000 bytes apart, sets a 40-byte pitch, 200-line height and four-plane count, then calls a library routine through a jump table and a helper.
- `clear_bitplane_with_blit`: Starts a blitter operation with only the destination channel enabled and a zero minterm, writing zeros over a full 200-line, 40-byte-wide bitplane at the stored destination pointer, with library calls before and after.
- `setup_blitter_transfer_parameters`: Configures a series of memory locations with values for a blitter operation and sets specific parameters via registers.
- `decompress_bitstream_data_2`: Extracts word-sized values from a compressed bitstream by using bitwise shifts and lookups in two separate tables.
- `expand_compressed_data`: Decompresses a data stream by reading bit-packed values and performing lookups in two tables to reconstruct words.
- `decompress_bitstream_data`: Extracts and reconstructs word-sized values from a compressed bitstream using a lookup table and bitwise transformations.
- `unpack_bitstream_data`: Reads a bitstream from a source buffer and performs bitwise transformations to extract and reconstruct word-sized values.
- `register_interrupt_handler`: Sets values in memory and calls a function pointer retrieved from a global variable.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `find_record_by_key`: Searches a zero-terminated list of three-word records for the key passed in a data register and leaves its address register pointing at the matching record.
- `store_two_bytes`: Stores a single byte into two consecutive memory locations.
- `process_data_sequence`: A routine consisting of 49 instructions that performs a series of operations involving multiple registers and memory accesses.
- `configure_audio_hardware`: Processes a bitfield to set various hardware registers and control bits, including volume and channel settings.
- `update_sprite_hardware`: Iterates through a table to update DMA controller registers and sprite position or attribute data based on values fetched from memory.
- `configure_dma_channels`: Iterates through a set of DMA channel configurations to update hardware registers.
- `update_hardware_registers_via_table`: This routine iterates through a table to fetch data and writes values to hardware registers.
- `update_state_and_offset`: Clears a specific bit in a status byte and conditionally increments a pointer offset and a counter based on comparisons between memory values.
- `set_bit_in_byte`: Sets the third bit of a byte at a specific offset from a pointer.
- `update_sprite_animation_state_2`: Updates sprite animation frame indices and handles frame looping or transitions based on property flags.
- `process_sprite_data_stream`: Iterates through a data stream to extract sprite attributes, animation frame indices, and property flags.
- `parse_sprite_definition_data`: Iterates through a data stream to extract sprite attributes and properties.
- `parse_sprite_attribute_table`: Iterates through a data stream to extract sprite attributes and control flags.
- `update_entity_state_and_audio`: Updates entity properties, animation frames, and bitwise flags by fetching data from lookup tables.
- `update_entity_animation_and_state`: Updates entity properties and animation frames by fetching data from lookup tables and bitwise flags.
- `update_entity_state_and_attributes`: Iterates through a data structure to check specific bitflags and updates entity properties by fetching values from a lookup table.
- `update_sprite_animation_state`: Iterates through a data stream to update sprite properties and animation frames.
- `return_without_action`: A bare return with no work and no calls.
- `compute_channel_volume_output`: Subtracts an attenuation taken from the channel's field from a helper's level, floored at zero, and writes the result with a second helper value into the channel's two output words.
- `clear_dma_control_register`: The routine writes zero to the DMACON hardware register and sets a global variable to a bit mask derived from an input value.
- `parse_sprite_attribute_stream`: Iterates through a data stream to extract sprite properties, animation frame indices, and attribute flags.
- `copy_data_to_hardware`: Copies sixteen bytes from a source pointer to both a hardware register address and another memory location.
- `convert_number_to_string_2`: Converts a numeric value into a decimal string and stores the resulting characters into a destination buffer.
- `convert_number_to_string`: Decomposes a numeric value into decimal digits and stores them as character bytes in a destination buffer.
- `copy_table_data`: Iterates through a table to copy bytes from one memory location to another based on an offset.
- `clear_byte_at_offset`: Sets a byte to zero at a memory location calculated from a base address and an offset retrieved from a table.
- `traverse_pointer_chain`: Follows a linked list of pointers until a null pointer is encountered, executing a routine at each step.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `insert_into_linked_list_2`: Calls a function via a pointer found in a structure and inserts a new node into a linked list.
- `remove_linked_list_node`: Searches a linked list for a specific node and removes it by updating pointers, then calls a function via a global pointer.
- `set_four_plane_pointers`: Writes a base address into four per-plane pointer slots, each 8000 bytes further along than the last, splitting each 32-bit address into high and low words.
- `init_seven_channel_slots`: Stores a 32-bit pointer from a data area into a header, then fills seven four-word slots with a fixed constant and zero.
- `call_nonzero_handler_table`: Walks an eight-entry table of routine pointers and calls each non-null entry in turn, returning zero.
- `find_empty_handler_slot`: Iterates through an eight-entry table to find the first null pointer and stores a value there.
- `clear_matching_integer`: Iterates through a list of integers and sets a value to zero if it matches the provided parameter.
- `search_and_clear_value`: Iterates through a list of pointers until a specific value is matched, then sets the value at that pointer to zero.
- `initialize_system_subsystems`: Allocates memory, opens libraries, and initializes several data records and hardware registers including DMA and display parameters.
- `release_system_resources`: Calls functions via pointers stored in global variables and releases memory and interrupt server resources.
- `clear_and_open_library`: Fills a memory range with zeros, opens a library, and executes a function from the returned library base if it is not null.
- `wait_for_bit_clear`: Enters a loop that polls a specific bit in a byte until that bit is cleared.
- `store_word_to_offset`: Stores a 16-bit word into a memory location calculated by adding an offset to a pointer.
- `store_word_to_offset_2`: Writes a 16-bit value to a specific offset from a base pointer.
- `perform_blitter_area_transform`: Calculates dimensions and source/destination offsets to perform a series of blitter operations.
- `swap_and_translate_byte_pairs`: Swaps bytes between two positions of a buffer, mapping each through a translation table as it goes, over a block whose dimensions come from the caller's registers.
- `clip_bitplane_rectangle`: Calculates the intersection of a rectangular area with a bounding box and applies a bitwise AND mask to the corresponding bitplane data.
- `and_mask_column_four_planes`: ANDs a 16-bit mask into a column of words across four bitplanes, stepping down the rows with a 40-byte pitch to clear the masked bits.
- `store_word_to_offset_3`: Stores two consecutive 16-bit words into memory at a fixed offset from a pointer.
- `wait_blitter_idle_keep_value`: Spins while the blitter busy flag is set, then returns its data register unchanged.
- `compare_strings_with_limit`: Compares two null-terminated strings up to a specified maximum length and returns the difference between the first non-matching characters.
- `compare_strings_case_insensitive`: Compares two strings case-insensitively by normalizing characters within a specific range and returns the difference of the first mismatch.
- `evaluate_string_match`: Compares strings and updates an object's state by calling specific functions based on the comparison results.
- `search_string_list`: Iterates through a list of strings, performing case-insensitive comparisons until a match is found or a limit is reached.
- `load_word_from_table`: Loads a 32-bit value from a table using an index and a base pointer stored in memory.
- `call_recursive_function`: The routine performs a recursive call to itself without performing any other operations before returning.
- `search_and_load_table_entry`: Searches for a string within a list and, if a match is found, loads a 32-bit value from a table using an index.
- `reset_object_status`: Checks a condition and, if met, updates an object's state using a function and resets two specific fields.
- `update_table_entry`: Updates an entry in a table using one of three modes: direct assignment, addition, or addition using a value from another table.
- `update_object_state`: Compares a value against a global variable and either returns a status or updates the variable and calls another routine.
- `copy_memory_block_2`: Copies a specified number of bytes from a source to a destination.
- `process_segmented_data`: Iterates through a table of data structures to perform calculations and updates based on specific bitwise and arithmetic conditions.
- `nop_padding_gap`: This routine is empty and contains no executable instructions.
- `process_string_request`: Searches for a string and, if not found, iterates through a list of structures to update a specific field and store a result in a buffer.
- `process_object_state_transition`: Checks a specific field in an object structure for a constant value and, if matched, updates a field in a separate data structure via a calculated offset before calling another routine.
- `update_object_state_two`: Writes four values to a structure and calls a function pointer via a pointer in a global table.
- `update_object_state_three`: Writes four values to a structure, using a different constant value, and calls a function pointer via a pointer in a global table.
- `reset_object_state_nine`: Writes a zero and a constant to a structure and calls a function pointer via a pointer in a global table.
- `set_object_state_four`: Writes a constant to a structure and calls a function pointer via a pointer in a global table.
- `set_status_fifteen`: Writes a constant to a structure and returns a value from a different offset in that structure.
- `release_all_resources`: Checks multiple global pointers and calls various routines to clear fields and release sub-blocks or resources.
- `initialize_data_records`: Allocates and initializes several data records through a series of conditional checks and indirect calls.
- `safe_memory_copy`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `perform_signed_division`: Determines the sign of the result based on input registers, calls a routine to perform unsigned 32-bit division, and negates the quotient if the sign flag indicates a negative result.
- `calculate_integer_division`: Performs division and modulo operations on two integers using a loop-based arithmetic approach.
- `perform_bitwise_arithmetic_loop`: Executes a nested loop structure that performs bit shifts, sign-extension checks, and decrements on integer values.
- `execute_nested_loops`: Performs a series of arithmetic operations within nested loops involving bit shifts and decrements.
- `append_string_to_buffer`: Finds the end of a string and then copies a second string to that location until a null terminator is reached.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `allocate_and_initialize_record`: Allocates a new record, populates its fields with provided parameters, and manages its internal pointer chain.
- `reset_record_and_release_resources`: Releases a record's sub-block when one is present, marks the record's state as inactive and clears its counter field.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `assign_parameter_to_record`: Attempts to allocate a record and, if successful, assigns a parameter and a value to its internal fields.
- `clear_record_fields`: Sets specific fields within a structure to maximum values and calls another routine using a value from the structure.
- `execute_indirect_call_2`: Performs an indirect call to a function address retrieved from a global table.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_indirect_function`: Calls a function through a pointer located at a specific offset within a structure.
- `call_indirect_function_2`: Calls a function using a pointer retrieved from a structure at a different offset.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_indirect_function_3`: Executes a function call via a pointer found at a third unique offset within a structure.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
